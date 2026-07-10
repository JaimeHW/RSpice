#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_133(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2689] = (2.0 == 2.0);s.store_scalar(2689, if s.b[2689] { 1.0 } else { 0.0 });
        if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (!s.b[2688])) && s.b[2689]) {s.store_scalar(720, 2.0);}
        s.b[2690] = (2.0 == 4.0);s.store_scalar(2690, if s.b[2690] { 1.0 } else { 0.0 });
        if ((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (!s.b[2688])) && (!s.b[2689])) && s.b[2690]) {s.store_scalar(720, 3.0);}
        s.b[2691] = (2.0 == 8.0);s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });
        if (((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (!s.b[2688])) && (!s.b[2689])) && (!s.b[2690])) && s.b[2691]) {s.store_scalar(720, 4.0);}
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {s.store_scalar(719, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) && (!s.b[2687])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2686]) {
        }
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && (!s.b[2686])) {s.copy_ad(335, 2683);s.store_scalar(334, 1.0);}
        s.b[2692] = (s.v[334] < 1.0);s.store_scalar(2692, if s.b[2692] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2685]) && s.b[2692]) {s.store_primal_offset(2645, 2645, 2.0);}
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2685])) {
            if (s.v[2683] <= s.v[386]) {
                s.copy_ad(335, 2683);
            } else {
                s.copy_ad(335, 386);
            }
        }
        s.b[2693] = (s.v[2683] >= s.v[386]);s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && (!s.b[2685])) && s.b[2693]) {s.store_primal_offset(2645, 2645, 2.0);}
        s.b[2694] = (s.v[2645] >= 2.0);s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) {s.copy_ad(2684, 404);s.store_mul(354, 335, 2639);s.store_sub_div_rhs_indices(404, 402, 354, 413);}
        s.b[2695] = (p.p33 == 2.0);s.store_scalar(2695, if s.b[2695] { 1.0 } else { 0.0 });s.b[2696] = ((s.v[404] > (s.v[2684] - 0.1)) && (0.1 >= 0.0));s.store_scalar(2696, if s.b[2696] { 1.0 } else { 0.0 });
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {s.store_offset_sub(781, 404, 2684, 0.1);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2697] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2697, if s.b[2697] { 1.0 } else { 0.0 });s.b[2698] = (2.0 == 1.0);s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });
        if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && s.b[2698]) {s.store_scalar(720, 1.0);}
        s.b[2699] = (2.0 == 2.0);s.store_scalar(2699, if s.b[2699] { 1.0 } else { 0.0 });
        if ((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) && s.b[2699]) {s.store_scalar(720, 2.0);}
        s.b[2700] = (2.0 == 4.0);s.store_scalar(2700, if s.b[2700] { 1.0 } else { 0.0 });
        if (((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) && (!s.b[2699])) && s.b[2700]) {s.store_scalar(720, 3.0);}
        s.b[2701] = (2.0 == 8.0);s.store_scalar(2701, if s.b[2701] { 1.0 } else { 0.0 });
        if ((((((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (!s.b[2698])) && (!s.b[2699])) && (!s.b[2700])) && s.b[2701]) {s.store_scalar(720, 4.0);}
        if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) {s.store_scalar(719, 0.0);}
        let mut t3: usize = 0;
        while {
            let t2: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t2 != 0.0
        } {
            t3 += 1;assert!(t3 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && s.b[2697]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) && (!s.b[2697])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_add_offset_lhs(404, 2684, (-0.1), 780);}
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && s.b[2696]) {
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) {
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && s.b[2695]) && (!s.b[2696])) {s.store_scalar(334, 1.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2694]) && (!s.b[2695])) {
            if (s.v[404] <= s.v[2684]) {
            } else {
                s.copy_ad(404, 2684);
            }
        }
        if ((s.v[2623] != 0.0) && (!s.b[2663])) {s.copy_ad(2646, 404);}
        s.b[2702] = (p.p33 == 1.0);s.store_scalar(2702, if s.b[2702] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_scalar(79, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_134(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2639)), s.ad_value(155)), 2.0);}
        s.b[2703] = (s.v[411] > 0.0);s.store_scalar(2703, if s.b[2703] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2703]) {s.store_sub_from_scalar(336, p.p334, 411);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2703])) {s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);}
        s.b[2704] = (s.v[336] < 0.0);s.store_scalar(2704, if s.b[2704] { 1.0 } else { 0.0 });
        if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2703])) && s.b[2704]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2703])) {s.store_scaled_sqrt_mul(600, 651, 336, p.p432);s.store_sub_from_scalar(336, p.p334, 600);}
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2705] = (s.v[336] < 0.0);s.store_scalar(2705, if s.b[2705] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2705]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2639, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_135(
        s: &mut ReactiveScratch,
    ) {
        let mut t6: usize = 0;
        while {
            let t4: f64 = (s.v[421] + 1.0);let t5: f64 = if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (s.v[97] <= t4)) { 1.0 } else { 0.0 };
            t5 != 0.0
        } {
            t6 += 1;assert!(t6 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[2707] = (s.v[333] < 60.0);s.store_scalar(2707, if s.b[2707] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2707]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2707])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_mul(415, 154, 416);}
            s.b[2708] = (s.v[116] < 0.0);s.store_scalar(2708, if s.b[2708] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2708]) {s.store_scalar(334, (-0.7071067811865475));s.store_mul(223, 116, 334);s.store_mul(420, 154, 334);}
            s.b[2709] = (s.v[116] < 1e-6);s.store_scalar(2709, if s.b[2709] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && s.b[2709]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(338, 334, 336);}
            s.b[2710] = (s.v[338] > 0.0);s.store_scalar(2710, if s.b[2710] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && s.b[2709]) && s.b[2710]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);}
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && s.b[2709]) && (!s.b[2710])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && (!s.b[2709])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));}
            s.b[2711] = (s.v[338] > 0.0);s.store_scalar(2711, if s.b[2711] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && (!s.b[2709])) && s.b[2711]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);}
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2708])) && (!s.b[2709])) && (!s.b[2711])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            s.b[2712] = (s.v[116] < 0.0);s.store_scalar(2712, if s.b[2712] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2712]) {s.store_scalar(214, 0.0);s.store_scalar(215, 0.0);s.store_neg(216, 223);s.store_neg(217, 420);}
            s.b[2713] = (s.v[116] < 60.0);s.store_scalar(2713, if s.b[2713] { 1.0 } else { 0.0 });s.b[2714] = (s.v[116] < 5e-5);s.store_scalar(2714, if s.b[2714] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && s.b[2713]) && s.b[2714]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            if ((((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && s.b[2713]) && (!s.b[2714])) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && (!s.b[2713])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[2715] = (s.v[214] > 0.0);s.store_scalar(2715, if s.b[2715] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && s.b[2715]) {s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_add_product_indices(217, 215, 0.5, 420, 223, (2.0 * 0.5), 216, 1.0);}
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2712])) && (!s.b[2715])) {s.copy_ad(216, 223);s.copy_ad(217, 420);}
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[2716] = (s.v[79] == 1.0);s.store_scalar(2716, if s.b[2716] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && s.b[2716]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2717] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2717, if s.b[2717] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) && s.b[2717]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) {s.store_add(404, 404, 236);}
            s.b[2718] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2718, if s.b[2718] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) && (!s.b[2716])) && s.b[2718]) {s.store_scalar(79, 1.0);}
            if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_136(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2663])) && s.b[2702]) {s.store_mul(2637, 982, 223);s.store_mul(2638, 2639, 2637);s.store_offset_div(100, 2638, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        s.b[2720] = (p.p33 == 4.0);s.store_scalar(2720, if s.b[2720] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));s.store_div(334, 394, 409);s.store_square(405, 334);s.store_mul(222, 405, 229);s.copy_ad(404, 2646);s.store_scalar(79, 0.0);s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2639)), s.ad_value(155)), 2.0);}
        s.b[2721] = (s.v[411] > 0.0);s.store_scalar(2721, if s.b[2721] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2721]) {s.store_sub_from_scalar(336, p.p334, 411);}
        if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2721])) {s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);}
        s.b[2722] = (s.v[336] < 0.0);s.store_scalar(2722, if s.b[2722] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2721])) && s.b[2722]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2721])) {s.store_scaled_sqrt_mul(600, 651, 336, p.p432);s.store_sub_from_scalar(336, p.p334, 600);}
        if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2723] = (s.v[336] < 0.0);s.store_scalar(2723, if s.b[2723] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2723]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((s.v[2623] != 0.0) && s.b[2720]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2639, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_137(
        s: &mut ReactiveScratch,
    ) {
        let mut t9: usize = 0;
        while {
            let t7: f64 = (s.v[421] + 1.0);let t8: f64 = if (((s.v[2623] != 0.0) && s.b[2720]) && (s.v[97] <= t7)) { 1.0 } else { 0.0 };
            t8 != 0.0
        } {
            t9 += 1;assert!(t9 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[2725] = (s.v[333] < 60.0);s.store_scalar(2725, if s.b[2725] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2725]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2725])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_mul(415, 154, 416);}
            s.b[2726] = (((s.v[116]) as f64).abs() < 1e-6);s.store_scalar(2726, if s.b[2726] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2726]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(2647, 334, 336);s.store_mul_add_scaled_product_rhs_indices(2648, 154, 335, 1.0, 417, 337, (-1.0));}
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2726])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(2647, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));s.store_mul_sub_mixed_iaa(2648, 154, A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335)));}
            s.b[2727] = (((s.v[116]) as f64).abs() < 5e-5);s.store_scalar(2727, if s.b[2727] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2727]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            s.b[2728] = (((s.v[116]) as f64).abs() < 60.0);s.store_scalar(2728, if s.b[2728] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2727])) && s.b[2728]) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2727])) && (!s.b[2728])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[2729] = (s.v[214] > 0.0);s.store_scalar(2729, if s.b[2729] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2729]) {s.store_sqrt_add(216, 2647, 214);s.store_div_scaled_inputs2_indices(217, 2648, 0.5, 215, 0.5, 216, 1.0);}
            s.b[2730] = (s.v[2647] > 0.0);s.store_scalar(2730, if s.b[2730] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2729])) && s.b[2730]) {s.store_sqrt(216, 2647);s.store_div_scaled_inputs_indices(217, 2648, 0.5, 216, 1.0);}
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2729])) && (!s.b[2730])) {s.store_scalar(216, 0.0);s.store_scalar(217, 0.0);}
            if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_scale(216, 216, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_scale(217, 217, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[2731] = (s.v[79] > 0.0);s.store_scalar(2731, if s.b[2731] { 1.0 } else { 0.0 });
            if (((s.v[2623] != 0.0) && s.b[2720]) && s.b[2731]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2732] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2732, if s.b[2732] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) && s.b[2732]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if (((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) {s.store_add(404, 404, 236);}
            s.b[2733] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2733, if s.b[2733] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && s.b[2720]) && (!s.b[2731])) && s.b[2733]) {s.store_primal_offset(79, 79, 2.0);}
            if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_138(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.v[2623] != 0.0) && s.b[2720]) {
            if (s.v[2647] >= 0.0) {
                s.store_scaled_sqrt(223, 2647, (if (s.v[116] >= 0.0) { 1.0 } else { (-1.0) }));
            } else {
                s.store_scalar(223, 0.0);
            }
        }
        if ((s.v[2623] != 0.0) && s.b[2720]) {s.store_mul(2637, 982, 223);s.store_mul(2638, 2639, 2637);s.store_offset_div(100, 2638, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        if (s.v[2623] != 0.0) {s.store_sub(399, 398, 354);}
        s.b[2735] = (s.v[407] < 0.0);s.store_scalar(2735, if s.b[2735] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2735]) {s.store_neg(407, 407);}
        s.b[2736] = (p.p55 == 0.0);s.store_scalar(2736, if s.b[2736] { 1.0 } else { 0.0 });s.b[2737] = (p.p50 == 0.0);s.store_scalar(2737, if s.b[2737] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) && s.b[2737]) {s.store_neg(2640, 404);}
        if ((((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) && (!s.b[2737])) {s.copy_ad(2640, 396);}
        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {s.store_sqrt_offset_square_offset(782, 2640, p.p137, ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(2640), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(2640), p.p137), 782, 0.5);}
        s.b[2738] = (s.v[336] < 0.0);s.store_scalar(2738, if s.b[2738] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) && s.b[2738]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {s.store_scaled_sqrt_mul(600, 651, 336, p.p432);s.store_add_scaled_inputs3_indices(781, 407, 1.0, 600, (-1.0), 407, (-0.1));s.store_scaled_mul(782, 407, 407, (4.0 * 0.1));}
        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((s.v[2623] != 0.0) && s.b[2735]) && s.b[2736]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(603, 407, 1.0, 781, (-0.5), 782, (-0.5));s.store_sub(407, 407, 603);}
        s.b[2739] = (1.0 == 1.0);s.store_scalar(2739, if s.b[2739] { 1.0 } else { 0.0 });s.b[2740] = (1.0 == 2.0);s.store_scalar(2740, if s.b[2740] { 1.0 } else { 0.0 });s.b[2741] = (1.0 == 3.0);s.store_scalar(2741, if s.b[2741] { 1.0 } else { 0.0 });s.b[2742] = (1.0 == 4.0);s.store_scalar(2742, if s.b[2742] { 1.0 } else { 0.0 });s.b[2743] = (p.p55 == 1.0);s.store_scalar(2743, if s.b[2743] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2739]) && s.b[2743]) {s.store_scale(338, 407, s.v[635]);}
        if (((s.v[2623] != 0.0) && s.b[2739]) && (!s.b[2743])) {s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));}
        if ((s.v[2623] != 0.0) && s.b[2739]) {s.store_mul(353, 338, 398);s.store_mul(356, 338, 354);}
        if ((s.v[2623] != 0.0) && (s.b[2740] && (!s.b[2739]))) {s.store_scale(338, 407, (s.v[635] * s.v[526]));s.store_mul(351, 338, 398);s.store_mul(359, 338, 354);}
        s.b[2744] = (p.p55 == 1.0);s.store_scalar(2744, if s.b[2744] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) && s.b[2744]) {s.store_scale(338, 407, s.v[635]);}
        if (((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) && (!s.b[2744])) {s.store_scale(338, 407, (s.v[635] * (1.0 - s.v[526])));}
        if ((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) {s.copy_ad(697, 404);}
        s.b[2745] = (p.p430 == 0.0);s.store_scalar(2745, if s.b[2745] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) && s.b[2745]) {s.copy_ad(698, 354);}
        if ((s.v[2623] != 0.0) && (s.b[2741] && (!(s.b[2739] || s.b[2740])))) {s.store_mul(352, 338, 398);s.store_mul(355, 338, 354);s.copy_ad(816, 355);}
        if ((s.v[2623] != 0.0) && (s.b[2742] && (!((s.b[2739] || s.b[2740]) || s.b[2741])))) {s.store_scale(338, 407, (s.v[635] * s.v[526]));s.store_mul(350, 338, 398);s.store_mul(358, 338, 354);}
        s.store_scalar(2623, 0.0);s.b[2746] = (2.0 == 1.0);s.store_scalar(2746, if s.b[2746] { 1.0 } else { 0.0 });s.b[2747] = (2.0 == 2.0);s.store_scalar(2747, if s.b[2747] { 1.0 } else { 0.0 });s.b[2748] = (2.0 == 3.0);s.store_scalar(2748, if s.b[2748] { 1.0 } else { 0.0 });s.b[2749] = (2.0 == 4.0);s.store_scalar(2749, if s.b[2749] { 1.0 } else { 0.0 });s.b[2750] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));s.store_scalar(2750, if s.b[2750] { 1.0 } else { 0.0 });
        if (s.b[2746] && s.b[2750]) {s.store_scalar(2623, 1.0);s.store_scalar(2621, 1.0);s.store_sub(395, 731, 728);s.store_neg(396, 728);s.store_scalar(409, s.v[460]);s.store_scalar(407, p.p66);s.store_scalar(411, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_139(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.b[2746] && s.b[2750]) {s.copy_ad(410, 687);s.store_scalar(413, s.v[188]);}
        s.b[2751] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));s.store_scalar(2751, if s.b[2751] { 1.0 } else { 0.0 });
        if ((s.b[2747] && (!s.b[2746])) && s.b[2751]) {s.store_scalar(2623, 1.0);s.store_sub(395, 734, 735);s.store_neg(396, 735);}
        s.b[2752] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));s.store_scalar(2752, if s.b[2752] { 1.0 } else { 0.0 });
        if ((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) {s.store_scalar(2623, 1.0);s.store_scalar(2624, 1.0);s.store_sub(395, 731, 728);s.store_sub(396, 729, 728);s.store_scalar(409, s.v[459]);s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));s.copy_ad(411, 384);s.copy_ad(410, 686);s.copy_ad(413, 412);s.store_neg(407, 407);}
        s.b[2753] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));s.store_scalar(2753, if s.b[2753] { 1.0 } else { 0.0 });
        if (((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) {s.store_neg(407, 407);s.store_scalar(335, p.p63);s.store_offset_div_scaled_product_indices(996, 335, 335, 1.0, 651, 1.0, (-p.p137));}
        s.b[2754] = (p.p113 > 0.0);s.store_scalar(2754, if s.b[2754] { 1.0 } else { 0.0 });s.b[2755] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));s.store_scalar(2755, if s.b[2755] { 1.0 } else { 0.0 });
        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && s.b[2755]) {
        }
        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && (!s.b[2755])) {s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));}
        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && (!s.b[2755])) {s.store_mul(784, 783, 396);s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);s.store_powf(782, 781, (1.0 / p.p113));s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);}
        if ((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) {s.store_sqrt_offset_square_offset(782, 396, p.p137, ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);}
        s.b[2756] = (s.v[336] < 0.0);s.store_scalar(2756, if s.b[2756] { 1.0 } else { 0.0 });
        if (((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) && s.b[2756]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.b[2748] && (!(s.b[2746] || s.b[2747]))) && s.b[2752]) && s.b[2753]) && s.b[2754]) {s.store_scaled_sqrt_mul(600, 651, 336, p.p432);s.store_sub(407, 407, 600);}
        s.b[2757] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));s.store_scalar(2757, if s.b[2757] { 1.0 } else { 0.0 });
        if ((s.b[2749] && (!((s.b[2746] || s.b[2747]) || s.b[2748]))) && s.b[2757]) {s.store_scalar(2623, 1.0);s.store_sub(395, 734, 735);s.store_sub(396, 733, 735);}
        if (s.v[2623] != 0.0) {s.store_scalar(2765, 0.4);s.store_scalar(2766, 0.0);s.store_scalar(223, 0.0);s.store_scalar(214, 0.0);s.store_scalar(216, 0.0);s.store_scalar(232, 0.0);s.store_scalar(236, 0.0);s.store_scalar(233, 0.0);s.store_scalar(217, 0.0);s.store_scalar(420, 0.0);s.store_scalar(215, 0.0);s.store_scalar(447, 0.0);s.store_scalar(445, 0.0);s.store_scalar(446, 0.0);s.store_scalar(79, (-1.0));s.store_scalar(2767, 0.0);s.store_scalar(2768, 0.0);s.store_mul_scaled_ln_ad_rhs(2763, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2763), (-0.1));s.store_scalar(782, ((4.0 * 0.8) * 0.1));}
        if (s.v[2623] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.v[2623] != 0.0) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(2764, 781, (-0.5), 782, (-0.5), 0.8);}
        s.b[2770] = (s.v[2765] > (s.v[2764] * 0.5));s.store_scalar(2770, if s.b[2770] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2770]) {s.store_scale(2765, 2764, 0.5);}
        s.b[2771] = param_given[338];s.store_scalar(2771, if s.b[2771] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2771]) {s.store_scalar(2764, p.p338);}
        s.b[2772] = param_given[339];s.store_scalar(2772, if s.b[2772] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2772]) {s.store_scalar(2765, p.p339);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_140(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        s.b[2773] = param_given[338];s.store_scalar(2773, if s.b[2773] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2772])) && s.b[2773]) {s.store_scale(2765, 2764, 0.5);}
        s.b[2774] = (s.v[2765] > (s.v[2764] * 0.5));s.store_scalar(2774, if s.b[2774] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2774]) {s.store_scale(2765, 2764, 0.5);}
        s.b[2775] = (p.p38 == 1.0);s.store_scalar(2775, if s.b[2775] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2775]) {s.store_neg(334, 396);}
        s.b[2776] = (s.v[334] > s.v[2765]);s.store_scalar(2776, if s.b[2776] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2775]) && s.b[2776]) {s.store_sub(335, 334, 2765);s.store_sub(336, 2764, 2765);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);s.store_neg(345, 345);s.store_add(344, 2765, 333);}
        if (((s.v[2623] != 0.0) && s.b[2775]) && (!s.b[2776])) {s.copy_ad(344, 334);}
        if ((s.v[2623] != 0.0) && s.b[2775]) {s.store_neg(397, 344);}
        if ((s.v[2623] != 0.0) && (!s.b[2775])) {s.copy_ad(397, 396);}
        if (s.v[2623] != 0.0) {s.store_div(212, 410, 413);s.store_square(213, 212);s.store_sub_from_scalar(402, s.v[458], 395);s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);s.store_scalar(2759, 0.0);s.store_primal_scale(2760, 409, 1.6021918e-19);s.store_div(334, 394, 409);s.store_square(405, 334);}
        s.b[2777] = ((s.v[154] * (-s.v[397])) >= 500.0);s.store_scalar(2777, if s.b[2777] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2777]) {s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(334, 1.403592217853e217);}
        if ((s.v[2623] != 0.0) && (!s.b[2777])) {s.store_mul_scale_offset_indices(781, 154, 397, -1.0, 0.0);s.store_scalar(229, 1.0);}
        let mut tb: usize = 0;
        while {
            let ta: f64 = if (((s.v[2623] != 0.0) && (!s.b[2777])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            ta != 0.0
        } {
            tb += 1;assert!(tb <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2623] != 0.0) && (!s.b[2777])) {s.store_scale(229, 229, 1.14200738981568e26);s.store_offset(781, 781, (-60.0));}
        }
        if ((s.v[2623] != 0.0) && (!s.b[2777])) {s.store_mul_exp_rhs(229, 229, 781);s.copy_ad(334, 229);}
        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));s.store_scalar(782, (4.0 * 0.5));}
        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);}
        s.b[2778] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));s.store_scalar(2778, if s.b[2778] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);s.store_square(722, 781);s.store_square(723, 335);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2779] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2779, if s.b[2779] { 1.0 } else { 0.0 });s.b[2780] = (1.0 == 1.0);s.store_scalar(2780, if s.b[2780] { 1.0 } else { 0.0 });
        if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && s.b[2780]) {s.store_scalar(720, 1.0);}
        s.b[2781] = (1.0 == 2.0);s.store_scalar(2781, if s.b[2781] { 1.0 } else { 0.0 });
        if ((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (!s.b[2780])) && s.b[2781]) {s.store_scalar(720, 2.0);}
        s.b[2782] = (1.0 == 4.0);s.store_scalar(2782, if s.b[2782] { 1.0 } else { 0.0 });
        if (((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (!s.b[2780])) && (!s.b[2781])) && s.b[2782]) {s.store_scalar(720, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_141(
        s: &mut ReactiveScratch,
    ) {
        s.b[2783] = (1.0 == 8.0);s.store_scalar(2783, if s.b[2783] { 1.0 } else { 0.0 });
        if ((((((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (!s.b[2780])) && (!s.b[2781])) && (!s.b[2782])) && s.b[2783]) {s.store_scalar(720, 4.0);}
        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) {s.store_scalar(719, 0.0);}
        let mut td: usize = 0;
        while {
            let tc: f64 = if (((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tc != 0.0
        } {
            td += 1;assert!(td <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && s.b[2779]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) && (!s.b[2779])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 335, 726);s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);}
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && s.b[2778]) {
        }
        if (((s.v[2623] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2778])) {s.store_add(335, 402, 397);s.store_scalar(334, 1.0);}
        if ((s.v[2623] != 0.0) && (s.v[406] != 0.0)) {s.store_sub(397, 335, 402);s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);}
        s.b[2784] = (s.v[402] < s.v[403]);s.store_scalar(2784, if s.b[2784] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2784]) {s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_add_rhs(332, 154, 402, 397);s.store_div_scalar_by_product_indices(335, 1.0, 154, 410, 1.0);s.store_mul(333, 335, 413);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_sub_from_scalar_scaled_mul_mixed_ia(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);s.store_square(276, 278);}
        s.b[2785] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(2785, if s.b[2785] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2784]) && s.b[2785]) {s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);}
        if (((s.v[2623] != 0.0) && s.b[2784]) && (!s.b[2785])) {s.store_sqrt_add(275, 277, 276);s.store_sub(274, 275, 278);}
        if ((s.v[2623] != 0.0) && s.b[2784]) {s.store_powf(273, 274, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div(116, 272, 273);s.store_mul(335, 116, 155);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_sub_div_lhs_indices(404, 335, 337, 397);s.store_sub(336, 402, 404);s.store_mul(398, 413, 336);s.copy_ad(354, 398);s.copy_ad(2767, 404);}
        s.b[2786] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));s.store_scalar(2786, if s.b[2786] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2786]) {s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);}
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2786])) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));}
        if ((s.v[2623] != 0.0) && (!s.b[2784])) {s.store_mul_add_rhs(116, 154, 89, 397);}
        s.b[2787] = (s.v[116] >= 3.0);s.store_scalar(2787, if s.b[2787] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2787]) {s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_142(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2787]) {s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);}
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2787])) {s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), 1.0, 434, 434, 9.0);s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);}
        s.b[2788] = (p.p33 > 0.0);s.store_scalar(2788, if s.b[2788] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {s.store_offset_add(442, 402, 397, 0.1);s.store_mul(222, 405, 229);s.store_mul(443, 405, 229);s.store_mul(334, 156, 213);s.store_mul(444, 154, 442);s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);}
        s.b[2789] = (p.p33 == 2.0);s.store_scalar(2789, if s.b[2789] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2789]) {s.store_offset_sub(781, 444, 447, (-1.0));s.store_scale(782, 444, 4.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2789]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2789]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && (!s.b[2789])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {s.store_sub(444, 444, 447);s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_143(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);}
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) {s.copy_ad(445, 116);}
        s.b[2790] = (p.p33 == 2.0);s.store_scalar(2790, if s.b[2790] { 1.0 } else { 0.0 });s.b[2791] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));s.store_scalar(2791, if s.b[2791] { 1.0 } else { 0.0 });
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);s.store_square(722, 781);s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2792] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2792, if s.b[2792] { 1.0 } else { 0.0 });s.b[2793] = (2.0 == 1.0);s.store_scalar(2793, if s.b[2793] { 1.0 } else { 0.0 });
        if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && s.b[2793]) {s.store_scalar(720, 1.0);}
        s.b[2794] = (2.0 == 2.0);s.store_scalar(2794, if s.b[2794] { 1.0 } else { 0.0 });
        if ((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (!s.b[2793])) && s.b[2794]) {s.store_scalar(720, 2.0);}
        s.b[2795] = (2.0 == 4.0);s.store_scalar(2795, if s.b[2795] { 1.0 } else { 0.0 });
        if (((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (!s.b[2793])) && (!s.b[2794])) && s.b[2795]) {s.store_scalar(720, 3.0);}
        s.b[2796] = (2.0 == 8.0);s.store_scalar(2796, if s.b[2796] { 1.0 } else { 0.0 });
        if ((((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (!s.b[2793])) && (!s.b[2794])) && (!s.b[2795])) && s.b[2796]) {s.store_scalar(720, 4.0);}
        if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) {s.store_scalar(719, 0.0);}
        let mut tf: usize = 0;
        while {
            let te: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            te != 0.0
        } {
            tf += 1;assert!(tf <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && s.b[2792]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) && (!s.b[2792])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);}
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && s.b[2791]) {
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && s.b[2790]) && (!s.b[2791])) {s.copy_ad(116, 445);s.store_scalar(335, 1.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2788]) && (!s.b[2790])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }
        s.b[2797] = (p.p33 == 1.0);s.store_scalar(2797, if s.b[2797] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[2798] = (s.v[411] > 0.0);s.store_scalar(2798, if s.b[2798] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2798]) {s.store_sub_from_scalar(336, p.p334, 411);}
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && (!s.b[2798])) {s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);}
        s.b[2799] = (s.v[336] < 0.0);s.store_scalar(2799, if s.b[2799] { 1.0 } else { 0.0 });
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && (!s.b[2798])) && s.b[2799]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && (!s.b[2798])) {s.store_scaled_sqrt_mul(600, 651, 336, p.p432);s.store_sub_from_scalar(336, p.p334, 600);}
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2800] = (s.v[336] < 0.0);s.store_scalar(2800, if s.b[2800] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2800]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {s.copy_ad(386, 336);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_144(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {s.store_mul3_affine_lhs(418, 2760, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_add(414, 404, 397);s.store_mul_sub_rhs(333, 419, 414, 418);}
        s.b[2801] = (s.v[333] < 60.0);s.store_scalar(2801, if s.b[2801] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2801]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);}
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && (!s.b[2801])) {s.store_sub(416, 414, 418);}
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) {s.store_mul(415, 154, 416);}
        s.b[2802] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));s.store_scalar(2802, if s.b[2802] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2797]) && s.b[2802]) {s.store_primal_offset(2766, 2766, 1.0);s.copy_ad(116, 447);}
        if ((s.v[2623] != 0.0) && (!s.b[2784])) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[2803] = (((s.v[116]) as f64).abs() > 1e-6);s.store_scalar(2803, if s.b[2803] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2803]) {s.store_add_offset_lhs_mixed_ia(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));s.store_sqrt(336, 335);}
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2803])) {s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));}
        if ((s.v[2623] != 0.0) && (!s.b[2784])) {s.store_mul(354, 410, 336);s.store_mul_sub_rhs(398, 413, 402, 404);s.store_div(2804, 354, 2760);}
        s.b[2806] = (p.p33 == 2.0);s.store_scalar(2806, if s.b[2806] { 1.0 } else { 0.0 });s.b[2807] = ((s.v[2804] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));s.store_scalar(2807, if s.b[2807] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {s.store_add_scaled_inputs3_indices(781, 2804, 1.0, 386, (-1.0), 386, 0.1);s.store_square(722, 781);s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2808] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2808, if s.b[2808] { 1.0 } else { 0.0 });s.b[2809] = (2.0 == 1.0);s.store_scalar(2809, if s.b[2809] { 1.0 } else { 0.0 });
        if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && s.b[2809]) {s.store_scalar(720, 1.0);}
        s.b[2810] = (2.0 == 2.0);s.store_scalar(2810, if s.b[2810] { 1.0 } else { 0.0 });
        if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && (!s.b[2809])) && s.b[2810]) {s.store_scalar(720, 2.0);}
        s.b[2811] = (2.0 == 4.0);s.store_scalar(2811, if s.b[2811] { 1.0 } else { 0.0 });
        if ((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && (!s.b[2809])) && (!s.b[2810])) && s.b[2811]) {s.store_scalar(720, 3.0);}
        s.b[2812] = (2.0 == 8.0);s.store_scalar(2812, if s.b[2812] { 1.0 } else { 0.0 });
        if (((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && (!s.b[2809])) && (!s.b[2810])) && (!s.b[2811])) && s.b[2812]) {s.store_scalar(720, 4.0);}
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) {s.store_scalar(719, 0.0);}
        let mut t11: usize = 0;
        while {
            let t10: f64 = if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t10 != 0.0
        } {
            t11 += 1;assert!(t11 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && s.b[2808]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) && (!s.b[2808])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 386, 0.1, 0.0, 726);s.store_div_scaled_product3_indices(334, 386, 725, 726, 0.1, 770, 1.0);s.store_add_scaled_inputs3_indices(335, 386, 1.0, 386, (-0.1), 780, 1.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2807]) {
        }
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && (!s.b[2807])) {s.copy_ad(335, 2804);s.store_scalar(334, 1.0);}
        s.b[2813] = (s.v[334] < 1.0);s.store_scalar(2813, if s.b[2813] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2806]) && s.b[2813]) {s.store_primal_offset(2766, 2766, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_145(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2806])) {
            if (s.v[2804] <= s.v[386]) {
                s.copy_ad(335, 2804);
            } else {
                s.copy_ad(335, 386);
            }
        }
        s.b[2814] = (s.v[2804] >= s.v[386]);s.store_scalar(2814, if s.b[2814] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && (!s.b[2806])) && s.b[2814]) {s.store_primal_offset(2766, 2766, 2.0);}
        s.b[2815] = (s.v[2766] >= 2.0);s.store_scalar(2815, if s.b[2815] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) {s.copy_ad(2805, 404);s.store_mul(354, 335, 2760);s.store_sub_div_rhs_indices(404, 402, 354, 413);}
        s.b[2816] = (p.p33 == 2.0);s.store_scalar(2816, if s.b[2816] { 1.0 } else { 0.0 });s.b[2817] = ((s.v[404] > (s.v[2805] - 0.1)) && (0.1 >= 0.0));s.store_scalar(2817, if s.b[2817] { 1.0 } else { 0.0 });
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {s.store_offset_sub(781, 404, 2805, 0.1);s.store_square(722, 781);s.store_scalar(723, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2818] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2818, if s.b[2818] { 1.0 } else { 0.0 });s.b[2819] = (2.0 == 1.0);s.store_scalar(2819, if s.b[2819] { 1.0 } else { 0.0 });
        if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && s.b[2819]) {s.store_scalar(720, 1.0);}
        s.b[2820] = (2.0 == 2.0);s.store_scalar(2820, if s.b[2820] { 1.0 } else { 0.0 });
        if ((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) && s.b[2820]) {s.store_scalar(720, 2.0);}
        s.b[2821] = (2.0 == 4.0);s.store_scalar(2821, if s.b[2821] { 1.0 } else { 0.0 });
        if (((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && s.b[2821]) {s.store_scalar(720, 3.0);}
        s.b[2822] = (2.0 == 8.0);s.store_scalar(2822, if s.b[2822] { 1.0 } else { 0.0 });
        if ((((((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && (!s.b[2821])) && s.b[2822]) {s.store_scalar(720, 4.0);}
        if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) {s.store_scalar(719, 0.0);}
        let mut t13: usize = 0;
        while {
            let t12: f64 = if (((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t12 != 0.0
        } {
            t13 += 1;assert!(t13 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && s.b[2818]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) && (!s.b[2818])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 0.1);s.store_div_scaled_product_indices(334, 725, 726, 0.1, 770, 1.0);s.store_add_offset_lhs(404, 2805, (-0.1), 780);}
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && s.b[2817]) {
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && (!s.b[2817])) {
        }
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && s.b[2816]) && (!s.b[2817])) {s.store_scalar(334, 1.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2815]) && (!s.b[2816])) {
            if (s.v[404] <= s.v[2805]) {
            } else {
                s.copy_ad(404, 2805);
            }
        }
        if ((s.v[2623] != 0.0) && (!s.b[2784])) {s.copy_ad(2767, 404);}
        s.b[2823] = (p.p33 == 1.0);s.store_scalar(2823, if s.b[2823] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {s.store_scalar(79, 0.0);s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2760)), s.ad_value(155)), 2.0);}
        s.b[2824] = (s.v[411] > 0.0);s.store_scalar(2824, if s.b[2824] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2824]) {s.store_sub_from_scalar(336, p.p334, 411);}
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2824])) {s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);}
        s.b[2825] = (s.v[336] < 0.0);s.store_scalar(2825, if s.b[2825] { 1.0 } else { 0.0 });
        if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2824])) && s.b[2825]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2824])) {s.store_scaled_sqrt_mul(600, 651, 336, p.p432);s.store_sub_from_scalar(336, p.p334, 600);}
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2826] = (s.v[336] < 0.0);s.store_scalar(2826, if s.b[2826] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2826]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_146(
        s: &mut ReactiveScratch,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2760, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_147(
        s: &mut ReactiveScratch,
    ) {
        let mut t16: usize = 0;
        while {
            let t14: f64 = (s.v[421] + 1.0);let t15: f64 = if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (s.v[97] <= t14)) { 1.0 } else { 0.0 };
            t15 != 0.0
        } {
            t16 += 1;assert!(t16 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {s.store_add(414, 404, 397);s.store_mul(116, 154, 414);s.store_mul_sub_rhs(333, 419, 414, 418);}
            s.b[2828] = (s.v[333] < 60.0);s.store_scalar(2828, if s.b[2828] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2828]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);s.store_div_scaled_value_offset_denominator(417, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2828])) {s.store_sub(416, 414, 418);s.store_scalar(417, 1.0);}
            if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {s.store_mul(415, 154, 416);}
            s.b[2829] = (s.v[116] < 0.0);s.store_scalar(2829, if s.b[2829] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2829]) {s.store_scalar(334, (-0.7071067811865475));s.store_mul(223, 116, 334);s.store_mul(420, 154, 334);}
            s.b[2830] = (s.v[116] < 1e-6);s.store_scalar(2830, if s.b[2830] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && s.b[2830]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 415, 1.0, 415, 1.0, 415, 1.0, 415, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 415, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(415), 1.0, A::scale(s.ad_value(415), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sub(338, 334, 336);}
            s.b[2831] = (s.v[338] > 0.0);s.store_scalar(2831, if s.b[2831] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && s.b[2830]) && s.b[2831]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(417), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);}
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && s.b[2830]) && (!s.b[2831])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && (!s.b[2830])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 415);s.store_add_scaled_inputs4_indices(338, 116, 1.0, 415, (-1.0), 334, 1.0, 335, (-1.0));}
            s.b[2832] = (s.v[338] > 0.0);s.store_scalar(2832, if s.b[2832] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && (!s.b[2830])) && s.b[2832]) {s.store_sqrt(223, 338);s.store_div_scaled_product_mixed_iai(420, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(417), 1.0, s.ad_value(335))), 0.5, 223, 1.0);}
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2829])) && (!s.b[2830])) && (!s.b[2832])) {s.store_scalar(223, 0.0);s.store_scalar(420, 0.0);}
            s.b[2833] = (s.v[116] < 0.0);s.store_scalar(2833, if s.b[2833] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2833]) {s.store_scalar(214, 0.0);s.store_scalar(215, 0.0);s.store_neg(216, 223);s.store_neg(217, 420);}
            s.b[2834] = (s.v[116] < 60.0);s.store_scalar(2834, if s.b[2834] { 1.0 } else { 0.0 });s.b[2835] = (s.v[116] < 5e-5);s.store_scalar(2835, if s.b[2835] { 1.0 } else { 0.0 });
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && s.b[2834]) && s.b[2835]) {s.store_scaled_mul_ad(334, A::square(s.ad_value(116)), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (4.0), A::scale_offset(s.ad_value(116), 0.2, 1.0)), 1.0)), 1.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_scaled_lhs(s.ad_value(116), 1.0 / (2.0), A::offset(A::mul_scaled_lhs(s.ad_value(116), 1.0 / (3.0), A::scale_offset(s.ad_value(116), 0.25, 1.0)), 1.0)), 1.0, 1.0);s.store_mul(214, 222, 334);s.store_mul3_lhs(215, 222, 335, 154);}
            if ((((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && s.b[2834]) && (!s.b[2835])) {s.store_exp(227, 116);s.store_offset(335, 227, (-1.0));s.store_mul_sub_rhs(214, 222, 335, 116);s.store_mul3_lhs(215, 222, 154, 335);}
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && (!s.b[2834])) {s.store_exp_mul(231, 154, 404);s.store_mul_mixed_ia(214, 405, A::add_scaled_offset_product_rhs(s.ad_value(231), 1.0, s.ad_value(229), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 405, 154, A::sub(s.ad_value(231), s.ad_value(229)));}
            s.b[2836] = (s.v[214] > 0.0);s.store_scalar(2836, if s.b[2836] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && s.b[2836]) {s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_add_product_indices(217, 215, 0.5, 420, 223, (2.0 * 0.5), 216, 1.0);}
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2833])) && (!s.b[2836])) {s.copy_ad(216, 223);s.copy_ad(217, 420);}
            if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {s.store_add_scaled_inputs_product_indices(232, 404, 1.0, 402, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[2837] = (s.v[79] == 1.0);s.store_scalar(2837, if s.b[2837] { 1.0 } else { 0.0 });
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && s.b[2837]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[404]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(404))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2838] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2838, if s.b[2838] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) && s.b[2838]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) {s.store_add(404, 404, 236);}
            s.b[2839] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2839, if s.b[2839] { 1.0 } else { 0.0 });
            if (((((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) && (!s.b[2837])) && s.b[2839]) {s.store_scalar(79, 1.0);}
            if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_148(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2623] != 0.0) && (!s.b[2784])) && s.b[2823]) {s.store_mul(2758, 982, 223);s.store_mul(2759, 2760, 2758);s.store_offset_div(100, 2759, 410, (10.0 * 2.220446049250313e-16));s.store_mul(354, 410, 100);s.store_div_from_scalar_add_ad(335, 1.0, s.ad_value(216), s.ad_value(100));s.store_mul3_lhs(399, 410, 214, 335);s.store_add(398, 354, 399);}
        s.b[2841] = (p.p33 == 4.0);s.store_scalar(2841, if s.b[2841] { 1.0 } else { 0.0 });
        if ((s.v[2623] != 0.0) && s.b[2841]) {s.store_exp_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0));s.store_div(334, 394, 409);s.store_square(405, 334);s.store_mul(222, 405, 229);s.copy_ad(404, 2767);s.store_scalar(79, 0.0);s.store_sqrt_scaled_input_ad(982, A::mul(A::div_from_scalar(1.034943e-10, s.ad_value(2760)), s.ad_value(155)), 2.0);}
        s.b[2842] = (s.v[411] > 0.0);s.store_scalar(2842, if s.b[2842] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2841]) && s.b[2842]) {s.store_sub_from_scalar(336, p.p334, 411);}
        if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2842])) {s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);}
        s.b[2843] = (s.v[336] < 0.0);s.store_scalar(2843, if s.b[2843] { 1.0 } else { 0.0 });
        if ((((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2842])) && s.b[2843]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2623] != 0.0) && s.b[2841]) && (!s.b[2842])) {s.store_scaled_sqrt_mul(600, 651, 336, p.p432);s.store_sub_from_scalar(336, p.p334, 600);}
        if ((s.v[2623] != 0.0) && s.b[2841]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2844] = (s.v[336] < 0.0);s.store_scalar(2844, if s.b[2844] { 1.0 } else { 0.0 });
        if (((s.v[2623] != 0.0) && s.b[2841]) && s.b[2844]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((s.v[2623] != 0.0) && s.b[2841]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2760, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_scalar(97, 1.0);}
    }
}
