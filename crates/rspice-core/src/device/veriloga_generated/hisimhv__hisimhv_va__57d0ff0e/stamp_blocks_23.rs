#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_147(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) {s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);}
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) {s.copy_ad(445, 116);}
        s.b[2792] = (p[33] == 2.0);s.store_scalar(2792, if s.b[2792] { 1.0 } else { 0.0 });s.b[2793] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));s.store_scalar(2793, if s.b[2793] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) {s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);s.store_square(722, 781);s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2794] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2794, if s.b[2794] { 1.0 } else { 0.0 });s.b[2795] = (2.0 == 1.0);s.store_scalar(2795, if s.b[2795] { 1.0 } else { 0.0 });
        if (((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) && s.b[2795]) {s.store_scalar(720, 1.0);}
        s.b[2796] = (2.0 == 2.0);s.store_scalar(2796, if s.b[2796] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) && (!s.b[2795])) && s.b[2796]) {s.store_scalar(720, 2.0);}
        s.b[2797] = (2.0 == 4.0);s.store_scalar(2797, if s.b[2797] { 1.0 } else { 0.0 });
        if (((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) && (!s.b[2795])) && (!s.b[2796])) && s.b[2797]) {s.store_scalar(720, 3.0);}
        s.b[2798] = (2.0 == 8.0);s.store_scalar(2798, if s.b[2798] { 1.0 } else { 0.0 });
        if ((((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) && (!s.b[2795])) && (!s.b[2796])) && (!s.b[2797])) && s.b[2798]) {s.store_scalar(720, 4.0);}
        if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) {s.store_scalar(719, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;
            if t1 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t1, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && s.b[2794]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) && (!s.b[2794])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);}
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && s.b[2793]) {
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && s.b[2792]) && (!s.b[2793])) {s.copy_ad(116, 445);s.store_scalar(335, 1.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2790]) && (!s.b[2792])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }
        s.b[2799] = (p[33] == 1.0);s.store_scalar(2799, if s.b[2799] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[2800] = (s.v[411] > 0.0);s.store_scalar(2800, if s.b[2800] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && s.b[2800]) {s.store_sub_from_scalar(336, p[334], 411);}
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && (!s.b[2800])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[2801] = (s.v[336] < 0.0);s.store_scalar(2801, if s.b[2801] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && (!s.b[2800])) && s.b[2801]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && (!s.b[2800])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2802] = (s.v[336] < 0.0);s.store_scalar(2802, if s.b[2802] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && s.b[2802]) {s.store_scalar(336, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_148(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && s.b[2802]) {s.store_scalar(343, 0.0);}
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2762, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_add(414, 404, 397);s.store_mul_sub_rhs(333, 419, 414, 418);}
        s.b[2803] = (s.v[333] < 60.0);s.store_scalar(2803, if s.b[2803] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && s.b[2803]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);}
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && (!s.b[2803])) {s.store_sub(416, 414, 418);}
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) {s.store_mul(415, 154, 416);}
        s.b[2804] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));s.store_scalar(2804, if s.b[2804] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2799]) && s.b[2804]) {s.store_primal_offset(2768, 2768, 1.0);s.copy_ad(116, 447);}
        if ((s.v[2625] != 0.0) && (!s.b[2786])) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[2805] = (((s.v[116]) as f64).abs() > 1e-6);s.store_scalar(2805, if s.b[2805] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2805]) {s.store_add_offset_lhs_mixed_ia(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));s.store_sqrt(336, 335);}
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && (!s.b[2805])) {s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));}
        if ((s.v[2625] != 0.0) && (!s.b[2786])) {s.store_mul(354, 410, 336);s.store_mul_sub_rhs(398, 413, 402, 404);s.store_div(2806, 354, 2762);}
        s.b[2808] = (p[33] == 2.0);s.store_scalar(2808, if s.b[2808] { 1.0 } else { 0.0 });s.b[2809] = ((s.v[2806] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));s.store_scalar(2809, if s.b[2809] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) {s.store_add_scaled_inputs3_indices(781, 2806, 1.0, 386, (-1.0), 386, 0.1);s.store_square(722, 781);s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2810] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2810, if s.b[2810] { 1.0 } else { 0.0 });s.b[2811] = (2.0 == 1.0);s.store_scalar(2811, if s.b[2811] { 1.0 } else { 0.0 });
        if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) && s.b[2811]) {s.store_scalar(720, 1.0);}
        s.b[2812] = (2.0 == 2.0);s.store_scalar(2812, if s.b[2812] { 1.0 } else { 0.0 });
        if (((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) && (!s.b[2811])) && s.b[2812]) {s.store_scalar(720, 2.0);}
        s.b[2813] = (2.0 == 4.0);s.store_scalar(2813, if s.b[2813] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) && (!s.b[2811])) && (!s.b[2812])) && s.b[2813]) {s.store_scalar(720, 3.0);}
        s.b[2814] = (2.0 == 8.0);s.store_scalar(2814, if s.b[2814] { 1.0 } else { 0.0 });
        if (((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) && (!s.b[2811])) && (!s.b[2812])) && (!s.b[2813])) && s.b[2814]) {s.store_scalar(720, 4.0);}
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) {s.store_scalar(719, 0.0);}
        let mut t3: usize = 0;
        while {
            let t2: f64 = if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;
            if t3 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t3, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && s.b[2810]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) && (!s.b[2810])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2809]) {
        }
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && (!s.b[2809])) {s.copy_ad(335, 2806);s.store_scalar(334, 1.0);}
        s.b[2815] = (s.v[334] < 1.0);s.store_scalar(2815, if s.b[2815] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_149(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2808]) && s.b[2815]) {s.store_primal_offset(2768, 2768, 2.0);}
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && (!s.b[2808])) {
            if (s.v[2806] <= s.v[386]) {
                s.copy_ad(335, 2806);
            } else {
                s.copy_ad(335, 386);
            }
        }
        s.b[2816] = (s.v[2806] >= s.v[386]);s.store_scalar(2816, if s.b[2816] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && (!s.b[2808])) && s.b[2816]) {s.store_primal_offset(2768, 2768, 2.0);}
        s.b[2817] = (s.v[2768] >= 2.0);s.store_scalar(2817, if s.b[2817] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) {s.copy_ad(2807, 404);s.store_mul(354, 335, 2762);s.store_sub_div_rhs_indices(404, 402, 354, 413);}
        s.b[2818] = (p[33] == 2.0);s.store_scalar(2818, if s.b[2818] { 1.0 } else { 0.0 });s.b[2819] = ((s.v[404] > (s.v[2807] - 0.1)) && (0.1 >= 0.0));s.store_scalar(2819, if s.b[2819] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) {s.store_offset_sub(781, 404, 2807, 0.1);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2820] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2820, if s.b[2820] { 1.0 } else { 0.0 });s.b[2821] = (2.0 == 1.0);s.store_scalar(2821, if s.b[2821] { 1.0 } else { 0.0 });
        if (((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) && s.b[2821]) {s.store_scalar(720, 1.0);}
        s.b[2822] = (2.0 == 2.0);s.store_scalar(2822, if s.b[2822] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) && (!s.b[2821])) && s.b[2822]) {s.store_scalar(720, 2.0);}
        s.b[2823] = (2.0 == 4.0);s.store_scalar(2823, if s.b[2823] { 1.0 } else { 0.0 });
        if (((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) && (!s.b[2821])) && (!s.b[2822])) && s.b[2823]) {s.store_scalar(720, 3.0);}
        s.b[2824] = (2.0 == 8.0);s.store_scalar(2824, if s.b[2824] { 1.0 } else { 0.0 });
        if ((((((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) && (!s.b[2821])) && (!s.b[2822])) && (!s.b[2823])) && s.b[2824]) {s.store_scalar(720, 4.0);}
        if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) {s.store_scalar(719, 0.0);}
        let mut t5: usize = 0;
        while {
            let t4: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t4 != 0.0
        } {
            t5 += 1;
            if t5 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t5, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && s.b[2820]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) && (!s.b[2820])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_add_offset_lhs(404, 2807, (-0.1), 780);}
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && s.b[2819]) {
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) {
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) {s.store_scalar(334, 1.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2817]) && (!s.b[2818])) {
            if (s.v[404] <= s.v[2807]) {
            } else {
                s.copy_ad(404, 2807);
            }
        }
        if ((s.v[2625] != 0.0) && (!s.b[2786])) {s.copy_ad(2769, 404);}
        s.b[2825] = (p[33] == 1.0);s.store_scalar(2825, if s.b[2825] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {s.store_scalar(79, 0.0);s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2762)), s.ad_value(155)), 2.0);}
        s.b[2826] = (s.v[411] > 0.0);s.store_scalar(2826, if s.b[2826] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && s.b[2826]) {s.store_sub_from_scalar(336, p[334], 411);}
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2826])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[2827] = (s.v[336] < 0.0);s.store_scalar(2827, if s.b[2827] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2826])) && s.b[2827]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2826])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2828] = (s.v[336] < 0.0);s.store_scalar(2828, if s.b[2828] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_150(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && s.b[2828]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2762, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_151(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut t8: usize = 0;
        while {
            let t6: f64 = (s.v[421] + 1.0);let t7: f64 = if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (s.v[97] <= t6)) { 1.0 } else { 0.0 };
            t7 != 0.0
        } {
            t8 += 1;
            if t8 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t8, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[2830] = (s.v[333] < 60.0);s.store_scalar(2830, if s.b[2830] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && s.b[2830]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2830])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {s.store_mul(415, 154, 416);}
            s.b[2831] = (s.v[116] < 0.0);s.store_scalar(2831, if s.b[2831] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && s.b[2831]) {s.store_scalar(334, (-0.7071067811865475));s.store_mul(223, 116, 334);s.store_mul(420, 154, 334);}
            s.b[2832] = (s.v[116] < 1e-6);s.store_scalar(2832, if s.b[2832] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2831])) && s.b[2832]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(338, 334, 336);}
            s.b[2833] = (s.v[338] > 0.0);s.store_scalar(2833, if s.b[2833] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2831])) && s.b[2832]) && s.b[2833]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);}
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2831])) && s.b[2832]) && (!s.b[2833])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2831])) && (!s.b[2832])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));}
            s.b[2834] = (s.v[338] > 0.0);s.store_scalar(2834, if s.b[2834] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2831])) && (!s.b[2832])) && s.b[2834]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);}
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2831])) && (!s.b[2832])) && (!s.b[2834])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            s.b[2835] = (s.v[116] < 0.0);s.store_scalar(2835, if s.b[2835] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && s.b[2835]) {s.store_scalar(214, 0.0);s.store_scalar(215, 0.0);s.store_neg(216, 223);s.store_neg(217, 420);}
            s.b[2836] = (s.v[116] < 60.0);s.store_scalar(2836, if s.b[2836] { 1.0 } else { 0.0 });s.b[2837] = (s.v[116] < 5e-5);s.store_scalar(2837, if s.b[2837] { 1.0 } else { 0.0 });
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2835])) && s.b[2836]) && s.b[2837]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            if ((((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2835])) && s.b[2836]) && (!s.b[2837])) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2835])) && (!s.b[2836])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[2838] = (s.v[214] > 0.0);s.store_scalar(2838, if s.b[2838] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2835])) && s.b[2838]) {s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_add_product_indices(217, 215, 0.5, 420, 223, (2.0 * 0.5), 216, 1.0);}
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2835])) && (!s.b[2838])) {s.copy_ad(216, 223);s.copy_ad(217, 420);}
            if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[2839] = (s.v[79] == 1.0);s.store_scalar(2839, if s.b[2839] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && s.b[2839]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2839])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2839])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2840] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2840, if s.b[2840] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2839])) && s.b[2840]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2839])) {s.store_add(404, 404, 236);}
            s.b[2841] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2841, if s.b[2841] { 1.0 } else { 0.0 });
            if (((((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) && (!s.b[2839])) && s.b[2841]) {s.store_scalar(79, 1.0);}
            if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_152(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (!s.b[2786])) && s.b[2825]) {s.store_mul(2760, 982, 223);s.store_mul(2761, 2762, 2760);s.store_offset_div(100, 2761, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        s.b[2843] = (p[33] == 4.0);s.store_scalar(2843, if s.b[2843] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[2843]) {s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));s.store_div(334, 394, 409);s.store_square(405, 334);s.store_mul(222, 405, 229);s.copy_ad(404, 2769);s.store_scalar(79, 0.0);s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2762)), s.ad_value(155)), 2.0);}
        s.b[2844] = (s.v[411] > 0.0);s.store_scalar(2844, if s.b[2844] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2844]) {s.store_sub_from_scalar(336, p[334], 411);}
        if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2844])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[2845] = (s.v[336] < 0.0);s.store_scalar(2845, if s.b[2845] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2844])) && s.b[2845]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2844])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if ((s.v[2625] != 0.0) && s.b[2843]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2846] = (s.v[336] < 0.0);s.store_scalar(2846, if s.b[2846] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2846]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((s.v[2625] != 0.0) && s.b[2843]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2762, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_153(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        let mut tb: usize = 0;
        while {
            let t9: f64 = (s.v[421] + 1.0);let ta: f64 = if (((s.v[2625] != 0.0) && s.b[2843]) && (s.v[97] <= t9)) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;
            if tb > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tb, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.v[2625] != 0.0) && s.b[2843]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[2848] = (s.v[333] < 60.0);s.store_scalar(2848, if s.b[2848] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2848]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2848])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if ((s.v[2625] != 0.0) && s.b[2843]) {s.store_mul(415, 154, 416);}
            s.b[2849] = (((s.v[116]) as f64).abs() < 1e-6);s.store_scalar(2849, if s.b[2849] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2849]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(2770, 334, 336);s.store_mul_add_scaled_product_rhs_indices(2771, 154, 335, 1.0, 417, 337, (-1.0));}
            if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2849])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(2770, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));s.store_mul_sub_mixed_iaa(2771, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));}
            s.b[2850] = (((s.v[116]) as f64).abs() < 5e-5);s.store_scalar(2850, if s.b[2850] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2850]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            s.b[2851] = (((s.v[116]) as f64).abs() < 60.0);s.store_scalar(2851, if s.b[2851] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2850])) && s.b[2851]) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2850])) && (!s.b[2851])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[2852] = (s.v[214] > 0.0);s.store_scalar(2852, if s.b[2852] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2852]) {s.store_sqrt_add(216, 2770, 214);s.store_div_scaled_inputs2_indices(217, 2771, 0.5, 215, 0.5, 216, 1.0);}
            s.b[2853] = (s.v[2770] > 0.0);s.store_scalar(2853, if s.b[2853] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2852])) && s.b[2853]) {s.store_sqrt(216, 2770);s.store_div_scaled_inputs_indices(217, 2771, 0.5, 216, 1.0);}
            if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2852])) && (!s.b[2853])) {s.store_scalar(216, 0.0);s.store_scalar(217, 0.0);}
            if ((s.v[2625] != 0.0) && s.b[2843]) {s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((s.v[2625] != 0.0) && s.b[2843]) {s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((s.v[2625] != 0.0) && s.b[2843]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[2854] = (s.v[79] > 0.0);s.store_scalar(2854, if s.b[2854] { 1.0 } else { 0.0 });
            if (((s.v[2625] != 0.0) && s.b[2843]) && s.b[2854]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2854])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2854])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2855] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2855, if s.b[2855] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2854])) && s.b[2855]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2854])) {s.store_add(404, 404, 236);}
            s.b[2856] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2856, if s.b[2856] { 1.0 } else { 0.0 });
            if ((((s.v[2625] != 0.0) && s.b[2843]) && (!s.b[2854])) && s.b[2856]) {s.store_primal_offset(79, 79, 2.0);}
            if ((s.v[2625] != 0.0) && s.b[2843]) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_154(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.v[2625] != 0.0) && s.b[2843]) {
            if (s.v[2770] >= 0.0) {
                s.store_scaled_sqrt(223, 2770, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }
        if ((s.v[2625] != 0.0) && s.b[2843]) {s.store_mul(2760, 982, 223);s.store_mul(2761, 2762, 2760);s.store_offset_div(100, 2761, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        if (s.v[2625] != 0.0) {s.store_sub(399, 398, 354);}
        s.b[2858] = (s.v[407] < 0.0);s.store_scalar(2858, if s.b[2858] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[2858]) {s.store_neg(407, 407);}
        s.b[2859] = (p[55] == 0.0);s.store_scalar(2859, if s.b[2859] { 1.0 } else { 0.0 });s.b[2860] = (p[50] == 0.0);s.store_scalar(2860, if s.b[2860] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) && s.b[2860]) {s.store_neg(2763, 404);}
        if ((((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) && (!s.b[2860])) {s.copy_ad(2763, 396);}
        if (((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) {s.store_sqrt_offset_square_offset(782, 2763, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2763), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(2763), p[137]), 782, 0.5);}
        s.b[2861] = (s.v[336] < 0.0);s.store_scalar(2861, if s.b[2861] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) && s.b[2861]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));}
        if (((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.v[2625] != 0.0) && s.b[2858]) && s.b[2859]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub(407, 407, 603);}
        s.b[2862] = (2.0 == 1.0);s.store_scalar(2862, if s.b[2862] { 1.0 } else { 0.0 });s.b[2863] = (2.0 == 2.0);s.store_scalar(2863, if s.b[2863] { 1.0 } else { 0.0 });s.b[2864] = (2.0 == 3.0);s.store_scalar(2864, if s.b[2864] { 1.0 } else { 0.0 });s.b[2865] = (2.0 == 4.0);s.store_scalar(2865, if s.b[2865] { 1.0 } else { 0.0 });s.b[2866] = (p[55] == 1.0);s.store_scalar(2866, if s.b[2866] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && s.b[2862]) && s.b[2866]) {s.store_scale(338, 407, s.v[635]);}
        if (((s.v[2625] != 0.0) && s.b[2862]) && (!s.b[2866])) {s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));}
        if ((s.v[2625] != 0.0) && s.b[2862]) {s.store_mul(353, 338, 398);s.store_mul(356, 338, 354);}
        if ((s.v[2625] != 0.0) && (s.b[2863] && (!s.b[2862]))) {s.store_scale(338, 407, (s.v[635] * s.v[526]));s.store_mul(351, 338, 398);s.store_mul(359, 338, 354);}
        s.b[2867] = (p[55] == 1.0);s.store_scalar(2867, if s.b[2867] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (s.b[2864] && (!(s.b[2862] || s.b[2863])))) && s.b[2867]) {s.store_scale(338, 407, s.v[635]);}
        if (((s.v[2625] != 0.0) && (s.b[2864] && (!(s.b[2862] || s.b[2863])))) && (!s.b[2867])) {s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));}
        if ((s.v[2625] != 0.0) && (s.b[2864] && (!(s.b[2862] || s.b[2863])))) {s.copy_ad(697, 404);}
        s.b[2868] = (p[430] == 0.0);s.store_scalar(2868, if s.b[2868] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (s.b[2864] && (!(s.b[2862] || s.b[2863])))) && s.b[2868]) {s.copy_ad(698, 354);}
        if ((s.v[2625] != 0.0) && (s.b[2864] && (!(s.b[2862] || s.b[2863])))) {s.store_mul(352, 338, 398);s.store_mul(355, 338, 354);s.copy_ad(816, 355);}
        if ((s.v[2625] != 0.0) && (s.b[2865] && (!((s.b[2862] || s.b[2863]) || s.b[2864])))) {s.store_scale(338, 407, (s.v[635] * s.v[526]));s.store_mul(350, 338, 398);s.store_mul(358, 338, 354);}
        s.store_scalar(2625, 0.0);s.b[2869] = (3.0 == 1.0);s.store_scalar(2869, if s.b[2869] { 1.0 } else { 0.0 });s.b[2870] = (3.0 == 2.0);s.store_scalar(2870, if s.b[2870] { 1.0 } else { 0.0 });s.b[2871] = (3.0 == 3.0);s.store_scalar(2871, if s.b[2871] { 1.0 } else { 0.0 });s.b[2872] = (3.0 == 4.0);s.store_scalar(2872, if s.b[2872] { 1.0 } else { 0.0 });s.b[2873] = (((p[36] == 1.0) && (p[66] > 0.0)) && (s.v[460] > 0.0));s.store_scalar(2873, if s.b[2873] { 1.0 } else { 0.0 });
        if (s.b[2869] && s.b[2873]) {s.store_scalar(2625, 1.0);s.store_scalar(2623, 1.0);s.store_sub(395, 731, 728);s.store_neg(396, 728);s.store_scalar(409, s.v[460]);s.store_scalar(407, p[66]);s.store_scalar(411, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_155(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[2869] && s.b[2873]) {s.copy_ad(410, 687);s.store_scalar(413, s.v[188]);}
        s.b[2874] = (((((p[36] == 1.0) && (p[66] > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p[55] != 1.0));s.store_scalar(2874, if s.b[2874] { 1.0 } else { 0.0 });
        if ((s.b[2870] && (!s.b[2869])) && s.b[2874]) {s.store_scalar(2625, 1.0);s.store_sub(395, 734, 735);s.store_neg(396, 735);}
        s.b[2875] = (((p[35] == 1.0) && (p[63] > 0.0)) && (s.v[459] > 0.0));s.store_scalar(2875, if s.b[2875] { 1.0 } else { 0.0 });
        if ((s.b[2871] && (!(s.b[2869] || s.b[2870]))) && s.b[2875]) {s.store_scalar(2625, 1.0);s.store_scalar(2626, 1.0);s.store_sub(395, 731, 728);s.store_sub(396, 729, 728);s.store_scalar(409, s.v[459]);s.store_scalar(407, (p[63] + (p[64] * p[55])));s.copy_ad(411, 384);s.copy_ad(410, 686);s.copy_ad(413, 412);s.store_neg(407, 407);}
        s.b[2876] = (((s.v[407] < 0.0) && (p[432] > 0.0)) && (p[55] == 1.0));s.store_scalar(2876, if s.b[2876] { 1.0 } else { 0.0 });
        if (((s.b[2871] && (!(s.b[2869] || s.b[2870]))) && s.b[2875]) && s.b[2876]) {s.store_neg(407, 407);s.store_scalar(335, p[63]);s.store_offset_div_scaled_product_indices(996, 335, 335, 1.0, 651, 1.0, (-p[137]));}
        s.b[2877] = (p[113] > 0.0);s.store_scalar(2877, if s.b[2877] { 1.0 } else { 0.0 });s.b[2878] = ((s.v[396] == 0.0) || (p[113] <= 0.0));s.store_scalar(2878, if s.b[2878] { 1.0 } else { 0.0 });
        if (((((s.b[2871] && (!(s.b[2869] || s.b[2870]))) && s.b[2875]) && s.b[2876]) && s.b[2877]) && s.b[2878]) {
        }
        if (((((s.b[2871] && (!(s.b[2869] || s.b[2870]))) && s.b[2875]) && s.b[2876]) && s.b[2877]) && (!s.b[2878])) {s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));}
        if (((((s.b[2871] && (!(s.b[2869] || s.b[2870]))) && s.b[2875]) && s.b[2876]) && s.b[2877]) && (!s.b[2878])) {s.store_mul(784, 783, 396);s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p[113], 1.0);s.store_powf(782, 781, (1.0 / p[113]));s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);}
        if ((((s.b[2871] && (!(s.b[2869] || s.b[2870]))) && s.b[2875]) && s.b[2876]) && s.b[2877]) {s.store_sqrt_offset_square_offset(782, 396, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(396), p[137]), 782, 0.5);}
        s.b[2879] = (s.v[336] < 0.0);s.store_scalar(2879, if s.b[2879] { 1.0 } else { 0.0 });
        if (((((s.b[2871] && (!(s.b[2869] || s.b[2870]))) && s.b[2875]) && s.b[2876]) && s.b[2877]) && s.b[2879]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.b[2871] && (!(s.b[2869] || s.b[2870]))) && s.b[2875]) && s.b[2876]) && s.b[2877]) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub(407, 407, 600);}
        s.b[2880] = (((((p[35] == 1.0) && (p[63] > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p[55] != 1.0));s.store_scalar(2880, if s.b[2880] { 1.0 } else { 0.0 });
        if ((s.b[2872] && (!((s.b[2869] || s.b[2870]) || s.b[2871]))) && s.b[2880]) {s.store_scalar(2625, 1.0);s.store_sub(395, 734, 735);s.store_sub(396, 733, 735);}
        if (s.v[2625] != 0.0) {s.store_scalar(2888, 0.4);s.store_scalar(2889, 0.0);s.store_scalar(223, 0.0);s.store_scalar(214, 0.0);s.store_scalar(216, 0.0);s.store_scalar(232, 0.0);s.store_scalar(236, 0.0);s.store_scalar(233, 0.0);s.store_scalar(217, 0.0);s.store_scalar(420, 0.0);s.store_scalar(215, 0.0);s.store_scalar(447, 0.0);s.store_scalar(445, 0.0);s.store_scalar(446, 0.0);s.store_scalar(79, (-1.0));s.store_scalar(2890, 0.0);s.store_scalar(2891, 0.0);s.store_mul_scaled_ln_ad_rhs(2886, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2886), (-0.1));s.store_scalar(782, ((4.0 * 0.8) * 0.1));}
        if (s.v[2625] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.v[2625] != 0.0) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(2887, 781, (-0.5), 782, (-0.5), 0.8);}
        s.b[2893] = (s.v[2888] > (s.v[2887] * 0.5));s.store_scalar(2893, if s.b[2893] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[2893]) {s.store_scale(2888, 2887, 0.5);}
        s.b[2894] = param_given[338];s.store_scalar(2894, if s.b[2894] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[2894]) {s.store_scalar(2887, p[338]);}
        s.b[2895] = param_given[339];s.store_scalar(2895, if s.b[2895] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[2895]) {s.store_scalar(2888, p[339]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_156(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[2896] = param_given[338];s.store_scalar(2896, if s.b[2896] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2895])) && s.b[2896]) {s.store_scale(2888, 2887, 0.5);}
        s.b[2897] = (s.v[2888] > (s.v[2887] * 0.5));s.store_scalar(2897, if s.b[2897] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[2897]) {s.store_scale(2888, 2887, 0.5);}
        s.b[2898] = (p[38] == 1.0);s.store_scalar(2898, if s.b[2898] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[2898]) {s.store_neg(334, 396);}
        s.b[2899] = (s.v[334] > s.v[2888]);s.store_scalar(2899, if s.b[2899] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && s.b[2898]) && s.b[2899]) {s.store_sub(335, 334, 2888);s.store_sub(336, 2887, 2888);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);s.store_neg(345, 345);s.store_add(344, 2888, 333);}
        if (((s.v[2625] != 0.0) && s.b[2898]) && (!s.b[2899])) {s.copy_ad(344, 334);}
        if ((s.v[2625] != 0.0) && s.b[2898]) {s.store_neg(397, 344);}
        if ((s.v[2625] != 0.0) && (!s.b[2898])) {s.copy_ad(397, 396);}
        if (s.v[2625] != 0.0) {s.store_div(212, 410, 413);s.store_square(213, 212);s.store_sub_from_scalar(402, s.v[458], 395);s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);s.store_scalar(2882, 0.0);s.store_primal_scale(2883, 409, 1.6021918e-19);s.store_div(334, 394, 409);s.store_square(405, 334);}
        s.b[2900] = ((s.v[154] * (-s.v[397])) >= 500.0);s.store_scalar(2900, if s.b[2900] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[2900]) {s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(334, 1.403592217853e217);}
        if ((s.v[2625] != 0.0) && (!s.b[2900])) {s.store_mul_scale_offset_indices(781, 154, 397, -1.0, 0.0);s.store_scalar(229, 1.0);}
        let mut td: usize = 0;
        while {
            let tc: f64 = if (((s.v[2625] != 0.0) && (!s.b[2900])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;
            if td > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", td, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((s.v[2625] != 0.0) && (!s.b[2900])) {s.store_scale(229, 229, 1.14200738981568e26);s.store_offset(781, 781, (-60.0));}
        }
        if ((s.v[2625] != 0.0) && (!s.b[2900])) {s.store_mul_exp_rhs(229, 229, 781);s.copy_ad(334, 229);}
        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));s.store_scalar(782, (4.0 * 0.5));}
        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);}
        s.b[2901] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));s.store_scalar(2901, if s.b[2901] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) {s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);s.store_square(722, 781);s.store_square(723, 335);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2902] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2902, if s.b[2902] { 1.0 } else { 0.0 });s.b[2903] = (1.0 == 1.0);s.store_scalar(2903, if s.b[2903] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && s.b[2903]) {s.store_scalar(720, 1.0);}
        s.b[2904] = (1.0 == 2.0);s.store_scalar(2904, if s.b[2904] { 1.0 } else { 0.0 });
        if ((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && (!s.b[2903])) && s.b[2904]) {s.store_scalar(720, 2.0);}
        s.b[2905] = (1.0 == 4.0);s.store_scalar(2905, if s.b[2905] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_157(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
    ) {
        if (((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && (!s.b[2903])) && (!s.b[2904])) && s.b[2905]) {s.store_scalar(720, 3.0);}
        s.b[2906] = (1.0 == 8.0);s.store_scalar(2906, if s.b[2906] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && (!s.b[2903])) && (!s.b[2904])) && (!s.b[2905])) && s.b[2906]) {s.store_scalar(720, 4.0);}
        if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) {s.store_scalar(719, 0.0);}
        let mut tf: usize = 0;
        while {
            let te: f64 = if (((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;
            if tf > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", tf, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && s.b[2902]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) && (!s.b[2902])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 335, 726);s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);}
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && s.b[2901]) {
        }
        if (((s.v[2625] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2901])) {s.store_add(335, 402, 397);s.store_scalar(334, 1.0);}
        if ((s.v[2625] != 0.0) && (s.v[406] != 0.0)) {s.store_sub(397, 335, 402);s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);}
        s.b[2907] = (s.v[402] < s.v[403]);s.store_scalar(2907, if s.b[2907] { 1.0 } else { 0.0 });
        if ((s.v[2625] != 0.0) && s.b[2907]) {s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_add_rhs(332, 154, 402, 397);s.store_div_scalar_by_product_indices(335, 1.0, 154, 410, 1.0);s.store_mul(333, 335, 413);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_sub_from_scalar_scaled_mul_mixed_ia(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);s.store_square(276, 278);}
        s.b[2908] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(2908, if s.b[2908] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && s.b[2907]) && s.b[2908]) {s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);}
        if (((s.v[2625] != 0.0) && s.b[2907]) && (!s.b[2908])) {s.store_sqrt_add(275, 277, 276);s.store_sub(274, 275, 278);}
        if ((s.v[2625] != 0.0) && s.b[2907]) {s.store_powf(273, 274, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div(116, 272, 273);s.store_mul(335, 116, 155);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_sub_div_lhs_indices(404, 335, 337, 397);s.store_sub(336, 402, 404);s.store_mul(398, 413, 336);s.copy_ad(354, 398);s.copy_ad(2890, 404);}
        s.b[2909] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));s.store_scalar(2909, if s.b[2909] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2909]) {s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2909])) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));}
        if ((s.v[2625] != 0.0) && (!s.b[2907])) {s.store_mul_add_rhs(116, 154, 89, 397);}
        s.b[2910] = (s.v[116] >= 3.0);s.store_scalar(2910, if s.b[2910] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2910]) {s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_158(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2910]) {s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2910])) {s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), 1.0, 434, 434, 9.0);s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);}
        s.b[2911] = (p[33] > 0.0);s.store_scalar(2911, if s.b[2911] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {s.store_offset_add(442, 402, 397, 0.1);s.store_mul(222, 405, 229);s.store_mul(443, 405, 229);s.store_mul(334, 156, 213);s.store_mul(444, 154, 442);s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);}
        s.b[2912] = (p[33] == 2.0);s.store_scalar(2912, if s.b[2912] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2912]) {s.store_offset_sub(781, 444, 447, (-1.0));s.store_scale(782, 444, 4.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2912]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2912]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && (!s.b[2912])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {s.store_sub(444, 444, 447);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_159(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) {s.copy_ad(445, 116);}
        s.b[2913] = (p[33] == 2.0);s.store_scalar(2913, if s.b[2913] { 1.0 } else { 0.0 });s.b[2914] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));s.store_scalar(2914, if s.b[2914] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);s.store_square(722, 781);s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2915] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2915, if s.b[2915] { 1.0 } else { 0.0 });s.b[2916] = (2.0 == 1.0);s.store_scalar(2916, if s.b[2916] { 1.0 } else { 0.0 });
        if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && s.b[2916]) {s.store_scalar(720, 1.0);}
        s.b[2917] = (2.0 == 2.0);s.store_scalar(2917, if s.b[2917] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && (!s.b[2916])) && s.b[2917]) {s.store_scalar(720, 2.0);}
        s.b[2918] = (2.0 == 4.0);s.store_scalar(2918, if s.b[2918] { 1.0 } else { 0.0 });
        if (((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && (!s.b[2916])) && (!s.b[2917])) && s.b[2918]) {s.store_scalar(720, 3.0);}
        s.b[2919] = (2.0 == 8.0);s.store_scalar(2919, if s.b[2919] { 1.0 } else { 0.0 });
        if ((((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && (!s.b[2916])) && (!s.b[2917])) && (!s.b[2918])) && s.b[2919]) {s.store_scalar(720, 4.0);}
        if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) {s.store_scalar(719, 0.0);}
        let mut t11: usize = 0;
        while {
            let t10: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;
            if t11 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t11, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && s.b[2915]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) && (!s.b[2915])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);}
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && s.b[2914]) {
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && s.b[2913]) && (!s.b[2914])) {s.copy_ad(116, 445);s.store_scalar(335, 1.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2911]) && (!s.b[2913])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }
        s.b[2920] = (p[33] == 1.0);s.store_scalar(2920, if s.b[2920] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[2921] = (s.v[411] > 0.0);s.store_scalar(2921, if s.b[2921] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2921]) {s.store_sub_from_scalar(336, p[334], 411);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && (!s.b[2921])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[2922] = (s.v[336] < 0.0);s.store_scalar(2922, if s.b[2922] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && (!s.b[2921])) && s.b[2922]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && (!s.b[2921])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2923] = (s.v[336] < 0.0);s.store_scalar(2923, if s.b[2923] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2923]) {s.store_scalar(336, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_160(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2923]) {s.store_scalar(343, 0.0);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2883, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_add(414, 404, 397);s.store_mul_sub_rhs(333, 419, 414, 418);}
        s.b[2924] = (s.v[333] < 60.0);s.store_scalar(2924, if s.b[2924] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2924]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && (!s.b[2924])) {s.store_sub(416, 414, 418);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) {s.store_mul(415, 154, 416);}
        s.b[2925] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));s.store_scalar(2925, if s.b[2925] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2920]) && s.b[2925]) {s.store_primal_offset(2889, 2889, 1.0);s.copy_ad(116, 447);}
        if ((s.v[2625] != 0.0) && (!s.b[2907])) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[2926] = (((s.v[116]) as f64).abs() > 1e-6);s.store_scalar(2926, if s.b[2926] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2926]) {s.store_add_offset_lhs_mixed_ia(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));s.store_sqrt(336, 335);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2926])) {s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));}
        if ((s.v[2625] != 0.0) && (!s.b[2907])) {s.store_mul(354, 410, 336);s.store_mul_sub_rhs(398, 413, 402, 404);s.store_div(2927, 354, 2883);}
        s.b[2929] = (p[33] == 2.0);s.store_scalar(2929, if s.b[2929] { 1.0 } else { 0.0 });s.b[2930] = ((s.v[2927] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));s.store_scalar(2930, if s.b[2930] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) {s.store_add_scaled_inputs3_indices(781, 2927, 1.0, 386, (-1.0), 386, 0.1);s.store_square(722, 781);s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2931] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2931, if s.b[2931] { 1.0 } else { 0.0 });s.b[2932] = (2.0 == 1.0);s.store_scalar(2932, if s.b[2932] { 1.0 } else { 0.0 });
        if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && s.b[2932]) {s.store_scalar(720, 1.0);}
        s.b[2933] = (2.0 == 2.0);s.store_scalar(2933, if s.b[2933] { 1.0 } else { 0.0 });
        if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && (!s.b[2932])) && s.b[2933]) {s.store_scalar(720, 2.0);}
        s.b[2934] = (2.0 == 4.0);s.store_scalar(2934, if s.b[2934] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && (!s.b[2932])) && (!s.b[2933])) && s.b[2934]) {s.store_scalar(720, 3.0);}
        s.b[2935] = (2.0 == 8.0);s.store_scalar(2935, if s.b[2935] { 1.0 } else { 0.0 });
        if (((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && (!s.b[2932])) && (!s.b[2933])) && (!s.b[2934])) && s.b[2935]) {s.store_scalar(720, 4.0);}
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) {s.store_scalar(719, 0.0);}
        let mut t13: usize = 0;
        while {
            let t12: f64 = if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t12 != 0.0
        } {
            t13 += 1;
            if t13 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t13, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && s.b[2931]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) && (!s.b[2931])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2930]) {
        }
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && (!s.b[2930])) {s.copy_ad(335, 2927);s.store_scalar(334, 1.0);}
        s.b[2936] = (s.v[334] < 1.0);s.store_scalar(2936, if s.b[2936] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_161(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2929]) && s.b[2936]) {s.store_primal_offset(2889, 2889, 2.0);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2929])) {
            if (s.v[2927] <= s.v[386]) {
                s.copy_ad(335, 2927);
            } else {
                s.copy_ad(335, 386);
            }
        }
        s.b[2937] = (s.v[2927] >= s.v[386]);s.store_scalar(2937, if s.b[2937] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && (!s.b[2929])) && s.b[2937]) {s.store_primal_offset(2889, 2889, 2.0);}
        s.b[2938] = (s.v[2889] >= 2.0);s.store_scalar(2938, if s.b[2938] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) {s.copy_ad(2928, 404);s.store_mul(354, 335, 2883);s.store_sub_div_rhs_indices(404, 402, 354, 413);}
        s.b[2939] = (p[33] == 2.0);s.store_scalar(2939, if s.b[2939] { 1.0 } else { 0.0 });s.b[2940] = ((s.v[404] > (s.v[2928] - 0.1)) && (0.1 >= 0.0));s.store_scalar(2940, if s.b[2940] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) {s.store_offset_sub(781, 404, 2928, 0.1);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2941] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2941, if s.b[2941] { 1.0 } else { 0.0 });s.b[2942] = (2.0 == 1.0);s.store_scalar(2942, if s.b[2942] { 1.0 } else { 0.0 });
        if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && s.b[2942]) {s.store_scalar(720, 1.0);}
        s.b[2943] = (2.0 == 2.0);s.store_scalar(2943, if s.b[2943] { 1.0 } else { 0.0 });
        if ((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && (!s.b[2942])) && s.b[2943]) {s.store_scalar(720, 2.0);}
        s.b[2944] = (2.0 == 4.0);s.store_scalar(2944, if s.b[2944] { 1.0 } else { 0.0 });
        if (((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && (!s.b[2942])) && (!s.b[2943])) && s.b[2944]) {s.store_scalar(720, 3.0);}
        s.b[2945] = (2.0 == 8.0);s.store_scalar(2945, if s.b[2945] { 1.0 } else { 0.0 });
        if ((((((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && (!s.b[2942])) && (!s.b[2943])) && (!s.b[2944])) && s.b[2945]) {s.store_scalar(720, 4.0);}
        if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) {s.store_scalar(719, 0.0);}
        let mut t15: usize = 0;
        while {
            let t14: f64 = if (((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t14 != 0.0
        } {
            t15 += 1;
            if t15 > Self::MAX_ANALOG_LOOP_ITERATIONS { ctx.report_analog_loop_limit("reactive stamp", t15, Self::MAX_ANALOG_LOOP_ITERATIONS); break; }
            if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && s.b[2941]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) && (!s.b[2941])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_add_offset_lhs(404, 2928, (-0.1), 780);}
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && s.b[2940]) {
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && (!s.b[2940])) {
        }
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && s.b[2939]) && (!s.b[2940])) {s.store_scalar(334, 1.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2938]) && (!s.b[2939])) {
            if (s.v[404] <= s.v[2928]) {
            } else {
                s.copy_ad(404, 2928);
            }
        }
        if ((s.v[2625] != 0.0) && (!s.b[2907])) {s.copy_ad(2890, 404);}
        s.b[2946] = (p[33] == 1.0);s.store_scalar(2946, if s.b[2946] { 1.0 } else { 0.0 });
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {s.store_scalar(79, 0.0);s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2883)), s.ad_value(155)), 2.0);}
        s.b[2947] = (s.v[411] > 0.0);s.store_scalar(2947, if s.b[2947] { 1.0 } else { 0.0 });
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2947]) {s.store_sub_from_scalar(336, p[334], 411);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2947])) {s.store_sqrt_offset_square_offset(782, 729, p[137], ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p[137], s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p[137]), 782, 0.5);}
        s.b[2948] = (s.v[336] < 0.0);s.store_scalar(2948, if s.b[2948] { 1.0 } else { 0.0 });
        if (((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2947])) && s.b[2948]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && (!s.b[2947])) {s.store_scaled_sqrt_mul(600, 651, 336, p[432]);s.store_sub_from_scalar(336, p[334], 600);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p[334] * 0.01)) * (p[334] * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2949] = (s.v[336] < 0.0);s.store_scalar(2949, if s.b[2949] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_162(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) && s.b[2949]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2625] != 0.0) && (!s.b[2907])) && s.b[2946]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2883, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
}
