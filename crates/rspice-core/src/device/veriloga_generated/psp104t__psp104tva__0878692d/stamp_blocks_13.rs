#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && s.b[2645]) && s.b[2646]) {s.store_mul_ad_product_lhs_mixed_ai(2540, A::mul3(A::square(A::mul(s.ad_value(2539), s.ad_value(451))), s.ad_value(2539), s.ad_value(451)), 2539, 451);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && s.b[2645]) && (!s.b[2646])) {s.store_powf_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(451))), p.p882);}
        s.b[2647] = (s.v[474] == 1.0);s.store_scalar(2647, if s.b[2647] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {
            if (s.v[821] < p.p887) {
                if (((s.v[821] - p.p887) / p.p888) < (-37.0)) {
                    s.store_scalar(2567, p.p887);
                } else {
                    s.store_offset_scaled_ad(2567, A::ln_one_plus_exp(A::scaled_offset(s.ad_value(821), (-p.p887), 1.0 / (p.p888))), p.p888, p.p887);
                }
            } else {
                if (((s.v[821] - p.p887) / p.p888) > 37.0) {
                    s.copy_ad(2567, 821);
                } else {
                    s.store_add_scaled_inputs_mixed_ia(2567, 821, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(821), (-1.0 / (p.p888)), ((p.p887) * (1.0 / (p.p888))))), p.p888);
                }
            }
        }
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {s.store_primal_scaled_mul(2525, 658, 658, 4.0);s.store_primal_div(2526, 658, 659);s.store_add_scaled_product_indices(2527, 2567, 1.0, 658, 2526, 1.0);s.store_add(2528, 659, 2527);s.store_sub(2529, 659, 2527);s.store_sqrt_square_add(2530, 2529, 2525);s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 659, 2.0, 2528, 1.0, 2530, 1.0, 1.0);}
        s.b[2648] = (s.v[411] == 0.5);s.store_scalar(2648, if s.b[2648] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && s.b[2648]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2568), s.v[408]));}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && (!s.b[2648])) {s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2568), s.v[408])), s.v[411]);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {s.store_add_scaled_inputs3_offset_indices(1904, 2540, ((-s.v[420]) * p.p30), 2567, (s.v[423] * p.p30), 2568, ((-s.v[423]) * p.p30), (s.v[420] * p.p30));s.store_sub_offset_lhs(2567, 821, p.p887, 2567);s.store_primal_scaled_mul(2525, 658, 658, 4.0);s.store_primal_div(2526, 658, 659);s.store_add_scaled_product_indices(2527, 2567, 1.0, 658, 2526, 1.0);s.store_add(2528, 659, 2527);s.store_sub(2529, 659, 2527);s.store_sqrt_square_add(2530, 2529, 2525);s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 659, 2.0, 2528, 1.0, 2530, 1.0, 1.0);}
        s.b[2649] = (s.v[468] == 0.5);s.store_scalar(2649, if s.b[2649] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && s.b[2649]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(467)));}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && (!s.b[2649])) {s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2568, 467, 468);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {s.store_add_scaled_product_mixed_aia(473, A::mul_sub_from_scalar_rhs(s.ad_value(471), 1.0, s.ad_value(2540)), p.p30, 472, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);s.store_add(1904, 1904, 473);}
        s.b[2650] = (s.v[411] == 0.5);s.store_scalar(2650, if s.b[2650] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) && s.b[2650]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2532), s.v[408]));}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) && (!s.b[2650])) {s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[408])), s.v[411]);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) {s.store_add_scaled_inputs3_offset_indices(1904, 2540, ((-s.v[420]) * p.p30), 821, (s.v[423] * p.p30), 2532, ((-s.v[423]) * p.p30), (s.v[420] * p.p30));}
        s.b[2651] = (s.v[637] > 0.0);s.store_scalar(2651, if s.b[2651] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2569] && (!s.b[2570])) && s.b[2651]) {s.store_mul_sub_mixed_iaa(644, 637, A::pow(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), s.ad_value(638)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(638)));s.store_add(642, 543, 644);s.store_div_from_scalar(617, 1.0, 642);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2651])) {s.copy_ad(642, 543);}
        s.b[2652] = (s.v[639] > 0.0);s.store_scalar(2652, if s.b[2652] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2652]) {s.store_mul_sub_mixed_iaa(646, 639, A::pow(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), s.ad_value(640)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(640)));s.store_mul_scale_offset_indices(611, 611, 646, 1.0, 1.0);}
        if (s.b[2569] && (!s.b[2570])) {s.store_scalar(2538, 0.0);s.store_scalar(2535, 0.0);}
        s.b[2653] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));s.store_scalar(2653, if s.b[2653] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2653]) {s.store_primal_scaled_mul(2525, 685, 685, 4.0);s.store_primal_div(2526, 685, 686);s.store_add_scaled_product_indices(2527, 822, 1.0, 685, 2526, 1.0);s.store_add(2528, 686, 2527);s.store_sub(2529, 686, 2527);s.store_sqrt_square_add(2530, 2529, 2525);s.store_div_scaled_product_add_scaled_denominator_indices(2532, 822, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);}
        s.b[2654] = (s.v[822] < s.v[682]);s.store_scalar(2654, if s.b[2654] { 1.0 } else { 0.0 });s.b[2655] = (((((-0.5) * (s.v[822] * s.v[372]))) as f64).abs() < 230.25850929940458);s.store_scalar(2655, if s.b[2655] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) && s.b[2655]) {s.store_exp_scaled_input(2533, 822, (s.v[372] * (-0.5)));}
        s.b[2656] = (((-0.5) * (s.v[822] * s.v[372])) < 0.0);s.store_scalar(2656, if s.b[2656] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) && (!s.b[2655])) && s.b[2656]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2533, 1e-100, (-230.25850929940458), A::scale(s.ad_value(822), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) && (!s.b[2655])) && (!s.b[2656])) {s.store_scaled_offset_ad(2533, A::mul_offset_rhs(A::scale_offset(s.ad_value(822), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(822), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(822), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) {s.store_div_from_scalar(2534, 1.0, 2533);s.store_square(2531, 2534);}
        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && (!s.b[2654])) {s.store_mul_scale_offset_mixed_ia(2531, 683, A::sub_scaled_inputs(s.ad_value(822), s.v[372], s.ad_value(682), s.v[372]), 1.0, 1.0);s.store_sqrt(2534, 2531);s.store_div_from_scalar(2533, 1.0, 2534);}
        if ((s.b[2569] && (!s.b[2570])) && s.b[2653]) {s.store_offset(2531, 2531, (-1.0));}
        s.b[2657] = (s.v[822] > 0.0);s.store_scalar(2657, if s.b[2657] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2657]) {s.store_scaled_ln_ad(2535, A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2533), 1.0, A::offset(s.ad_value(2533), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && (!s.b[2657])) {s.store_sub_mixed_ai(2535, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2534), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2534), 1.0, A::scale_offset(s.ad_value(2534), 3.0, 1.0))))), (s.v[371] * 2.0)), 822);}
        if ((s.b[2569] && (!s.b[2570])) && s.b[2653]) {s.store_sub(2536, 684, 2535);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2569] && (!s.b[2570])) && s.b[2653]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2537, 822, 0.5, 2536, 0.5, 822, 2536, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2538, 822, 0.5, 687, 0.5, 822, 687, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_scaled_sub_mixed_ia(2539, 822, A::sqrt_square_offset(s.ad_value(822), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[2658] = (s.v[674] == 0.0);s.store_scalar(2658, if s.b[2658] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2658]) {s.store_scalar(1905, 0.0);}
        s.b[2659] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));s.store_scalar(2659, if s.b[2659] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) {s.store_sub(2543, 570, 2537);}
        s.b[2661] = (s.v[512] == 0.5);s.store_scalar(2661, if s.b[2661] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && s.b[2661]) {s.store_sqrt_mul(2540, 2543, 597);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && (!s.b[2661])) {s.store_pow_mul_base_indices(2540, 2543, 597, 512);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) {s.store_mul(2547, 591, 2540);}
        s.b[2662] = (s.v[526] == 0.0);s.store_scalar(2662, if s.b[2662] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) {s.store_mul_div_scaled_product_indices(2550, 606, 2547, 576, 1.0, 2543, 1.0);s.store_div_scaled_inputs_indices(2551, 603, 0.666666666666667, 2550, 1.0);s.store_square(2552, 2551);s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);s.store_sqrt(2554, 2553);s.store_mul(2555, 2553, 2554);s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);s.store_add_scaled_value_products_mixed_aiiii(2560, A::mul3(s.ad_value(603), s.ad_value(2551), s.ad_value(2554)), 1.0, 603, 2553, (-1.0), 2550, 2555, 0.5);s.store_mul_scale_offset_indices(2561, 2558, 2559, 1.0, (-1.0));s.store_square(2522, 2561);}
        s.b[2665] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && s.b[2665]) {s.store_exp_sub(2540, 2560, 2522);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2665])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2666] = (s.v[2561] > 0.0);s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });s.b[2667] = (s.v[2560] > (-230.25850929940458));s.store_scalar(2667, if s.b[2667] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2667]) {s.store_exp(2540, 2560);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2667])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2668] = (s.v[532] == 0.0);s.store_scalar(2668, if s.b[2668] { 1.0 } else { 0.0 });s.b[2669] = (s.v[512] == 0.5);s.store_scalar(2669, if s.b[2669] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && s.b[2669]) {s.store_sqrt_mul_sub_lhs(2540, 509, 2538, 597);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2669])) {s.store_pow_mul_base_mixed_ai(2540, A::sub(s.ad_value(509), s.ad_value(2538)), 597, 512);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) {s.store_mul_div_scaled_product_mixed_iaii(2565, 579, A::sub(s.ad_value(509), s.ad_value(2538)), 594, 1.0, 2540, 1.0);}
        s.b[2670] = (((((-s.v[609]) / s.v[2565])) as f64).abs() < 230.25850929940458);s.store_scalar(2670, if s.b[2670] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && s.b[2670]) {s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0));}
        s.b[2671] = (((-s.v[609]) / s.v[2565]) < 0.0);s.store_scalar(2671, if s.b[2671] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2670])) && s.b[2671]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 609, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2670])) && (!s.b[2671])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 609, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2672] = (s.v[541] > 1000.0);s.store_scalar(2672, if s.b[2672] { 1.0 } else { 0.0 });s.b[2673] = (s.v[2539] > ((-s.v[445]) * s.v[541]));s.store_scalar(2673, if s.b[2673] { 1.0 } else { 0.0 });s.b[2674] = (s.v[544] == 4.0);s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && s.b[2673]) && s.b[2674]) {s.store_mul_ad_product_lhs_mixed_ai(2540, A::mul3(A::square(A::mul(s.ad_value(2539), s.ad_value(615))), s.ad_value(2539), s.ad_value(615)), 2539, 615);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && s.b[2673]) && (!s.b[2674])) {s.store_pow_abs_mul_base_indices(2540, 2539, 615, 544);}
        s.b[2675] = (s.v[576] == 0.5);s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2675]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(573)));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2675])) {s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2532, 573, 576);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) {s.store_add_scaled_product_mixed_aia(1905, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2540)), p.p30, 588, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);}
        s.b[2676] = (s.v[675] == 0.0);s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2676]) {s.store_scalar(1906, 0.0);}
        s.b[2677] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));s.store_scalar(2677, if s.b[2677] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) {s.store_sub(2543, 571, 2537);}
        s.b[2679] = (s.v[513] == 0.5);s.store_scalar(2679, if s.b[2679] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && s.b[2679]) {s.store_sqrt_mul(2540, 2543, 598);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && (!s.b[2679])) {s.store_pow_mul_base_indices(2540, 2543, 598, 513);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) {s.store_mul(2547, 592, 2540);}
        s.b[2680] = (s.v[527] == 0.0);s.store_scalar(2680, if s.b[2680] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) {s.store_mul_div_scaled_product_indices(2550, 607, 2547, 577, 1.0, 2543, 1.0);s.store_div_scaled_inputs_indices(2551, 604, 0.666666666666667, 2550, 1.0);s.store_square(2552, 2551);s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);s.store_sqrt(2554, 2553);s.store_mul(2555, 2553, 2554);s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);s.store_add_scaled_value_products_mixed_aiiii(2560, A::mul3(s.ad_value(604), s.ad_value(2551), s.ad_value(2554)), 1.0, 604, 2553, (-1.0), 2550, 2555, 0.5);s.store_mul_scale_offset_indices(2561, 2558, 2559, 1.0, (-1.0));s.store_square(2522, 2561);}
        s.b[2683] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));s.store_scalar(2683, if s.b[2683] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && s.b[2683]) {s.store_exp_sub(2540, 2560, 2522);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2683])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2684] = (s.v[2561] > 0.0);s.store_scalar(2684, if s.b[2684] { 1.0 } else { 0.0 });s.b[2685] = (s.v[2560] > (-230.25850929940458));s.store_scalar(2685, if s.b[2685] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2685]) {s.store_exp(2540, 2560);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2685])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2686] = (s.v[533] == 0.0);s.store_scalar(2686, if s.b[2686] { 1.0 } else { 0.0 });s.b[2687] = (s.v[513] == 0.5);s.store_scalar(2687, if s.b[2687] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && s.b[2687]) {s.store_sqrt_mul_sub_lhs(2540, 510, 2538, 598);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2687])) {s.store_pow_mul_base_mixed_ai(2540, A::sub(s.ad_value(510), s.ad_value(2538)), 598, 513);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) {s.store_mul_div_scaled_product_mixed_iaii(2565, 580, A::sub(s.ad_value(510), s.ad_value(2538)), 595, 1.0, 2540, 1.0);}
        s.b[2688] = (((((-s.v[610]) / s.v[2565])) as f64).abs() < 230.25850929940458);s.store_scalar(2688, if s.b[2688] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && s.b[2688]) {s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0));}
        s.b[2689] = (((-s.v[610]) / s.v[2565]) < 0.0);s.store_scalar(2689, if s.b[2689] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2688])) && s.b[2689]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 610, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2688])) && (!s.b[2689])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 610, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2690] = (s.v[542] > 1000.0);s.store_scalar(2690, if s.b[2690] { 1.0 } else { 0.0 });s.b[2691] = (s.v[2539] > ((-s.v[445]) * s.v[542]));s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });s.b[2692] = (s.v[545] == 4.0);s.store_scalar(2692, if s.b[2692] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && s.b[2691]) && s.b[2692]) {s.store_mul_ad_product_lhs_mixed_ai(2540, A::mul3(A::square(A::mul(s.ad_value(2539), s.ad_value(616))), s.ad_value(2539), s.ad_value(616)), 2539, 616);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && s.b[2691]) && (!s.b[2692])) {s.store_pow_abs_mul_base_indices(2540, 2539, 616, 545);}
        s.b[2693] = (s.v[577] == 0.5);s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2693]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(574)));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2693])) {s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2532, 574, 577);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) {s.store_add_scaled_product_mixed_aia(1906, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2540)), p.p30, 589, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);}
        s.b[2694] = (s.v[676] == 0.0);s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2694]) {s.store_scalar(1907, 0.0);}
        s.b[2695] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));s.store_scalar(2695, if s.b[2695] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) {s.store_sub(2543, 572, 2537);}
        s.b[2697] = (s.v[514] == 0.5);s.store_scalar(2697, if s.b[2697] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && s.b[2697]) {s.store_sqrt_mul(2540, 2543, 599);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && (!s.b[2697])) {s.store_pow_mul_base_indices(2540, 2543, 599, 514);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) {s.store_mul(2547, 593, 2540);}
        s.b[2698] = (s.v[528] == 0.0);s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) {s.store_mul_div_scaled_product_indices(2550, 608, 2547, 578, 1.0, 2543, 1.0);s.store_div_scaled_inputs_indices(2551, 605, 0.666666666666667, 2550, 1.0);s.store_square(2552, 2551);s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);s.store_sqrt(2554, 2553);s.store_mul(2555, 2553, 2554);s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);s.store_add_scaled_value_products_mixed_aiiii(2560, A::mul3(s.ad_value(605), s.ad_value(2551), s.ad_value(2554)), 1.0, 605, 2553, (-1.0), 2550, 2555, 0.5);s.store_mul_scale_offset_indices(2561, 2558, 2559, 1.0, (-1.0));s.store_square(2522, 2561);}
        s.b[2701] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));s.store_scalar(2701, if s.b[2701] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2701]) {s.store_exp_sub(2540, 2560, 2522);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2701])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2702] = (s.v[2561] > 0.0);s.store_scalar(2702, if s.b[2702] { 1.0 } else { 0.0 });s.b[2703] = (s.v[2560] > (-230.25850929940458));s.store_scalar(2703, if s.b[2703] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2702])) && s.b[2703]) {s.store_exp(2540, 2560);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2702])) && (!s.b[2703])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2704] = (s.v[534] == 0.0);s.store_scalar(2704, if s.b[2704] { 1.0 } else { 0.0 });s.b[2705] = (s.v[514] == 0.5);s.store_scalar(2705, if s.b[2705] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && s.b[2705]) {s.store_sqrt_mul_sub_lhs(2540, 511, 2538, 599);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2705])) {s.store_pow_mul_base_mixed_ai(2540, A::sub(s.ad_value(511), s.ad_value(2538)), 599, 514);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) {s.store_mul_div_scaled_product_mixed_iaii(2565, 581, A::sub(s.ad_value(511), s.ad_value(2538)), 596, 1.0, 2540, 1.0);}
        s.b[2706] = (((((-s.v[611]) / s.v[2565])) as f64).abs() < 230.25850929940458);s.store_scalar(2706, if s.b[2706] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && s.b[2706]) {s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0));}
        s.b[2707] = (((-s.v[611]) / s.v[2565]) < 0.0);s.store_scalar(2707, if s.b[2707] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2706])) && s.b[2707]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 611, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2706])) && (!s.b[2707])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 611, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2708] = (s.v[642] > 1000.0);s.store_scalar(2708, if s.b[2708] { 1.0 } else { 0.0 });s.b[2709] = (s.v[2539] > ((-s.v[445]) * s.v[642]));s.store_scalar(2709, if s.b[2709] { 1.0 } else { 0.0 });s.b[2710] = (s.v[546] == 4.0);s.store_scalar(2710, if s.b[2710] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && s.b[2709]) && s.b[2710]) {s.store_mul_ad_product_lhs_mixed_ai(2540, A::mul3(A::square(A::mul(s.ad_value(2539), s.ad_value(617))), s.ad_value(2539), s.ad_value(617)), 2539, 617);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && s.b[2709]) && (!s.b[2710])) {s.store_pow_abs_mul_base_indices(2540, 2539, 617, 546);}
        s.b[2711] = (s.v[636] == 1.0);s.store_scalar(2711, if s.b[2711] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            if (s.v[822] < s.v[551]) {
                if (((s.v[822] - s.v[551]) / s.v[552]) < (-37.0)) {
                    s.copy_ad(2567, 551);
                } else {
                    s.store_add_scaled_product_mixed_iai(2567, 551, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(822), 1.0, s.ad_value(551), (-1.0), s.ad_value(552), 1.0)), 552, 1.0);
                }
            } else {
                if (((s.v[822] - s.v[551]) / s.v[552]) > 37.0) {
                    s.copy_ad(2567, 822);
                } else {
                    s.store_add_scaled_product_mixed_iai(2567, 822, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(551), 1.0, s.ad_value(822), (-1.0), s.ad_value(552), 1.0)), 552, 1.0);
                }
            }
        }
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {s.store_primal_scaled_mul(2525, 685, 685, 4.0);s.store_primal_div(2526, 685, 686);s.store_add_scaled_product_indices(2527, 2567, 1.0, 685, 2526, 1.0);s.store_add(2528, 686, 2527);s.store_sub(2529, 686, 2527);s.store_sqrt_square_add(2530, 2529, 2525);s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);}
        s.b[2712] = (s.v[578] == 0.5);s.store_scalar(2712, if s.b[2712] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && s.b[2712]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(575)));}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && (!s.b[2712])) {s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2568, 575, 578);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2540)), p.p30, 590, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);s.store_add_scaled_inputs3_indices(2567, 822, 1.0, 551, 1.0, 2567, -1.0);s.store_primal_scaled_mul(2525, 685, 685, 4.0);s.store_primal_div(2526, 685, 686);s.store_add_scaled_product_indices(2527, 2567, 1.0, 685, 2526, 1.0);s.store_add(2528, 686, 2527);s.store_sub(2529, 686, 2527);s.store_sqrt_square_add(2530, 2529, 2525);s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);}
        s.b[2713] = (s.v[631] == 0.5);s.store_scalar(2713, if s.b[2713] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && s.b[2713]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(630)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && (!s.b[2713])) {s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2568, 630, 631);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {s.store_add_scaled_product_mixed_aia(473, A::mul_sub_from_scalar_rhs(s.ad_value(634), 1.0, s.ad_value(2540)), p.p30, 635, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);s.store_add(1907, 1907, 473);}
        s.b[2714] = (s.v[578] == 0.5);s.store_scalar(2714, if s.b[2714] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) && s.b[2714]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(575)));}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) && (!s.b[2714])) {s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2532, 575, 578);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) {s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2540)), p.p30, 590, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);}
        s.store_add_scaled_inputs3_indices(839, 840, (-1.0), 841, (-1.0), 842, (-1.0));s.store_add(843, 843, 1894);s.store_add(844, 844, 1895);s.store_add_scaled_products3_indices(846, 647, 1902, 1.0, 648, 1903, 1.0, 649, 1904, 1.0);s.store_add_scaled_products3_indices(847, 674, 1905, 1.0, 675, 1906, 1.0, 676, 1907, 1.0);s.b[2729] = (s.v[820] < 0.0);s.store_scalar(2729, if s.b[2729] { 1.0 } else { 0.0 });
        if s.b[2729] {s.copy_ad(2728, 842);s.copy_ad(842, 839);s.copy_ad(839, 2728);}
        s.store_mul(849, 1888, 1879);s.b[2762] = ((s.v[1813] > 0.0) && (s.v[1917] > 0.0));s.store_scalar(2762, if s.b[2762] { 1.0 } else { 0.0 });s.b[2767] = ((((p.p50 == 1.0) && (s.v[1920] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));s.store_scalar(2767, if s.b[2767] { 1.0 } else { 0.0 });
        if (s.b[2762] && s.b[2767]) {s.store_div_scaled_product3_mixed_aiia(849, A::square(s.ad_value(1892)), 1888, 1879, 1.0, A::square(s.ad_value(1890)), 1.0);}
        s.b[2771] = (((p.p46 != 0.0) && (s.v[285] > 0.0)) && (s.v[1864] > 0.0));s.store_scalar(2771, if s.b[2771] { 1.0 } else { 0.0 });
        if s.b[2771] {s.store_div_scaled_inputs_indices(1930, 1867, 4.0, 1925, 1.0);s.store_mul(1930, 760, 1916);s.store_mul(1930, 1848, 1861);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq0_e972, eq0_e972_d_n0, eq0_e972_d_n1, eq0_e972_d_n2, eq0_e972_d_n3, eq0_e972_d_n4, eq0_e972_d_n5, eq0_e972_d_n6, eq0_e972_d_n7, eq0_e972_d_n8, eq0_e972_d_n9, eq0_e972_d_n10, eq0_e972_d_n11, eq0_e972_d_n12, eq0_e972_d_b0, eq0_e972_d_b1, eq0_e972_d_b2, eq0_e972_d_b3, eq0_e972_d_b4, eq0_e972_d_b5, eq0_e972_d_b6,) = {
    if s.b[2715] {
        let eq0_e966: f64 = (s.v[0] * s.v[15]);let eq0_e966_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq0_e966_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq0_e966_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq0_e966_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq0_e966_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq0_e966_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq0_e966_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq0_e966_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq0_e966_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq0_e966_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq0_e966_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq0_e966_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq0_e966_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq0_e966_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq0_e966_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq0_e966_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq0_e966_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq0_e966_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq0_e966_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq0_e966_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq0_e968: f64 = (eq0_e966 * p.p32);let eq0_e968_d_n0: f64 = (eq0_e966_d_n0 * p.p32);let eq0_e968_d_n1: f64 = (eq0_e966_d_n1 * p.p32);let eq0_e968_d_n2: f64 = (eq0_e966_d_n2 * p.p32);let eq0_e968_d_n3: f64 = (eq0_e966_d_n3 * p.p32);let eq0_e968_d_n4: f64 = (eq0_e966_d_n4 * p.p32);let eq0_e968_d_n5: f64 = (eq0_e966_d_n5 * p.p32);let eq0_e968_d_n6: f64 = (eq0_e966_d_n6 * p.p32);let eq0_e968_d_n7: f64 = (eq0_e966_d_n7 * p.p32);let eq0_e968_d_n8: f64 = (eq0_e966_d_n8 * p.p32);let eq0_e968_d_n9: f64 = (eq0_e966_d_n9 * p.p32);let eq0_e968_d_n10: f64 = (eq0_e966_d_n10 * p.p32);let eq0_e968_d_n11: f64 = (eq0_e966_d_n11 * p.p32);let eq0_e968_d_n12: f64 = (eq0_e966_d_n12 * p.p32);let eq0_e968_d_b0: f64 = (eq0_e966_d_b0 * p.p32);let eq0_e968_d_b1: f64 = (eq0_e966_d_b1 * p.p32);let eq0_e968_d_b2: f64 = (eq0_e966_d_b2 * p.p32);let eq0_e968_d_b3: f64 = (eq0_e966_d_b3 * p.p32);let eq0_e968_d_b4: f64 = (eq0_e966_d_b4 * p.p32);let eq0_e968_d_b5: f64 = (eq0_e966_d_b5 * p.p32);let eq0_e968_d_b6: f64 = (eq0_e966_d_b6 * p.p32);let eq0_e970: f64 = (eq0_e968 * s.v[836]);let eq0_e970_d_n0: f64 = ((eq0_e968_d_n0 * s.v[836]) + (eq0_e968 * s.dn[836][0]));let eq0_e970_d_n1: f64 = ((eq0_e968_d_n1 * s.v[836]) + (eq0_e968 * s.dn[836][1]));let eq0_e970_d_n2: f64 = ((eq0_e968_d_n2 * s.v[836]) + (eq0_e968 * s.dn[836][2]));let eq0_e970_d_n3: f64 = ((eq0_e968_d_n3 * s.v[836]) + (eq0_e968 * s.dn[836][3]));let eq0_e970_d_n4: f64 = ((eq0_e968_d_n4 * s.v[836]) + (eq0_e968 * s.dn[836][4]));let eq0_e970_d_n5: f64 = ((eq0_e968_d_n5 * s.v[836]) + (eq0_e968 * s.dn[836][5]));let eq0_e970_d_n6: f64 = ((eq0_e968_d_n6 * s.v[836]) + (eq0_e968 * s.dn[836][6]));let eq0_e970_d_n7: f64 = ((eq0_e968_d_n7 * s.v[836]) + (eq0_e968 * s.dn[836][7]));let eq0_e970_d_n8: f64 = ((eq0_e968_d_n8 * s.v[836]) + (eq0_e968 * s.dn[836][8]));let eq0_e970_d_n9: f64 = ((eq0_e968_d_n9 * s.v[836]) + (eq0_e968 * s.dn[836][9]));let eq0_e970_d_n10: f64 = ((eq0_e968_d_n10 * s.v[836]) + (eq0_e968 * s.dn[836][10]));let eq0_e970_d_n11: f64 = ((eq0_e968_d_n11 * s.v[836]) + (eq0_e968 * s.dn[836][11]));let eq0_e970_d_n12: f64 = ((eq0_e968_d_n12 * s.v[836]) + (eq0_e968 * s.dn[836][12]));let eq0_e970_d_b0: f64 = ((eq0_e968_d_b0 * s.v[836]) + (eq0_e968 * s.db[836][0]));let eq0_e970_d_b1: f64 = ((eq0_e968_d_b1 * s.v[836]) + (eq0_e968 * s.db[836][1]));let eq0_e970_d_b2: f64 = ((eq0_e968_d_b2 * s.v[836]) + (eq0_e968 * s.db[836][2]));let eq0_e970_d_b3: f64 = ((eq0_e968_d_b3 * s.v[836]) + (eq0_e968 * s.db[836][3]));
        let eq0_e970_d_b4: f64 = ((eq0_e968_d_b4 * s.v[836]) + (eq0_e968 * s.db[836][4]));let eq0_e970_d_b5: f64 = ((eq0_e968_d_b5 * s.v[836]) + (eq0_e968 * s.db[836][5]));let eq0_e970_d_b6: f64 = ((eq0_e968_d_b6 * s.v[836]) + (eq0_e968 * s.db[836][6]));
        (eq0_e970, eq0_e970_d_n0, eq0_e970_d_n1, eq0_e970_d_n2, eq0_e970_d_n3, eq0_e970_d_n4, eq0_e970_d_n5, eq0_e970_d_n6, eq0_e970_d_n7, eq0_e970_d_n8, eq0_e970_d_n9, eq0_e970_d_n10, eq0_e970_d_n11, eq0_e970_d_n12, eq0_e970_d_b0, eq0_e970_d_b1, eq0_e970_d_b2, eq0_e970_d_b3, eq0_e970_d_b4, eq0_e970_d_b5, eq0_e970_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e972;let eq0_node_derivatives: [f64; 13] = [eq0_e972_d_n0, eq0_e972_d_n1, eq0_e972_d_n2, eq0_e972_d_n3, eq0_e972_d_n4, eq0_e972_d_n5, eq0_e972_d_n6, eq0_e972_d_n7, eq0_e972_d_n8, eq0_e972_d_n9, eq0_e972_d_n10, eq0_e972_d_n11, eq0_e972_d_n12];let eq0_branch_derivatives: [f64; 7] = [eq0_e972_d_b0, eq0_e972_d_b1, eq0_e972_d_b2, eq0_e972_d_b3, eq0_e972_d_b4, eq0_e972_d_b5, eq0_e972_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq1_e984, eq1_e984_d_n0, eq1_e984_d_n1, eq1_e984_d_n2, eq1_e984_d_n3, eq1_e984_d_n4, eq1_e984_d_n5, eq1_e984_d_n6, eq1_e984_d_n7, eq1_e984_d_n8, eq1_e984_d_n9, eq1_e984_d_n10, eq1_e984_d_n11, eq1_e984_d_n12, eq1_e984_d_b0, eq1_e984_d_b1, eq1_e984_d_b2, eq1_e984_d_b3, eq1_e984_d_b4, eq1_e984_d_b5, eq1_e984_d_b6,) = {
    if s.b[2715] {
        let eq1_e976: f64 = (s.v[0] * s.v[15]);let eq1_e976_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq1_e976_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq1_e976_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq1_e976_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq1_e976_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq1_e976_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq1_e976_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq1_e976_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq1_e976_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq1_e976_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq1_e976_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq1_e976_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq1_e976_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq1_e976_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq1_e976_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq1_e976_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq1_e976_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq1_e976_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq1_e976_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq1_e976_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq1_e978: f64 = (eq1_e976 * p.p32);let eq1_e978_d_n0: f64 = (eq1_e976_d_n0 * p.p32);let eq1_e978_d_n1: f64 = (eq1_e976_d_n1 * p.p32);let eq1_e978_d_n2: f64 = (eq1_e976_d_n2 * p.p32);let eq1_e978_d_n3: f64 = (eq1_e976_d_n3 * p.p32);let eq1_e978_d_n4: f64 = (eq1_e976_d_n4 * p.p32);let eq1_e978_d_n5: f64 = (eq1_e976_d_n5 * p.p32);let eq1_e978_d_n6: f64 = (eq1_e976_d_n6 * p.p32);let eq1_e978_d_n7: f64 = (eq1_e976_d_n7 * p.p32);let eq1_e978_d_n8: f64 = (eq1_e976_d_n8 * p.p32);let eq1_e978_d_n9: f64 = (eq1_e976_d_n9 * p.p32);let eq1_e978_d_n10: f64 = (eq1_e976_d_n10 * p.p32);let eq1_e978_d_n11: f64 = (eq1_e976_d_n11 * p.p32);let eq1_e978_d_n12: f64 = (eq1_e976_d_n12 * p.p32);let eq1_e978_d_b0: f64 = (eq1_e976_d_b0 * p.p32);let eq1_e978_d_b1: f64 = (eq1_e976_d_b1 * p.p32);let eq1_e978_d_b2: f64 = (eq1_e976_d_b2 * p.p32);let eq1_e978_d_b3: f64 = (eq1_e976_d_b3 * p.p32);let eq1_e978_d_b4: f64 = (eq1_e976_d_b4 * p.p32);let eq1_e978_d_b5: f64 = (eq1_e976_d_b5 * p.p32);let eq1_e978_d_b6: f64 = (eq1_e976_d_b6 * p.p32);let eq1_e981: f64 = (s.v[827] + s.v[835]);let eq1_e981_d_n0: f64 = (s.dn[827][0] + s.dn[835][0]);let eq1_e981_d_n1: f64 = (s.dn[827][1] + s.dn[835][1]);let eq1_e981_d_n2: f64 = (s.dn[827][2] + s.dn[835][2]);let eq1_e981_d_n3: f64 = (s.dn[827][3] + s.dn[835][3]);let eq1_e981_d_n4: f64 = (s.dn[827][4] + s.dn[835][4]);let eq1_e981_d_n5: f64 = (s.dn[827][5] + s.dn[835][5]);let eq1_e981_d_n6: f64 = (s.dn[827][6] + s.dn[835][6]);let eq1_e981_d_n7: f64 = (s.dn[827][7] + s.dn[835][7]);let eq1_e981_d_n8: f64 = (s.dn[827][8] + s.dn[835][8]);let eq1_e981_d_n9: f64 = (s.dn[827][9] + s.dn[835][9]);let eq1_e981_d_n10: f64 = (s.dn[827][10] + s.dn[835][10]);let eq1_e981_d_n11: f64 = (s.dn[827][11] + s.dn[835][11]);let eq1_e981_d_n12: f64 = (s.dn[827][12] + s.dn[835][12]);let eq1_e981_d_b0: f64 = (s.db[827][0] + s.db[835][0]);let eq1_e981_d_b1: f64 = (s.db[827][1] + s.db[835][1]);let eq1_e981_d_b2: f64 = (s.db[827][2] + s.db[835][2]);let eq1_e981_d_b3: f64 = (s.db[827][3] + s.db[835][3]);let eq1_e981_d_b4: f64 = (s.db[827][4] + s.db[835][4]);let eq1_e981_d_b5: f64 = (s.db[827][5] + s.db[835][5]);let eq1_e981_d_b6: f64 = (s.db[827][6] + s.db[835][6]);let eq1_e982: f64 = (eq1_e978 * eq1_e981);let eq1_e982_d_n0: f64 = ((eq1_e978_d_n0 * eq1_e981) + (eq1_e978 * eq1_e981_d_n0));let eq1_e982_d_n1: f64 = ((eq1_e978_d_n1 * eq1_e981) + (eq1_e978 * eq1_e981_d_n1));let eq1_e982_d_n2: f64 = ((eq1_e978_d_n2 * eq1_e981) + (eq1_e978 * eq1_e981_d_n2));
        let eq1_e982_d_n3: f64 = ((eq1_e978_d_n3 * eq1_e981) + (eq1_e978 * eq1_e981_d_n3));let eq1_e982_d_n4: f64 = ((eq1_e978_d_n4 * eq1_e981) + (eq1_e978 * eq1_e981_d_n4));let eq1_e982_d_n5: f64 = ((eq1_e978_d_n5 * eq1_e981) + (eq1_e978 * eq1_e981_d_n5));let eq1_e982_d_n6: f64 = ((eq1_e978_d_n6 * eq1_e981) + (eq1_e978 * eq1_e981_d_n6));let eq1_e982_d_n7: f64 = ((eq1_e978_d_n7 * eq1_e981) + (eq1_e978 * eq1_e981_d_n7));let eq1_e982_d_n8: f64 = ((eq1_e978_d_n8 * eq1_e981) + (eq1_e978 * eq1_e981_d_n8));let eq1_e982_d_n9: f64 = ((eq1_e978_d_n9 * eq1_e981) + (eq1_e978 * eq1_e981_d_n9));let eq1_e982_d_n10: f64 = ((eq1_e978_d_n10 * eq1_e981) + (eq1_e978 * eq1_e981_d_n10));let eq1_e982_d_n11: f64 = ((eq1_e978_d_n11 * eq1_e981) + (eq1_e978 * eq1_e981_d_n11));let eq1_e982_d_n12: f64 = ((eq1_e978_d_n12 * eq1_e981) + (eq1_e978 * eq1_e981_d_n12));let eq1_e982_d_b0: f64 = ((eq1_e978_d_b0 * eq1_e981) + (eq1_e978 * eq1_e981_d_b0));let eq1_e982_d_b1: f64 = ((eq1_e978_d_b1 * eq1_e981) + (eq1_e978 * eq1_e981_d_b1));let eq1_e982_d_b2: f64 = ((eq1_e978_d_b2 * eq1_e981) + (eq1_e978 * eq1_e981_d_b2));let eq1_e982_d_b3: f64 = ((eq1_e978_d_b3 * eq1_e981) + (eq1_e978 * eq1_e981_d_b3));let eq1_e982_d_b4: f64 = ((eq1_e978_d_b4 * eq1_e981) + (eq1_e978 * eq1_e981_d_b4));let eq1_e982_d_b5: f64 = ((eq1_e978_d_b5 * eq1_e981) + (eq1_e978 * eq1_e981_d_b5));let eq1_e982_d_b6: f64 = ((eq1_e978_d_b6 * eq1_e981) + (eq1_e978 * eq1_e981_d_b6));
        (eq1_e982, eq1_e982_d_n0, eq1_e982_d_n1, eq1_e982_d_n2, eq1_e982_d_n3, eq1_e982_d_n4, eq1_e982_d_n5, eq1_e982_d_n6, eq1_e982_d_n7, eq1_e982_d_n8, eq1_e982_d_n9, eq1_e982_d_n10, eq1_e982_d_n11, eq1_e982_d_n12, eq1_e982_d_b0, eq1_e982_d_b1, eq1_e982_d_b2, eq1_e982_d_b3, eq1_e982_d_b4, eq1_e982_d_b5, eq1_e982_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e984;let eq1_node_derivatives: [f64; 13] = [eq1_e984_d_n0, eq1_e984_d_n1, eq1_e984_d_n2, eq1_e984_d_n3, eq1_e984_d_n4, eq1_e984_d_n5, eq1_e984_d_n6, eq1_e984_d_n7, eq1_e984_d_n8, eq1_e984_d_n9, eq1_e984_d_n10, eq1_e984_d_n11, eq1_e984_d_n12];let eq1_branch_derivatives: [f64; 7] = [eq1_e984_d_b0, eq1_e984_d_b1, eq1_e984_d_b2, eq1_e984_d_b3, eq1_e984_d_b4, eq1_e984_d_b5, eq1_e984_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_2(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq2_e994, eq2_e994_d_n0, eq2_e994_d_n1, eq2_e994_d_n2, eq2_e994_d_n3, eq2_e994_d_n4, eq2_e994_d_n5, eq2_e994_d_n6, eq2_e994_d_n7, eq2_e994_d_n8, eq2_e994_d_n9, eq2_e994_d_n10, eq2_e994_d_n11, eq2_e994_d_n12, eq2_e994_d_b0, eq2_e994_d_b1, eq2_e994_d_b2, eq2_e994_d_b3, eq2_e994_d_b4, eq2_e994_d_b5, eq2_e994_d_b6,) = {
    if s.b[2715] {
        let eq2_e988: f64 = (s.v[0] * s.v[15]);let eq2_e988_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq2_e988_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq2_e988_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq2_e988_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq2_e988_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq2_e988_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq2_e988_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq2_e988_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq2_e988_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq2_e988_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq2_e988_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq2_e988_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq2_e988_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq2_e988_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq2_e988_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq2_e988_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq2_e988_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq2_e988_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq2_e988_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq2_e988_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq2_e990: f64 = (eq2_e988 * p.p32);let eq2_e990_d_n0: f64 = (eq2_e988_d_n0 * p.p32);let eq2_e990_d_n1: f64 = (eq2_e988_d_n1 * p.p32);let eq2_e990_d_n2: f64 = (eq2_e988_d_n2 * p.p32);let eq2_e990_d_n3: f64 = (eq2_e988_d_n3 * p.p32);let eq2_e990_d_n4: f64 = (eq2_e988_d_n4 * p.p32);let eq2_e990_d_n5: f64 = (eq2_e988_d_n5 * p.p32);let eq2_e990_d_n6: f64 = (eq2_e988_d_n6 * p.p32);let eq2_e990_d_n7: f64 = (eq2_e988_d_n7 * p.p32);let eq2_e990_d_n8: f64 = (eq2_e988_d_n8 * p.p32);let eq2_e990_d_n9: f64 = (eq2_e988_d_n9 * p.p32);let eq2_e990_d_n10: f64 = (eq2_e988_d_n10 * p.p32);let eq2_e990_d_n11: f64 = (eq2_e988_d_n11 * p.p32);let eq2_e990_d_n12: f64 = (eq2_e988_d_n12 * p.p32);let eq2_e990_d_b0: f64 = (eq2_e988_d_b0 * p.p32);let eq2_e990_d_b1: f64 = (eq2_e988_d_b1 * p.p32);let eq2_e990_d_b2: f64 = (eq2_e988_d_b2 * p.p32);let eq2_e990_d_b3: f64 = (eq2_e988_d_b3 * p.p32);let eq2_e990_d_b4: f64 = (eq2_e988_d_b4 * p.p32);let eq2_e990_d_b5: f64 = (eq2_e988_d_b5 * p.p32);let eq2_e990_d_b6: f64 = (eq2_e988_d_b6 * p.p32);let eq2_e992: f64 = (eq2_e990 * s.v[830]);let eq2_e992_d_n0: f64 = ((eq2_e990_d_n0 * s.v[830]) + (eq2_e990 * s.dn[830][0]));let eq2_e992_d_n1: f64 = ((eq2_e990_d_n1 * s.v[830]) + (eq2_e990 * s.dn[830][1]));let eq2_e992_d_n2: f64 = ((eq2_e990_d_n2 * s.v[830]) + (eq2_e990 * s.dn[830][2]));let eq2_e992_d_n3: f64 = ((eq2_e990_d_n3 * s.v[830]) + (eq2_e990 * s.dn[830][3]));let eq2_e992_d_n4: f64 = ((eq2_e990_d_n4 * s.v[830]) + (eq2_e990 * s.dn[830][4]));let eq2_e992_d_n5: f64 = ((eq2_e990_d_n5 * s.v[830]) + (eq2_e990 * s.dn[830][5]));let eq2_e992_d_n6: f64 = ((eq2_e990_d_n6 * s.v[830]) + (eq2_e990 * s.dn[830][6]));let eq2_e992_d_n7: f64 = ((eq2_e990_d_n7 * s.v[830]) + (eq2_e990 * s.dn[830][7]));let eq2_e992_d_n8: f64 = ((eq2_e990_d_n8 * s.v[830]) + (eq2_e990 * s.dn[830][8]));let eq2_e992_d_n9: f64 = ((eq2_e990_d_n9 * s.v[830]) + (eq2_e990 * s.dn[830][9]));let eq2_e992_d_n10: f64 = ((eq2_e990_d_n10 * s.v[830]) + (eq2_e990 * s.dn[830][10]));let eq2_e992_d_n11: f64 = ((eq2_e990_d_n11 * s.v[830]) + (eq2_e990 * s.dn[830][11]));let eq2_e992_d_n12: f64 = ((eq2_e990_d_n12 * s.v[830]) + (eq2_e990 * s.dn[830][12]));let eq2_e992_d_b0: f64 = ((eq2_e990_d_b0 * s.v[830]) + (eq2_e990 * s.db[830][0]));let eq2_e992_d_b1: f64 = ((eq2_e990_d_b1 * s.v[830]) + (eq2_e990 * s.db[830][1]));let eq2_e992_d_b2: f64 = ((eq2_e990_d_b2 * s.v[830]) + (eq2_e990 * s.db[830][2]));let eq2_e992_d_b3: f64 = ((eq2_e990_d_b3 * s.v[830]) + (eq2_e990 * s.db[830][3]));
        let eq2_e992_d_b4: f64 = ((eq2_e990_d_b4 * s.v[830]) + (eq2_e990 * s.db[830][4]));let eq2_e992_d_b5: f64 = ((eq2_e990_d_b5 * s.v[830]) + (eq2_e990 * s.db[830][5]));let eq2_e992_d_b6: f64 = ((eq2_e990_d_b6 * s.v[830]) + (eq2_e990 * s.db[830][6]));
        (eq2_e992, eq2_e992_d_n0, eq2_e992_d_n1, eq2_e992_d_n2, eq2_e992_d_n3, eq2_e992_d_n4, eq2_e992_d_n5, eq2_e992_d_n6, eq2_e992_d_n7, eq2_e992_d_n8, eq2_e992_d_n9, eq2_e992_d_n10, eq2_e992_d_n11, eq2_e992_d_n12, eq2_e992_d_b0, eq2_e992_d_b1, eq2_e992_d_b2, eq2_e992_d_b3, eq2_e992_d_b4, eq2_e992_d_b5, eq2_e992_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e994;let eq2_node_derivatives: [f64; 13] = [eq2_e994_d_n0, eq2_e994_d_n1, eq2_e994_d_n2, eq2_e994_d_n3, eq2_e994_d_n4, eq2_e994_d_n5, eq2_e994_d_n6, eq2_e994_d_n7, eq2_e994_d_n8, eq2_e994_d_n9, eq2_e994_d_n10, eq2_e994_d_n11, eq2_e994_d_n12];let eq2_branch_derivatives: [f64; 7] = [eq2_e994_d_b0, eq2_e994_d_b1, eq2_e994_d_b2, eq2_e994_d_b3, eq2_e994_d_b4, eq2_e994_d_b5, eq2_e994_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
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
        let (eq3_e1004, eq3_e1004_d_n0, eq3_e1004_d_n1, eq3_e1004_d_n2, eq3_e1004_d_n3, eq3_e1004_d_n4, eq3_e1004_d_n5, eq3_e1004_d_n6, eq3_e1004_d_n7, eq3_e1004_d_n8, eq3_e1004_d_n9, eq3_e1004_d_n10, eq3_e1004_d_n11, eq3_e1004_d_n12, eq3_e1004_d_b0, eq3_e1004_d_b1, eq3_e1004_d_b2, eq3_e1004_d_b3, eq3_e1004_d_b4, eq3_e1004_d_b5, eq3_e1004_d_b6,) = {
    if s.b[2715] {
        let eq3_e998: f64 = (s.v[0] * s.v[15]);let eq3_e998_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq3_e998_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq3_e998_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq3_e998_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq3_e998_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq3_e998_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq3_e998_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq3_e998_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq3_e998_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq3_e998_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq3_e998_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq3_e998_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq3_e998_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq3_e998_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq3_e998_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq3_e998_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq3_e998_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq3_e998_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq3_e998_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq3_e998_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq3_e1000: f64 = (eq3_e998 * p.p32);let eq3_e1000_d_n0: f64 = (eq3_e998_d_n0 * p.p32);let eq3_e1000_d_n1: f64 = (eq3_e998_d_n1 * p.p32);let eq3_e1000_d_n2: f64 = (eq3_e998_d_n2 * p.p32);let eq3_e1000_d_n3: f64 = (eq3_e998_d_n3 * p.p32);let eq3_e1000_d_n4: f64 = (eq3_e998_d_n4 * p.p32);let eq3_e1000_d_n5: f64 = (eq3_e998_d_n5 * p.p32);let eq3_e1000_d_n6: f64 = (eq3_e998_d_n6 * p.p32);let eq3_e1000_d_n7: f64 = (eq3_e998_d_n7 * p.p32);let eq3_e1000_d_n8: f64 = (eq3_e998_d_n8 * p.p32);let eq3_e1000_d_n9: f64 = (eq3_e998_d_n9 * p.p32);let eq3_e1000_d_n10: f64 = (eq3_e998_d_n10 * p.p32);let eq3_e1000_d_n11: f64 = (eq3_e998_d_n11 * p.p32);let eq3_e1000_d_n12: f64 = (eq3_e998_d_n12 * p.p32);let eq3_e1000_d_b0: f64 = (eq3_e998_d_b0 * p.p32);let eq3_e1000_d_b1: f64 = (eq3_e998_d_b1 * p.p32);let eq3_e1000_d_b2: f64 = (eq3_e998_d_b2 * p.p32);let eq3_e1000_d_b3: f64 = (eq3_e998_d_b3 * p.p32);let eq3_e1000_d_b4: f64 = (eq3_e998_d_b4 * p.p32);let eq3_e1000_d_b5: f64 = (eq3_e998_d_b5 * p.p32);let eq3_e1000_d_b6: f64 = (eq3_e998_d_b6 * p.p32);let eq3_e1002: f64 = (eq3_e1000 * s.v[831]);let eq3_e1002_d_n0: f64 = ((eq3_e1000_d_n0 * s.v[831]) + (eq3_e1000 * s.dn[831][0]));let eq3_e1002_d_n1: f64 = ((eq3_e1000_d_n1 * s.v[831]) + (eq3_e1000 * s.dn[831][1]));let eq3_e1002_d_n2: f64 = ((eq3_e1000_d_n2 * s.v[831]) + (eq3_e1000 * s.dn[831][2]));let eq3_e1002_d_n3: f64 = ((eq3_e1000_d_n3 * s.v[831]) + (eq3_e1000 * s.dn[831][3]));let eq3_e1002_d_n4: f64 = ((eq3_e1000_d_n4 * s.v[831]) + (eq3_e1000 * s.dn[831][4]));let eq3_e1002_d_n5: f64 = ((eq3_e1000_d_n5 * s.v[831]) + (eq3_e1000 * s.dn[831][5]));let eq3_e1002_d_n6: f64 = ((eq3_e1000_d_n6 * s.v[831]) + (eq3_e1000 * s.dn[831][6]));let eq3_e1002_d_n7: f64 = ((eq3_e1000_d_n7 * s.v[831]) + (eq3_e1000 * s.dn[831][7]));let eq3_e1002_d_n8: f64 = ((eq3_e1000_d_n8 * s.v[831]) + (eq3_e1000 * s.dn[831][8]));let eq3_e1002_d_n9: f64 = ((eq3_e1000_d_n9 * s.v[831]) + (eq3_e1000 * s.dn[831][9]));let eq3_e1002_d_n10: f64 = ((eq3_e1000_d_n10 * s.v[831]) + (eq3_e1000 * s.dn[831][10]));let eq3_e1002_d_n11: f64 = ((eq3_e1000_d_n11 * s.v[831]) + (eq3_e1000 * s.dn[831][11]));let eq3_e1002_d_n12: f64 = ((eq3_e1000_d_n12 * s.v[831]) + (eq3_e1000 * s.dn[831][12]));let eq3_e1002_d_b0: f64 = ((eq3_e1000_d_b0 * s.v[831]) + (eq3_e1000 * s.db[831][0]));let eq3_e1002_d_b1: f64 = ((eq3_e1000_d_b1 * s.v[831]) + (eq3_e1000 * s.db[831][1]));let eq3_e1002_d_b2: f64 = ((eq3_e1000_d_b2 * s.v[831]) + (eq3_e1000 * s.db[831][2]));
        let eq3_e1002_d_b3: f64 = ((eq3_e1000_d_b3 * s.v[831]) + (eq3_e1000 * s.db[831][3]));let eq3_e1002_d_b4: f64 = ((eq3_e1000_d_b4 * s.v[831]) + (eq3_e1000 * s.db[831][4]));let eq3_e1002_d_b5: f64 = ((eq3_e1000_d_b5 * s.v[831]) + (eq3_e1000 * s.db[831][5]));let eq3_e1002_d_b6: f64 = ((eq3_e1000_d_b6 * s.v[831]) + (eq3_e1000 * s.db[831][6]));
        (eq3_e1002, eq3_e1002_d_n0, eq3_e1002_d_n1, eq3_e1002_d_n2, eq3_e1002_d_n3, eq3_e1002_d_n4, eq3_e1002_d_n5, eq3_e1002_d_n6, eq3_e1002_d_n7, eq3_e1002_d_n8, eq3_e1002_d_n9, eq3_e1002_d_n10, eq3_e1002_d_n11, eq3_e1002_d_n12, eq3_e1002_d_b0, eq3_e1002_d_b1, eq3_e1002_d_b2, eq3_e1002_d_b3, eq3_e1002_d_b4, eq3_e1002_d_b5, eq3_e1002_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e1004;let eq3_node_derivatives: [f64; 13] = [eq3_e1004_d_n0, eq3_e1004_d_n1, eq3_e1004_d_n2, eq3_e1004_d_n3, eq3_e1004_d_n4, eq3_e1004_d_n5, eq3_e1004_d_n6, eq3_e1004_d_n7, eq3_e1004_d_n8, eq3_e1004_d_n9, eq3_e1004_d_n10, eq3_e1004_d_n11, eq3_e1004_d_n12];let eq3_branch_derivatives: [f64; 7] = [eq3_e1004_d_b0, eq3_e1004_d_b1, eq3_e1004_d_b2, eq3_e1004_d_b3, eq3_e1004_d_b4, eq3_e1004_d_b5, eq3_e1004_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
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
        let (eq4_e1015, eq4_e1015_d_n0, eq4_e1015_d_n1, eq4_e1015_d_n2, eq4_e1015_d_n3, eq4_e1015_d_n4, eq4_e1015_d_n5, eq4_e1015_d_n6, eq4_e1015_d_n7, eq4_e1015_d_n8, eq4_e1015_d_n9, eq4_e1015_d_n10, eq4_e1015_d_n11, eq4_e1015_d_n12, eq4_e1015_d_b0, eq4_e1015_d_b1, eq4_e1015_d_b2, eq4_e1015_d_b3, eq4_e1015_d_b4, eq4_e1015_d_b5, eq4_e1015_d_b6,) = {
    if (!s.b[2715]) {
        let eq4_e1009: f64 = (s.v[0] * s.v[15]);let eq4_e1009_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq4_e1009_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq4_e1009_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq4_e1009_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq4_e1009_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq4_e1009_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq4_e1009_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq4_e1009_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq4_e1009_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq4_e1009_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq4_e1009_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq4_e1009_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq4_e1009_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq4_e1009_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq4_e1009_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq4_e1009_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq4_e1009_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq4_e1009_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq4_e1009_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq4_e1009_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq4_e1011: f64 = (eq4_e1009 * p.p32);let eq4_e1011_d_n0: f64 = (eq4_e1009_d_n0 * p.p32);let eq4_e1011_d_n1: f64 = (eq4_e1009_d_n1 * p.p32);let eq4_e1011_d_n2: f64 = (eq4_e1009_d_n2 * p.p32);let eq4_e1011_d_n3: f64 = (eq4_e1009_d_n3 * p.p32);let eq4_e1011_d_n4: f64 = (eq4_e1009_d_n4 * p.p32);let eq4_e1011_d_n5: f64 = (eq4_e1009_d_n5 * p.p32);let eq4_e1011_d_n6: f64 = (eq4_e1009_d_n6 * p.p32);let eq4_e1011_d_n7: f64 = (eq4_e1009_d_n7 * p.p32);let eq4_e1011_d_n8: f64 = (eq4_e1009_d_n8 * p.p32);let eq4_e1011_d_n9: f64 = (eq4_e1009_d_n9 * p.p32);let eq4_e1011_d_n10: f64 = (eq4_e1009_d_n10 * p.p32);let eq4_e1011_d_n11: f64 = (eq4_e1009_d_n11 * p.p32);let eq4_e1011_d_n12: f64 = (eq4_e1009_d_n12 * p.p32);let eq4_e1011_d_b0: f64 = (eq4_e1009_d_b0 * p.p32);let eq4_e1011_d_b1: f64 = (eq4_e1009_d_b1 * p.p32);let eq4_e1011_d_b2: f64 = (eq4_e1009_d_b2 * p.p32);let eq4_e1011_d_b3: f64 = (eq4_e1009_d_b3 * p.p32);let eq4_e1011_d_b4: f64 = (eq4_e1009_d_b4 * p.p32);let eq4_e1011_d_b5: f64 = (eq4_e1009_d_b5 * p.p32);let eq4_e1011_d_b6: f64 = (eq4_e1009_d_b6 * p.p32);let eq4_e1013: f64 = (eq4_e1011 * s.v[836]);let eq4_e1013_d_n0: f64 = ((eq4_e1011_d_n0 * s.v[836]) + (eq4_e1011 * s.dn[836][0]));let eq4_e1013_d_n1: f64 = ((eq4_e1011_d_n1 * s.v[836]) + (eq4_e1011 * s.dn[836][1]));let eq4_e1013_d_n2: f64 = ((eq4_e1011_d_n2 * s.v[836]) + (eq4_e1011 * s.dn[836][2]));let eq4_e1013_d_n3: f64 = ((eq4_e1011_d_n3 * s.v[836]) + (eq4_e1011 * s.dn[836][3]));let eq4_e1013_d_n4: f64 = ((eq4_e1011_d_n4 * s.v[836]) + (eq4_e1011 * s.dn[836][4]));let eq4_e1013_d_n5: f64 = ((eq4_e1011_d_n5 * s.v[836]) + (eq4_e1011 * s.dn[836][5]));let eq4_e1013_d_n6: f64 = ((eq4_e1011_d_n6 * s.v[836]) + (eq4_e1011 * s.dn[836][6]));let eq4_e1013_d_n7: f64 = ((eq4_e1011_d_n7 * s.v[836]) + (eq4_e1011 * s.dn[836][7]));let eq4_e1013_d_n8: f64 = ((eq4_e1011_d_n8 * s.v[836]) + (eq4_e1011 * s.dn[836][8]));let eq4_e1013_d_n9: f64 = ((eq4_e1011_d_n9 * s.v[836]) + (eq4_e1011 * s.dn[836][9]));let eq4_e1013_d_n10: f64 = ((eq4_e1011_d_n10 * s.v[836]) + (eq4_e1011 * s.dn[836][10]));let eq4_e1013_d_n11: f64 = ((eq4_e1011_d_n11 * s.v[836]) + (eq4_e1011 * s.dn[836][11]));let eq4_e1013_d_n12: f64 = ((eq4_e1011_d_n12 * s.v[836]) + (eq4_e1011 * s.dn[836][12]));let eq4_e1013_d_b0: f64 = ((eq4_e1011_d_b0 * s.v[836]) + (eq4_e1011 * s.db[836][0]));let eq4_e1013_d_b1: f64 = ((eq4_e1011_d_b1 * s.v[836]) + (eq4_e1011 * s.db[836][1]));let eq4_e1013_d_b2: f64 = ((eq4_e1011_d_b2 * s.v[836]) + (eq4_e1011 * s.db[836][2]));
        let eq4_e1013_d_b3: f64 = ((eq4_e1011_d_b3 * s.v[836]) + (eq4_e1011 * s.db[836][3]));let eq4_e1013_d_b4: f64 = ((eq4_e1011_d_b4 * s.v[836]) + (eq4_e1011 * s.db[836][4]));let eq4_e1013_d_b5: f64 = ((eq4_e1011_d_b5 * s.v[836]) + (eq4_e1011 * s.db[836][5]));let eq4_e1013_d_b6: f64 = ((eq4_e1011_d_b6 * s.v[836]) + (eq4_e1011 * s.db[836][6]));
        (eq4_e1013, eq4_e1013_d_n0, eq4_e1013_d_n1, eq4_e1013_d_n2, eq4_e1013_d_n3, eq4_e1013_d_n4, eq4_e1013_d_n5, eq4_e1013_d_n6, eq4_e1013_d_n7, eq4_e1013_d_n8, eq4_e1013_d_n9, eq4_e1013_d_n10, eq4_e1013_d_n11, eq4_e1013_d_n12, eq4_e1013_d_b0, eq4_e1013_d_b1, eq4_e1013_d_b2, eq4_e1013_d_b3, eq4_e1013_d_b4, eq4_e1013_d_b5, eq4_e1013_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1015;let eq4_node_derivatives: [f64; 13] = [eq4_e1015_d_n0, eq4_e1015_d_n1, eq4_e1015_d_n2, eq4_e1015_d_n3, eq4_e1015_d_n4, eq4_e1015_d_n5, eq4_e1015_d_n6, eq4_e1015_d_n7, eq4_e1015_d_n8, eq4_e1015_d_n9, eq4_e1015_d_n10, eq4_e1015_d_n11, eq4_e1015_d_n12];let eq4_branch_derivatives: [f64; 7] = [eq4_e1015_d_b0, eq4_e1015_d_b1, eq4_e1015_d_b2, eq4_e1015_d_b3, eq4_e1015_d_b4, eq4_e1015_d_b5, eq4_e1015_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
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
        let (eq5_e1028, eq5_e1028_d_n0, eq5_e1028_d_n1, eq5_e1028_d_n2, eq5_e1028_d_n3, eq5_e1028_d_n4, eq5_e1028_d_n5, eq5_e1028_d_n6, eq5_e1028_d_n7, eq5_e1028_d_n8, eq5_e1028_d_n9, eq5_e1028_d_n10, eq5_e1028_d_n11, eq5_e1028_d_n12, eq5_e1028_d_b0, eq5_e1028_d_b1, eq5_e1028_d_b2, eq5_e1028_d_b3, eq5_e1028_d_b4, eq5_e1028_d_b5, eq5_e1028_d_b6,) = {
    if (!s.b[2715]) {
        let eq5_e1020: f64 = (s.v[0] * s.v[15]);let eq5_e1020_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq5_e1020_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq5_e1020_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq5_e1020_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq5_e1020_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq5_e1020_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq5_e1020_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq5_e1020_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq5_e1020_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq5_e1020_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq5_e1020_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq5_e1020_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq5_e1020_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq5_e1020_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq5_e1020_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq5_e1020_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq5_e1020_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq5_e1020_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq5_e1020_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq5_e1020_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq5_e1022: f64 = (eq5_e1020 * p.p32);let eq5_e1022_d_n0: f64 = (eq5_e1020_d_n0 * p.p32);let eq5_e1022_d_n1: f64 = (eq5_e1020_d_n1 * p.p32);let eq5_e1022_d_n2: f64 = (eq5_e1020_d_n2 * p.p32);let eq5_e1022_d_n3: f64 = (eq5_e1020_d_n3 * p.p32);let eq5_e1022_d_n4: f64 = (eq5_e1020_d_n4 * p.p32);let eq5_e1022_d_n5: f64 = (eq5_e1020_d_n5 * p.p32);let eq5_e1022_d_n6: f64 = (eq5_e1020_d_n6 * p.p32);let eq5_e1022_d_n7: f64 = (eq5_e1020_d_n7 * p.p32);let eq5_e1022_d_n8: f64 = (eq5_e1020_d_n8 * p.p32);let eq5_e1022_d_n9: f64 = (eq5_e1020_d_n9 * p.p32);let eq5_e1022_d_n10: f64 = (eq5_e1020_d_n10 * p.p32);let eq5_e1022_d_n11: f64 = (eq5_e1020_d_n11 * p.p32);let eq5_e1022_d_n12: f64 = (eq5_e1020_d_n12 * p.p32);let eq5_e1022_d_b0: f64 = (eq5_e1020_d_b0 * p.p32);let eq5_e1022_d_b1: f64 = (eq5_e1020_d_b1 * p.p32);let eq5_e1022_d_b2: f64 = (eq5_e1020_d_b2 * p.p32);let eq5_e1022_d_b3: f64 = (eq5_e1020_d_b3 * p.p32);let eq5_e1022_d_b4: f64 = (eq5_e1020_d_b4 * p.p32);let eq5_e1022_d_b5: f64 = (eq5_e1020_d_b5 * p.p32);let eq5_e1022_d_b6: f64 = (eq5_e1020_d_b6 * p.p32);let eq5_e1025: f64 = (s.v[827] + s.v[835]);let eq5_e1025_d_n0: f64 = (s.dn[827][0] + s.dn[835][0]);let eq5_e1025_d_n1: f64 = (s.dn[827][1] + s.dn[835][1]);let eq5_e1025_d_n2: f64 = (s.dn[827][2] + s.dn[835][2]);let eq5_e1025_d_n3: f64 = (s.dn[827][3] + s.dn[835][3]);let eq5_e1025_d_n4: f64 = (s.dn[827][4] + s.dn[835][4]);let eq5_e1025_d_n5: f64 = (s.dn[827][5] + s.dn[835][5]);let eq5_e1025_d_n6: f64 = (s.dn[827][6] + s.dn[835][6]);let eq5_e1025_d_n7: f64 = (s.dn[827][7] + s.dn[835][7]);let eq5_e1025_d_n8: f64 = (s.dn[827][8] + s.dn[835][8]);let eq5_e1025_d_n9: f64 = (s.dn[827][9] + s.dn[835][9]);let eq5_e1025_d_n10: f64 = (s.dn[827][10] + s.dn[835][10]);let eq5_e1025_d_n11: f64 = (s.dn[827][11] + s.dn[835][11]);let eq5_e1025_d_n12: f64 = (s.dn[827][12] + s.dn[835][12]);let eq5_e1025_d_b0: f64 = (s.db[827][0] + s.db[835][0]);let eq5_e1025_d_b1: f64 = (s.db[827][1] + s.db[835][1]);let eq5_e1025_d_b2: f64 = (s.db[827][2] + s.db[835][2]);let eq5_e1025_d_b3: f64 = (s.db[827][3] + s.db[835][3]);let eq5_e1025_d_b4: f64 = (s.db[827][4] + s.db[835][4]);let eq5_e1025_d_b5: f64 = (s.db[827][5] + s.db[835][5]);let eq5_e1025_d_b6: f64 = (s.db[827][6] + s.db[835][6]);let eq5_e1026: f64 = (eq5_e1022 * eq5_e1025);let eq5_e1026_d_n0: f64 = ((eq5_e1022_d_n0 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n0));let eq5_e1026_d_n1: f64 = ((eq5_e1022_d_n1 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n1));
        let eq5_e1026_d_n2: f64 = ((eq5_e1022_d_n2 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n2));let eq5_e1026_d_n3: f64 = ((eq5_e1022_d_n3 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n3));let eq5_e1026_d_n4: f64 = ((eq5_e1022_d_n4 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n4));let eq5_e1026_d_n5: f64 = ((eq5_e1022_d_n5 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n5));let eq5_e1026_d_n6: f64 = ((eq5_e1022_d_n6 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n6));let eq5_e1026_d_n7: f64 = ((eq5_e1022_d_n7 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n7));let eq5_e1026_d_n8: f64 = ((eq5_e1022_d_n8 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n8));let eq5_e1026_d_n9: f64 = ((eq5_e1022_d_n9 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n9));let eq5_e1026_d_n10: f64 = ((eq5_e1022_d_n10 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n10));let eq5_e1026_d_n11: f64 = ((eq5_e1022_d_n11 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n11));let eq5_e1026_d_n12: f64 = ((eq5_e1022_d_n12 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n12));let eq5_e1026_d_b0: f64 = ((eq5_e1022_d_b0 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b0));let eq5_e1026_d_b1: f64 = ((eq5_e1022_d_b1 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b1));let eq5_e1026_d_b2: f64 = ((eq5_e1022_d_b2 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b2));let eq5_e1026_d_b3: f64 = ((eq5_e1022_d_b3 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b3));let eq5_e1026_d_b4: f64 = ((eq5_e1022_d_b4 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b4));let eq5_e1026_d_b5: f64 = ((eq5_e1022_d_b5 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b5));let eq5_e1026_d_b6: f64 = ((eq5_e1022_d_b6 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b6));
        (eq5_e1026, eq5_e1026_d_n0, eq5_e1026_d_n1, eq5_e1026_d_n2, eq5_e1026_d_n3, eq5_e1026_d_n4, eq5_e1026_d_n5, eq5_e1026_d_n6, eq5_e1026_d_n7, eq5_e1026_d_n8, eq5_e1026_d_n9, eq5_e1026_d_n10, eq5_e1026_d_n11, eq5_e1026_d_n12, eq5_e1026_d_b0, eq5_e1026_d_b1, eq5_e1026_d_b2, eq5_e1026_d_b3, eq5_e1026_d_b4, eq5_e1026_d_b5, eq5_e1026_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1028;let eq5_node_derivatives: [f64; 13] = [eq5_e1028_d_n0, eq5_e1028_d_n1, eq5_e1028_d_n2, eq5_e1028_d_n3, eq5_e1028_d_n4, eq5_e1028_d_n5, eq5_e1028_d_n6, eq5_e1028_d_n7, eq5_e1028_d_n8, eq5_e1028_d_n9, eq5_e1028_d_n10, eq5_e1028_d_n11, eq5_e1028_d_n12];let eq5_branch_derivatives: [f64; 7] = [eq5_e1028_d_b0, eq5_e1028_d_b1, eq5_e1028_d_b2, eq5_e1028_d_b3, eq5_e1028_d_b4, eq5_e1028_d_b5, eq5_e1028_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_6(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq6_e1039, eq6_e1039_d_n0, eq6_e1039_d_n1, eq6_e1039_d_n2, eq6_e1039_d_n3, eq6_e1039_d_n4, eq6_e1039_d_n5, eq6_e1039_d_n6, eq6_e1039_d_n7, eq6_e1039_d_n8, eq6_e1039_d_n9, eq6_e1039_d_n10, eq6_e1039_d_n11, eq6_e1039_d_n12, eq6_e1039_d_b0, eq6_e1039_d_b1, eq6_e1039_d_b2, eq6_e1039_d_b3, eq6_e1039_d_b4, eq6_e1039_d_b5, eq6_e1039_d_b6,) = {
    if (!s.b[2715]) {
        let eq6_e1033: f64 = (s.v[0] * s.v[15]);let eq6_e1033_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq6_e1033_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq6_e1033_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq6_e1033_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq6_e1033_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq6_e1033_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq6_e1033_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq6_e1033_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq6_e1033_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq6_e1033_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq6_e1033_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq6_e1033_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq6_e1033_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq6_e1033_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq6_e1033_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq6_e1033_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq6_e1033_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq6_e1033_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq6_e1033_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq6_e1033_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq6_e1035: f64 = (eq6_e1033 * p.p32);let eq6_e1035_d_n0: f64 = (eq6_e1033_d_n0 * p.p32);let eq6_e1035_d_n1: f64 = (eq6_e1033_d_n1 * p.p32);let eq6_e1035_d_n2: f64 = (eq6_e1033_d_n2 * p.p32);let eq6_e1035_d_n3: f64 = (eq6_e1033_d_n3 * p.p32);let eq6_e1035_d_n4: f64 = (eq6_e1033_d_n4 * p.p32);let eq6_e1035_d_n5: f64 = (eq6_e1033_d_n5 * p.p32);let eq6_e1035_d_n6: f64 = (eq6_e1033_d_n6 * p.p32);let eq6_e1035_d_n7: f64 = (eq6_e1033_d_n7 * p.p32);let eq6_e1035_d_n8: f64 = (eq6_e1033_d_n8 * p.p32);let eq6_e1035_d_n9: f64 = (eq6_e1033_d_n9 * p.p32);let eq6_e1035_d_n10: f64 = (eq6_e1033_d_n10 * p.p32);let eq6_e1035_d_n11: f64 = (eq6_e1033_d_n11 * p.p32);let eq6_e1035_d_n12: f64 = (eq6_e1033_d_n12 * p.p32);let eq6_e1035_d_b0: f64 = (eq6_e1033_d_b0 * p.p32);let eq6_e1035_d_b1: f64 = (eq6_e1033_d_b1 * p.p32);let eq6_e1035_d_b2: f64 = (eq6_e1033_d_b2 * p.p32);let eq6_e1035_d_b3: f64 = (eq6_e1033_d_b3 * p.p32);let eq6_e1035_d_b4: f64 = (eq6_e1033_d_b4 * p.p32);let eq6_e1035_d_b5: f64 = (eq6_e1033_d_b5 * p.p32);let eq6_e1035_d_b6: f64 = (eq6_e1033_d_b6 * p.p32);let eq6_e1037: f64 = (eq6_e1035 * s.v[830]);let eq6_e1037_d_n0: f64 = ((eq6_e1035_d_n0 * s.v[830]) + (eq6_e1035 * s.dn[830][0]));let eq6_e1037_d_n1: f64 = ((eq6_e1035_d_n1 * s.v[830]) + (eq6_e1035 * s.dn[830][1]));let eq6_e1037_d_n2: f64 = ((eq6_e1035_d_n2 * s.v[830]) + (eq6_e1035 * s.dn[830][2]));let eq6_e1037_d_n3: f64 = ((eq6_e1035_d_n3 * s.v[830]) + (eq6_e1035 * s.dn[830][3]));let eq6_e1037_d_n4: f64 = ((eq6_e1035_d_n4 * s.v[830]) + (eq6_e1035 * s.dn[830][4]));let eq6_e1037_d_n5: f64 = ((eq6_e1035_d_n5 * s.v[830]) + (eq6_e1035 * s.dn[830][5]));let eq6_e1037_d_n6: f64 = ((eq6_e1035_d_n6 * s.v[830]) + (eq6_e1035 * s.dn[830][6]));let eq6_e1037_d_n7: f64 = ((eq6_e1035_d_n7 * s.v[830]) + (eq6_e1035 * s.dn[830][7]));let eq6_e1037_d_n8: f64 = ((eq6_e1035_d_n8 * s.v[830]) + (eq6_e1035 * s.dn[830][8]));let eq6_e1037_d_n9: f64 = ((eq6_e1035_d_n9 * s.v[830]) + (eq6_e1035 * s.dn[830][9]));let eq6_e1037_d_n10: f64 = ((eq6_e1035_d_n10 * s.v[830]) + (eq6_e1035 * s.dn[830][10]));let eq6_e1037_d_n11: f64 = ((eq6_e1035_d_n11 * s.v[830]) + (eq6_e1035 * s.dn[830][11]));let eq6_e1037_d_n12: f64 = ((eq6_e1035_d_n12 * s.v[830]) + (eq6_e1035 * s.dn[830][12]));let eq6_e1037_d_b0: f64 = ((eq6_e1035_d_b0 * s.v[830]) + (eq6_e1035 * s.db[830][0]));let eq6_e1037_d_b1: f64 = ((eq6_e1035_d_b1 * s.v[830]) + (eq6_e1035 * s.db[830][1]));let eq6_e1037_d_b2: f64 = ((eq6_e1035_d_b2 * s.v[830]) + (eq6_e1035 * s.db[830][2]));
        let eq6_e1037_d_b3: f64 = ((eq6_e1035_d_b3 * s.v[830]) + (eq6_e1035 * s.db[830][3]));let eq6_e1037_d_b4: f64 = ((eq6_e1035_d_b4 * s.v[830]) + (eq6_e1035 * s.db[830][4]));let eq6_e1037_d_b5: f64 = ((eq6_e1035_d_b5 * s.v[830]) + (eq6_e1035 * s.db[830][5]));let eq6_e1037_d_b6: f64 = ((eq6_e1035_d_b6 * s.v[830]) + (eq6_e1035 * s.db[830][6]));
        (eq6_e1037, eq6_e1037_d_n0, eq6_e1037_d_n1, eq6_e1037_d_n2, eq6_e1037_d_n3, eq6_e1037_d_n4, eq6_e1037_d_n5, eq6_e1037_d_n6, eq6_e1037_d_n7, eq6_e1037_d_n8, eq6_e1037_d_n9, eq6_e1037_d_n10, eq6_e1037_d_n11, eq6_e1037_d_n12, eq6_e1037_d_b0, eq6_e1037_d_b1, eq6_e1037_d_b2, eq6_e1037_d_b3, eq6_e1037_d_b4, eq6_e1037_d_b5, eq6_e1037_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1039;let eq6_node_derivatives: [f64; 13] = [eq6_e1039_d_n0, eq6_e1039_d_n1, eq6_e1039_d_n2, eq6_e1039_d_n3, eq6_e1039_d_n4, eq6_e1039_d_n5, eq6_e1039_d_n6, eq6_e1039_d_n7, eq6_e1039_d_n8, eq6_e1039_d_n9, eq6_e1039_d_n10, eq6_e1039_d_n11, eq6_e1039_d_n12];let eq6_branch_derivatives: [f64; 7] = [eq6_e1039_d_b0, eq6_e1039_d_b1, eq6_e1039_d_b2, eq6_e1039_d_b3, eq6_e1039_d_b4, eq6_e1039_d_b5, eq6_e1039_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_7(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq7_e1050, eq7_e1050_d_n0, eq7_e1050_d_n1, eq7_e1050_d_n2, eq7_e1050_d_n3, eq7_e1050_d_n4, eq7_e1050_d_n5, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9, eq7_e1050_d_n10, eq7_e1050_d_n11, eq7_e1050_d_n12, eq7_e1050_d_b0, eq7_e1050_d_b1, eq7_e1050_d_b2, eq7_e1050_d_b3, eq7_e1050_d_b4, eq7_e1050_d_b5, eq7_e1050_d_b6,) = {
    if (!s.b[2715]) {
        let eq7_e1044: f64 = (s.v[0] * s.v[15]);let eq7_e1044_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq7_e1044_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq7_e1044_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq7_e1044_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq7_e1044_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq7_e1044_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq7_e1044_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq7_e1044_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq7_e1044_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq7_e1044_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq7_e1044_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq7_e1044_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq7_e1044_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq7_e1044_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq7_e1044_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq7_e1044_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq7_e1044_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq7_e1044_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq7_e1044_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq7_e1044_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq7_e1046: f64 = (eq7_e1044 * p.p32);let eq7_e1046_d_n0: f64 = (eq7_e1044_d_n0 * p.p32);let eq7_e1046_d_n1: f64 = (eq7_e1044_d_n1 * p.p32);let eq7_e1046_d_n2: f64 = (eq7_e1044_d_n2 * p.p32);let eq7_e1046_d_n3: f64 = (eq7_e1044_d_n3 * p.p32);let eq7_e1046_d_n4: f64 = (eq7_e1044_d_n4 * p.p32);let eq7_e1046_d_n5: f64 = (eq7_e1044_d_n5 * p.p32);let eq7_e1046_d_n6: f64 = (eq7_e1044_d_n6 * p.p32);let eq7_e1046_d_n7: f64 = (eq7_e1044_d_n7 * p.p32);let eq7_e1046_d_n8: f64 = (eq7_e1044_d_n8 * p.p32);let eq7_e1046_d_n9: f64 = (eq7_e1044_d_n9 * p.p32);let eq7_e1046_d_n10: f64 = (eq7_e1044_d_n10 * p.p32);let eq7_e1046_d_n11: f64 = (eq7_e1044_d_n11 * p.p32);let eq7_e1046_d_n12: f64 = (eq7_e1044_d_n12 * p.p32);let eq7_e1046_d_b0: f64 = (eq7_e1044_d_b0 * p.p32);let eq7_e1046_d_b1: f64 = (eq7_e1044_d_b1 * p.p32);let eq7_e1046_d_b2: f64 = (eq7_e1044_d_b2 * p.p32);let eq7_e1046_d_b3: f64 = (eq7_e1044_d_b3 * p.p32);let eq7_e1046_d_b4: f64 = (eq7_e1044_d_b4 * p.p32);let eq7_e1046_d_b5: f64 = (eq7_e1044_d_b5 * p.p32);let eq7_e1046_d_b6: f64 = (eq7_e1044_d_b6 * p.p32);let eq7_e1048: f64 = (eq7_e1046 * s.v[831]);let eq7_e1048_d_n0: f64 = ((eq7_e1046_d_n0 * s.v[831]) + (eq7_e1046 * s.dn[831][0]));let eq7_e1048_d_n1: f64 = ((eq7_e1046_d_n1 * s.v[831]) + (eq7_e1046 * s.dn[831][1]));let eq7_e1048_d_n2: f64 = ((eq7_e1046_d_n2 * s.v[831]) + (eq7_e1046 * s.dn[831][2]));let eq7_e1048_d_n3: f64 = ((eq7_e1046_d_n3 * s.v[831]) + (eq7_e1046 * s.dn[831][3]));let eq7_e1048_d_n4: f64 = ((eq7_e1046_d_n4 * s.v[831]) + (eq7_e1046 * s.dn[831][4]));let eq7_e1048_d_n5: f64 = ((eq7_e1046_d_n5 * s.v[831]) + (eq7_e1046 * s.dn[831][5]));let eq7_e1048_d_n6: f64 = ((eq7_e1046_d_n6 * s.v[831]) + (eq7_e1046 * s.dn[831][6]));let eq7_e1048_d_n7: f64 = ((eq7_e1046_d_n7 * s.v[831]) + (eq7_e1046 * s.dn[831][7]));let eq7_e1048_d_n8: f64 = ((eq7_e1046_d_n8 * s.v[831]) + (eq7_e1046 * s.dn[831][8]));let eq7_e1048_d_n9: f64 = ((eq7_e1046_d_n9 * s.v[831]) + (eq7_e1046 * s.dn[831][9]));let eq7_e1048_d_n10: f64 = ((eq7_e1046_d_n10 * s.v[831]) + (eq7_e1046 * s.dn[831][10]));let eq7_e1048_d_n11: f64 = ((eq7_e1046_d_n11 * s.v[831]) + (eq7_e1046 * s.dn[831][11]));let eq7_e1048_d_n12: f64 = ((eq7_e1046_d_n12 * s.v[831]) + (eq7_e1046 * s.dn[831][12]));let eq7_e1048_d_b0: f64 = ((eq7_e1046_d_b0 * s.v[831]) + (eq7_e1046 * s.db[831][0]));let eq7_e1048_d_b1: f64 = ((eq7_e1046_d_b1 * s.v[831]) + (eq7_e1046 * s.db[831][1]));let eq7_e1048_d_b2: f64 = ((eq7_e1046_d_b2 * s.v[831]) + (eq7_e1046 * s.db[831][2]));
        let eq7_e1048_d_b3: f64 = ((eq7_e1046_d_b3 * s.v[831]) + (eq7_e1046 * s.db[831][3]));let eq7_e1048_d_b4: f64 = ((eq7_e1046_d_b4 * s.v[831]) + (eq7_e1046 * s.db[831][4]));let eq7_e1048_d_b5: f64 = ((eq7_e1046_d_b5 * s.v[831]) + (eq7_e1046 * s.db[831][5]));let eq7_e1048_d_b6: f64 = ((eq7_e1046_d_b6 * s.v[831]) + (eq7_e1046 * s.db[831][6]));
        (eq7_e1048, eq7_e1048_d_n0, eq7_e1048_d_n1, eq7_e1048_d_n2, eq7_e1048_d_n3, eq7_e1048_d_n4, eq7_e1048_d_n5, eq7_e1048_d_n6, eq7_e1048_d_n7, eq7_e1048_d_n8, eq7_e1048_d_n9, eq7_e1048_d_n10, eq7_e1048_d_n11, eq7_e1048_d_n12, eq7_e1048_d_b0, eq7_e1048_d_b1, eq7_e1048_d_b2, eq7_e1048_d_b3, eq7_e1048_d_b4, eq7_e1048_d_b5, eq7_e1048_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1050;let eq7_node_derivatives: [f64; 13] = [eq7_e1050_d_n0, eq7_e1050_d_n1, eq7_e1050_d_n2, eq7_e1050_d_n3, eq7_e1050_d_n4, eq7_e1050_d_n5, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9, eq7_e1050_d_n10, eq7_e1050_d_n11, eq7_e1050_d_n12];let eq7_branch_derivatives: [f64; 7] = [eq7_e1050_d_b0, eq7_e1050_d_b1, eq7_e1050_d_b2, eq7_e1050_d_b3, eq7_e1050_d_b4, eq7_e1050_d_b5, eq7_e1050_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
    }
    #[inline(never)]
    pub(super) fn stamp_transient_equations_block_8(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let eq8_e1053: f64 = (s.v[0] * s.v[15]);let eq8_e1053_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));let eq8_e1053_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));let eq8_e1053_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));let eq8_e1053_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));let eq8_e1053_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));let eq8_e1053_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));let eq8_e1053_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));let eq8_e1053_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));let eq8_e1053_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));let eq8_e1053_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));let eq8_e1053_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));let eq8_e1053_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));let eq8_e1053_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));let eq8_e1053_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));let eq8_e1053_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));let eq8_e1053_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));let eq8_e1053_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));let eq8_e1053_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));let eq8_e1053_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));let eq8_e1053_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));let eq8_e1055: f64 = (eq8_e1053 * p.p32);let eq8_e1055_d_n0: f64 = (eq8_e1053_d_n0 * p.p32);let eq8_e1055_d_n1: f64 = (eq8_e1053_d_n1 * p.p32);let eq8_e1055_d_n2: f64 = (eq8_e1053_d_n2 * p.p32);let eq8_e1055_d_n3: f64 = (eq8_e1053_d_n3 * p.p32);let eq8_e1055_d_n4: f64 = (eq8_e1053_d_n4 * p.p32);let eq8_e1055_d_n5: f64 = (eq8_e1053_d_n5 * p.p32);let eq8_e1055_d_n6: f64 = (eq8_e1053_d_n6 * p.p32);let eq8_e1055_d_n7: f64 = (eq8_e1053_d_n7 * p.p32);let eq8_e1055_d_n8: f64 = (eq8_e1053_d_n8 * p.p32);let eq8_e1055_d_n9: f64 = (eq8_e1053_d_n9 * p.p32);let eq8_e1055_d_n10: f64 = (eq8_e1053_d_n10 * p.p32);let eq8_e1055_d_n11: f64 = (eq8_e1053_d_n11 * p.p32);let eq8_e1055_d_n12: f64 = (eq8_e1053_d_n12 * p.p32);let eq8_e1055_d_b0: f64 = (eq8_e1053_d_b0 * p.p32);let eq8_e1055_d_b1: f64 = (eq8_e1053_d_b1 * p.p32);let eq8_e1055_d_b2: f64 = (eq8_e1053_d_b2 * p.p32);let eq8_e1055_d_b3: f64 = (eq8_e1053_d_b3 * p.p32);let eq8_e1055_d_b4: f64 = (eq8_e1053_d_b4 * p.p32);let eq8_e1055_d_b5: f64 = (eq8_e1053_d_b5 * p.p32);let eq8_e1055_d_b6: f64 = (eq8_e1053_d_b6 * p.p32);let eq8_e1057: f64 = (eq8_e1055 * s.v[832]);let eq8_e1057_d_n0: f64 = ((eq8_e1055_d_n0 * s.v[832]) + (eq8_e1055 * s.dn[832][0]));let eq8_e1057_d_n1: f64 = ((eq8_e1055_d_n1 * s.v[832]) + (eq8_e1055 * s.dn[832][1]));let eq8_e1057_d_n2: f64 = ((eq8_e1055_d_n2 * s.v[832]) + (eq8_e1055 * s.dn[832][2]));let eq8_e1057_d_n3: f64 = ((eq8_e1055_d_n3 * s.v[832]) + (eq8_e1055 * s.dn[832][3]));let eq8_e1057_d_n4: f64 = ((eq8_e1055_d_n4 * s.v[832]) + (eq8_e1055 * s.dn[832][4]));let eq8_e1057_d_n5: f64 = ((eq8_e1055_d_n5 * s.v[832]) + (eq8_e1055 * s.dn[832][5]));let eq8_e1057_d_n6: f64 = ((eq8_e1055_d_n6 * s.v[832]) + (eq8_e1055 * s.dn[832][6]));let eq8_e1057_d_n7: f64 = ((eq8_e1055_d_n7 * s.v[832]) + (eq8_e1055 * s.dn[832][7]));let eq8_e1057_d_n8: f64 = ((eq8_e1055_d_n8 * s.v[832]) + (eq8_e1055 * s.dn[832][8]));let eq8_e1057_d_n9: f64 = ((eq8_e1055_d_n9 * s.v[832]) + (eq8_e1055 * s.dn[832][9]));let eq8_e1057_d_n10: f64 = ((eq8_e1055_d_n10 * s.v[832]) + (eq8_e1055 * s.dn[832][10]));let eq8_e1057_d_n11: f64 = ((eq8_e1055_d_n11 * s.v[832]) + (eq8_e1055 * s.dn[832][11]));let eq8_e1057_d_n12: f64 = ((eq8_e1055_d_n12 * s.v[832]) + (eq8_e1055 * s.dn[832][12]));let eq8_e1057_d_b0: f64 = ((eq8_e1055_d_b0 * s.v[832]) + (eq8_e1055 * s.db[832][0]));let eq8_e1057_d_b1: f64 = ((eq8_e1055_d_b1 * s.v[832]) + (eq8_e1055 * s.db[832][1]));let eq8_e1057_d_b2: f64 = ((eq8_e1055_d_b2 * s.v[832]) + (eq8_e1055 * s.db[832][2]));
        let eq8_e1057_d_b3: f64 = ((eq8_e1055_d_b3 * s.v[832]) + (eq8_e1055 * s.db[832][3]));let eq8_e1057_d_b4: f64 = ((eq8_e1055_d_b4 * s.v[832]) + (eq8_e1055 * s.db[832][4]));let eq8_e1057_d_b5: f64 = ((eq8_e1055_d_b5 * s.v[832]) + (eq8_e1055 * s.db[832][5]));let eq8_e1057_d_b6: f64 = ((eq8_e1055_d_b6 * s.v[832]) + (eq8_e1055 * s.db[832][6]));let eq8_value: f64 = eq8_e1057;let eq8_node_derivatives: [f64; 13] = [eq8_e1057_d_n0, eq8_e1057_d_n1, eq8_e1057_d_n2, eq8_e1057_d_n3, eq8_e1057_d_n4, eq8_e1057_d_n5, eq8_e1057_d_n6, eq8_e1057_d_n7, eq8_e1057_d_n8, eq8_e1057_d_n9, eq8_e1057_d_n10, eq8_e1057_d_n11, eq8_e1057_d_n12];let eq8_branch_derivatives: [f64; 7] = [eq8_e1057_d_b0, eq8_e1057_d_b1, eq8_e1057_d_b2, eq8_e1057_d_b3, eq8_e1057_d_b4, eq8_e1057_d_b5, eq8_e1057_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(9),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
    }
}
