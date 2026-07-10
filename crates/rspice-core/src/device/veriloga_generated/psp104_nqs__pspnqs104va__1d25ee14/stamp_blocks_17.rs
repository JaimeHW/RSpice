#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2684] = (p.p874 > 0.0);s.store_scalar(2684, if s.b[2684] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2684]) {s.store_scaled_offset_ad(644, A::powf(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt_square_offset(A::add(s.ad_value(825), s.ad_value(827)), (0.001 * 0.001)), 0.5), p.p875), (-(((0.5 * 0.001)) as f64).powf(p.p875)), p.p874);s.store_mul_scale_offset_indices(443, 443, 644, 1.0, 1.0);}
        if (s.b[2665] && (!s.b[2666])) {s.store_scalar(2634, 0.0);s.store_scalar(2631, 0.0);}
        s.b[2685] = (!(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)));s.store_scalar(2685, if s.b[2685] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2685]) {s.store_primal_scaled_mul(2621, 657, 657, 4.0);s.store_primal_div(2622, 657, 658);s.store_add_scaled_product_indices(2623, 832, 1.0, 657, 2622, 1.0);s.store_add(2624, 658, 2623);s.store_sub(2625, 658, 2623);s.store_sqrt_square_add(2626, 2625, 2621);s.store_div_scaled_product_add_scaled_denominator_indices(2628, 832, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
        s.b[2686] = (s.v[832] < s.v[654]);s.store_scalar(2686, if s.b[2686] { 1.0 } else { 0.0 });s.b[2687] = (((((-0.5) * (s.v[832] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(2687, if s.b[2687] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) && s.b[2687]) {s.store_exp_scaled_input(2629, 832, (s.v[371] * (-0.5)));}
        s.b[2688] = (((-0.5) * (s.v[832] * s.v[371])) < 0.0);s.store_scalar(2688, if s.b[2688] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) && (!s.b[2687])) && s.b[2688]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2629, 1e-100, (-230.25850929940458), A::scale(s.ad_value(832), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) && (!s.b[2687])) && (!s.b[2688])) {s.store_scaled_offset_ad(2629, A::mul_offset_rhs(A::scale_offset(s.ad_value(832), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(832), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(832), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2686]) {s.store_div_from_scalar(2630, 1.0, 2629);s.store_square(2627, 2630);}
        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && (!s.b[2686])) {s.store_mul_scale_offset_mixed_ia(2627, 655, A::sub_scaled_inputs(s.ad_value(832), s.v[371], s.ad_value(654), s.v[371]), 1.0, 1.0);s.store_sqrt(2630, 2627);s.store_div_from_scalar(2629, 1.0, 2630);}
        if ((s.b[2665] && (!s.b[2666])) && s.b[2685]) {s.store_offset(2627, 2627, (-1.0));}
        s.b[2689] = (s.v[832] > 0.0);s.store_scalar(2689, if s.b[2689] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && s.b[2689]) {s.store_scaled_ln_ad(2631, A::add(A::offset(s.ad_value(2629), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2629), 1.0, A::offset(s.ad_value(2629), 3.0)))), (s.v[370] * 2.0));}
        if (((s.b[2665] && (!s.b[2666])) && s.b[2685]) && (!s.b[2689])) {s.store_sub_mixed_ai(2631, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2630), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2630), 1.0, A::scale_offset(s.ad_value(2630), 3.0, 1.0))))), (s.v[370] * 2.0)), 832);}
        if ((s.b[2665] && (!s.b[2666])) && s.b[2685]) {s.store_sub(2632, 656, 2631);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2633, 832, 0.5, 2632, 0.5, 832, 2632, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2634, 832, 0.5, 659, 0.5, 832, 659, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_scaled_sub_mixed_ia(2635, 832, A::sqrt_square_offset(s.ad_value(832), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[2690] = (s.v[646] == 0.0);s.store_scalar(2690, if s.b[2690] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2690]) {s.store_scalar(1918, 0.0);}
        s.b[2691] = ((p.p840 == 0.0) && (p.p845 == 0.0));s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {s.store_sub_from_scalar(2639, s.v[393], 2633);}
        s.b[2693] = (p.p831 == 0.5);s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && s.b[2693]) {s.store_sqrt_scaled_input(2636, 2639, s.v[429]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && (!s.b[2693])) {s.store_powf_scaled_input(2636, 2639, s.v[429], p.p831);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {s.store_scale(2643, 2636, s.v[423]);}
        s.b[2694] = (p.p845 == 0.0);s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) {s.store_div_scaled_inputs_indices(2646, 2643, (s.v[408] * s.v[438]), 2639, 1.0);s.store_div_from_scalar(2647, (0.666666666666667 * s.v[435]), 2646);s.store_square(2648, 2647);s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);s.store_sqrt(2650, 2649);s.store_mul(2651, 2649, 2650);s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);s.store_add_scaled_value_products_indices(2656, 2649, (-s.v[435]), 2647, 2650, s.v[435], 2646, 2651, 0.5);s.store_mul_scale_offset_indices(2657, 2654, 2655, 1.0, (-1.0));s.store_square(2618, 2657);}
        s.b[2697] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));s.store_scalar(2697, if s.b[2697] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && s.b[2697]) {s.store_exp_sub(2636, 2656, 2618);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2697])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2698] = (s.v[2657] > 0.0);s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });s.b[2699] = (s.v[2656] > (-230.25850929940458));s.store_scalar(2699, if s.b[2699] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2699]) {s.store_exp(2636, 2656);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2699])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2700] = (p.p851 == 0.0);s.store_scalar(2700, if s.b[2700] { 1.0 } else { 0.0 });s.b[2701] = (p.p831 == 0.5);s.store_scalar(2701, if s.b[2701] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2701]) {s.store_sqrt_scaled_input_ad(2636, A::sub_from_scalar(p.p828, s.ad_value(2634)), s.v[429]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2701])) {s.store_powf_scale_offset_input(2636, 2634, (-s.v[429]), ((p.p828) * (s.v[429])), p.p831);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) {s.store_div_scaled_offset_numerator_indices(2661, 2634, ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), 2636, 1.0);}
        s.b[2702] = (((((-s.v[441]) / s.v[2661])) as f64).abs() < 230.25850929940458);s.store_scalar(2702, if s.b[2702] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2702]) {s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(2661), 1.0));}
        s.b[2703] = (((-s.v[441]) / s.v[2661]) < 0.0);s.store_scalar(2703, if s.b[2703] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2702])) && s.b[2703]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 441, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2702])) && (!s.b[2703])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 441, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2704] = (p.p860 > 1000.0);s.store_scalar(2704, if s.b[2704] { 1.0 } else { 0.0 });s.b[2705] = (s.v[2635] > ((-s.v[444]) * p.p860));s.store_scalar(2705, if s.b[2705] { 1.0 } else { 0.0 });s.b[2706] = (p.p863 == 4.0);s.store_scalar(2706, if s.b[2706] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2704])) && s.b[2705]) && s.b[2706]) {s.store_mul_scale_offset_mixed_ai(2636, A::mul3_scaled_output(s.ad_value(2635), s.ad_value(2635), s.ad_value(2635), ((s.v[448] * s.v[448]) * s.v[448])), 2635, s.v[448], 0.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2704])) && s.b[2705]) && (!s.b[2706])) {s.store_powf_ad(2636, A::abs_scaled_input(s.ad_value(2635), s.v[448]), p.p863);}
        s.b[2707] = (s.v[408] == 0.5);s.store_scalar(2707, if s.b[2707] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2707]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[405]));}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2707])) {s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[405])), s.v[408]);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) {s.store_add_scaled_inputs3_offset_indices(1918, 2636, ((-s.v[417]) * p.p30), 832, (s.v[420] * p.p30), 2628, ((-s.v[420]) * p.p30), (s.v[417] * p.p30));}
        s.b[2708] = (s.v[647] == 0.0);s.store_scalar(2708, if s.b[2708] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2708]) {s.store_scalar(1919, 0.0);}
        s.b[2709] = ((p.p841 == 0.0) && (p.p846 == 0.0));s.store_scalar(2709, if s.b[2709] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) {s.store_sub_from_scalar(2639, s.v[394], 2633);}
        s.b[2711] = (p.p832 == 0.5);s.store_scalar(2711, if s.b[2711] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) && s.b[2711]) {s.store_sqrt_scaled_input(2636, 2639, s.v[430]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) && (!s.b[2711])) {s.store_powf_scaled_input(2636, 2639, s.v[430], p.p832);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) {s.store_scale(2643, 2636, s.v[424]);}
        s.b[2712] = (p.p846 == 0.0);s.store_scalar(2712, if s.b[2712] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) {s.store_div_scaled_inputs_indices(2646, 2643, (s.v[409] * s.v[439]), 2639, 1.0);s.store_div_from_scalar(2647, (0.666666666666667 * s.v[436]), 2646);s.store_square(2648, 2647);s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);s.store_sqrt(2650, 2649);s.store_mul(2651, 2649, 2650);s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);s.store_add_scaled_value_products_indices(2656, 2649, (-s.v[436]), 2647, 2650, s.v[436], 2646, 2651, 0.5);s.store_mul_scale_offset_indices(2657, 2654, 2655, 1.0, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) {s.store_square(2618, 2657);}
        s.b[2715] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));s.store_scalar(2715, if s.b[2715] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && s.b[2715]) {s.store_exp_sub(2636, 2656, 2618);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2715])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2716] = (s.v[2657] > 0.0);s.store_scalar(2716, if s.b[2716] { 1.0 } else { 0.0 });s.b[2717] = (s.v[2656] > (-230.25850929940458));s.store_scalar(2717, if s.b[2717] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2716])) && s.b[2717]) {s.store_exp(2636, 2656);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2716])) && (!s.b[2717])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2718] = (p.p852 == 0.0);s.store_scalar(2718, if s.b[2718] { 1.0 } else { 0.0 });s.b[2719] = (p.p832 == 0.5);s.store_scalar(2719, if s.b[2719] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && s.b[2719]) {s.store_sqrt_scaled_input_ad(2636, A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[430]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2719])) {s.store_powf_scale_offset_input(2636, 2634, (-s.v[430]), ((p.p829) * (s.v[430])), p.p832);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) {s.store_div_scaled_offset_numerator_indices(2661, 2634, ((-s.v[427]) * s.v[412]), (((p.p829) * (s.v[427])) * s.v[412]), 2636, 1.0);}
        s.b[2720] = (((((-s.v[442]) / s.v[2661])) as f64).abs() < 230.25850929940458);s.store_scalar(2720, if s.b[2720] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && s.b[2720]) {s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2661), 1.0));}
        s.b[2721] = (((-s.v[442]) / s.v[2661]) < 0.0);s.store_scalar(2721, if s.b[2721] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2720])) && s.b[2721]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 442, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2720])) && (!s.b[2721])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 442, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2722] = (p.p861 > 1000.0);s.store_scalar(2722, if s.b[2722] { 1.0 } else { 0.0 });s.b[2723] = (s.v[2635] > ((-s.v[444]) * p.p861));s.store_scalar(2723, if s.b[2723] { 1.0 } else { 0.0 });s.b[2724] = (p.p864 == 4.0);s.store_scalar(2724, if s.b[2724] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2722])) && s.b[2723]) && s.b[2724]) {s.store_mul_scale_offset_mixed_ai(2636, A::mul3_scaled_output(s.ad_value(2635), s.ad_value(2635), s.ad_value(2635), ((s.v[449] * s.v[449]) * s.v[449])), 2635, s.v[449], 0.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2722])) && s.b[2723]) && (!s.b[2724])) {s.store_powf_ad(2636, A::abs_scaled_input(s.ad_value(2635), s.v[449]), p.p864);}
        s.b[2725] = (s.v[409] == 0.5);s.store_scalar(2725, if s.b[2725] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && s.b[2725]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[406]));}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2725])) {s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[406])), s.v[409]);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) {s.store_add_scaled_inputs3_offset_indices(1919, 2636, ((-s.v[418]) * p.p30), 832, (s.v[421] * p.p30), 2628, ((-s.v[421]) * p.p30), (s.v[418] * p.p30));}
        s.b[2726] = (s.v[648] == 0.0);s.store_scalar(2726, if s.b[2726] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2726]) {s.store_scalar(1920, 0.0);}
        s.b[2727] = ((p.p842 == 0.0) && (p.p847 == 0.0));s.store_scalar(2727, if s.b[2727] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) {s.store_sub_from_scalar(2639, s.v[395], 2633);}
        s.b[2729] = (p.p833 == 0.5);s.store_scalar(2729, if s.b[2729] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) && s.b[2729]) {s.store_sqrt_scaled_input(2636, 2639, s.v[431]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) && (!s.b[2729])) {s.store_powf_scaled_input(2636, 2639, s.v[431], p.p833);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) {s.store_scale(2643, 2636, s.v[425]);}
        s.b[2730] = (p.p847 == 0.0);s.store_scalar(2730, if s.b[2730] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) {s.store_div_scaled_inputs_indices(2646, 2643, (s.v[410] * s.v[440]), 2639, 1.0);s.store_div_from_scalar(2647, (0.666666666666667 * s.v[437]), 2646);s.store_square(2648, 2647);s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);s.store_sqrt(2650, 2649);s.store_mul(2651, 2649, 2650);s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);s.store_add_scaled_value_products_indices(2656, 2649, (-s.v[437]), 2647, 2650, s.v[437], 2646, 2651, 0.5);s.store_mul_scale_offset_indices(2657, 2654, 2655, 1.0, (-1.0));s.store_square(2618, 2657);}
        s.b[2733] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));s.store_scalar(2733, if s.b[2733] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && s.b[2733]) {s.store_exp_sub(2636, 2656, 2618);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2733])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2734] = (s.v[2657] > 0.0);s.store_scalar(2734, if s.b[2734] { 1.0 } else { 0.0 });s.b[2735] = (s.v[2656] > (-230.25850929940458));s.store_scalar(2735, if s.b[2735] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2734])) && s.b[2735]) {s.store_exp(2636, 2656);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2734])) && (!s.b[2735])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2736] = (p.p853 == 0.0);s.store_scalar(2736, if s.b[2736] { 1.0 } else { 0.0 });s.b[2737] = (p.p833 == 0.5);s.store_scalar(2737, if s.b[2737] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && s.b[2737]) {s.store_sqrt_scaled_input_ad(2636, A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[431]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2737])) {s.store_powf_scale_offset_input(2636, 2634, (-s.v[431]), ((p.p830) * (s.v[431])), p.p833);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) {s.store_div_scaled_offset_numerator_indices(2661, 2634, ((-s.v[428]) * s.v[413]), (((p.p830) * (s.v[428])) * s.v[413]), 2636, 1.0);}
        s.b[2738] = (((((-s.v[443]) / s.v[2661])) as f64).abs() < 230.25850929940458);s.store_scalar(2738, if s.b[2738] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && s.b[2738]) {s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2661), 1.0));}
        s.b[2739] = (((-s.v[443]) / s.v[2661]) < 0.0);s.store_scalar(2739, if s.b[2739] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2738])) && s.b[2739]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 443, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2738])) && (!s.b[2739])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 443, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2740] = (s.v[640] > 1000.0);s.store_scalar(2740, if s.b[2740] { 1.0 } else { 0.0 });s.b[2741] = (s.v[2635] > ((-s.v[444]) * s.v[640]));s.store_scalar(2741, if s.b[2741] { 1.0 } else { 0.0 });s.b[2742] = (p.p865 == 4.0);s.store_scalar(2742, if s.b[2742] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2740])) && s.b[2741]) && s.b[2742]) {s.store_mul_ad_product_lhs_mixed_ai(2636, A::mul3(A::square(A::mul(s.ad_value(2635), s.ad_value(450))), s.ad_value(2635), s.ad_value(450)), 2635, 450);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2740])) && s.b[2741]) && (!s.b[2742])) {s.store_powf_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(450))), p.p865);}
        s.b[2743] = (s.v[473] == 1.0);s.store_scalar(2743, if s.b[2743] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            if (s.v[832] < p.p870) {
                if (((s.v[832] - p.p870) / p.p871) < (-37.0)) {
                    s.store_scalar(2663, p.p870);
                } else {
                    s.store_offset_scaled_ad(2663, A::ln_one_plus_exp(A::scaled_offset(s.ad_value(832), (-p.p870), 1.0 / (p.p871))), p.p871, p.p870);
                }
            } else {
                if (((s.v[832] - p.p870) / p.p871) > 37.0) {
                    s.copy_ad(2663, 832);
                } else {
                    s.store_add_scaled_inputs_mixed_ia(2663, 832, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(832), (-1.0 / (p.p871)), ((p.p870) * (1.0 / (p.p871))))), p.p871);
                }
            }
        }
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {s.store_primal_scaled_mul(2621, 657, 657, 4.0);s.store_primal_div(2622, 657, 658);s.store_add_scaled_product_indices(2623, 2663, 1.0, 657, 2622, 1.0);s.store_add(2624, 658, 2623);s.store_sub(2625, 658, 2623);s.store_sqrt_square_add(2626, 2625, 2621);s.store_div_scaled_product_add_scaled_denominator_indices(2664, 2663, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
        s.b[2744] = (s.v[410] == 0.5);s.store_scalar(2744, if s.b[2744] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && s.b[2744]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2664), s.v[407]));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && (!s.b[2744])) {s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2664), s.v[407])), s.v[410]);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {s.store_add_scaled_inputs3_offset_indices(1920, 2636, ((-s.v[419]) * p.p30), 2663, (s.v[422] * p.p30), 2664, ((-s.v[422]) * p.p30), (s.v[419] * p.p30));s.store_sub_offset_lhs(2663, 832, p.p870, 2663);s.store_primal_scaled_mul(2621, 657, 657, 4.0);s.store_primal_div(2622, 657, 658);s.store_add_scaled_product_indices(2623, 2663, 1.0, 657, 2622, 1.0);s.store_add(2624, 658, 2623);s.store_sub(2625, 658, 2623);s.store_sqrt_square_add(2626, 2625, 2621);s.store_div_scaled_product_add_scaled_denominator_indices(2664, 2663, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
        s.b[2745] = (s.v[467] == 0.5);s.store_scalar(2745, if s.b[2745] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && s.b[2745]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(466)));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && (!s.b[2745])) {s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2664, 466, 467);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {s.store_add_scaled_product_mixed_aia(472, A::mul_sub_from_scalar_rhs(s.ad_value(470), 1.0, s.ad_value(2636)), p.p30, 471, A::sub(s.ad_value(2663), s.ad_value(2664)), p.p30);s.store_add(1920, 1920, 472);}
        s.b[2746] = (s.v[410] == 0.5);s.store_scalar(2746, if s.b[2746] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) && s.b[2746]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[407]));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) && (!s.b[2746])) {s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[407])), s.v[410]);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) {s.store_add_scaled_inputs3_offset_indices(1920, 2636, ((-s.v[419]) * p.p30), 832, (s.v[422] * p.p30), 2628, ((-s.v[422]) * p.p30), (s.v[419] * p.p30));}
        s.b[2747] = (s.v[636] > 0.0);s.store_scalar(2747, if s.b[2747] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2747]) {s.store_mul_sub_mixed_iaa(643, 636, A::pow(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt_square_offset(A::add(s.ad_value(825), s.ad_value(827)), (0.001 * 0.001)), 0.5), s.ad_value(637)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(637)));s.store_add(641, 542, 643);s.store_div_from_scalar(616, 1.0, 641);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2747])) {s.copy_ad(641, 542);}
        s.b[2748] = (s.v[638] > 0.0);s.store_scalar(2748, if s.b[2748] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2748]) {s.store_mul_sub_mixed_iaa(645, 638, A::pow(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt_square_offset(A::add(s.ad_value(825), s.ad_value(827)), (0.001 * 0.001)), 0.5), s.ad_value(639)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(639)));s.store_mul_scale_offset_indices(610, 610, 645, 1.0, 1.0);}
        if (s.b[2665] && (!s.b[2666])) {s.store_scalar(2634, 0.0);s.store_scalar(2631, 0.0);}
        s.b[2749] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));s.store_scalar(2749, if s.b[2749] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2749]) {s.store_primal_scaled_mul(2621, 684, 684, 4.0);s.store_primal_div(2622, 684, 685);s.store_add_scaled_product_indices(2623, 833, 1.0, 684, 2622, 1.0);s.store_add(2624, 685, 2623);s.store_sub(2625, 685, 2623);s.store_sqrt_square_add(2626, 2625, 2621);s.store_div_scaled_product_add_scaled_denominator_indices(2628, 833, 685, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
        s.b[2750] = (s.v[833] < s.v[681]);s.store_scalar(2750, if s.b[2750] { 1.0 } else { 0.0 });s.b[2751] = (((((-0.5) * (s.v[833] * s.v[371]))) as f64).abs() < 230.25850929940458);s.store_scalar(2751, if s.b[2751] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) && s.b[2751]) {s.store_exp_scaled_input(2629, 833, (s.v[371] * (-0.5)));}
        s.b[2752] = (((-0.5) * (s.v[833] * s.v[371])) < 0.0);s.store_scalar(2752, if s.b[2752] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) && (!s.b[2751])) && s.b[2752]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2629, 1e-100, (-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) && (!s.b[2751])) && (!s.b[2752])) {s.store_scaled_offset_ad(2629, A::mul_offset_rhs(A::scale_offset(s.ad_value(833), (s.v[371] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(833), (s.v[371] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(833), (((s.v[371] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) {s.store_div_from_scalar(2630, 1.0, 2629);s.store_square(2627, 2630);}
        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && (!s.b[2750])) {s.store_mul_scale_offset_mixed_ia(2627, 682, A::sub_scaled_inputs(s.ad_value(833), s.v[371], s.ad_value(681), s.v[371]), 1.0, 1.0);s.store_sqrt(2630, 2627);s.store_div_from_scalar(2629, 1.0, 2630);}
        if ((s.b[2665] && (!s.b[2666])) && s.b[2749]) {s.store_offset(2627, 2627, (-1.0));}
        s.b[2753] = (s.v[833] > 0.0);s.store_scalar(2753, if s.b[2753] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_67(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2753]) {s.store_scaled_ln_ad(2631, A::add(A::offset(s.ad_value(2629), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2629), 1.0, A::offset(s.ad_value(2629), 3.0)))), (s.v[370] * 2.0));}
        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && (!s.b[2753])) {s.store_sub_mixed_ai(2631, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2630), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2630), 1.0, A::scale_offset(s.ad_value(2630), 3.0, 1.0))))), (s.v[370] * 2.0)), 833);}
        if ((s.b[2665] && (!s.b[2666])) && s.b[2749]) {s.store_sub(2632, 683, 2631);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2633, 833, 0.5, 2632, 0.5, 833, 2632, ((4.0 * s.v[370]) * s.v[370]), (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2634, 833, 0.5, 686, 0.5, 833, 686, ((4.0 * s.v[368]) * s.v[368]), (-0.5));s.store_scaled_sub_mixed_ia(2635, 833, A::sqrt_square_offset(s.ad_value(833), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[2754] = (s.v[673] == 0.0);s.store_scalar(2754, if s.b[2754] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2754]) {s.store_scalar(1921, 0.0);}
        s.b[2755] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));s.store_scalar(2755, if s.b[2755] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) {s.store_sub(2639, 569, 2633);}
        s.b[2757] = (s.v[511] == 0.5);s.store_scalar(2757, if s.b[2757] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) && s.b[2757]) {s.store_sqrt_mul(2636, 2639, 596);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) && (!s.b[2757])) {s.store_pow_mul_base_indices(2636, 2639, 596, 511);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) {s.store_mul(2643, 590, 2636);}
        s.b[2758] = (s.v[525] == 0.0);s.store_scalar(2758, if s.b[2758] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) {s.store_mul_div_scaled_product_indices(2646, 605, 2643, 575, 1.0, 2639, 1.0);s.store_div_scaled_inputs_indices(2647, 602, 0.666666666666667, 2646, 1.0);s.store_square(2648, 2647);s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);s.store_sqrt(2650, 2649);s.store_mul(2651, 2649, 2650);s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);s.store_add_scaled_value_products_mixed_aiiii(2656, A::mul3(s.ad_value(602), s.ad_value(2647), s.ad_value(2650)), 1.0, 602, 2649, (-1.0), 2646, 2651, 0.5);s.store_mul_scale_offset_indices(2657, 2654, 2655, 1.0, (-1.0));s.store_square(2618, 2657);}
        s.b[2761] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));s.store_scalar(2761, if s.b[2761] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && s.b[2761]) {s.store_exp_sub(2636, 2656, 2618);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2761])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2762] = (s.v[2657] > 0.0);s.store_scalar(2762, if s.b[2762] { 1.0 } else { 0.0 });s.b[2763] = (s.v[2656] > (-230.25850929940458));s.store_scalar(2763, if s.b[2763] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2762])) && s.b[2763]) {s.store_exp(2636, 2656);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2762])) && (!s.b[2763])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2764] = (s.v[531] == 0.0);s.store_scalar(2764, if s.b[2764] { 1.0 } else { 0.0 });s.b[2765] = (s.v[511] == 0.5);s.store_scalar(2765, if s.b[2765] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && s.b[2765]) {s.store_sqrt_mul_sub_lhs(2636, 508, 2634, 596);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && (!s.b[2765])) {s.store_pow_mul_base_mixed_ai(2636, A::sub(s.ad_value(508), s.ad_value(2634)), 596, 511);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) {s.store_mul_div_scaled_product_mixed_iaii(2661, 578, A::sub(s.ad_value(508), s.ad_value(2634)), 593, 1.0, 2636, 1.0);}
        s.b[2766] = (((((-s.v[608]) / s.v[2661])) as f64).abs() < 230.25850929940458);s.store_scalar(2766, if s.b[2766] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && s.b[2766]) {s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(608), -1.0, s.ad_value(2661), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_68(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2767] = (((-s.v[608]) / s.v[2661]) < 0.0);s.store_scalar(2767, if s.b[2767] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && (!s.b[2766])) && s.b[2767]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 608, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && (!s.b[2766])) && (!s.b[2767])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 608, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2768] = (s.v[540] > 1000.0);s.store_scalar(2768, if s.b[2768] { 1.0 } else { 0.0 });s.b[2769] = (s.v[2635] > ((-s.v[444]) * s.v[540]));s.store_scalar(2769, if s.b[2769] { 1.0 } else { 0.0 });s.b[2770] = (s.v[543] == 4.0);s.store_scalar(2770, if s.b[2770] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2768])) && s.b[2769]) && s.b[2770]) {s.store_mul_ad_product_lhs_mixed_ai(2636, A::mul3(A::square(A::mul(s.ad_value(2635), s.ad_value(614))), s.ad_value(2635), s.ad_value(614)), 2635, 614);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2768])) && s.b[2769]) && (!s.b[2770])) {s.store_pow_abs_mul_base_indices(2636, 2635, 614, 543);}
        s.b[2771] = (s.v[575] == 0.5);s.store_scalar(2771, if s.b[2771] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && s.b[2771]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(572)));}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2771])) {s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2628, 572, 575);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) {s.store_add_scaled_product_mixed_aia(1921, A::mul_sub_from_scalar_rhs(s.ad_value(584), 1.0, s.ad_value(2636)), p.p30, 587, A::sub(s.ad_value(833), s.ad_value(2628)), p.p30);}
        s.b[2772] = (s.v[674] == 0.0);s.store_scalar(2772, if s.b[2772] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2772]) {s.store_scalar(1922, 0.0);}
        s.b[2773] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));s.store_scalar(2773, if s.b[2773] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) {s.store_sub(2639, 570, 2633);}
        s.b[2775] = (s.v[512] == 0.5);s.store_scalar(2775, if s.b[2775] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) && s.b[2775]) {s.store_sqrt_mul(2636, 2639, 597);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) && (!s.b[2775])) {s.store_pow_mul_base_indices(2636, 2639, 597, 512);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) {s.store_mul(2643, 591, 2636);}
        s.b[2776] = (s.v[526] == 0.0);s.store_scalar(2776, if s.b[2776] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) {s.store_mul_div_scaled_product_indices(2646, 606, 2643, 576, 1.0, 2639, 1.0);s.store_div_scaled_inputs_indices(2647, 603, 0.666666666666667, 2646, 1.0);s.store_square(2648, 2647);s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);s.store_sqrt(2650, 2649);s.store_mul(2651, 2649, 2650);s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);s.store_add_scaled_value_products_mixed_aiiii(2656, A::mul3(s.ad_value(603), s.ad_value(2647), s.ad_value(2650)), 1.0, 603, 2649, (-1.0), 2646, 2651, 0.5);s.store_mul_scale_offset_indices(2657, 2654, 2655, 1.0, (-1.0));s.store_square(2618, 2657);}
        s.b[2779] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));s.store_scalar(2779, if s.b[2779] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && s.b[2779]) {s.store_exp_sub(2636, 2656, 2618);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2779])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2780] = (s.v[2657] > 0.0);s.store_scalar(2780, if s.b[2780] { 1.0 } else { 0.0 });s.b[2781] = (s.v[2656] > (-230.25850929940458));s.store_scalar(2781, if s.b[2781] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2780])) && s.b[2781]) {s.store_exp(2636, 2656);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2780])) && (!s.b[2781])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2782] = (s.v[532] == 0.0);s.store_scalar(2782, if s.b[2782] { 1.0 } else { 0.0 });s.b[2783] = (s.v[512] == 0.5);s.store_scalar(2783, if s.b[2783] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && s.b[2783]) {s.store_sqrt_mul_sub_lhs(2636, 509, 2634, 597);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_69(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2783])) {s.store_pow_mul_base_mixed_ai(2636, A::sub(s.ad_value(509), s.ad_value(2634)), 597, 512);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) {s.store_mul_div_scaled_product_mixed_iaii(2661, 579, A::sub(s.ad_value(509), s.ad_value(2634)), 594, 1.0, 2636, 1.0);}
        s.b[2784] = (((((-s.v[609]) / s.v[2661])) as f64).abs() < 230.25850929940458);s.store_scalar(2784, if s.b[2784] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && s.b[2784]) {s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2661), 1.0));}
        s.b[2785] = (((-s.v[609]) / s.v[2661]) < 0.0);s.store_scalar(2785, if s.b[2785] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2784])) && s.b[2785]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 609, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2784])) && (!s.b[2785])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 609, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2786] = (s.v[541] > 1000.0);s.store_scalar(2786, if s.b[2786] { 1.0 } else { 0.0 });s.b[2787] = (s.v[2635] > ((-s.v[444]) * s.v[541]));s.store_scalar(2787, if s.b[2787] { 1.0 } else { 0.0 });s.b[2788] = (s.v[544] == 4.0);s.store_scalar(2788, if s.b[2788] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && s.b[2787]) && s.b[2788]) {s.store_mul_ad_product_lhs_mixed_ai(2636, A::mul3(A::square(A::mul(s.ad_value(2635), s.ad_value(615))), s.ad_value(2635), s.ad_value(615)), 2635, 615);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && s.b[2787]) && (!s.b[2788])) {s.store_pow_abs_mul_base_indices(2636, 2635, 615, 544);}
        s.b[2789] = (s.v[576] == 0.5);s.store_scalar(2789, if s.b[2789] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && s.b[2789]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(573)));}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2789])) {s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2628, 573, 576);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) {s.store_add_scaled_product_mixed_aia(1922, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2636)), p.p30, 588, A::sub(s.ad_value(833), s.ad_value(2628)), p.p30);}
        s.b[2790] = (s.v[675] == 0.0);s.store_scalar(2790, if s.b[2790] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2790]) {s.store_scalar(1923, 0.0);}
        s.b[2791] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));s.store_scalar(2791, if s.b[2791] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {s.store_sub(2639, 571, 2633);}
        s.b[2793] = (s.v[513] == 0.5);s.store_scalar(2793, if s.b[2793] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && s.b[2793]) {s.store_sqrt_mul(2636, 2639, 598);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && (!s.b[2793])) {s.store_pow_mul_base_indices(2636, 2639, 598, 513);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {s.store_mul(2643, 592, 2636);}
        s.b[2794] = (s.v[527] == 0.0);s.store_scalar(2794, if s.b[2794] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) {s.store_mul_div_scaled_product_indices(2646, 607, 2643, 577, 1.0, 2639, 1.0);s.store_div_scaled_inputs_indices(2647, 604, 0.666666666666667, 2646, 1.0);s.store_square(2648, 2647);s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);s.store_sqrt(2650, 2649);s.store_mul(2651, 2649, 2650);s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);s.store_add_scaled_value_products_mixed_aiiii(2656, A::mul3(s.ad_value(604), s.ad_value(2647), s.ad_value(2650)), 1.0, 604, 2649, (-1.0), 2646, 2651, 0.5);s.store_mul_scale_offset_indices(2657, 2654, 2655, 1.0, (-1.0));s.store_square(2618, 2657);}
        s.b[2797] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));s.store_scalar(2797, if s.b[2797] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && s.b[2797]) {s.store_exp_sub(2636, 2656, 2618);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2797])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_70(
        s: &mut ReactiveScratch,
    ) {
        s.b[2798] = (s.v[2657] > 0.0);s.store_scalar(2798, if s.b[2798] { 1.0 } else { 0.0 });s.b[2799] = (s.v[2656] > (-230.25850929940458));s.store_scalar(2799, if s.b[2799] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2798])) && s.b[2799]) {s.store_exp(2636, 2656);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2798])) && (!s.b[2799])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2800] = (s.v[533] == 0.0);s.store_scalar(2800, if s.b[2800] { 1.0 } else { 0.0 });s.b[2801] = (s.v[513] == 0.5);s.store_scalar(2801, if s.b[2801] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && s.b[2801]) {s.store_sqrt_mul_sub_lhs(2636, 510, 2634, 598);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2801])) {s.store_pow_mul_base_mixed_ai(2636, A::sub(s.ad_value(510), s.ad_value(2634)), 598, 513);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) {s.store_mul_div_scaled_product_mixed_iaii(2661, 580, A::sub(s.ad_value(510), s.ad_value(2634)), 595, 1.0, 2636, 1.0);}
        s.b[2802] = (((((-s.v[610]) / s.v[2661])) as f64).abs() < 230.25850929940458);s.store_scalar(2802, if s.b[2802] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && s.b[2802]) {s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2661), 1.0));}
        s.b[2803] = (((-s.v[610]) / s.v[2661]) < 0.0);s.store_scalar(2803, if s.b[2803] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2802])) && s.b[2803]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 610, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2802])) && (!s.b[2803])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 610, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2804] = (s.v[641] > 1000.0);s.store_scalar(2804, if s.b[2804] { 1.0 } else { 0.0 });s.b[2805] = (s.v[2635] > ((-s.v[444]) * s.v[641]));s.store_scalar(2805, if s.b[2805] { 1.0 } else { 0.0 });s.b[2806] = (s.v[545] == 4.0);s.store_scalar(2806, if s.b[2806] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && s.b[2805]) && s.b[2806]) {s.store_mul_ad_product_lhs_mixed_ai(2636, A::mul3(A::square(A::mul(s.ad_value(2635), s.ad_value(616))), s.ad_value(2635), s.ad_value(616)), 2635, 616);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && s.b[2805]) && (!s.b[2806])) {s.store_pow_abs_mul_base_indices(2636, 2635, 616, 545);}
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
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && s.b[2808]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(574)));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && (!s.b[2808])) {s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2664, 574, 577);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_71(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {s.store_add_scaled_product_mixed_aia(1923, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2636)), p.p30, 589, A::sub(s.ad_value(2663), s.ad_value(2664)), p.p30);s.store_add_scaled_inputs3_indices(2663, 833, 1.0, 550, 1.0, 2663, -1.0);s.store_primal_scaled_mul(2621, 684, 684, 4.0);s.store_primal_div(2622, 684, 685);s.store_add_scaled_product_indices(2623, 2663, 1.0, 684, 2622, 1.0);s.store_add(2624, 685, 2623);s.store_sub(2625, 685, 2623);s.store_sqrt_square_add(2626, 2625, 2621);s.store_div_scaled_product_add_scaled_denominator_indices(2664, 2663, 685, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
        s.b[2809] = (s.v[630] == 0.5);s.store_scalar(2809, if s.b[2809] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && s.b[2809]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(629)));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && (!s.b[2809])) {s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2664, 629, 630);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {s.store_add_scaled_product_mixed_aia(472, A::mul_sub_from_scalar_rhs(s.ad_value(633), 1.0, s.ad_value(2636)), p.p30, 634, A::sub(s.ad_value(2663), s.ad_value(2664)), p.p30);s.store_add(1923, 1923, 472);}
        s.b[2810] = (s.v[577] == 0.5);s.store_scalar(2810, if s.b[2810] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) && s.b[2810]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(574)));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) && (!s.b[2810])) {s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2628, 574, 577);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) {s.store_add_scaled_product_mixed_aia(1923, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2636)), p.p30, 589, A::sub(s.ad_value(833), s.ad_value(2628)), p.p30);}
        s.store_scalar(1942, 0.0);s.store_scalar(1943, 0.0);s.store_scalar(1944, 0.0);s.store_scalar(1945, 0.0);s.store_scalar(1946, 0.0);s.store_scalar(1947, 0.0);s.store_scalar(1948, 0.0);s.store_scalar(1949, 0.0);s.store_scalar(1950, 0.0);s.store_scalar(1951, 0.0);s.store_scalar(1952, 0.0);s.store_scalar(1953, 0.0);s.store_scalar(1954, 0.0);s.store_scalar(1955, 0.0);s.store_scalar(1956, 0.0);s.store_scalar(1957, 0.0);s.store_scalar(1958, 0.0);s.store_scalar(1959, 0.0);s.b[2811] = (s.v[1] != 0.0);s.store_scalar(2811, if s.b[2811] { 1.0 } else { 0.0 });
        if s.b[2811] {s.store_scalar(1988, 0.0);s.store_scalar(1992, 0.0);s.store_scalar(1986, 0.0);s.store_scalar(1987, 0.0);s.store_scalar(1993, 0.0);s.store_scalar(1969, 0.0);s.store_scalar(1970, 0.0);s.store_scalar(1971, 0.0);s.store_scalar(1972, 0.0);s.store_scalar(1973, 0.0);s.store_scalar(1974, 0.0);s.store_scalar(1975, 0.0);s.store_scalar(1976, 0.0);s.store_scalar(1977, 0.0);s.store_scalar(1960, 0.0);s.store_scalar(1961, 0.0);s.store_scalar(1962, 0.0);s.store_scalar(1963, 0.0);s.store_scalar(1964, 0.0);s.store_scalar(1965, 0.0);s.store_scalar(1966, 0.0);s.store_scalar(1967, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_72(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2811] {s.store_scalar(1968, 0.0);}
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
    pub(super) fn stamp_reactive_block_73(
        s: &mut ReactiveScratch,
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
    pub(super) fn stamp_reactive_block_74(
        s: &mut ReactiveScratch,
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
    pub(super) fn stamp_reactive_block_75(
        s: &mut ReactiveScratch,
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
    pub(super) fn stamp_reactive_block_76(
        s: &mut ReactiveScratch,
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
    pub(super) fn stamp_reactive_block_77(
        s: &mut ReactiveScratch,
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
}
