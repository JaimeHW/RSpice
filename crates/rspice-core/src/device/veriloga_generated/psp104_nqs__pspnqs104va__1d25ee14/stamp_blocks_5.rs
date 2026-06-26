#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2702] = (((((-s.v[441]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2702] = if s.b[2702] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2702]) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(441)), s.ad_value(2661)));
        }

        s.b[2703] = (((-s.v[441]) / s.v[2661]) < 0.0);
        s.v[2703] = if s.b[2703] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2702])) && s.b[2703]) {
            let assign57440_ad_e72651: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign57440_ad_e72651);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2702])) && (!s.b[2703])) {
            let assign57450_ad_e72702: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(2636, assign57450_ad_e72702);
        }

        s.b[2704] = (p.p860 > 1000.0);
        s.v[2704] = if s.b[2704] { 1.0 } else { 0.0 };

        s.b[2705] = (s.v[2635] > ((-s.v[444]) * p.p860));
        s.v[2705] = if s.b[2705] { 1.0 } else { 0.0 };

        s.b[2706] = (p.p863 == 4.0);
        s.v[2706] = if s.b[2706] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2704])) && s.b[2705]) && s.b[2706]) {
            s.store_mul_scaled_ad_lhs(2636, A::mul(A::mul(A::scale(s.ad_value(2635), s.v[448]), A::scale(s.ad_value(2635), s.v[448])), A::scale(s.ad_value(2635), s.v[448])), 2635, s.v[448]);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2704])) && s.b[2705]) && (!s.b[2706])) {
            s.store_powf_ad(2636, A::abs(A::scale(s.ad_value(2635), s.v[448])), p.p863);
        }

        s.b[2707] = (s.v[408] == 0.5);
        s.v[2707] = if s.b[2707] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2707]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[405]));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2707])) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[405])), s.v[408]);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) {
            s.store_scaled_add_ad(1918, A::scale(A::sub_from_scalar(1.0, s.ad_value(2636)), s.v[417]), A::scale(A::sub(s.ad_value(832), s.ad_value(2628)), s.v[420]), p.p30);
        }

        s.b[2708] = (s.v[647] == 0.0);
        s.v[2708] = if s.b[2708] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2708]) {
            s.store_scalar(1919, 0.0);
        }

        s.b[2709] = ((p.p841 == 0.0) && (p.p846 == 0.0));
        s.v[2709] = if s.b[2709] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) {
            s.store_sub_from_scalar(2639, s.v[394], 2633);
        }

        s.b[2711] = (p.p832 == 0.5);
        s.v[2711] = if s.b[2711] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) && s.b[2711]) {
            s.store_sqrt_scaled_input(2636, 2639, s.v[430]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) && (!s.b[2711])) {
            s.store_powf_ad(2636, A::scale(s.ad_value(2639), s.v[430]), p.p832);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) {
            s.store_scale(2643, 2636, s.v[424]);
        }

        s.b[2712] = (p.p846 == 0.0);
        s.v[2712] = if s.b[2712] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) {
            s.store_scaled_div(2646, 2643, 2639, ((s.v[409]) * (s.v[439])));
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[436]), 2646);
            s.store_square(2648, 2647);
            s.store_sqrt_div_ad(2649, A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0));
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
            s.store_sqrt_scaled_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
            s.store_add_ad(2656, A::sub(A::mul(A::scale(s.ad_value(2647), s.v[436]), s.ad_value(2650)), A::scale(s.ad_value(2649), s.v[436])), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2715] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.v[2715] = if s.b[2715] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && s.b[2715]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2715])) {
            let assign58000_ad_e73577: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(2636, assign58000_ad_e73577);
        }

        s.b[2716] = (s.v[2657] > 0.0);
        s.v[2716] = if s.b[2716] { 1.0 } else { 0.0 };

        s.b[2717] = (s.v[2656] > (-230.25850929940458));
        s.v[2717] = if s.b[2717] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2716])) && s.b[2717]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2712])) && (!s.b[2716])) && (!s.b[2717])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        s.b[2718] = (p.p852 == 0.0);
        s.v[2718] = if s.b[2718] { 1.0 } else { 0.0 };

        s.b[2719] = (p.p832 == 0.5);
        s.v[2719] = if s.b[2719] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && s.b[2719]) {
            s.store_sqrt_scaled_ad(2636, A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[430]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2719])) {
            s.store_powf_ad(2636, A::scale(A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[430]), p.p832);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) {
            s.store_scaled_div_ad_lhs(2661, A::scale(A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[427]), 2636, s.v[412]);
        }

        s.b[2720] = (((((-s.v[442]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2720] = if s.b[2720] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && s.b[2720]) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(442)), s.ad_value(2661)));
        }

        s.b[2721] = (((-s.v[442]) / s.v[2661]) < 0.0);
        s.v[2721] = if s.b[2721] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2720])) && s.b[2721]) {
            let assign58190_ad_e73917: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign58190_ad_e73917);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2720])) && (!s.b[2721])) {
            let assign58200_ad_e73968: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(2636, assign58200_ad_e73968);
        }

        s.b[2722] = (p.p861 > 1000.0);
        s.v[2722] = if s.b[2722] { 1.0 } else { 0.0 };

        s.b[2723] = (s.v[2635] > ((-s.v[444]) * p.p861));
        s.v[2723] = if s.b[2723] { 1.0 } else { 0.0 };

        s.b[2724] = (p.p864 == 4.0);
        s.v[2724] = if s.b[2724] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2722])) && s.b[2723]) && s.b[2724]) {
            s.store_mul_scaled_ad_lhs(2636, A::mul(A::mul(A::scale(s.ad_value(2635), s.v[449]), A::scale(s.ad_value(2635), s.v[449])), A::scale(s.ad_value(2635), s.v[449])), 2635, s.v[449]);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2722])) && s.b[2723]) && (!s.b[2724])) {
            s.store_powf_ad(2636, A::abs(A::scale(s.ad_value(2635), s.v[449])), p.p864);
        }

        s.b[2725] = (s.v[409] == 0.5);
        s.v[2725] = if s.b[2725] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && s.b[2725]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[406]));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2725])) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[406])), s.v[409]);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) {
            s.store_scaled_add_ad(1919, A::scale(A::sub_from_scalar(1.0, s.ad_value(2636)), s.v[418]), A::scale(A::sub(s.ad_value(832), s.ad_value(2628)), s.v[421]), p.p30);
        }

        s.b[2726] = (s.v[648] == 0.0);
        s.v[2726] = if s.b[2726] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2726]) {
            s.store_scalar(1920, 0.0);
        }

        s.b[2727] = ((p.p842 == 0.0) && (p.p847 == 0.0));
        s.v[2727] = if s.b[2727] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) {
            s.store_sub_from_scalar(2639, s.v[395], 2633);
        }

        s.b[2729] = (p.p833 == 0.5);
        s.v[2729] = if s.b[2729] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) && s.b[2729]) {
            s.store_sqrt_scaled_input(2636, 2639, s.v[431]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) && (!s.b[2729])) {
            s.store_powf_ad(2636, A::scale(s.ad_value(2639), s.v[431]), p.p833);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) {
            s.store_scale(2643, 2636, s.v[425]);
        }

        s.b[2730] = (p.p847 == 0.0);
        s.v[2730] = if s.b[2730] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) {
            s.store_scaled_div(2646, 2643, 2639, ((s.v[410]) * (s.v[440])));
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[437]), 2646);
            s.store_square(2648, 2647);
            s.store_sqrt_div_ad(2649, A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0));
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
            s.store_sqrt_scaled_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
            s.store_add_ad(2656, A::sub(A::mul(A::scale(s.ad_value(2647), s.v[437]), s.ad_value(2650)), A::scale(s.ad_value(2649), s.v[437])), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2733] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.v[2733] = if s.b[2733] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && s.b[2733]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2733])) {
            let assign58750_ad_e74843: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(2636, assign58750_ad_e74843);
        }

        s.b[2734] = (s.v[2657] > 0.0);
        s.v[2734] = if s.b[2734] { 1.0 } else { 0.0 };

        s.b[2735] = (s.v[2656] > (-230.25850929940458));
        s.v[2735] = if s.b[2735] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2734])) && s.b[2735]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2730])) && (!s.b[2734])) && (!s.b[2735])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        s.b[2736] = (p.p853 == 0.0);
        s.v[2736] = if s.b[2736] { 1.0 } else { 0.0 };

        s.b[2737] = (p.p833 == 0.5);
        s.v[2737] = if s.b[2737] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && s.b[2737]) {
            s.store_sqrt_scaled_ad(2636, A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[431]);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2737])) {
            s.store_powf_ad(2636, A::scale(A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[431]), p.p833);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) {
            s.store_scaled_div_ad_lhs(2661, A::scale(A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[428]), 2636, s.v[413]);
        }

        s.b[2738] = (((((-s.v[443]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2738] = if s.b[2738] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && s.b[2738]) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(443)), s.ad_value(2661)));
        }

        s.b[2739] = (((-s.v[443]) / s.v[2661]) < 0.0);
        s.v[2739] = if s.b[2739] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2738])) && s.b[2739]) {
            let assign58940_ad_e75183: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign58940_ad_e75183);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2738])) && (!s.b[2739])) {
            let assign58950_ad_e75234: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(2636, assign58950_ad_e75234);
        }

        s.b[2740] = (s.v[640] > 1000.0);
        s.v[2740] = if s.b[2740] { 1.0 } else { 0.0 };

        s.b[2741] = (s.v[2635] > ((-s.v[444]) * s.v[640]));
        s.v[2741] = if s.b[2741] { 1.0 } else { 0.0 };

        s.b[2742] = (p.p865 == 4.0);
        s.v[2742] = if s.b[2742] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2740])) && s.b[2741]) && s.b[2742]) {
            s.store_mul_ad(2636, A::mul(A::mul(A::mul(s.ad_value(2635), s.ad_value(450)), A::mul(s.ad_value(2635), s.ad_value(450))), A::mul(s.ad_value(2635), s.ad_value(450))), A::mul(s.ad_value(2635), s.ad_value(450)));
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2740])) && s.b[2741]) && (!s.b[2742])) {
            s.store_powf_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(450))), p.p865);
        }

        s.b[2743] = (s.v[473] == 1.0);
        s.v[2743] = if s.b[2743] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            let assign59070_ad_e75459: A = {
                if (s.v[832] < p.p870) {
                    {
                        if (((s.v[832] - p.p870) / p.p871) < (-37.0)) {
                            A::constant(p.p870)
                        } else {
                            A::offset(A::scale(A::ln(A::offset(A::exp(A::scale(A::offset(s.ad_value(832), (-p.p870)), 1.0 / (p.p871))), 1.0)), p.p871), p.p870)
                        }
                    }
                } else {
                    {
                        if (((s.v[832] - p.p870) / p.p871) > 37.0) {
                            s.ad_value(832)
                        } else {
                            A::add(s.ad_value(832), A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(p.p870, s.ad_value(832)), 1.0 / (p.p871))), 1.0)), p.p871))
                        }
                    }
                }
            };
            s.store_ad_value(2663, assign59070_ad_e75459);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            s.store_scaled_square(2621, 657, 4.0);
            s.store_div(2622, 657, 658);
            s.store_add_ad_rhs(2623, 2663, A::mul(s.ad_value(657), s.ad_value(2622)));
            s.store_add(2624, 658, 2623);
            s.store_sub(2625, 658, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_scaled_div_ad(2664, A::mul(s.ad_value(2663), s.ad_value(658)), A::add(s.ad_value(2624), s.ad_value(2626)), 2.0);
        }

        s.b[2744] = (s.v[410] == 0.5);
        s.v[2744] = if s.b[2744] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && s.b[2744]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2664), s.v[407]));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && (!s.b[2744])) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2664), s.v[407])), s.v[410]);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            s.store_scaled_add_ad(1920, A::scale(A::sub_from_scalar(1.0, s.ad_value(2636)), s.v[419]), A::scale(A::sub(s.ad_value(2663), s.ad_value(2664)), s.v[422]), p.p30);
            s.store_sub_ad_lhs(2663, A::offset(s.ad_value(832), p.p870), 2663);
            s.store_scaled_square(2621, 657, 4.0);
            s.store_div(2622, 657, 658);
            s.store_add_ad_rhs(2623, 2663, A::mul(s.ad_value(657), s.ad_value(2622)));
            s.store_add(2624, 658, 2623);
            s.store_sub(2625, 658, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_scaled_div_ad(2664, A::mul(s.ad_value(2663), s.ad_value(658)), A::add(s.ad_value(2624), s.ad_value(2626)), 2.0);
        }

        s.b[2745] = (s.v[467] == 0.5);
        s.v[2745] = if s.b[2745] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && s.b[2745]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(466)));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && (!s.b[2745])) {
            s.store_pow_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(466))), s.ad_value(467));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            s.store_scaled_add_ad(472, A::mul(s.ad_value(470), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(471), A::sub(s.ad_value(2663), s.ad_value(2664))), p.p30);
            s.store_add(1920, 1920, 472);
        }

        s.b[2746] = (s.v[410] == 0.5);
        s.v[2746] = if s.b[2746] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) && s.b[2746]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[407]));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) && (!s.b[2746])) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[407])), s.v[410]);
        }

    }

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) {
            s.store_scaled_add_ad(1920, A::scale(A::sub_from_scalar(1.0, s.ad_value(2636)), s.v[419]), A::scale(A::sub(s.ad_value(832), s.ad_value(2628)), s.v[422]), p.p30);
        }

        s.b[2747] = (s.v[636] > 0.0);
        s.v[2747] = if s.b[2747] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2747]) {
            s.store_mul_sub_ad_rhs(643, 636, A::pow(A::scale(A::add(A::add(s.ad_value(825), s.ad_value(827)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001)))), 0.5), s.ad_value(637)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(637)));
            s.store_add(641, 542, 643);
            s.store_div_from_scalar(616, 1.0, 641);
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2747])) {
            s.copy_ad(641, 542);
        }

        s.b[2748] = (s.v[638] > 0.0);
        s.v[2748] = if s.b[2748] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2748]) {
            s.store_mul_sub_ad_rhs(645, 638, A::pow(A::scale(A::add(A::add(s.ad_value(825), s.ad_value(827)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001)))), 0.5), s.ad_value(639)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(639)));
            s.store_mul_offset_rhs(610, 610, 645, 1.0);
        }

        if (s.b[2665] && (!s.b[2666])) {
            s.store_scalar(2634, 0.0);
            s.store_scalar(2631, 0.0);
        }

        s.b[2749] = (!(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)));
        s.v[2749] = if s.b[2749] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2749]) {
            s.store_scaled_square(2621, 684, 4.0);
            s.store_div(2622, 684, 685);
            s.store_add_ad_rhs(2623, 833, A::mul(s.ad_value(684), s.ad_value(2622)));
            s.store_add(2624, 685, 2623);
            s.store_sub(2625, 685, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_scaled_div_ad(2628, A::mul(s.ad_value(833), s.ad_value(685)), A::add(s.ad_value(2624), s.ad_value(2626)), 2.0);
        }

        s.b[2750] = (s.v[833] < s.v[681]);
        s.v[2750] = if s.b[2750] { 1.0 } else { 0.0 };

        s.b[2751] = (((((-0.5) * (s.v[833] * s.v[371]))) as f64).abs() < 230.25850929940458);
        s.v[2751] = if s.b[2751] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) && s.b[2751]) {
            s.store_exp_scaled_input(2629, 833, (s.v[371] * (-0.5)));
        }

        s.b[2752] = (((-0.5) * (s.v[833] * s.v[371])) < 0.0);
        s.v[2752] = if s.b[2752] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) && (!s.b[2751])) && s.b[2752]) {
            let assign59600_ad_e76293: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(2629, assign59600_ad_e76293);
        }

        if (((((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) && (!s.b[2751])) && (!s.b[2752])) {
            s.store_scaled_offset_ad(2629, A::mul(A::offset(A::scale(s.ad_value(833), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(833), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(833), (s.v[371] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2750]) {
            s.store_div_from_scalar(2630, 1.0, 2629);
            s.store_square(2627, 2630);
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && (!s.b[2750])) {
            s.store_mul_offset_ad_lhs(2627, A::scale(A::sub(s.ad_value(833), s.ad_value(681)), s.v[371]), 1.0, 682);
            s.store_sqrt(2630, 2627);
            s.store_div_from_scalar(2629, 1.0, 2630);
        }

        if ((s.b[2665] && (!s.b[2666])) && s.b[2749]) {
            s.store_offset(2627, 2627, (-1.0));
        }

        s.b[2753] = (s.v[833] > 0.0);
        s.v[2753] = if s.b[2753] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && s.b[2753]) {
            s.store_scaled_ln_ad(2631, A::add(A::offset(s.ad_value(2629), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2629), 1.0), A::offset(s.ad_value(2629), 3.0)))), (s.v[370] * 2.0));
        }

        if (((s.b[2665] && (!s.b[2666])) && s.b[2749]) && (!s.b[2753])) {
            s.store_sub_ad_lhs(2631, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2630), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2630), 1.0), A::offset(A::scale(s.ad_value(2630), 3.0), 1.0))))), (s.v[370] * 2.0)), 833);
        }

        if ((s.b[2665] && (!s.b[2666])) && s.b[2749]) {
            s.store_sub(2632, 683, 2631);
            s.store_scaled_sub_ad(2633, A::add(s.ad_value(833), s.ad_value(2632)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(833), s.ad_value(2632)), A::sub(s.ad_value(833), s.ad_value(2632))), ((4.0 * s.v[370]) * s.v[370]))), 0.5);
            s.store_scaled_sub_ad(2634, A::add(s.ad_value(833), s.ad_value(686)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(833), s.ad_value(686)), A::sub(s.ad_value(833), s.ad_value(686))), ((4.0 * s.v[368]) * s.v[368]))), 0.5);
            s.store_scaled_sub_ad_rhs(2635, 833, A::sqrt(A::offset(A::mul(s.ad_value(833), s.ad_value(833)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[2754] = (s.v[673] == 0.0);
        s.v[2754] = if s.b[2754] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2754]) {
            s.store_scalar(1921, 0.0);
        }

        s.b[2755] = ((s.v[522] == 0.0) && (s.v[525] == 0.0));
        s.v[2755] = if s.b[2755] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) {
            s.store_sub(2639, 569, 2633);
        }

        s.b[2757] = (s.v[511] == 0.5);
        s.v[2757] = if s.b[2757] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) && s.b[2757]) {
            s.store_sqrt_mul(2636, 2639, 596);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) && (!s.b[2757])) {
            s.store_pow_ad(2636, A::mul(s.ad_value(2639), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2755])) {
            s.store_mul(2643, 590, 2636);
        }

        s.b[2758] = (s.v[525] == 0.0);
        s.v[2758] = if s.b[2758] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) {
            s.store_mul_div_ad_rhs(2646, 605, A::mul(s.ad_value(2643), s.ad_value(575)), s.ad_value(2639));
            s.store_scaled_div(2647, 602, 2646, 0.666666666666667);
            s.store_square(2648, 2647);
            s.store_sqrt_div_ad(2649, A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0));
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
            s.store_sqrt_scaled_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
            s.store_add_ad(2656, A::sub(A::mul(A::mul(s.ad_value(602), s.ad_value(2647)), s.ad_value(2650)), A::mul(s.ad_value(602), s.ad_value(2649))), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2761] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.v[2761] = if s.b[2761] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && s.b[2761]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2761])) {
            let assign60150_ad_e77243: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(2636, assign60150_ad_e77243);
        }

        s.b[2762] = (s.v[2657] > 0.0);
        s.v[2762] = if s.b[2762] { 1.0 } else { 0.0 };

        s.b[2763] = (s.v[2656] > (-230.25850929940458));
        s.v[2763] = if s.b[2763] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2762])) && s.b[2763]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2758])) && (!s.b[2762])) && (!s.b[2763])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        s.b[2764] = (s.v[531] == 0.0);
        s.v[2764] = if s.b[2764] { 1.0 } else { 0.0 };

        s.b[2765] = (s.v[511] == 0.5);
        s.v[2765] = if s.b[2765] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && s.b[2765]) {
            s.store_sqrt_mul_ad(2636, A::sub(s.ad_value(508), s.ad_value(2634)), s.ad_value(596));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && (!s.b[2765])) {
            s.store_pow_ad(2636, A::mul(A::sub(s.ad_value(508), s.ad_value(2634)), s.ad_value(596)), s.ad_value(511));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) {
            s.store_mul_div_ad_rhs(2661, 578, A::mul(A::sub(s.ad_value(508), s.ad_value(2634)), s.ad_value(593)), s.ad_value(2636));
        }

        s.b[2766] = (((((-s.v[608]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2766] = if s.b[2766] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && s.b[2766]) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(608)), s.ad_value(2661)));
        }

        s.b[2767] = (((-s.v[608]) / s.v[2661]) < 0.0);
        s.v[2767] = if s.b[2767] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && (!s.b[2766])) && s.b[2767]) {
            let assign60340_ad_e77583: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(608)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(608)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(608)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign60340_ad_e77583);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2764])) && (!s.b[2766])) && (!s.b[2767])) {
            let assign60350_ad_e77634: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(608)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(608)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(608)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(2636, assign60350_ad_e77634);
        }

        s.b[2768] = (s.v[540] > 1000.0);
        s.v[2768] = if s.b[2768] { 1.0 } else { 0.0 };

        s.b[2769] = (s.v[2635] > ((-s.v[444]) * s.v[540]));
        s.v[2769] = if s.b[2769] { 1.0 } else { 0.0 };

        s.b[2770] = (s.v[543] == 4.0);
        s.v[2770] = if s.b[2770] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2768])) && s.b[2769]) && s.b[2770]) {
            s.store_mul_ad(2636, A::mul(A::mul(A::mul(s.ad_value(2635), s.ad_value(614)), A::mul(s.ad_value(2635), s.ad_value(614))), A::mul(s.ad_value(2635), s.ad_value(614))), A::mul(s.ad_value(2635), s.ad_value(614)));
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2768])) && s.b[2769]) && (!s.b[2770])) {
            s.store_pow_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(614))), s.ad_value(543));
        }

        s.b[2771] = (s.v[575] == 0.5);
        s.v[2771] = if s.b[2771] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && s.b[2771]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(572)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) && (!s.b[2771])) {
            s.store_pow_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(572))), s.ad_value(575));
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) {
            s.store_scaled_add_ad(1921, A::mul(s.ad_value(584), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(587), A::sub(s.ad_value(833), s.ad_value(2628))), p.p30);
        }

        s.b[2772] = (s.v[674] == 0.0);
        s.v[2772] = if s.b[2772] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2772]) {
            s.store_scalar(1922, 0.0);
        }

        s.b[2773] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[2773] = if s.b[2773] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) {
            s.store_sub(2639, 570, 2633);
        }

        s.b[2775] = (s.v[512] == 0.5);
        s.v[2775] = if s.b[2775] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) && s.b[2775]) {
            s.store_sqrt_mul(2636, 2639, 597);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) && (!s.b[2775])) {
            s.store_pow_ad(2636, A::mul(s.ad_value(2639), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2773])) {
            s.store_mul(2643, 591, 2636);
        }

        s.b[2776] = (s.v[526] == 0.0);
        s.v[2776] = if s.b[2776] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) {
            s.store_mul_div_ad_rhs(2646, 606, A::mul(s.ad_value(2643), s.ad_value(576)), s.ad_value(2639));
            s.store_scaled_div(2647, 603, 2646, 0.666666666666667);
            s.store_square(2648, 2647);
            s.store_sqrt_div_ad(2649, A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0));
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
            s.store_sqrt_scaled_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
            s.store_add_ad(2656, A::sub(A::mul(A::mul(s.ad_value(603), s.ad_value(2647)), s.ad_value(2650)), A::mul(s.ad_value(603), s.ad_value(2649))), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2779] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.v[2779] = if s.b[2779] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && s.b[2779]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2779])) {
            let assign60900_ad_e78509: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(2636, assign60900_ad_e78509);
        }

        s.b[2780] = (s.v[2657] > 0.0);
        s.v[2780] = if s.b[2780] { 1.0 } else { 0.0 };

        s.b[2781] = (s.v[2656] > (-230.25850929940458));
        s.v[2781] = if s.b[2781] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2780])) && s.b[2781]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2776])) && (!s.b[2780])) && (!s.b[2781])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        s.b[2782] = (s.v[532] == 0.0);
        s.v[2782] = if s.b[2782] { 1.0 } else { 0.0 };

        s.b[2783] = (s.v[512] == 0.5);
        s.v[2783] = if s.b[2783] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && s.b[2783]) {
            s.store_sqrt_mul_ad(2636, A::sub(s.ad_value(509), s.ad_value(2634)), s.ad_value(597));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2783])) {
            s.store_pow_ad(2636, A::mul(A::sub(s.ad_value(509), s.ad_value(2634)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) {
            s.store_mul_div_ad_rhs(2661, 579, A::mul(A::sub(s.ad_value(509), s.ad_value(2634)), s.ad_value(594)), s.ad_value(2636));
        }

        s.b[2784] = (((((-s.v[609]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2784] = if s.b[2784] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && s.b[2784]) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(609)), s.ad_value(2661)));
        }

        s.b[2785] = (((-s.v[609]) / s.v[2661]) < 0.0);
        s.v[2785] = if s.b[2785] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2784])) && s.b[2785]) {
            let assign61090_ad_e78849: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign61090_ad_e78849);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2782])) && (!s.b[2784])) && (!s.b[2785])) {
            let assign61100_ad_e78900: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(2636, assign61100_ad_e78900);
        }

        s.b[2786] = (s.v[541] > 1000.0);
        s.v[2786] = if s.b[2786] { 1.0 } else { 0.0 };

        s.b[2787] = (s.v[2635] > ((-s.v[444]) * s.v[541]));
        s.v[2787] = if s.b[2787] { 1.0 } else { 0.0 };

        s.b[2788] = (s.v[544] == 4.0);
        s.v[2788] = if s.b[2788] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && s.b[2787]) && s.b[2788]) {
            s.store_mul_ad(2636, A::mul(A::mul(A::mul(s.ad_value(2635), s.ad_value(615)), A::mul(s.ad_value(2635), s.ad_value(615))), A::mul(s.ad_value(2635), s.ad_value(615))), A::mul(s.ad_value(2635), s.ad_value(615)));
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2786])) && s.b[2787]) && (!s.b[2788])) {
            s.store_pow_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(615))), s.ad_value(544));
        }

        s.b[2789] = (s.v[576] == 0.5);
        s.v[2789] = if s.b[2789] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && s.b[2789]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(573)));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) && (!s.b[2789])) {
            s.store_pow_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(573))), s.ad_value(576));
        }

        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2772])) {
            s.store_scaled_add_ad(1922, A::mul(s.ad_value(585), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(588), A::sub(s.ad_value(833), s.ad_value(2628))), p.p30);
        }

        s.b[2790] = (s.v[675] == 0.0);
        s.v[2790] = if s.b[2790] { 1.0 } else { 0.0 };

        if ((s.b[2665] && (!s.b[2666])) && s.b[2790]) {
            s.store_scalar(1923, 0.0);
        }

        s.b[2791] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[2791] = if s.b[2791] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {
            s.store_sub(2639, 571, 2633);
        }

        s.b[2793] = (s.v[513] == 0.5);
        s.v[2793] = if s.b[2793] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && s.b[2793]) {
            s.store_sqrt_mul(2636, 2639, 598);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) && (!s.b[2793])) {
            s.store_pow_ad(2636, A::mul(s.ad_value(2639), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2791])) {
            s.store_mul(2643, 592, 2636);
        }

        s.b[2794] = (s.v[527] == 0.0);
        s.v[2794] = if s.b[2794] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) {
            s.store_mul_div_ad_rhs(2646, 607, A::mul(s.ad_value(2643), s.ad_value(577)), s.ad_value(2639));
            s.store_scaled_div(2647, 604, 2646, 0.666666666666667);
            s.store_square(2648, 2647);
            s.store_sqrt_div_ad(2649, A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0));
            s.store_sqrt(2650, 2649);
            s.store_mul(2651, 2649, 2650);
            s.store_sqrt_scaled_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
            s.store_add_ad(2656, A::sub(A::mul(A::mul(s.ad_value(604), s.ad_value(2647)), s.ad_value(2650)), A::mul(s.ad_value(604), s.ad_value(2649))), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
            s.store_mul_offset_lhs(2657, 2655, (-1.0), 2654);
            s.store_square(2618, 2657);
        }

        s.b[2797] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));
        s.v[2797] = if s.b[2797] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && s.b[2797]) {
            s.store_exp_sub(2636, 2656, 2618);
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2797])) {
            let assign61650_ad_e79775: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad_value(2636, assign61650_ad_e79775);
        }

        s.b[2798] = (s.v[2657] > 0.0);
        s.v[2798] = if s.b[2798] { 1.0 } else { 0.0 };

        s.b[2799] = (s.v[2656] > (-230.25850929940458));
        s.v[2799] = if s.b[2799] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2798])) && s.b[2799]) {
            s.store_exp(2636, 2656);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2794])) && (!s.b[2798])) && (!s.b[2799])) {
            s.store_div_from_scalar_offset_ad(2636, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        s.b[2800] = (s.v[533] == 0.0);
        s.v[2800] = if s.b[2800] { 1.0 } else { 0.0 };

        s.b[2801] = (s.v[513] == 0.5);
        s.v[2801] = if s.b[2801] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && s.b[2801]) {
            s.store_sqrt_mul_ad(2636, A::sub(s.ad_value(510), s.ad_value(2634)), s.ad_value(598));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2801])) {
            s.store_pow_ad(2636, A::mul(A::sub(s.ad_value(510), s.ad_value(2634)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) {
            s.store_mul_div_ad_rhs(2661, 580, A::mul(A::sub(s.ad_value(510), s.ad_value(2634)), s.ad_value(595)), s.ad_value(2636));
        }

        s.b[2802] = (((((-s.v[610]) / s.v[2661])) as f64).abs() < 230.25850929940458);
        s.v[2802] = if s.b[2802] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && s.b[2802]) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(610)), s.ad_value(2661)));
        }

        s.b[2803] = (((-s.v[610]) / s.v[2661]) < 0.0);
        s.v[2803] = if s.b[2803] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2802])) && s.b[2803]) {
            let assign61840_ad_e80115: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign61840_ad_e80115);
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2800])) && (!s.b[2802])) && (!s.b[2803])) {
            let assign61850_ad_e80166: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad_value(2636, assign61850_ad_e80166);
        }

        s.b[2804] = (s.v[641] > 1000.0);
        s.v[2804] = if s.b[2804] { 1.0 } else { 0.0 };

        s.b[2805] = (s.v[2635] > ((-s.v[444]) * s.v[641]));
        s.v[2805] = if s.b[2805] { 1.0 } else { 0.0 };

        s.b[2806] = (s.v[545] == 4.0);
        s.v[2806] = if s.b[2806] { 1.0 } else { 0.0 };

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && s.b[2805]) && s.b[2806]) {
            s.store_mul_ad(2636, A::mul(A::mul(A::mul(s.ad_value(2635), s.ad_value(616)), A::mul(s.ad_value(2635), s.ad_value(616))), A::mul(s.ad_value(2635), s.ad_value(616))), A::mul(s.ad_value(2635), s.ad_value(616)));
        }

        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2804])) && s.b[2805]) && (!s.b[2806])) {
            s.store_pow_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(616))), s.ad_value(545));
        }

        s.b[2807] = (s.v[635] == 1.0);
        s.v[2807] = if s.b[2807] { 1.0 } else { 0.0 };

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            let assign61970_ad_e80391: A = {
                if (s.v[833] < s.v[550]) {
                    {
                        if (((s.v[833] - s.v[550]) / s.v[551]) < (-37.0)) {
                            s.ad_value(550)
                        } else {
                            A::add(s.ad_value(550), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(833), s.ad_value(550)), s.ad_value(551))), 1.0)), s.ad_value(551)))
                        }
                    }
                } else {
                    {
                        if (((s.v[833] - s.v[550]) / s.v[551]) > 37.0) {
                            s.ad_value(833)
                        } else {
                            A::add(s.ad_value(833), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(550), s.ad_value(833)), s.ad_value(551))), 1.0)), s.ad_value(551)))
                        }
                    }
                }
            };
            s.store_ad_value(2663, assign61970_ad_e80391);
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            s.store_scaled_square(2621, 684, 4.0);
            s.store_div(2622, 684, 685);
            s.store_add_ad_rhs(2623, 2663, A::mul(s.ad_value(684), s.ad_value(2622)));
            s.store_add(2624, 685, 2623);
            s.store_sub(2625, 685, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_scaled_div_ad(2664, A::mul(s.ad_value(2663), s.ad_value(685)), A::add(s.ad_value(2624), s.ad_value(2626)), 2.0);
        }

        s.b[2808] = (s.v[577] == 0.5);
        s.v[2808] = if s.b[2808] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && s.b[2808]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(574)));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && (!s.b[2808])) {
            s.store_pow_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(574))), s.ad_value(577));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            s.store_scaled_add_ad(1923, A::mul(s.ad_value(586), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(589), A::sub(s.ad_value(2663), s.ad_value(2664))), p.p30);
            s.store_sub_ad_lhs(2663, A::add(s.ad_value(833), s.ad_value(550)), 2663);
            s.store_scaled_square(2621, 684, 4.0);
            s.store_div(2622, 684, 685);
            s.store_add_ad_rhs(2623, 2663, A::mul(s.ad_value(684), s.ad_value(2622)));
            s.store_add(2624, 685, 2623);
            s.store_sub(2625, 685, 2623);
            s.store_sqrt_square_add(2626, 2625, 2621);
            s.store_scaled_div_ad(2664, A::mul(s.ad_value(2663), s.ad_value(685)), A::add(s.ad_value(2624), s.ad_value(2626)), 2.0);
        }

        s.b[2809] = (s.v[630] == 0.5);
        s.v[2809] = if s.b[2809] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && s.b[2809]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(629)));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) && (!s.b[2809])) {
            s.store_pow_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(629))), s.ad_value(630));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && s.b[2807]) {
            s.store_scaled_add_ad(472, A::mul(s.ad_value(633), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(634), A::sub(s.ad_value(2663), s.ad_value(2664))), p.p30);
            s.store_add(1923, 1923, 472);
        }

        s.b[2810] = (s.v[577] == 0.5);
        s.v[2810] = if s.b[2810] { 1.0 } else { 0.0 };

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) && s.b[2810]) {
            s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2628), s.ad_value(574)));
        }

        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) && (!s.b[2810])) {
            s.store_pow_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(574))), s.ad_value(577));
        }

        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2790])) && (!s.b[2807])) {
            s.store_scaled_add_ad(1923, A::mul(s.ad_value(586), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(589), A::sub(s.ad_value(833), s.ad_value(2628))), p.p30);
        }

        s.v[1942] = 0.0;

        s.v[1943] = 0.0;

        s.v[1944] = 0.0;

        s.v[1945] = 0.0;

        s.v[1946] = 0.0;

        s.v[1947] = 0.0;

        s.v[1948] = 0.0;

        s.v[1949] = 0.0;

        s.v[1950] = 0.0;

        s.v[1951] = 0.0;

        s.v[1952] = 0.0;

        s.v[1953] = 0.0;

        s.v[1954] = 0.0;

        s.v[1955] = 0.0;

        s.v[1956] = 0.0;

        s.v[1957] = 0.0;

        s.v[1958] = 0.0;

        s.v[1959] = 0.0;

        s.b[2811] = (s.v[1] != 0.0);
        s.v[2811] = if s.b[2811] { 1.0 } else { 0.0 };

        if s.b[2811] {
            s.store_scalar(1988, 0.0);
            s.store_scalar(1992, 0.0);
            s.store_scalar(1986, 0.0);
            s.store_scalar(1987, 0.0);
            s.store_scalar(1993, 0.0);
            s.store_scalar(1969, 0.0);
            s.store_scalar(1970, 0.0);
            s.store_scalar(1971, 0.0);
            s.store_scalar(1972, 0.0);
            s.store_scalar(1973, 0.0);
            s.store_scalar(1974, 0.0);
            s.store_scalar(1975, 0.0);
            s.store_scalar(1976, 0.0);
            s.store_scalar(1977, 0.0);
            s.store_scalar(1960, 0.0);
            s.store_scalar(1961, 0.0);
            s.store_scalar(1962, 0.0);
            s.store_scalar(1963, 0.0);
            s.store_scalar(1964, 0.0);
            s.store_scalar(1965, 0.0);
            s.store_scalar(1966, 0.0);
            s.store_scalar(1967, 0.0);
            s.store_scalar(1968, 0.0);
        }

        s.b[2812] = (s.v[1890] > 0.0);
        s.v[2812] = if s.b[2812] { 1.0 } else { 0.0 };

        s.b[2813] = (s.v[1] == 1.0);
        s.v[2813] = if s.b[2813] { 1.0 } else { 0.0 };

        if ((s.b[2811] && s.b[2812]) && s.b[2813]) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.5, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2814] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.v[2814] = if s.b[2814] { 1.0 } else { 0.0 };

        if (((s.b[2811] && s.b[2812]) && s.b[2813]) && s.b[2814]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.b[2815] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.v[2815] = if s.b[2815] { 1.0 } else { 0.0 };

        if ((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && s.b[2815]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2816] = ((-s.v[1960]) < 0.0);
        s.v[2816] = if s.b[2816] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && (!s.b[2815])) && s.b[2816]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && (!s.b[2815])) && (!s.b[2816])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2817] = (s.v[1960] > s.v[1933]);
        s.v[2817] = if s.b[2817] { 1.0 } else { 0.0 };

        if ((((s.b[2811] && s.b[2812]) && s.b[2813]) && (!s.b[2814])) && s.b[2817]) {
            s.store_neg(1996, 1996);
        }

        if ((s.b[2811] && s.b[2812]) && s.b[2813]) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
        }

        s.b[2818] = (s.v[1] == 2.0);
        s.v[2818] = if s.b[2818] { 1.0 } else { 0.0 };

        if (((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.3333333333333333, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2819] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.v[2819] = if s.b[2819] { 1.0 } else { 0.0 };

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && s.b[2819]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.b[2820] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.v[2820] = if s.b[2820] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && s.b[2820]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2821] = ((-s.v[1960]) < 0.0);
        s.v[2821] = if s.b[2821] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && s.b[2821]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && (!s.b[2820])) && (!s.b[2821])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2822] = (s.v[1960] > s.v[1933]);
        s.v[2822] = if s.b[2822] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2819])) && s.b[2822]) {
            s.store_neg(1996, 1996);
        }

        if (((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
            s.store_add_ad_rhs(1961, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.6666666666666666, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2823] = (((s.v[1961]) as f64).abs() <= s.v[1933]);
        s.v[2823] = if s.b[2823] { 1.0 } else { 0.0 };

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && s.b[2823]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1961), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1961), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1961), 0.16666666666666666)))));
        }

    }

    pub(super) fn stamp_reactive_block_23(
        s: &mut ReactiveScratch,
    ) {
        s.b[2824] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);
        s.v[2824] = if s.b[2824] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && s.b[2824]) {
            s.store_exp_neg_input(2027, 1961);
        }

        s.b[2825] = ((-s.v[1961]) < 0.0);
        s.v[2825] = if s.b[2825] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && (!s.b[2824])) && s.b[2825]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && (!s.b[2824])) && (!s.b[2825])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));
        }

        s.b[2826] = (s.v[1961] > s.v[1933]);
        s.v[2826] = if s.b[2826] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && (!s.b[2823])) && s.b[2826]) {
            s.store_neg(1996, 1996);
        }

        if (((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) {
            s.store_sub_ad_lhs(1943, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1961))), 1996);
        }

        s.b[2827] = (s.v[831] < 0.0);
        s.v[2827] = if s.b[2827] { 1.0 } else { 0.0 };

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && s.b[2818]) && s.b[2827]) {
            s.copy_ad(2027, 1942);
            s.copy_ad(1942, 1943);
            s.copy_ad(1943, 2027);
        }

        s.b[2828] = (s.v[1] == 3.0);
        s.v[2828] = if s.b[2828] { 1.0 } else { 0.0 };

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.25, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2829] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.v[2829] = if s.b[2829] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2829]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.b[2830] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.v[2830] = if s.b[2830] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && s.b[2830]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2831] = ((-s.v[1960]) < 0.0);
        s.v[2831] = if s.b[2831] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && (!s.b[2830])) && s.b[2831]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && (!s.b[2830])) && (!s.b[2831])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2832] = (s.v[1960] > s.v[1933]);
        s.v[2832] = if s.b[2832] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2829])) && s.b[2832]) {
            s.store_neg(1996, 1996);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
            s.store_add_ad_rhs(1961, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.5, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2833] = (((s.v[1961]) as f64).abs() <= s.v[1933]);
        s.v[2833] = if s.b[2833] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2833]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1961), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1961), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1961), 0.16666666666666666)))));
        }

        s.b[2834] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);
        s.v[2834] = if s.b[2834] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && s.b[2834]) {
            s.store_exp_neg_input(2027, 1961);
        }

        s.b[2835] = ((-s.v[1961]) < 0.0);
        s.v[2835] = if s.b[2835] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && (!s.b[2834])) && s.b[2835]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && (!s.b[2834])) && (!s.b[2835])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));
        }

        s.b[2836] = (s.v[1961] > s.v[1933]);
        s.v[2836] = if s.b[2836] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2833])) && s.b[2836]) {
            s.store_neg(1996, 1996);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {
            s.store_sub_ad_lhs(1943, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1961))), 1996);
            s.store_add_ad_rhs(1962, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.75, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2837] = (((s.v[1962]) as f64).abs() <= s.v[1933]);
        s.v[2837] = if s.b[2837] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2837]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1962), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1962), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1962), 0.16666666666666666)))));
        }

        s.b[2838] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);
        s.v[2838] = if s.b[2838] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && s.b[2838]) {
            s.store_exp_neg_input(2027, 1962);
        }

        s.b[2839] = ((-s.v[1962]) < 0.0);
        s.v[2839] = if s.b[2839] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && (!s.b[2838])) && s.b[2839]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && (!s.b[2838])) && (!s.b[2839])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));
        }

        s.b[2840] = (s.v[1962] > s.v[1933]);
        s.v[2840] = if s.b[2840] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && (!s.b[2837])) && s.b[2840]) {
            s.store_neg(1996, 1996);
        }

        if ((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) {
            s.store_sub_ad_lhs(1944, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1962))), 1996);
        }

        s.b[2841] = (s.v[831] < 0.0);
        s.v[2841] = if s.b[2841] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && s.b[2828]) && s.b[2841]) {
            s.copy_ad(2027, 1942);
            s.copy_ad(1942, 1944);
            s.copy_ad(1944, 2027);
        }

        s.b[2842] = (s.v[1] == 5.0);
        s.v[2842] = if s.b[2842] { 1.0 } else { 0.0 };

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.16666666666666666, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2843] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.v[2843] = if s.b[2843] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2843]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.b[2844] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.v[2844] = if s.b[2844] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && s.b[2844]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2845] = ((-s.v[1960]) < 0.0);
        s.v[2845] = if s.b[2845] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && (!s.b[2844])) && s.b[2845]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && (!s.b[2844])) && (!s.b[2845])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2846] = (s.v[1960] > s.v[1933]);
        s.v[2846] = if s.b[2846] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2843])) && s.b[2846]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
            s.store_add_ad_rhs(1961, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.3333333333333333, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2847] = (((s.v[1961]) as f64).abs() <= s.v[1933]);
        s.v[2847] = if s.b[2847] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2847]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1961), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1961), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1961), 0.16666666666666666)))));
        }

        s.b[2848] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);
        s.v[2848] = if s.b[2848] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && s.b[2848]) {
            s.store_exp_neg_input(2027, 1961);
        }

        s.b[2849] = ((-s.v[1961]) < 0.0);
        s.v[2849] = if s.b[2849] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && (!s.b[2848])) && s.b[2849]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && (!s.b[2848])) && (!s.b[2849])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));
        }

        s.b[2850] = (s.v[1961] > s.v[1933]);
        s.v[2850] = if s.b[2850] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2847])) && s.b[2850]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_sub_ad_lhs(1943, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1961))), 1996);
            s.store_add_ad_rhs(1962, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.5, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2851] = (((s.v[1962]) as f64).abs() <= s.v[1933]);
        s.v[2851] = if s.b[2851] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2851]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1962), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1962), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1962), 0.16666666666666666)))));
        }

        s.b[2852] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);
        s.v[2852] = if s.b[2852] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && s.b[2852]) {
            s.store_exp_neg_input(2027, 1962);
        }

        s.b[2853] = ((-s.v[1962]) < 0.0);
        s.v[2853] = if s.b[2853] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && (!s.b[2852])) && s.b[2853]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && (!s.b[2852])) && (!s.b[2853])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));
        }

        s.b[2854] = (s.v[1962] > s.v[1933]);
        s.v[2854] = if s.b[2854] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2851])) && s.b[2854]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_sub_ad_lhs(1944, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1962))), 1996);
            s.store_add_ad_rhs(1963, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.6666666666666666, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2855] = (((s.v[1963]) as f64).abs() <= s.v[1933]);
        s.v[2855] = if s.b[2855] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2855]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1963), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1963), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1963), 0.16666666666666666)))));
        }

        s.b[2856] = ((((-s.v[1963])) as f64).abs() < 230.25850929940458);
        s.v[2856] = if s.b[2856] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && s.b[2856]) {
            s.store_exp_neg_input(2027, 1963);
        }

        s.b[2857] = ((-s.v[1963]) < 0.0);
        s.v[2857] = if s.b[2857] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && (!s.b[2856])) && s.b[2857]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && (!s.b[2856])) && (!s.b[2857])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0)));
        }

        s.b[2858] = (s.v[1963] > s.v[1933]);
        s.v[2858] = if s.b[2858] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2855])) && s.b[2858]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_sub_ad_lhs(1945, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1963))), 1996);
            s.store_add_ad_rhs(1964, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.8333333333333333, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2859] = (((s.v[1964]) as f64).abs() <= s.v[1933]);
        s.v[2859] = if s.b[2859] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2859]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1964), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1964), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1964), 0.16666666666666666)))));
        }

        s.b[2860] = ((((-s.v[1964])) as f64).abs() < 230.25850929940458);
        s.v[2860] = if s.b[2860] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && s.b[2860]) {
            s.store_exp_neg_input(2027, 1964);
        }

        s.b[2861] = ((-s.v[1964]) < 0.0);
        s.v[2861] = if s.b[2861] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && (!s.b[2860])) && s.b[2861]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && (!s.b[2860])) && (!s.b[2861])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0)));
        }

        s.b[2862] = (s.v[1964] > s.v[1933]);
        s.v[2862] = if s.b[2862] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && (!s.b[2859])) && s.b[2862]) {
            s.store_neg(1996, 1996);
        }

        if (((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) {
            s.store_sub_ad_lhs(1946, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1964))), 1996);
        }

        s.b[2863] = (s.v[831] < 0.0);
        s.v[2863] = if s.b[2863] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && s.b[2842]) && s.b[2863]) {
            s.copy_ad(2027, 1942);
            s.copy_ad(1942, 1946);
            s.copy_ad(1946, 2027);
            s.copy_ad(2027, 1943);
            s.copy_ad(1943, 1945);
            s.copy_ad(1945, 2027);
        }

        s.b[2864] = (s.v[1] == 9.0);
        s.v[2864] = if s.b[2864] { 1.0 } else { 0.0 };

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.1, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2865] = (((s.v[1960]) as f64).abs() <= s.v[1933]);
        s.v[2865] = if s.b[2865] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2865]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.b[2866] = ((((-s.v[1960])) as f64).abs() < 230.25850929940458);
        s.v[2866] = if s.b[2866] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && s.b[2866]) {
            s.store_exp_neg_input(2027, 1960);
        }

        s.b[2867] = ((-s.v[1960]) < 0.0);
        s.v[2867] = if s.b[2867] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && (!s.b[2866])) && s.b[2867]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && (!s.b[2866])) && (!s.b[2867])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0)));
        }

        s.b[2868] = (s.v[1960] > s.v[1933]);
        s.v[2868] = if s.b[2868] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2865])) && s.b[2868]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
            s.store_add_ad_rhs(1961, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.2, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2869] = (((s.v[1961]) as f64).abs() <= s.v[1933]);
        s.v[2869] = if s.b[2869] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2869]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1961), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1961), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1961), 0.16666666666666666)))));
        }

        s.b[2870] = ((((-s.v[1961])) as f64).abs() < 230.25850929940458);
        s.v[2870] = if s.b[2870] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_24(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && s.b[2870]) {
            s.store_exp_neg_input(2027, 1961);
        }

        s.b[2871] = ((-s.v[1961]) < 0.0);
        s.v[2871] = if s.b[2871] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && (!s.b[2870])) && s.b[2871]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && (!s.b[2870])) && (!s.b[2871])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0)));
        }

        s.b[2872] = (s.v[1961] > s.v[1933]);
        s.v[2872] = if s.b[2872] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2869])) && s.b[2872]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_sub_ad_lhs(1943, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1961))), 1996);
            s.store_add_ad_rhs(1962, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.3, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2873] = (((s.v[1962]) as f64).abs() <= s.v[1933]);
        s.v[2873] = if s.b[2873] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2873]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1962), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1962), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1962), 0.16666666666666666)))));
        }

        s.b[2874] = ((((-s.v[1962])) as f64).abs() < 230.25850929940458);
        s.v[2874] = if s.b[2874] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && s.b[2874]) {
            s.store_exp_neg_input(2027, 1962);
        }

        s.b[2875] = ((-s.v[1962]) < 0.0);
        s.v[2875] = if s.b[2875] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && (!s.b[2874])) && s.b[2875]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && (!s.b[2874])) && (!s.b[2875])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0)));
        }

        s.b[2876] = (s.v[1962] > s.v[1933]);
        s.v[2876] = if s.b[2876] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2873])) && s.b[2876]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_sub_ad_lhs(1944, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1962))), 1996);
            s.store_add_ad_rhs(1963, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.4, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2877] = (((s.v[1963]) as f64).abs() <= s.v[1933]);
        s.v[2877] = if s.b[2877] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2877]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1963), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1963), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1963), 0.16666666666666666)))));
        }

        s.b[2878] = ((((-s.v[1963])) as f64).abs() < 230.25850929940458);
        s.v[2878] = if s.b[2878] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && s.b[2878]) {
            s.store_exp_neg_input(2027, 1963);
        }

        s.b[2879] = ((-s.v[1963]) < 0.0);
        s.v[2879] = if s.b[2879] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && (!s.b[2878])) && s.b[2879]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && (!s.b[2878])) && (!s.b[2879])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0)));
        }

        s.b[2880] = (s.v[1963] > s.v[1933]);
        s.v[2880] = if s.b[2880] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2877])) && s.b[2880]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_sub_ad_lhs(1945, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1963))), 1996);
            s.store_add_ad_rhs(1964, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.5, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2881] = (((s.v[1964]) as f64).abs() <= s.v[1933]);
        s.v[2881] = if s.b[2881] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2881]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1964), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1964), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1964), 0.16666666666666666)))));
        }

        s.b[2882] = ((((-s.v[1964])) as f64).abs() < 230.25850929940458);
        s.v[2882] = if s.b[2882] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && s.b[2882]) {
            s.store_exp_neg_input(2027, 1964);
        }

        s.b[2883] = ((-s.v[1964]) < 0.0);
        s.v[2883] = if s.b[2883] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && (!s.b[2882])) && s.b[2883]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && (!s.b[2882])) && (!s.b[2883])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0)));
        }

        s.b[2884] = (s.v[1964] > s.v[1933]);
        s.v[2884] = if s.b[2884] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2881])) && s.b[2884]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_sub_ad_lhs(1946, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1964))), 1996);
            s.store_add_ad_rhs(1965, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.6, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2885] = (((s.v[1965]) as f64).abs() <= s.v[1933]);
        s.v[2885] = if s.b[2885] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2885]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1965), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1965), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1965), 0.16666666666666666)))));
        }

        s.b[2886] = ((((-s.v[1965])) as f64).abs() < 230.25850929940458);
        s.v[2886] = if s.b[2886] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && s.b[2886]) {
            s.store_exp_neg_input(2027, 1965);
        }

        s.b[2887] = ((-s.v[1965]) < 0.0);
        s.v[2887] = if s.b[2887] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && (!s.b[2886])) && s.b[2887]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1965))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1965))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1965))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && (!s.b[2886])) && (!s.b[2887])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1965)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1965)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1965)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1965)), (-1.0)));
        }

        s.b[2888] = (s.v[1965] > s.v[1933]);
        s.v[2888] = if s.b[2888] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2885])) && s.b[2888]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_sub_ad_lhs(1947, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1965))), 1996);
            s.store_add_ad_rhs(1966, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.7, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2889] = (((s.v[1966]) as f64).abs() <= s.v[1933]);
        s.v[2889] = if s.b[2889] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2889]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1966), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1966), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1966), 0.16666666666666666)))));
        }

        s.b[2890] = ((((-s.v[1966])) as f64).abs() < 230.25850929940458);
        s.v[2890] = if s.b[2890] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && s.b[2890]) {
            s.store_exp_neg_input(2027, 1966);
        }

        s.b[2891] = ((-s.v[1966]) < 0.0);
        s.v[2891] = if s.b[2891] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && (!s.b[2890])) && s.b[2891]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1966))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1966))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1966))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && (!s.b[2890])) && (!s.b[2891])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1966)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1966)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1966)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1966)), (-1.0)));
        }

        s.b[2892] = (s.v[1966] > s.v[1933]);
        s.v[2892] = if s.b[2892] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2889])) && s.b[2892]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_sub_ad_lhs(1948, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1966))), 1996);
            s.store_add_ad_rhs(1967, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.8, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2893] = (((s.v[1967]) as f64).abs() <= s.v[1933]);
        s.v[2893] = if s.b[2893] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2893]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1967), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1967), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1967), 0.16666666666666666)))));
        }

        s.b[2894] = ((((-s.v[1967])) as f64).abs() < 230.25850929940458);
        s.v[2894] = if s.b[2894] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && s.b[2894]) {
            s.store_exp_neg_input(2027, 1967);
        }

        s.b[2895] = ((-s.v[1967]) < 0.0);
        s.v[2895] = if s.b[2895] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && (!s.b[2894])) && s.b[2895]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1967))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1967))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1967))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && (!s.b[2894])) && (!s.b[2895])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1967)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1967)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1967)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1967)), (-1.0)));
        }

        s.b[2896] = (s.v[1967] > s.v[1933]);
        s.v[2896] = if s.b[2896] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2893])) && s.b[2896]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_sub_ad_lhs(1949, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1967))), 1996);
            s.store_add_ad_rhs(1968, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.9, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.b[2897] = (((s.v[1968]) as f64).abs() <= s.v[1933]);
        s.v[2897] = if s.b[2897] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2897]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1968), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1968), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1968), 0.16666666666666666)))));
        }

        s.b[2898] = ((((-s.v[1968])) as f64).abs() < 230.25850929940458);
        s.v[2898] = if s.b[2898] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && s.b[2898]) {
            s.store_exp_neg_input(2027, 1968);
        }

        s.b[2899] = ((-s.v[1968]) < 0.0);
        s.v[2899] = if s.b[2899] { 1.0 } else { 0.0 };

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && (!s.b[2898])) && s.b[2899]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1968))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1968))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1968))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && (!s.b[2898])) && (!s.b[2899])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1968)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1968)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1968)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1968)), (-1.0)));
        }

        s.b[2900] = (s.v[1968] > s.v[1933]);
        s.v[2900] = if s.b[2900] { 1.0 } else { 0.0 };

        if ((((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && (!s.b[2897])) && s.b[2900]) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) {
            s.store_sub_ad_lhs(1950, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1968))), 1996);
        }

        s.b[2901] = (s.v[831] < 0.0);
        s.v[2901] = if s.b[2901] { 1.0 } else { 0.0 };

        if (((((((s.b[2811] && s.b[2812]) && (!s.b[2813])) && (!s.b[2818])) && (!s.b[2828])) && (!s.b[2842])) && s.b[2864]) && s.b[2901]) {
            s.copy_ad(2027, 1942);
            s.copy_ad(1942, 1950);
            s.copy_ad(1950, 2027);
            s.copy_ad(2027, 1943);
            s.copy_ad(1943, 1949);
            s.copy_ad(1949, 2027);
            s.copy_ad(2027, 1944);
            s.copy_ad(1944, 1948);
            s.copy_ad(1948, 2027);
            s.copy_ad(2027, 1945);
            s.copy_ad(1945, 1947);
            s.copy_ad(1947, 2027);
        }

        s.v[1983] = 0.0;

        s.v[1984] = 0.0;

        s.v[1978] = 0.0;

        s.v[1979] = 0.0;

        s.b[2902] = (s.v[1] != 0.0);
        s.v[2902] = if s.b[2902] { 1.0 } else { 0.0 };

        if s.b[2902] {
            s.store_sub_ad_rhs(1983, 1934, A::mul(A::mul(A::scale(s.ad_value(831), 0.5), s.ad_value(1893)), s.ad_value(1932)));
            s.store_add_ad_rhs(1984, 1934, A::mul(A::mul(A::scale(s.ad_value(831), 0.5), s.ad_value(1893)), s.ad_value(1932)));
            s.store_scalar(1978, 0.0);
            s.store_scalar(1979, 0.0);
        }

        s.b[2903] = (s.v[1983] > 0.0);
        s.v[2903] = if s.b[2903] { 1.0 } else { 0.0 };

        s.b[2904] = (((s.v[1983]) as f64).abs() <= s.v[1933]);
        s.v[2904] = if s.b[2904] { 1.0 } else { 0.0 };

        if ((s.b[2902] && s.b[2903]) && s.b[2904]) {
            s.store_mul_ad(1997, A::mul(A::scale(s.ad_value(1983), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1983), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1983), 0.16666666666666666)))));
        }

        s.b[2905] = ((((-s.v[1983])) as f64).abs() < 230.25850929940458);
        s.v[2905] = if s.b[2905] { 1.0 } else { 0.0 };

        if (((s.b[2902] && s.b[2903]) && (!s.b[2904])) && s.b[2905]) {
            s.store_exp_neg_input(2027, 1983);
        }

        s.b[2906] = ((-s.v[1983]) < 0.0);
        s.v[2906] = if s.b[2906] { 1.0 } else { 0.0 };

        if ((((s.b[2902] && s.b[2903]) && (!s.b[2904])) && (!s.b[2905])) && s.b[2906]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1983))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1983))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1983))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2902] && s.b[2903]) && (!s.b[2904])) && (!s.b[2905])) && (!s.b[2906])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1983)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1983)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1983)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2902] && s.b[2903]) && (!s.b[2904])) {
            s.store_mul_sqrt_ad_rhs(1997, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1983)), (-1.0)));
        }

        s.b[2907] = (s.v[1983] > s.v[1933]);
        s.v[2907] = if s.b[2907] { 1.0 } else { 0.0 };

        if (((s.b[2902] && s.b[2903]) && (!s.b[2904])) && s.b[2907]) {
            s.store_neg(1997, 1997);
        }

        if (s.b[2902] && s.b[2903]) {
            s.store_sub_ad_lhs(1978, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1983))), 1997);
        }

        s.b[2908] = (s.v[1984] > 0.0);
        s.v[2908] = if s.b[2908] { 1.0 } else { 0.0 };

        s.b[2909] = (((s.v[1984]) as f64).abs() <= s.v[1933]);
        s.v[2909] = if s.b[2909] { 1.0 } else { 0.0 };

        if ((s.b[2902] && s.b[2908]) && s.b[2909]) {
            s.store_mul_ad(1997, A::mul(A::scale(s.ad_value(1984), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1984), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1984), 0.16666666666666666)))));
        }

        s.b[2910] = ((((-s.v[1984])) as f64).abs() < 230.25850929940458);
        s.v[2910] = if s.b[2910] { 1.0 } else { 0.0 };

        if (((s.b[2902] && s.b[2908]) && (!s.b[2909])) && s.b[2910]) {
            s.store_exp_neg_input(2027, 1984);
        }

        s.b[2911] = ((-s.v[1984]) < 0.0);
        s.v[2911] = if s.b[2911] { 1.0 } else { 0.0 };

        if ((((s.b[2902] && s.b[2908]) && (!s.b[2909])) && (!s.b[2910])) && s.b[2911]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1984))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1984))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1984))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2902] && s.b[2908]) && (!s.b[2909])) && (!s.b[2910])) && (!s.b[2911])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(1984)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1984)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1984)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2902] && s.b[2908]) && (!s.b[2909])) {
            s.store_mul_sqrt_ad_rhs(1997, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(1984)), (-1.0)));
        }

        s.b[2912] = (s.v[1984] > s.v[1933]);
        s.v[2912] = if s.b[2912] { 1.0 } else { 0.0 };

        if (((s.b[2902] && s.b[2908]) && (!s.b[2909])) && s.b[2912]) {
            s.store_neg(1997, 1997);
        }

        if (s.b[2902] && s.b[2908]) {
            s.store_sub_ad_lhs(1979, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1984))), 1997);
        }

        s.store_scaled_voltage(1969, ctx, nodes, Some(12), None, s.v[3]);

        s.store_scaled_voltage(1970, ctx, nodes, Some(13), None, s.v[3]);

        s.store_scaled_voltage(1971, ctx, nodes, Some(14), None, s.v[3]);

        s.store_scaled_voltage(1972, ctx, nodes, Some(15), None, s.v[3]);

        s.store_scaled_voltage(1973, ctx, nodes, Some(16), None, s.v[3]);

    }

    pub(super) fn stamp_reactive_block_25(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.store_scaled_voltage(1974, ctx, nodes, Some(17), None, s.v[3]);

        s.store_scaled_voltage(1975, ctx, nodes, Some(18), None, s.v[3]);

        s.store_scaled_voltage(1976, ctx, nodes, Some(19), None, s.v[3]);

        s.store_scaled_voltage(1977, ctx, nodes, Some(20), None, s.v[3]);

        s.v[1995] = 0.0;

        s.b[2921] = (s.v[1] != 0.0);
        s.v[2921] = if s.b[2921] { 1.0 } else { 0.0 };

        if s.b[2921] {
            s.store_div_ad(1995, A::mul(A::mul(s.ad_value(307), s.ad_value(1888)), s.ad_value(716)), A::mul(s.ad_value(1904), s.ad_value(1906)));
            s.store_mul_ad_product_lhs(2018, A::square(s.ad_value(1907)), s.ad_value(1888), 1888);
        }

        s.b[2922] = (s.v[1] == 1.0);
        s.v[2922] = if s.b[2922] { 1.0 } else { 0.0 };

        if (s.b[2921] && s.b[2922]) {
            s.store_sub(1992, 1979, 1978);
            s.store_sub_scaled_ad_lhs(1993, A::scale(A::add(s.ad_value(1978), s.ad_value(1979)), 6.0), 1969, 12.0);
        }

        s.b[2923] = (s.v[1] == 2.0);
        s.v[2923] = if s.b[2923] { 1.0 } else { 0.0 };

        if ((s.b[2921] && (!s.b[2922])) && s.b[2923]) {
            s.store_scale_ad(1992, A::sub(A::add(A::sub(A::scale(s.ad_value(1978), (-7.0)), A::scale(s.ad_value(1969), 3.0)), A::scale(s.ad_value(1970), 12.0)), A::scale(s.ad_value(1979), 2.0)), 0.2);
            s.store_scaled_add_ad_lhs(1993, A::sub(A::add(A::scale(s.ad_value(1978), (-4.0)), A::scale(s.ad_value(1969), 9.0)), A::scale(s.ad_value(1970), 6.0)), 1979, ((-18.0) / 5.0));
        }

        s.b[2924] = (s.v[1] == 3.0);
        s.v[2924] = if s.b[2924] { 1.0 } else { 0.0 };

        if (((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && s.b[2924]) {
            s.store_scaled_add_ad_lhs(1992, A::sub(A::add(A::sub(A::scale(s.ad_value(1978), (-13.0)), A::scale(s.ad_value(1969), 6.0)), A::scale(s.ad_value(1970), 24.0)), A::scale(s.ad_value(1971), 6.0)), 1979, 0.14285714285714285);
            s.store_scale_ad(1993, A::add(A::sub(A::add(A::sub(A::scale(s.ad_value(1978), 180.0), A::scale(s.ad_value(1969), 408.0)), A::scale(s.ad_value(1970), 288.0)), A::scale(s.ad_value(1971), 72.0)), A::scale(s.ad_value(1979), 12.0)), 0.14285714285714285);
        }

        s.b[2925] = (s.v[1] == 5.0);
        s.v[2925] = if s.b[2925] { 1.0 } else { 0.0 };

        if ((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && s.b[2925]) {
            s.store_scale_ad(1992, A::add(A::add(A::sub(A::sub(A::add(A::sub(A::scale(s.ad_value(1978), (-181.0)), A::scale(s.ad_value(1969), 84.0)), A::scale(s.ad_value(1972), 24.0)), A::scale(s.ad_value(1973), 6.0)), A::scale(s.ad_value(1971), 90.0)), s.ad_value(1979)), A::scale(s.ad_value(1970), 336.0)), 0.015384615384615385);
            s.store_scale_ad(1993, A::add(A::sub(A::add(A::add(A::sub(A::sub(A::scale(s.ad_value(1972), 432.0), A::scale(s.ad_value(1973), 108.0)), A::scale(s.ad_value(1971), 1620.0)), A::scale(s.ad_value(1979), 18.0)), A::scale(s.ad_value(1978), 3762.0)), A::scale(s.ad_value(1969), 8532.0)), A::scale(s.ad_value(1970), 6048.0)), 0.015384615384615385);
        }

        s.b[2926] = (s.v[1] == 9.0);
        s.v[2926] = if s.b[2926] { 1.0 } else { 0.0 };

        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && s.b[2926]) {
            let assign66170_ad_e88035: A = A::sub(A::add(A::sub(A::sub(A::add(A::sub(A::add(A::add(A::scale(s.ad_value(1974), 1680.0), A::scale(s.ad_value(1972), 23400.0)), A::scale(s.ad_value(1979), 5.0)), A::scale(s.ad_value(1971), 87330.0)), A::scale(s.ad_value(1976), 120.0)), A::scale(s.ad_value(1975), 450.0)), A::scale(s.ad_value(1969), 81480.0)), A::scale(s.ad_value(1970), 325920.0)), A::scale(s.ad_value(1978), 175565.0));
            s.store_sub_scaled_ad_lhs(1992, A::scale(A::sub(assign66170_ad_e88035, A::scale(s.ad_value(1977), 30.0)), 2.6434745829918846e-5), 1973, (30.0 / 181.0));
        }

        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && s.b[2926]) {
            let assign66180_ad_e88100: A = A::sub(A::add(A::add(A::add(A::add(A::sub(A::sub(A::add(A::scale(s.ad_value(1975), (-13500.0)), A::scale(s.ad_value(1972), 702000.0)), A::scale(s.ad_value(1971), 2619900.0)), A::scale(s.ad_value(1969), 13793100.0)), A::scale(s.ad_value(1970), 9777600.0)), A::scale(s.ad_value(1978), 6081750.0)), A::scale(s.ad_value(1979), 150.0)), A::scale(s.ad_value(1976), 3600.0)), A::scale(s.ad_value(1977), 900.0));
            s.store_sub_scaled_ad_lhs(1993, A::scale(A::add(assign66180_ad_e88100, A::scale(s.ad_value(1974), 50400.0)), 2.6434745829918846e-5), 1973, (900.0 / 181.0));
        }

        if (((((s.b[2921] && (!s.b[2922])) && (!s.b[2923])) && (!s.b[2924])) && (!s.b[2925])) && (!s.b[2926])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2921] {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1969), s.ad_value(1937)), 1890);
        }

        s.b[2927] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[2927] = if s.b[2927] { 1.0 } else { 0.0 };

        if (s.b[2921] && s.b[2927]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2928] = (s.v[2027] < (-s.v[1941]));
        s.v[2928] = if s.b[2928] { 1.0 } else { 0.0 };

        if ((s.b[2921] && (!s.b[2927])) && s.b[2928]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[2929] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[2929] = if s.b[2929] { 1.0 } else { 0.0 };

        if (((s.b[2921] && (!s.b[2927])) && s.b[2928]) && s.b[2929]) {
            s.store_exp(2005, 2015);
        }

        s.b[2930] = (s.v[2015] < 0.0);
        s.v[2930] = if s.b[2930] { 1.0 } else { 0.0 };

        if ((((s.b[2921] && (!s.b[2927])) && s.b[2928]) && (!s.b[2929])) && s.b[2930]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2921] && (!s.b[2927])) && s.b[2928]) && (!s.b[2929])) && (!s.b[2930])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2921] && (!s.b[2927])) && s.b[2928]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[2931] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[2931] = if s.b[2931] { 1.0 } else { 0.0 };

        if (((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && s.b[2931]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2932] = ((-s.v[2011]) < 0.0);
        s.v[2932] = if s.b[2932] { 1.0 } else { 0.0 };

        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2931])) && s.b[2932]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2931])) && (!s.b[2932])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[2933] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[2933] = if s.b[2933] { 1.0 } else { 0.0 };

        if (((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && s.b[2933]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[2934] = ((-s.v[2013]) < 0.0);
        s.v[2934] = if s.b[2934] { 1.0 } else { 0.0 };

        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2933])) && s.b[2934]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) && (!s.b[2933])) && (!s.b[2934])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2921] && (!s.b[2927])) && (!s.b[2928])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2016, 2013, 2014);
        }

        s.b[2935] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[2935] = if s.b[2935] { 1.0 } else { 0.0 };

        if (s.b[2921] && s.b[2935]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
            s.store_mul_scaled_ad_rhs(1991, 1889, (-0.70710678), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
            s.store_mul_scaled_ad_rhs(1990, 1889, (-0.235702), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.b[2936] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[2936] = if s.b[2936] { 1.0 } else { 0.0 };

        if ((s.b[2921] && (!s.b[2935])) && s.b[2936]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[2937] = ((-s.v[2016]) < 0.0);
        s.v[2937] = if s.b[2937] { 1.0 } else { 0.0 };

        if (((s.b[2921] && (!s.b[2935])) && (!s.b[2936])) && s.b[2937]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[2921] && (!s.b[2935])) && (!s.b[2936])) && (!s.b[2937])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2921] && (!s.b[2935])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[2938] = (s.v[2016] > s.v[1933]);
        s.v[2938] = if s.b[2938] { 1.0 } else { 0.0 };

        if ((s.b[2921] && (!s.b[2935])) && s.b[2938]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[2921] && (!s.b[2935])) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if s.b[2921] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1969, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul(A::mul(s.ad_value(1969), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[2939] = (s.v[0] == (-1.0));
        s.v[2939] = if s.b[2939] { 1.0 } else { 0.0 };

        if (s.b[2921] && s.b[2939]) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if s.b[2921] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
            s.store_mul_sub_ad_rhs(1951, 2019, s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027)));
        }

        if (!s.b[2921]) {
            s.store_scalar(2018, 0.0);
        }

        s.b[2940] = (s.v[1] >= 2.0);
        s.v[2940] = if s.b[2940] { 1.0 } else { 0.0 };

        s.b[2941] = (s.v[1] == 2.0);
        s.v[2941] = if s.b[2941] { 1.0 } else { 0.0 };

        if (s.b[2940] && s.b[2941]) {
            s.store_scale_ad(1992, A::add(A::add(A::sub(A::scale(s.ad_value(1978), 2.0), A::scale(s.ad_value(1969), 12.0)), A::scale(s.ad_value(1970), 3.0)), A::scale(s.ad_value(1979), 7.0)), 0.2);
            s.store_scaled_add_ad_lhs(1993, A::sub(A::add(A::scale(s.ad_value(1979), (-4.0)), A::scale(s.ad_value(1970), 9.0)), A::scale(s.ad_value(1969), 6.0)), 1978, ((-18.0) / 5.0));
        }

        s.b[2942] = (s.v[1] == 3.0);
        s.v[2942] = if s.b[2942] { 1.0 } else { 0.0 };

        if ((s.b[2940] && (!s.b[2941])) && s.b[2942]) {
            s.store_sub_scaled_ad_lhs(1992, A::add(A::sub(A::scale(s.ad_value(1978), 0.5), A::scale(s.ad_value(1969), 3.0)), A::scale(s.ad_value(1971), 3.0)), 1979, 0.5);
            s.store_scale_ad(1993, A::sub(A::add(A::sub(A::add(A::scale(s.ad_value(1978), (-48.0)), A::scale(s.ad_value(1969), 288.0)), A::scale(s.ad_value(1970), 480.0)), A::scale(s.ad_value(1971), 288.0)), A::scale(s.ad_value(1979), 48.0)), 0.14285714285714285);
        }

        s.b[2943] = (s.v[1] == 5.0);
        s.v[2943] = if s.b[2943] { 1.0 } else { 0.0 };

        if (((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && s.b[2943]) {
            s.store_add_ad(1992, A::scale(A::add(A::sub(A::sub(A::scale(s.ad_value(1969), (-291.0)), A::scale(s.ad_value(1970), 6.0)), A::scale(s.ad_value(1972), 84.0)), A::scale(s.ad_value(1973), 21.0)), 0.015384615384615385), A::scale(A::add(A::sub(A::scale(s.ad_value(1971), 630.0), A::scale(s.ad_value(1979), 7.0)), A::scale(s.ad_value(1978), 97.0)), 0.007692307692307693));
            s.store_scale_ad(1993, A::sub(A::add(A::sub(A::sub(A::add(A::add(A::scale(s.ad_value(1972), (-1728.0)), A::scale(s.ad_value(1973), 432.0)), A::scale(s.ad_value(1971), 6480.0)), A::scale(s.ad_value(1979), 72.0)), A::scale(s.ad_value(1978), 1008.0)), A::scale(s.ad_value(1969), 6048.0)), A::scale(s.ad_value(1970), 10152.0)), 0.015384615384615385);
        }

        s.b[2944] = (s.v[1] == 9.0);
        s.v[2944] = if s.b[2944] { 1.0 } else { 0.0 };

        if ((((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && (!s.b[2943])) && s.b[2944]) {
            let assign67050_ad_e89539: A = A::scale(A::sub(A::add(A::sub(A::add(A::sub(A::add(A::sub(A::scale(s.ad_value(1974), (-5880.0)), A::scale(s.ad_value(1972), 81900.0)), A::scale(s.ad_value(1971), 305655.0)), A::scale(s.ad_value(1976), 420.0)), A::scale(s.ad_value(1977), 105.0)), A::scale(s.ad_value(1969), 282255.0)), A::scale(s.ad_value(1975), 1575.0)), A::scale(s.ad_value(1970), 5850.0)), 2.6434745829918846e-5);
            s.store_add_ad(1992, A::add(assign67050_ad_e89539, A::scale(s.ad_value(1973), (105.0 / 181.0))), A::scale(A::sub(A::scale(s.ad_value(1978), 94085.0), A::scale(s.ad_value(1979), 35.0)), 1.3217372914959423e-5));
        }

        if ((((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && (!s.b[2943])) && s.b[2944]) {
            let assign67060_ad_e89604: A = A::add(A::sub(A::sub(A::sub(A::sub(A::add(A::sub(A::add(A::scale(s.ad_value(1969), 9777600.0), A::scale(s.ad_value(1975), 54000.0)), A::scale(s.ad_value(1972), 2808000.0)), A::scale(s.ad_value(1971), 10479600.0)), A::scale(s.ad_value(1970), 16413000.0)), A::scale(s.ad_value(1978), 1629600.0)), A::scale(s.ad_value(1979), 600.0)), A::scale(s.ad_value(1976), 14400.0)), A::scale(s.ad_value(1977), 3600.0));
            s.store_add_scaled_ad_lhs(1993, A::scale(A::sub(assign67060_ad_e89604, A::scale(s.ad_value(1974), 201600.0)), 2.6434745829918846e-5), 1973, (3600.0 * 0.0055248618784530384));
        }

        if ((((s.b[2940] && (!s.b[2941])) && (!s.b[2942])) && (!s.b[2943])) && (!s.b[2944])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2940] {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1970), s.ad_value(1937)), 1890);
        }

        s.b[2945] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[2945] = if s.b[2945] { 1.0 } else { 0.0 };

        if (s.b[2940] && s.b[2945]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2946] = (s.v[2027] < (-s.v[1941]));
        s.v[2946] = if s.b[2946] { 1.0 } else { 0.0 };

        if ((s.b[2940] && (!s.b[2945])) && s.b[2946]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[2947] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[2947] = if s.b[2947] { 1.0 } else { 0.0 };

        if (((s.b[2940] && (!s.b[2945])) && s.b[2946]) && s.b[2947]) {
            s.store_exp(2005, 2015);
        }

        s.b[2948] = (s.v[2015] < 0.0);
        s.v[2948] = if s.b[2948] { 1.0 } else { 0.0 };

        if ((((s.b[2940] && (!s.b[2945])) && s.b[2946]) && (!s.b[2947])) && s.b[2948]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2940] && (!s.b[2945])) && s.b[2946]) && (!s.b[2947])) && (!s.b[2948])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2940] && (!s.b[2945])) && s.b[2946]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

    }

    pub(super) fn stamp_reactive_block_26(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2940] && (!s.b[2945])) && s.b[2946]) {
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[2949] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[2949] = if s.b[2949] { 1.0 } else { 0.0 };

        if (((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && s.b[2949]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2950] = ((-s.v[2011]) < 0.0);
        s.v[2950] = if s.b[2950] { 1.0 } else { 0.0 };

        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2949])) && s.b[2950]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2949])) && (!s.b[2950])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[2951] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[2951] = if s.b[2951] { 1.0 } else { 0.0 };

        if (((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && s.b[2951]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[2952] = ((-s.v[2013]) < 0.0);
        s.v[2952] = if s.b[2952] { 1.0 } else { 0.0 };

        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2951])) && s.b[2952]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) && (!s.b[2951])) && (!s.b[2952])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2940] && (!s.b[2945])) && (!s.b[2946])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2016, 2013, 2014);
        }

        s.b[2953] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[2953] = if s.b[2953] { 1.0 } else { 0.0 };

        if (s.b[2940] && s.b[2953]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
            s.store_mul_scaled_ad_rhs(1991, 1889, (-0.70710678), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
            s.store_mul_scaled_ad_rhs(1990, 1889, (-0.235702), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.b[2954] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[2954] = if s.b[2954] { 1.0 } else { 0.0 };

        if ((s.b[2940] && (!s.b[2953])) && s.b[2954]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[2955] = ((-s.v[2016]) < 0.0);
        s.v[2955] = if s.b[2955] { 1.0 } else { 0.0 };

        if (((s.b[2940] && (!s.b[2953])) && (!s.b[2954])) && s.b[2955]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[2940] && (!s.b[2953])) && (!s.b[2954])) && (!s.b[2955])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2940] && (!s.b[2953])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[2956] = (s.v[2016] > s.v[1933]);
        s.v[2956] = if s.b[2956] { 1.0 } else { 0.0 };

        if ((s.b[2940] && (!s.b[2953])) && s.b[2956]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[2940] && (!s.b[2953])) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if s.b[2940] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1970, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul(A::mul(s.ad_value(1970), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[2957] = (s.v[0] == (-1.0));
        s.v[2957] = if s.b[2957] { 1.0 } else { 0.0 };

        if (s.b[2940] && s.b[2957]) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if s.b[2940] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
            s.store_mul_sub_ad_rhs(1952, 2019, s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027)));
        }

        s.b[2958] = (s.v[1] >= 3.0);
        s.v[2958] = if s.b[2958] { 1.0 } else { 0.0 };

        s.b[2959] = (s.v[1] == 3.0);
        s.v[2959] = if s.b[2959] { 1.0 } else { 0.0 };

        if (s.b[2958] && s.b[2959]) {
            s.store_scaled_sub_ad_lhs(1992, A::add(A::sub(A::add(A::scale(s.ad_value(1979), 13.0), A::scale(s.ad_value(1971), 6.0)), A::scale(s.ad_value(1970), 24.0)), A::scale(s.ad_value(1969), 6.0)), 1978, 0.14285714285714285);
            s.store_scale_ad(1993, A::add(A::sub(A::add(A::sub(A::scale(s.ad_value(1979), 180.0), A::scale(s.ad_value(1971), 408.0)), A::scale(s.ad_value(1970), 288.0)), A::scale(s.ad_value(1969), 72.0)), A::scale(s.ad_value(1978), 12.0)), 0.14285714285714285);
        }

        s.b[2960] = (s.v[1] == 5.0);
        s.v[2960] = if s.b[2960] { 1.0 } else { 0.0 };

        if ((s.b[2958] && (!s.b[2959])) && s.b[2960]) {
            s.store_scaled_sub_ad_lhs(1992, A::add(A::sub(A::add(A::sub(s.ad_value(1979), A::scale(s.ad_value(1973), 6.0)), A::scale(s.ad_value(1972), 24.0)), A::scale(s.ad_value(1970), 24.0)), A::scale(s.ad_value(1969), 6.0)), 1978, 0.2);
            s.store_scaled_add_ad(1993, A::sub(A::sub(A::scale(A::add(s.ad_value(1972), s.ad_value(1970)), 1296.0), A::scale(A::add(s.ad_value(1973), s.ad_value(1969)), 324.0)), A::scale(s.ad_value(1971), 2052.0)), A::scale(A::add(s.ad_value(1979), s.ad_value(1978)), 54.0), 0.07692307692307693);
        }

        s.b[2961] = (s.v[1] == 9.0);
        s.v[2961] = if s.b[2961] { 1.0 } else { 0.0 };

        if (((s.b[2958] && (!s.b[2959])) && (!s.b[2960])) && s.b[2961]) {
            let assign67890_ad_e90954: A = A::sub(A::add(A::sub(A::sub(A::add(A::sub(A::add(A::add(A::scale(s.ad_value(1974), 21840.0), A::scale(s.ad_value(1972), 304200.0)), A::scale(s.ad_value(1979), 65.0)), A::scale(s.ad_value(1971), 420.0)), A::scale(s.ad_value(1976), 1560.0)), A::scale(s.ad_value(1978), 12605.0)), A::scale(s.ad_value(1977), 390.0)), A::scale(s.ad_value(1969), 75630.0)), A::scale(s.ad_value(1975), 5850.0));
            s.store_sub_scaled_ad_lhs(1992, A::scale(A::sub(assign67890_ad_e90954, A::scale(s.ad_value(1970), 302520.0)), 2.6434745829918846e-5), 1973, (390.0 / 181.0));
        }

        if (((s.b[2958] && (!s.b[2959])) && (!s.b[2960])) && s.b[2961]) {
            let assign67900_ad_e91013: A = A::sub(A::add(A::add(A::add(A::add(A::sub(A::add(A::sub(A::scale(s.ad_value(1969), (-2619900.0)), A::scale(s.ad_value(1975), 202500.0)), A::scale(s.ad_value(1972), 10530000.0)), A::scale(s.ad_value(1971), 16601100.0)), A::scale(s.ad_value(1970), 10479600.0)), A::scale(s.ad_value(1978), 436650.0)), A::scale(s.ad_value(1979), 2250.0)), A::scale(s.ad_value(1976), 54000.0)), A::scale(s.ad_value(1977), 13500.0));
            s.store_sub_scaled_ad_lhs(1993, A::scale(A::add(assign67900_ad_e91013, A::scale(s.ad_value(1974), 756000.0)), 2.6434745829918846e-5), 1973, (13500.0 * 0.0055248618784530384));
        }

        if (((s.b[2958] && (!s.b[2959])) && (!s.b[2960])) && (!s.b[2961])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2958] {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1971), s.ad_value(1937)), 1890);
        }

        s.b[2962] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[2962] = if s.b[2962] { 1.0 } else { 0.0 };

        if (s.b[2958] && s.b[2962]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2963] = (s.v[2027] < (-s.v[1941]));
        s.v[2963] = if s.b[2963] { 1.0 } else { 0.0 };

        if ((s.b[2958] && (!s.b[2962])) && s.b[2963]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[2964] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[2964] = if s.b[2964] { 1.0 } else { 0.0 };

        if (((s.b[2958] && (!s.b[2962])) && s.b[2963]) && s.b[2964]) {
            s.store_exp(2005, 2015);
        }

        s.b[2965] = (s.v[2015] < 0.0);
        s.v[2965] = if s.b[2965] { 1.0 } else { 0.0 };

        if ((((s.b[2958] && (!s.b[2962])) && s.b[2963]) && (!s.b[2964])) && s.b[2965]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2958] && (!s.b[2962])) && s.b[2963]) && (!s.b[2964])) && (!s.b[2965])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2958] && (!s.b[2962])) && s.b[2963]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[2966] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[2966] = if s.b[2966] { 1.0 } else { 0.0 };

        if (((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && s.b[2966]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2967] = ((-s.v[2011]) < 0.0);
        s.v[2967] = if s.b[2967] { 1.0 } else { 0.0 };

        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2966])) && s.b[2967]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2966])) && (!s.b[2967])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[2968] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[2968] = if s.b[2968] { 1.0 } else { 0.0 };

        if (((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && s.b[2968]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[2969] = ((-s.v[2013]) < 0.0);
        s.v[2969] = if s.b[2969] { 1.0 } else { 0.0 };

        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2968])) && s.b[2969]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) && (!s.b[2968])) && (!s.b[2969])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2958] && (!s.b[2962])) && (!s.b[2963])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2016, 2013, 2014);
        }

        s.b[2970] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[2970] = if s.b[2970] { 1.0 } else { 0.0 };

        if (s.b[2958] && s.b[2970]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
            s.store_mul_scaled_ad_rhs(1991, 1889, (-0.70710678), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
            s.store_mul_scaled_ad_rhs(1990, 1889, (-0.235702), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.b[2971] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[2971] = if s.b[2971] { 1.0 } else { 0.0 };

        if ((s.b[2958] && (!s.b[2970])) && s.b[2971]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[2972] = ((-s.v[2016]) < 0.0);
        s.v[2972] = if s.b[2972] { 1.0 } else { 0.0 };

        if (((s.b[2958] && (!s.b[2970])) && (!s.b[2971])) && s.b[2972]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[2958] && (!s.b[2970])) && (!s.b[2971])) && (!s.b[2972])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2958] && (!s.b[2970])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[2973] = (s.v[2016] > s.v[1933]);
        s.v[2973] = if s.b[2973] { 1.0 } else { 0.0 };

        if ((s.b[2958] && (!s.b[2970])) && s.b[2973]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[2958] && (!s.b[2970])) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if s.b[2958] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1971, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul(A::mul(s.ad_value(1971), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[2974] = (s.v[0] == (-1.0));
        s.v[2974] = if s.b[2974] { 1.0 } else { 0.0 };

        if (s.b[2958] && s.b[2974]) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if s.b[2958] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
            s.store_mul_sub_ad_rhs(1953, 2019, s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027)));
        }

        s.b[2975] = (s.v[1] >= 4.0);
        s.v[2975] = if s.b[2975] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_27(
        s: &mut ReactiveScratch,
    ) {
        s.b[2976] = (s.v[1] == 5.0);
        s.v[2976] = if s.b[2976] { 1.0 } else { 0.0 };

        if (s.b[2975] && s.b[2976]) {
            s.store_scale_ad(1992, A::add(A::sub(A::add(A::sub(A::add(A::add(A::scale(s.ad_value(1971), (-630.0)), A::scale(s.ad_value(1972), 12.0)), A::scale(s.ad_value(1973), 582.0)), A::scale(s.ad_value(1979), 97.0)), A::scale(s.ad_value(1978), 7.0)), A::scale(s.ad_value(1969), 42.0)), A::scale(s.ad_value(1970), 168.0)), 0.007692307692307693);
            s.store_scale_ad(1993, A::sub(A::add(A::sub(A::sub(A::add(A::add(A::scale(s.ad_value(1972), (-10152.0)), A::scale(s.ad_value(1973), 6048.0)), A::scale(s.ad_value(1971), 6480.0)), A::scale(s.ad_value(1979), 1008.0)), A::scale(s.ad_value(1978), 72.0)), A::scale(s.ad_value(1969), 432.0)), A::scale(s.ad_value(1970), 1728.0)), 0.015384615384615385);
        }

        s.b[2977] = (s.v[1] == 9.0);
        s.v[2977] = if s.b[2977] { 1.0 } else { 0.0 };

        if ((s.b[2975] && (!s.b[2976])) && s.b[2977]) {
            let assign68700_ad_e92310: A = A::scale(A::add(A::add(A::sub(A::add(A::sub(A::sub(A::sub(A::scale(s.ad_value(1974), (-81480.0)), A::scale(s.ad_value(1972), 30.0)), A::scale(s.ad_value(1971), 303975.0)), A::scale(s.ad_value(1976), 5820.0)), A::scale(s.ad_value(1977), 1455.0)), A::scale(s.ad_value(1969), 20265.0)), A::scale(s.ad_value(1975), 21825.0)), A::scale(s.ad_value(1970), 81060.0)), 2.6434745829918846e-5);
            s.store_add_scaled_ad_lhs(1992, A::add(A::sub(assign68700_ad_e92310, A::scale(s.ad_value(1979), (485.0 / 75658.0))), A::scale(s.ad_value(1973), (1455.0 * 0.0055248618784530384))), 1978, (6755.0 * 1.3217372914959423e-5));
        }

        if ((s.b[2975] && (!s.b[2976])) && s.b[2977]) {
            let assign68710_ad_e92371: A = A::add(A::sub(A::sub(A::sub(A::sub(A::add(A::sub(A::add(A::scale(s.ad_value(1969), 702000.0), A::scale(s.ad_value(1975), 756000.0)), A::scale(s.ad_value(1972), 16614600.0)), A::scale(s.ad_value(1971), 10530000.0)), A::scale(s.ad_value(1970), 2808000.0)), A::scale(s.ad_value(1978), 117000.0)), A::scale(s.ad_value(1979), 8400.0)), A::scale(s.ad_value(1976), 201600.0)), A::scale(s.ad_value(1977), 50400.0));
            s.store_add_scaled_ad_lhs(1993, A::scale(A::sub(assign68710_ad_e92371, A::scale(s.ad_value(1974), 2822400.0)), 2.6434745829918846e-5), 1973, (50400.0 * 0.0055248618784530384));
        }

        if ((s.b[2975] && (!s.b[2976])) && (!s.b[2977])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2975] {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1972), s.ad_value(1937)), 1890);
        }

        s.b[2978] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[2978] = if s.b[2978] { 1.0 } else { 0.0 };

        if (s.b[2975] && s.b[2978]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2979] = (s.v[2027] < (-s.v[1941]));
        s.v[2979] = if s.b[2979] { 1.0 } else { 0.0 };

        if ((s.b[2975] && (!s.b[2978])) && s.b[2979]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[2980] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[2980] = if s.b[2980] { 1.0 } else { 0.0 };

        if (((s.b[2975] && (!s.b[2978])) && s.b[2979]) && s.b[2980]) {
            s.store_exp(2005, 2015);
        }

        s.b[2981] = (s.v[2015] < 0.0);
        s.v[2981] = if s.b[2981] { 1.0 } else { 0.0 };

        if ((((s.b[2975] && (!s.b[2978])) && s.b[2979]) && (!s.b[2980])) && s.b[2981]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2975] && (!s.b[2978])) && s.b[2979]) && (!s.b[2980])) && (!s.b[2981])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2975] && (!s.b[2978])) && s.b[2979]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[2982] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[2982] = if s.b[2982] { 1.0 } else { 0.0 };

        if (((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && s.b[2982]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2983] = ((-s.v[2011]) < 0.0);
        s.v[2983] = if s.b[2983] { 1.0 } else { 0.0 };

        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2982])) && s.b[2983]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2982])) && (!s.b[2983])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[2984] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[2984] = if s.b[2984] { 1.0 } else { 0.0 };

        if (((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && s.b[2984]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[2985] = ((-s.v[2013]) < 0.0);
        s.v[2985] = if s.b[2985] { 1.0 } else { 0.0 };

        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2984])) && s.b[2985]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) && (!s.b[2984])) && (!s.b[2985])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2975] && (!s.b[2978])) && (!s.b[2979])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2016, 2013, 2014);
        }

        s.b[2986] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[2986] = if s.b[2986] { 1.0 } else { 0.0 };

        if (s.b[2975] && s.b[2986]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
            s.store_mul_scaled_ad_rhs(1991, 1889, (-0.70710678), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
            s.store_mul_scaled_ad_rhs(1990, 1889, (-0.235702), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.b[2987] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[2987] = if s.b[2987] { 1.0 } else { 0.0 };

        if ((s.b[2975] && (!s.b[2986])) && s.b[2987]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[2988] = ((-s.v[2016]) < 0.0);
        s.v[2988] = if s.b[2988] { 1.0 } else { 0.0 };

        if (((s.b[2975] && (!s.b[2986])) && (!s.b[2987])) && s.b[2988]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[2975] && (!s.b[2986])) && (!s.b[2987])) && (!s.b[2988])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2975] && (!s.b[2986])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[2989] = (s.v[2016] > s.v[1933]);
        s.v[2989] = if s.b[2989] { 1.0 } else { 0.0 };

        if ((s.b[2975] && (!s.b[2986])) && s.b[2989]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[2975] && (!s.b[2986])) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if s.b[2975] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1972, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul(A::mul(s.ad_value(1972), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[2990] = (s.v[0] == (-1.0));
        s.v[2990] = if s.b[2990] { 1.0 } else { 0.0 };

        if (s.b[2975] && s.b[2990]) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if s.b[2975] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
            s.store_mul_sub_ad_rhs(1954, 2019, s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027)));
        }

        s.b[2991] = (s.v[1] >= 5.0);
        s.v[2991] = if s.b[2991] { 1.0 } else { 0.0 };

        s.b[2992] = (s.v[1] == 5.0);
        s.v[2992] = if s.b[2992] { 1.0 } else { 0.0 };

        if (s.b[2991] && s.b[2992]) {
            s.store_scale_ad(1992, A::sub(A::add(A::sub(A::add(A::add(A::add(A::scale(s.ad_value(1972), (-336.0)), A::scale(s.ad_value(1973), 84.0)), A::scale(s.ad_value(1971), 90.0)), A::scale(s.ad_value(1979), 181.0)), s.ad_value(1978)), A::scale(s.ad_value(1969), 6.0)), A::scale(s.ad_value(1970), 24.0)), 0.015384615384615385);
            s.store_scale_ad(1993, A::sub(A::sub(A::sub(A::add(A::add(A::add(A::scale(s.ad_value(1978), 18.0), A::scale(s.ad_value(1979), 3762.0)), A::scale(s.ad_value(1972), 6048.0)), A::scale(s.ad_value(1970), 432.0)), A::scale(s.ad_value(1971), 1620.0)), A::scale(s.ad_value(1969), 108.0)), A::scale(s.ad_value(1973), 8532.0)), 0.015384615384615385);
        }

        s.b[2993] = (s.v[1] == 9.0);
        s.v[2993] = if s.b[2993] { 1.0 } else { 0.0 };

        if ((s.b[2991] && (!s.b[2992])) && s.b[2993]) {
            let assign69510_ad_e93656: A = A::scale(A::sub(A::add(A::add(A::add(A::scale(A::sub(s.ad_value(1974), s.ad_value(1972)), 1680.0), A::scale(A::sub(s.ad_value(1979), s.ad_value(1978)), 5.0)), A::scale(A::sub(s.ad_value(1971), s.ad_value(1975)), 450.0)), A::scale(A::sub(s.ad_value(1976), s.ad_value(1970)), 120.0)), A::scale(A::sub(s.ad_value(1977), s.ad_value(1969)), 30.0)), 0.004784688995215311);
            s.store_ad_value(1992, assign69510_ad_e93656);
        }

        if ((s.b[2991] && (!s.b[2992])) && s.b[2993]) {
            let assign69520_ad_e93698: A = A::add(A::add(A::add(A::sub(A::sub(A::scale(A::add(s.ad_value(1969), s.ad_value(1977)), (-900.0)), A::scale(A::add(s.ad_value(1975), s.ad_value(1971)), 13500.0)), A::scale(s.ad_value(1973), 79500.0)), A::scale(A::add(s.ad_value(1972), s.ad_value(1974)), 50400.0)), A::scale(A::add(s.ad_value(1970), s.ad_value(1976)), 3600.0)), A::scale(A::add(s.ad_value(1978), s.ad_value(1979)), 150.0));
            s.store_scale_ad(1993, assign69520_ad_e93698, 0.0055248618784530384);
        }

        if ((s.b[2991] && (!s.b[2992])) && (!s.b[2993])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[2991] {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1973), s.ad_value(1937)), 1890);
        }

        s.b[2994] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[2994] = if s.b[2994] { 1.0 } else { 0.0 };

        if (s.b[2991] && s.b[2994]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[2995] = (s.v[2027] < (-s.v[1941]));
        s.v[2995] = if s.b[2995] { 1.0 } else { 0.0 };

        if ((s.b[2991] && (!s.b[2994])) && s.b[2995]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[2996] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[2996] = if s.b[2996] { 1.0 } else { 0.0 };

        if (((s.b[2991] && (!s.b[2994])) && s.b[2995]) && s.b[2996]) {
            s.store_exp(2005, 2015);
        }

        s.b[2997] = (s.v[2015] < 0.0);
        s.v[2997] = if s.b[2997] { 1.0 } else { 0.0 };

        if ((((s.b[2991] && (!s.b[2994])) && s.b[2995]) && (!s.b[2996])) && s.b[2997]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2991] && (!s.b[2994])) && s.b[2995]) && (!s.b[2996])) && (!s.b[2997])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2991] && (!s.b[2994])) && s.b[2995]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[2998] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[2998] = if s.b[2998] { 1.0 } else { 0.0 };

        if (((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && s.b[2998]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[2999] = ((-s.v[2011]) < 0.0);
        s.v[2999] = if s.b[2999] { 1.0 } else { 0.0 };

        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[2998])) && s.b[2999]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[2998])) && (!s.b[2999])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3000] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3000] = if s.b[3000] { 1.0 } else { 0.0 };

        if (((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && s.b[3000]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3001] = ((-s.v[2013]) < 0.0);
        s.v[3001] = if s.b[3001] { 1.0 } else { 0.0 };

        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[3000])) && s.b[3001]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) && (!s.b[3000])) && (!s.b[3001])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

    }

    pub(super) fn stamp_reactive_block_28(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2991] && (!s.b[2994])) && (!s.b[2995])) {
            s.store_add(2016, 2013, 2014);
        }

        s.b[3002] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[3002] = if s.b[3002] { 1.0 } else { 0.0 };

        if (s.b[2991] && s.b[3002]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
            s.store_mul_scaled_ad_rhs(1991, 1889, (-0.70710678), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
            s.store_mul_scaled_ad_rhs(1990, 1889, (-0.235702), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.b[3003] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[3003] = if s.b[3003] { 1.0 } else { 0.0 };

        if ((s.b[2991] && (!s.b[3002])) && s.b[3003]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3004] = ((-s.v[2016]) < 0.0);
        s.v[3004] = if s.b[3004] { 1.0 } else { 0.0 };

        if (((s.b[2991] && (!s.b[3002])) && (!s.b[3003])) && s.b[3004]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[2991] && (!s.b[3002])) && (!s.b[3003])) && (!s.b[3004])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[2991] && (!s.b[3002])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[3005] = (s.v[2016] > s.v[1933]);
        s.v[3005] = if s.b[3005] { 1.0 } else { 0.0 };

        if ((s.b[2991] && (!s.b[3002])) && s.b[3005]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[2991] && (!s.b[3002])) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if s.b[2991] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1973, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul(A::mul(s.ad_value(1973), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3006] = (s.v[0] == (-1.0));
        s.v[3006] = if s.b[3006] { 1.0 } else { 0.0 };

        if (s.b[2991] && s.b[3006]) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if s.b[2991] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
            s.store_mul_sub_ad_rhs(1955, 2019, s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027)));
        }

        s.b[3007] = (s.v[1] >= 6.0);
        s.v[3007] = if s.b[3007] { 1.0 } else { 0.0 };

        s.b[3008] = (s.v[1] == 9.0);
        s.v[3008] = if s.b[3008] { 1.0 } else { 0.0 };

        if (s.b[3007] && s.b[3008]) {
            let assign70290_ad_e94902: A = A::scale(A::add(A::add(A::sub(A::add(A::sub(A::sub(A::add(A::scale(s.ad_value(1974), 30.0), A::scale(s.ad_value(1972), 81480.0)), A::scale(s.ad_value(1971), 21825.0)), A::scale(s.ad_value(1976), 81060.0)), A::scale(s.ad_value(1977), 20265.0)), A::scale(s.ad_value(1969), 1455.0)), A::scale(s.ad_value(1975), 303975.0)), A::scale(s.ad_value(1970), 5820.0)), 2.6434745829918846e-5);
            s.store_sub_scaled_ad_lhs(1992, A::sub(assign70290_ad_e94902, A::scale(A::sub(A::scale(s.ad_value(1979), 6755.0), A::scale(s.ad_value(1978), 485.0)), 1.3217372914959423e-5)), 1973, (1455.0 / 181.0));
        }

        if (s.b[3007] && s.b[3008]) {
            let assign70300_ad_e94958: A = A::add(A::sub(A::sub(A::sub(A::sub(A::add(A::sub(A::add(A::scale(s.ad_value(1969), 50400.0), A::scale(s.ad_value(1975), 10530000.0)), A::scale(s.ad_value(1972), 2822400.0)), A::scale(s.ad_value(1971), 756000.0)), A::scale(s.ad_value(1970), 201600.0)), A::scale(s.ad_value(1978), 8400.0)), A::scale(s.ad_value(1979), 117000.0)), A::scale(s.ad_value(1976), 2808000.0)), A::scale(s.ad_value(1977), 702000.0));
            s.store_add_scaled_ad_lhs(1993, A::scale(A::sub(assign70300_ad_e94958, A::scale(s.ad_value(1974), 16614600.0)), 2.6434745829918846e-5), 1973, (50400.0 * 0.0055248618784530384));
        }

        if (s.b[3007] && (!s.b[3008])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[3007] {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1974), s.ad_value(1937)), 1890);
        }

        s.b[3009] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3009] = if s.b[3009] { 1.0 } else { 0.0 };

        if (s.b[3007] && s.b[3009]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[3010] = (s.v[2027] < (-s.v[1941]));
        s.v[3010] = if s.b[3010] { 1.0 } else { 0.0 };

        if ((s.b[3007] && (!s.b[3009])) && s.b[3010]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3011] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3011] = if s.b[3011] { 1.0 } else { 0.0 };

        if (((s.b[3007] && (!s.b[3009])) && s.b[3010]) && s.b[3011]) {
            s.store_exp(2005, 2015);
        }

        s.b[3012] = (s.v[2015] < 0.0);
        s.v[3012] = if s.b[3012] { 1.0 } else { 0.0 };

        if ((((s.b[3007] && (!s.b[3009])) && s.b[3010]) && (!s.b[3011])) && s.b[3012]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[3007] && (!s.b[3009])) && s.b[3010]) && (!s.b[3011])) && (!s.b[3012])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[3007] && (!s.b[3009])) && s.b[3010]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3013] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3013] = if s.b[3013] { 1.0 } else { 0.0 };

        if (((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && s.b[3013]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3014] = ((-s.v[2011]) < 0.0);
        s.v[3014] = if s.b[3014] { 1.0 } else { 0.0 };

        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3013])) && s.b[3014]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3013])) && (!s.b[3014])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3015] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3015] = if s.b[3015] { 1.0 } else { 0.0 };

        if (((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && s.b[3015]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3016] = ((-s.v[2013]) < 0.0);
        s.v[3016] = if s.b[3016] { 1.0 } else { 0.0 };

        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3015])) && s.b[3016]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) && (!s.b[3015])) && (!s.b[3016])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[3007] && (!s.b[3009])) && (!s.b[3010])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2016, 2013, 2014);
        }

        s.b[3017] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[3017] = if s.b[3017] { 1.0 } else { 0.0 };

        if (s.b[3007] && s.b[3017]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
            s.store_mul_scaled_ad_rhs(1991, 1889, (-0.70710678), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
            s.store_mul_scaled_ad_rhs(1990, 1889, (-0.235702), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.b[3018] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[3018] = if s.b[3018] { 1.0 } else { 0.0 };

        if ((s.b[3007] && (!s.b[3017])) && s.b[3018]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3019] = ((-s.v[2016]) < 0.0);
        s.v[3019] = if s.b[3019] { 1.0 } else { 0.0 };

        if (((s.b[3007] && (!s.b[3017])) && (!s.b[3018])) && s.b[3019]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[3007] && (!s.b[3017])) && (!s.b[3018])) && (!s.b[3019])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[3007] && (!s.b[3017])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[3020] = (s.v[2016] > s.v[1933]);
        s.v[3020] = if s.b[3020] { 1.0 } else { 0.0 };

        if ((s.b[3007] && (!s.b[3017])) && s.b[3020]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[3007] && (!s.b[3017])) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if s.b[3007] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1974, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul(A::mul(s.ad_value(1974), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3021] = (s.v[0] == (-1.0));
        s.v[3021] = if s.b[3021] { 1.0 } else { 0.0 };

        if (s.b[3007] && s.b[3021]) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if s.b[3007] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
            s.store_mul_sub_ad_rhs(1956, 2019, s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027)));
        }

        s.b[3022] = (s.v[1] >= 7.0);
        s.v[3022] = if s.b[3022] { 1.0 } else { 0.0 };

        s.b[3023] = (s.v[1] == 9.0);
        s.v[3023] = if s.b[3023] { 1.0 } else { 0.0 };

        if (s.b[3022] && s.b[3023]) {
            let assign71070_ad_e96169: A = A::add(A::add(A::sub(A::sub(A::add(A::add(A::add(A::sub(A::scale(s.ad_value(1974), (-304200.0)), A::scale(s.ad_value(1972), 21840.0)), A::scale(s.ad_value(1979), 12605.0)), A::scale(s.ad_value(1971), 5850.0)), A::scale(s.ad_value(1976), 302520.0)), A::scale(s.ad_value(1978), 65.0)), A::scale(s.ad_value(1977), 75630.0)), A::scale(s.ad_value(1969), 390.0)), A::scale(s.ad_value(1975), 420.0));
            s.store_add_scaled_ad_lhs(1992, A::scale(A::sub(assign71070_ad_e96169, A::scale(s.ad_value(1970), 1560.0)), 2.6434745829918846e-5), 1973, (390.0 / 181.0));
        }

        if (s.b[3022] && s.b[3023]) {
            let assign71080_ad_e96222: A = A::sub(A::add(A::add(A::add(A::add(A::sub(A::add(A::sub(A::scale(s.ad_value(1969), (-13500.0)), A::scale(s.ad_value(1975), 16601100.0)), A::scale(s.ad_value(1972), 756000.0)), A::scale(s.ad_value(1971), 202500.0)), A::scale(s.ad_value(1970), 54000.0)), A::scale(s.ad_value(1978), 2250.0)), A::scale(s.ad_value(1979), 436650.0)), A::scale(s.ad_value(1976), 10479600.0)), A::scale(s.ad_value(1977), 2619900.0));
            s.store_sub_scaled_ad_lhs(1993, A::scale(A::add(assign71080_ad_e96222, A::scale(s.ad_value(1974), 10530000.0)), 2.6434745829918846e-5), 1973, (13500.0 * 0.0055248618784530384));
        }

        if (s.b[3022] && (!s.b[3023])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[3022] {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1975), s.ad_value(1937)), 1890);
        }

        s.b[3024] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3024] = if s.b[3024] { 1.0 } else { 0.0 };

        if (s.b[3022] && s.b[3024]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[3025] = (s.v[2027] < (-s.v[1941]));
        s.v[3025] = if s.b[3025] { 1.0 } else { 0.0 };

        if ((s.b[3022] && (!s.b[3024])) && s.b[3025]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3026] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3026] = if s.b[3026] { 1.0 } else { 0.0 };

        if (((s.b[3022] && (!s.b[3024])) && s.b[3025]) && s.b[3026]) {
            s.store_exp(2005, 2015);
        }

        s.b[3027] = (s.v[2015] < 0.0);
        s.v[3027] = if s.b[3027] { 1.0 } else { 0.0 };

        if ((((s.b[3022] && (!s.b[3024])) && s.b[3025]) && (!s.b[3026])) && s.b[3027]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[3022] && (!s.b[3024])) && s.b[3025]) && (!s.b[3026])) && (!s.b[3027])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[3022] && (!s.b[3024])) && s.b[3025]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
        }

    }

    pub(super) fn stamp_reactive_block_29(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[3022] && (!s.b[3024])) && s.b[3025]) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3028] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3028] = if s.b[3028] { 1.0 } else { 0.0 };

        if (((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && s.b[3028]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3029] = ((-s.v[2011]) < 0.0);
        s.v[3029] = if s.b[3029] { 1.0 } else { 0.0 };

        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3028])) && s.b[3029]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3028])) && (!s.b[3029])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3030] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3030] = if s.b[3030] { 1.0 } else { 0.0 };

        if (((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && s.b[3030]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3031] = ((-s.v[2013]) < 0.0);
        s.v[3031] = if s.b[3031] { 1.0 } else { 0.0 };

        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3030])) && s.b[3031]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) && (!s.b[3030])) && (!s.b[3031])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[3022] && (!s.b[3024])) && (!s.b[3025])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2016, 2013, 2014);
        }

        s.b[3032] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[3032] = if s.b[3032] { 1.0 } else { 0.0 };

        if (s.b[3022] && s.b[3032]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
            s.store_mul_scaled_ad_rhs(1991, 1889, (-0.70710678), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
            s.store_mul_scaled_ad_rhs(1990, 1889, (-0.235702), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.b[3033] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[3033] = if s.b[3033] { 1.0 } else { 0.0 };

        if ((s.b[3022] && (!s.b[3032])) && s.b[3033]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3034] = ((-s.v[2016]) < 0.0);
        s.v[3034] = if s.b[3034] { 1.0 } else { 0.0 };

        if (((s.b[3022] && (!s.b[3032])) && (!s.b[3033])) && s.b[3034]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[3022] && (!s.b[3032])) && (!s.b[3033])) && (!s.b[3034])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[3022] && (!s.b[3032])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[3035] = (s.v[2016] > s.v[1933]);
        s.v[3035] = if s.b[3035] { 1.0 } else { 0.0 };

        if ((s.b[3022] && (!s.b[3032])) && s.b[3035]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[3022] && (!s.b[3032])) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if s.b[3022] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1975, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul(A::mul(s.ad_value(1975), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3036] = (s.v[0] == (-1.0));
        s.v[3036] = if s.b[3036] { 1.0 } else { 0.0 };

        if (s.b[3022] && s.b[3036]) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if s.b[3022] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
            s.store_mul_sub_ad_rhs(1957, 2019, s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027)));
        }

        s.b[3037] = (s.v[1] >= 8.0);
        s.v[3037] = if s.b[3037] { 1.0 } else { 0.0 };

        s.b[3038] = (s.v[1] == 9.0);
        s.v[3038] = if s.b[3038] { 1.0 } else { 0.0 };

        if (s.b[3037] && s.b[3038]) {
            let assign71850_ad_e97430: A = A::scale(A::add(A::sub(A::sub(A::add(A::add(A::sub(A::add(A::scale(s.ad_value(1974), 81900.0), A::scale(s.ad_value(1972), 5880.0)), A::scale(s.ad_value(1971), 1575.0)), A::scale(s.ad_value(1976), 5850.0)), A::scale(s.ad_value(1977), 282255.0)), A::scale(s.ad_value(1969), 105.0)), A::scale(s.ad_value(1975), 305655.0)), A::scale(s.ad_value(1970), 420.0)), 2.6434745829918846e-5);
            s.store_sub_scaled_ad_lhs(1992, A::add(assign71850_ad_e97430, A::scale(A::sub(A::scale(s.ad_value(1978), 35.0), A::scale(s.ad_value(1979), 94085.0)), 1.3217372914959423e-5)), 1973, (105.0 / 181.0));
        }

        if (s.b[3037] && s.b[3038]) {
            let assign71860_ad_e97486: A = A::add(A::sub(A::sub(A::sub(A::sub(A::add(A::sub(A::add(A::scale(s.ad_value(1969), 3600.0), A::scale(s.ad_value(1975), 10479600.0)), A::scale(s.ad_value(1972), 201600.0)), A::scale(s.ad_value(1971), 54000.0)), A::scale(s.ad_value(1970), 14400.0)), A::scale(s.ad_value(1978), 600.0)), A::scale(s.ad_value(1979), 1629600.0)), A::scale(s.ad_value(1976), 16413000.0)), A::scale(s.ad_value(1977), 9777600.0));
            s.store_add_scaled_ad_lhs(1993, A::scale(A::sub(assign71860_ad_e97486, A::scale(s.ad_value(1974), 2808000.0)), 2.6434745829918846e-5), 1973, (3600.0 * 0.0055248618784530384));
        }

        if (s.b[3037] && (!s.b[3038])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[3037] {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1976), s.ad_value(1937)), 1890);
        }

        s.b[3039] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3039] = if s.b[3039] { 1.0 } else { 0.0 };

        if (s.b[3037] && s.b[3039]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[3040] = (s.v[2027] < (-s.v[1941]));
        s.v[3040] = if s.b[3040] { 1.0 } else { 0.0 };

        if ((s.b[3037] && (!s.b[3039])) && s.b[3040]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3041] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3041] = if s.b[3041] { 1.0 } else { 0.0 };

        if (((s.b[3037] && (!s.b[3039])) && s.b[3040]) && s.b[3041]) {
            s.store_exp(2005, 2015);
        }

        s.b[3042] = (s.v[2015] < 0.0);
        s.v[3042] = if s.b[3042] { 1.0 } else { 0.0 };

        if ((((s.b[3037] && (!s.b[3039])) && s.b[3040]) && (!s.b[3041])) && s.b[3042]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[3037] && (!s.b[3039])) && s.b[3040]) && (!s.b[3041])) && (!s.b[3042])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[3037] && (!s.b[3039])) && s.b[3040]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3043] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3043] = if s.b[3043] { 1.0 } else { 0.0 };

        if (((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && s.b[3043]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3044] = ((-s.v[2011]) < 0.0);
        s.v[3044] = if s.b[3044] { 1.0 } else { 0.0 };

        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3043])) && s.b[3044]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3043])) && (!s.b[3044])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3045] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3045] = if s.b[3045] { 1.0 } else { 0.0 };

        if (((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && s.b[3045]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3046] = ((-s.v[2013]) < 0.0);
        s.v[3046] = if s.b[3046] { 1.0 } else { 0.0 };

        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3045])) && s.b[3046]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) && (!s.b[3045])) && (!s.b[3046])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[3037] && (!s.b[3039])) && (!s.b[3040])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2016, 2013, 2014);
        }

        s.b[3047] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[3047] = if s.b[3047] { 1.0 } else { 0.0 };

        if (s.b[3037] && s.b[3047]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
            s.store_mul_scaled_ad_rhs(1991, 1889, (-0.70710678), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
            s.store_mul_scaled_ad_rhs(1990, 1889, (-0.235702), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.b[3048] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[3048] = if s.b[3048] { 1.0 } else { 0.0 };

        if ((s.b[3037] && (!s.b[3047])) && s.b[3048]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3049] = ((-s.v[2016]) < 0.0);
        s.v[3049] = if s.b[3049] { 1.0 } else { 0.0 };

        if (((s.b[3037] && (!s.b[3047])) && (!s.b[3048])) && s.b[3049]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[3037] && (!s.b[3047])) && (!s.b[3048])) && (!s.b[3049])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[3037] && (!s.b[3047])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[3050] = (s.v[2016] > s.v[1933]);
        s.v[3050] = if s.b[3050] { 1.0 } else { 0.0 };

        if ((s.b[3037] && (!s.b[3047])) && s.b[3050]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[3037] && (!s.b[3047])) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if s.b[3037] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1976, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul(A::mul(s.ad_value(1976), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3051] = (s.v[0] == (-1.0));
        s.v[3051] = if s.b[3051] { 1.0 } else { 0.0 };

        if (s.b[3037] && s.b[3051]) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if s.b[3037] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
            s.store_mul_sub_ad_rhs(1958, 2019, s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027)));
        }

        s.b[3052] = (s.v[1] >= 9.0);
        s.v[3052] = if s.b[3052] { 1.0 } else { 0.0 };

        s.b[3053] = (s.v[1] == 9.0);
        s.v[3053] = if s.b[3053] { 1.0 } else { 0.0 };

        if (s.b[3052] && s.b[3053]) {
            let assign72630_ad_e98697: A = A::add(A::add(A::add(A::sub(A::sub(A::add(A::add(A::sub(A::scale(s.ad_value(1974), (-23400.0)), A::scale(s.ad_value(1972), 1680.0)), A::scale(s.ad_value(1979), 175565.0)), A::scale(s.ad_value(1971), 450.0)), A::scale(s.ad_value(1976), 325920.0)), A::scale(s.ad_value(1978), 5.0)), A::scale(s.ad_value(1977), 81480.0)), A::scale(s.ad_value(1969), 30.0)), A::scale(s.ad_value(1975), 87330.0));
            s.store_add_scaled_ad_lhs(1992, A::scale(A::sub(assign72630_ad_e98697, A::scale(s.ad_value(1970), 120.0)), 2.6434745829918846e-5), 1973, (30.0 * 0.0055248618784530384));
        }

        if (s.b[3052] && s.b[3053]) {
            let assign72640_ad_e98750: A = A::sub(A::add(A::add(A::add(A::add(A::sub(A::add(A::sub(A::scale(s.ad_value(1969), (-900.0)), A::scale(s.ad_value(1975), 2619900.0)), A::scale(s.ad_value(1972), 50400.0)), A::scale(s.ad_value(1971), 13500.0)), A::scale(s.ad_value(1970), 3600.0)), A::scale(s.ad_value(1978), 150.0)), A::scale(s.ad_value(1979), 6081750.0)), A::scale(s.ad_value(1976), 9777600.0)), A::scale(s.ad_value(1977), 13793100.0));
            s.store_sub_scaled_ad_lhs(1993, A::scale(A::add(assign72640_ad_e98750, A::scale(s.ad_value(1974), 702000.0)), 2.6434745829918846e-5), 1973, (900.0 * 0.0055248618784530384));
        }

    }

    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[3052] && (!s.b[3053])) {
            s.store_scalar(1992, 0.0);
            s.store_scalar(1993, 0.0);
        }

        if s.b[3052] {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1977), s.ad_value(1937)), 1890);
        }

        s.b[3054] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3054] = if s.b[3054] { 1.0 } else { 0.0 };

        if (s.b[3052] && s.b[3054]) {
            s.store_div(2016, 2027, 1940);
        }

        s.b[3055] = (s.v[2027] < (-s.v[1941]));
        s.v[3055] = if s.b[3055] { 1.0 } else { 0.0 };

        if ((s.b[3052] && (!s.b[3054])) && s.b[3055]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3056] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3056] = if s.b[3056] { 1.0 } else { 0.0 };

        if (((s.b[3052] && (!s.b[3054])) && s.b[3055]) && s.b[3056]) {
            s.store_exp(2005, 2015);
        }

        s.b[3057] = (s.v[2015] < 0.0);
        s.v[3057] = if s.b[3057] { 1.0 } else { 0.0 };

        if ((((s.b[3052] && (!s.b[3054])) && s.b[3055]) && (!s.b[3056])) && s.b[3057]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[3052] && (!s.b[3054])) && s.b[3055]) && (!s.b[3056])) && (!s.b[3057])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[3052] && (!s.b[3054])) && s.b[3055]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3058] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3058] = if s.b[3058] { 1.0 } else { 0.0 };

        if (((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && s.b[3058]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3059] = ((-s.v[2011]) < 0.0);
        s.v[3059] = if s.b[3059] { 1.0 } else { 0.0 };

        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3058])) && s.b[3059]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3058])) && (!s.b[3059])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3060] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3060] = if s.b[3060] { 1.0 } else { 0.0 };

        if (((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && s.b[3060]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3061] = ((-s.v[2013]) < 0.0);
        s.v[3061] = if s.b[3061] { 1.0 } else { 0.0 };

        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3060])) && s.b[3061]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) && (!s.b[3060])) && (!s.b[3061])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((s.b[3052] && (!s.b[3054])) && (!s.b[3055])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2016, 2013, 2014);
        }

        s.b[3062] = (((s.v[2016]) as f64).abs() <= s.v[1933]);
        s.v[3062] = if s.b[3062] { 1.0 } else { 0.0 };

        if (s.b[3052] && s.b[3062]) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
            s.store_mul_scaled_ad_rhs(1991, 1889, (-0.70710678), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
            s.store_mul_scaled_ad_rhs(1990, 1889, (-0.235702), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.b[3063] = ((((-s.v[2016])) as f64).abs() < 230.25850929940458);
        s.v[3063] = if s.b[3063] { 1.0 } else { 0.0 };

        if ((s.b[3052] && (!s.b[3062])) && s.b[3063]) {
            s.store_exp_neg_input(2027, 2016);
        }

        s.b[3064] = ((-s.v[2016]) < 0.0);
        s.v[3064] = if s.b[3064] { 1.0 } else { 0.0 };

        if (((s.b[3052] && (!s.b[3062])) && (!s.b[3063])) && s.b[3064]) {
            s.store_div_from_scalar_offset_ad(2027, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((s.b[3052] && (!s.b[3062])) && (!s.b[3063])) && (!s.b[3064])) {
            s.store_scaled_offset_ad(2027, A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (s.b[3052] && (!s.b[3062])) {
            s.store_mul_sqrt_ad_rhs(1996, 1889, A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0)));
        }

        s.b[3065] = (s.v[2016] > s.v[1933]);
        s.v[3065] = if s.b[3065] { 1.0 } else { 0.0 };

        if ((s.b[3052] && (!s.b[3062])) && s.b[3065]) {
            s.store_neg(1996, 1996);
        }

        if (s.b[3052] && (!s.b[3062])) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if s.b[3052] {
            s.store_sub(1988, 1937, 1991);
            s.store_div_from_scalar(1989, 1.0, 1988);
            s.store_offset_mul(1987, 1977, 1989, (-1.0));
            s.store_mul_sub_from_scalar_ad_lhs(1986, 1.0, A::mul(A::mul(A::mul(s.ad_value(1977), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989)), 1989);
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
            s.store_mul_ad_product_lhs(1994, A::square(s.ad_value(1992)), s.ad_value(1989), 1989);
            s.store_mul(1985, 2018, 1994);
        }

        s.b[3066] = (s.v[0] == (-1.0));
        s.v[3066] = if s.b[3066] { 1.0 } else { 0.0 };

        if (s.b[3052] && s.b[3066]) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if s.b[3052] {
            s.store_sqrt_offset_scaled_input(2027, 1985, 2.0, 1.0);
            s.store_div_from_scalar_offset_input(2019, 2.0, 2027, 1.0);
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
            s.store_mul_sub_ad_rhs(1959, 2019, s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027)));
        }

        s.v[1981] = 0.0;

        s.v[1982] = 0.0;

        s.v[1980] = 0.0;

        s.b[3067] = (s.v[1] != 0.0);
        s.v[3067] = if s.b[3067] { 1.0 } else { 0.0 };

        s.b[3068] = (s.v[1] == 1.0);
        s.v[3068] = if s.b[3068] { 1.0 } else { 0.0 };

        if (s.b[3067] && s.b[3068]) {
            s.store_scaled_add_ad_lhs(1981, A::add(A::scale(s.ad_value(1978), 17.0), A::scale(s.ad_value(1969), 30.0)), 1979, 0.010416666666666666);
            s.store_scale_ad(1982, A::add(A::add(s.ad_value(1978), A::scale(s.ad_value(1969), 30.0)), A::scale(s.ad_value(1979), 17.0)), 0.010416666666666666);
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1969), s.ad_value(1937)), 1890);
        }

        s.b[3069] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3069] = if s.b[3069] { 1.0 } else { 0.0 };

        if ((s.b[3067] && s.b[3068]) && s.b[3069]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3070] = (s.v[2027] < (-s.v[1941]));
        s.v[3070] = if s.b[3070] { 1.0 } else { 0.0 };

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3071] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3071] = if s.b[3071] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) && s.b[3071]) {
            s.store_exp(2005, 2015);
        }

        s.b[3072] = (s.v[2015] < 0.0);
        s.v[3072] = if s.b[3072] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) && (!s.b[3071])) && s.b[3072]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) && (!s.b[3071])) && (!s.b[3072])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && s.b[3070]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2028, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3073] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3073] = if s.b[3073] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && s.b[3073]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3074] = ((-s.v[2011]) < 0.0);
        s.v[3074] = if s.b[3074] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3073])) && s.b[3074]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3073])) && (!s.b[3074])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3075] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3075] = if s.b[3075] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && s.b[3075]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3076] = ((-s.v[2013]) < 0.0);
        s.v[3076] = if s.b[3076] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3075])) && s.b[3076]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) && (!s.b[3075])) && (!s.b[3076])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((s.b[3067] && s.b[3068]) && (!s.b[3069])) && (!s.b[3070])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2028, 2013, 2014);
        }

        if (s.b[3067] && s.b[3068]) {
            s.store_sub_ad_rhs(1980, 1890, A::scale(A::add(A::add(s.ad_value(1983), A::scale(s.ad_value(2028), 4.0)), s.ad_value(1984)), 0.16666666666666666));
        }

        s.b[3077] = (s.v[1] == 2.0);
        s.v[3077] = if s.b[3077] { 1.0 } else { 0.0 };

        if ((s.b[3067] && (!s.b[3068])) && s.b[3077]) {
            s.store_scaled_add_ad_lhs(1981, A::add(A::add(A::scale(s.ad_value(1978), 11.0), A::scale(s.ad_value(1969), 24.0)), A::scale(s.ad_value(1970), 9.0)), 1979, 0.011111111111111112);
            s.store_scaled_add_ad_lhs(1982, A::add(A::add(A::scale(s.ad_value(1979), 11.0), A::scale(s.ad_value(1970), 24.0)), A::scale(s.ad_value(1969), 9.0)), 1978, 0.011111111111111112);
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1969), s.ad_value(1937)), 1890);
        }

        s.b[3078] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3078] = if s.b[3078] { 1.0 } else { 0.0 };

        if (((s.b[3067] && (!s.b[3068])) && s.b[3077]) && s.b[3078]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3079] = (s.v[2027] < (-s.v[1941]));
        s.v[3079] = if s.b[3079] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
        }

    }

    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
    ) {
        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) {
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3080] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3080] = if s.b[3080] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) && s.b[3080]) {
            s.store_exp(2005, 2015);
        }

        s.b[3081] = (s.v[2015] < 0.0);
        s.v[3081] = if s.b[3081] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) && (!s.b[3080])) && s.b[3081]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) && (!s.b[3080])) && (!s.b[3081])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && s.b[3079]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2028, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3082] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3082] = if s.b[3082] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && s.b[3082]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3083] = ((-s.v[2011]) < 0.0);
        s.v[3083] = if s.b[3083] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && (!s.b[3082])) && s.b[3083]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && (!s.b[3082])) && (!s.b[3083])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3084] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3084] = if s.b[3084] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && s.b[3084]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3085] = ((-s.v[2013]) < 0.0);
        s.v[3085] = if s.b[3085] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && (!s.b[3084])) && s.b[3085]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) && (!s.b[3084])) && (!s.b[3085])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3078])) && (!s.b[3079])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2028, 2013, 2014);
        }

        if ((s.b[3067] && (!s.b[3068])) && s.b[3077]) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1970), s.ad_value(1937)), 1890);
        }

        s.b[3086] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3086] = if s.b[3086] { 1.0 } else { 0.0 };

        if (((s.b[3067] && (!s.b[3068])) && s.b[3077]) && s.b[3086]) {
            s.store_div(2029, 2027, 1940);
        }

        s.b[3087] = (s.v[2027] < (-s.v[1941]));
        s.v[3087] = if s.b[3087] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3088] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3088] = if s.b[3088] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) && s.b[3088]) {
            s.store_exp(2005, 2015);
        }

        s.b[3089] = (s.v[2015] < 0.0);
        s.v[3089] = if s.b[3089] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) && (!s.b[3088])) && s.b[3089]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) && (!s.b[3088])) && (!s.b[3089])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && s.b[3087]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2029, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3090] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3090] = if s.b[3090] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && s.b[3090]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3091] = ((-s.v[2011]) < 0.0);
        s.v[3091] = if s.b[3091] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && (!s.b[3090])) && s.b[3091]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && (!s.b[3090])) && (!s.b[3091])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3092] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3092] = if s.b[3092] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && s.b[3092]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3093] = ((-s.v[2013]) < 0.0);
        s.v[3093] = if s.b[3093] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && (!s.b[3092])) && s.b[3093]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) && (!s.b[3092])) && (!s.b[3093])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((s.b[3067] && (!s.b[3068])) && s.b[3077]) && (!s.b[3086])) && (!s.b[3087])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2029, 2013, 2014);
        }

        if ((s.b[3067] && (!s.b[3068])) && s.b[3077]) {
            s.store_sub_ad_rhs(1980, 1890, A::scale(A::add(A::add(s.ad_value(1983), A::scale(A::add(s.ad_value(2028), s.ad_value(2029)), 3.0)), s.ad_value(1984)), 0.125));
        }

        s.b[3094] = (s.v[1] == 3.0);
        s.v[3094] = if s.b[3094] { 1.0 } else { 0.0 };

        if (((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) {
            s.store_scale_ad(1981, A::add(A::add(A::add(A::add(A::scale(s.ad_value(1978), 251.0), A::scale(s.ad_value(1969), 594.0)), A::scale(s.ad_value(1970), 312.0)), A::scale(s.ad_value(1971), 174.0)), A::scale(s.ad_value(1979), 13.0)), 0.0003720238095238095);
            s.store_scale_ad(1982, A::add(A::add(A::add(A::add(A::scale(s.ad_value(1979), 251.0), A::scale(s.ad_value(1971), 594.0)), A::scale(s.ad_value(1970), 312.0)), A::scale(s.ad_value(1969), 174.0)), A::scale(s.ad_value(1978), 13.0)), 0.0003720238095238095);
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1969), s.ad_value(1937)), 1890);
        }

        s.b[3095] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3095] = if s.b[3095] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && s.b[3095]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3096] = (s.v[2027] < (-s.v[1941]));
        s.v[3096] = if s.b[3096] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3097] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3097] = if s.b[3097] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) && s.b[3097]) {
            s.store_exp(2005, 2015);
        }

        s.b[3098] = (s.v[2015] < 0.0);
        s.v[3098] = if s.b[3098] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) && (!s.b[3097])) && s.b[3098]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) && (!s.b[3097])) && (!s.b[3098])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && s.b[3096]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2028, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3099] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3099] = if s.b[3099] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && s.b[3099]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3100] = ((-s.v[2011]) < 0.0);
        s.v[3100] = if s.b[3100] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && (!s.b[3099])) && s.b[3100]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && (!s.b[3099])) && (!s.b[3100])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3101] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3101] = if s.b[3101] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && s.b[3101]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3102] = ((-s.v[2013]) < 0.0);
        s.v[3102] = if s.b[3102] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && (!s.b[3101])) && s.b[3102]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) && (!s.b[3101])) && (!s.b[3102])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3095])) && (!s.b[3096])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2028, 2013, 2014);
        }

    }

    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1970), s.ad_value(1937)), 1890);
        }

        s.b[3103] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3103] = if s.b[3103] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && s.b[3103]) {
            s.store_div(2029, 2027, 1940);
        }

        s.b[3104] = (s.v[2027] < (-s.v[1941]));
        s.v[3104] = if s.b[3104] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3105] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3105] = if s.b[3105] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) && s.b[3105]) {
            s.store_exp(2005, 2015);
        }

        s.b[3106] = (s.v[2015] < 0.0);
        s.v[3106] = if s.b[3106] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) && (!s.b[3105])) && s.b[3106]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) && (!s.b[3105])) && (!s.b[3106])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && s.b[3104]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2029, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3107] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3107] = if s.b[3107] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && s.b[3107]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3108] = ((-s.v[2011]) < 0.0);
        s.v[3108] = if s.b[3108] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && (!s.b[3107])) && s.b[3108]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && (!s.b[3107])) && (!s.b[3108])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3109] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3109] = if s.b[3109] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && s.b[3109]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3110] = ((-s.v[2013]) < 0.0);
        s.v[3110] = if s.b[3110] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && (!s.b[3109])) && s.b[3110]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) && (!s.b[3109])) && (!s.b[3110])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3103])) && (!s.b[3104])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2029, 2013, 2014);
        }

        if (((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1971), s.ad_value(1937)), 1890);
        }

        s.b[3111] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3111] = if s.b[3111] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && s.b[3111]) {
            s.store_div(2020, 2027, 1940);
        }

        s.b[3112] = (s.v[2027] < (-s.v[1941]));
        s.v[3112] = if s.b[3112] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3113] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3113] = if s.b[3113] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) && s.b[3113]) {
            s.store_exp(2005, 2015);
        }

        s.b[3114] = (s.v[2015] < 0.0);
        s.v[3114] = if s.b[3114] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) && (!s.b[3113])) && s.b[3114]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) && (!s.b[3113])) && (!s.b[3114])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && s.b[3112]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2020, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3115] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3115] = if s.b[3115] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && s.b[3115]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3116] = ((-s.v[2011]) < 0.0);
        s.v[3116] = if s.b[3116] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && (!s.b[3115])) && s.b[3116]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && (!s.b[3115])) && (!s.b[3116])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3117] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3117] = if s.b[3117] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && s.b[3117]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3118] = ((-s.v[2013]) < 0.0);
        s.v[3118] = if s.b[3118] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && (!s.b[3117])) && s.b[3118]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) && (!s.b[3117])) && (!s.b[3118])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) && (!s.b[3111])) && (!s.b[3112])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2020, 2013, 2014);
        }

        if (((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && s.b[3094]) {
            s.store_sub_ad_rhs(1980, 1890, A::scale(A::add(A::add(A::add(A::add(s.ad_value(1983), A::scale(s.ad_value(2028), 4.0)), A::scale(s.ad_value(2029), 2.0)), A::scale(s.ad_value(2020), 4.0)), s.ad_value(1984)), 0.08333333333333333));
        }

        s.b[3119] = (s.v[1] == 5.0);
        s.v[3119] = if s.b[3119] { 1.0 } else { 0.0 };

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_ad(1981, A::scale(A::add(A::scale(s.ad_value(1978), 1187.0), A::scale(s.ad_value(1979), 43.0)), 5.341880341880342e-5), A::scale(A::add(A::add(A::add(A::add(A::scale(s.ad_value(1969), 503.0), A::scale(s.ad_value(1972), 172.0)), A::scale(s.ad_value(1973), 87.0)), A::scale(s.ad_value(1971), 265.0)), A::scale(s.ad_value(1970), 328.0)), 0.0003205128205128205));
            s.store_add_ad(1982, A::scale(A::add(A::scale(s.ad_value(1979), 1187.0), A::scale(s.ad_value(1978), 43.0)), 5.341880341880342e-5), A::scale(A::add(A::add(A::add(A::add(A::scale(s.ad_value(1973), 503.0), A::scale(s.ad_value(1970), 172.0)), A::scale(s.ad_value(1969), 87.0)), A::scale(s.ad_value(1971), 265.0)), A::scale(s.ad_value(1972), 328.0)), 0.0003205128205128205));
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1969), s.ad_value(1937)), 1890);
        }

        s.b[3120] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3120] = if s.b[3120] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3120]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3121] = (s.v[2027] < (-s.v[1941]));
        s.v[3121] = if s.b[3121] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3122] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3122] = if s.b[3122] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) && s.b[3122]) {
            s.store_exp(2005, 2015);
        }

        s.b[3123] = (s.v[2015] < 0.0);
        s.v[3123] = if s.b[3123] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) && (!s.b[3122])) && s.b[3123]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) && (!s.b[3122])) && (!s.b[3123])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && s.b[3121]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2028, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3124] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3124] = if s.b[3124] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && s.b[3124]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3125] = ((-s.v[2011]) < 0.0);
        s.v[3125] = if s.b[3125] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && (!s.b[3124])) && s.b[3125]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && (!s.b[3124])) && (!s.b[3125])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3126] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3126] = if s.b[3126] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && s.b[3126]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3127] = ((-s.v[2013]) < 0.0);
        s.v[3127] = if s.b[3127] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && (!s.b[3126])) && s.b[3127]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) && (!s.b[3126])) && (!s.b[3127])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
        }

    }

    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3120])) && (!s.b[3121])) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2028, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1970), s.ad_value(1937)), 1890);
        }

        s.b[3128] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3128] = if s.b[3128] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3128]) {
            s.store_div(2029, 2027, 1940);
        }

        s.b[3129] = (s.v[2027] < (-s.v[1941]));
        s.v[3129] = if s.b[3129] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3130] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3130] = if s.b[3130] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) && s.b[3130]) {
            s.store_exp(2005, 2015);
        }

        s.b[3131] = (s.v[2015] < 0.0);
        s.v[3131] = if s.b[3131] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) && (!s.b[3130])) && s.b[3131]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) && (!s.b[3130])) && (!s.b[3131])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && s.b[3129]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2029, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3132] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3132] = if s.b[3132] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && s.b[3132]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3133] = ((-s.v[2011]) < 0.0);
        s.v[3133] = if s.b[3133] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && (!s.b[3132])) && s.b[3133]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && (!s.b[3132])) && (!s.b[3133])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3134] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3134] = if s.b[3134] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && s.b[3134]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3135] = ((-s.v[2013]) < 0.0);
        s.v[3135] = if s.b[3135] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && (!s.b[3134])) && s.b[3135]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) && (!s.b[3134])) && (!s.b[3135])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3128])) && (!s.b[3129])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2029, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1971), s.ad_value(1937)), 1890);
        }

        s.b[3136] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3136] = if s.b[3136] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3136]) {
            s.store_div(2020, 2027, 1940);
        }

        s.b[3137] = (s.v[2027] < (-s.v[1941]));
        s.v[3137] = if s.b[3137] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3138] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3138] = if s.b[3138] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) && s.b[3138]) {
            s.store_exp(2005, 2015);
        }

        s.b[3139] = (s.v[2015] < 0.0);
        s.v[3139] = if s.b[3139] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) && (!s.b[3138])) && s.b[3139]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) && (!s.b[3138])) && (!s.b[3139])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && s.b[3137]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2020, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3140] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3140] = if s.b[3140] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && s.b[3140]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3141] = ((-s.v[2011]) < 0.0);
        s.v[3141] = if s.b[3141] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && (!s.b[3140])) && s.b[3141]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && (!s.b[3140])) && (!s.b[3141])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3142] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3142] = if s.b[3142] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && s.b[3142]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3143] = ((-s.v[2013]) < 0.0);
        s.v[3143] = if s.b[3143] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && (!s.b[3142])) && s.b[3143]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) && (!s.b[3142])) && (!s.b[3143])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3136])) && (!s.b[3137])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2020, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1972), s.ad_value(1937)), 1890);
        }

        s.b[3144] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3144] = if s.b[3144] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3144]) {
            s.store_div(2021, 2027, 1940);
        }

        s.b[3145] = (s.v[2027] < (-s.v[1941]));
        s.v[3145] = if s.b[3145] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3146] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3146] = if s.b[3146] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) && s.b[3146]) {
            s.store_exp(2005, 2015);
        }

        s.b[3147] = (s.v[2015] < 0.0);
        s.v[3147] = if s.b[3147] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) && (!s.b[3146])) && s.b[3147]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) && (!s.b[3146])) && (!s.b[3147])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && s.b[3145]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2021, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3148] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3148] = if s.b[3148] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && s.b[3148]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3149] = ((-s.v[2011]) < 0.0);
        s.v[3149] = if s.b[3149] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3148])) && s.b[3149]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3148])) && (!s.b[3149])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3150] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3150] = if s.b[3150] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && s.b[3150]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3151] = ((-s.v[2013]) < 0.0);
        s.v[3151] = if s.b[3151] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3150])) && s.b[3151]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) && (!s.b[3150])) && (!s.b[3151])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

    }

    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
    ) {
        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3144])) && (!s.b[3145])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2021, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1973), s.ad_value(1937)), 1890);
        }

        s.b[3152] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3152] = if s.b[3152] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && s.b[3152]) {
            s.store_div(2022, 2027, 1940);
        }

        s.b[3153] = (s.v[2027] < (-s.v[1941]));
        s.v[3153] = if s.b[3153] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3154] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3154] = if s.b[3154] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) && s.b[3154]) {
            s.store_exp(2005, 2015);
        }

        s.b[3155] = (s.v[2015] < 0.0);
        s.v[3155] = if s.b[3155] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) && (!s.b[3154])) && s.b[3155]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) && (!s.b[3154])) && (!s.b[3155])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && s.b[3153]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2022, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3156] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3156] = if s.b[3156] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && s.b[3156]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3157] = ((-s.v[2011]) < 0.0);
        s.v[3157] = if s.b[3157] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && (!s.b[3156])) && s.b[3157]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && (!s.b[3156])) && (!s.b[3157])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3158] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3158] = if s.b[3158] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && s.b[3158]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3159] = ((-s.v[2013]) < 0.0);
        s.v[3159] = if s.b[3159] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && (!s.b[3158])) && s.b[3159]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) && (!s.b[3158])) && (!s.b[3159])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) && (!s.b[3152])) && (!s.b[3153])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2022, 2013, 2014);
        }

        if ((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && s.b[3119]) {
            s.store_sub_ad_rhs(1980, 1890, A::scale(A::add(A::add(A::add(s.ad_value(1983), A::scale(A::add(A::add(s.ad_value(2028), s.ad_value(2020)), s.ad_value(2022)), 4.0)), A::scale(A::add(s.ad_value(2029), s.ad_value(2021)), 2.0)), s.ad_value(1984)), 0.05555555555555555));
        }

        s.b[3160] = (s.v[1] == 9.0);
        s.v[3160] = if s.b[3160] { 1.0 } else { 0.0 };

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            let assign78550_ad_e112431: A = A::add(A::add(A::scale(A::add(A::scale(s.ad_value(1976), 75653.0), A::scale(s.ad_value(1972), 225999.0)), 2.6434745829918845e-7), A::scale(A::add(A::add(A::add(A::scale(s.ad_value(1977), 151321.0), A::scale(s.ad_value(1975), 454023.0)), A::scale(s.ad_value(1971), 1073767.0)), A::scale(s.ad_value(1969), 1564569.0)), 6.608686457479711e-8)), A::scale(s.ad_value(1974), (75623.0 * 5.286949165983769e-7)));
            s.store_add_ad(1981, A::add(A::add(assign78550_ad_e112431, A::scale(s.ad_value(1973), (145.0 * 0.0003453038674033149))), A::scale(s.ad_value(1970), (72263.0 * 1.0573898331967538e-6))), A::scale(A::add(A::scale(s.ad_value(1978), 3504517.0), A::scale(s.ad_value(1979), 75653.0)), 1.1014477429132853e-8));
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            let assign78560_ad_e112503: A = A::add(A::add(A::scale(A::add(A::scale(s.ad_value(1970), 75653.0), A::scale(s.ad_value(1974), 225999.0)), 2.6434745829918845e-7), A::scale(A::add(A::add(A::add(A::scale(s.ad_value(1969), 151321.0), A::scale(s.ad_value(1971), 454023.0)), A::scale(s.ad_value(1975), 1073767.0)), A::scale(s.ad_value(1977), 1564569.0)), 6.608686457479711e-8)), A::scale(s.ad_value(1972), (75623.0 * 5.286949165983769e-7)));
            s.store_add_ad(1982, A::add(A::add(assign78560_ad_e112503, A::scale(s.ad_value(1973), (145.0 * 0.0003453038674033149))), A::scale(s.ad_value(1976), (72263.0 * 1.0573898331967538e-6))), A::scale(A::add(A::scale(s.ad_value(1979), 3504517.0), A::scale(s.ad_value(1978), 75653.0)), 1.1014477429132853e-8));
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1969), s.ad_value(1937)), 1890);
        }

        s.b[3161] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3161] = if s.b[3161] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3161]) {
            s.store_div(2028, 2027, 1940);
        }

        s.b[3162] = (s.v[2027] < (-s.v[1941]));
        s.v[3162] = if s.b[3162] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3163] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3163] = if s.b[3163] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) && s.b[3163]) {
            s.store_exp(2005, 2015);
        }

        s.b[3164] = (s.v[2015] < 0.0);
        s.v[3164] = if s.b[3164] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) && (!s.b[3163])) && s.b[3164]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) && (!s.b[3163])) && (!s.b[3164])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && s.b[3162]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2028, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3165] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3165] = if s.b[3165] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && s.b[3165]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3166] = ((-s.v[2011]) < 0.0);
        s.v[3166] = if s.b[3166] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && (!s.b[3165])) && s.b[3166]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && (!s.b[3165])) && (!s.b[3166])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3167] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3167] = if s.b[3167] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && s.b[3167]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3168] = ((-s.v[2013]) < 0.0);
        s.v[3168] = if s.b[3168] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && (!s.b[3167])) && s.b[3168]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) && (!s.b[3167])) && (!s.b[3168])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3161])) && (!s.b[3162])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2028, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1970), s.ad_value(1937)), 1890);
        }

        s.b[3169] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3169] = if s.b[3169] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3169]) {
            s.store_div(2029, 2027, 1940);
        }

        s.b[3170] = (s.v[2027] < (-s.v[1941]));
        s.v[3170] = if s.b[3170] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3171] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3171] = if s.b[3171] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) && s.b[3171]) {
            s.store_exp(2005, 2015);
        }

        s.b[3172] = (s.v[2015] < 0.0);
        s.v[3172] = if s.b[3172] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) && (!s.b[3171])) && s.b[3172]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) && (!s.b[3171])) && (!s.b[3172])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && s.b[3170]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2029, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3173] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3173] = if s.b[3173] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && s.b[3173]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3174] = ((-s.v[2011]) < 0.0);
        s.v[3174] = if s.b[3174] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && (!s.b[3173])) && s.b[3174]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && (!s.b[3173])) && (!s.b[3174])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
        }

    }

    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
    ) {
        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) {
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3175] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3175] = if s.b[3175] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && s.b[3175]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3176] = ((-s.v[2013]) < 0.0);
        s.v[3176] = if s.b[3176] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && (!s.b[3175])) && s.b[3176]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) && (!s.b[3175])) && (!s.b[3176])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3169])) && (!s.b[3170])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2029, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1971), s.ad_value(1937)), 1890);
        }

        s.b[3177] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3177] = if s.b[3177] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3177]) {
            s.store_div(2020, 2027, 1940);
        }

        s.b[3178] = (s.v[2027] < (-s.v[1941]));
        s.v[3178] = if s.b[3178] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3179] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3179] = if s.b[3179] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) && s.b[3179]) {
            s.store_exp(2005, 2015);
        }

        s.b[3180] = (s.v[2015] < 0.0);
        s.v[3180] = if s.b[3180] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) && (!s.b[3179])) && s.b[3180]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) && (!s.b[3179])) && (!s.b[3180])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && s.b[3178]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2020, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3181] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3181] = if s.b[3181] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && s.b[3181]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3182] = ((-s.v[2011]) < 0.0);
        s.v[3182] = if s.b[3182] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && (!s.b[3181])) && s.b[3182]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && (!s.b[3181])) && (!s.b[3182])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3183] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3183] = if s.b[3183] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && s.b[3183]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3184] = ((-s.v[2013]) < 0.0);
        s.v[3184] = if s.b[3184] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && (!s.b[3183])) && s.b[3184]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) && (!s.b[3183])) && (!s.b[3184])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3177])) && (!s.b[3178])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2020, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1972), s.ad_value(1937)), 1890);
        }

        s.b[3185] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3185] = if s.b[3185] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3185]) {
            s.store_div(2021, 2027, 1940);
        }

        s.b[3186] = (s.v[2027] < (-s.v[1941]));
        s.v[3186] = if s.b[3186] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3187] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3187] = if s.b[3187] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) && s.b[3187]) {
            s.store_exp(2005, 2015);
        }

        s.b[3188] = (s.v[2015] < 0.0);
        s.v[3188] = if s.b[3188] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) && (!s.b[3187])) && s.b[3188]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) && (!s.b[3187])) && (!s.b[3188])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && s.b[3186]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2021, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3189] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3189] = if s.b[3189] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && s.b[3189]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3190] = ((-s.v[2011]) < 0.0);
        s.v[3190] = if s.b[3190] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3189])) && s.b[3190]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3189])) && (!s.b[3190])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.b[3191] = ((((-s.v[2013])) as f64).abs() < 230.25850929940458);
        s.v[3191] = if s.b[3191] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && s.b[3191]) {
            s.store_exp_neg_input(2005, 2013);
        }

        s.b[3192] = ((-s.v[2013]) < 0.0);
        s.v[3192] = if s.b[3192] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3191])) && s.b[3192]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) && (!s.b[3191])) && (!s.b[3192])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3185])) && (!s.b[3186])) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_add(2021, 2013, 2014);
        }

        if (((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1973), s.ad_value(1937)), 1890);
        }

        s.b[3193] = (((s.v[2027]) as f64).abs() <= s.v[1941]);
        s.v[3193] = if s.b[3193] { 1.0 } else { 0.0 };

        if ((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && s.b[3193]) {
            s.store_div(2022, 2027, 1940);
        }

        s.b[3194] = (s.v[2027] < (-s.v[1941]));
        s.v[3194] = if s.b[3194] { 1.0 } else { 0.0 };

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) {
            s.store_neg(1999, 2027);
            s.store_scaled_div(2000, 1999, 1940, 1.25);
            s.store_scaled_sub_ad(2001, A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0)), 0.5);
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
            s.store_add(824, 2002, 2003);
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.b[3195] = (((s.v[2015]) as f64).abs() < 230.25850929940458);
        s.v[3195] = if s.b[3195] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) && s.b[3195]) {
            s.store_exp(2005, 2015);
        }

        s.b[3196] = (s.v[2015] < 0.0);
        s.v[3196] = if s.b[3196] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) && (!s.b[3195])) && s.b[3196]) {
            s.store_div_from_scalar_offset_ad(2005, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) && (!s.b[3195])) && (!s.b[3196])) {
            s.store_scaled_offset_ad(2005, A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && s.b[3194]) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
            s.store_neg_ad(2022, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) {
            s.store_div_from_scalar_offset_scaled_input(1998, 1.0, 1938, 0.732464877560822, 1.25);
            s.store_mul_ad_affine_product_lhs(2010, A::scale(s.ad_value(1940), 1.25), s.ad_value(1998), 1.0, (-1.0), 1998);
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.b[3197] = ((((-s.v[2011])) as f64).abs() < 230.25850929940458);
        s.v[3197] = if s.b[3197] { 1.0 } else { 0.0 };

        if ((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && s.b[3197]) {
            s.store_exp_neg_input(2009, 2011);
        }

        s.b[3198] = ((-s.v[2011]) < 0.0);
        s.v[3198] = if s.b[3198] { 1.0 } else { 0.0 };

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && (!s.b[3197])) && s.b[3198]) {
            s.store_div_from_scalar_offset_ad(2009, 1e-100, A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
        }

        if (((((((((s.b[3067] && (!s.b[3068])) && (!s.b[3077])) && (!s.b[3094])) && (!s.b[3119])) && s.b[3160]) && (!s.b[3193])) && (!s.b[3194])) && (!s.b[3197])) && (!s.b[3198])) {
            s.store_scaled_offset_ad(2009, A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0, 1e100);
        }

    }
}
