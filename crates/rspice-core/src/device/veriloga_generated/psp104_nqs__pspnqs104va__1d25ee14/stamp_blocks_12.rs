#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_192(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2782] = (s.v[532] == 0.0);s.store_scalar(2782, if s.b[2782] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && s.b[2782]) {s.store_scalar(2660, 0.0);}
        s.b[2783] = (s.v[512] == 0.5);s.store_scalar(2783, if s.b[2783] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && s.b[2783]) {s.store_sqrt_mul_sub_lhs(2636, 509, 2634, 597);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2783])) {s.store_pow_mul_base_mixed_ai(2636, A::sub(s.ad_value(509), s.ad_value(2634)), 597, 512);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) {s.store_mul_div_scaled_product_mixed_iaii(2661, 579, A::sub(s.ad_value(509), s.ad_value(2634)), 594, 1.0, 2636, 1.0);}
        s.b[2784] = (((((-s.v[609]) / s.v[2661])) as f64).abs() < 230.25850929940458);s.store_scalar(2784, if s.b[2784] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && s.b[2784]) {s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2661), 1.0));}
        s.b[2785] = (((-s.v[609]) / s.v[2661]) < 0.0);s.store_scalar(2785, if s.b[2785] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2784])) && s.b[2785]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 609, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2784])) && (!s.b[2785])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 609, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) {s.store_mul_ad_product_lhs_mixed_ia(2660, 532, A::mul3(s.ad_value(833), s.ad_value(2661), s.ad_value(2661)), 2636);}
        s.b[2786] = (s.v[541] > 1000.0);s.store_scalar(2786, if s.b[2786] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && s.b[2786]) {s.store_scalar(2662, 1.0);}
        s.b[2787] = (s.v[2635] > ((-s.v[444]) * s.v[541]));s.store_scalar(2787, if s.b[2787] { 1.0 } else { 0.0 });s.b[2788] = (s.v[544] == 4.0);s.store_scalar(2788, if s.b[2788] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && s.b[2787]) && s.b[2788]) {s.store_mul_ad_product_lhs_mixed_ai(2636, A::mul3(A::square(A::mul(s.ad_value(2635), s.ad_value(615))), s.ad_value(2635), s.ad_value(615)), 2635, 615);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && s.b[2787]) && (!s.b[2788])) {s.store_pow_abs_mul_base_indices(2636, 2635, 615, 544);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && s.b[2787]) {s.store_div_from_scalar_sub_from_scalar_ad(2662, 1.0, 1.0, s.ad_value(2636));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && (!s.b[2787])) {s.store_add_scaled_product_mixed_iai(2662, 612, 1.0, A::add_scaled_inputs(s.ad_value(2635), 1.0, s.ad_value(541), s.v[444]), 618, 1.0);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) {s.store_mul_scale_offset_mixed_ia(1916, 2662, A::add_scaled_inputs4(s.ad_value(2637), 1.0, s.ad_value(2638), 1.0, s.ad_value(2645), 1.0, s.ad_value(2660), 1.0), p[29], 0.0);}
        s.b[2789] = (s.v[576] == 0.5);s.store_scalar(2789, if s.b[2789] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && s.b[2789]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(573)));}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2789])) {s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2628, 573, 576);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) {s.store_add_scaled_product_mixed_aia(1922, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2636)), p[30], 588, A::sub(s.ad_value(833), s.ad_value(2628)), p[30]);}
        s.b[2790] = (s.v[675] == 0.0);s.store_scalar(2790, if s.b[2790] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2790]) {s.store_scalar(1917, 0.0);s.store_scalar(1923, 0.0);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) {s.store_mul(2637, 565, 2627);}
        s.b[2791] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));s.store_scalar(2791, if s.b[2791] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2791]) {s.store_scalar(2638, 0.0);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {s.store_sub(2639, 571, 2633);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_193(
        s: &mut Scratch,
    ) {
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));}
        s.b[2792] = (s.v[513] == 0.5);s.store_scalar(2792, if s.b[2792] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && s.b[2792]) {s.store_scalar(2641, 0.0);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && (!s.b[2792])) {s.store_mul_scale_offset(2641, A::add(A::div_scaled_product(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2640)), 1.0), s.ad_value(2640)), A::scale(s.ad_value(513), 2.0), -1.0, 1.0);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {s.store_add(2642, 2640, 2641);}
        s.b[2793] = (s.v[513] == 0.5);s.store_scalar(2793, if s.b[2793] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && s.b[2793]) {s.store_sqrt_mul(2636, 2639, 598);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && (!s.b[2793])) {s.store_pow_mul_base_indices(2636, 2639, 598, 513);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {s.store_mul(2643, 592, 2636);s.store_mul_ad_product_lhs_mixed_ia(2644, 562, A::offset(s.ad_value(2630), (-1.0)), 2643);s.store_mul3_lhs(2638, 524, 2644, 2642);}
        s.b[2794] = (s.v[527] == 0.0);s.store_scalar(2794, if s.b[2794] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2794]) {s.store_scalar(2645, 0.0);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) {s.store_mul_div_scaled_product_indices(2646, 607, 2643, 577, 1.0, 2639, 1.0);s.store_div_scaled_inputs_indices(2647, 604, 0.666666666666667, 2646, 1.0);s.store_square(2648, 2647);s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);s.store_sqrt(2650, 2649);s.store_mul(2651, 2649, 2650);}
        s.b[2795] = (((-s.v[513]) * s.v[580]) == (-1.0));s.store_scalar(2795, if s.b[2795] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && s.b[2795]) {s.store_div_from_scalar_offset_product(2652, 1.0, 2646, 2651, 1.0);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2795])) {s.store_pow_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) {s.store_div_scaled_product_add_scaled_denominator_indices(2653, 2642, 2652, 1.0, 2642, 1.0, 2652, 1.0, 1.0);s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);s.store_add_scaled_value_products_mixed_aiiii(2656, A::mul3(s.ad_value(604), s.ad_value(2647), s.ad_value(2650)), 1.0, 604, 2649, (-1.0), 2646, 2651, 0.5);s.store_mul_scale_offset_indices(2657, 2654, 2655, 1.0, (-1.0));s.store_square(2618, 2657);}
        s.b[2796] = (s.v[2657] > 0.0);s.store_scalar(2796, if s.b[2796] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && s.b[2796]) {s.store_div_from_scalar_offset_scaled_input(2619, 1.0, 2657, s.v[372], 1.0);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2796])) {s.store_div_from_scalar_sub_from_scalar_ad(2619, 1.0, 1.0, A::scale(s.ad_value(2657), s.v[372]));}
        s.b[2797] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));s.store_scalar(2797, if s.b[2797] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && s.b[2797]) {s.store_exp_sub(2636, 2656, 2618);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2797])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) {s.store_mul_mixed_ai(2620, A::add_scaled_inputs_product(s.ad_value(2619), 0.29214664, A::square(s.ad_value(2619)), s.v[373], A::square(s.ad_value(2619)), s.ad_value(2619), s.v[374]), 2636);}
        s.b[2798] = (s.v[2657] > 0.0);s.store_scalar(2798, if s.b[2798] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && s.b[2798]) {s.copy_ad(2658, 2620);}
        s.b[2799] = (s.v[2656] > (-230.25850929940458));s.store_scalar(2799, if s.b[2799] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2798])) && s.b[2799]) {s.store_exp(2636, 2656);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2798])) && (!s.b[2799])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2798])) {s.store_sub_scaled_inputs(2658, 2636, 2.0, 2620, 1.0);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) {s.store_div_scaled_product_indices(2659, 604, 2658, (1.772453850905516 * 0.5), 2654, 1.0);s.store_mul_product3_indices(2645, 527, 2644, 2659, 2653, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_194(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2800] = (s.v[533] == 0.0);s.store_scalar(2800, if s.b[2800] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2800]) {s.store_scalar(2660, 0.0);}
        s.b[2801] = (s.v[513] == 0.5);s.store_scalar(2801, if s.b[2801] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && s.b[2801]) {s.store_sqrt_mul_sub_lhs(2636, 510, 2634, 598);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2801])) {s.store_pow_mul_base_mixed_ai(2636, A::sub(s.ad_value(510), s.ad_value(2634)), 598, 513);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) {s.store_mul_div_scaled_product_mixed_iaii(2661, 580, A::sub(s.ad_value(510), s.ad_value(2634)), 595, 1.0, 2636, 1.0);}
        s.b[2802] = (((((-s.v[610]) / s.v[2661])) as f64).abs() < 230.25850929940458);s.store_scalar(2802, if s.b[2802] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && s.b[2802]) {s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2661), 1.0));}
        s.b[2803] = (((-s.v[610]) / s.v[2661]) < 0.0);s.store_scalar(2803, if s.b[2803] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2802])) && s.b[2803]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 610, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2802])) && (!s.b[2803])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 610, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) {s.store_mul_ad_product_lhs_mixed_ia(2660, 533, A::mul3(s.ad_value(833), s.ad_value(2661), s.ad_value(2661)), 2636);}
        s.b[2804] = (s.v[641] > 1000.0);s.store_scalar(2804, if s.b[2804] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2804]) {s.store_scalar(2662, 1.0);}
        s.b[2805] = (s.v[2635] > ((-s.v[444]) * s.v[641]));s.store_scalar(2805, if s.b[2805] { 1.0 } else { 0.0 });s.b[2806] = (s.v[545] == 4.0);s.store_scalar(2806, if s.b[2806] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && s.b[2805]) && s.b[2806]) {s.store_mul_ad_product_lhs_mixed_ai(2636, A::mul3(A::square(A::mul(s.ad_value(2635), s.ad_value(616))), s.ad_value(2635), s.ad_value(616)), 2635, 616);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && s.b[2805]) && (!s.b[2806])) {s.store_pow_abs_mul_base_indices(2636, 2635, 616, 545);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && s.b[2805]) {s.store_div_from_scalar_sub_from_scalar_ad(2662, 1.0, 1.0, s.ad_value(2636));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && (!s.b[2805])) {s.store_add_scaled_product_mixed_iai(2662, 613, 1.0, A::add_scaled_inputs(s.ad_value(2635), 1.0, s.ad_value(641), s.v[444]), 619, 1.0);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) {s.store_mul_scale_offset_mixed_ia(1917, 2662, A::add_scaled_inputs4(s.ad_value(2637), 1.0, s.ad_value(2638), 1.0, s.ad_value(2645), 1.0, s.ad_value(2660), 1.0), p[29], 0.0);}
        s.b[2807] = (s.v[635] == 1.0);s.store_scalar(2807, if s.b[2807] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            if (s.v[833] < s.v[550]) {
                if (((s.v[833] - s.v[550]) / s.v[551]) < (-37.0)) {
                    s.copy_ad(2663, 550);
                } else {
                    s.store_add_scaled_product_mixed_iai(2663, 550, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(833), 1.0, s.ad_value(550), (-1.0), s.ad_value(551), 1.0)), 551, 1.0);
                }
            } else {
                if (((s.v[833] - s.v[550]) / s.v[551]) > 37.0) {
                    s.copy_ad(2663, 833);
                } else {
                    s.store_add_scaled_product_mixed_iai(2663, 833, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(550), 1.0, s.ad_value(833), (-1.0), s.ad_value(551), 1.0)), 551, 1.0);
                }
            }
        }
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {s.store_primal_scaled_mul(2621, 684, 684, 4.0);s.store_primal_div(2622, 684, 685);s.store_add_scaled_product_indices(2623, 2663, 1.0, 684, 2622, 1.0);s.store_add(2624, 685, 2623);s.store_sub(2625, 685, 2623);s.store_sqrt_square_add(2626, 2625, 2621);s.store_div_scaled_product_add_scaled_denominator_indices(2664, 2663, 685, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
        s.b[2808] = (s.v[577] == 0.5);s.store_scalar(2808, if s.b[2808] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_195(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && s.b[2808]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(574)));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && (!s.b[2808])) {s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2664, 574, 577);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {s.store_add_scaled_product_mixed_aia(1923, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2636)), p[30], 589, A::sub(s.ad_value(2663), s.ad_value(2664)), p[30]);s.store_add_scaled_inputs3_indices(2663, 833, 1.0, 550, 1.0, 2663, -1.0);s.store_primal_scaled_mul(2621, 684, 684, 4.0);s.store_primal_div(2622, 684, 685);s.store_add_scaled_product_indices(2623, 2663, 1.0, 684, 2622, 1.0);s.store_add(2624, 685, 2623);s.store_sub(2625, 685, 2623);s.store_sqrt_square_add(2626, 2625, 2621);s.store_div_scaled_product_add_scaled_denominator_indices(2664, 2663, 685, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
        s.b[2809] = (s.v[630] == 0.5);s.store_scalar(2809, if s.b[2809] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && s.b[2809]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(629)));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && (!s.b[2809])) {s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2664, 629, 630);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {s.store_add_scaled_product_mixed_aia(472, A::mul_sub_from_scalar_rhs(s.ad_value(633), 1.0, s.ad_value(2636)), p[30], 634, A::sub(s.ad_value(2663), s.ad_value(2664)), p[30]);s.store_add(1923, 1923, 472);}
        s.b[2810] = (s.v[577] == 0.5);s.store_scalar(2810, if s.b[2810] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) && s.b[2810]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(574)));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) && (!s.b[2810])) {s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2628, 574, 577);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) {s.store_add_scaled_product_mixed_aia(1923, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2636)), p[30], 589, A::sub(s.ad_value(833), s.ad_value(2628)), p[30]);}
        if (s.b[2665] && (!s.b[2666])) {s.store_add_scaled_products3_indices(849, 673, 1915, 1.0, 674, 1916, 1.0, 675, 1917, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_196(
        s: &mut Scratch,
    ) {
        s.store_scalar(1942, 0.0);s.store_scalar(1943, 0.0);s.store_scalar(1944, 0.0);s.store_scalar(1945, 0.0);s.store_scalar(1946, 0.0);s.store_scalar(1947, 0.0);s.store_scalar(1948, 0.0);s.store_scalar(1949, 0.0);s.store_scalar(1950, 0.0);s.store_scalar(1951, 0.0);s.store_scalar(1952, 0.0);s.store_scalar(1953, 0.0);s.store_scalar(1954, 0.0);s.store_scalar(1955, 0.0);s.store_scalar(1956, 0.0);s.store_scalar(1957, 0.0);s.store_scalar(1958, 0.0);s.store_scalar(1959, 0.0);s.b[2811] = (s.v[1] != 0.0);s.store_scalar(2811, if s.b[2811] { 1.0 } else { 0.0 });
        if s.b[2811] {s.store_scalar(1988, 0.0);s.store_scalar(1992, 0.0);s.store_scalar(1986, 0.0);s.store_scalar(1987, 0.0);s.store_scalar(1993, 0.0);s.store_scalar(1969, 0.0);s.store_scalar(1970, 0.0);s.store_scalar(1971, 0.0);s.store_scalar(1972, 0.0);s.store_scalar(1973, 0.0);s.store_scalar(1974, 0.0);s.store_scalar(1975, 0.0);s.store_scalar(1976, 0.0);s.store_scalar(1977, 0.0);s.store_scalar(1960, 0.0);s.store_scalar(1961, 0.0);s.store_scalar(1962, 0.0);s.store_scalar(1963, 0.0);s.store_scalar(1964, 0.0);s.store_scalar(1965, 0.0);s.store_scalar(1966, 0.0);s.store_scalar(1967, 0.0);s.store_scalar(1968, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_197(
        s: &mut Scratch,
    ) {
        s.b[2812] = (s.v[1890] > 0.0);s.store_scalar(2812, if s.b[2812] { 1.0 } else { 0.0 });s.b[2813] = (s.v[1] == 1.0);s.store_scalar(2813, if s.b[2813] { 1.0 } else { 0.0 });
        if ((s.b[2811] && s.b[2812]) && s.b[2813]) {s.store_add_scaled_product_mixed_iai(1960, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.5, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2814] = (((s.v[1960]) as f64).abs() <= s.v[1933]);s.store_scalar(2814, if s.b[2814] { 1.0 } else { 0.0 });
        if (((s.b[2811] && s.b[2812]) && s.b[2813]) && s.b[2814]) {s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2815] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);s.store_scalar(2815, if s.b[2815] { 1.0 } else { 0.0 });
        if ((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && s.b[2815]) {s.store_exp_neg_input(2027, 1960);}
        s.b[2816] = ((-s.v[1960]) < 0.0);s.store_scalar(2816, if s.b[2816] { 1.0 } else { 0.0 });
        if (((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && (!s.b[2815])) && s.b[2816]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && (!s.b[2815])) && (!s.b[2816])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1960)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));}
        s.b[2817] = (s.v[1960] > s.v[1933]);s.store_scalar(2817, if s.b[2817] { 1.0 } else { 0.0 });
        if ((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && s.b[2817]) {s.store_neg(1996, 1996);}
        if ((s.b[2811] && s.b[2812]) && s.b[2813]) {s.store_add_scaled_product_right_sub(1942, 1996, (-1.0), 1937, 1890, 1960, -1.0);}
        s.b[2818] = (s.v[1] == 2.0);s.store_scalar(2818, if s.b[2818] { 1.0 } else { 0.0 });
        if (((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) {s.store_add_scaled_product_mixed_iai(1960, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.3333333333333333, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2819] = (((s.v[1960]) as f64).abs() <= s.v[1933]);s.store_scalar(2819, if s.b[2819] { 1.0 } else { 0.0 });
        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && s.b[2819]) {s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2820] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);s.store_scalar(2820, if s.b[2820] { 1.0 } else { 0.0 });
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && s.b[2820]) {s.store_exp_neg_input(2027, 1960);}
        s.b[2821] = ((-s.v[1960]) < 0.0);s.store_scalar(2821, if s.b[2821] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && s.b[2821]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && (!s.b[2821])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1960)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));}
        s.b[2822] = (s.v[1960] > s.v[1933]);s.store_scalar(2822, if s.b[2822] { 1.0 } else { 0.0 });
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && s.b[2822]) {s.store_neg(1996, 1996);}
        if (((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) {s.store_add_scaled_product_right_sub(1942, 1996, (-1.0), 1937, 1890, 1960, -1.0);s.store_add_scaled_product_mixed_iai(1961, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.6666666666666666, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2823] = (((s.v[1961]) as f64).abs() <= s.v[1933]);s.store_scalar(2823, if s.b[2823] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_198(
        s: &mut Scratch,
    ) {
        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && s.b[2823]) {s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2824] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);s.store_scalar(2824, if s.b[2824] { 1.0 } else { 0.0 });
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && s.b[2824]) {s.store_exp_neg_input(2027, 1961);}
        s.b[2825] = ((-s.v[1961]) < 0.0);s.store_scalar(2825, if s.b[2825] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && (!s.b[2824])) && s.b[2825]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && (!s.b[2824])) && (!s.b[2825])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1961)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));}
        s.b[2826] = (s.v[1961] > s.v[1933]);s.store_scalar(2826, if s.b[2826] { 1.0 } else { 0.0 });
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && s.b[2826]) {s.store_neg(1996, 1996);}
        if (((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) {s.store_add_scaled_product_right_sub(1943, 1996, (-1.0), 1937, 1890, 1961, -1.0);}
        s.b[2827] = (s.v[831] < 0.0);s.store_scalar(2827, if s.b[2827] { 1.0 } else { 0.0 });
        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && s.b[2827]) {s.copy_ad(2027, 1942);s.copy_ad(1942, 1943);s.copy_ad(1943, 2027);}
        s.b[2828] = (s.v[1] == 3.0);s.store_scalar(2828, if s.b[2828] { 1.0 } else { 0.0 });
        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {s.store_add_scaled_product_mixed_iai(1960, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.25, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2829] = (((s.v[1960]) as f64).abs() <= s.v[1933]);s.store_scalar(2829, if s.b[2829] { 1.0 } else { 0.0 });
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2829]) {s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2830] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);s.store_scalar(2830, if s.b[2830] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && s.b[2830]) {s.store_exp_neg_input(2027, 1960);}
        s.b[2831] = ((-s.v[1960]) < 0.0);s.store_scalar(2831, if s.b[2831] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && (!s.b[2830])) && s.b[2831]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && (!s.b[2830])) && (!s.b[2831])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1960)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));}
        s.b[2832] = (s.v[1960] > s.v[1933]);s.store_scalar(2832, if s.b[2832] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && s.b[2832]) {s.store_neg(1996, 1996);}
        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {s.store_add_scaled_product_right_sub(1942, 1996, (-1.0), 1937, 1890, 1960, -1.0);s.store_add_scaled_product_mixed_iai(1961, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.5, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2833] = (((s.v[1961]) as f64).abs() <= s.v[1933]);s.store_scalar(2833, if s.b[2833] { 1.0 } else { 0.0 });
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2833]) {s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2834] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);s.store_scalar(2834, if s.b[2834] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && s.b[2834]) {s.store_exp_neg_input(2027, 1961);}
        s.b[2835] = ((-s.v[1961]) < 0.0);s.store_scalar(2835, if s.b[2835] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_199(
        s: &mut Scratch,
    ) {
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && (!s.b[2834])) && s.b[2835]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && (!s.b[2834])) && (!s.b[2835])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1961)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));}
        s.b[2836] = (s.v[1961] > s.v[1933]);s.store_scalar(2836, if s.b[2836] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && s.b[2836]) {s.store_neg(1996, 1996);}
        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {s.store_add_scaled_product_right_sub(1943, 1996, (-1.0), 1937, 1890, 1961, -1.0);s.store_add_scaled_product_mixed_iai(1962, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.75, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2837] = (((s.v[1962]) as f64).abs() <= s.v[1933]);s.store_scalar(2837, if s.b[2837] { 1.0 } else { 0.0 });
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2837]) {s.store_mul_ad_affine_product_rhs(1996, 1962, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1962), 1.0, A::scale(s.ad_value(1962), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2838] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);s.store_scalar(2838, if s.b[2838] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && s.b[2838]) {s.store_exp_neg_input(2027, 1962);}
        s.b[2839] = ((-s.v[1962]) < 0.0);s.store_scalar(2839, if s.b[2839] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && (!s.b[2838])) && s.b[2839]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1962)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && (!s.b[2838])) && (!s.b[2839])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1962)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));}
        s.b[2840] = (s.v[1962] > s.v[1933]);s.store_scalar(2840, if s.b[2840] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && s.b[2840]) {s.store_neg(1996, 1996);}
        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {s.store_add_scaled_product_right_sub(1944, 1996, (-1.0), 1937, 1890, 1962, -1.0);}
        s.b[2841] = (s.v[831] < 0.0);s.store_scalar(2841, if s.b[2841] { 1.0 } else { 0.0 });
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2841]) {s.copy_ad(2027, 1942);s.copy_ad(1942, 1944);s.copy_ad(1944, 2027);}
        s.b[2842] = (s.v[1] == 5.0);s.store_scalar(2842, if s.b[2842] { 1.0 } else { 0.0 });
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {s.store_add_scaled_product_mixed_iai(1960, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.16666666666666666, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2843] = (((s.v[1960]) as f64).abs() <= s.v[1933]);s.store_scalar(2843, if s.b[2843] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2843]) {s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2844] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);s.store_scalar(2844, if s.b[2844] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && s.b[2844]) {s.store_exp_neg_input(2027, 1960);}
        s.b[2845] = ((-s.v[1960]) < 0.0);s.store_scalar(2845, if s.b[2845] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && (!s.b[2844])) && s.b[2845]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && (!s.b[2844])) && (!s.b[2845])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1960)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));}
        s.b[2846] = (s.v[1960] > s.v[1933]);s.store_scalar(2846, if s.b[2846] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_200(
        s: &mut Scratch,
    ) {
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && s.b[2846]) {s.store_neg(1996, 1996);}
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {s.store_add_scaled_product_right_sub(1942, 1996, (-1.0), 1937, 1890, 1960, -1.0);s.store_add_scaled_product_mixed_iai(1961, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.3333333333333333, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2847] = (((s.v[1961]) as f64).abs() <= s.v[1933]);s.store_scalar(2847, if s.b[2847] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2847]) {s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2848] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);s.store_scalar(2848, if s.b[2848] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && s.b[2848]) {s.store_exp_neg_input(2027, 1961);}
        s.b[2849] = ((-s.v[1961]) < 0.0);s.store_scalar(2849, if s.b[2849] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && (!s.b[2848])) && s.b[2849]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && (!s.b[2848])) && (!s.b[2849])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1961)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));}
        s.b[2850] = (s.v[1961] > s.v[1933]);s.store_scalar(2850, if s.b[2850] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && s.b[2850]) {s.store_neg(1996, 1996);}
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {s.store_add_scaled_product_right_sub(1943, 1996, (-1.0), 1937, 1890, 1961, -1.0);s.store_add_scaled_product_mixed_iai(1962, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.5, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2851] = (((s.v[1962]) as f64).abs() <= s.v[1933]);s.store_scalar(2851, if s.b[2851] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2851]) {s.store_mul_ad_affine_product_rhs(1996, 1962, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1962), 1.0, A::scale(s.ad_value(1962), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2852] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);s.store_scalar(2852, if s.b[2852] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && s.b[2852]) {s.store_exp_neg_input(2027, 1962);}
        s.b[2853] = ((-s.v[1962]) < 0.0);s.store_scalar(2853, if s.b[2853] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && (!s.b[2852])) && s.b[2853]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1962)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && (!s.b[2852])) && (!s.b[2853])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1962)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));}
        s.b[2854] = (s.v[1962] > s.v[1933]);s.store_scalar(2854, if s.b[2854] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && s.b[2854]) {s.store_neg(1996, 1996);}
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {s.store_add_scaled_product_right_sub(1944, 1996, (-1.0), 1937, 1890, 1962, -1.0);s.store_add_scaled_product_mixed_iai(1963, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.6666666666666666, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2855] = (((s.v[1963]) as f64).abs() <= s.v[1933]);s.store_scalar(2855, if s.b[2855] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2855]) {s.store_mul_ad_affine_product_rhs(1996, 1963, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1963), 1.0, A::scale(s.ad_value(1963), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_201(
        s: &mut Scratch,
    ) {
        s.b[2856] = ((((-s.v[1963])) as f64).abs() < 230.25850929940458);s.store_scalar(2856, if s.b[2856] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && s.b[2856]) {s.store_exp_neg_input(2027, 1963);}
        s.b[2857] = ((-s.v[1963]) < 0.0);s.store_scalar(2857, if s.b[2857] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && (!s.b[2856])) && s.b[2857]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1963)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && (!s.b[2856])) && (!s.b[2857])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1963)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0)));}
        s.b[2858] = (s.v[1963] > s.v[1933]);s.store_scalar(2858, if s.b[2858] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && s.b[2858]) {s.store_neg(1996, 1996);}
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {s.store_add_scaled_product_right_sub(1945, 1996, (-1.0), 1937, 1890, 1963, -1.0);s.store_add_scaled_product_mixed_iai(1964, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.8333333333333333, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2859] = (((s.v[1964]) as f64).abs() <= s.v[1933]);s.store_scalar(2859, if s.b[2859] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2859]) {s.store_mul_ad_affine_product_rhs(1996, 1964, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1964), 1.0, A::scale(s.ad_value(1964), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2860] = ((((-s.v[1964])) as f64).abs() < 230.25850929940458);s.store_scalar(2860, if s.b[2860] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && s.b[2860]) {s.store_exp_neg_input(2027, 1964);}
        s.b[2861] = ((-s.v[1964]) < 0.0);s.store_scalar(2861, if s.b[2861] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && (!s.b[2860])) && s.b[2861]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1964)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && (!s.b[2860])) && (!s.b[2861])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1964)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0)));}
        s.b[2862] = (s.v[1964] > s.v[1933]);s.store_scalar(2862, if s.b[2862] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && s.b[2862]) {s.store_neg(1996, 1996);}
        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {s.store_add_scaled_product_right_sub(1946, 1996, (-1.0), 1937, 1890, 1964, -1.0);}
        s.b[2863] = (s.v[831] < 0.0);s.store_scalar(2863, if s.b[2863] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2863]) {s.copy_ad(2027, 1942);s.copy_ad(1942, 1946);s.copy_ad(1946, 2027);s.copy_ad(2027, 1943);s.copy_ad(1943, 1945);s.copy_ad(1945, 2027);}
        s.b[2864] = (s.v[1] == 9.0);s.store_scalar(2864, if s.b[2864] { 1.0 } else { 0.0 });
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_mixed_iai(1960, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.1, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2865] = (((s.v[1960]) as f64).abs() <= s.v[1933]);s.store_scalar(2865, if s.b[2865] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2865]) {s.store_mul_ad_affine_product_rhs(1996, 1960, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1960), 1.0, A::scale(s.ad_value(1960), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2866] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);s.store_scalar(2866, if s.b[2866] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && s.b[2866]) {s.store_exp_neg_input(2027, 1960);}
        s.b[2867] = ((-s.v[1960]) < 0.0);s.store_scalar(2867, if s.b[2867] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && (!s.b[2866])) && s.b[2867]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1960)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_202(
        s: &mut Scratch,
    ) {
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && (!s.b[2866])) && (!s.b[2867])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1960)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));}
        s.b[2868] = (s.v[1960] > s.v[1933]);s.store_scalar(2868, if s.b[2868] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && s.b[2868]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1942, 1996, (-1.0), 1937, 1890, 1960, -1.0);s.store_add_scaled_product_mixed_iai(1961, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.2, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2869] = (((s.v[1961]) as f64).abs() <= s.v[1933]);s.store_scalar(2869, if s.b[2869] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2869]) {s.store_mul_ad_affine_product_rhs(1996, 1961, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1961), 1.0, A::scale(s.ad_value(1961), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2870] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);s.store_scalar(2870, if s.b[2870] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && s.b[2870]) {s.store_exp_neg_input(2027, 1961);}
        s.b[2871] = ((-s.v[1961]) < 0.0);s.store_scalar(2871, if s.b[2871] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && (!s.b[2870])) && s.b[2871]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1961)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && (!s.b[2870])) && (!s.b[2871])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1961)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));}
        s.b[2872] = (s.v[1961] > s.v[1933]);s.store_scalar(2872, if s.b[2872] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && s.b[2872]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1943, 1996, (-1.0), 1937, 1890, 1961, -1.0);s.store_add_scaled_product_mixed_iai(1962, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.3, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2873] = (((s.v[1962]) as f64).abs() <= s.v[1933]);s.store_scalar(2873, if s.b[2873] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2873]) {s.store_mul_ad_affine_product_rhs(1996, 1962, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1962), 1.0, A::scale(s.ad_value(1962), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2874] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);s.store_scalar(2874, if s.b[2874] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && s.b[2874]) {s.store_exp_neg_input(2027, 1962);}
        s.b[2875] = ((-s.v[1962]) < 0.0);s.store_scalar(2875, if s.b[2875] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && (!s.b[2874])) && s.b[2875]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1962)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && (!s.b[2874])) && (!s.b[2875])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1962)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));}
        s.b[2876] = (s.v[1962] > s.v[1933]);s.store_scalar(2876, if s.b[2876] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && s.b[2876]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1944, 1996, (-1.0), 1937, 1890, 1962, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_203(
        s: &mut Scratch,
    ) {
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_mixed_iai(1963, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.4, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2877] = (((s.v[1963]) as f64).abs() <= s.v[1933]);s.store_scalar(2877, if s.b[2877] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2877]) {s.store_mul_ad_affine_product_rhs(1996, 1963, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1963), 1.0, A::scale(s.ad_value(1963), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2878] = ((((-s.v[1963])) as f64).abs() < 230.25850929940458);s.store_scalar(2878, if s.b[2878] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && s.b[2878]) {s.store_exp_neg_input(2027, 1963);}
        s.b[2879] = ((-s.v[1963]) < 0.0);s.store_scalar(2879, if s.b[2879] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && (!s.b[2878])) && s.b[2879]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1963)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && (!s.b[2878])) && (!s.b[2879])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1963)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0)));}
        s.b[2880] = (s.v[1963] > s.v[1933]);s.store_scalar(2880, if s.b[2880] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && s.b[2880]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1945, 1996, (-1.0), 1937, 1890, 1963, -1.0);s.store_add_scaled_product_mixed_iai(1964, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.5, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2881] = (((s.v[1964]) as f64).abs() <= s.v[1933]);s.store_scalar(2881, if s.b[2881] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2881]) {s.store_mul_ad_affine_product_rhs(1996, 1964, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1964), 1.0, A::scale(s.ad_value(1964), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2882] = ((((-s.v[1964])) as f64).abs() < 230.25850929940458);s.store_scalar(2882, if s.b[2882] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && s.b[2882]) {s.store_exp_neg_input(2027, 1964);}
        s.b[2883] = ((-s.v[1964]) < 0.0);s.store_scalar(2883, if s.b[2883] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && (!s.b[2882])) && s.b[2883]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1964)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && (!s.b[2882])) && (!s.b[2883])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1964)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0)));}
        s.b[2884] = (s.v[1964] > s.v[1933]);s.store_scalar(2884, if s.b[2884] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && s.b[2884]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1946, 1996, (-1.0), 1937, 1890, 1964, -1.0);s.store_add_scaled_product_mixed_iai(1965, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.6, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2885] = (((s.v[1965]) as f64).abs() <= s.v[1933]);s.store_scalar(2885, if s.b[2885] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2885]) {s.store_mul_ad_affine_product_rhs(1996, 1965, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1965), 1.0, A::scale(s.ad_value(1965), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2886] = ((((-s.v[1965])) as f64).abs() < 230.25850929940458);s.store_scalar(2886, if s.b[2886] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_204(
        s: &mut Scratch,
    ) {
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && s.b[2886]) {s.store_exp_neg_input(2027, 1965);}
        s.b[2887] = ((-s.v[1965]) < 0.0);s.store_scalar(2887, if s.b[2887] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && (!s.b[2886])) && s.b[2887]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1965)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && (!s.b[2886])) && (!s.b[2887])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1965)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1965)), (-1.0)));}
        s.b[2888] = (s.v[1965] > s.v[1933]);s.store_scalar(2888, if s.b[2888] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && s.b[2888]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1947, 1996, (-1.0), 1937, 1890, 1965, -1.0);s.store_add_scaled_product_mixed_iai(1966, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.7, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2889] = (((s.v[1966]) as f64).abs() <= s.v[1933]);s.store_scalar(2889, if s.b[2889] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2889]) {s.store_mul_ad_affine_product_rhs(1996, 1966, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1966), 1.0, A::scale(s.ad_value(1966), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2890] = ((((-s.v[1966])) as f64).abs() < 230.25850929940458);s.store_scalar(2890, if s.b[2890] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && s.b[2890]) {s.store_exp_neg_input(2027, 1966);}
        s.b[2891] = ((-s.v[1966]) < 0.0);s.store_scalar(2891, if s.b[2891] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && (!s.b[2890])) && s.b[2891]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1966)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && (!s.b[2890])) && (!s.b[2891])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1966)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1966)), (-1.0)));}
        s.b[2892] = (s.v[1966] > s.v[1933]);s.store_scalar(2892, if s.b[2892] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && s.b[2892]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1948, 1996, (-1.0), 1937, 1890, 1966, -1.0);s.store_add_scaled_product_mixed_iai(1967, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.8, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2893] = (((s.v[1967]) as f64).abs() <= s.v[1933]);s.store_scalar(2893, if s.b[2893] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2893]) {s.store_mul_ad_affine_product_rhs(1996, 1967, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1967), 1.0, A::scale(s.ad_value(1967), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2894] = ((((-s.v[1967])) as f64).abs() < 230.25850929940458);s.store_scalar(2894, if s.b[2894] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && s.b[2894]) {s.store_exp_neg_input(2027, 1967);}
        s.b[2895] = ((-s.v[1967]) < 0.0);s.store_scalar(2895, if s.b[2895] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && (!s.b[2894])) && s.b[2895]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1967)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && (!s.b[2894])) && (!s.b[2895])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1967)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1967)), (-1.0)));}
        s.b[2896] = (s.v[1967] > s.v[1933]);s.store_scalar(2896, if s.b[2896] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && s.b[2896]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1949, 1996, (-1.0), 1937, 1890, 1967, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_205(
        s: &mut Scratch,
    ) {
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_mixed_iai(1968, 1934, 1.0, A::mul_sub_from_scalar_rhs(s.ad_value(1909), 1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_div_scaled_inputs_lhs(s.ad_value(1893), 2.0, s.ad_value(1909), 1.0, 0.9, s.ad_value(1936))))), 1932, 1.0);}
        s.b[2897] = (((s.v[1968]) as f64).abs() <= s.v[1933]);s.store_scalar(2897, if s.b[2897] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2897]) {s.store_mul_ad_affine_product_rhs(1996, 1968, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1968), 1.0, A::scale(s.ad_value(1968), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2898] = ((((-s.v[1968])) as f64).abs() < 230.25850929940458);s.store_scalar(2898, if s.b[2898] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && s.b[2898]) {s.store_exp_neg_input(2027, 1968);}
        s.b[2899] = ((-s.v[1968]) < 0.0);s.store_scalar(2899, if s.b[2899] { 1.0 } else { 0.0 });
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && (!s.b[2898])) && s.b[2899]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1968)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && (!s.b[2898])) && (!s.b[2899])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1968)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) {s.store_mul_sqrt_mixed_ia(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1968)), (-1.0)));}
        s.b[2900] = (s.v[1968] > s.v[1933]);s.store_scalar(2900, if s.b[2900] { 1.0 } else { 0.0 });
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && s.b[2900]) {s.store_neg(1996, 1996);}
        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {s.store_add_scaled_product_right_sub(1950, 1996, (-1.0), 1937, 1890, 1968, -1.0);}
        s.b[2901] = (s.v[831] < 0.0);s.store_scalar(2901, if s.b[2901] { 1.0 } else { 0.0 });
        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2901]) {s.copy_ad(2027, 1942);s.copy_ad(1942, 1950);s.copy_ad(1950, 2027);s.copy_ad(2027, 1943);s.copy_ad(1943, 1949);s.copy_ad(1949, 2027);s.copy_ad(2027, 1944);s.copy_ad(1944, 1948);s.copy_ad(1948, 2027);s.copy_ad(2027, 1945);s.copy_ad(1945, 1947);s.copy_ad(1947, 2027);}
        s.store_scalar(1983, 0.0);s.store_scalar(1984, 0.0);s.store_scalar(1978, 0.0);s.store_scalar(1979, 0.0);s.b[2902] = (s.v[1] != 0.0);s.store_scalar(2902, if s.b[2902] { 1.0 } else { 0.0 });
        if s.b[2902] {s.store_sub_mixed_ia(1983, 1934, A::mul3_scaled_output(s.ad_value(831), s.ad_value(1893), s.ad_value(1932), 0.5));s.store_add_product3_rhs_indices(1984, 1934, 831, 1893, 1932, 0.5);s.store_scalar(1978, 0.0);s.store_scalar(1979, 0.0);}
        s.b[2903] = (s.v[1983] > 0.0);s.store_scalar(2903, if s.b[2903] { 1.0 } else { 0.0 });s.b[2904] = (((s.v[1983]) as f64).abs() <= s.v[1933]);s.store_scalar(2904, if s.b[2904] { 1.0 } else { 0.0 });
        if ((s.b[2902] && s.b[2903]) && s.b[2904]) {s.store_mul_ad_affine_product_rhs(1997, 1983, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1983), 1.0, A::scale(s.ad_value(1983), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2905] = ((((-s.v[1983])) as f64).abs() < 230.25850929940458);s.store_scalar(2905, if s.b[2905] { 1.0 } else { 0.0 });
        if (((s.b[2902] && s.b[2903]) && (!s.b[2904])) && s.b[2905]) {s.store_exp_neg_input(2027, 1983);}
        s.b[2906] = ((-s.v[1983]) < 0.0);s.store_scalar(2906, if s.b[2906] { 1.0 } else { 0.0 });
        if ((((s.b[2902] && s.b[2903]) && (!s.b[2904])) && (!s.b[2905])) && s.b[2906]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1983)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2902] && s.b[2903]) && (!s.b[2904])) && (!s.b[2905])) && (!s.b[2906])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1983)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_206(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[2902] && s.b[2903]) && (!s.b[2904])) {s.store_mul_sqrt_mixed_ia(1997, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1983)), (-1.0)));}
        s.b[2907] = (s.v[1983] > s.v[1933]);s.store_scalar(2907, if s.b[2907] { 1.0 } else { 0.0 });
        if (((s.b[2902] && s.b[2903]) && (!s.b[2904])) && s.b[2907]) {s.store_neg(1997, 1997);}
        if (s.b[2902] && s.b[2903]) {s.store_add_scaled_product_right_sub(1978, 1997, (-1.0), 1937, 1890, 1983, -1.0);}
        s.b[2908] = (s.v[1984] > 0.0);s.store_scalar(2908, if s.b[2908] { 1.0 } else { 0.0 });s.b[2909] = (((s.v[1984]) as f64).abs() <= s.v[1933]);s.store_scalar(2909, if s.b[2909] { 1.0 } else { 0.0 });
        if ((s.b[2902] && s.b[2908]) && s.b[2909]) {s.store_mul_ad_affine_product_rhs(1997, 1984, s.ad_value(1889), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1984), 1.0, A::scale(s.ad_value(1984), 0.16666666666666666), 0.16666666666666666)), (-0.70710678), 0.0);}
        s.b[2910] = ((((-s.v[1984])) as f64).abs() < 230.25850929940458);s.store_scalar(2910, if s.b[2910] { 1.0 } else { 0.0 });
        if (((s.b[2902] && s.b[2908]) && (!s.b[2909])) && s.b[2910]) {s.store_exp_neg_input(2027, 1984);}
        s.b[2911] = ((-s.v[1984]) < 0.0);s.store_scalar(2911, if s.b[2911] { 1.0 } else { 0.0 });
        if ((((s.b[2902] && s.b[2908]) && (!s.b[2909])) && (!s.b[2910])) && s.b[2911]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1984)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2902] && s.b[2908]) && (!s.b[2909])) && (!s.b[2910])) && (!s.b[2911])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(1984)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2902] && s.b[2908]) && (!s.b[2909])) {s.store_mul_sqrt_mixed_ia(1997, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1984)), (-1.0)));}
        s.b[2912] = (s.v[1984] > s.v[1933]);s.store_scalar(2912, if s.b[2912] { 1.0 } else { 0.0 });
        if (((s.b[2902] && s.b[2908]) && (!s.b[2909])) && s.b[2912]) {s.store_neg(1997, 1997);}
        if (s.b[2902] && s.b[2908]) {s.store_add_scaled_product_right_sub(1979, 1997, (-1.0), 1937, 1890, 1984, -1.0);}
        s.b[2913] = (s.v[831] > 0.0);s.store_scalar(2913, if s.b[2913] { 1.0 } else { 0.0 });s.b[2914] = (s.v[300] > 0.0);s.store_scalar(2914, if s.b[2914] { 1.0 } else { 0.0 });s.b[2915] = (s.v[301] > 0.0);s.store_scalar(2915, if s.b[2915] { 1.0 } else { 0.0 });s.b[2916] = (s.v[302] > 0.0);s.store_scalar(2916, if s.b[2916] { 1.0 } else { 0.0 });s.b[2917] = (s.v[303] > 0.0);s.store_scalar(2917, if s.b[2917] { 1.0 } else { 0.0 });s.b[2918] = (s.v[304] > 0.0);s.store_scalar(2918, if s.b[2918] { 1.0 } else { 0.0 });s.b[2919] = (s.v[305] > 0.0);s.store_scalar(2919, if s.b[2919] { 1.0 } else { 0.0 });s.b[2920] = (s.v[306] > 0.0);s.store_scalar(2920, if s.b[2920] { 1.0 } else { 0.0 });s.store_scaled_voltage(1969, ctx, nodes, Some(12), None, s.v[3]);s.store_scaled_voltage(1970, ctx, nodes, Some(13), None, s.v[3]);s.store_scaled_voltage(1971, ctx, nodes, Some(14), None, s.v[3]);s.store_scaled_voltage(1972, ctx, nodes, Some(15), None, s.v[3]);s.store_scaled_voltage(1973, ctx, nodes, Some(16), None, s.v[3]);s.store_scaled_voltage(1974, ctx, nodes, Some(17), None, s.v[3]);s.store_scaled_voltage(1975, ctx, nodes, Some(18), None, s.v[3]);s.store_scaled_voltage(1976, ctx, nodes, Some(19), None, s.v[3]);s.store_scaled_voltage(1977, ctx, nodes, Some(20), None, s.v[3]);s.store_scalar(1995, 0.0);s.b[2921] = (s.v[1] != 0.0);s.store_scalar(2921, if s.b[2921] { 1.0 } else { 0.0 });
        if s.b[2921] {s.store_div_scaled_product3_by_product_indices(1995, 307, 1888, 716, 1.0, 1904, 1906, 1.0);s.store_mul_ad_product_lhs_mixed_ai(2018, A::square(s.ad_value(1907)), 1888, 1888);}
        s.b[2922] = (s.v[1] == 1.0);s.store_scalar(2922, if s.b[2922] { 1.0 } else { 0.0 });
        if (s.b[2921] && s.b[2922]) {s.store_sub(1992, 1979, 1978);s.store_add_scaled_inputs3_indices(1993, 1978, 6.0, 1979, 6.0, 1969, (-12.0));}
        s.b[2923] = (s.v[1] == 2.0);s.store_scalar(2923, if s.b[2923] { 1.0 } else { 0.0 });
        if ((s.b[2921] && (!s.b[2922])) && s.b[2923]) {s.store_add_scaled_inputs4_indices(1992, 1978, ((-7.0) * 0.2), 1969, ((-3.0) * 0.2), 1970, (12.0 * 0.2), 1979, ((-2.0) * 0.2));s.store_add_scaled_inputs4_indices(1993, 1978, ((-4.0) * ((-18.0) / 5.0)), 1969, (9.0 * ((-18.0) / 5.0)), 1970, ((-6.0) * ((-18.0) / 5.0)), 1979, ((-18.0) / 5.0));}
        s.b[2924] = (s.v[1] == 3.0);s.store_scalar(2924, if s.b[2924] { 1.0 } else { 0.0 });
        if (((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && s.b[2924]) {s.store_scaled_add_mixed_ai(1992, A::add_scaled_inputs4(s.ad_value(1978), (-13.0), s.ad_value(1969), (-6.0), s.ad_value(1970), 24.0, s.ad_value(1971), (-6.0)), 1979, 0.14285714285714285);s.store_add_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs4(s.ad_value(1978), 180.0, s.ad_value(1969), (-408.0), s.ad_value(1970), 288.0, s.ad_value(1971), (-72.0)), 0.14285714285714285, 1979, (12.0 * 0.14285714285714285));}
        s.b[2925] = (s.v[1] == 5.0);s.store_scalar(2925, if s.b[2925] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_207(
        s: &mut Scratch,
    ) {
        if ((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && s.b[2925]) {s.store_add_scaled_inputs_mixed_ai(1992, A::add(A::sub_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1978), (-181.0), s.ad_value(1969), (-84.0), s.ad_value(1972), 24.0, s.ad_value(1973), (-6.0)), 1.0, s.ad_value(1971), 90.0), s.ad_value(1979)), 0.015384615384615385, 1970, (336.0 * 0.015384615384615385));s.store_add_scaled_inputs_mixed_ai(1993, A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1972), 432.0, s.ad_value(1973), (-108.0), s.ad_value(1971), (-1620.0), s.ad_value(1979), 18.0), 1.0, s.ad_value(1978), 3762.0), 1.0, s.ad_value(1969), 8532.0), 0.015384615384615385, 1970, (6048.0 * 0.015384615384615385));}
        s.b[2926] = (s.v[1] == 9.0);s.store_scalar(2926, if s.b[2926] { 1.0 } else { 0.0 });
        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && s.b[2926]) {s.store_sub_scaled_inputs_mixed_ai(1992, A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::sub_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1974), 1680.0, s.ad_value(1972), 23400.0, s.ad_value(1979), 5.0, s.ad_value(1971), (-87330.0)), 1.0, s.ad_value(1976), 120.0), 1.0, s.ad_value(1975), 450.0), 1.0, s.ad_value(1969), 81480.0), 1.0, s.ad_value(1970), 325920.0), 1.0, s.ad_value(1978), 175565.0), 2.6434745829918846e-5, s.ad_value(1977), (30.0 * 2.6434745829918846e-5)), 1.0, 1973, (30.0 / 181.0));s.store_sub_scaled_inputs_mixed_ai(1993, A::add_scaled_inputs(A::sub_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs(A::add_scaled_inputs4(s.ad_value(1975), (-13500.0), s.ad_value(1972), 702000.0, s.ad_value(1971), (-2619900.0), s.ad_value(1969), (-13793100.0)), 1.0, s.ad_value(1970), 9777600.0), 1.0, s.ad_value(1978), 6081750.0), 1.0, s.ad_value(1979), 150.0), 1.0, s.ad_value(1976), 3600.0), 1.0, s.ad_value(1977), 900.0), 2.6434745829918846e-5, s.ad_value(1974), (50400.0 * 2.6434745829918846e-5)), 1.0, 1973, (900.0 / 181.0));}
        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && (!s.b[2926])) {s.store_scalar(1992, 0.0);s.store_scalar(1993, 0.0);}
        if s.b[2921] {s.store_add_div_lhs_indices(2027, 1969, 1937, 1890);}
        s.b[2927] = (((s.v[2027]) as f64).abs() <= s.v[1941]);s.store_scalar(2927, if s.b[2927] { 1.0 } else { 0.0 });
        if (s.b[2921] && s.b[2927]) {s.store_div(2016, 2027, 1940);}
        s.b[2928] = (s.v[2027] < (-s.v[1941]));s.store_scalar(2928, if s.b[2928] { 1.0 } else { 0.0 });
        if ((s.b[2921] && (!s.b[2927])) && s.b[2928]) {s.store_neg(1999, 2027);s.store_div_scaled_inputs_indices(2000, 1999, 1.25, 1940, 1.0);s.store_scaled_sub_offset_sqrt_square_offset(2001, 2000, 10.0, (-6.0), 64.0, 0.5);s.store_add_scaled_sub_square_product_mixed_ia(2002, 1999, 2001, 1.0, 1939, A::offset(s.ad_value(2001), 1.0), 1.0);s.store_add_scaled_inputs3_indices(2003, 1999, 2.0, 2001, (-2.0), 1939, -1.0);s.store_sub_ln_div_lhs(2004, 2002, 1939, 2001);s.store_add(824, 2002, 2003);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2004, A::sub_scaled_inputs(A::square(s.ad_value(2003)), 0.5, s.ad_value(2002), 1.0), 1.0);}
    }
}
