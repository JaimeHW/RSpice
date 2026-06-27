#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2569] && s.b[2570]) {
            s.store_scalar(2573, 0.0);
            s.store_scalar(2574, 0.0);
            s.store_scaled_mul(2525, 685, 685, 4.0);
            s.store_div(2526, 685, 686);
            s.store_add_scaled_product_indices(2527, 822, 1.0, 685, 2526, 1.0);
            s.store_add(2528, 686, 2527);
            s.store_sub(2529, 686, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2574, 822, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2581] = (s.v[679] > 0.5);
        s.v[2581] = if s.b[2581] { 1.0 } else { 0.0 };

        s.b[2582] = (s.v[576] == 0.5);
        s.v[2582] = if s.b[2582] { 1.0 } else { 0.0 };

        if (((s.b[2569] && s.b[2570]) && s.b[2581]) && s.b[2582]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::mul(s.ad_value(2574), s.ad_value(573)));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2581]) && (!s.b[2582])) {
            s.store_pow_ad(2573, A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(573))), s.ad_value(576));
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2581]) {
            s.store_add_scaled_product_mixed_aia(1905, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2573)), 1.0, 588, A::sub(s.ad_value(822), s.ad_value(2574)), 1.0);
        }

        s.b[2583] = (s.v[680] > 0.5);
        s.v[2583] = if s.b[2583] { 1.0 } else { 0.0 };

        s.b[2584] = (s.v[577] == 0.5);
        s.v[2584] = if s.b[2584] { 1.0 } else { 0.0 };

        if (((s.b[2569] && s.b[2570]) && s.b[2583]) && s.b[2584]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::mul(s.ad_value(2574), s.ad_value(574)));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2583]) && (!s.b[2584])) {
            s.store_pow_ad(2573, A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(574))), s.ad_value(577));
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2583]) {
            s.store_add_scaled_product_mixed_aia(1906, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2573)), 1.0, 589, A::sub(s.ad_value(822), s.ad_value(2574)), 1.0);
        }

        s.b[2585] = (s.v[681] > 0.5);
        s.v[2585] = if s.b[2585] { 1.0 } else { 0.0 };

        s.b[2586] = (s.v[578] == 0.5);
        s.v[2586] = if s.b[2586] { 1.0 } else { 0.0 };

        if (((s.b[2569] && s.b[2570]) && s.b[2585]) && s.b[2586]) {
            s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::mul(s.ad_value(2574), s.ad_value(575)));
        }

        if (((s.b[2569] && s.b[2570]) && s.b[2585]) && (!s.b[2586])) {
            s.store_pow_ad(2573, A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(575))), s.ad_value(578));
        }

        if ((s.b[2569] && s.b[2570]) && s.b[2585]) {
            s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2573)), 1.0, 590, A::sub(s.ad_value(822), s.ad_value(2574)), 1.0);
        }

        s.b[2587] = (p.p889 > 0.0);
        s.v[2587] = if s.b[2587] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2587]) {
            s.store_scaled_offset_ad(643, A::powf(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), p.p890), (-(((0.5 * 0.001)) as f64).powf(p.p890)), p.p889);
            s.store_offset(641, 643, p.p879);
            s.store_div_from_scalar(451, 1.0, 641);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2587])) {
            s.store_scalar(641, p.p879);
        }

        s.b[2588] = (p.p891 > 0.0);
        s.v[2588] = if s.b[2588] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2588]) {
            s.store_scaled_offset_ad(645, A::powf(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), p.p892), (-(((0.5 * 0.001)) as f64).powf(p.p892)), p.p891);
            s.store_mul_offset_rhs(444, 444, 645, 1.0);
        }

        if (s.b[2569] && (!s.b[2570])) {
            s.store_scalar(2538, 0.0);
            s.store_scalar(2535, 0.0);
        }

        s.b[2589] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));
        s.v[2589] = if s.b[2589] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2589]) {
            s.store_scaled_mul(2525, 658, 658, 4.0);
            s.store_div(2526, 658, 659);
            s.store_add_scaled_product_indices(2527, 821, 1.0, 658, 2526, 1.0);
            s.store_add(2528, 659, 2527);
            s.store_sub(2529, 659, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2532, 821, 659, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2590] = (s.v[821] < s.v[655]);
        s.v[2590] = if s.b[2590] { 1.0 } else { 0.0 };

        s.b[2591] = (((((-0.5) * (s.v[821] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[2591] = if s.b[2591] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) && s.b[2591]) {
            s.store_exp_scaled_input(2533, 821, (s.v[372] * (-0.5)));
        }

        s.b[2592] = (((-0.5) * (s.v[821] * s.v[372])) < 0.0);
        s.v[2592] = if s.b[2592] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) && (!s.b[2591])) && s.b[2592]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2533, 1e-100, (-230.25850929940458), A::scale(s.ad_value(821), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) && (!s.b[2591])) && (!s.b[2592])) {
            s.store_scaled_offset_ad(2533, A::mul_offset_rhs(A::scale_offset(s.ad_value(821), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(821), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(821), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) {
            s.store_div_from_scalar(2534, 1.0, 2533);
            s.store_square(2531, 2534);
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && (!s.b[2590])) {
            s.store_mul_offset_ad_lhs(2531, A::sub_scaled_inputs(s.ad_value(821), s.v[372], s.ad_value(655), s.v[372]), 1.0, 656);
            s.store_sqrt(2534, 2531);
            s.store_div_from_scalar(2533, 1.0, 2534);
        }

        if ((s.b[2569] && (!s.b[2570])) && s.b[2589]) {
            s.store_offset(2531, 2531, (-1.0));
        }

        s.b[2593] = (s.v[821] > 0.0);
        s.v[2593] = if s.b[2593] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2593]) {
            s.store_scaled_ln_ad(2535, A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2533), 1.0, A::offset(s.ad_value(2533), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && (!s.b[2593])) {
            s.store_sub_ad_lhs(2535, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2534), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2534), 1.0, A::scale_offset(s.ad_value(2534), 3.0, 1.0))))), (s.v[371] * 2.0)), 821);
        }

        if ((s.b[2569] && (!s.b[2570])) && s.b[2589]) {
            s.store_sub(2536, 657, 2535);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2537, 821, 0.5, 2536, 0.5, A::offset(A::mul(A::sub(s.ad_value(821), s.ad_value(2536)), A::sub(s.ad_value(821), s.ad_value(2536))), ((4.0 * s.v[371]) * s.v[371])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2538, 821, 0.5, 660, 0.5, A::offset(A::mul(A::sub(s.ad_value(821), s.ad_value(660)), A::sub(s.ad_value(821), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369])), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(2539, 821, 821, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[2594] = (s.v[647] == 0.0);
        s.v[2594] = if s.b[2594] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2594]) {
            s.store_scalar(1902, 0.0);
        }

        s.b[2595] = ((p.p857 == 0.0) && (p.p862 == 0.0));
        s.v[2595] = if s.b[2595] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) {
            s.store_sub_from_scalar(2543, s.v[394], 2537);
        }

        s.b[2597] = (p.p848 == 0.5);
        s.v[2597] = if s.b[2597] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) && s.b[2597]) {
            s.store_sqrt_scaled_input(2540, 2543, s.v[430]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) && (!s.b[2597])) {
            s.store_powf_ad(2540, A::scale(s.ad_value(2543), s.v[430]), p.p848);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) {
            s.store_scale(2547, 2540, s.v[424]);
        }

        s.b[2598] = (p.p862 == 0.0);
        s.v[2598] = if s.b[2598] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) {
            s.store_div_scaled_inputs_indices(2550, 2547, (s.v[409] * s.v[439]), 2543, 1.0);
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[436]), 2550);
            s.store_square(2552, 2551);
            s.store_sqrt_ad(2553, A::div_scaled_product_offset_denominator(s.ad_value(2552), s.ad_value(2552), 1.0, A::square(s.ad_value(2552)), 1.0, 1.0));
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, s.ad_value(2553), (-s.v[436]), s.ad_value(2551), s.ad_value(2554), s.v[436], s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2601] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.v[2601] = if s.b[2601] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && s.b[2601]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2601])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2602] = (s.v[2561] > 0.0);
        s.v[2602] = if s.b[2602] { 1.0 } else { 0.0 };

        s.b[2603] = (s.v[2560] > (-230.25850929940458));
        s.v[2603] = if s.b[2603] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2602])) && s.b[2603]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2603])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2604] = (p.p868 == 0.0);
        s.v[2604] = if s.b[2604] { 1.0 } else { 0.0 };

        s.b[2605] = (p.p848 == 0.5);
        s.v[2605] = if s.b[2605] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && s.b[2605]) {
            s.store_sqrt_scaled_input_ad(2540, A::sub_from_scalar(p.p845, s.ad_value(2538)), s.v[430]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && (!s.b[2605])) {
            s.store_powf_ad(2540, A::scale_offset(s.ad_value(2538), (-s.v[430]), ((p.p845) * (s.v[430]))), p.p848);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) {
            s.store_div_scaled_offset_numerator(2565, s.ad_value(2538), ((-s.v[427]) * s.v[412]), (((p.p845) * (s.v[427])) * s.v[412]), s.ad_value(2540), 1.0);
        }

        s.b[2606] = (((((-s.v[442]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.v[2606] = if s.b[2606] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && s.b[2606]) {
            s.store_exp_div_scaled_inputs_indices(2540, 442, -1.0, 2565, 1.0);
        }

        s.b[2607] = (((-s.v[442]) / s.v[2565]) < 0.0);
        s.v[2607] = if s.b[2607] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && (!s.b[2606])) && s.b[2607]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && (!s.b[2606])) && (!s.b[2607])) {
            let assign57180_ad_e72546: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2540, assign57180_ad_e72546, 1e100);
        }

        s.b[2608] = (p.p877 > 1000.0);
        s.v[2608] = if s.b[2608] { 1.0 } else { 0.0 };

        s.b[2609] = (s.v[2539] > ((-s.v[445]) * p.p877));
        s.v[2609] = if s.b[2609] { 1.0 } else { 0.0 };

        s.b[2610] = (p.p880 == 4.0);
        s.v[2610] = if s.b[2610] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2608])) && s.b[2609]) && s.b[2610]) {
            s.store_mul_scaled_ad_lhs(2540, A::mul3_scaled_output(s.ad_value(2539), s.ad_value(2539), s.ad_value(2539), ((s.v[449] * s.v[449]) * s.v[449])), 2539, s.v[449]);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2608])) && s.b[2609]) && (!s.b[2610])) {
            s.store_powf_ad(2540, A::abs_scaled_input(s.ad_value(2539), s.v[449]), p.p880);
        }

        s.b[2611] = (s.v[409] == 0.5);
        s.v[2611] = if s.b[2611] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && s.b[2611]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2532), s.v[406]));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2611])) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[406])), s.v[409]);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) {
            s.store_add_scaled_inputs3_offset_indices(1902, 2540, ((-s.v[418]) * p.p30), 821, (s.v[421] * p.p30), 2532, ((-s.v[421]) * p.p30), (s.v[418] * p.p30));
        }

        s.b[2612] = (s.v[648] == 0.0);
        s.v[2612] = if s.b[2612] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2612]) {
            s.store_scalar(1903, 0.0);
        }

        s.b[2613] = ((p.p858 == 0.0) && (p.p863 == 0.0));
        s.v[2613] = if s.b[2613] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) {
            s.store_sub_from_scalar(2543, s.v[395], 2537);
        }

        s.b[2615] = (p.p849 == 0.5);
        s.v[2615] = if s.b[2615] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) && s.b[2615]) {
            s.store_sqrt_scaled_input(2540, 2543, s.v[431]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) && (!s.b[2615])) {
            s.store_powf_ad(2540, A::scale(s.ad_value(2543), s.v[431]), p.p849);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) {
            s.store_scale(2547, 2540, s.v[425]);
        }

        s.b[2616] = (p.p863 == 0.0);
        s.v[2616] = if s.b[2616] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) {
            s.store_div_scaled_inputs_indices(2550, 2547, (s.v[410] * s.v[440]), 2543, 1.0);
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[437]), 2550);
            s.store_square(2552, 2551);
            s.store_sqrt_ad(2553, A::div_scaled_product_offset_denominator(s.ad_value(2552), s.ad_value(2552), 1.0, A::square(s.ad_value(2552)), 1.0, 1.0));
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, s.ad_value(2553), (-s.v[437]), s.ad_value(2551), s.ad_value(2554), s.v[437], s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2619] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.v[2619] = if s.b[2619] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && s.b[2619]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2619])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2620] = (s.v[2561] > 0.0);
        s.v[2620] = if s.b[2620] { 1.0 } else { 0.0 };

        s.b[2621] = (s.v[2560] > (-230.25850929940458));
        s.v[2621] = if s.b[2621] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2621]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2621])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2622] = (p.p869 == 0.0);
        s.v[2622] = if s.b[2622] { 1.0 } else { 0.0 };

        s.b[2623] = (p.p849 == 0.5);
        s.v[2623] = if s.b[2623] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && s.b[2623]) {
            s.store_sqrt_scaled_input_ad(2540, A::sub_from_scalar(p.p846, s.ad_value(2538)), s.v[431]);
        }

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && (!s.b[2623])) {
            s.store_powf_ad(2540, A::scale_offset(s.ad_value(2538), (-s.v[431]), ((p.p846) * (s.v[431]))), p.p849);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) {
            s.store_div_scaled_offset_numerator(2565, s.ad_value(2538), ((-s.v[428]) * s.v[413]), (((p.p846) * (s.v[428])) * s.v[413]), s.ad_value(2540), 1.0);
        }

        s.b[2624] = (((((-s.v[443]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.v[2624] = if s.b[2624] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && s.b[2624]) {
            s.store_exp_div_scaled_inputs_indices(2540, 443, -1.0, 2565, 1.0);
        }

        s.b[2625] = (((-s.v[443]) / s.v[2565]) < 0.0);
        s.v[2625] = if s.b[2625] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && (!s.b[2624])) && s.b[2625]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && (!s.b[2624])) && (!s.b[2625])) {
            let assign57930_ad_e73812: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2540, assign57930_ad_e73812, 1e100);
        }

        s.b[2626] = (p.p878 > 1000.0);
        s.v[2626] = if s.b[2626] { 1.0 } else { 0.0 };

        s.b[2627] = (s.v[2539] > ((-s.v[445]) * p.p878));
        s.v[2627] = if s.b[2627] { 1.0 } else { 0.0 };

        s.b[2628] = (p.p881 == 4.0);
        s.v[2628] = if s.b[2628] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2626])) && s.b[2627]) && s.b[2628]) {
            s.store_mul_scaled_ad_lhs(2540, A::mul3_scaled_output(s.ad_value(2539), s.ad_value(2539), s.ad_value(2539), ((s.v[450] * s.v[450]) * s.v[450])), 2539, s.v[450]);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2626])) && s.b[2627]) && (!s.b[2628])) {
            s.store_powf_ad(2540, A::abs_scaled_input(s.ad_value(2539), s.v[450]), p.p881);
        }

        s.b[2629] = (s.v[410] == 0.5);
        s.v[2629] = if s.b[2629] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && s.b[2629]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2532), s.v[407]));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2629])) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[407])), s.v[410]);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) {
            s.store_add_scaled_inputs3_offset_indices(1903, 2540, ((-s.v[419]) * p.p30), 821, (s.v[422] * p.p30), 2532, ((-s.v[422]) * p.p30), (s.v[419] * p.p30));
        }

        s.b[2630] = (s.v[649] == 0.0);
        s.v[2630] = if s.b[2630] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2630]) {
            s.store_scalar(1904, 0.0);
        }

        s.b[2631] = ((p.p859 == 0.0) && (p.p864 == 0.0));
        s.v[2631] = if s.b[2631] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) {
            s.store_sub_from_scalar(2543, s.v[396], 2537);
        }

        s.b[2633] = (p.p850 == 0.5);
        s.v[2633] = if s.b[2633] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) && s.b[2633]) {
            s.store_sqrt_scaled_input(2540, 2543, s.v[432]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) && (!s.b[2633])) {
            s.store_powf_ad(2540, A::scale(s.ad_value(2543), s.v[432]), p.p850);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) {
            s.store_scale(2547, 2540, s.v[426]);
        }

        s.b[2634] = (p.p864 == 0.0);
        s.v[2634] = if s.b[2634] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) {
            s.store_div_scaled_inputs_indices(2550, 2547, (s.v[411] * s.v[441]), 2543, 1.0);
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[438]), 2550);
            s.store_square(2552, 2551);
            s.store_sqrt_ad(2553, A::div_scaled_product_offset_denominator(s.ad_value(2552), s.ad_value(2552), 1.0, A::square(s.ad_value(2552)), 1.0, 1.0));
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, s.ad_value(2553), (-s.v[438]), s.ad_value(2551), s.ad_value(2554), s.v[438], s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2637] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.v[2637] = if s.b[2637] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && s.b[2637]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2637])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2638] = (s.v[2561] > 0.0);
        s.v[2638] = if s.b[2638] { 1.0 } else { 0.0 };

        s.b[2639] = (s.v[2560] > (-230.25850929940458));
        s.v[2639] = if s.b[2639] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2638])) && s.b[2639]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2638])) && (!s.b[2639])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2640] = (p.p870 == 0.0);
        s.v[2640] = if s.b[2640] { 1.0 } else { 0.0 };

        s.b[2641] = (p.p850 == 0.5);
        s.v[2641] = if s.b[2641] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && s.b[2641]) {
            s.store_sqrt_scaled_input_ad(2540, A::sub_from_scalar(p.p847, s.ad_value(2538)), s.v[432]);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && (!s.b[2641])) {
            s.store_powf_ad(2540, A::scale_offset(s.ad_value(2538), (-s.v[432]), ((p.p847) * (s.v[432]))), p.p850);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) {
            s.store_div_scaled_offset_numerator(2565, s.ad_value(2538), ((-s.v[429]) * s.v[414]), (((p.p847) * (s.v[429])) * s.v[414]), s.ad_value(2540), 1.0);
        }

        s.b[2642] = (((((-s.v[444]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.v[2642] = if s.b[2642] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && s.b[2642]) {
            s.store_exp_div_scaled_inputs_indices(2540, 444, -1.0, 2565, 1.0);
        }

        s.b[2643] = (((-s.v[444]) / s.v[2565]) < 0.0);
        s.v[2643] = if s.b[2643] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && (!s.b[2642])) && s.b[2643]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && (!s.b[2642])) && (!s.b[2643])) {
            let assign58680_ad_e75078: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2540, assign58680_ad_e75078, 1e100);
        }

        s.b[2644] = (s.v[641] > 1000.0);
        s.v[2644] = if s.b[2644] { 1.0 } else { 0.0 };

        s.b[2645] = (s.v[2539] > ((-s.v[445]) * s.v[641]));
        s.v[2645] = if s.b[2645] { 1.0 } else { 0.0 };

        s.b[2646] = (p.p882 == 4.0);
        s.v[2646] = if s.b[2646] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && s.b[2645]) && s.b[2646]) {
            s.store_mul_ad_product_lhs(2540, A::mul3(A::mul3(s.ad_value(2539), s.ad_value(451), A::mul(s.ad_value(2539), s.ad_value(451))), s.ad_value(2539), s.ad_value(451)), s.ad_value(2539), 451);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && s.b[2645]) && (!s.b[2646])) {
            s.store_powf_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(451))), p.p882);
        }

        s.b[2647] = (s.v[474] == 1.0);
        s.v[2647] = if s.b[2647] { 1.0 } else { 0.0 };

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
                    s.store_add_scaled_inputs_ad_rhs(2567, 821, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(821), (-1.0 / (p.p888)), ((p.p887) * (1.0 / (p.p888))))), p.p888);
                }
            }
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {
            s.store_scaled_mul(2525, 658, 658, 4.0);
            s.store_div(2526, 658, 659);
            s.store_add_scaled_product_indices(2527, 2567, 1.0, 658, 2526, 1.0);
            s.store_add(2528, 659, 2527);
            s.store_sub(2529, 659, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 659, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2648] = (s.v[411] == 0.5);
        s.v[2648] = if s.b[2648] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && s.b[2648]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2568), s.v[408]));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && (!s.b[2648])) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2568), s.v[408])), s.v[411]);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {
            s.store_add_scaled_inputs3_offset_indices(1904, 2540, ((-s.v[420]) * p.p30), 2567, (s.v[423] * p.p30), 2568, ((-s.v[423]) * p.p30), (s.v[420] * p.p30));
            s.store_sub_offset_lhs(2567, 821, p.p887, 2567);
            s.store_scaled_mul(2525, 658, 658, 4.0);
            s.store_div(2526, 658, 659);
            s.store_add_scaled_product_indices(2527, 2567, 1.0, 658, 2526, 1.0);
            s.store_add(2528, 659, 2527);
            s.store_sub(2529, 659, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 659, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2649] = (s.v[468] == 0.5);
        s.v[2649] = if s.b[2649] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && s.b[2649]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(467)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) && (!s.b[2649])) {
            s.store_pow_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(467))), s.ad_value(468));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {
            s.store_add_scaled_product_mixed_aia(473, A::mul_sub_from_scalar_rhs(s.ad_value(471), 1.0, s.ad_value(2540)), p.p30, 472, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);
            s.store_add(1904, 1904, 473);
        }

        s.b[2650] = (s.v[411] == 0.5);
        s.v[2650] = if s.b[2650] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) && s.b[2650]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2532), s.v[408]));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) && (!s.b[2650])) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[408])), s.v[411]);
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) {
            s.store_add_scaled_inputs3_offset_indices(1904, 2540, ((-s.v[420]) * p.p30), 821, (s.v[423] * p.p30), 2532, ((-s.v[423]) * p.p30), (s.v[420] * p.p30));
        }

        s.b[2651] = (s.v[637] > 0.0);
        s.v[2651] = if s.b[2651] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2651]) {
            s.store_mul_sub_ad_rhs(644, 637, A::pow(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), s.ad_value(638)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(638)));
            s.store_add(642, 543, 644);
            s.store_div_from_scalar(617, 1.0, 642);
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2651])) {
            s.copy_ad(642, 543);
        }

        s.b[2652] = (s.v[639] > 0.0);
        s.v[2652] = if s.b[2652] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2652]) {
            s.store_mul_sub_ad_rhs(646, 639, A::pow(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), s.ad_value(640)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(640)));
            s.store_mul_offset_rhs(611, 611, 646, 1.0);
        }

        if (s.b[2569] && (!s.b[2570])) {
            s.store_scalar(2538, 0.0);
            s.store_scalar(2535, 0.0);
        }

        s.b[2653] = (!(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)));
        s.v[2653] = if s.b[2653] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2653]) {
            s.store_scaled_mul(2525, 685, 685, 4.0);
            s.store_div(2526, 685, 686);
            s.store_add_scaled_product_indices(2527, 822, 1.0, 685, 2526, 1.0);
            s.store_add(2528, 686, 2527);
            s.store_sub(2529, 686, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2532, 822, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2654] = (s.v[822] < s.v[682]);
        s.v[2654] = if s.b[2654] { 1.0 } else { 0.0 };

        s.b[2655] = (((((-0.5) * (s.v[822] * s.v[372]))) as f64).abs() < 230.25850929940458);
        s.v[2655] = if s.b[2655] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) && s.b[2655]) {
            s.store_exp_scaled_input(2533, 822, (s.v[372] * (-0.5)));
        }

        s.b[2656] = (((-0.5) * (s.v[822] * s.v[372])) < 0.0);
        s.v[2656] = if s.b[2656] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) && (!s.b[2655])) && s.b[2656]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2533, 1e-100, (-230.25850929940458), A::scale(s.ad_value(822), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) && (!s.b[2655])) && (!s.b[2656])) {
            s.store_scaled_offset_ad(2533, A::mul_offset_rhs(A::scale_offset(s.ad_value(822), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(822), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(822), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2654]) {
            s.store_div_from_scalar(2534, 1.0, 2533);
            s.store_square(2531, 2534);
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && (!s.b[2654])) {
            s.store_mul_offset_ad_lhs(2531, A::sub_scaled_inputs(s.ad_value(822), s.v[372], s.ad_value(682), s.v[372]), 1.0, 683);
            s.store_sqrt(2534, 2531);
            s.store_div_from_scalar(2533, 1.0, 2534);
        }

        if ((s.b[2569] && (!s.b[2570])) && s.b[2653]) {
            s.store_offset(2531, 2531, (-1.0));
        }

        s.b[2657] = (s.v[822] > 0.0);
        s.v[2657] = if s.b[2657] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2657]) {
            s.store_scaled_ln_ad(2535, A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2533), 1.0, A::offset(s.ad_value(2533), 3.0)))), (s.v[371] * 2.0));
        }

        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && (!s.b[2657])) {
            s.store_sub_ad_lhs(2535, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2534), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2534), 1.0, A::scale_offset(s.ad_value(2534), 3.0, 1.0))))), (s.v[371] * 2.0)), 822);
        }

        if ((s.b[2569] && (!s.b[2570])) && s.b[2653]) {
            s.store_sub(2536, 684, 2535);
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2537, 822, 0.5, 2536, 0.5, A::offset(A::mul(A::sub(s.ad_value(822), s.ad_value(2536)), A::sub(s.ad_value(822), s.ad_value(2536))), ((4.0 * s.v[371]) * s.v[371])), (-0.5));
            s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2538, 822, 0.5, 687, 0.5, A::offset(A::mul(A::sub(s.ad_value(822), s.ad_value(687)), A::sub(s.ad_value(822), s.ad_value(687))), ((4.0 * s.v[369]) * s.v[369])), (-0.5));
            s.store_scaled_sub_sqrt_square_offset_rhs(2539, 822, 822, ((4.0 * 1e-6) * 1e-6), 0.5);
        }

        s.b[2658] = (s.v[674] == 0.0);
        s.v[2658] = if s.b[2658] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2658]) {
            s.store_scalar(1905, 0.0);
        }

        s.b[2659] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));
        s.v[2659] = if s.b[2659] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) {
            s.store_sub(2543, 570, 2537);
        }

        s.b[2661] = (s.v[512] == 0.5);
        s.v[2661] = if s.b[2661] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && s.b[2661]) {
            s.store_sqrt_mul(2540, 2543, 597);
        }

    }

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && (!s.b[2661])) {
            s.store_pow_ad(2540, A::mul(s.ad_value(2543), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) {
            s.store_mul(2547, 591, 2540);
        }

        s.b[2662] = (s.v[526] == 0.0);
        s.v[2662] = if s.b[2662] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) {
            s.store_mul_div_scaled_product_rhs(2550, 606, s.ad_value(2547), s.ad_value(576), 1.0, s.ad_value(2543), 1.0);
            s.store_div_scaled_inputs_indices(2551, 603, 0.666666666666667, 2550, 1.0);
            s.store_square(2552, 2551);
            s.store_sqrt_ad(2553, A::div_scaled_product_offset_denominator(s.ad_value(2552), s.ad_value(2552), 1.0, A::square(s.ad_value(2552)), 1.0, 1.0));
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, A::mul3(s.ad_value(603), s.ad_value(2551), s.ad_value(2554)), 1.0, s.ad_value(603), s.ad_value(2553), (-1.0), s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2665] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.v[2665] = if s.b[2665] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && s.b[2665]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2665])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2666] = (s.v[2561] > 0.0);
        s.v[2666] = if s.b[2666] { 1.0 } else { 0.0 };

        s.b[2667] = (s.v[2560] > (-230.25850929940458));
        s.v[2667] = if s.b[2667] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2667]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2667])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2668] = (s.v[532] == 0.0);
        s.v[2668] = if s.b[2668] { 1.0 } else { 0.0 };

        s.b[2669] = (s.v[512] == 0.5);
        s.v[2669] = if s.b[2669] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && s.b[2669]) {
            s.store_sqrt_mul_ad(2540, A::sub(s.ad_value(509), s.ad_value(2538)), s.ad_value(597));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2669])) {
            s.store_pow_ad(2540, A::mul(A::sub(s.ad_value(509), s.ad_value(2538)), s.ad_value(597)), s.ad_value(512));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) {
            s.store_mul_div_scaled_product_rhs(2565, 579, A::sub(s.ad_value(509), s.ad_value(2538)), s.ad_value(594), 1.0, s.ad_value(2540), 1.0);
        }

        s.b[2670] = (((((-s.v[609]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.v[2670] = if s.b[2670] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && s.b[2670]) {
            s.store_exp_div_scaled_inputs_indices(2540, 609, -1.0, 2565, 1.0);
        }

        s.b[2671] = (((-s.v[609]) / s.v[2565]) < 0.0);
        s.v[2671] = if s.b[2671] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2670])) && s.b[2671]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2670])) && (!s.b[2671])) {
            let assign60080_ad_e77478: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2540, assign60080_ad_e77478, 1e100);
        }

        s.b[2672] = (s.v[541] > 1000.0);
        s.v[2672] = if s.b[2672] { 1.0 } else { 0.0 };

        s.b[2673] = (s.v[2539] > ((-s.v[445]) * s.v[541]));
        s.v[2673] = if s.b[2673] { 1.0 } else { 0.0 };

        s.b[2674] = (s.v[544] == 4.0);
        s.v[2674] = if s.b[2674] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && s.b[2673]) && s.b[2674]) {
            s.store_mul_ad_product_lhs(2540, A::mul3(A::mul3(s.ad_value(2539), s.ad_value(615), A::mul(s.ad_value(2539), s.ad_value(615))), s.ad_value(2539), s.ad_value(615)), s.ad_value(2539), 615);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && s.b[2673]) && (!s.b[2674])) {
            s.store_pow_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(615))), s.ad_value(544));
        }

        s.b[2675] = (s.v[576] == 0.5);
        s.v[2675] = if s.b[2675] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2675]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(573)));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2675])) {
            s.store_pow_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(573))), s.ad_value(576));
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) {
            s.store_add_scaled_product_mixed_aia(1905, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2540)), p.p30, 588, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);
        }

        s.b[2676] = (s.v[675] == 0.0);
        s.v[2676] = if s.b[2676] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2676]) {
            s.store_scalar(1906, 0.0);
        }

        s.b[2677] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));
        s.v[2677] = if s.b[2677] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) {
            s.store_sub(2543, 571, 2537);
        }

        s.b[2679] = (s.v[513] == 0.5);
        s.v[2679] = if s.b[2679] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && s.b[2679]) {
            s.store_sqrt_mul(2540, 2543, 598);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && (!s.b[2679])) {
            s.store_pow_ad(2540, A::mul(s.ad_value(2543), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) {
            s.store_mul(2547, 592, 2540);
        }

        s.b[2680] = (s.v[527] == 0.0);
        s.v[2680] = if s.b[2680] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) {
            s.store_mul_div_scaled_product_rhs(2550, 607, s.ad_value(2547), s.ad_value(577), 1.0, s.ad_value(2543), 1.0);
            s.store_div_scaled_inputs_indices(2551, 604, 0.666666666666667, 2550, 1.0);
            s.store_square(2552, 2551);
            s.store_sqrt_ad(2553, A::div_scaled_product_offset_denominator(s.ad_value(2552), s.ad_value(2552), 1.0, A::square(s.ad_value(2552)), 1.0, 1.0));
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, A::mul3(s.ad_value(604), s.ad_value(2551), s.ad_value(2554)), 1.0, s.ad_value(604), s.ad_value(2553), (-1.0), s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2683] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.v[2683] = if s.b[2683] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && s.b[2683]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2683])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2684] = (s.v[2561] > 0.0);
        s.v[2684] = if s.b[2684] { 1.0 } else { 0.0 };

        s.b[2685] = (s.v[2560] > (-230.25850929940458));
        s.v[2685] = if s.b[2685] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2685]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2685])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2686] = (s.v[533] == 0.0);
        s.v[2686] = if s.b[2686] { 1.0 } else { 0.0 };

        s.b[2687] = (s.v[513] == 0.5);
        s.v[2687] = if s.b[2687] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && s.b[2687]) {
            s.store_sqrt_mul_ad(2540, A::sub(s.ad_value(510), s.ad_value(2538)), s.ad_value(598));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2687])) {
            s.store_pow_ad(2540, A::mul(A::sub(s.ad_value(510), s.ad_value(2538)), s.ad_value(598)), s.ad_value(513));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) {
            s.store_mul_div_scaled_product_rhs(2565, 580, A::sub(s.ad_value(510), s.ad_value(2538)), s.ad_value(595), 1.0, s.ad_value(2540), 1.0);
        }

        s.b[2688] = (((((-s.v[610]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.v[2688] = if s.b[2688] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && s.b[2688]) {
            s.store_exp_div_scaled_inputs_indices(2540, 610, -1.0, 2565, 1.0);
        }

        s.b[2689] = (((-s.v[610]) / s.v[2565]) < 0.0);
        s.v[2689] = if s.b[2689] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2688])) && s.b[2689]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2688])) && (!s.b[2689])) {
            let assign60830_ad_e78744: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2540, assign60830_ad_e78744, 1e100);
        }

        s.b[2690] = (s.v[542] > 1000.0);
        s.v[2690] = if s.b[2690] { 1.0 } else { 0.0 };

        s.b[2691] = (s.v[2539] > ((-s.v[445]) * s.v[542]));
        s.v[2691] = if s.b[2691] { 1.0 } else { 0.0 };

        s.b[2692] = (s.v[545] == 4.0);
        s.v[2692] = if s.b[2692] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && s.b[2691]) && s.b[2692]) {
            s.store_mul_ad_product_lhs(2540, A::mul3(A::mul3(s.ad_value(2539), s.ad_value(616), A::mul(s.ad_value(2539), s.ad_value(616))), s.ad_value(2539), s.ad_value(616)), s.ad_value(2539), 616);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && s.b[2691]) && (!s.b[2692])) {
            s.store_pow_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(616))), s.ad_value(545));
        }

        s.b[2693] = (s.v[577] == 0.5);
        s.v[2693] = if s.b[2693] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2693]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(574)));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2693])) {
            s.store_pow_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(574))), s.ad_value(577));
        }

        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) {
            s.store_add_scaled_product_mixed_aia(1906, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2540)), p.p30, 589, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);
        }

        s.b[2694] = (s.v[676] == 0.0);
        s.v[2694] = if s.b[2694] { 1.0 } else { 0.0 };

        if ((s.b[2569] && (!s.b[2570])) && s.b[2694]) {
            s.store_scalar(1907, 0.0);
        }

        s.b[2695] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));
        s.v[2695] = if s.b[2695] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) {
            s.store_sub(2543, 572, 2537);
        }

        s.b[2697] = (s.v[514] == 0.5);
        s.v[2697] = if s.b[2697] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && s.b[2697]) {
            s.store_sqrt_mul(2540, 2543, 599);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && (!s.b[2697])) {
            s.store_pow_ad(2540, A::mul(s.ad_value(2543), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) {
            s.store_mul(2547, 593, 2540);
        }

        s.b[2698] = (s.v[528] == 0.0);
        s.v[2698] = if s.b[2698] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) {
            s.store_mul_div_scaled_product_rhs(2550, 608, s.ad_value(2547), s.ad_value(578), 1.0, s.ad_value(2543), 1.0);
            s.store_div_scaled_inputs_indices(2551, 605, 0.666666666666667, 2550, 1.0);
            s.store_square(2552, 2551);
            s.store_sqrt_ad(2553, A::div_scaled_product_offset_denominator(s.ad_value(2552), s.ad_value(2552), 1.0, A::square(s.ad_value(2552)), 1.0, 1.0));
            s.store_sqrt(2554, 2553);
            s.store_mul(2555, 2553, 2554);
            s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);
            s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);
            s.store_add_scaled_value_products(2560, A::mul3(s.ad_value(605), s.ad_value(2551), s.ad_value(2554)), 1.0, s.ad_value(605), s.ad_value(2553), (-1.0), s.ad_value(2550), s.ad_value(2555), 0.5);
            s.store_mul_offset_lhs(2561, 2559, (-1.0), 2558);
            s.store_square(2522, 2561);
        }

        s.b[2701] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));
        s.v[2701] = if s.b[2701] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2701]) {
            s.store_exp_sub(2540, 2560, 2522);
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2701])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2702] = (s.v[2561] > 0.0);
        s.v[2702] = if s.b[2702] { 1.0 } else { 0.0 };

        s.b[2703] = (s.v[2560] > (-230.25850929940458));
        s.v[2703] = if s.b[2703] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2702])) && s.b[2703]) {
            s.store_exp(2540, 2560);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2702])) && (!s.b[2703])) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        s.b[2704] = (s.v[534] == 0.0);
        s.v[2704] = if s.b[2704] { 1.0 } else { 0.0 };

        s.b[2705] = (s.v[514] == 0.5);
        s.v[2705] = if s.b[2705] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && s.b[2705]) {
            s.store_sqrt_mul_ad(2540, A::sub(s.ad_value(511), s.ad_value(2538)), s.ad_value(599));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2705])) {
            s.store_pow_ad(2540, A::mul(A::sub(s.ad_value(511), s.ad_value(2538)), s.ad_value(599)), s.ad_value(514));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) {
            s.store_mul_div_scaled_product_rhs(2565, 581, A::sub(s.ad_value(511), s.ad_value(2538)), s.ad_value(596), 1.0, s.ad_value(2540), 1.0);
        }

        s.b[2706] = (((((-s.v[611]) / s.v[2565])) as f64).abs() < 230.25850929940458);
        s.v[2706] = if s.b[2706] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && s.b[2706]) {
            s.store_exp_div_scaled_inputs_indices(2540, 611, -1.0, 2565, 1.0);
        }

        s.b[2707] = (((-s.v[611]) / s.v[2565]) < 0.0);
        s.v[2707] = if s.b[2707] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2706])) && s.b[2707]) {
            s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2706])) && (!s.b[2707])) {
            let assign61580_ad_e80010: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2540, assign61580_ad_e80010, 1e100);
        }

        s.b[2708] = (s.v[642] > 1000.0);
        s.v[2708] = if s.b[2708] { 1.0 } else { 0.0 };

        s.b[2709] = (s.v[2539] > ((-s.v[445]) * s.v[642]));
        s.v[2709] = if s.b[2709] { 1.0 } else { 0.0 };

        s.b[2710] = (s.v[546] == 4.0);
        s.v[2710] = if s.b[2710] { 1.0 } else { 0.0 };

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && s.b[2709]) && s.b[2710]) {
            s.store_mul_ad_product_lhs(2540, A::mul3(A::mul3(s.ad_value(2539), s.ad_value(617), A::mul(s.ad_value(2539), s.ad_value(617))), s.ad_value(2539), s.ad_value(617)), s.ad_value(2539), 617);
        }

        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && s.b[2709]) && (!s.b[2710])) {
            s.store_pow_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(617))), s.ad_value(546));
        }

        s.b[2711] = (s.v[636] == 1.0);
        s.v[2711] = if s.b[2711] { 1.0 } else { 0.0 };

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            if (s.v[822] < s.v[551]) {
                if (((s.v[822] - s.v[551]) / s.v[552]) < (-37.0)) {
                    s.copy_ad(2567, 551);
                } else {
                    s.store_add_scaled_product_left_ad(2567, 551, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(822), 1.0, s.ad_value(551), (-1.0), s.ad_value(552), 1.0)), 552, 1.0);
                }
            } else {
                if (((s.v[822] - s.v[551]) / s.v[552]) > 37.0) {
                    s.copy_ad(2567, 822);
                } else {
                    s.store_add_scaled_product_left_ad(2567, 822, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(551), 1.0, s.ad_value(822), (-1.0), s.ad_value(552), 1.0)), 552, 1.0);
                }
            }
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            s.store_scaled_mul(2525, 685, 685, 4.0);
            s.store_div(2526, 685, 686);
            s.store_add_scaled_product_indices(2527, 2567, 1.0, 685, 2526, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            s.store_add(2528, 686, 2527);
            s.store_sub(2529, 686, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2712] = (s.v[578] == 0.5);
        s.v[2712] = if s.b[2712] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && s.b[2712]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(575)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && (!s.b[2712])) {
            s.store_pow_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(575))), s.ad_value(578));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2540)), p.p30, 590, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);
            s.store_add_scaled_inputs3_indices(2567, 822, 1.0, 551, 1.0, 2567, -1.0);
            s.store_scaled_mul(2525, 685, 685, 4.0);
            s.store_div(2526, 685, 686);
            s.store_add_scaled_product_indices(2527, 2567, 1.0, 685, 2526, 1.0);
            s.store_add(2528, 686, 2527);
            s.store_sub(2529, 686, 2527);
            s.store_sqrt_square_add(2530, 2529, 2525);
            s.store_div_scaled_product_add_scaled_denominator_indices(2568, 2567, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);
        }

        s.b[2713] = (s.v[631] == 0.5);
        s.v[2713] = if s.b[2713] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && s.b[2713]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2568), s.ad_value(630)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && (!s.b[2713])) {
            s.store_pow_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(630))), s.ad_value(631));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {
            s.store_add_scaled_product_mixed_aia(473, A::mul_sub_from_scalar_rhs(s.ad_value(634), 1.0, s.ad_value(2540)), p.p30, 635, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);
            s.store_add(1907, 1907, 473);
        }

        s.b[2714] = (s.v[578] == 0.5);
        s.v[2714] = if s.b[2714] { 1.0 } else { 0.0 };

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) && s.b[2714]) {
            s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(575)));
        }

        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) && (!s.b[2714])) {
            s.store_pow_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(575))), s.ad_value(578));
        }

        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) {
            s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2540)), p.p30, 590, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);
        }

        s.store_neg_ad(839, A::add_scaled_inputs3(s.ad_value(840), 1.0, s.ad_value(841), 1.0, s.ad_value(842), 1.0));

        s.store_add(843, 843, 1894);

        s.store_add(844, 844, 1895);

        s.store_add_scaled_products3(846, s.ad_value(647), s.ad_value(1902), 1.0, s.ad_value(648), s.ad_value(1903), 1.0, s.ad_value(649), s.ad_value(1904), 1.0);

        s.store_add_scaled_products3(847, s.ad_value(674), s.ad_value(1905), 1.0, s.ad_value(675), s.ad_value(1906), 1.0, s.ad_value(676), s.ad_value(1907), 1.0);

        s.b[2729] = (s.v[820] < 0.0);
        s.v[2729] = if s.b[2729] { 1.0 } else { 0.0 };

        if s.b[2729] {
            s.copy_ad(2728, 842);
            s.copy_ad(842, 839);
            s.copy_ad(839, 2728);
        }

        s.store_mul(849, 1888, 1879);

        s.b[2762] = ((s.v[1813] > 0.0) && (s.v[1917] > 0.0));
        s.v[2762] = if s.b[2762] { 1.0 } else { 0.0 };

        s.b[2767] = ((((p.p50 == 1.0) && (s.v[1920] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));
        s.v[2767] = if s.b[2767] { 1.0 } else { 0.0 };

        if (s.b[2762] && s.b[2767]) {
            s.store_div_scaled_product3_mixed_aiia(849, A::square(s.ad_value(1892)), 1888, 1879, 1.0, A::square(s.ad_value(1890)), 1.0);
        }

        s.b[2771] = (((p.p46 != 0.0) && (s.v[285] > 0.0)) && (s.v[1864] > 0.0));
        s.v[2771] = if s.b[2771] { 1.0 } else { 0.0 };

        if s.b[2771] {
            s.store_div_scaled_inputs_indices(1930, 1867, 4.0, 1925, 1.0);
            s.store_mul(1930, 760, 1916);
            s.store_mul(1930, 1848, 1861);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq0_e972, eq0_e972_d_n0, eq0_e972_d_n1, eq0_e972_d_n2, eq0_e972_d_n3, eq0_e972_d_n4, eq0_e972_d_n5, eq0_e972_d_n6, eq0_e972_d_n7, eq0_e972_d_n8, eq0_e972_d_n9, eq0_e972_d_n10, eq0_e972_d_n11, eq0_e972_d_n12, eq0_e972_d_b0, eq0_e972_d_b1, eq0_e972_d_b2, eq0_e972_d_b3, eq0_e972_d_b4, eq0_e972_d_b5, eq0_e972_d_b6,) = {
    if s.b[2715] {
        let eq0_e966: f64 = (s.v[0] * s.v[15]);
        let eq0_e966_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq0_e966_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq0_e966_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq0_e966_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq0_e966_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq0_e966_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq0_e966_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq0_e966_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq0_e966_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq0_e966_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq0_e966_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq0_e966_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq0_e966_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq0_e966_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq0_e966_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq0_e966_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq0_e966_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq0_e966_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq0_e966_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq0_e966_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq0_e968: f64 = (eq0_e966 * p.p32);
        let eq0_e968_d_n0: f64 = (eq0_e966_d_n0 * p.p32);
        let eq0_e968_d_n1: f64 = (eq0_e966_d_n1 * p.p32);
        let eq0_e968_d_n2: f64 = (eq0_e966_d_n2 * p.p32);
        let eq0_e968_d_n3: f64 = (eq0_e966_d_n3 * p.p32);
        let eq0_e968_d_n4: f64 = (eq0_e966_d_n4 * p.p32);
        let eq0_e968_d_n5: f64 = (eq0_e966_d_n5 * p.p32);
        let eq0_e968_d_n6: f64 = (eq0_e966_d_n6 * p.p32);
        let eq0_e968_d_n7: f64 = (eq0_e966_d_n7 * p.p32);
        let eq0_e968_d_n8: f64 = (eq0_e966_d_n8 * p.p32);
        let eq0_e968_d_n9: f64 = (eq0_e966_d_n9 * p.p32);
        let eq0_e968_d_n10: f64 = (eq0_e966_d_n10 * p.p32);
        let eq0_e968_d_n11: f64 = (eq0_e966_d_n11 * p.p32);
        let eq0_e968_d_n12: f64 = (eq0_e966_d_n12 * p.p32);
        let eq0_e968_d_b0: f64 = (eq0_e966_d_b0 * p.p32);
        let eq0_e968_d_b1: f64 = (eq0_e966_d_b1 * p.p32);
        let eq0_e968_d_b2: f64 = (eq0_e966_d_b2 * p.p32);
        let eq0_e968_d_b3: f64 = (eq0_e966_d_b3 * p.p32);
        let eq0_e968_d_b4: f64 = (eq0_e966_d_b4 * p.p32);
        let eq0_e968_d_b5: f64 = (eq0_e966_d_b5 * p.p32);
        let eq0_e968_d_b6: f64 = (eq0_e966_d_b6 * p.p32);
        let eq0_e970: f64 = (eq0_e968 * s.v[836]);
        let eq0_e970_d_n0: f64 = ((eq0_e968_d_n0 * s.v[836]) + (eq0_e968 * s.dn[836][0]));
        let eq0_e970_d_n1: f64 = ((eq0_e968_d_n1 * s.v[836]) + (eq0_e968 * s.dn[836][1]));
        let eq0_e970_d_n2: f64 = ((eq0_e968_d_n2 * s.v[836]) + (eq0_e968 * s.dn[836][2]));
        let eq0_e970_d_n3: f64 = ((eq0_e968_d_n3 * s.v[836]) + (eq0_e968 * s.dn[836][3]));
        let eq0_e970_d_n4: f64 = ((eq0_e968_d_n4 * s.v[836]) + (eq0_e968 * s.dn[836][4]));
        let eq0_e970_d_n5: f64 = ((eq0_e968_d_n5 * s.v[836]) + (eq0_e968 * s.dn[836][5]));
        let eq0_e970_d_n6: f64 = ((eq0_e968_d_n6 * s.v[836]) + (eq0_e968 * s.dn[836][6]));
        let eq0_e970_d_n7: f64 = ((eq0_e968_d_n7 * s.v[836]) + (eq0_e968 * s.dn[836][7]));
        let eq0_e970_d_n8: f64 = ((eq0_e968_d_n8 * s.v[836]) + (eq0_e968 * s.dn[836][8]));
        let eq0_e970_d_n9: f64 = ((eq0_e968_d_n9 * s.v[836]) + (eq0_e968 * s.dn[836][9]));
        let eq0_e970_d_n10: f64 = ((eq0_e968_d_n10 * s.v[836]) + (eq0_e968 * s.dn[836][10]));
        let eq0_e970_d_n11: f64 = ((eq0_e968_d_n11 * s.v[836]) + (eq0_e968 * s.dn[836][11]));
        let eq0_e970_d_n12: f64 = ((eq0_e968_d_n12 * s.v[836]) + (eq0_e968 * s.dn[836][12]));
        let eq0_e970_d_b0: f64 = ((eq0_e968_d_b0 * s.v[836]) + (eq0_e968 * s.db[836][0]));
        let eq0_e970_d_b1: f64 = ((eq0_e968_d_b1 * s.v[836]) + (eq0_e968 * s.db[836][1]));
        let eq0_e970_d_b2: f64 = ((eq0_e968_d_b2 * s.v[836]) + (eq0_e968 * s.db[836][2]));
        let eq0_e970_d_b3: f64 = ((eq0_e968_d_b3 * s.v[836]) + (eq0_e968 * s.db[836][3]));
        let eq0_e970_d_b4: f64 = ((eq0_e968_d_b4 * s.v[836]) + (eq0_e968 * s.db[836][4]));
        let eq0_e970_d_b5: f64 = ((eq0_e968_d_b5 * s.v[836]) + (eq0_e968 * s.db[836][5]));
        let eq0_e970_d_b6: f64 = ((eq0_e968_d_b6 * s.v[836]) + (eq0_e968 * s.db[836][6]));
        (eq0_e970, eq0_e970_d_n0, eq0_e970_d_n1, eq0_e970_d_n2, eq0_e970_d_n3, eq0_e970_d_n4, eq0_e970_d_n5, eq0_e970_d_n6, eq0_e970_d_n7, eq0_e970_d_n8, eq0_e970_d_n9, eq0_e970_d_n10, eq0_e970_d_n11, eq0_e970_d_n12, eq0_e970_d_b0, eq0_e970_d_b1, eq0_e970_d_b2, eq0_e970_d_b3, eq0_e970_d_b4, eq0_e970_d_b5, eq0_e970_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e972;
        let eq0_node_derivatives: [f64; 13] = [eq0_e972_d_n0, eq0_e972_d_n1, eq0_e972_d_n2, eq0_e972_d_n3, eq0_e972_d_n4, eq0_e972_d_n5, eq0_e972_d_n6, eq0_e972_d_n7, eq0_e972_d_n8, eq0_e972_d_n9, eq0_e972_d_n10, eq0_e972_d_n11, eq0_e972_d_n12];
        let eq0_branch_derivatives: [f64; 7] = [eq0_e972_d_b0, eq0_e972_d_b1, eq0_e972_d_b2, eq0_e972_d_b3, eq0_e972_d_b4, eq0_e972_d_b5, eq0_e972_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e984, eq1_e984_d_n0, eq1_e984_d_n1, eq1_e984_d_n2, eq1_e984_d_n3, eq1_e984_d_n4, eq1_e984_d_n5, eq1_e984_d_n6, eq1_e984_d_n7, eq1_e984_d_n8, eq1_e984_d_n9, eq1_e984_d_n10, eq1_e984_d_n11, eq1_e984_d_n12, eq1_e984_d_b0, eq1_e984_d_b1, eq1_e984_d_b2, eq1_e984_d_b3, eq1_e984_d_b4, eq1_e984_d_b5, eq1_e984_d_b6,) = {
    if s.b[2715] {
        let eq1_e976: f64 = (s.v[0] * s.v[15]);
        let eq1_e976_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq1_e976_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq1_e976_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq1_e976_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq1_e976_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq1_e976_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq1_e976_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq1_e976_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq1_e976_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq1_e976_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq1_e976_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq1_e976_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq1_e976_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq1_e976_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq1_e976_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq1_e976_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq1_e976_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq1_e976_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq1_e976_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq1_e976_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq1_e978: f64 = (eq1_e976 * p.p32);
        let eq1_e978_d_n0: f64 = (eq1_e976_d_n0 * p.p32);
        let eq1_e978_d_n1: f64 = (eq1_e976_d_n1 * p.p32);
        let eq1_e978_d_n2: f64 = (eq1_e976_d_n2 * p.p32);
        let eq1_e978_d_n3: f64 = (eq1_e976_d_n3 * p.p32);
        let eq1_e978_d_n4: f64 = (eq1_e976_d_n4 * p.p32);
        let eq1_e978_d_n5: f64 = (eq1_e976_d_n5 * p.p32);
        let eq1_e978_d_n6: f64 = (eq1_e976_d_n6 * p.p32);
        let eq1_e978_d_n7: f64 = (eq1_e976_d_n7 * p.p32);
        let eq1_e978_d_n8: f64 = (eq1_e976_d_n8 * p.p32);
        let eq1_e978_d_n9: f64 = (eq1_e976_d_n9 * p.p32);
        let eq1_e978_d_n10: f64 = (eq1_e976_d_n10 * p.p32);
        let eq1_e978_d_n11: f64 = (eq1_e976_d_n11 * p.p32);
        let eq1_e978_d_n12: f64 = (eq1_e976_d_n12 * p.p32);
        let eq1_e978_d_b0: f64 = (eq1_e976_d_b0 * p.p32);
        let eq1_e978_d_b1: f64 = (eq1_e976_d_b1 * p.p32);
        let eq1_e978_d_b2: f64 = (eq1_e976_d_b2 * p.p32);
        let eq1_e978_d_b3: f64 = (eq1_e976_d_b3 * p.p32);
        let eq1_e978_d_b4: f64 = (eq1_e976_d_b4 * p.p32);
        let eq1_e978_d_b5: f64 = (eq1_e976_d_b5 * p.p32);
        let eq1_e978_d_b6: f64 = (eq1_e976_d_b6 * p.p32);
        let eq1_e981: f64 = (s.v[827] + s.v[835]);
        let eq1_e981_d_n0: f64 = (s.dn[827][0] + s.dn[835][0]);
        let eq1_e981_d_n1: f64 = (s.dn[827][1] + s.dn[835][1]);
        let eq1_e981_d_n2: f64 = (s.dn[827][2] + s.dn[835][2]);
        let eq1_e981_d_n3: f64 = (s.dn[827][3] + s.dn[835][3]);
        let eq1_e981_d_n4: f64 = (s.dn[827][4] + s.dn[835][4]);
        let eq1_e981_d_n5: f64 = (s.dn[827][5] + s.dn[835][5]);
        let eq1_e981_d_n6: f64 = (s.dn[827][6] + s.dn[835][6]);
        let eq1_e981_d_n7: f64 = (s.dn[827][7] + s.dn[835][7]);
        let eq1_e981_d_n8: f64 = (s.dn[827][8] + s.dn[835][8]);
        let eq1_e981_d_n9: f64 = (s.dn[827][9] + s.dn[835][9]);
        let eq1_e981_d_n10: f64 = (s.dn[827][10] + s.dn[835][10]);
        let eq1_e981_d_n11: f64 = (s.dn[827][11] + s.dn[835][11]);
        let eq1_e981_d_n12: f64 = (s.dn[827][12] + s.dn[835][12]);
        let eq1_e981_d_b0: f64 = (s.db[827][0] + s.db[835][0]);
        let eq1_e981_d_b1: f64 = (s.db[827][1] + s.db[835][1]);
        let eq1_e981_d_b2: f64 = (s.db[827][2] + s.db[835][2]);
        let eq1_e981_d_b3: f64 = (s.db[827][3] + s.db[835][3]);
        let eq1_e981_d_b4: f64 = (s.db[827][4] + s.db[835][4]);
        let eq1_e981_d_b5: f64 = (s.db[827][5] + s.db[835][5]);
        let eq1_e981_d_b6: f64 = (s.db[827][6] + s.db[835][6]);
        let eq1_e982: f64 = (eq1_e978 * eq1_e981);
        let eq1_e982_d_n0: f64 = ((eq1_e978_d_n0 * eq1_e981) + (eq1_e978 * eq1_e981_d_n0));
        let eq1_e982_d_n1: f64 = ((eq1_e978_d_n1 * eq1_e981) + (eq1_e978 * eq1_e981_d_n1));
        let eq1_e982_d_n2: f64 = ((eq1_e978_d_n2 * eq1_e981) + (eq1_e978 * eq1_e981_d_n2));
        let eq1_e982_d_n3: f64 = ((eq1_e978_d_n3 * eq1_e981) + (eq1_e978 * eq1_e981_d_n3));
        let eq1_e982_d_n4: f64 = ((eq1_e978_d_n4 * eq1_e981) + (eq1_e978 * eq1_e981_d_n4));
        let eq1_e982_d_n5: f64 = ((eq1_e978_d_n5 * eq1_e981) + (eq1_e978 * eq1_e981_d_n5));
        let eq1_e982_d_n6: f64 = ((eq1_e978_d_n6 * eq1_e981) + (eq1_e978 * eq1_e981_d_n6));
        let eq1_e982_d_n7: f64 = ((eq1_e978_d_n7 * eq1_e981) + (eq1_e978 * eq1_e981_d_n7));
        let eq1_e982_d_n8: f64 = ((eq1_e978_d_n8 * eq1_e981) + (eq1_e978 * eq1_e981_d_n8));
        let eq1_e982_d_n9: f64 = ((eq1_e978_d_n9 * eq1_e981) + (eq1_e978 * eq1_e981_d_n9));
        let eq1_e982_d_n10: f64 = ((eq1_e978_d_n10 * eq1_e981) + (eq1_e978 * eq1_e981_d_n10));
        let eq1_e982_d_n11: f64 = ((eq1_e978_d_n11 * eq1_e981) + (eq1_e978 * eq1_e981_d_n11));
        let eq1_e982_d_n12: f64 = ((eq1_e978_d_n12 * eq1_e981) + (eq1_e978 * eq1_e981_d_n12));
        let eq1_e982_d_b0: f64 = ((eq1_e978_d_b0 * eq1_e981) + (eq1_e978 * eq1_e981_d_b0));
        let eq1_e982_d_b1: f64 = ((eq1_e978_d_b1 * eq1_e981) + (eq1_e978 * eq1_e981_d_b1));
        let eq1_e982_d_b2: f64 = ((eq1_e978_d_b2 * eq1_e981) + (eq1_e978 * eq1_e981_d_b2));
        let eq1_e982_d_b3: f64 = ((eq1_e978_d_b3 * eq1_e981) + (eq1_e978 * eq1_e981_d_b3));
        let eq1_e982_d_b4: f64 = ((eq1_e978_d_b4 * eq1_e981) + (eq1_e978 * eq1_e981_d_b4));
        let eq1_e982_d_b5: f64 = ((eq1_e978_d_b5 * eq1_e981) + (eq1_e978 * eq1_e981_d_b5));
        let eq1_e982_d_b6: f64 = ((eq1_e978_d_b6 * eq1_e981) + (eq1_e978 * eq1_e981_d_b6));
        (eq1_e982, eq1_e982_d_n0, eq1_e982_d_n1, eq1_e982_d_n2, eq1_e982_d_n3, eq1_e982_d_n4, eq1_e982_d_n5, eq1_e982_d_n6, eq1_e982_d_n7, eq1_e982_d_n8, eq1_e982_d_n9, eq1_e982_d_n10, eq1_e982_d_n11, eq1_e982_d_n12, eq1_e982_d_b0, eq1_e982_d_b1, eq1_e982_d_b2, eq1_e982_d_b3, eq1_e982_d_b4, eq1_e982_d_b5, eq1_e982_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e984;
        let eq1_node_derivatives: [f64; 13] = [eq1_e984_d_n0, eq1_e984_d_n1, eq1_e984_d_n2, eq1_e984_d_n3, eq1_e984_d_n4, eq1_e984_d_n5, eq1_e984_d_n6, eq1_e984_d_n7, eq1_e984_d_n8, eq1_e984_d_n9, eq1_e984_d_n10, eq1_e984_d_n11, eq1_e984_d_n12];
        let eq1_branch_derivatives: [f64; 7] = [eq1_e984_d_b0, eq1_e984_d_b1, eq1_e984_d_b2, eq1_e984_d_b3, eq1_e984_d_b4, eq1_e984_d_b5, eq1_e984_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e994, eq2_e994_d_n0, eq2_e994_d_n1, eq2_e994_d_n2, eq2_e994_d_n3, eq2_e994_d_n4, eq2_e994_d_n5, eq2_e994_d_n6, eq2_e994_d_n7, eq2_e994_d_n8, eq2_e994_d_n9, eq2_e994_d_n10, eq2_e994_d_n11, eq2_e994_d_n12, eq2_e994_d_b0, eq2_e994_d_b1, eq2_e994_d_b2, eq2_e994_d_b3, eq2_e994_d_b4, eq2_e994_d_b5, eq2_e994_d_b6,) = {
    if s.b[2715] {
        let eq2_e988: f64 = (s.v[0] * s.v[15]);
        let eq2_e988_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq2_e988_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq2_e988_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq2_e988_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq2_e988_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq2_e988_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq2_e988_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq2_e988_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq2_e988_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq2_e988_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq2_e988_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq2_e988_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq2_e988_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq2_e988_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq2_e988_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq2_e988_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq2_e988_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq2_e988_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq2_e988_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq2_e988_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq2_e990: f64 = (eq2_e988 * p.p32);
        let eq2_e990_d_n0: f64 = (eq2_e988_d_n0 * p.p32);
        let eq2_e990_d_n1: f64 = (eq2_e988_d_n1 * p.p32);
        let eq2_e990_d_n2: f64 = (eq2_e988_d_n2 * p.p32);
        let eq2_e990_d_n3: f64 = (eq2_e988_d_n3 * p.p32);
        let eq2_e990_d_n4: f64 = (eq2_e988_d_n4 * p.p32);
        let eq2_e990_d_n5: f64 = (eq2_e988_d_n5 * p.p32);
        let eq2_e990_d_n6: f64 = (eq2_e988_d_n6 * p.p32);
        let eq2_e990_d_n7: f64 = (eq2_e988_d_n7 * p.p32);
        let eq2_e990_d_n8: f64 = (eq2_e988_d_n8 * p.p32);
        let eq2_e990_d_n9: f64 = (eq2_e988_d_n9 * p.p32);
        let eq2_e990_d_n10: f64 = (eq2_e988_d_n10 * p.p32);
        let eq2_e990_d_n11: f64 = (eq2_e988_d_n11 * p.p32);
        let eq2_e990_d_n12: f64 = (eq2_e988_d_n12 * p.p32);
        let eq2_e990_d_b0: f64 = (eq2_e988_d_b0 * p.p32);
        let eq2_e990_d_b1: f64 = (eq2_e988_d_b1 * p.p32);
        let eq2_e990_d_b2: f64 = (eq2_e988_d_b2 * p.p32);
        let eq2_e990_d_b3: f64 = (eq2_e988_d_b3 * p.p32);
        let eq2_e990_d_b4: f64 = (eq2_e988_d_b4 * p.p32);
        let eq2_e990_d_b5: f64 = (eq2_e988_d_b5 * p.p32);
        let eq2_e990_d_b6: f64 = (eq2_e988_d_b6 * p.p32);
        let eq2_e992: f64 = (eq2_e990 * s.v[830]);
        let eq2_e992_d_n0: f64 = ((eq2_e990_d_n0 * s.v[830]) + (eq2_e990 * s.dn[830][0]));
        let eq2_e992_d_n1: f64 = ((eq2_e990_d_n1 * s.v[830]) + (eq2_e990 * s.dn[830][1]));
        let eq2_e992_d_n2: f64 = ((eq2_e990_d_n2 * s.v[830]) + (eq2_e990 * s.dn[830][2]));
        let eq2_e992_d_n3: f64 = ((eq2_e990_d_n3 * s.v[830]) + (eq2_e990 * s.dn[830][3]));
        let eq2_e992_d_n4: f64 = ((eq2_e990_d_n4 * s.v[830]) + (eq2_e990 * s.dn[830][4]));
        let eq2_e992_d_n5: f64 = ((eq2_e990_d_n5 * s.v[830]) + (eq2_e990 * s.dn[830][5]));
        let eq2_e992_d_n6: f64 = ((eq2_e990_d_n6 * s.v[830]) + (eq2_e990 * s.dn[830][6]));
        let eq2_e992_d_n7: f64 = ((eq2_e990_d_n7 * s.v[830]) + (eq2_e990 * s.dn[830][7]));
        let eq2_e992_d_n8: f64 = ((eq2_e990_d_n8 * s.v[830]) + (eq2_e990 * s.dn[830][8]));
        let eq2_e992_d_n9: f64 = ((eq2_e990_d_n9 * s.v[830]) + (eq2_e990 * s.dn[830][9]));
        let eq2_e992_d_n10: f64 = ((eq2_e990_d_n10 * s.v[830]) + (eq2_e990 * s.dn[830][10]));
        let eq2_e992_d_n11: f64 = ((eq2_e990_d_n11 * s.v[830]) + (eq2_e990 * s.dn[830][11]));
        let eq2_e992_d_n12: f64 = ((eq2_e990_d_n12 * s.v[830]) + (eq2_e990 * s.dn[830][12]));
        let eq2_e992_d_b0: f64 = ((eq2_e990_d_b0 * s.v[830]) + (eq2_e990 * s.db[830][0]));
        let eq2_e992_d_b1: f64 = ((eq2_e990_d_b1 * s.v[830]) + (eq2_e990 * s.db[830][1]));
        let eq2_e992_d_b2: f64 = ((eq2_e990_d_b2 * s.v[830]) + (eq2_e990 * s.db[830][2]));
        let eq2_e992_d_b3: f64 = ((eq2_e990_d_b3 * s.v[830]) + (eq2_e990 * s.db[830][3]));
        let eq2_e992_d_b4: f64 = ((eq2_e990_d_b4 * s.v[830]) + (eq2_e990 * s.db[830][4]));
        let eq2_e992_d_b5: f64 = ((eq2_e990_d_b5 * s.v[830]) + (eq2_e990 * s.db[830][5]));
        let eq2_e992_d_b6: f64 = ((eq2_e990_d_b6 * s.v[830]) + (eq2_e990 * s.db[830][6]));
        (eq2_e992, eq2_e992_d_n0, eq2_e992_d_n1, eq2_e992_d_n2, eq2_e992_d_n3, eq2_e992_d_n4, eq2_e992_d_n5, eq2_e992_d_n6, eq2_e992_d_n7, eq2_e992_d_n8, eq2_e992_d_n9, eq2_e992_d_n10, eq2_e992_d_n11, eq2_e992_d_n12, eq2_e992_d_b0, eq2_e992_d_b1, eq2_e992_d_b2, eq2_e992_d_b3, eq2_e992_d_b4, eq2_e992_d_b5, eq2_e992_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e994;
        let eq2_node_derivatives: [f64; 13] = [eq2_e994_d_n0, eq2_e994_d_n1, eq2_e994_d_n2, eq2_e994_d_n3, eq2_e994_d_n4, eq2_e994_d_n5, eq2_e994_d_n6, eq2_e994_d_n7, eq2_e994_d_n8, eq2_e994_d_n9, eq2_e994_d_n10, eq2_e994_d_n11, eq2_e994_d_n12];
        let eq2_branch_derivatives: [f64; 7] = [eq2_e994_d_b0, eq2_e994_d_b1, eq2_e994_d_b2, eq2_e994_d_b3, eq2_e994_d_b4, eq2_e994_d_b5, eq2_e994_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e1004, eq3_e1004_d_n0, eq3_e1004_d_n1, eq3_e1004_d_n2, eq3_e1004_d_n3, eq3_e1004_d_n4, eq3_e1004_d_n5, eq3_e1004_d_n6, eq3_e1004_d_n7, eq3_e1004_d_n8, eq3_e1004_d_n9, eq3_e1004_d_n10, eq3_e1004_d_n11, eq3_e1004_d_n12, eq3_e1004_d_b0, eq3_e1004_d_b1, eq3_e1004_d_b2, eq3_e1004_d_b3, eq3_e1004_d_b4, eq3_e1004_d_b5, eq3_e1004_d_b6,) = {
    if s.b[2715] {
        let eq3_e998: f64 = (s.v[0] * s.v[15]);
        let eq3_e998_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq3_e998_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq3_e998_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq3_e998_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq3_e998_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq3_e998_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq3_e998_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq3_e998_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq3_e998_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq3_e998_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq3_e998_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq3_e998_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq3_e998_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq3_e998_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq3_e998_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq3_e998_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq3_e998_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq3_e998_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq3_e998_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq3_e998_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq3_e1000: f64 = (eq3_e998 * p.p32);
        let eq3_e1000_d_n0: f64 = (eq3_e998_d_n0 * p.p32);
        let eq3_e1000_d_n1: f64 = (eq3_e998_d_n1 * p.p32);
        let eq3_e1000_d_n2: f64 = (eq3_e998_d_n2 * p.p32);
        let eq3_e1000_d_n3: f64 = (eq3_e998_d_n3 * p.p32);
        let eq3_e1000_d_n4: f64 = (eq3_e998_d_n4 * p.p32);
        let eq3_e1000_d_n5: f64 = (eq3_e998_d_n5 * p.p32);
        let eq3_e1000_d_n6: f64 = (eq3_e998_d_n6 * p.p32);
        let eq3_e1000_d_n7: f64 = (eq3_e998_d_n7 * p.p32);
        let eq3_e1000_d_n8: f64 = (eq3_e998_d_n8 * p.p32);
        let eq3_e1000_d_n9: f64 = (eq3_e998_d_n9 * p.p32);
        let eq3_e1000_d_n10: f64 = (eq3_e998_d_n10 * p.p32);
        let eq3_e1000_d_n11: f64 = (eq3_e998_d_n11 * p.p32);
        let eq3_e1000_d_n12: f64 = (eq3_e998_d_n12 * p.p32);
        let eq3_e1000_d_b0: f64 = (eq3_e998_d_b0 * p.p32);
        let eq3_e1000_d_b1: f64 = (eq3_e998_d_b1 * p.p32);
        let eq3_e1000_d_b2: f64 = (eq3_e998_d_b2 * p.p32);
        let eq3_e1000_d_b3: f64 = (eq3_e998_d_b3 * p.p32);
        let eq3_e1000_d_b4: f64 = (eq3_e998_d_b4 * p.p32);
        let eq3_e1000_d_b5: f64 = (eq3_e998_d_b5 * p.p32);
        let eq3_e1000_d_b6: f64 = (eq3_e998_d_b6 * p.p32);
        let eq3_e1002: f64 = (eq3_e1000 * s.v[831]);
        let eq3_e1002_d_n0: f64 = ((eq3_e1000_d_n0 * s.v[831]) + (eq3_e1000 * s.dn[831][0]));
        let eq3_e1002_d_n1: f64 = ((eq3_e1000_d_n1 * s.v[831]) + (eq3_e1000 * s.dn[831][1]));
        let eq3_e1002_d_n2: f64 = ((eq3_e1000_d_n2 * s.v[831]) + (eq3_e1000 * s.dn[831][2]));
        let eq3_e1002_d_n3: f64 = ((eq3_e1000_d_n3 * s.v[831]) + (eq3_e1000 * s.dn[831][3]));
        let eq3_e1002_d_n4: f64 = ((eq3_e1000_d_n4 * s.v[831]) + (eq3_e1000 * s.dn[831][4]));
        let eq3_e1002_d_n5: f64 = ((eq3_e1000_d_n5 * s.v[831]) + (eq3_e1000 * s.dn[831][5]));
        let eq3_e1002_d_n6: f64 = ((eq3_e1000_d_n6 * s.v[831]) + (eq3_e1000 * s.dn[831][6]));
        let eq3_e1002_d_n7: f64 = ((eq3_e1000_d_n7 * s.v[831]) + (eq3_e1000 * s.dn[831][7]));
        let eq3_e1002_d_n8: f64 = ((eq3_e1000_d_n8 * s.v[831]) + (eq3_e1000 * s.dn[831][8]));
        let eq3_e1002_d_n9: f64 = ((eq3_e1000_d_n9 * s.v[831]) + (eq3_e1000 * s.dn[831][9]));
        let eq3_e1002_d_n10: f64 = ((eq3_e1000_d_n10 * s.v[831]) + (eq3_e1000 * s.dn[831][10]));
        let eq3_e1002_d_n11: f64 = ((eq3_e1000_d_n11 * s.v[831]) + (eq3_e1000 * s.dn[831][11]));
        let eq3_e1002_d_n12: f64 = ((eq3_e1000_d_n12 * s.v[831]) + (eq3_e1000 * s.dn[831][12]));
        let eq3_e1002_d_b0: f64 = ((eq3_e1000_d_b0 * s.v[831]) + (eq3_e1000 * s.db[831][0]));
        let eq3_e1002_d_b1: f64 = ((eq3_e1000_d_b1 * s.v[831]) + (eq3_e1000 * s.db[831][1]));
        let eq3_e1002_d_b2: f64 = ((eq3_e1000_d_b2 * s.v[831]) + (eq3_e1000 * s.db[831][2]));
        let eq3_e1002_d_b3: f64 = ((eq3_e1000_d_b3 * s.v[831]) + (eq3_e1000 * s.db[831][3]));
        let eq3_e1002_d_b4: f64 = ((eq3_e1000_d_b4 * s.v[831]) + (eq3_e1000 * s.db[831][4]));
        let eq3_e1002_d_b5: f64 = ((eq3_e1000_d_b5 * s.v[831]) + (eq3_e1000 * s.db[831][5]));
        let eq3_e1002_d_b6: f64 = ((eq3_e1000_d_b6 * s.v[831]) + (eq3_e1000 * s.db[831][6]));
        (eq3_e1002, eq3_e1002_d_n0, eq3_e1002_d_n1, eq3_e1002_d_n2, eq3_e1002_d_n3, eq3_e1002_d_n4, eq3_e1002_d_n5, eq3_e1002_d_n6, eq3_e1002_d_n7, eq3_e1002_d_n8, eq3_e1002_d_n9, eq3_e1002_d_n10, eq3_e1002_d_n11, eq3_e1002_d_n12, eq3_e1002_d_b0, eq3_e1002_d_b1, eq3_e1002_d_b2, eq3_e1002_d_b3, eq3_e1002_d_b4, eq3_e1002_d_b5, eq3_e1002_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e1004;
        let eq3_node_derivatives: [f64; 13] = [eq3_e1004_d_n0, eq3_e1004_d_n1, eq3_e1004_d_n2, eq3_e1004_d_n3, eq3_e1004_d_n4, eq3_e1004_d_n5, eq3_e1004_d_n6, eq3_e1004_d_n7, eq3_e1004_d_n8, eq3_e1004_d_n9, eq3_e1004_d_n10, eq3_e1004_d_n11, eq3_e1004_d_n12];
        let eq3_branch_derivatives: [f64; 7] = [eq3_e1004_d_b0, eq3_e1004_d_b1, eq3_e1004_d_b2, eq3_e1004_d_b3, eq3_e1004_d_b4, eq3_e1004_d_b5, eq3_e1004_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e1015, eq4_e1015_d_n0, eq4_e1015_d_n1, eq4_e1015_d_n2, eq4_e1015_d_n3, eq4_e1015_d_n4, eq4_e1015_d_n5, eq4_e1015_d_n6, eq4_e1015_d_n7, eq4_e1015_d_n8, eq4_e1015_d_n9, eq4_e1015_d_n10, eq4_e1015_d_n11, eq4_e1015_d_n12, eq4_e1015_d_b0, eq4_e1015_d_b1, eq4_e1015_d_b2, eq4_e1015_d_b3, eq4_e1015_d_b4, eq4_e1015_d_b5, eq4_e1015_d_b6,) = {
    if (!s.b[2715]) {
        let eq4_e1009: f64 = (s.v[0] * s.v[15]);
        let eq4_e1009_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq4_e1009_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq4_e1009_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq4_e1009_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq4_e1009_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq4_e1009_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq4_e1009_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq4_e1009_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq4_e1009_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq4_e1009_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq4_e1009_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq4_e1009_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq4_e1009_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq4_e1009_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq4_e1009_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq4_e1009_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq4_e1009_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq4_e1009_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq4_e1009_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq4_e1009_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq4_e1011: f64 = (eq4_e1009 * p.p32);
        let eq4_e1011_d_n0: f64 = (eq4_e1009_d_n0 * p.p32);
        let eq4_e1011_d_n1: f64 = (eq4_e1009_d_n1 * p.p32);
        let eq4_e1011_d_n2: f64 = (eq4_e1009_d_n2 * p.p32);
        let eq4_e1011_d_n3: f64 = (eq4_e1009_d_n3 * p.p32);
        let eq4_e1011_d_n4: f64 = (eq4_e1009_d_n4 * p.p32);
        let eq4_e1011_d_n5: f64 = (eq4_e1009_d_n5 * p.p32);
        let eq4_e1011_d_n6: f64 = (eq4_e1009_d_n6 * p.p32);
        let eq4_e1011_d_n7: f64 = (eq4_e1009_d_n7 * p.p32);
        let eq4_e1011_d_n8: f64 = (eq4_e1009_d_n8 * p.p32);
        let eq4_e1011_d_n9: f64 = (eq4_e1009_d_n9 * p.p32);
        let eq4_e1011_d_n10: f64 = (eq4_e1009_d_n10 * p.p32);
        let eq4_e1011_d_n11: f64 = (eq4_e1009_d_n11 * p.p32);
        let eq4_e1011_d_n12: f64 = (eq4_e1009_d_n12 * p.p32);
        let eq4_e1011_d_b0: f64 = (eq4_e1009_d_b0 * p.p32);
        let eq4_e1011_d_b1: f64 = (eq4_e1009_d_b1 * p.p32);
        let eq4_e1011_d_b2: f64 = (eq4_e1009_d_b2 * p.p32);
        let eq4_e1011_d_b3: f64 = (eq4_e1009_d_b3 * p.p32);
        let eq4_e1011_d_b4: f64 = (eq4_e1009_d_b4 * p.p32);
        let eq4_e1011_d_b5: f64 = (eq4_e1009_d_b5 * p.p32);
        let eq4_e1011_d_b6: f64 = (eq4_e1009_d_b6 * p.p32);
        let eq4_e1013: f64 = (eq4_e1011 * s.v[836]);
        let eq4_e1013_d_n0: f64 = ((eq4_e1011_d_n0 * s.v[836]) + (eq4_e1011 * s.dn[836][0]));
        let eq4_e1013_d_n1: f64 = ((eq4_e1011_d_n1 * s.v[836]) + (eq4_e1011 * s.dn[836][1]));
        let eq4_e1013_d_n2: f64 = ((eq4_e1011_d_n2 * s.v[836]) + (eq4_e1011 * s.dn[836][2]));
        let eq4_e1013_d_n3: f64 = ((eq4_e1011_d_n3 * s.v[836]) + (eq4_e1011 * s.dn[836][3]));
        let eq4_e1013_d_n4: f64 = ((eq4_e1011_d_n4 * s.v[836]) + (eq4_e1011 * s.dn[836][4]));
        let eq4_e1013_d_n5: f64 = ((eq4_e1011_d_n5 * s.v[836]) + (eq4_e1011 * s.dn[836][5]));
        let eq4_e1013_d_n6: f64 = ((eq4_e1011_d_n6 * s.v[836]) + (eq4_e1011 * s.dn[836][6]));
        let eq4_e1013_d_n7: f64 = ((eq4_e1011_d_n7 * s.v[836]) + (eq4_e1011 * s.dn[836][7]));
        let eq4_e1013_d_n8: f64 = ((eq4_e1011_d_n8 * s.v[836]) + (eq4_e1011 * s.dn[836][8]));
        let eq4_e1013_d_n9: f64 = ((eq4_e1011_d_n9 * s.v[836]) + (eq4_e1011 * s.dn[836][9]));
        let eq4_e1013_d_n10: f64 = ((eq4_e1011_d_n10 * s.v[836]) + (eq4_e1011 * s.dn[836][10]));
        let eq4_e1013_d_n11: f64 = ((eq4_e1011_d_n11 * s.v[836]) + (eq4_e1011 * s.dn[836][11]));
        let eq4_e1013_d_n12: f64 = ((eq4_e1011_d_n12 * s.v[836]) + (eq4_e1011 * s.dn[836][12]));
        let eq4_e1013_d_b0: f64 = ((eq4_e1011_d_b0 * s.v[836]) + (eq4_e1011 * s.db[836][0]));
        let eq4_e1013_d_b1: f64 = ((eq4_e1011_d_b1 * s.v[836]) + (eq4_e1011 * s.db[836][1]));
        let eq4_e1013_d_b2: f64 = ((eq4_e1011_d_b2 * s.v[836]) + (eq4_e1011 * s.db[836][2]));
        let eq4_e1013_d_b3: f64 = ((eq4_e1011_d_b3 * s.v[836]) + (eq4_e1011 * s.db[836][3]));
        let eq4_e1013_d_b4: f64 = ((eq4_e1011_d_b4 * s.v[836]) + (eq4_e1011 * s.db[836][4]));
        let eq4_e1013_d_b5: f64 = ((eq4_e1011_d_b5 * s.v[836]) + (eq4_e1011 * s.db[836][5]));
        let eq4_e1013_d_b6: f64 = ((eq4_e1011_d_b6 * s.v[836]) + (eq4_e1011 * s.db[836][6]));
        (eq4_e1013, eq4_e1013_d_n0, eq4_e1013_d_n1, eq4_e1013_d_n2, eq4_e1013_d_n3, eq4_e1013_d_n4, eq4_e1013_d_n5, eq4_e1013_d_n6, eq4_e1013_d_n7, eq4_e1013_d_n8, eq4_e1013_d_n9, eq4_e1013_d_n10, eq4_e1013_d_n11, eq4_e1013_d_n12, eq4_e1013_d_b0, eq4_e1013_d_b1, eq4_e1013_d_b2, eq4_e1013_d_b3, eq4_e1013_d_b4, eq4_e1013_d_b5, eq4_e1013_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e1015;
        let eq4_node_derivatives: [f64; 13] = [eq4_e1015_d_n0, eq4_e1015_d_n1, eq4_e1015_d_n2, eq4_e1015_d_n3, eq4_e1015_d_n4, eq4_e1015_d_n5, eq4_e1015_d_n6, eq4_e1015_d_n7, eq4_e1015_d_n8, eq4_e1015_d_n9, eq4_e1015_d_n10, eq4_e1015_d_n11, eq4_e1015_d_n12];
        let eq4_branch_derivatives: [f64; 7] = [eq4_e1015_d_b0, eq4_e1015_d_b1, eq4_e1015_d_b2, eq4_e1015_d_b3, eq4_e1015_d_b4, eq4_e1015_d_b5, eq4_e1015_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq5_e1028, eq5_e1028_d_n0, eq5_e1028_d_n1, eq5_e1028_d_n2, eq5_e1028_d_n3, eq5_e1028_d_n4, eq5_e1028_d_n5, eq5_e1028_d_n6, eq5_e1028_d_n7, eq5_e1028_d_n8, eq5_e1028_d_n9, eq5_e1028_d_n10, eq5_e1028_d_n11, eq5_e1028_d_n12, eq5_e1028_d_b0, eq5_e1028_d_b1, eq5_e1028_d_b2, eq5_e1028_d_b3, eq5_e1028_d_b4, eq5_e1028_d_b5, eq5_e1028_d_b6,) = {
    if (!s.b[2715]) {
        let eq5_e1020: f64 = (s.v[0] * s.v[15]);
        let eq5_e1020_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq5_e1020_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq5_e1020_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq5_e1020_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq5_e1020_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq5_e1020_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq5_e1020_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq5_e1020_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq5_e1020_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq5_e1020_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq5_e1020_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq5_e1020_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq5_e1020_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq5_e1020_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq5_e1020_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq5_e1020_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq5_e1020_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq5_e1020_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq5_e1020_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq5_e1020_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq5_e1022: f64 = (eq5_e1020 * p.p32);
        let eq5_e1022_d_n0: f64 = (eq5_e1020_d_n0 * p.p32);
        let eq5_e1022_d_n1: f64 = (eq5_e1020_d_n1 * p.p32);
        let eq5_e1022_d_n2: f64 = (eq5_e1020_d_n2 * p.p32);
        let eq5_e1022_d_n3: f64 = (eq5_e1020_d_n3 * p.p32);
        let eq5_e1022_d_n4: f64 = (eq5_e1020_d_n4 * p.p32);
        let eq5_e1022_d_n5: f64 = (eq5_e1020_d_n5 * p.p32);
        let eq5_e1022_d_n6: f64 = (eq5_e1020_d_n6 * p.p32);
        let eq5_e1022_d_n7: f64 = (eq5_e1020_d_n7 * p.p32);
        let eq5_e1022_d_n8: f64 = (eq5_e1020_d_n8 * p.p32);
        let eq5_e1022_d_n9: f64 = (eq5_e1020_d_n9 * p.p32);
        let eq5_e1022_d_n10: f64 = (eq5_e1020_d_n10 * p.p32);
        let eq5_e1022_d_n11: f64 = (eq5_e1020_d_n11 * p.p32);
        let eq5_e1022_d_n12: f64 = (eq5_e1020_d_n12 * p.p32);
        let eq5_e1022_d_b0: f64 = (eq5_e1020_d_b0 * p.p32);
        let eq5_e1022_d_b1: f64 = (eq5_e1020_d_b1 * p.p32);
        let eq5_e1022_d_b2: f64 = (eq5_e1020_d_b2 * p.p32);
        let eq5_e1022_d_b3: f64 = (eq5_e1020_d_b3 * p.p32);
        let eq5_e1022_d_b4: f64 = (eq5_e1020_d_b4 * p.p32);
        let eq5_e1022_d_b5: f64 = (eq5_e1020_d_b5 * p.p32);
        let eq5_e1022_d_b6: f64 = (eq5_e1020_d_b6 * p.p32);
        let eq5_e1025: f64 = (s.v[827] + s.v[835]);
        let eq5_e1025_d_n0: f64 = (s.dn[827][0] + s.dn[835][0]);
        let eq5_e1025_d_n1: f64 = (s.dn[827][1] + s.dn[835][1]);
        let eq5_e1025_d_n2: f64 = (s.dn[827][2] + s.dn[835][2]);
        let eq5_e1025_d_n3: f64 = (s.dn[827][3] + s.dn[835][3]);
        let eq5_e1025_d_n4: f64 = (s.dn[827][4] + s.dn[835][4]);
        let eq5_e1025_d_n5: f64 = (s.dn[827][5] + s.dn[835][5]);
        let eq5_e1025_d_n6: f64 = (s.dn[827][6] + s.dn[835][6]);
        let eq5_e1025_d_n7: f64 = (s.dn[827][7] + s.dn[835][7]);
        let eq5_e1025_d_n8: f64 = (s.dn[827][8] + s.dn[835][8]);
        let eq5_e1025_d_n9: f64 = (s.dn[827][9] + s.dn[835][9]);
        let eq5_e1025_d_n10: f64 = (s.dn[827][10] + s.dn[835][10]);
        let eq5_e1025_d_n11: f64 = (s.dn[827][11] + s.dn[835][11]);
        let eq5_e1025_d_n12: f64 = (s.dn[827][12] + s.dn[835][12]);
        let eq5_e1025_d_b0: f64 = (s.db[827][0] + s.db[835][0]);
        let eq5_e1025_d_b1: f64 = (s.db[827][1] + s.db[835][1]);
        let eq5_e1025_d_b2: f64 = (s.db[827][2] + s.db[835][2]);
        let eq5_e1025_d_b3: f64 = (s.db[827][3] + s.db[835][3]);
        let eq5_e1025_d_b4: f64 = (s.db[827][4] + s.db[835][4]);
        let eq5_e1025_d_b5: f64 = (s.db[827][5] + s.db[835][5]);
        let eq5_e1025_d_b6: f64 = (s.db[827][6] + s.db[835][6]);
        let eq5_e1026: f64 = (eq5_e1022 * eq5_e1025);
        let eq5_e1026_d_n0: f64 = ((eq5_e1022_d_n0 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n0));
        let eq5_e1026_d_n1: f64 = ((eq5_e1022_d_n1 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n1));
        let eq5_e1026_d_n2: f64 = ((eq5_e1022_d_n2 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n2));
        let eq5_e1026_d_n3: f64 = ((eq5_e1022_d_n3 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n3));
        let eq5_e1026_d_n4: f64 = ((eq5_e1022_d_n4 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n4));
        let eq5_e1026_d_n5: f64 = ((eq5_e1022_d_n5 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n5));
        let eq5_e1026_d_n6: f64 = ((eq5_e1022_d_n6 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n6));
        let eq5_e1026_d_n7: f64 = ((eq5_e1022_d_n7 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n7));
        let eq5_e1026_d_n8: f64 = ((eq5_e1022_d_n8 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n8));
        let eq5_e1026_d_n9: f64 = ((eq5_e1022_d_n9 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n9));
        let eq5_e1026_d_n10: f64 = ((eq5_e1022_d_n10 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n10));
        let eq5_e1026_d_n11: f64 = ((eq5_e1022_d_n11 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n11));
        let eq5_e1026_d_n12: f64 = ((eq5_e1022_d_n12 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_n12));
        let eq5_e1026_d_b0: f64 = ((eq5_e1022_d_b0 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b0));
        let eq5_e1026_d_b1: f64 = ((eq5_e1022_d_b1 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b1));
        let eq5_e1026_d_b2: f64 = ((eq5_e1022_d_b2 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b2));
        let eq5_e1026_d_b3: f64 = ((eq5_e1022_d_b3 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b3));
        let eq5_e1026_d_b4: f64 = ((eq5_e1022_d_b4 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b4));
        let eq5_e1026_d_b5: f64 = ((eq5_e1022_d_b5 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b5));
        let eq5_e1026_d_b6: f64 = ((eq5_e1022_d_b6 * eq5_e1025) + (eq5_e1022 * eq5_e1025_d_b6));
        (eq5_e1026, eq5_e1026_d_n0, eq5_e1026_d_n1, eq5_e1026_d_n2, eq5_e1026_d_n3, eq5_e1026_d_n4, eq5_e1026_d_n5, eq5_e1026_d_n6, eq5_e1026_d_n7, eq5_e1026_d_n8, eq5_e1026_d_n9, eq5_e1026_d_n10, eq5_e1026_d_n11, eq5_e1026_d_n12, eq5_e1026_d_b0, eq5_e1026_d_b1, eq5_e1026_d_b2, eq5_e1026_d_b3, eq5_e1026_d_b4, eq5_e1026_d_b5, eq5_e1026_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1028;
        let eq5_node_derivatives: [f64; 13] = [eq5_e1028_d_n0, eq5_e1028_d_n1, eq5_e1028_d_n2, eq5_e1028_d_n3, eq5_e1028_d_n4, eq5_e1028_d_n5, eq5_e1028_d_n6, eq5_e1028_d_n7, eq5_e1028_d_n8, eq5_e1028_d_n9, eq5_e1028_d_n10, eq5_e1028_d_n11, eq5_e1028_d_n12];
        let eq5_branch_derivatives: [f64; 7] = [eq5_e1028_d_b0, eq5_e1028_d_b1, eq5_e1028_d_b2, eq5_e1028_d_b3, eq5_e1028_d_b4, eq5_e1028_d_b5, eq5_e1028_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
        let (eq6_e1039, eq6_e1039_d_n0, eq6_e1039_d_n1, eq6_e1039_d_n2, eq6_e1039_d_n3, eq6_e1039_d_n4, eq6_e1039_d_n5, eq6_e1039_d_n6, eq6_e1039_d_n7, eq6_e1039_d_n8, eq6_e1039_d_n9, eq6_e1039_d_n10, eq6_e1039_d_n11, eq6_e1039_d_n12, eq6_e1039_d_b0, eq6_e1039_d_b1, eq6_e1039_d_b2, eq6_e1039_d_b3, eq6_e1039_d_b4, eq6_e1039_d_b5, eq6_e1039_d_b6,) = {
    if (!s.b[2715]) {
        let eq6_e1033: f64 = (s.v[0] * s.v[15]);
        let eq6_e1033_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq6_e1033_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq6_e1033_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq6_e1033_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq6_e1033_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq6_e1033_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq6_e1033_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq6_e1033_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq6_e1033_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq6_e1033_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq6_e1033_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq6_e1033_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq6_e1033_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq6_e1033_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq6_e1033_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq6_e1033_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq6_e1033_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq6_e1033_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq6_e1033_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq6_e1033_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq6_e1035: f64 = (eq6_e1033 * p.p32);
        let eq6_e1035_d_n0: f64 = (eq6_e1033_d_n0 * p.p32);
        let eq6_e1035_d_n1: f64 = (eq6_e1033_d_n1 * p.p32);
        let eq6_e1035_d_n2: f64 = (eq6_e1033_d_n2 * p.p32);
        let eq6_e1035_d_n3: f64 = (eq6_e1033_d_n3 * p.p32);
        let eq6_e1035_d_n4: f64 = (eq6_e1033_d_n4 * p.p32);
        let eq6_e1035_d_n5: f64 = (eq6_e1033_d_n5 * p.p32);
        let eq6_e1035_d_n6: f64 = (eq6_e1033_d_n6 * p.p32);
        let eq6_e1035_d_n7: f64 = (eq6_e1033_d_n7 * p.p32);
        let eq6_e1035_d_n8: f64 = (eq6_e1033_d_n8 * p.p32);
        let eq6_e1035_d_n9: f64 = (eq6_e1033_d_n9 * p.p32);
        let eq6_e1035_d_n10: f64 = (eq6_e1033_d_n10 * p.p32);
        let eq6_e1035_d_n11: f64 = (eq6_e1033_d_n11 * p.p32);
        let eq6_e1035_d_n12: f64 = (eq6_e1033_d_n12 * p.p32);
        let eq6_e1035_d_b0: f64 = (eq6_e1033_d_b0 * p.p32);
        let eq6_e1035_d_b1: f64 = (eq6_e1033_d_b1 * p.p32);
        let eq6_e1035_d_b2: f64 = (eq6_e1033_d_b2 * p.p32);
        let eq6_e1035_d_b3: f64 = (eq6_e1033_d_b3 * p.p32);
        let eq6_e1035_d_b4: f64 = (eq6_e1033_d_b4 * p.p32);
        let eq6_e1035_d_b5: f64 = (eq6_e1033_d_b5 * p.p32);
        let eq6_e1035_d_b6: f64 = (eq6_e1033_d_b6 * p.p32);
        let eq6_e1037: f64 = (eq6_e1035 * s.v[830]);
        let eq6_e1037_d_n0: f64 = ((eq6_e1035_d_n0 * s.v[830]) + (eq6_e1035 * s.dn[830][0]));
        let eq6_e1037_d_n1: f64 = ((eq6_e1035_d_n1 * s.v[830]) + (eq6_e1035 * s.dn[830][1]));
        let eq6_e1037_d_n2: f64 = ((eq6_e1035_d_n2 * s.v[830]) + (eq6_e1035 * s.dn[830][2]));
        let eq6_e1037_d_n3: f64 = ((eq6_e1035_d_n3 * s.v[830]) + (eq6_e1035 * s.dn[830][3]));
        let eq6_e1037_d_n4: f64 = ((eq6_e1035_d_n4 * s.v[830]) + (eq6_e1035 * s.dn[830][4]));
        let eq6_e1037_d_n5: f64 = ((eq6_e1035_d_n5 * s.v[830]) + (eq6_e1035 * s.dn[830][5]));
        let eq6_e1037_d_n6: f64 = ((eq6_e1035_d_n6 * s.v[830]) + (eq6_e1035 * s.dn[830][6]));
        let eq6_e1037_d_n7: f64 = ((eq6_e1035_d_n7 * s.v[830]) + (eq6_e1035 * s.dn[830][7]));
        let eq6_e1037_d_n8: f64 = ((eq6_e1035_d_n8 * s.v[830]) + (eq6_e1035 * s.dn[830][8]));
        let eq6_e1037_d_n9: f64 = ((eq6_e1035_d_n9 * s.v[830]) + (eq6_e1035 * s.dn[830][9]));
        let eq6_e1037_d_n10: f64 = ((eq6_e1035_d_n10 * s.v[830]) + (eq6_e1035 * s.dn[830][10]));
        let eq6_e1037_d_n11: f64 = ((eq6_e1035_d_n11 * s.v[830]) + (eq6_e1035 * s.dn[830][11]));
        let eq6_e1037_d_n12: f64 = ((eq6_e1035_d_n12 * s.v[830]) + (eq6_e1035 * s.dn[830][12]));
        let eq6_e1037_d_b0: f64 = ((eq6_e1035_d_b0 * s.v[830]) + (eq6_e1035 * s.db[830][0]));
        let eq6_e1037_d_b1: f64 = ((eq6_e1035_d_b1 * s.v[830]) + (eq6_e1035 * s.db[830][1]));
        let eq6_e1037_d_b2: f64 = ((eq6_e1035_d_b2 * s.v[830]) + (eq6_e1035 * s.db[830][2]));
        let eq6_e1037_d_b3: f64 = ((eq6_e1035_d_b3 * s.v[830]) + (eq6_e1035 * s.db[830][3]));
        let eq6_e1037_d_b4: f64 = ((eq6_e1035_d_b4 * s.v[830]) + (eq6_e1035 * s.db[830][4]));
        let eq6_e1037_d_b5: f64 = ((eq6_e1035_d_b5 * s.v[830]) + (eq6_e1035 * s.db[830][5]));
        let eq6_e1037_d_b6: f64 = ((eq6_e1035_d_b6 * s.v[830]) + (eq6_e1035 * s.db[830][6]));
        (eq6_e1037, eq6_e1037_d_n0, eq6_e1037_d_n1, eq6_e1037_d_n2, eq6_e1037_d_n3, eq6_e1037_d_n4, eq6_e1037_d_n5, eq6_e1037_d_n6, eq6_e1037_d_n7, eq6_e1037_d_n8, eq6_e1037_d_n9, eq6_e1037_d_n10, eq6_e1037_d_n11, eq6_e1037_d_n12, eq6_e1037_d_b0, eq6_e1037_d_b1, eq6_e1037_d_b2, eq6_e1037_d_b3, eq6_e1037_d_b4, eq6_e1037_d_b5, eq6_e1037_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1039;
        let eq6_node_derivatives: [f64; 13] = [eq6_e1039_d_n0, eq6_e1039_d_n1, eq6_e1039_d_n2, eq6_e1039_d_n3, eq6_e1039_d_n4, eq6_e1039_d_n5, eq6_e1039_d_n6, eq6_e1039_d_n7, eq6_e1039_d_n8, eq6_e1039_d_n9, eq6_e1039_d_n10, eq6_e1039_d_n11, eq6_e1039_d_n12];
        let eq6_branch_derivatives: [f64; 7] = [eq6_e1039_d_b0, eq6_e1039_d_b1, eq6_e1039_d_b2, eq6_e1039_d_b3, eq6_e1039_d_b4, eq6_e1039_d_b5, eq6_e1039_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e1050, eq7_e1050_d_n0, eq7_e1050_d_n1, eq7_e1050_d_n2, eq7_e1050_d_n3, eq7_e1050_d_n4, eq7_e1050_d_n5, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9, eq7_e1050_d_n10, eq7_e1050_d_n11, eq7_e1050_d_n12, eq7_e1050_d_b0, eq7_e1050_d_b1, eq7_e1050_d_b2, eq7_e1050_d_b3, eq7_e1050_d_b4, eq7_e1050_d_b5, eq7_e1050_d_b6,) = {
    if (!s.b[2715]) {
        let eq7_e1044: f64 = (s.v[0] * s.v[15]);
        let eq7_e1044_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq7_e1044_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq7_e1044_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq7_e1044_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq7_e1044_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq7_e1044_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq7_e1044_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq7_e1044_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq7_e1044_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq7_e1044_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq7_e1044_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq7_e1044_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq7_e1044_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq7_e1044_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq7_e1044_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq7_e1044_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq7_e1044_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq7_e1044_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq7_e1044_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq7_e1044_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq7_e1046: f64 = (eq7_e1044 * p.p32);
        let eq7_e1046_d_n0: f64 = (eq7_e1044_d_n0 * p.p32);
        let eq7_e1046_d_n1: f64 = (eq7_e1044_d_n1 * p.p32);
        let eq7_e1046_d_n2: f64 = (eq7_e1044_d_n2 * p.p32);
        let eq7_e1046_d_n3: f64 = (eq7_e1044_d_n3 * p.p32);
        let eq7_e1046_d_n4: f64 = (eq7_e1044_d_n4 * p.p32);
        let eq7_e1046_d_n5: f64 = (eq7_e1044_d_n5 * p.p32);
        let eq7_e1046_d_n6: f64 = (eq7_e1044_d_n6 * p.p32);
        let eq7_e1046_d_n7: f64 = (eq7_e1044_d_n7 * p.p32);
        let eq7_e1046_d_n8: f64 = (eq7_e1044_d_n8 * p.p32);
        let eq7_e1046_d_n9: f64 = (eq7_e1044_d_n9 * p.p32);
        let eq7_e1046_d_n10: f64 = (eq7_e1044_d_n10 * p.p32);
        let eq7_e1046_d_n11: f64 = (eq7_e1044_d_n11 * p.p32);
        let eq7_e1046_d_n12: f64 = (eq7_e1044_d_n12 * p.p32);
        let eq7_e1046_d_b0: f64 = (eq7_e1044_d_b0 * p.p32);
        let eq7_e1046_d_b1: f64 = (eq7_e1044_d_b1 * p.p32);
        let eq7_e1046_d_b2: f64 = (eq7_e1044_d_b2 * p.p32);
        let eq7_e1046_d_b3: f64 = (eq7_e1044_d_b3 * p.p32);
        let eq7_e1046_d_b4: f64 = (eq7_e1044_d_b4 * p.p32);
        let eq7_e1046_d_b5: f64 = (eq7_e1044_d_b5 * p.p32);
        let eq7_e1046_d_b6: f64 = (eq7_e1044_d_b6 * p.p32);
        let eq7_e1048: f64 = (eq7_e1046 * s.v[831]);
        let eq7_e1048_d_n0: f64 = ((eq7_e1046_d_n0 * s.v[831]) + (eq7_e1046 * s.dn[831][0]));
        let eq7_e1048_d_n1: f64 = ((eq7_e1046_d_n1 * s.v[831]) + (eq7_e1046 * s.dn[831][1]));
        let eq7_e1048_d_n2: f64 = ((eq7_e1046_d_n2 * s.v[831]) + (eq7_e1046 * s.dn[831][2]));
        let eq7_e1048_d_n3: f64 = ((eq7_e1046_d_n3 * s.v[831]) + (eq7_e1046 * s.dn[831][3]));
        let eq7_e1048_d_n4: f64 = ((eq7_e1046_d_n4 * s.v[831]) + (eq7_e1046 * s.dn[831][4]));
        let eq7_e1048_d_n5: f64 = ((eq7_e1046_d_n5 * s.v[831]) + (eq7_e1046 * s.dn[831][5]));
        let eq7_e1048_d_n6: f64 = ((eq7_e1046_d_n6 * s.v[831]) + (eq7_e1046 * s.dn[831][6]));
        let eq7_e1048_d_n7: f64 = ((eq7_e1046_d_n7 * s.v[831]) + (eq7_e1046 * s.dn[831][7]));
        let eq7_e1048_d_n8: f64 = ((eq7_e1046_d_n8 * s.v[831]) + (eq7_e1046 * s.dn[831][8]));
        let eq7_e1048_d_n9: f64 = ((eq7_e1046_d_n9 * s.v[831]) + (eq7_e1046 * s.dn[831][9]));
        let eq7_e1048_d_n10: f64 = ((eq7_e1046_d_n10 * s.v[831]) + (eq7_e1046 * s.dn[831][10]));
        let eq7_e1048_d_n11: f64 = ((eq7_e1046_d_n11 * s.v[831]) + (eq7_e1046 * s.dn[831][11]));
        let eq7_e1048_d_n12: f64 = ((eq7_e1046_d_n12 * s.v[831]) + (eq7_e1046 * s.dn[831][12]));
        let eq7_e1048_d_b0: f64 = ((eq7_e1046_d_b0 * s.v[831]) + (eq7_e1046 * s.db[831][0]));
        let eq7_e1048_d_b1: f64 = ((eq7_e1046_d_b1 * s.v[831]) + (eq7_e1046 * s.db[831][1]));
        let eq7_e1048_d_b2: f64 = ((eq7_e1046_d_b2 * s.v[831]) + (eq7_e1046 * s.db[831][2]));
        let eq7_e1048_d_b3: f64 = ((eq7_e1046_d_b3 * s.v[831]) + (eq7_e1046 * s.db[831][3]));
        let eq7_e1048_d_b4: f64 = ((eq7_e1046_d_b4 * s.v[831]) + (eq7_e1046 * s.db[831][4]));
        let eq7_e1048_d_b5: f64 = ((eq7_e1046_d_b5 * s.v[831]) + (eq7_e1046 * s.db[831][5]));
        let eq7_e1048_d_b6: f64 = ((eq7_e1046_d_b6 * s.v[831]) + (eq7_e1046 * s.db[831][6]));
        (eq7_e1048, eq7_e1048_d_n0, eq7_e1048_d_n1, eq7_e1048_d_n2, eq7_e1048_d_n3, eq7_e1048_d_n4, eq7_e1048_d_n5, eq7_e1048_d_n6, eq7_e1048_d_n7, eq7_e1048_d_n8, eq7_e1048_d_n9, eq7_e1048_d_n10, eq7_e1048_d_n11, eq7_e1048_d_n12, eq7_e1048_d_b0, eq7_e1048_d_b1, eq7_e1048_d_b2, eq7_e1048_d_b3, eq7_e1048_d_b4, eq7_e1048_d_b5, eq7_e1048_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1050;
        let eq7_node_derivatives: [f64; 13] = [eq7_e1050_d_n0, eq7_e1050_d_n1, eq7_e1050_d_n2, eq7_e1050_d_n3, eq7_e1050_d_n4, eq7_e1050_d_n5, eq7_e1050_d_n6, eq7_e1050_d_n7, eq7_e1050_d_n8, eq7_e1050_d_n9, eq7_e1050_d_n10, eq7_e1050_d_n11, eq7_e1050_d_n12];
        let eq7_branch_derivatives: [f64; 7] = [eq7_e1050_d_b0, eq7_e1050_d_b1, eq7_e1050_d_b2, eq7_e1050_d_b3, eq7_e1050_d_b4, eq7_e1050_d_b5, eq7_e1050_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_e1053: f64 = (s.v[0] * s.v[15]);
        let eq8_e1053_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq8_e1053_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq8_e1053_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq8_e1053_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq8_e1053_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq8_e1053_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq8_e1053_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq8_e1053_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq8_e1053_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq8_e1053_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq8_e1053_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq8_e1053_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq8_e1053_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq8_e1053_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq8_e1053_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq8_e1053_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq8_e1053_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq8_e1053_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq8_e1053_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq8_e1053_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq8_e1055: f64 = (eq8_e1053 * p.p32);
        let eq8_e1055_d_n0: f64 = (eq8_e1053_d_n0 * p.p32);
        let eq8_e1055_d_n1: f64 = (eq8_e1053_d_n1 * p.p32);
        let eq8_e1055_d_n2: f64 = (eq8_e1053_d_n2 * p.p32);
        let eq8_e1055_d_n3: f64 = (eq8_e1053_d_n3 * p.p32);
        let eq8_e1055_d_n4: f64 = (eq8_e1053_d_n4 * p.p32);
        let eq8_e1055_d_n5: f64 = (eq8_e1053_d_n5 * p.p32);
        let eq8_e1055_d_n6: f64 = (eq8_e1053_d_n6 * p.p32);
        let eq8_e1055_d_n7: f64 = (eq8_e1053_d_n7 * p.p32);
        let eq8_e1055_d_n8: f64 = (eq8_e1053_d_n8 * p.p32);
        let eq8_e1055_d_n9: f64 = (eq8_e1053_d_n9 * p.p32);
        let eq8_e1055_d_n10: f64 = (eq8_e1053_d_n10 * p.p32);
        let eq8_e1055_d_n11: f64 = (eq8_e1053_d_n11 * p.p32);
        let eq8_e1055_d_n12: f64 = (eq8_e1053_d_n12 * p.p32);
        let eq8_e1055_d_b0: f64 = (eq8_e1053_d_b0 * p.p32);
        let eq8_e1055_d_b1: f64 = (eq8_e1053_d_b1 * p.p32);
        let eq8_e1055_d_b2: f64 = (eq8_e1053_d_b2 * p.p32);
        let eq8_e1055_d_b3: f64 = (eq8_e1053_d_b3 * p.p32);
        let eq8_e1055_d_b4: f64 = (eq8_e1053_d_b4 * p.p32);
        let eq8_e1055_d_b5: f64 = (eq8_e1053_d_b5 * p.p32);
        let eq8_e1055_d_b6: f64 = (eq8_e1053_d_b6 * p.p32);
        let eq8_e1057: f64 = (eq8_e1055 * s.v[832]);
        let eq8_e1057_d_n0: f64 = ((eq8_e1055_d_n0 * s.v[832]) + (eq8_e1055 * s.dn[832][0]));
        let eq8_e1057_d_n1: f64 = ((eq8_e1055_d_n1 * s.v[832]) + (eq8_e1055 * s.dn[832][1]));
        let eq8_e1057_d_n2: f64 = ((eq8_e1055_d_n2 * s.v[832]) + (eq8_e1055 * s.dn[832][2]));
        let eq8_e1057_d_n3: f64 = ((eq8_e1055_d_n3 * s.v[832]) + (eq8_e1055 * s.dn[832][3]));
        let eq8_e1057_d_n4: f64 = ((eq8_e1055_d_n4 * s.v[832]) + (eq8_e1055 * s.dn[832][4]));
        let eq8_e1057_d_n5: f64 = ((eq8_e1055_d_n5 * s.v[832]) + (eq8_e1055 * s.dn[832][5]));
        let eq8_e1057_d_n6: f64 = ((eq8_e1055_d_n6 * s.v[832]) + (eq8_e1055 * s.dn[832][6]));
        let eq8_e1057_d_n7: f64 = ((eq8_e1055_d_n7 * s.v[832]) + (eq8_e1055 * s.dn[832][7]));
        let eq8_e1057_d_n8: f64 = ((eq8_e1055_d_n8 * s.v[832]) + (eq8_e1055 * s.dn[832][8]));
        let eq8_e1057_d_n9: f64 = ((eq8_e1055_d_n9 * s.v[832]) + (eq8_e1055 * s.dn[832][9]));
        let eq8_e1057_d_n10: f64 = ((eq8_e1055_d_n10 * s.v[832]) + (eq8_e1055 * s.dn[832][10]));
        let eq8_e1057_d_n11: f64 = ((eq8_e1055_d_n11 * s.v[832]) + (eq8_e1055 * s.dn[832][11]));
        let eq8_e1057_d_n12: f64 = ((eq8_e1055_d_n12 * s.v[832]) + (eq8_e1055 * s.dn[832][12]));
        let eq8_e1057_d_b0: f64 = ((eq8_e1055_d_b0 * s.v[832]) + (eq8_e1055 * s.db[832][0]));
        let eq8_e1057_d_b1: f64 = ((eq8_e1055_d_b1 * s.v[832]) + (eq8_e1055 * s.db[832][1]));
        let eq8_e1057_d_b2: f64 = ((eq8_e1055_d_b2 * s.v[832]) + (eq8_e1055 * s.db[832][2]));
        let eq8_e1057_d_b3: f64 = ((eq8_e1055_d_b3 * s.v[832]) + (eq8_e1055 * s.db[832][3]));
        let eq8_e1057_d_b4: f64 = ((eq8_e1055_d_b4 * s.v[832]) + (eq8_e1055 * s.db[832][4]));
        let eq8_e1057_d_b5: f64 = ((eq8_e1055_d_b5 * s.v[832]) + (eq8_e1055 * s.db[832][5]));
        let eq8_e1057_d_b6: f64 = ((eq8_e1055_d_b6 * s.v[832]) + (eq8_e1055 * s.db[832][6]));
        let eq8_value: f64 = eq8_e1057;
        let eq8_node_derivatives: [f64; 13] = [eq8_e1057_d_n0, eq8_e1057_d_n1, eq8_e1057_d_n2, eq8_e1057_d_n3, eq8_e1057_d_n4, eq8_e1057_d_n5, eq8_e1057_d_n6, eq8_e1057_d_n7, eq8_e1057_d_n8, eq8_e1057_d_n9, eq8_e1057_d_n10, eq8_e1057_d_n11, eq8_e1057_d_n12];
        let eq8_branch_derivatives: [f64; 7] = [eq8_e1057_d_b0, eq8_e1057_d_b1, eq8_e1057_d_b2, eq8_e1057_d_b3, eq8_e1057_d_b4, eq8_e1057_d_b5, eq8_e1057_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(9),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e1060: f64 = (s.v[0] * s.v[15]);
        let eq9_e1060_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq9_e1060_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq9_e1060_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq9_e1060_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq9_e1060_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq9_e1060_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq9_e1060_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq9_e1060_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq9_e1060_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq9_e1060_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq9_e1060_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq9_e1060_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq9_e1060_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq9_e1060_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq9_e1060_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq9_e1060_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq9_e1060_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq9_e1060_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq9_e1060_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq9_e1060_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq9_e1062: f64 = (eq9_e1060 * p.p32);
        let eq9_e1062_d_n0: f64 = (eq9_e1060_d_n0 * p.p32);
        let eq9_e1062_d_n1: f64 = (eq9_e1060_d_n1 * p.p32);
        let eq9_e1062_d_n2: f64 = (eq9_e1060_d_n2 * p.p32);
        let eq9_e1062_d_n3: f64 = (eq9_e1060_d_n3 * p.p32);
        let eq9_e1062_d_n4: f64 = (eq9_e1060_d_n4 * p.p32);
        let eq9_e1062_d_n5: f64 = (eq9_e1060_d_n5 * p.p32);
        let eq9_e1062_d_n6: f64 = (eq9_e1060_d_n6 * p.p32);
        let eq9_e1062_d_n7: f64 = (eq9_e1060_d_n7 * p.p32);
        let eq9_e1062_d_n8: f64 = (eq9_e1060_d_n8 * p.p32);
        let eq9_e1062_d_n9: f64 = (eq9_e1060_d_n9 * p.p32);
        let eq9_e1062_d_n10: f64 = (eq9_e1060_d_n10 * p.p32);
        let eq9_e1062_d_n11: f64 = (eq9_e1060_d_n11 * p.p32);
        let eq9_e1062_d_n12: f64 = (eq9_e1060_d_n12 * p.p32);
        let eq9_e1062_d_b0: f64 = (eq9_e1060_d_b0 * p.p32);
        let eq9_e1062_d_b1: f64 = (eq9_e1060_d_b1 * p.p32);
        let eq9_e1062_d_b2: f64 = (eq9_e1060_d_b2 * p.p32);
        let eq9_e1062_d_b3: f64 = (eq9_e1060_d_b3 * p.p32);
        let eq9_e1062_d_b4: f64 = (eq9_e1060_d_b4 * p.p32);
        let eq9_e1062_d_b5: f64 = (eq9_e1060_d_b5 * p.p32);
        let eq9_e1062_d_b6: f64 = (eq9_e1060_d_b6 * p.p32);
        let eq9_e1064: f64 = (eq9_e1062 * s.v[828]);
        let eq9_e1064_d_n0: f64 = ((eq9_e1062_d_n0 * s.v[828]) + (eq9_e1062 * s.dn[828][0]));
        let eq9_e1064_d_n1: f64 = ((eq9_e1062_d_n1 * s.v[828]) + (eq9_e1062 * s.dn[828][1]));
        let eq9_e1064_d_n2: f64 = ((eq9_e1062_d_n2 * s.v[828]) + (eq9_e1062 * s.dn[828][2]));
        let eq9_e1064_d_n3: f64 = ((eq9_e1062_d_n3 * s.v[828]) + (eq9_e1062 * s.dn[828][3]));
        let eq9_e1064_d_n4: f64 = ((eq9_e1062_d_n4 * s.v[828]) + (eq9_e1062 * s.dn[828][4]));
        let eq9_e1064_d_n5: f64 = ((eq9_e1062_d_n5 * s.v[828]) + (eq9_e1062 * s.dn[828][5]));
        let eq9_e1064_d_n6: f64 = ((eq9_e1062_d_n6 * s.v[828]) + (eq9_e1062 * s.dn[828][6]));
        let eq9_e1064_d_n7: f64 = ((eq9_e1062_d_n7 * s.v[828]) + (eq9_e1062 * s.dn[828][7]));
        let eq9_e1064_d_n8: f64 = ((eq9_e1062_d_n8 * s.v[828]) + (eq9_e1062 * s.dn[828][8]));
        let eq9_e1064_d_n9: f64 = ((eq9_e1062_d_n9 * s.v[828]) + (eq9_e1062 * s.dn[828][9]));
        let eq9_e1064_d_n10: f64 = ((eq9_e1062_d_n10 * s.v[828]) + (eq9_e1062 * s.dn[828][10]));
        let eq9_e1064_d_n11: f64 = ((eq9_e1062_d_n11 * s.v[828]) + (eq9_e1062 * s.dn[828][11]));
        let eq9_e1064_d_n12: f64 = ((eq9_e1062_d_n12 * s.v[828]) + (eq9_e1062 * s.dn[828][12]));
        let eq9_e1064_d_b0: f64 = ((eq9_e1062_d_b0 * s.v[828]) + (eq9_e1062 * s.db[828][0]));
        let eq9_e1064_d_b1: f64 = ((eq9_e1062_d_b1 * s.v[828]) + (eq9_e1062 * s.db[828][1]));
        let eq9_e1064_d_b2: f64 = ((eq9_e1062_d_b2 * s.v[828]) + (eq9_e1062 * s.db[828][2]));
        let eq9_e1064_d_b3: f64 = ((eq9_e1062_d_b3 * s.v[828]) + (eq9_e1062 * s.db[828][3]));
        let eq9_e1064_d_b4: f64 = ((eq9_e1062_d_b4 * s.v[828]) + (eq9_e1062 * s.db[828][4]));
        let eq9_e1064_d_b5: f64 = ((eq9_e1062_d_b5 * s.v[828]) + (eq9_e1062 * s.db[828][5]));
        let eq9_e1064_d_b6: f64 = ((eq9_e1062_d_b6 * s.v[828]) + (eq9_e1062 * s.db[828][6]));
        let eq9_value: f64 = eq9_e1064;
        let eq9_node_derivatives: [f64; 13] = [eq9_e1064_d_n0, eq9_e1064_d_n1, eq9_e1064_d_n2, eq9_e1064_d_n3, eq9_e1064_d_n4, eq9_e1064_d_n5, eq9_e1064_d_n6, eq9_e1064_d_n7, eq9_e1064_d_n8, eq9_e1064_d_n9, eq9_e1064_d_n10, eq9_e1064_d_n11, eq9_e1064_d_n12];
        let eq9_branch_derivatives: [f64; 7] = [eq9_e1064_d_b0, eq9_e1064_d_b1, eq9_e1064_d_b2, eq9_e1064_d_b3, eq9_e1064_d_b4, eq9_e1064_d_b5, eq9_e1064_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e1067: f64 = (s.v[0] * s.v[15]);
        let eq10_e1067_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq10_e1067_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq10_e1067_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq10_e1067_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq10_e1067_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq10_e1067_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq10_e1067_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq10_e1067_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq10_e1067_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq10_e1067_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq10_e1067_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq10_e1067_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq10_e1067_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq10_e1067_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq10_e1067_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq10_e1067_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq10_e1067_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq10_e1067_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq10_e1067_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq10_e1067_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq10_e1069: f64 = (eq10_e1067 * p.p32);
        let eq10_e1069_d_n0: f64 = (eq10_e1067_d_n0 * p.p32);
        let eq10_e1069_d_n1: f64 = (eq10_e1067_d_n1 * p.p32);
        let eq10_e1069_d_n2: f64 = (eq10_e1067_d_n2 * p.p32);
        let eq10_e1069_d_n3: f64 = (eq10_e1067_d_n3 * p.p32);
        let eq10_e1069_d_n4: f64 = (eq10_e1067_d_n4 * p.p32);
        let eq10_e1069_d_n5: f64 = (eq10_e1067_d_n5 * p.p32);
        let eq10_e1069_d_n6: f64 = (eq10_e1067_d_n6 * p.p32);
        let eq10_e1069_d_n7: f64 = (eq10_e1067_d_n7 * p.p32);
        let eq10_e1069_d_n8: f64 = (eq10_e1067_d_n8 * p.p32);
        let eq10_e1069_d_n9: f64 = (eq10_e1067_d_n9 * p.p32);
        let eq10_e1069_d_n10: f64 = (eq10_e1067_d_n10 * p.p32);
        let eq10_e1069_d_n11: f64 = (eq10_e1067_d_n11 * p.p32);
        let eq10_e1069_d_n12: f64 = (eq10_e1067_d_n12 * p.p32);
        let eq10_e1069_d_b0: f64 = (eq10_e1067_d_b0 * p.p32);
        let eq10_e1069_d_b1: f64 = (eq10_e1067_d_b1 * p.p32);
        let eq10_e1069_d_b2: f64 = (eq10_e1067_d_b2 * p.p32);
        let eq10_e1069_d_b3: f64 = (eq10_e1067_d_b3 * p.p32);
        let eq10_e1069_d_b4: f64 = (eq10_e1067_d_b4 * p.p32);
        let eq10_e1069_d_b5: f64 = (eq10_e1067_d_b5 * p.p32);
        let eq10_e1069_d_b6: f64 = (eq10_e1067_d_b6 * p.p32);
        let eq10_e1071: f64 = (eq10_e1069 * s.v[829]);
        let eq10_e1071_d_n0: f64 = ((eq10_e1069_d_n0 * s.v[829]) + (eq10_e1069 * s.dn[829][0]));
        let eq10_e1071_d_n1: f64 = ((eq10_e1069_d_n1 * s.v[829]) + (eq10_e1069 * s.dn[829][1]));
        let eq10_e1071_d_n2: f64 = ((eq10_e1069_d_n2 * s.v[829]) + (eq10_e1069 * s.dn[829][2]));
        let eq10_e1071_d_n3: f64 = ((eq10_e1069_d_n3 * s.v[829]) + (eq10_e1069 * s.dn[829][3]));
        let eq10_e1071_d_n4: f64 = ((eq10_e1069_d_n4 * s.v[829]) + (eq10_e1069 * s.dn[829][4]));
        let eq10_e1071_d_n5: f64 = ((eq10_e1069_d_n5 * s.v[829]) + (eq10_e1069 * s.dn[829][5]));
        let eq10_e1071_d_n6: f64 = ((eq10_e1069_d_n6 * s.v[829]) + (eq10_e1069 * s.dn[829][6]));
        let eq10_e1071_d_n7: f64 = ((eq10_e1069_d_n7 * s.v[829]) + (eq10_e1069 * s.dn[829][7]));
        let eq10_e1071_d_n8: f64 = ((eq10_e1069_d_n8 * s.v[829]) + (eq10_e1069 * s.dn[829][8]));
        let eq10_e1071_d_n9: f64 = ((eq10_e1069_d_n9 * s.v[829]) + (eq10_e1069 * s.dn[829][9]));
        let eq10_e1071_d_n10: f64 = ((eq10_e1069_d_n10 * s.v[829]) + (eq10_e1069 * s.dn[829][10]));
        let eq10_e1071_d_n11: f64 = ((eq10_e1069_d_n11 * s.v[829]) + (eq10_e1069 * s.dn[829][11]));
        let eq10_e1071_d_n12: f64 = ((eq10_e1069_d_n12 * s.v[829]) + (eq10_e1069 * s.dn[829][12]));
        let eq10_e1071_d_b0: f64 = ((eq10_e1069_d_b0 * s.v[829]) + (eq10_e1069 * s.db[829][0]));
        let eq10_e1071_d_b1: f64 = ((eq10_e1069_d_b1 * s.v[829]) + (eq10_e1069 * s.db[829][1]));
        let eq10_e1071_d_b2: f64 = ((eq10_e1069_d_b2 * s.v[829]) + (eq10_e1069 * s.db[829][2]));
        let eq10_e1071_d_b3: f64 = ((eq10_e1069_d_b3 * s.v[829]) + (eq10_e1069 * s.db[829][3]));
        let eq10_e1071_d_b4: f64 = ((eq10_e1069_d_b4 * s.v[829]) + (eq10_e1069 * s.db[829][4]));
        let eq10_e1071_d_b5: f64 = ((eq10_e1069_d_b5 * s.v[829]) + (eq10_e1069 * s.db[829][5]));
        let eq10_e1071_d_b6: f64 = ((eq10_e1069_d_b6 * s.v[829]) + (eq10_e1069 * s.db[829][6]));
        let eq10_value: f64 = eq10_e1071;
        let eq10_node_derivatives: [f64; 13] = [eq10_e1071_d_n0, eq10_e1071_d_n1, eq10_e1071_d_n2, eq10_e1071_d_n3, eq10_e1071_d_n4, eq10_e1071_d_n5, eq10_e1071_d_n6, eq10_e1071_d_n7, eq10_e1071_d_n8, eq10_e1071_d_n9, eq10_e1071_d_n10, eq10_e1071_d_n11, eq10_e1071_d_n12];
        let eq10_branch_derivatives: [f64; 7] = [eq10_e1071_d_b0, eq10_e1071_d_b1, eq10_e1071_d_b2, eq10_e1071_d_b3, eq10_e1071_d_b4, eq10_e1071_d_b5, eq10_e1071_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_2(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq11_e1074: f64 = (s.v[0] * s.v[15]);
        let eq11_e1074_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq11_e1074_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq11_e1074_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq11_e1074_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq11_e1074_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq11_e1074_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq11_e1074_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq11_e1074_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq11_e1074_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq11_e1074_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq11_e1074_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq11_e1074_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq11_e1074_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq11_e1074_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq11_e1074_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq11_e1074_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq11_e1074_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq11_e1074_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq11_e1074_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq11_e1074_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq11_e1076: f64 = (eq11_e1074 * p.p32);
        let eq11_e1076_d_n0: f64 = (eq11_e1074_d_n0 * p.p32);
        let eq11_e1076_d_n1: f64 = (eq11_e1074_d_n1 * p.p32);
        let eq11_e1076_d_n2: f64 = (eq11_e1074_d_n2 * p.p32);
        let eq11_e1076_d_n3: f64 = (eq11_e1074_d_n3 * p.p32);
        let eq11_e1076_d_n4: f64 = (eq11_e1074_d_n4 * p.p32);
        let eq11_e1076_d_n5: f64 = (eq11_e1074_d_n5 * p.p32);
        let eq11_e1076_d_n6: f64 = (eq11_e1074_d_n6 * p.p32);
        let eq11_e1076_d_n7: f64 = (eq11_e1074_d_n7 * p.p32);
        let eq11_e1076_d_n8: f64 = (eq11_e1074_d_n8 * p.p32);
        let eq11_e1076_d_n9: f64 = (eq11_e1074_d_n9 * p.p32);
        let eq11_e1076_d_n10: f64 = (eq11_e1074_d_n10 * p.p32);
        let eq11_e1076_d_n11: f64 = (eq11_e1074_d_n11 * p.p32);
        let eq11_e1076_d_n12: f64 = (eq11_e1074_d_n12 * p.p32);
        let eq11_e1076_d_b0: f64 = (eq11_e1074_d_b0 * p.p32);
        let eq11_e1076_d_b1: f64 = (eq11_e1074_d_b1 * p.p32);
        let eq11_e1076_d_b2: f64 = (eq11_e1074_d_b2 * p.p32);
        let eq11_e1076_d_b3: f64 = (eq11_e1074_d_b3 * p.p32);
        let eq11_e1076_d_b4: f64 = (eq11_e1074_d_b4 * p.p32);
        let eq11_e1076_d_b5: f64 = (eq11_e1074_d_b5 * p.p32);
        let eq11_e1076_d_b6: f64 = (eq11_e1074_d_b6 * p.p32);
        let eq11_e1078: f64 = (eq11_e1076 * s.v[833]);
        let eq11_e1078_d_n0: f64 = ((eq11_e1076_d_n0 * s.v[833]) + (eq11_e1076 * s.dn[833][0]));
        let eq11_e1078_d_n1: f64 = ((eq11_e1076_d_n1 * s.v[833]) + (eq11_e1076 * s.dn[833][1]));
        let eq11_e1078_d_n2: f64 = ((eq11_e1076_d_n2 * s.v[833]) + (eq11_e1076 * s.dn[833][2]));
        let eq11_e1078_d_n3: f64 = ((eq11_e1076_d_n3 * s.v[833]) + (eq11_e1076 * s.dn[833][3]));
        let eq11_e1078_d_n4: f64 = ((eq11_e1076_d_n4 * s.v[833]) + (eq11_e1076 * s.dn[833][4]));
        let eq11_e1078_d_n5: f64 = ((eq11_e1076_d_n5 * s.v[833]) + (eq11_e1076 * s.dn[833][5]));
        let eq11_e1078_d_n6: f64 = ((eq11_e1076_d_n6 * s.v[833]) + (eq11_e1076 * s.dn[833][6]));
        let eq11_e1078_d_n7: f64 = ((eq11_e1076_d_n7 * s.v[833]) + (eq11_e1076 * s.dn[833][7]));
        let eq11_e1078_d_n8: f64 = ((eq11_e1076_d_n8 * s.v[833]) + (eq11_e1076 * s.dn[833][8]));
        let eq11_e1078_d_n9: f64 = ((eq11_e1076_d_n9 * s.v[833]) + (eq11_e1076 * s.dn[833][9]));
        let eq11_e1078_d_n10: f64 = ((eq11_e1076_d_n10 * s.v[833]) + (eq11_e1076 * s.dn[833][10]));
        let eq11_e1078_d_n11: f64 = ((eq11_e1076_d_n11 * s.v[833]) + (eq11_e1076 * s.dn[833][11]));
        let eq11_e1078_d_n12: f64 = ((eq11_e1076_d_n12 * s.v[833]) + (eq11_e1076 * s.dn[833][12]));
        let eq11_e1078_d_b0: f64 = ((eq11_e1076_d_b0 * s.v[833]) + (eq11_e1076 * s.db[833][0]));
        let eq11_e1078_d_b1: f64 = ((eq11_e1076_d_b1 * s.v[833]) + (eq11_e1076 * s.db[833][1]));
        let eq11_e1078_d_b2: f64 = ((eq11_e1076_d_b2 * s.v[833]) + (eq11_e1076 * s.db[833][2]));
        let eq11_e1078_d_b3: f64 = ((eq11_e1076_d_b3 * s.v[833]) + (eq11_e1076 * s.db[833][3]));
        let eq11_e1078_d_b4: f64 = ((eq11_e1076_d_b4 * s.v[833]) + (eq11_e1076 * s.db[833][4]));
        let eq11_e1078_d_b5: f64 = ((eq11_e1076_d_b5 * s.v[833]) + (eq11_e1076 * s.db[833][5]));
        let eq11_e1078_d_b6: f64 = ((eq11_e1076_d_b6 * s.v[833]) + (eq11_e1076 * s.db[833][6]));
        let eq11_value: f64 = eq11_e1078;
        let eq11_node_derivatives: [f64; 13] = [eq11_e1078_d_n0, eq11_e1078_d_n1, eq11_e1078_d_n2, eq11_e1078_d_n3, eq11_e1078_d_n4, eq11_e1078_d_n5, eq11_e1078_d_n6, eq11_e1078_d_n7, eq11_e1078_d_n8, eq11_e1078_d_n9, eq11_e1078_d_n10, eq11_e1078_d_n11, eq11_e1078_d_n12];
        let eq11_branch_derivatives: [f64; 7] = [eq11_e1078_d_b0, eq11_e1078_d_b1, eq11_e1078_d_b2, eq11_e1078_d_b3, eq11_e1078_d_b4, eq11_e1078_d_b5, eq11_e1078_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(9),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e1081: f64 = (s.v[0] * s.v[15]);
        let eq12_e1081_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq12_e1081_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq12_e1081_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq12_e1081_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq12_e1081_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq12_e1081_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq12_e1081_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq12_e1081_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq12_e1081_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq12_e1081_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq12_e1081_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq12_e1081_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq12_e1081_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq12_e1081_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq12_e1081_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq12_e1081_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq12_e1081_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq12_e1081_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq12_e1081_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq12_e1081_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq12_e1083: f64 = (eq12_e1081 * p.p32);
        let eq12_e1083_d_n0: f64 = (eq12_e1081_d_n0 * p.p32);
        let eq12_e1083_d_n1: f64 = (eq12_e1081_d_n1 * p.p32);
        let eq12_e1083_d_n2: f64 = (eq12_e1081_d_n2 * p.p32);
        let eq12_e1083_d_n3: f64 = (eq12_e1081_d_n3 * p.p32);
        let eq12_e1083_d_n4: f64 = (eq12_e1081_d_n4 * p.p32);
        let eq12_e1083_d_n5: f64 = (eq12_e1081_d_n5 * p.p32);
        let eq12_e1083_d_n6: f64 = (eq12_e1081_d_n6 * p.p32);
        let eq12_e1083_d_n7: f64 = (eq12_e1081_d_n7 * p.p32);
        let eq12_e1083_d_n8: f64 = (eq12_e1081_d_n8 * p.p32);
        let eq12_e1083_d_n9: f64 = (eq12_e1081_d_n9 * p.p32);
        let eq12_e1083_d_n10: f64 = (eq12_e1081_d_n10 * p.p32);
        let eq12_e1083_d_n11: f64 = (eq12_e1081_d_n11 * p.p32);
        let eq12_e1083_d_n12: f64 = (eq12_e1081_d_n12 * p.p32);
        let eq12_e1083_d_b0: f64 = (eq12_e1081_d_b0 * p.p32);
        let eq12_e1083_d_b1: f64 = (eq12_e1081_d_b1 * p.p32);
        let eq12_e1083_d_b2: f64 = (eq12_e1081_d_b2 * p.p32);
        let eq12_e1083_d_b3: f64 = (eq12_e1081_d_b3 * p.p32);
        let eq12_e1083_d_b4: f64 = (eq12_e1081_d_b4 * p.p32);
        let eq12_e1083_d_b5: f64 = (eq12_e1081_d_b5 * p.p32);
        let eq12_e1083_d_b6: f64 = (eq12_e1081_d_b6 * p.p32);
        let eq12_e1085: f64 = (eq12_e1083 * s.v[834]);
        let eq12_e1085_d_n0: f64 = ((eq12_e1083_d_n0 * s.v[834]) + (eq12_e1083 * s.dn[834][0]));
        let eq12_e1085_d_n1: f64 = ((eq12_e1083_d_n1 * s.v[834]) + (eq12_e1083 * s.dn[834][1]));
        let eq12_e1085_d_n2: f64 = ((eq12_e1083_d_n2 * s.v[834]) + (eq12_e1083 * s.dn[834][2]));
        let eq12_e1085_d_n3: f64 = ((eq12_e1083_d_n3 * s.v[834]) + (eq12_e1083 * s.dn[834][3]));
        let eq12_e1085_d_n4: f64 = ((eq12_e1083_d_n4 * s.v[834]) + (eq12_e1083 * s.dn[834][4]));
        let eq12_e1085_d_n5: f64 = ((eq12_e1083_d_n5 * s.v[834]) + (eq12_e1083 * s.dn[834][5]));
        let eq12_e1085_d_n6: f64 = ((eq12_e1083_d_n6 * s.v[834]) + (eq12_e1083 * s.dn[834][6]));
        let eq12_e1085_d_n7: f64 = ((eq12_e1083_d_n7 * s.v[834]) + (eq12_e1083 * s.dn[834][7]));
        let eq12_e1085_d_n8: f64 = ((eq12_e1083_d_n8 * s.v[834]) + (eq12_e1083 * s.dn[834][8]));
        let eq12_e1085_d_n9: f64 = ((eq12_e1083_d_n9 * s.v[834]) + (eq12_e1083 * s.dn[834][9]));
        let eq12_e1085_d_n10: f64 = ((eq12_e1083_d_n10 * s.v[834]) + (eq12_e1083 * s.dn[834][10]));
        let eq12_e1085_d_n11: f64 = ((eq12_e1083_d_n11 * s.v[834]) + (eq12_e1083 * s.dn[834][11]));
        let eq12_e1085_d_n12: f64 = ((eq12_e1083_d_n12 * s.v[834]) + (eq12_e1083 * s.dn[834][12]));
        let eq12_e1085_d_b0: f64 = ((eq12_e1083_d_b0 * s.v[834]) + (eq12_e1083 * s.db[834][0]));
        let eq12_e1085_d_b1: f64 = ((eq12_e1083_d_b1 * s.v[834]) + (eq12_e1083 * s.db[834][1]));
        let eq12_e1085_d_b2: f64 = ((eq12_e1083_d_b2 * s.v[834]) + (eq12_e1083 * s.db[834][2]));
        let eq12_e1085_d_b3: f64 = ((eq12_e1083_d_b3 * s.v[834]) + (eq12_e1083 * s.db[834][3]));
        let eq12_e1085_d_b4: f64 = ((eq12_e1083_d_b4 * s.v[834]) + (eq12_e1083 * s.db[834][4]));
        let eq12_e1085_d_b5: f64 = ((eq12_e1083_d_b5 * s.v[834]) + (eq12_e1083 * s.db[834][5]));
        let eq12_e1085_d_b6: f64 = ((eq12_e1083_d_b6 * s.v[834]) + (eq12_e1083 * s.db[834][6]));
        let eq12_value: f64 = eq12_e1085;
        let eq12_node_derivatives: [f64; 13] = [eq12_e1085_d_n0, eq12_e1085_d_n1, eq12_e1085_d_n2, eq12_e1085_d_n3, eq12_e1085_d_n4, eq12_e1085_d_n5, eq12_e1085_d_n6, eq12_e1085_d_n7, eq12_e1085_d_n8, eq12_e1085_d_n9, eq12_e1085_d_n10, eq12_e1085_d_n11, eq12_e1085_d_n12];
        let eq12_branch_derivatives: [f64; 7] = [eq12_e1085_d_b0, eq12_e1085_d_b1, eq12_e1085_d_b2, eq12_e1085_d_b3, eq12_e1085_d_b4, eq12_e1085_d_b5, eq12_e1085_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
            multiplicity,
        );
        let eq13_e1088: f64 = (s.v[0] * s.v[15]);
        let eq13_e1088_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq13_e1088_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq13_e1088_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq13_e1088_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq13_e1088_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq13_e1088_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq13_e1088_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq13_e1088_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq13_e1088_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq13_e1088_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq13_e1088_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq13_e1088_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq13_e1088_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq13_e1088_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq13_e1088_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq13_e1088_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq13_e1088_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq13_e1088_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq13_e1088_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq13_e1088_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq13_e1090: f64 = (eq13_e1088 * p.p32);
        let eq13_e1090_d_n0: f64 = (eq13_e1088_d_n0 * p.p32);
        let eq13_e1090_d_n1: f64 = (eq13_e1088_d_n1 * p.p32);
        let eq13_e1090_d_n2: f64 = (eq13_e1088_d_n2 * p.p32);
        let eq13_e1090_d_n3: f64 = (eq13_e1088_d_n3 * p.p32);
        let eq13_e1090_d_n4: f64 = (eq13_e1088_d_n4 * p.p32);
        let eq13_e1090_d_n5: f64 = (eq13_e1088_d_n5 * p.p32);
        let eq13_e1090_d_n6: f64 = (eq13_e1088_d_n6 * p.p32);
        let eq13_e1090_d_n7: f64 = (eq13_e1088_d_n7 * p.p32);
        let eq13_e1090_d_n8: f64 = (eq13_e1088_d_n8 * p.p32);
        let eq13_e1090_d_n9: f64 = (eq13_e1088_d_n9 * p.p32);
        let eq13_e1090_d_n10: f64 = (eq13_e1088_d_n10 * p.p32);
        let eq13_e1090_d_n11: f64 = (eq13_e1088_d_n11 * p.p32);
        let eq13_e1090_d_n12: f64 = (eq13_e1088_d_n12 * p.p32);
        let eq13_e1090_d_b0: f64 = (eq13_e1088_d_b0 * p.p32);
        let eq13_e1090_d_b1: f64 = (eq13_e1088_d_b1 * p.p32);
        let eq13_e1090_d_b2: f64 = (eq13_e1088_d_b2 * p.p32);
        let eq13_e1090_d_b3: f64 = (eq13_e1088_d_b3 * p.p32);
        let eq13_e1090_d_b4: f64 = (eq13_e1088_d_b4 * p.p32);
        let eq13_e1090_d_b5: f64 = (eq13_e1088_d_b5 * p.p32);
        let eq13_e1090_d_b6: f64 = (eq13_e1088_d_b6 * p.p32);
        let eq13_e1092: f64 = (eq13_e1090 * s.v[837]);
        let eq13_e1092_d_n0: f64 = ((eq13_e1090_d_n0 * s.v[837]) + (eq13_e1090 * s.dn[837][0]));
        let eq13_e1092_d_n1: f64 = ((eq13_e1090_d_n1 * s.v[837]) + (eq13_e1090 * s.dn[837][1]));
        let eq13_e1092_d_n2: f64 = ((eq13_e1090_d_n2 * s.v[837]) + (eq13_e1090 * s.dn[837][2]));
        let eq13_e1092_d_n3: f64 = ((eq13_e1090_d_n3 * s.v[837]) + (eq13_e1090 * s.dn[837][3]));
        let eq13_e1092_d_n4: f64 = ((eq13_e1090_d_n4 * s.v[837]) + (eq13_e1090 * s.dn[837][4]));
        let eq13_e1092_d_n5: f64 = ((eq13_e1090_d_n5 * s.v[837]) + (eq13_e1090 * s.dn[837][5]));
        let eq13_e1092_d_n6: f64 = ((eq13_e1090_d_n6 * s.v[837]) + (eq13_e1090 * s.dn[837][6]));
        let eq13_e1092_d_n7: f64 = ((eq13_e1090_d_n7 * s.v[837]) + (eq13_e1090 * s.dn[837][7]));
        let eq13_e1092_d_n8: f64 = ((eq13_e1090_d_n8 * s.v[837]) + (eq13_e1090 * s.dn[837][8]));
        let eq13_e1092_d_n9: f64 = ((eq13_e1090_d_n9 * s.v[837]) + (eq13_e1090 * s.dn[837][9]));
        let eq13_e1092_d_n10: f64 = ((eq13_e1090_d_n10 * s.v[837]) + (eq13_e1090 * s.dn[837][10]));
        let eq13_e1092_d_n11: f64 = ((eq13_e1090_d_n11 * s.v[837]) + (eq13_e1090 * s.dn[837][11]));
        let eq13_e1092_d_n12: f64 = ((eq13_e1090_d_n12 * s.v[837]) + (eq13_e1090 * s.dn[837][12]));
        let eq13_e1092_d_b0: f64 = ((eq13_e1090_d_b0 * s.v[837]) + (eq13_e1090 * s.db[837][0]));
        let eq13_e1092_d_b1: f64 = ((eq13_e1090_d_b1 * s.v[837]) + (eq13_e1090 * s.db[837][1]));
        let eq13_e1092_d_b2: f64 = ((eq13_e1090_d_b2 * s.v[837]) + (eq13_e1090 * s.db[837][2]));
        let eq13_e1092_d_b3: f64 = ((eq13_e1090_d_b3 * s.v[837]) + (eq13_e1090 * s.db[837][3]));
        let eq13_e1092_d_b4: f64 = ((eq13_e1090_d_b4 * s.v[837]) + (eq13_e1090 * s.db[837][4]));
        let eq13_e1092_d_b5: f64 = ((eq13_e1090_d_b5 * s.v[837]) + (eq13_e1090 * s.db[837][5]));
        let eq13_e1092_d_b6: f64 = ((eq13_e1090_d_b6 * s.v[837]) + (eq13_e1090 * s.db[837][6]));
        let eq13_value: f64 = eq13_e1092;
        let eq13_node_derivatives: [f64; 13] = [eq13_e1092_d_n0, eq13_e1092_d_n1, eq13_e1092_d_n2, eq13_e1092_d_n3, eq13_e1092_d_n4, eq13_e1092_d_n5, eq13_e1092_d_n6, eq13_e1092_d_n7, eq13_e1092_d_n8, eq13_e1092_d_n9, eq13_e1092_d_n10, eq13_e1092_d_n11, eq13_e1092_d_n12];
        let eq13_branch_derivatives: [f64; 7] = [eq13_e1092_d_b0, eq13_e1092_d_b1, eq13_e1092_d_b2, eq13_e1092_d_b3, eq13_e1092_d_b4, eq13_e1092_d_b5, eq13_e1092_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e1095: f64 = (s.v[0] * s.v[15]);
        let eq14_e1095_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq14_e1095_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq14_e1095_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq14_e1095_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq14_e1095_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq14_e1095_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq14_e1095_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq14_e1095_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq14_e1095_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq14_e1095_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq14_e1095_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq14_e1095_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq14_e1095_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq14_e1095_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq14_e1095_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq14_e1095_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq14_e1095_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq14_e1095_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq14_e1095_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq14_e1095_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq14_e1097: f64 = (eq14_e1095 * p.p32);
        let eq14_e1097_d_n0: f64 = (eq14_e1095_d_n0 * p.p32);
        let eq14_e1097_d_n1: f64 = (eq14_e1095_d_n1 * p.p32);
        let eq14_e1097_d_n2: f64 = (eq14_e1095_d_n2 * p.p32);
        let eq14_e1097_d_n3: f64 = (eq14_e1095_d_n3 * p.p32);
        let eq14_e1097_d_n4: f64 = (eq14_e1095_d_n4 * p.p32);
        let eq14_e1097_d_n5: f64 = (eq14_e1095_d_n5 * p.p32);
        let eq14_e1097_d_n6: f64 = (eq14_e1095_d_n6 * p.p32);
        let eq14_e1097_d_n7: f64 = (eq14_e1095_d_n7 * p.p32);
        let eq14_e1097_d_n8: f64 = (eq14_e1095_d_n8 * p.p32);
        let eq14_e1097_d_n9: f64 = (eq14_e1095_d_n9 * p.p32);
        let eq14_e1097_d_n10: f64 = (eq14_e1095_d_n10 * p.p32);
        let eq14_e1097_d_n11: f64 = (eq14_e1095_d_n11 * p.p32);
        let eq14_e1097_d_n12: f64 = (eq14_e1095_d_n12 * p.p32);
        let eq14_e1097_d_b0: f64 = (eq14_e1095_d_b0 * p.p32);
        let eq14_e1097_d_b1: f64 = (eq14_e1095_d_b1 * p.p32);
        let eq14_e1097_d_b2: f64 = (eq14_e1095_d_b2 * p.p32);
        let eq14_e1097_d_b3: f64 = (eq14_e1095_d_b3 * p.p32);
        let eq14_e1097_d_b4: f64 = (eq14_e1095_d_b4 * p.p32);
        let eq14_e1097_d_b5: f64 = (eq14_e1095_d_b5 * p.p32);
        let eq14_e1097_d_b6: f64 = (eq14_e1095_d_b6 * p.p32);
        let eq14_e1099: f64 = (eq14_e1097 * s.v[838]);
        let eq14_e1099_d_n0: f64 = ((eq14_e1097_d_n0 * s.v[838]) + (eq14_e1097 * s.dn[838][0]));
        let eq14_e1099_d_n1: f64 = ((eq14_e1097_d_n1 * s.v[838]) + (eq14_e1097 * s.dn[838][1]));
        let eq14_e1099_d_n2: f64 = ((eq14_e1097_d_n2 * s.v[838]) + (eq14_e1097 * s.dn[838][2]));
        let eq14_e1099_d_n3: f64 = ((eq14_e1097_d_n3 * s.v[838]) + (eq14_e1097 * s.dn[838][3]));
        let eq14_e1099_d_n4: f64 = ((eq14_e1097_d_n4 * s.v[838]) + (eq14_e1097 * s.dn[838][4]));
        let eq14_e1099_d_n5: f64 = ((eq14_e1097_d_n5 * s.v[838]) + (eq14_e1097 * s.dn[838][5]));
        let eq14_e1099_d_n6: f64 = ((eq14_e1097_d_n6 * s.v[838]) + (eq14_e1097 * s.dn[838][6]));
        let eq14_e1099_d_n7: f64 = ((eq14_e1097_d_n7 * s.v[838]) + (eq14_e1097 * s.dn[838][7]));
        let eq14_e1099_d_n8: f64 = ((eq14_e1097_d_n8 * s.v[838]) + (eq14_e1097 * s.dn[838][8]));
        let eq14_e1099_d_n9: f64 = ((eq14_e1097_d_n9 * s.v[838]) + (eq14_e1097 * s.dn[838][9]));
        let eq14_e1099_d_n10: f64 = ((eq14_e1097_d_n10 * s.v[838]) + (eq14_e1097 * s.dn[838][10]));
        let eq14_e1099_d_n11: f64 = ((eq14_e1097_d_n11 * s.v[838]) + (eq14_e1097 * s.dn[838][11]));
        let eq14_e1099_d_n12: f64 = ((eq14_e1097_d_n12 * s.v[838]) + (eq14_e1097 * s.dn[838][12]));
        let eq14_e1099_d_b0: f64 = ((eq14_e1097_d_b0 * s.v[838]) + (eq14_e1097 * s.db[838][0]));
        let eq14_e1099_d_b1: f64 = ((eq14_e1097_d_b1 * s.v[838]) + (eq14_e1097 * s.db[838][1]));
        let eq14_e1099_d_b2: f64 = ((eq14_e1097_d_b2 * s.v[838]) + (eq14_e1097 * s.db[838][2]));
        let eq14_e1099_d_b3: f64 = ((eq14_e1097_d_b3 * s.v[838]) + (eq14_e1097 * s.db[838][3]));
        let eq14_e1099_d_b4: f64 = ((eq14_e1097_d_b4 * s.v[838]) + (eq14_e1097 * s.db[838][4]));
        let eq14_e1099_d_b5: f64 = ((eq14_e1097_d_b5 * s.v[838]) + (eq14_e1097 * s.db[838][5]));
        let eq14_e1099_d_b6: f64 = ((eq14_e1097_d_b6 * s.v[838]) + (eq14_e1097 * s.db[838][6]));
        let eq14_value: f64 = eq14_e1099;
        let eq14_node_derivatives: [f64; 13] = [eq14_e1099_d_n0, eq14_e1099_d_n1, eq14_e1099_d_n2, eq14_e1099_d_n3, eq14_e1099_d_n4, eq14_e1099_d_n5, eq14_e1099_d_n6, eq14_e1099_d_n7, eq14_e1099_d_n8, eq14_e1099_d_n9, eq14_e1099_d_n10, eq14_e1099_d_n11, eq14_e1099_d_n12];
        let eq14_branch_derivatives: [f64; 7] = [eq14_e1099_d_b0, eq14_e1099_d_b1, eq14_e1099_d_b2, eq14_e1099_d_b3, eq14_e1099_d_b4, eq14_e1099_d_b5, eq14_e1099_d_b6];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(8),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e1109, eq15_e1109_d_n0, eq15_e1109_d_n1, eq15_e1109_d_n2, eq15_e1109_d_n3, eq15_e1109_d_n4, eq15_e1109_d_n5, eq15_e1109_d_n6, eq15_e1109_d_n7, eq15_e1109_d_n8, eq15_e1109_d_n9, eq15_e1109_d_n10, eq15_e1109_d_n11, eq15_e1109_d_n12, eq15_e1109_d_b0, eq15_e1109_d_b1, eq15_e1109_d_b2, eq15_e1109_d_b3, eq15_e1109_d_b4, eq15_e1109_d_b5, eq15_e1109_d_b6,) = {
    if s.b[2716] {
        let eq15_e1103: f64 = (s.v[15] * p.p32);
        let eq15_e1103_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq15_e1103_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq15_e1103_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq15_e1103_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq15_e1103_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq15_e1103_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq15_e1103_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq15_e1103_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq15_e1103_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq15_e1103_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq15_e1103_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq15_e1103_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq15_e1103_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq15_e1103_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq15_e1103_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq15_e1103_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq15_e1103_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq15_e1103_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq15_e1103_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq15_e1103_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq15_e1105: f64 = (eq15_e1103 * s.v[800]);
        let eq15_e1105_d_n0: f64 = ((eq15_e1103_d_n0 * s.v[800]) + (eq15_e1103 * s.dn[800][0]));
        let eq15_e1105_d_n1: f64 = ((eq15_e1103_d_n1 * s.v[800]) + (eq15_e1103 * s.dn[800][1]));
        let eq15_e1105_d_n2: f64 = ((eq15_e1103_d_n2 * s.v[800]) + (eq15_e1103 * s.dn[800][2]));
        let eq15_e1105_d_n3: f64 = ((eq15_e1103_d_n3 * s.v[800]) + (eq15_e1103 * s.dn[800][3]));
        let eq15_e1105_d_n4: f64 = ((eq15_e1103_d_n4 * s.v[800]) + (eq15_e1103 * s.dn[800][4]));
        let eq15_e1105_d_n5: f64 = ((eq15_e1103_d_n5 * s.v[800]) + (eq15_e1103 * s.dn[800][5]));
        let eq15_e1105_d_n6: f64 = ((eq15_e1103_d_n6 * s.v[800]) + (eq15_e1103 * s.dn[800][6]));
        let eq15_e1105_d_n7: f64 = ((eq15_e1103_d_n7 * s.v[800]) + (eq15_e1103 * s.dn[800][7]));
        let eq15_e1105_d_n8: f64 = ((eq15_e1103_d_n8 * s.v[800]) + (eq15_e1103 * s.dn[800][8]));
        let eq15_e1105_d_n9: f64 = ((eq15_e1103_d_n9 * s.v[800]) + (eq15_e1103 * s.dn[800][9]));
        let eq15_e1105_d_n10: f64 = ((eq15_e1103_d_n10 * s.v[800]) + (eq15_e1103 * s.dn[800][10]));
        let eq15_e1105_d_n11: f64 = ((eq15_e1103_d_n11 * s.v[800]) + (eq15_e1103 * s.dn[800][11]));
        let eq15_e1105_d_n12: f64 = ((eq15_e1103_d_n12 * s.v[800]) + (eq15_e1103 * s.dn[800][12]));
        let eq15_e1105_d_b0: f64 = ((eq15_e1103_d_b0 * s.v[800]) + (eq15_e1103 * s.db[800][0]));
        let eq15_e1105_d_b1: f64 = ((eq15_e1103_d_b1 * s.v[800]) + (eq15_e1103 * s.db[800][1]));
        let eq15_e1105_d_b2: f64 = ((eq15_e1103_d_b2 * s.v[800]) + (eq15_e1103 * s.db[800][2]));
        let eq15_e1105_d_b3: f64 = ((eq15_e1103_d_b3 * s.v[800]) + (eq15_e1103 * s.db[800][3]));
        let eq15_e1105_d_b4: f64 = ((eq15_e1103_d_b4 * s.v[800]) + (eq15_e1103 * s.db[800][4]));
        let eq15_e1105_d_b5: f64 = ((eq15_e1103_d_b5 * s.v[800]) + (eq15_e1103 * s.db[800][5]));
        let eq15_e1105_d_b6: f64 = ((eq15_e1103_d_b6 * s.v[800]) + (eq15_e1103 * s.db[800][6]));
        let eq15_e1107: f64 = (eq15_e1105 * (nv1 - nv6));
        let eq15_e1107_d_n0: f64 = (eq15_e1105_d_n0 * (nv1 - nv6));
        let eq15_e1107_d_n1: f64 = ((eq15_e1105_d_n1 * (nv1 - nv6)) + eq15_e1105);
        let eq15_e1107_d_n2: f64 = (eq15_e1105_d_n2 * (nv1 - nv6));
        let eq15_e1107_d_n3: f64 = (eq15_e1105_d_n3 * (nv1 - nv6));
        let eq15_e1107_d_n4: f64 = (eq15_e1105_d_n4 * (nv1 - nv6));
        let eq15_e1107_d_n5: f64 = (eq15_e1105_d_n5 * (nv1 - nv6));
        let eq15_e1107_d_n6: f64 = ((eq15_e1105_d_n6 * (nv1 - nv6)) + (-eq15_e1105));
        let eq15_e1107_d_n7: f64 = (eq15_e1105_d_n7 * (nv1 - nv6));
        let eq15_e1107_d_n8: f64 = (eq15_e1105_d_n8 * (nv1 - nv6));
        let eq15_e1107_d_n9: f64 = (eq15_e1105_d_n9 * (nv1 - nv6));
        let eq15_e1107_d_n10: f64 = (eq15_e1105_d_n10 * (nv1 - nv6));
        let eq15_e1107_d_n11: f64 = (eq15_e1105_d_n11 * (nv1 - nv6));
        let eq15_e1107_d_n12: f64 = (eq15_e1105_d_n12 * (nv1 - nv6));
        let eq15_e1107_d_b0: f64 = (eq15_e1105_d_b0 * (nv1 - nv6));
        let eq15_e1107_d_b1: f64 = (eq15_e1105_d_b1 * (nv1 - nv6));
        let eq15_e1107_d_b2: f64 = (eq15_e1105_d_b2 * (nv1 - nv6));
        let eq15_e1107_d_b3: f64 = (eq15_e1105_d_b3 * (nv1 - nv6));
        let eq15_e1107_d_b4: f64 = (eq15_e1105_d_b4 * (nv1 - nv6));
        let eq15_e1107_d_b5: f64 = (eq15_e1105_d_b5 * (nv1 - nv6));
        let eq15_e1107_d_b6: f64 = (eq15_e1105_d_b6 * (nv1 - nv6));
        (eq15_e1107, eq15_e1107_d_n0, eq15_e1107_d_n1, eq15_e1107_d_n2, eq15_e1107_d_n3, eq15_e1107_d_n4, eq15_e1107_d_n5, eq15_e1107_d_n6, eq15_e1107_d_n7, eq15_e1107_d_n8, eq15_e1107_d_n9, eq15_e1107_d_n10, eq15_e1107_d_n11, eq15_e1107_d_n12, eq15_e1107_d_b0, eq15_e1107_d_b1, eq15_e1107_d_b2, eq15_e1107_d_b3, eq15_e1107_d_b4, eq15_e1107_d_b5, eq15_e1107_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1109;
        let eq15_node_derivatives: [f64; 13] = [eq15_e1109_d_n0, eq15_e1109_d_n1, eq15_e1109_d_n2, eq15_e1109_d_n3, eq15_e1109_d_n4, eq15_e1109_d_n5, eq15_e1109_d_n6, eq15_e1109_d_n7, eq15_e1109_d_n8, eq15_e1109_d_n9, eq15_e1109_d_n10, eq15_e1109_d_n11, eq15_e1109_d_n12];
        let eq15_branch_derivatives: [f64; 7] = [eq15_e1109_d_b0, eq15_e1109_d_b1, eq15_e1109_d_b2, eq15_e1109_d_b3, eq15_e1109_d_b4, eq15_e1109_d_b5, eq15_e1109_d_b6];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(6),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq17_e1124,) = {
    if (!s.b[2716]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1124;
        stamper.stamp_potential_const_local(
            0,
            eq17_value,
        );
        let (eq18_e1134, eq18_e1134_d_n0, eq18_e1134_d_n1, eq18_e1134_d_n2, eq18_e1134_d_n3, eq18_e1134_d_n4, eq18_e1134_d_n5, eq18_e1134_d_n6, eq18_e1134_d_n7, eq18_e1134_d_n8, eq18_e1134_d_n9, eq18_e1134_d_n10, eq18_e1134_d_n11, eq18_e1134_d_n12, eq18_e1134_d_b0, eq18_e1134_d_b1, eq18_e1134_d_b2, eq18_e1134_d_b3, eq18_e1134_d_b4, eq18_e1134_d_b5, eq18_e1134_d_b6,) = {
    if s.b[2717] {
        let eq18_e1128: f64 = (s.v[15] * p.p32);
        let eq18_e1128_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq18_e1128_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq18_e1128_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq18_e1128_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq18_e1128_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq18_e1128_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq18_e1128_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq18_e1128_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq18_e1128_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq18_e1128_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq18_e1128_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq18_e1128_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq18_e1128_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq18_e1128_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq18_e1128_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq18_e1128_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq18_e1128_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq18_e1128_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq18_e1128_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq18_e1128_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq18_e1130: f64 = (eq18_e1128 * s.v[801]);
        let eq18_e1130_d_n0: f64 = ((eq18_e1128_d_n0 * s.v[801]) + (eq18_e1128 * s.dn[801][0]));
        let eq18_e1130_d_n1: f64 = ((eq18_e1128_d_n1 * s.v[801]) + (eq18_e1128 * s.dn[801][1]));
        let eq18_e1130_d_n2: f64 = ((eq18_e1128_d_n2 * s.v[801]) + (eq18_e1128 * s.dn[801][2]));
        let eq18_e1130_d_n3: f64 = ((eq18_e1128_d_n3 * s.v[801]) + (eq18_e1128 * s.dn[801][3]));
        let eq18_e1130_d_n4: f64 = ((eq18_e1128_d_n4 * s.v[801]) + (eq18_e1128 * s.dn[801][4]));
        let eq18_e1130_d_n5: f64 = ((eq18_e1128_d_n5 * s.v[801]) + (eq18_e1128 * s.dn[801][5]));
        let eq18_e1130_d_n6: f64 = ((eq18_e1128_d_n6 * s.v[801]) + (eq18_e1128 * s.dn[801][6]));
        let eq18_e1130_d_n7: f64 = ((eq18_e1128_d_n7 * s.v[801]) + (eq18_e1128 * s.dn[801][7]));
        let eq18_e1130_d_n8: f64 = ((eq18_e1128_d_n8 * s.v[801]) + (eq18_e1128 * s.dn[801][8]));
        let eq18_e1130_d_n9: f64 = ((eq18_e1128_d_n9 * s.v[801]) + (eq18_e1128 * s.dn[801][9]));
        let eq18_e1130_d_n10: f64 = ((eq18_e1128_d_n10 * s.v[801]) + (eq18_e1128 * s.dn[801][10]));
        let eq18_e1130_d_n11: f64 = ((eq18_e1128_d_n11 * s.v[801]) + (eq18_e1128 * s.dn[801][11]));
        let eq18_e1130_d_n12: f64 = ((eq18_e1128_d_n12 * s.v[801]) + (eq18_e1128 * s.dn[801][12]));
        let eq18_e1130_d_b0: f64 = ((eq18_e1128_d_b0 * s.v[801]) + (eq18_e1128 * s.db[801][0]));
        let eq18_e1130_d_b1: f64 = ((eq18_e1128_d_b1 * s.v[801]) + (eq18_e1128 * s.db[801][1]));
        let eq18_e1130_d_b2: f64 = ((eq18_e1128_d_b2 * s.v[801]) + (eq18_e1128 * s.db[801][2]));
        let eq18_e1130_d_b3: f64 = ((eq18_e1128_d_b3 * s.v[801]) + (eq18_e1128 * s.db[801][3]));
        let eq18_e1130_d_b4: f64 = ((eq18_e1128_d_b4 * s.v[801]) + (eq18_e1128 * s.db[801][4]));
        let eq18_e1130_d_b5: f64 = ((eq18_e1128_d_b5 * s.v[801]) + (eq18_e1128 * s.db[801][5]));
        let eq18_e1130_d_b6: f64 = ((eq18_e1128_d_b6 * s.v[801]) + (eq18_e1128 * s.db[801][6]));
        let eq18_e1132: f64 = (eq18_e1130 * (nv2 - nv7));
        let eq18_e1132_d_n0: f64 = (eq18_e1130_d_n0 * (nv2 - nv7));
        let eq18_e1132_d_n1: f64 = (eq18_e1130_d_n1 * (nv2 - nv7));
        let eq18_e1132_d_n2: f64 = ((eq18_e1130_d_n2 * (nv2 - nv7)) + eq18_e1130);
        let eq18_e1132_d_n3: f64 = (eq18_e1130_d_n3 * (nv2 - nv7));
        let eq18_e1132_d_n4: f64 = (eq18_e1130_d_n4 * (nv2 - nv7));
        let eq18_e1132_d_n5: f64 = (eq18_e1130_d_n5 * (nv2 - nv7));
        let eq18_e1132_d_n6: f64 = (eq18_e1130_d_n6 * (nv2 - nv7));
        let eq18_e1132_d_n7: f64 = ((eq18_e1130_d_n7 * (nv2 - nv7)) + (-eq18_e1130));
        let eq18_e1132_d_n8: f64 = (eq18_e1130_d_n8 * (nv2 - nv7));
        let eq18_e1132_d_n9: f64 = (eq18_e1130_d_n9 * (nv2 - nv7));
        let eq18_e1132_d_n10: f64 = (eq18_e1130_d_n10 * (nv2 - nv7));
        let eq18_e1132_d_n11: f64 = (eq18_e1130_d_n11 * (nv2 - nv7));
        let eq18_e1132_d_n12: f64 = (eq18_e1130_d_n12 * (nv2 - nv7));
        let eq18_e1132_d_b0: f64 = (eq18_e1130_d_b0 * (nv2 - nv7));
        let eq18_e1132_d_b1: f64 = (eq18_e1130_d_b1 * (nv2 - nv7));
        let eq18_e1132_d_b2: f64 = (eq18_e1130_d_b2 * (nv2 - nv7));
        let eq18_e1132_d_b3: f64 = (eq18_e1130_d_b3 * (nv2 - nv7));
        let eq18_e1132_d_b4: f64 = (eq18_e1130_d_b4 * (nv2 - nv7));
        let eq18_e1132_d_b5: f64 = (eq18_e1130_d_b5 * (nv2 - nv7));
        let eq18_e1132_d_b6: f64 = (eq18_e1130_d_b6 * (nv2 - nv7));
        (eq18_e1132, eq18_e1132_d_n0, eq18_e1132_d_n1, eq18_e1132_d_n2, eq18_e1132_d_n3, eq18_e1132_d_n4, eq18_e1132_d_n5, eq18_e1132_d_n6, eq18_e1132_d_n7, eq18_e1132_d_n8, eq18_e1132_d_n9, eq18_e1132_d_n10, eq18_e1132_d_n11, eq18_e1132_d_n12, eq18_e1132_d_b0, eq18_e1132_d_b1, eq18_e1132_d_b2, eq18_e1132_d_b3, eq18_e1132_d_b4, eq18_e1132_d_b5, eq18_e1132_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1134;
        let eq18_node_derivatives: [f64; 13] = [eq18_e1134_d_n0, eq18_e1134_d_n1, eq18_e1134_d_n2, eq18_e1134_d_n3, eq18_e1134_d_n4, eq18_e1134_d_n5, eq18_e1134_d_n6, eq18_e1134_d_n7, eq18_e1134_d_n8, eq18_e1134_d_n9, eq18_e1134_d_n10, eq18_e1134_d_n11, eq18_e1134_d_n12];
        let eq18_branch_derivatives: [f64; 7] = [eq18_e1134_d_b0, eq18_e1134_d_b1, eq18_e1134_d_b2, eq18_e1134_d_b3, eq18_e1134_d_b4, eq18_e1134_d_b5, eq18_e1134_d_b6];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(7),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq20_e1149,) = {
    if (!s.b[2717]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1149;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_3(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let nv12 = ctx.node_voltage(nodes[12]);
        let (eq21_e1159, eq21_e1159_d_n0, eq21_e1159_d_n1, eq21_e1159_d_n2, eq21_e1159_d_n3, eq21_e1159_d_n4, eq21_e1159_d_n5, eq21_e1159_d_n6, eq21_e1159_d_n7, eq21_e1159_d_n8, eq21_e1159_d_n9, eq21_e1159_d_n10, eq21_e1159_d_n11, eq21_e1159_d_n12, eq21_e1159_d_b0, eq21_e1159_d_b1, eq21_e1159_d_b2, eq21_e1159_d_b3, eq21_e1159_d_b4, eq21_e1159_d_b5, eq21_e1159_d_b6,) = {
    if s.b[2718] {
        let eq21_e1153: f64 = (s.v[15] * p.p32);
        let eq21_e1153_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq21_e1153_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq21_e1153_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq21_e1153_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq21_e1153_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq21_e1153_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq21_e1153_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq21_e1153_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq21_e1153_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq21_e1153_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq21_e1153_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq21_e1153_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq21_e1153_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq21_e1153_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq21_e1153_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq21_e1153_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq21_e1153_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq21_e1153_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq21_e1153_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq21_e1153_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq21_e1155: f64 = (eq21_e1153 * s.v[802]);
        let eq21_e1155_d_n0: f64 = ((eq21_e1153_d_n0 * s.v[802]) + (eq21_e1153 * s.dn[802][0]));
        let eq21_e1155_d_n1: f64 = ((eq21_e1153_d_n1 * s.v[802]) + (eq21_e1153 * s.dn[802][1]));
        let eq21_e1155_d_n2: f64 = ((eq21_e1153_d_n2 * s.v[802]) + (eq21_e1153 * s.dn[802][2]));
        let eq21_e1155_d_n3: f64 = ((eq21_e1153_d_n3 * s.v[802]) + (eq21_e1153 * s.dn[802][3]));
        let eq21_e1155_d_n4: f64 = ((eq21_e1153_d_n4 * s.v[802]) + (eq21_e1153 * s.dn[802][4]));
        let eq21_e1155_d_n5: f64 = ((eq21_e1153_d_n5 * s.v[802]) + (eq21_e1153 * s.dn[802][5]));
        let eq21_e1155_d_n6: f64 = ((eq21_e1153_d_n6 * s.v[802]) + (eq21_e1153 * s.dn[802][6]));
        let eq21_e1155_d_n7: f64 = ((eq21_e1153_d_n7 * s.v[802]) + (eq21_e1153 * s.dn[802][7]));
        let eq21_e1155_d_n8: f64 = ((eq21_e1153_d_n8 * s.v[802]) + (eq21_e1153 * s.dn[802][8]));
        let eq21_e1155_d_n9: f64 = ((eq21_e1153_d_n9 * s.v[802]) + (eq21_e1153 * s.dn[802][9]));
        let eq21_e1155_d_n10: f64 = ((eq21_e1153_d_n10 * s.v[802]) + (eq21_e1153 * s.dn[802][10]));
        let eq21_e1155_d_n11: f64 = ((eq21_e1153_d_n11 * s.v[802]) + (eq21_e1153 * s.dn[802][11]));
        let eq21_e1155_d_n12: f64 = ((eq21_e1153_d_n12 * s.v[802]) + (eq21_e1153 * s.dn[802][12]));
        let eq21_e1155_d_b0: f64 = ((eq21_e1153_d_b0 * s.v[802]) + (eq21_e1153 * s.db[802][0]));
        let eq21_e1155_d_b1: f64 = ((eq21_e1153_d_b1 * s.v[802]) + (eq21_e1153 * s.db[802][1]));
        let eq21_e1155_d_b2: f64 = ((eq21_e1153_d_b2 * s.v[802]) + (eq21_e1153 * s.db[802][2]));
        let eq21_e1155_d_b3: f64 = ((eq21_e1153_d_b3 * s.v[802]) + (eq21_e1153 * s.db[802][3]));
        let eq21_e1155_d_b4: f64 = ((eq21_e1153_d_b4 * s.v[802]) + (eq21_e1153 * s.db[802][4]));
        let eq21_e1155_d_b5: f64 = ((eq21_e1153_d_b5 * s.v[802]) + (eq21_e1153 * s.db[802][5]));
        let eq21_e1155_d_b6: f64 = ((eq21_e1153_d_b6 * s.v[802]) + (eq21_e1153 * s.db[802][6]));
        let eq21_e1157: f64 = (eq21_e1155 * (nv0 - nv8));
        let eq21_e1157_d_n0: f64 = ((eq21_e1155_d_n0 * (nv0 - nv8)) + eq21_e1155);
        let eq21_e1157_d_n1: f64 = (eq21_e1155_d_n1 * (nv0 - nv8));
        let eq21_e1157_d_n2: f64 = (eq21_e1155_d_n2 * (nv0 - nv8));
        let eq21_e1157_d_n3: f64 = (eq21_e1155_d_n3 * (nv0 - nv8));
        let eq21_e1157_d_n4: f64 = (eq21_e1155_d_n4 * (nv0 - nv8));
        let eq21_e1157_d_n5: f64 = (eq21_e1155_d_n5 * (nv0 - nv8));
        let eq21_e1157_d_n6: f64 = (eq21_e1155_d_n6 * (nv0 - nv8));
        let eq21_e1157_d_n7: f64 = (eq21_e1155_d_n7 * (nv0 - nv8));
        let eq21_e1157_d_n8: f64 = ((eq21_e1155_d_n8 * (nv0 - nv8)) + (-eq21_e1155));
        let eq21_e1157_d_n9: f64 = (eq21_e1155_d_n9 * (nv0 - nv8));
        let eq21_e1157_d_n10: f64 = (eq21_e1155_d_n10 * (nv0 - nv8));
        let eq21_e1157_d_n11: f64 = (eq21_e1155_d_n11 * (nv0 - nv8));
        let eq21_e1157_d_n12: f64 = (eq21_e1155_d_n12 * (nv0 - nv8));
        let eq21_e1157_d_b0: f64 = (eq21_e1155_d_b0 * (nv0 - nv8));
        let eq21_e1157_d_b1: f64 = (eq21_e1155_d_b1 * (nv0 - nv8));
        let eq21_e1157_d_b2: f64 = (eq21_e1155_d_b2 * (nv0 - nv8));
        let eq21_e1157_d_b3: f64 = (eq21_e1155_d_b3 * (nv0 - nv8));
        let eq21_e1157_d_b4: f64 = (eq21_e1155_d_b4 * (nv0 - nv8));
        let eq21_e1157_d_b5: f64 = (eq21_e1155_d_b5 * (nv0 - nv8));
        let eq21_e1157_d_b6: f64 = (eq21_e1155_d_b6 * (nv0 - nv8));
        (eq21_e1157, eq21_e1157_d_n0, eq21_e1157_d_n1, eq21_e1157_d_n2, eq21_e1157_d_n3, eq21_e1157_d_n4, eq21_e1157_d_n5, eq21_e1157_d_n6, eq21_e1157_d_n7, eq21_e1157_d_n8, eq21_e1157_d_n9, eq21_e1157_d_n10, eq21_e1157_d_n11, eq21_e1157_d_n12, eq21_e1157_d_b0, eq21_e1157_d_b1, eq21_e1157_d_b2, eq21_e1157_d_b3, eq21_e1157_d_b4, eq21_e1157_d_b5, eq21_e1157_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1159;
        let eq21_node_derivatives: [f64; 13] = [eq21_e1159_d_n0, eq21_e1159_d_n1, eq21_e1159_d_n2, eq21_e1159_d_n3, eq21_e1159_d_n4, eq21_e1159_d_n5, eq21_e1159_d_n6, eq21_e1159_d_n7, eq21_e1159_d_n8, eq21_e1159_d_n9, eq21_e1159_d_n10, eq21_e1159_d_n11, eq21_e1159_d_n12];
        let eq21_branch_derivatives: [f64; 7] = [eq21_e1159_d_b0, eq21_e1159_d_b1, eq21_e1159_d_b2, eq21_e1159_d_b3, eq21_e1159_d_b4, eq21_e1159_d_b5, eq21_e1159_d_b6];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(8),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq23_e1174,) = {
    if (!s.b[2718]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1174;
        stamper.stamp_potential_const_local(
            2,
            eq23_value,
        );
        let (eq24_e1184, eq24_e1184_d_n0, eq24_e1184_d_n1, eq24_e1184_d_n2, eq24_e1184_d_n3, eq24_e1184_d_n4, eq24_e1184_d_n5, eq24_e1184_d_n6, eq24_e1184_d_n7, eq24_e1184_d_n8, eq24_e1184_d_n9, eq24_e1184_d_n10, eq24_e1184_d_n11, eq24_e1184_d_n12, eq24_e1184_d_b0, eq24_e1184_d_b1, eq24_e1184_d_b2, eq24_e1184_d_b3, eq24_e1184_d_b4, eq24_e1184_d_b5, eq24_e1184_d_b6,) = {
    if s.b[2719] {
        let eq24_e1178: f64 = (s.v[15] * p.p32);
        let eq24_e1178_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq24_e1178_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq24_e1178_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq24_e1178_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq24_e1178_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq24_e1178_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq24_e1178_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq24_e1178_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq24_e1178_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq24_e1178_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq24_e1178_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq24_e1178_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq24_e1178_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq24_e1178_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq24_e1178_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq24_e1178_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq24_e1178_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq24_e1178_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq24_e1178_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq24_e1178_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq24_e1180: f64 = (eq24_e1178 * s.v[803]);
        let eq24_e1180_d_n0: f64 = ((eq24_e1178_d_n0 * s.v[803]) + (eq24_e1178 * s.dn[803][0]));
        let eq24_e1180_d_n1: f64 = ((eq24_e1178_d_n1 * s.v[803]) + (eq24_e1178 * s.dn[803][1]));
        let eq24_e1180_d_n2: f64 = ((eq24_e1178_d_n2 * s.v[803]) + (eq24_e1178 * s.dn[803][2]));
        let eq24_e1180_d_n3: f64 = ((eq24_e1178_d_n3 * s.v[803]) + (eq24_e1178 * s.dn[803][3]));
        let eq24_e1180_d_n4: f64 = ((eq24_e1178_d_n4 * s.v[803]) + (eq24_e1178 * s.dn[803][4]));
        let eq24_e1180_d_n5: f64 = ((eq24_e1178_d_n5 * s.v[803]) + (eq24_e1178 * s.dn[803][5]));
        let eq24_e1180_d_n6: f64 = ((eq24_e1178_d_n6 * s.v[803]) + (eq24_e1178 * s.dn[803][6]));
        let eq24_e1180_d_n7: f64 = ((eq24_e1178_d_n7 * s.v[803]) + (eq24_e1178 * s.dn[803][7]));
        let eq24_e1180_d_n8: f64 = ((eq24_e1178_d_n8 * s.v[803]) + (eq24_e1178 * s.dn[803][8]));
        let eq24_e1180_d_n9: f64 = ((eq24_e1178_d_n9 * s.v[803]) + (eq24_e1178 * s.dn[803][9]));
        let eq24_e1180_d_n10: f64 = ((eq24_e1178_d_n10 * s.v[803]) + (eq24_e1178 * s.dn[803][10]));
        let eq24_e1180_d_n11: f64 = ((eq24_e1178_d_n11 * s.v[803]) + (eq24_e1178 * s.dn[803][11]));
        let eq24_e1180_d_n12: f64 = ((eq24_e1178_d_n12 * s.v[803]) + (eq24_e1178 * s.dn[803][12]));
        let eq24_e1180_d_b0: f64 = ((eq24_e1178_d_b0 * s.v[803]) + (eq24_e1178 * s.db[803][0]));
        let eq24_e1180_d_b1: f64 = ((eq24_e1178_d_b1 * s.v[803]) + (eq24_e1178 * s.db[803][1]));
        let eq24_e1180_d_b2: f64 = ((eq24_e1178_d_b2 * s.v[803]) + (eq24_e1178 * s.db[803][2]));
        let eq24_e1180_d_b3: f64 = ((eq24_e1178_d_b3 * s.v[803]) + (eq24_e1178 * s.db[803][3]));
        let eq24_e1180_d_b4: f64 = ((eq24_e1178_d_b4 * s.v[803]) + (eq24_e1178 * s.db[803][4]));
        let eq24_e1180_d_b5: f64 = ((eq24_e1178_d_b5 * s.v[803]) + (eq24_e1178 * s.db[803][5]));
        let eq24_e1180_d_b6: f64 = ((eq24_e1178_d_b6 * s.v[803]) + (eq24_e1178 * s.db[803][6]));
        let eq24_e1182: f64 = (eq24_e1180 * (nv9 - nv10));
        let eq24_e1182_d_n0: f64 = (eq24_e1180_d_n0 * (nv9 - nv10));
        let eq24_e1182_d_n1: f64 = (eq24_e1180_d_n1 * (nv9 - nv10));
        let eq24_e1182_d_n2: f64 = (eq24_e1180_d_n2 * (nv9 - nv10));
        let eq24_e1182_d_n3: f64 = (eq24_e1180_d_n3 * (nv9 - nv10));
        let eq24_e1182_d_n4: f64 = (eq24_e1180_d_n4 * (nv9 - nv10));
        let eq24_e1182_d_n5: f64 = (eq24_e1180_d_n5 * (nv9 - nv10));
        let eq24_e1182_d_n6: f64 = (eq24_e1180_d_n6 * (nv9 - nv10));
        let eq24_e1182_d_n7: f64 = (eq24_e1180_d_n7 * (nv9 - nv10));
        let eq24_e1182_d_n8: f64 = (eq24_e1180_d_n8 * (nv9 - nv10));
        let eq24_e1182_d_n9: f64 = ((eq24_e1180_d_n9 * (nv9 - nv10)) + eq24_e1180);
        let eq24_e1182_d_n10: f64 = ((eq24_e1180_d_n10 * (nv9 - nv10)) + (-eq24_e1180));
        let eq24_e1182_d_n11: f64 = (eq24_e1180_d_n11 * (nv9 - nv10));
        let eq24_e1182_d_n12: f64 = (eq24_e1180_d_n12 * (nv9 - nv10));
        let eq24_e1182_d_b0: f64 = (eq24_e1180_d_b0 * (nv9 - nv10));
        let eq24_e1182_d_b1: f64 = (eq24_e1180_d_b1 * (nv9 - nv10));
        let eq24_e1182_d_b2: f64 = (eq24_e1180_d_b2 * (nv9 - nv10));
        let eq24_e1182_d_b3: f64 = (eq24_e1180_d_b3 * (nv9 - nv10));
        let eq24_e1182_d_b4: f64 = (eq24_e1180_d_b4 * (nv9 - nv10));
        let eq24_e1182_d_b5: f64 = (eq24_e1180_d_b5 * (nv9 - nv10));
        let eq24_e1182_d_b6: f64 = (eq24_e1180_d_b6 * (nv9 - nv10));
        (eq24_e1182, eq24_e1182_d_n0, eq24_e1182_d_n1, eq24_e1182_d_n2, eq24_e1182_d_n3, eq24_e1182_d_n4, eq24_e1182_d_n5, eq24_e1182_d_n6, eq24_e1182_d_n7, eq24_e1182_d_n8, eq24_e1182_d_n9, eq24_e1182_d_n10, eq24_e1182_d_n11, eq24_e1182_d_n12, eq24_e1182_d_b0, eq24_e1182_d_b1, eq24_e1182_d_b2, eq24_e1182_d_b3, eq24_e1182_d_b4, eq24_e1182_d_b5, eq24_e1182_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1184;
        let eq24_node_derivatives: [f64; 13] = [eq24_e1184_d_n0, eq24_e1184_d_n1, eq24_e1184_d_n2, eq24_e1184_d_n3, eq24_e1184_d_n4, eq24_e1184_d_n5, eq24_e1184_d_n6, eq24_e1184_d_n7, eq24_e1184_d_n8, eq24_e1184_d_n9, eq24_e1184_d_n10, eq24_e1184_d_n11, eq24_e1184_d_n12];
        let eq24_branch_derivatives: [f64; 7] = [eq24_e1184_d_b0, eq24_e1184_d_b1, eq24_e1184_d_b2, eq24_e1184_d_b3, eq24_e1184_d_b4, eq24_e1184_d_b5, eq24_e1184_d_b6];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(10),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq26_e1199,) = {
    if (!s.b[2719]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1199;
        stamper.stamp_potential_const_local(
            3,
            eq26_value,
        );
        let (eq27_e1209, eq27_e1209_d_n0, eq27_e1209_d_n1, eq27_e1209_d_n2, eq27_e1209_d_n3, eq27_e1209_d_n4, eq27_e1209_d_n5, eq27_e1209_d_n6, eq27_e1209_d_n7, eq27_e1209_d_n8, eq27_e1209_d_n9, eq27_e1209_d_n10, eq27_e1209_d_n11, eq27_e1209_d_n12, eq27_e1209_d_b0, eq27_e1209_d_b1, eq27_e1209_d_b2, eq27_e1209_d_b3, eq27_e1209_d_b4, eq27_e1209_d_b5, eq27_e1209_d_b6,) = {
    if s.b[2720] {
        let eq27_e1203: f64 = (s.v[15] * p.p32);
        let eq27_e1203_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq27_e1203_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq27_e1203_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq27_e1203_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq27_e1203_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq27_e1203_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq27_e1203_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq27_e1203_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq27_e1203_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq27_e1203_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq27_e1203_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq27_e1203_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq27_e1203_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq27_e1203_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq27_e1203_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq27_e1203_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq27_e1203_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq27_e1203_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq27_e1203_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq27_e1203_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq27_e1205: f64 = (eq27_e1203 * s.v[804]);
        let eq27_e1205_d_n0: f64 = ((eq27_e1203_d_n0 * s.v[804]) + (eq27_e1203 * s.dn[804][0]));
        let eq27_e1205_d_n1: f64 = ((eq27_e1203_d_n1 * s.v[804]) + (eq27_e1203 * s.dn[804][1]));
        let eq27_e1205_d_n2: f64 = ((eq27_e1203_d_n2 * s.v[804]) + (eq27_e1203 * s.dn[804][2]));
        let eq27_e1205_d_n3: f64 = ((eq27_e1203_d_n3 * s.v[804]) + (eq27_e1203 * s.dn[804][3]));
        let eq27_e1205_d_n4: f64 = ((eq27_e1203_d_n4 * s.v[804]) + (eq27_e1203 * s.dn[804][4]));
        let eq27_e1205_d_n5: f64 = ((eq27_e1203_d_n5 * s.v[804]) + (eq27_e1203 * s.dn[804][5]));
        let eq27_e1205_d_n6: f64 = ((eq27_e1203_d_n6 * s.v[804]) + (eq27_e1203 * s.dn[804][6]));
        let eq27_e1205_d_n7: f64 = ((eq27_e1203_d_n7 * s.v[804]) + (eq27_e1203 * s.dn[804][7]));
        let eq27_e1205_d_n8: f64 = ((eq27_e1203_d_n8 * s.v[804]) + (eq27_e1203 * s.dn[804][8]));
        let eq27_e1205_d_n9: f64 = ((eq27_e1203_d_n9 * s.v[804]) + (eq27_e1203 * s.dn[804][9]));
        let eq27_e1205_d_n10: f64 = ((eq27_e1203_d_n10 * s.v[804]) + (eq27_e1203 * s.dn[804][10]));
        let eq27_e1205_d_n11: f64 = ((eq27_e1203_d_n11 * s.v[804]) + (eq27_e1203 * s.dn[804][11]));
        let eq27_e1205_d_n12: f64 = ((eq27_e1203_d_n12 * s.v[804]) + (eq27_e1203 * s.dn[804][12]));
        let eq27_e1205_d_b0: f64 = ((eq27_e1203_d_b0 * s.v[804]) + (eq27_e1203 * s.db[804][0]));
        let eq27_e1205_d_b1: f64 = ((eq27_e1203_d_b1 * s.v[804]) + (eq27_e1203 * s.db[804][1]));
        let eq27_e1205_d_b2: f64 = ((eq27_e1203_d_b2 * s.v[804]) + (eq27_e1203 * s.db[804][2]));
        let eq27_e1205_d_b3: f64 = ((eq27_e1203_d_b3 * s.v[804]) + (eq27_e1203 * s.db[804][3]));
        let eq27_e1205_d_b4: f64 = ((eq27_e1203_d_b4 * s.v[804]) + (eq27_e1203 * s.db[804][4]));
        let eq27_e1205_d_b5: f64 = ((eq27_e1203_d_b5 * s.v[804]) + (eq27_e1203 * s.db[804][5]));
        let eq27_e1205_d_b6: f64 = ((eq27_e1203_d_b6 * s.v[804]) + (eq27_e1203 * s.db[804][6]));
        let eq27_e1207: f64 = (eq27_e1205 * (nv11 - nv10));
        let eq27_e1207_d_n0: f64 = (eq27_e1205_d_n0 * (nv11 - nv10));
        let eq27_e1207_d_n1: f64 = (eq27_e1205_d_n1 * (nv11 - nv10));
        let eq27_e1207_d_n2: f64 = (eq27_e1205_d_n2 * (nv11 - nv10));
        let eq27_e1207_d_n3: f64 = (eq27_e1205_d_n3 * (nv11 - nv10));
        let eq27_e1207_d_n4: f64 = (eq27_e1205_d_n4 * (nv11 - nv10));
        let eq27_e1207_d_n5: f64 = (eq27_e1205_d_n5 * (nv11 - nv10));
        let eq27_e1207_d_n6: f64 = (eq27_e1205_d_n6 * (nv11 - nv10));
        let eq27_e1207_d_n7: f64 = (eq27_e1205_d_n7 * (nv11 - nv10));
        let eq27_e1207_d_n8: f64 = (eq27_e1205_d_n8 * (nv11 - nv10));
        let eq27_e1207_d_n9: f64 = (eq27_e1205_d_n9 * (nv11 - nv10));
        let eq27_e1207_d_n10: f64 = ((eq27_e1205_d_n10 * (nv11 - nv10)) + (-eq27_e1205));
        let eq27_e1207_d_n11: f64 = ((eq27_e1205_d_n11 * (nv11 - nv10)) + eq27_e1205);
        let eq27_e1207_d_n12: f64 = (eq27_e1205_d_n12 * (nv11 - nv10));
        let eq27_e1207_d_b0: f64 = (eq27_e1205_d_b0 * (nv11 - nv10));
        let eq27_e1207_d_b1: f64 = (eq27_e1205_d_b1 * (nv11 - nv10));
        let eq27_e1207_d_b2: f64 = (eq27_e1205_d_b2 * (nv11 - nv10));
        let eq27_e1207_d_b3: f64 = (eq27_e1205_d_b3 * (nv11 - nv10));
        let eq27_e1207_d_b4: f64 = (eq27_e1205_d_b4 * (nv11 - nv10));
        let eq27_e1207_d_b5: f64 = (eq27_e1205_d_b5 * (nv11 - nv10));
        let eq27_e1207_d_b6: f64 = (eq27_e1205_d_b6 * (nv11 - nv10));
        (eq27_e1207, eq27_e1207_d_n0, eq27_e1207_d_n1, eq27_e1207_d_n2, eq27_e1207_d_n3, eq27_e1207_d_n4, eq27_e1207_d_n5, eq27_e1207_d_n6, eq27_e1207_d_n7, eq27_e1207_d_n8, eq27_e1207_d_n9, eq27_e1207_d_n10, eq27_e1207_d_n11, eq27_e1207_d_n12, eq27_e1207_d_b0, eq27_e1207_d_b1, eq27_e1207_d_b2, eq27_e1207_d_b3, eq27_e1207_d_b4, eq27_e1207_d_b5, eq27_e1207_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1209;
        let eq27_node_derivatives: [f64; 13] = [eq27_e1209_d_n0, eq27_e1209_d_n1, eq27_e1209_d_n2, eq27_e1209_d_n3, eq27_e1209_d_n4, eq27_e1209_d_n5, eq27_e1209_d_n6, eq27_e1209_d_n7, eq27_e1209_d_n8, eq27_e1209_d_n9, eq27_e1209_d_n10, eq27_e1209_d_n11, eq27_e1209_d_n12];
        let eq27_branch_derivatives: [f64; 7] = [eq27_e1209_d_b0, eq27_e1209_d_b1, eq27_e1209_d_b2, eq27_e1209_d_b3, eq27_e1209_d_b4, eq27_e1209_d_b5, eq27_e1209_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(10),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq29_e1224,) = {
    if (!s.b[2720]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1224;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
        let (eq30_e1234, eq30_e1234_d_n0, eq30_e1234_d_n1, eq30_e1234_d_n2, eq30_e1234_d_n3, eq30_e1234_d_n4, eq30_e1234_d_n5, eq30_e1234_d_n6, eq30_e1234_d_n7, eq30_e1234_d_n8, eq30_e1234_d_n9, eq30_e1234_d_n10, eq30_e1234_d_n11, eq30_e1234_d_n12, eq30_e1234_d_b0, eq30_e1234_d_b1, eq30_e1234_d_b2, eq30_e1234_d_b3, eq30_e1234_d_b4, eq30_e1234_d_b5, eq30_e1234_d_b6,) = {
    if s.b[2721] {
        let eq30_e1228: f64 = (s.v[15] * p.p32);
        let eq30_e1228_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq30_e1228_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq30_e1228_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq30_e1228_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq30_e1228_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq30_e1228_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq30_e1228_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq30_e1228_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq30_e1228_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq30_e1228_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq30_e1228_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq30_e1228_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq30_e1228_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq30_e1228_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq30_e1228_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq30_e1228_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq30_e1228_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq30_e1228_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq30_e1228_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq30_e1228_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq30_e1230: f64 = (eq30_e1228 * s.v[805]);
        let eq30_e1230_d_n0: f64 = ((eq30_e1228_d_n0 * s.v[805]) + (eq30_e1228 * s.dn[805][0]));
        let eq30_e1230_d_n1: f64 = ((eq30_e1228_d_n1 * s.v[805]) + (eq30_e1228 * s.dn[805][1]));
        let eq30_e1230_d_n2: f64 = ((eq30_e1228_d_n2 * s.v[805]) + (eq30_e1228 * s.dn[805][2]));
        let eq30_e1230_d_n3: f64 = ((eq30_e1228_d_n3 * s.v[805]) + (eq30_e1228 * s.dn[805][3]));
        let eq30_e1230_d_n4: f64 = ((eq30_e1228_d_n4 * s.v[805]) + (eq30_e1228 * s.dn[805][4]));
        let eq30_e1230_d_n5: f64 = ((eq30_e1228_d_n5 * s.v[805]) + (eq30_e1228 * s.dn[805][5]));
        let eq30_e1230_d_n6: f64 = ((eq30_e1228_d_n6 * s.v[805]) + (eq30_e1228 * s.dn[805][6]));
        let eq30_e1230_d_n7: f64 = ((eq30_e1228_d_n7 * s.v[805]) + (eq30_e1228 * s.dn[805][7]));
        let eq30_e1230_d_n8: f64 = ((eq30_e1228_d_n8 * s.v[805]) + (eq30_e1228 * s.dn[805][8]));
        let eq30_e1230_d_n9: f64 = ((eq30_e1228_d_n9 * s.v[805]) + (eq30_e1228 * s.dn[805][9]));
        let eq30_e1230_d_n10: f64 = ((eq30_e1228_d_n10 * s.v[805]) + (eq30_e1228 * s.dn[805][10]));
        let eq30_e1230_d_n11: f64 = ((eq30_e1228_d_n11 * s.v[805]) + (eq30_e1228 * s.dn[805][11]));
        let eq30_e1230_d_n12: f64 = ((eq30_e1228_d_n12 * s.v[805]) + (eq30_e1228 * s.dn[805][12]));
        let eq30_e1230_d_b0: f64 = ((eq30_e1228_d_b0 * s.v[805]) + (eq30_e1228 * s.db[805][0]));
        let eq30_e1230_d_b1: f64 = ((eq30_e1228_d_b1 * s.v[805]) + (eq30_e1228 * s.db[805][1]));
        let eq30_e1230_d_b2: f64 = ((eq30_e1228_d_b2 * s.v[805]) + (eq30_e1228 * s.db[805][2]));
        let eq30_e1230_d_b3: f64 = ((eq30_e1228_d_b3 * s.v[805]) + (eq30_e1228 * s.db[805][3]));
        let eq30_e1230_d_b4: f64 = ((eq30_e1228_d_b4 * s.v[805]) + (eq30_e1228 * s.db[805][4]));
        let eq30_e1230_d_b5: f64 = ((eq30_e1228_d_b5 * s.v[805]) + (eq30_e1228 * s.db[805][5]));
        let eq30_e1230_d_b6: f64 = ((eq30_e1228_d_b6 * s.v[805]) + (eq30_e1228 * s.db[805][6]));
        let eq30_e1232: f64 = (eq30_e1230 * (nv12 - nv10));
        let eq30_e1232_d_n0: f64 = (eq30_e1230_d_n0 * (nv12 - nv10));
        let eq30_e1232_d_n1: f64 = (eq30_e1230_d_n1 * (nv12 - nv10));
        let eq30_e1232_d_n2: f64 = (eq30_e1230_d_n2 * (nv12 - nv10));
        let eq30_e1232_d_n3: f64 = (eq30_e1230_d_n3 * (nv12 - nv10));
        let eq30_e1232_d_n4: f64 = (eq30_e1230_d_n4 * (nv12 - nv10));
        let eq30_e1232_d_n5: f64 = (eq30_e1230_d_n5 * (nv12 - nv10));
        let eq30_e1232_d_n6: f64 = (eq30_e1230_d_n6 * (nv12 - nv10));
        let eq30_e1232_d_n7: f64 = (eq30_e1230_d_n7 * (nv12 - nv10));
        let eq30_e1232_d_n8: f64 = (eq30_e1230_d_n8 * (nv12 - nv10));
        let eq30_e1232_d_n9: f64 = (eq30_e1230_d_n9 * (nv12 - nv10));
        let eq30_e1232_d_n10: f64 = ((eq30_e1230_d_n10 * (nv12 - nv10)) + (-eq30_e1230));
        let eq30_e1232_d_n11: f64 = (eq30_e1230_d_n11 * (nv12 - nv10));
        let eq30_e1232_d_n12: f64 = ((eq30_e1230_d_n12 * (nv12 - nv10)) + eq30_e1230);
        let eq30_e1232_d_b0: f64 = (eq30_e1230_d_b0 * (nv12 - nv10));
        let eq30_e1232_d_b1: f64 = (eq30_e1230_d_b1 * (nv12 - nv10));
        let eq30_e1232_d_b2: f64 = (eq30_e1230_d_b2 * (nv12 - nv10));
        let eq30_e1232_d_b3: f64 = (eq30_e1230_d_b3 * (nv12 - nv10));
        let eq30_e1232_d_b4: f64 = (eq30_e1230_d_b4 * (nv12 - nv10));
        let eq30_e1232_d_b5: f64 = (eq30_e1230_d_b5 * (nv12 - nv10));
        let eq30_e1232_d_b6: f64 = (eq30_e1230_d_b6 * (nv12 - nv10));
        (eq30_e1232, eq30_e1232_d_n0, eq30_e1232_d_n1, eq30_e1232_d_n2, eq30_e1232_d_n3, eq30_e1232_d_n4, eq30_e1232_d_n5, eq30_e1232_d_n6, eq30_e1232_d_n7, eq30_e1232_d_n8, eq30_e1232_d_n9, eq30_e1232_d_n10, eq30_e1232_d_n11, eq30_e1232_d_n12, eq30_e1232_d_b0, eq30_e1232_d_b1, eq30_e1232_d_b2, eq30_e1232_d_b3, eq30_e1232_d_b4, eq30_e1232_d_b5, eq30_e1232_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1234;
        let eq30_node_derivatives: [f64; 13] = [eq30_e1234_d_n0, eq30_e1234_d_n1, eq30_e1234_d_n2, eq30_e1234_d_n3, eq30_e1234_d_n4, eq30_e1234_d_n5, eq30_e1234_d_n6, eq30_e1234_d_n7, eq30_e1234_d_n8, eq30_e1234_d_n9, eq30_e1234_d_n10, eq30_e1234_d_n11, eq30_e1234_d_n12];
        let eq30_branch_derivatives: [f64; 7] = [eq30_e1234_d_b0, eq30_e1234_d_b1, eq30_e1234_d_b2, eq30_e1234_d_b3, eq30_e1234_d_b4, eq30_e1234_d_b5, eq30_e1234_d_b6];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(10),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq32_e1249,) = {
    if (!s.b[2721]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1249;
        stamper.stamp_potential_const_local(
            5,
            eq32_value,
        );
        let (eq33_e1259, eq33_e1259_d_n0, eq33_e1259_d_n1, eq33_e1259_d_n2, eq33_e1259_d_n3, eq33_e1259_d_n4, eq33_e1259_d_n5, eq33_e1259_d_n6, eq33_e1259_d_n7, eq33_e1259_d_n8, eq33_e1259_d_n9, eq33_e1259_d_n10, eq33_e1259_d_n11, eq33_e1259_d_n12, eq33_e1259_d_b0, eq33_e1259_d_b1, eq33_e1259_d_b2, eq33_e1259_d_b3, eq33_e1259_d_b4, eq33_e1259_d_b5, eq33_e1259_d_b6,) = {
    if s.b[2722] {
        let eq33_e1253: f64 = (s.v[15] * p.p32);
        let eq33_e1253_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq33_e1253_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq33_e1253_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq33_e1253_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq33_e1253_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq33_e1253_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq33_e1253_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq33_e1253_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq33_e1253_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq33_e1253_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq33_e1253_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq33_e1253_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq33_e1253_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq33_e1253_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq33_e1253_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq33_e1253_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq33_e1253_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq33_e1253_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq33_e1253_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq33_e1253_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq33_e1255: f64 = (eq33_e1253 * s.v[806]);
        let eq33_e1255_d_n0: f64 = ((eq33_e1253_d_n0 * s.v[806]) + (eq33_e1253 * s.dn[806][0]));
        let eq33_e1255_d_n1: f64 = ((eq33_e1253_d_n1 * s.v[806]) + (eq33_e1253 * s.dn[806][1]));
        let eq33_e1255_d_n2: f64 = ((eq33_e1253_d_n2 * s.v[806]) + (eq33_e1253 * s.dn[806][2]));
        let eq33_e1255_d_n3: f64 = ((eq33_e1253_d_n3 * s.v[806]) + (eq33_e1253 * s.dn[806][3]));
        let eq33_e1255_d_n4: f64 = ((eq33_e1253_d_n4 * s.v[806]) + (eq33_e1253 * s.dn[806][4]));
        let eq33_e1255_d_n5: f64 = ((eq33_e1253_d_n5 * s.v[806]) + (eq33_e1253 * s.dn[806][5]));
        let eq33_e1255_d_n6: f64 = ((eq33_e1253_d_n6 * s.v[806]) + (eq33_e1253 * s.dn[806][6]));
        let eq33_e1255_d_n7: f64 = ((eq33_e1253_d_n7 * s.v[806]) + (eq33_e1253 * s.dn[806][7]));
        let eq33_e1255_d_n8: f64 = ((eq33_e1253_d_n8 * s.v[806]) + (eq33_e1253 * s.dn[806][8]));
        let eq33_e1255_d_n9: f64 = ((eq33_e1253_d_n9 * s.v[806]) + (eq33_e1253 * s.dn[806][9]));
        let eq33_e1255_d_n10: f64 = ((eq33_e1253_d_n10 * s.v[806]) + (eq33_e1253 * s.dn[806][10]));
        let eq33_e1255_d_n11: f64 = ((eq33_e1253_d_n11 * s.v[806]) + (eq33_e1253 * s.dn[806][11]));
        let eq33_e1255_d_n12: f64 = ((eq33_e1253_d_n12 * s.v[806]) + (eq33_e1253 * s.dn[806][12]));
        let eq33_e1255_d_b0: f64 = ((eq33_e1253_d_b0 * s.v[806]) + (eq33_e1253 * s.db[806][0]));
        let eq33_e1255_d_b1: f64 = ((eq33_e1253_d_b1 * s.v[806]) + (eq33_e1253 * s.db[806][1]));
        let eq33_e1255_d_b2: f64 = ((eq33_e1253_d_b2 * s.v[806]) + (eq33_e1253 * s.db[806][2]));
        let eq33_e1255_d_b3: f64 = ((eq33_e1253_d_b3 * s.v[806]) + (eq33_e1253 * s.db[806][3]));
        let eq33_e1255_d_b4: f64 = ((eq33_e1253_d_b4 * s.v[806]) + (eq33_e1253 * s.db[806][4]));
        let eq33_e1255_d_b5: f64 = ((eq33_e1253_d_b5 * s.v[806]) + (eq33_e1253 * s.db[806][5]));
        let eq33_e1255_d_b6: f64 = ((eq33_e1253_d_b6 * s.v[806]) + (eq33_e1253 * s.db[806][6]));
        let eq33_e1257: f64 = (eq33_e1255 * (nv3 - nv10));
        let eq33_e1257_d_n0: f64 = (eq33_e1255_d_n0 * (nv3 - nv10));
        let eq33_e1257_d_n1: f64 = (eq33_e1255_d_n1 * (nv3 - nv10));
        let eq33_e1257_d_n2: f64 = (eq33_e1255_d_n2 * (nv3 - nv10));
        let eq33_e1257_d_n3: f64 = ((eq33_e1255_d_n3 * (nv3 - nv10)) + eq33_e1255);
        let eq33_e1257_d_n4: f64 = (eq33_e1255_d_n4 * (nv3 - nv10));
        let eq33_e1257_d_n5: f64 = (eq33_e1255_d_n5 * (nv3 - nv10));
        let eq33_e1257_d_n6: f64 = (eq33_e1255_d_n6 * (nv3 - nv10));
        let eq33_e1257_d_n7: f64 = (eq33_e1255_d_n7 * (nv3 - nv10));
        let eq33_e1257_d_n8: f64 = (eq33_e1255_d_n8 * (nv3 - nv10));
        let eq33_e1257_d_n9: f64 = (eq33_e1255_d_n9 * (nv3 - nv10));
        let eq33_e1257_d_n10: f64 = ((eq33_e1255_d_n10 * (nv3 - nv10)) + (-eq33_e1255));
        let eq33_e1257_d_n11: f64 = (eq33_e1255_d_n11 * (nv3 - nv10));
        let eq33_e1257_d_n12: f64 = (eq33_e1255_d_n12 * (nv3 - nv10));
        let eq33_e1257_d_b0: f64 = (eq33_e1255_d_b0 * (nv3 - nv10));
        let eq33_e1257_d_b1: f64 = (eq33_e1255_d_b1 * (nv3 - nv10));
        let eq33_e1257_d_b2: f64 = (eq33_e1255_d_b2 * (nv3 - nv10));
        let eq33_e1257_d_b3: f64 = (eq33_e1255_d_b3 * (nv3 - nv10));
        let eq33_e1257_d_b4: f64 = (eq33_e1255_d_b4 * (nv3 - nv10));
        let eq33_e1257_d_b5: f64 = (eq33_e1255_d_b5 * (nv3 - nv10));
        let eq33_e1257_d_b6: f64 = (eq33_e1255_d_b6 * (nv3 - nv10));
        (eq33_e1257, eq33_e1257_d_n0, eq33_e1257_d_n1, eq33_e1257_d_n2, eq33_e1257_d_n3, eq33_e1257_d_n4, eq33_e1257_d_n5, eq33_e1257_d_n6, eq33_e1257_d_n7, eq33_e1257_d_n8, eq33_e1257_d_n9, eq33_e1257_d_n10, eq33_e1257_d_n11, eq33_e1257_d_n12, eq33_e1257_d_b0, eq33_e1257_d_b1, eq33_e1257_d_b2, eq33_e1257_d_b3, eq33_e1257_d_b4, eq33_e1257_d_b5, eq33_e1257_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1259;
        let eq33_node_derivatives: [f64; 13] = [eq33_e1259_d_n0, eq33_e1259_d_n1, eq33_e1259_d_n2, eq33_e1259_d_n3, eq33_e1259_d_n4, eq33_e1259_d_n5, eq33_e1259_d_n6, eq33_e1259_d_n7, eq33_e1259_d_n8, eq33_e1259_d_n9, eq33_e1259_d_n10, eq33_e1259_d_n11, eq33_e1259_d_n12];
        let eq33_branch_derivatives: [f64; 7] = [eq33_e1259_d_b0, eq33_e1259_d_b1, eq33_e1259_d_b2, eq33_e1259_d_b3, eq33_e1259_d_b4, eq33_e1259_d_b5, eq33_e1259_d_b6];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(10),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq35_e1274,) = {
    if (!s.b[2722]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1274;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq38_e1286: f64 = (-s.v[15]);
        let eq38_e1286_d_n0: f64 = (-s.dn[15][0]);
        let eq38_e1286_d_n1: f64 = (-s.dn[15][1]);
        let eq38_e1286_d_n2: f64 = (-s.dn[15][2]);
        let eq38_e1286_d_n3: f64 = (-s.dn[15][3]);
        let eq38_e1286_d_n4: f64 = (-s.dn[15][4]);
        let eq38_e1286_d_n5: f64 = (-s.dn[15][5]);
        let eq38_e1286_d_n6: f64 = (-s.dn[15][6]);
        let eq38_e1286_d_n7: f64 = (-s.dn[15][7]);
        let eq38_e1286_d_n8: f64 = (-s.dn[15][8]);
        let eq38_e1286_d_n9: f64 = (-s.dn[15][9]);
        let eq38_e1286_d_n10: f64 = (-s.dn[15][10]);
        let eq38_e1286_d_n11: f64 = (-s.dn[15][11]);
        let eq38_e1286_d_n12: f64 = (-s.dn[15][12]);
        let eq38_e1286_d_b0: f64 = (-s.db[15][0]);
        let eq38_e1286_d_b1: f64 = (-s.db[15][1]);
        let eq38_e1286_d_b2: f64 = (-s.db[15][2]);
        let eq38_e1286_d_b3: f64 = (-s.db[15][3]);
        let eq38_e1286_d_b4: f64 = (-s.db[15][4]);
        let eq38_e1286_d_b5: f64 = (-s.db[15][5]);
        let eq38_e1286_d_b6: f64 = (-s.db[15][6]);
        let eq38_e1288: f64 = (eq38_e1286 * s.v[1915]);
        let eq38_e1288_d_n0: f64 = ((eq38_e1286_d_n0 * s.v[1915]) + (eq38_e1286 * s.dn[1915][0]));
        let eq38_e1288_d_n1: f64 = ((eq38_e1286_d_n1 * s.v[1915]) + (eq38_e1286 * s.dn[1915][1]));
        let eq38_e1288_d_n2: f64 = ((eq38_e1286_d_n2 * s.v[1915]) + (eq38_e1286 * s.dn[1915][2]));
        let eq38_e1288_d_n3: f64 = ((eq38_e1286_d_n3 * s.v[1915]) + (eq38_e1286 * s.dn[1915][3]));
        let eq38_e1288_d_n4: f64 = ((eq38_e1286_d_n4 * s.v[1915]) + (eq38_e1286 * s.dn[1915][4]));
        let eq38_e1288_d_n5: f64 = ((eq38_e1286_d_n5 * s.v[1915]) + (eq38_e1286 * s.dn[1915][5]));
        let eq38_e1288_d_n6: f64 = ((eq38_e1286_d_n6 * s.v[1915]) + (eq38_e1286 * s.dn[1915][6]));
        let eq38_e1288_d_n7: f64 = ((eq38_e1286_d_n7 * s.v[1915]) + (eq38_e1286 * s.dn[1915][7]));
        let eq38_e1288_d_n8: f64 = ((eq38_e1286_d_n8 * s.v[1915]) + (eq38_e1286 * s.dn[1915][8]));
        let eq38_e1288_d_n9: f64 = ((eq38_e1286_d_n9 * s.v[1915]) + (eq38_e1286 * s.dn[1915][9]));
        let eq38_e1288_d_n10: f64 = ((eq38_e1286_d_n10 * s.v[1915]) + (eq38_e1286 * s.dn[1915][10]));
        let eq38_e1288_d_n11: f64 = ((eq38_e1286_d_n11 * s.v[1915]) + (eq38_e1286 * s.dn[1915][11]));
        let eq38_e1288_d_n12: f64 = ((eq38_e1286_d_n12 * s.v[1915]) + (eq38_e1286 * s.dn[1915][12]));
        let eq38_e1288_d_b0: f64 = ((eq38_e1286_d_b0 * s.v[1915]) + (eq38_e1286 * s.db[1915][0]));
        let eq38_e1288_d_b1: f64 = ((eq38_e1286_d_b1 * s.v[1915]) + (eq38_e1286 * s.db[1915][1]));
        let eq38_e1288_d_b2: f64 = ((eq38_e1286_d_b2 * s.v[1915]) + (eq38_e1286 * s.db[1915][2]));
        let eq38_e1288_d_b3: f64 = ((eq38_e1286_d_b3 * s.v[1915]) + (eq38_e1286 * s.db[1915][3]));
        let eq38_e1288_d_b4: f64 = ((eq38_e1286_d_b4 * s.v[1915]) + (eq38_e1286 * s.db[1915][4]));
        let eq38_e1288_d_b5: f64 = ((eq38_e1286_d_b5 * s.v[1915]) + (eq38_e1286 * s.db[1915][5]));
        let eq38_e1288_d_b6: f64 = ((eq38_e1286_d_b6 * s.v[1915]) + (eq38_e1286 * s.db[1915][6]));
        let eq38_value: f64 = eq38_e1288;
        let eq38_node_derivatives: [f64; 13] = [eq38_e1288_d_n0, eq38_e1288_d_n1, eq38_e1288_d_n2, eq38_e1288_d_n3, eq38_e1288_d_n4, eq38_e1288_d_n5, eq38_e1288_d_n6, eq38_e1288_d_n7, eq38_e1288_d_n8, eq38_e1288_d_n9, eq38_e1288_d_n10, eq38_e1288_d_n11, eq38_e1288_d_n12];
        let eq38_branch_derivatives: [f64; 7] = [eq38_e1288_d_b0, eq38_e1288_d_b1, eq38_e1288_d_b2, eq38_e1288_d_b3, eq38_e1288_d_b4, eq38_e1288_d_b5, eq38_e1288_d_b6];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let eq39_e1291: f64 = (s.v[15] * s.v[306]);
        let eq39_e1291_d_n0: f64 = ((s.dn[15][0] * s.v[306]) + (s.v[15] * s.dn[306][0]));
        let eq39_e1291_d_n1: f64 = ((s.dn[15][1] * s.v[306]) + (s.v[15] * s.dn[306][1]));
        let eq39_e1291_d_n2: f64 = ((s.dn[15][2] * s.v[306]) + (s.v[15] * s.dn[306][2]));
        let eq39_e1291_d_n3: f64 = ((s.dn[15][3] * s.v[306]) + (s.v[15] * s.dn[306][3]));
        let eq39_e1291_d_n4: f64 = ((s.dn[15][4] * s.v[306]) + (s.v[15] * s.dn[306][4]));
        let eq39_e1291_d_n5: f64 = ((s.dn[15][5] * s.v[306]) + (s.v[15] * s.dn[306][5]));
        let eq39_e1291_d_n6: f64 = ((s.dn[15][6] * s.v[306]) + (s.v[15] * s.dn[306][6]));
        let eq39_e1291_d_n7: f64 = ((s.dn[15][7] * s.v[306]) + (s.v[15] * s.dn[306][7]));
        let eq39_e1291_d_n8: f64 = ((s.dn[15][8] * s.v[306]) + (s.v[15] * s.dn[306][8]));
        let eq39_e1291_d_n9: f64 = ((s.dn[15][9] * s.v[306]) + (s.v[15] * s.dn[306][9]));
        let eq39_e1291_d_n10: f64 = ((s.dn[15][10] * s.v[306]) + (s.v[15] * s.dn[306][10]));
        let eq39_e1291_d_n11: f64 = ((s.dn[15][11] * s.v[306]) + (s.v[15] * s.dn[306][11]));
        let eq39_e1291_d_n12: f64 = ((s.dn[15][12] * s.v[306]) + (s.v[15] * s.dn[306][12]));
        let eq39_e1291_d_b0: f64 = ((s.db[15][0] * s.v[306]) + (s.v[15] * s.db[306][0]));
        let eq39_e1291_d_b1: f64 = ((s.db[15][1] * s.v[306]) + (s.v[15] * s.db[306][1]));
        let eq39_e1291_d_b2: f64 = ((s.db[15][2] * s.v[306]) + (s.v[15] * s.db[306][2]));
        let eq39_e1291_d_b3: f64 = ((s.db[15][3] * s.v[306]) + (s.v[15] * s.db[306][3]));
        let eq39_e1291_d_b4: f64 = ((s.db[15][4] * s.v[306]) + (s.v[15] * s.db[306][4]));
        let eq39_e1291_d_b5: f64 = ((s.db[15][5] * s.v[306]) + (s.v[15] * s.db[306][5]));
        let eq39_e1291_d_b6: f64 = ((s.db[15][6] * s.v[306]) + (s.v[15] * s.db[306][6]));
        let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));
        let eq39_e1293_d_n0: f64 = (eq39_e1291_d_n0 * (nv4 - 0.0));
        let eq39_e1293_d_n1: f64 = (eq39_e1291_d_n1 * (nv4 - 0.0));
        let eq39_e1293_d_n2: f64 = (eq39_e1291_d_n2 * (nv4 - 0.0));
        let eq39_e1293_d_n3: f64 = (eq39_e1291_d_n3 * (nv4 - 0.0));
        let eq39_e1293_d_n4: f64 = ((eq39_e1291_d_n4 * (nv4 - 0.0)) + eq39_e1291);
        let eq39_e1293_d_n5: f64 = (eq39_e1291_d_n5 * (nv4 - 0.0));
        let eq39_e1293_d_n6: f64 = (eq39_e1291_d_n6 * (nv4 - 0.0));
        let eq39_e1293_d_n7: f64 = (eq39_e1291_d_n7 * (nv4 - 0.0));
        let eq39_e1293_d_n8: f64 = (eq39_e1291_d_n8 * (nv4 - 0.0));
        let eq39_e1293_d_n9: f64 = (eq39_e1291_d_n9 * (nv4 - 0.0));
        let eq39_e1293_d_n10: f64 = (eq39_e1291_d_n10 * (nv4 - 0.0));
        let eq39_e1293_d_n11: f64 = (eq39_e1291_d_n11 * (nv4 - 0.0));
        let eq39_e1293_d_n12: f64 = (eq39_e1291_d_n12 * (nv4 - 0.0));
        let eq39_e1293_d_b0: f64 = (eq39_e1291_d_b0 * (nv4 - 0.0));
        let eq39_e1293_d_b1: f64 = (eq39_e1291_d_b1 * (nv4 - 0.0));
        let eq39_e1293_d_b2: f64 = (eq39_e1291_d_b2 * (nv4 - 0.0));
        let eq39_e1293_d_b3: f64 = (eq39_e1291_d_b3 * (nv4 - 0.0));
        let eq39_e1293_d_b4: f64 = (eq39_e1291_d_b4 * (nv4 - 0.0));
        let eq39_e1293_d_b5: f64 = (eq39_e1291_d_b5 * (nv4 - 0.0));
        let eq39_e1293_d_b6: f64 = (eq39_e1291_d_b6 * (nv4 - 0.0));
        let eq39_e1294: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq39_e1293);
        let eq39_e1294_d_n0: f64 = (eq39_e1293_d_n0 * ddt_scale);
        let eq39_e1294_d_n1: f64 = (eq39_e1293_d_n1 * ddt_scale);
        let eq39_e1294_d_n2: f64 = (eq39_e1293_d_n2 * ddt_scale);
        let eq39_e1294_d_n3: f64 = (eq39_e1293_d_n3 * ddt_scale);
        let eq39_e1294_d_n4: f64 = (eq39_e1293_d_n4 * ddt_scale);
        let eq39_e1294_d_n5: f64 = (eq39_e1293_d_n5 * ddt_scale);
        let eq39_e1294_d_n6: f64 = (eq39_e1293_d_n6 * ddt_scale);
        let eq39_e1294_d_n7: f64 = (eq39_e1293_d_n7 * ddt_scale);
        let eq39_e1294_d_n8: f64 = (eq39_e1293_d_n8 * ddt_scale);
        let eq39_e1294_d_n9: f64 = (eq39_e1293_d_n9 * ddt_scale);
        let eq39_e1294_d_n10: f64 = (eq39_e1293_d_n10 * ddt_scale);
        let eq39_e1294_d_n11: f64 = (eq39_e1293_d_n11 * ddt_scale);
        let eq39_e1294_d_n12: f64 = (eq39_e1293_d_n12 * ddt_scale);
        let eq39_e1294_d_b0: f64 = (eq39_e1293_d_b0 * ddt_scale);
        let eq39_e1294_d_b1: f64 = (eq39_e1293_d_b1 * ddt_scale);
        let eq39_e1294_d_b2: f64 = (eq39_e1293_d_b2 * ddt_scale);
        let eq39_e1294_d_b3: f64 = (eq39_e1293_d_b3 * ddt_scale);
        let eq39_e1294_d_b4: f64 = (eq39_e1293_d_b4 * ddt_scale);
        let eq39_e1294_d_b5: f64 = (eq39_e1293_d_b5 * ddt_scale);
        let eq39_e1294_d_b6: f64 = (eq39_e1293_d_b6 * ddt_scale);
        let eq39_value: f64 = eq39_e1294;
        let eq39_node_derivatives: [f64; 13] = [eq39_e1294_d_n0, eq39_e1294_d_n1, eq39_e1294_d_n2, eq39_e1294_d_n3, eq39_e1294_d_n4, eq39_e1294_d_n5, eq39_e1294_d_n6, eq39_e1294_d_n7, eq39_e1294_d_n8, eq39_e1294_d_n9, eq39_e1294_d_n10, eq39_e1294_d_n11, eq39_e1294_d_n12];
        let eq39_branch_derivatives: [f64; 7] = [eq39_e1294_d_b0, eq39_e1294_d_b1, eq39_e1294_d_b2, eq39_e1294_d_b3, eq39_e1294_d_b4, eq39_e1294_d_b5, eq39_e1294_d_b6];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let eq40_e1297: f64 = (s.v[15] * (nv4 - 0.0));
        let eq40_e1297_d_n0: f64 = (s.dn[15][0] * (nv4 - 0.0));
        let eq40_e1297_d_n1: f64 = (s.dn[15][1] * (nv4 - 0.0));
        let eq40_e1297_d_n2: f64 = (s.dn[15][2] * (nv4 - 0.0));
        let eq40_e1297_d_n3: f64 = (s.dn[15][3] * (nv4 - 0.0));
        let eq40_e1297_d_n4: f64 = ((s.dn[15][4] * (nv4 - 0.0)) + s.v[15]);
        let eq40_e1297_d_n5: f64 = (s.dn[15][5] * (nv4 - 0.0));
        let eq40_e1297_d_n6: f64 = (s.dn[15][6] * (nv4 - 0.0));
        let eq40_e1297_d_n7: f64 = (s.dn[15][7] * (nv4 - 0.0));
        let eq40_e1297_d_n8: f64 = (s.dn[15][8] * (nv4 - 0.0));
        let eq40_e1297_d_n9: f64 = (s.dn[15][9] * (nv4 - 0.0));
        let eq40_e1297_d_n10: f64 = (s.dn[15][10] * (nv4 - 0.0));
        let eq40_e1297_d_n11: f64 = (s.dn[15][11] * (nv4 - 0.0));
        let eq40_e1297_d_n12: f64 = (s.dn[15][12] * (nv4 - 0.0));
        let eq40_e1297_d_b0: f64 = (s.db[15][0] * (nv4 - 0.0));
        let eq40_e1297_d_b1: f64 = (s.db[15][1] * (nv4 - 0.0));
        let eq40_e1297_d_b2: f64 = (s.db[15][2] * (nv4 - 0.0));
        let eq40_e1297_d_b3: f64 = (s.db[15][3] * (nv4 - 0.0));
        let eq40_e1297_d_b4: f64 = (s.db[15][4] * (nv4 - 0.0));
        let eq40_e1297_d_b5: f64 = (s.db[15][5] * (nv4 - 0.0));
        let eq40_e1297_d_b6: f64 = (s.db[15][6] * (nv4 - 0.0));
        let eq40_e1299: f64 = (eq40_e1297 / s.v[716]);
        let eq40_e1299_d_n0: f64 = (((eq40_e1297_d_n0 * s.v[716]) - (eq40_e1297 * s.dn[716][0])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n1: f64 = (((eq40_e1297_d_n1 * s.v[716]) - (eq40_e1297 * s.dn[716][1])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n2: f64 = (((eq40_e1297_d_n2 * s.v[716]) - (eq40_e1297 * s.dn[716][2])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n3: f64 = (((eq40_e1297_d_n3 * s.v[716]) - (eq40_e1297 * s.dn[716][3])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n4: f64 = (((eq40_e1297_d_n4 * s.v[716]) - (eq40_e1297 * s.dn[716][4])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n5: f64 = (((eq40_e1297_d_n5 * s.v[716]) - (eq40_e1297 * s.dn[716][5])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n6: f64 = (((eq40_e1297_d_n6 * s.v[716]) - (eq40_e1297 * s.dn[716][6])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n7: f64 = (((eq40_e1297_d_n7 * s.v[716]) - (eq40_e1297 * s.dn[716][7])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n8: f64 = (((eq40_e1297_d_n8 * s.v[716]) - (eq40_e1297 * s.dn[716][8])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n9: f64 = (((eq40_e1297_d_n9 * s.v[716]) - (eq40_e1297 * s.dn[716][9])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n10: f64 = (((eq40_e1297_d_n10 * s.v[716]) - (eq40_e1297 * s.dn[716][10])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n11: f64 = (((eq40_e1297_d_n11 * s.v[716]) - (eq40_e1297 * s.dn[716][11])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_n12: f64 = (((eq40_e1297_d_n12 * s.v[716]) - (eq40_e1297 * s.dn[716][12])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_b0: f64 = (((eq40_e1297_d_b0 * s.v[716]) - (eq40_e1297 * s.db[716][0])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_b1: f64 = (((eq40_e1297_d_b1 * s.v[716]) - (eq40_e1297 * s.db[716][1])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_b2: f64 = (((eq40_e1297_d_b2 * s.v[716]) - (eq40_e1297 * s.db[716][2])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_b3: f64 = (((eq40_e1297_d_b3 * s.v[716]) - (eq40_e1297 * s.db[716][3])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_b4: f64 = (((eq40_e1297_d_b4 * s.v[716]) - (eq40_e1297 * s.db[716][4])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_b5: f64 = (((eq40_e1297_d_b5 * s.v[716]) - (eq40_e1297 * s.db[716][5])) / (s.v[716] * s.v[716]));
        let eq40_e1299_d_b6: f64 = (((eq40_e1297_d_b6 * s.v[716]) - (eq40_e1297 * s.db[716][6])) / (s.v[716] * s.v[716]));
        let eq40_value: f64 = eq40_e1299;
        let eq40_node_derivatives: [f64; 13] = [eq40_e1299_d_n0, eq40_e1299_d_n1, eq40_e1299_d_n2, eq40_e1299_d_n3, eq40_e1299_d_n4, eq40_e1299_d_n5, eq40_e1299_d_n6, eq40_e1299_d_n7, eq40_e1299_d_n8, eq40_e1299_d_n9, eq40_e1299_d_n10, eq40_e1299_d_n11, eq40_e1299_d_n12];
        let eq40_branch_derivatives: [f64; 7] = [eq40_e1299_d_b0, eq40_e1299_d_b1, eq40_e1299_d_b2, eq40_e1299_d_b3, eq40_e1299_d_b4, eq40_e1299_d_b5, eq40_e1299_d_b6];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e1302: f64 = (s.v[0] * s.v[15]);
        let eq41_e1302_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq41_e1302_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq41_e1302_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq41_e1302_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq41_e1302_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq41_e1302_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq41_e1302_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq41_e1302_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq41_e1302_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq41_e1302_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq41_e1302_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq41_e1302_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq41_e1302_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq41_e1302_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq41_e1302_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq41_e1302_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq41_e1302_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq41_e1302_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq41_e1302_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq41_e1302_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1304_d_n0: f64 = (eq41_e1302_d_n0 * p.p33);
        let eq41_e1304_d_n1: f64 = (eq41_e1302_d_n1 * p.p33);
        let eq41_e1304_d_n2: f64 = (eq41_e1302_d_n2 * p.p33);
        let eq41_e1304_d_n3: f64 = (eq41_e1302_d_n3 * p.p33);
        let eq41_e1304_d_n4: f64 = (eq41_e1302_d_n4 * p.p33);
        let eq41_e1304_d_n5: f64 = (eq41_e1302_d_n5 * p.p33);
        let eq41_e1304_d_n6: f64 = (eq41_e1302_d_n6 * p.p33);
        let eq41_e1304_d_n7: f64 = (eq41_e1302_d_n7 * p.p33);
        let eq41_e1304_d_n8: f64 = (eq41_e1302_d_n8 * p.p33);
        let eq41_e1304_d_n9: f64 = (eq41_e1302_d_n9 * p.p33);
        let eq41_e1304_d_n10: f64 = (eq41_e1302_d_n10 * p.p33);
        let eq41_e1304_d_n11: f64 = (eq41_e1302_d_n11 * p.p33);
        let eq41_e1304_d_n12: f64 = (eq41_e1302_d_n12 * p.p33);
        let eq41_e1304_d_b0: f64 = (eq41_e1302_d_b0 * p.p33);
        let eq41_e1304_d_b1: f64 = (eq41_e1302_d_b1 * p.p33);
        let eq41_e1304_d_b2: f64 = (eq41_e1302_d_b2 * p.p33);
        let eq41_e1304_d_b3: f64 = (eq41_e1302_d_b3 * p.p33);
        let eq41_e1304_d_b4: f64 = (eq41_e1302_d_b4 * p.p33);
        let eq41_e1304_d_b5: f64 = (eq41_e1302_d_b5 * p.p33);
        let eq41_e1304_d_b6: f64 = (eq41_e1302_d_b6 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * s.v[840]);
        let eq41_e1306_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[840]) + (eq41_e1304 * s.dn[840][0]));
        let eq41_e1306_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[840]) + (eq41_e1304 * s.dn[840][1]));
        let eq41_e1306_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[840]) + (eq41_e1304 * s.dn[840][2]));
        let eq41_e1306_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[840]) + (eq41_e1304 * s.dn[840][3]));
        let eq41_e1306_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[840]) + (eq41_e1304 * s.dn[840][4]));
        let eq41_e1306_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[840]) + (eq41_e1304 * s.dn[840][5]));
        let eq41_e1306_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[840]) + (eq41_e1304 * s.dn[840][6]));
        let eq41_e1306_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[840]) + (eq41_e1304 * s.dn[840][7]));
        let eq41_e1306_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[840]) + (eq41_e1304 * s.dn[840][8]));
        let eq41_e1306_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[840]) + (eq41_e1304 * s.dn[840][9]));
        let eq41_e1306_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[840]) + (eq41_e1304 * s.dn[840][10]));
        let eq41_e1306_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[840]) + (eq41_e1304 * s.dn[840][11]));
        let eq41_e1306_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[840]) + (eq41_e1304 * s.dn[840][12]));
        let eq41_e1306_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[840]) + (eq41_e1304 * s.db[840][0]));
        let eq41_e1306_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[840]) + (eq41_e1304 * s.db[840][1]));
        let eq41_e1306_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[840]) + (eq41_e1304 * s.db[840][2]));
        let eq41_e1306_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[840]) + (eq41_e1304 * s.db[840][3]));
        let eq41_e1306_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[840]) + (eq41_e1304 * s.db[840][4]));
        let eq41_e1306_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[840]) + (eq41_e1304 * s.db[840][5]));
        let eq41_e1306_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[840]) + (eq41_e1304 * s.db[840][6]));
        let eq41_e1307: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq41_e1306);
        let eq41_e1307_d_n0: f64 = (eq41_e1306_d_n0 * ddt_scale);
        let eq41_e1307_d_n1: f64 = (eq41_e1306_d_n1 * ddt_scale);
        let eq41_e1307_d_n2: f64 = (eq41_e1306_d_n2 * ddt_scale);
        let eq41_e1307_d_n3: f64 = (eq41_e1306_d_n3 * ddt_scale);
        let eq41_e1307_d_n4: f64 = (eq41_e1306_d_n4 * ddt_scale);
        let eq41_e1307_d_n5: f64 = (eq41_e1306_d_n5 * ddt_scale);
        let eq41_e1307_d_n6: f64 = (eq41_e1306_d_n6 * ddt_scale);
        let eq41_e1307_d_n7: f64 = (eq41_e1306_d_n7 * ddt_scale);
        let eq41_e1307_d_n8: f64 = (eq41_e1306_d_n8 * ddt_scale);
        let eq41_e1307_d_n9: f64 = (eq41_e1306_d_n9 * ddt_scale);
        let eq41_e1307_d_n10: f64 = (eq41_e1306_d_n10 * ddt_scale);
        let eq41_e1307_d_n11: f64 = (eq41_e1306_d_n11 * ddt_scale);
        let eq41_e1307_d_n12: f64 = (eq41_e1306_d_n12 * ddt_scale);
        let eq41_e1307_d_b0: f64 = (eq41_e1306_d_b0 * ddt_scale);
        let eq41_e1307_d_b1: f64 = (eq41_e1306_d_b1 * ddt_scale);
        let eq41_e1307_d_b2: f64 = (eq41_e1306_d_b2 * ddt_scale);
        let eq41_e1307_d_b3: f64 = (eq41_e1306_d_b3 * ddt_scale);
        let eq41_e1307_d_b4: f64 = (eq41_e1306_d_b4 * ddt_scale);
        let eq41_e1307_d_b5: f64 = (eq41_e1306_d_b5 * ddt_scale);
        let eq41_e1307_d_b6: f64 = (eq41_e1306_d_b6 * ddt_scale);
        let eq41_value: f64 = eq41_e1307;
        let eq41_node_derivatives: [f64; 13] = [eq41_e1307_d_n0, eq41_e1307_d_n1, eq41_e1307_d_n2, eq41_e1307_d_n3, eq41_e1307_d_n4, eq41_e1307_d_n5, eq41_e1307_d_n6, eq41_e1307_d_n7, eq41_e1307_d_n8, eq41_e1307_d_n9, eq41_e1307_d_n10, eq41_e1307_d_n11, eq41_e1307_d_n12];
        let eq41_branch_derivatives: [f64; 7] = [eq41_e1307_d_b0, eq41_e1307_d_b1, eq41_e1307_d_b2, eq41_e1307_d_b3, eq41_e1307_d_b4, eq41_e1307_d_b5, eq41_e1307_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e1310: f64 = (s.v[0] * s.v[15]);
        let eq42_e1310_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq42_e1310_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq42_e1310_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq42_e1310_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq42_e1310_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq42_e1310_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq42_e1310_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq42_e1310_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq42_e1310_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq42_e1310_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq42_e1310_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq42_e1310_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq42_e1310_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq42_e1310_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq42_e1310_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq42_e1310_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq42_e1310_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq42_e1310_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq42_e1310_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq42_e1310_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1312_d_n0: f64 = (eq42_e1310_d_n0 * p.p33);
        let eq42_e1312_d_n1: f64 = (eq42_e1310_d_n1 * p.p33);
        let eq42_e1312_d_n2: f64 = (eq42_e1310_d_n2 * p.p33);
        let eq42_e1312_d_n3: f64 = (eq42_e1310_d_n3 * p.p33);
        let eq42_e1312_d_n4: f64 = (eq42_e1310_d_n4 * p.p33);
        let eq42_e1312_d_n5: f64 = (eq42_e1310_d_n5 * p.p33);
        let eq42_e1312_d_n6: f64 = (eq42_e1310_d_n6 * p.p33);
        let eq42_e1312_d_n7: f64 = (eq42_e1310_d_n7 * p.p33);
        let eq42_e1312_d_n8: f64 = (eq42_e1310_d_n8 * p.p33);
        let eq42_e1312_d_n9: f64 = (eq42_e1310_d_n9 * p.p33);
        let eq42_e1312_d_n10: f64 = (eq42_e1310_d_n10 * p.p33);
        let eq42_e1312_d_n11: f64 = (eq42_e1310_d_n11 * p.p33);
        let eq42_e1312_d_n12: f64 = (eq42_e1310_d_n12 * p.p33);
        let eq42_e1312_d_b0: f64 = (eq42_e1310_d_b0 * p.p33);
        let eq42_e1312_d_b1: f64 = (eq42_e1310_d_b1 * p.p33);
        let eq42_e1312_d_b2: f64 = (eq42_e1310_d_b2 * p.p33);
        let eq42_e1312_d_b3: f64 = (eq42_e1310_d_b3 * p.p33);
        let eq42_e1312_d_b4: f64 = (eq42_e1310_d_b4 * p.p33);
        let eq42_e1312_d_b5: f64 = (eq42_e1310_d_b5 * p.p33);
        let eq42_e1312_d_b6: f64 = (eq42_e1310_d_b6 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * s.v[841]);
        let eq42_e1314_d_n0: f64 = ((eq42_e1312_d_n0 * s.v[841]) + (eq42_e1312 * s.dn[841][0]));
        let eq42_e1314_d_n1: f64 = ((eq42_e1312_d_n1 * s.v[841]) + (eq42_e1312 * s.dn[841][1]));
        let eq42_e1314_d_n2: f64 = ((eq42_e1312_d_n2 * s.v[841]) + (eq42_e1312 * s.dn[841][2]));
        let eq42_e1314_d_n3: f64 = ((eq42_e1312_d_n3 * s.v[841]) + (eq42_e1312 * s.dn[841][3]));
        let eq42_e1314_d_n4: f64 = ((eq42_e1312_d_n4 * s.v[841]) + (eq42_e1312 * s.dn[841][4]));
        let eq42_e1314_d_n5: f64 = ((eq42_e1312_d_n5 * s.v[841]) + (eq42_e1312 * s.dn[841][5]));
        let eq42_e1314_d_n6: f64 = ((eq42_e1312_d_n6 * s.v[841]) + (eq42_e1312 * s.dn[841][6]));
        let eq42_e1314_d_n7: f64 = ((eq42_e1312_d_n7 * s.v[841]) + (eq42_e1312 * s.dn[841][7]));
        let eq42_e1314_d_n8: f64 = ((eq42_e1312_d_n8 * s.v[841]) + (eq42_e1312 * s.dn[841][8]));
        let eq42_e1314_d_n9: f64 = ((eq42_e1312_d_n9 * s.v[841]) + (eq42_e1312 * s.dn[841][9]));
        let eq42_e1314_d_n10: f64 = ((eq42_e1312_d_n10 * s.v[841]) + (eq42_e1312 * s.dn[841][10]));
        let eq42_e1314_d_n11: f64 = ((eq42_e1312_d_n11 * s.v[841]) + (eq42_e1312 * s.dn[841][11]));
        let eq42_e1314_d_n12: f64 = ((eq42_e1312_d_n12 * s.v[841]) + (eq42_e1312 * s.dn[841][12]));
        let eq42_e1314_d_b0: f64 = ((eq42_e1312_d_b0 * s.v[841]) + (eq42_e1312 * s.db[841][0]));
        let eq42_e1314_d_b1: f64 = ((eq42_e1312_d_b1 * s.v[841]) + (eq42_e1312 * s.db[841][1]));
        let eq42_e1314_d_b2: f64 = ((eq42_e1312_d_b2 * s.v[841]) + (eq42_e1312 * s.db[841][2]));
        let eq42_e1314_d_b3: f64 = ((eq42_e1312_d_b3 * s.v[841]) + (eq42_e1312 * s.db[841][3]));
        let eq42_e1314_d_b4: f64 = ((eq42_e1312_d_b4 * s.v[841]) + (eq42_e1312 * s.db[841][4]));
        let eq42_e1314_d_b5: f64 = ((eq42_e1312_d_b5 * s.v[841]) + (eq42_e1312 * s.db[841][5]));
        let eq42_e1314_d_b6: f64 = ((eq42_e1312_d_b6 * s.v[841]) + (eq42_e1312 * s.db[841][6]));
        let eq42_e1315: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq42_e1314);
        let eq42_e1315_d_n0: f64 = (eq42_e1314_d_n0 * ddt_scale);
        let eq42_e1315_d_n1: f64 = (eq42_e1314_d_n1 * ddt_scale);
        let eq42_e1315_d_n2: f64 = (eq42_e1314_d_n2 * ddt_scale);
        let eq42_e1315_d_n3: f64 = (eq42_e1314_d_n3 * ddt_scale);
        let eq42_e1315_d_n4: f64 = (eq42_e1314_d_n4 * ddt_scale);
        let eq42_e1315_d_n5: f64 = (eq42_e1314_d_n5 * ddt_scale);
        let eq42_e1315_d_n6: f64 = (eq42_e1314_d_n6 * ddt_scale);
        let eq42_e1315_d_n7: f64 = (eq42_e1314_d_n7 * ddt_scale);
        let eq42_e1315_d_n8: f64 = (eq42_e1314_d_n8 * ddt_scale);
        let eq42_e1315_d_n9: f64 = (eq42_e1314_d_n9 * ddt_scale);
        let eq42_e1315_d_n10: f64 = (eq42_e1314_d_n10 * ddt_scale);
        let eq42_e1315_d_n11: f64 = (eq42_e1314_d_n11 * ddt_scale);
        let eq42_e1315_d_n12: f64 = (eq42_e1314_d_n12 * ddt_scale);
        let eq42_e1315_d_b0: f64 = (eq42_e1314_d_b0 * ddt_scale);
        let eq42_e1315_d_b1: f64 = (eq42_e1314_d_b1 * ddt_scale);
        let eq42_e1315_d_b2: f64 = (eq42_e1314_d_b2 * ddt_scale);
        let eq42_e1315_d_b3: f64 = (eq42_e1314_d_b3 * ddt_scale);
        let eq42_e1315_d_b4: f64 = (eq42_e1314_d_b4 * ddt_scale);
        let eq42_e1315_d_b5: f64 = (eq42_e1314_d_b5 * ddt_scale);
        let eq42_e1315_d_b6: f64 = (eq42_e1314_d_b6 * ddt_scale);
        let eq42_value: f64 = eq42_e1315;
        let eq42_node_derivatives: [f64; 13] = [eq42_e1315_d_n0, eq42_e1315_d_n1, eq42_e1315_d_n2, eq42_e1315_d_n3, eq42_e1315_d_n4, eq42_e1315_d_n5, eq42_e1315_d_n6, eq42_e1315_d_n7, eq42_e1315_d_n8, eq42_e1315_d_n9, eq42_e1315_d_n10, eq42_e1315_d_n11, eq42_e1315_d_n12];
        let eq42_branch_derivatives: [f64; 7] = [eq42_e1315_d_b0, eq42_e1315_d_b1, eq42_e1315_d_b2, eq42_e1315_d_b3, eq42_e1315_d_b4, eq42_e1315_d_b5, eq42_e1315_d_b6];
        stamper.stamp_current_dense_local(
            Some(9),
            Some(7),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
            multiplicity,
        );
        let eq43_e1318: f64 = (s.v[0] * s.v[15]);
        let eq43_e1318_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq43_e1318_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq43_e1318_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq43_e1318_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq43_e1318_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq43_e1318_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq43_e1318_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq43_e1318_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq43_e1318_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq43_e1318_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq43_e1318_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq43_e1318_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq43_e1318_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq43_e1318_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq43_e1318_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq43_e1318_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq43_e1318_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq43_e1318_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq43_e1318_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq43_e1318_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1320_d_n0: f64 = (eq43_e1318_d_n0 * p.p33);
        let eq43_e1320_d_n1: f64 = (eq43_e1318_d_n1 * p.p33);
        let eq43_e1320_d_n2: f64 = (eq43_e1318_d_n2 * p.p33);
        let eq43_e1320_d_n3: f64 = (eq43_e1318_d_n3 * p.p33);
        let eq43_e1320_d_n4: f64 = (eq43_e1318_d_n4 * p.p33);
        let eq43_e1320_d_n5: f64 = (eq43_e1318_d_n5 * p.p33);
        let eq43_e1320_d_n6: f64 = (eq43_e1318_d_n6 * p.p33);
        let eq43_e1320_d_n7: f64 = (eq43_e1318_d_n7 * p.p33);
        let eq43_e1320_d_n8: f64 = (eq43_e1318_d_n8 * p.p33);
        let eq43_e1320_d_n9: f64 = (eq43_e1318_d_n9 * p.p33);
        let eq43_e1320_d_n10: f64 = (eq43_e1318_d_n10 * p.p33);
        let eq43_e1320_d_n11: f64 = (eq43_e1318_d_n11 * p.p33);
        let eq43_e1320_d_n12: f64 = (eq43_e1318_d_n12 * p.p33);
        let eq43_e1320_d_b0: f64 = (eq43_e1318_d_b0 * p.p33);
        let eq43_e1320_d_b1: f64 = (eq43_e1318_d_b1 * p.p33);
        let eq43_e1320_d_b2: f64 = (eq43_e1318_d_b2 * p.p33);
        let eq43_e1320_d_b3: f64 = (eq43_e1318_d_b3 * p.p33);
        let eq43_e1320_d_b4: f64 = (eq43_e1318_d_b4 * p.p33);
        let eq43_e1320_d_b5: f64 = (eq43_e1318_d_b5 * p.p33);
        let eq43_e1320_d_b6: f64 = (eq43_e1318_d_b6 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * s.v[842]);
        let eq43_e1322_d_n0: f64 = ((eq43_e1320_d_n0 * s.v[842]) + (eq43_e1320 * s.dn[842][0]));
        let eq43_e1322_d_n1: f64 = ((eq43_e1320_d_n1 * s.v[842]) + (eq43_e1320 * s.dn[842][1]));
        let eq43_e1322_d_n2: f64 = ((eq43_e1320_d_n2 * s.v[842]) + (eq43_e1320 * s.dn[842][2]));
        let eq43_e1322_d_n3: f64 = ((eq43_e1320_d_n3 * s.v[842]) + (eq43_e1320 * s.dn[842][3]));
        let eq43_e1322_d_n4: f64 = ((eq43_e1320_d_n4 * s.v[842]) + (eq43_e1320 * s.dn[842][4]));
        let eq43_e1322_d_n5: f64 = ((eq43_e1320_d_n5 * s.v[842]) + (eq43_e1320 * s.dn[842][5]));
        let eq43_e1322_d_n6: f64 = ((eq43_e1320_d_n6 * s.v[842]) + (eq43_e1320 * s.dn[842][6]));
        let eq43_e1322_d_n7: f64 = ((eq43_e1320_d_n7 * s.v[842]) + (eq43_e1320 * s.dn[842][7]));
        let eq43_e1322_d_n8: f64 = ((eq43_e1320_d_n8 * s.v[842]) + (eq43_e1320 * s.dn[842][8]));
        let eq43_e1322_d_n9: f64 = ((eq43_e1320_d_n9 * s.v[842]) + (eq43_e1320 * s.dn[842][9]));
        let eq43_e1322_d_n10: f64 = ((eq43_e1320_d_n10 * s.v[842]) + (eq43_e1320 * s.dn[842][10]));
        let eq43_e1322_d_n11: f64 = ((eq43_e1320_d_n11 * s.v[842]) + (eq43_e1320 * s.dn[842][11]));
        let eq43_e1322_d_n12: f64 = ((eq43_e1320_d_n12 * s.v[842]) + (eq43_e1320 * s.dn[842][12]));
        let eq43_e1322_d_b0: f64 = ((eq43_e1320_d_b0 * s.v[842]) + (eq43_e1320 * s.db[842][0]));
        let eq43_e1322_d_b1: f64 = ((eq43_e1320_d_b1 * s.v[842]) + (eq43_e1320 * s.db[842][1]));
        let eq43_e1322_d_b2: f64 = ((eq43_e1320_d_b2 * s.v[842]) + (eq43_e1320 * s.db[842][2]));
        let eq43_e1322_d_b3: f64 = ((eq43_e1320_d_b3 * s.v[842]) + (eq43_e1320 * s.db[842][3]));
        let eq43_e1322_d_b4: f64 = ((eq43_e1320_d_b4 * s.v[842]) + (eq43_e1320 * s.db[842][4]));
        let eq43_e1322_d_b5: f64 = ((eq43_e1320_d_b5 * s.v[842]) + (eq43_e1320 * s.db[842][5]));
        let eq43_e1322_d_b6: f64 = ((eq43_e1320_d_b6 * s.v[842]) + (eq43_e1320 * s.db[842][6]));
        let eq43_e1323: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq43_e1322);
        let eq43_e1323_d_n0: f64 = (eq43_e1322_d_n0 * ddt_scale);
        let eq43_e1323_d_n1: f64 = (eq43_e1322_d_n1 * ddt_scale);
        let eq43_e1323_d_n2: f64 = (eq43_e1322_d_n2 * ddt_scale);
        let eq43_e1323_d_n3: f64 = (eq43_e1322_d_n3 * ddt_scale);
        let eq43_e1323_d_n4: f64 = (eq43_e1322_d_n4 * ddt_scale);
        let eq43_e1323_d_n5: f64 = (eq43_e1322_d_n5 * ddt_scale);
        let eq43_e1323_d_n6: f64 = (eq43_e1322_d_n6 * ddt_scale);
        let eq43_e1323_d_n7: f64 = (eq43_e1322_d_n7 * ddt_scale);
        let eq43_e1323_d_n8: f64 = (eq43_e1322_d_n8 * ddt_scale);
        let eq43_e1323_d_n9: f64 = (eq43_e1322_d_n9 * ddt_scale);
        let eq43_e1323_d_n10: f64 = (eq43_e1322_d_n10 * ddt_scale);
        let eq43_e1323_d_n11: f64 = (eq43_e1322_d_n11 * ddt_scale);
        let eq43_e1323_d_n12: f64 = (eq43_e1322_d_n12 * ddt_scale);
        let eq43_e1323_d_b0: f64 = (eq43_e1322_d_b0 * ddt_scale);
        let eq43_e1323_d_b1: f64 = (eq43_e1322_d_b1 * ddt_scale);
        let eq43_e1323_d_b2: f64 = (eq43_e1322_d_b2 * ddt_scale);
        let eq43_e1323_d_b3: f64 = (eq43_e1322_d_b3 * ddt_scale);
        let eq43_e1323_d_b4: f64 = (eq43_e1322_d_b4 * ddt_scale);
        let eq43_e1323_d_b5: f64 = (eq43_e1322_d_b5 * ddt_scale);
        let eq43_e1323_d_b6: f64 = (eq43_e1322_d_b6 * ddt_scale);
        let eq43_value: f64 = eq43_e1323;
        let eq43_node_derivatives: [f64; 13] = [eq43_e1323_d_n0, eq43_e1323_d_n1, eq43_e1323_d_n2, eq43_e1323_d_n3, eq43_e1323_d_n4, eq43_e1323_d_n5, eq43_e1323_d_n6, eq43_e1323_d_n7, eq43_e1323_d_n8, eq43_e1323_d_n9, eq43_e1323_d_n10, eq43_e1323_d_n11, eq43_e1323_d_n12];
        let eq43_branch_derivatives: [f64; 7] = [eq43_e1323_d_b0, eq43_e1323_d_b1, eq43_e1323_d_b2, eq43_e1323_d_b3, eq43_e1323_d_b4, eq43_e1323_d_b5, eq43_e1323_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(7),
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_5(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let eq44_e1326: f64 = (s.v[0] * s.v[15]);
        let eq44_e1326_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq44_e1326_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq44_e1326_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq44_e1326_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq44_e1326_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq44_e1326_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq44_e1326_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq44_e1326_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq44_e1326_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq44_e1326_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq44_e1326_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq44_e1326_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq44_e1326_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq44_e1326_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq44_e1326_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq44_e1326_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq44_e1326_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq44_e1326_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq44_e1326_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq44_e1326_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq44_e1328: f64 = (eq44_e1326 * p.p33);
        let eq44_e1328_d_n0: f64 = (eq44_e1326_d_n0 * p.p33);
        let eq44_e1328_d_n1: f64 = (eq44_e1326_d_n1 * p.p33);
        let eq44_e1328_d_n2: f64 = (eq44_e1326_d_n2 * p.p33);
        let eq44_e1328_d_n3: f64 = (eq44_e1326_d_n3 * p.p33);
        let eq44_e1328_d_n4: f64 = (eq44_e1326_d_n4 * p.p33);
        let eq44_e1328_d_n5: f64 = (eq44_e1326_d_n5 * p.p33);
        let eq44_e1328_d_n6: f64 = (eq44_e1326_d_n6 * p.p33);
        let eq44_e1328_d_n7: f64 = (eq44_e1326_d_n7 * p.p33);
        let eq44_e1328_d_n8: f64 = (eq44_e1326_d_n8 * p.p33);
        let eq44_e1328_d_n9: f64 = (eq44_e1326_d_n9 * p.p33);
        let eq44_e1328_d_n10: f64 = (eq44_e1326_d_n10 * p.p33);
        let eq44_e1328_d_n11: f64 = (eq44_e1326_d_n11 * p.p33);
        let eq44_e1328_d_n12: f64 = (eq44_e1326_d_n12 * p.p33);
        let eq44_e1328_d_b0: f64 = (eq44_e1326_d_b0 * p.p33);
        let eq44_e1328_d_b1: f64 = (eq44_e1326_d_b1 * p.p33);
        let eq44_e1328_d_b2: f64 = (eq44_e1326_d_b2 * p.p33);
        let eq44_e1328_d_b3: f64 = (eq44_e1326_d_b3 * p.p33);
        let eq44_e1328_d_b4: f64 = (eq44_e1326_d_b4 * p.p33);
        let eq44_e1328_d_b5: f64 = (eq44_e1326_d_b5 * p.p33);
        let eq44_e1328_d_b6: f64 = (eq44_e1326_d_b6 * p.p33);
        let eq44_e1330: f64 = (eq44_e1328 * s.v[843]);
        let eq44_e1330_d_n0: f64 = ((eq44_e1328_d_n0 * s.v[843]) + (eq44_e1328 * s.dn[843][0]));
        let eq44_e1330_d_n1: f64 = ((eq44_e1328_d_n1 * s.v[843]) + (eq44_e1328 * s.dn[843][1]));
        let eq44_e1330_d_n2: f64 = ((eq44_e1328_d_n2 * s.v[843]) + (eq44_e1328 * s.dn[843][2]));
        let eq44_e1330_d_n3: f64 = ((eq44_e1328_d_n3 * s.v[843]) + (eq44_e1328 * s.dn[843][3]));
        let eq44_e1330_d_n4: f64 = ((eq44_e1328_d_n4 * s.v[843]) + (eq44_e1328 * s.dn[843][4]));
        let eq44_e1330_d_n5: f64 = ((eq44_e1328_d_n5 * s.v[843]) + (eq44_e1328 * s.dn[843][5]));
        let eq44_e1330_d_n6: f64 = ((eq44_e1328_d_n6 * s.v[843]) + (eq44_e1328 * s.dn[843][6]));
        let eq44_e1330_d_n7: f64 = ((eq44_e1328_d_n7 * s.v[843]) + (eq44_e1328 * s.dn[843][7]));
        let eq44_e1330_d_n8: f64 = ((eq44_e1328_d_n8 * s.v[843]) + (eq44_e1328 * s.dn[843][8]));
        let eq44_e1330_d_n9: f64 = ((eq44_e1328_d_n9 * s.v[843]) + (eq44_e1328 * s.dn[843][9]));
        let eq44_e1330_d_n10: f64 = ((eq44_e1328_d_n10 * s.v[843]) + (eq44_e1328 * s.dn[843][10]));
        let eq44_e1330_d_n11: f64 = ((eq44_e1328_d_n11 * s.v[843]) + (eq44_e1328 * s.dn[843][11]));
        let eq44_e1330_d_n12: f64 = ((eq44_e1328_d_n12 * s.v[843]) + (eq44_e1328 * s.dn[843][12]));
        let eq44_e1330_d_b0: f64 = ((eq44_e1328_d_b0 * s.v[843]) + (eq44_e1328 * s.db[843][0]));
        let eq44_e1330_d_b1: f64 = ((eq44_e1328_d_b1 * s.v[843]) + (eq44_e1328 * s.db[843][1]));
        let eq44_e1330_d_b2: f64 = ((eq44_e1328_d_b2 * s.v[843]) + (eq44_e1328 * s.db[843][2]));
        let eq44_e1330_d_b3: f64 = ((eq44_e1328_d_b3 * s.v[843]) + (eq44_e1328 * s.db[843][3]));
        let eq44_e1330_d_b4: f64 = ((eq44_e1328_d_b4 * s.v[843]) + (eq44_e1328 * s.db[843][4]));
        let eq44_e1330_d_b5: f64 = ((eq44_e1328_d_b5 * s.v[843]) + (eq44_e1328 * s.db[843][5]));
        let eq44_e1330_d_b6: f64 = ((eq44_e1328_d_b6 * s.v[843]) + (eq44_e1328 * s.db[843][6]));
        let eq44_e1331: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq44_e1330);
        let eq44_e1331_d_n0: f64 = (eq44_e1330_d_n0 * ddt_scale);
        let eq44_e1331_d_n1: f64 = (eq44_e1330_d_n1 * ddt_scale);
        let eq44_e1331_d_n2: f64 = (eq44_e1330_d_n2 * ddt_scale);
        let eq44_e1331_d_n3: f64 = (eq44_e1330_d_n3 * ddt_scale);
        let eq44_e1331_d_n4: f64 = (eq44_e1330_d_n4 * ddt_scale);
        let eq44_e1331_d_n5: f64 = (eq44_e1330_d_n5 * ddt_scale);
        let eq44_e1331_d_n6: f64 = (eq44_e1330_d_n6 * ddt_scale);
        let eq44_e1331_d_n7: f64 = (eq44_e1330_d_n7 * ddt_scale);
        let eq44_e1331_d_n8: f64 = (eq44_e1330_d_n8 * ddt_scale);
        let eq44_e1331_d_n9: f64 = (eq44_e1330_d_n9 * ddt_scale);
        let eq44_e1331_d_n10: f64 = (eq44_e1330_d_n10 * ddt_scale);
        let eq44_e1331_d_n11: f64 = (eq44_e1330_d_n11 * ddt_scale);
        let eq44_e1331_d_n12: f64 = (eq44_e1330_d_n12 * ddt_scale);
        let eq44_e1331_d_b0: f64 = (eq44_e1330_d_b0 * ddt_scale);
        let eq44_e1331_d_b1: f64 = (eq44_e1330_d_b1 * ddt_scale);
        let eq44_e1331_d_b2: f64 = (eq44_e1330_d_b2 * ddt_scale);
        let eq44_e1331_d_b3: f64 = (eq44_e1330_d_b3 * ddt_scale);
        let eq44_e1331_d_b4: f64 = (eq44_e1330_d_b4 * ddt_scale);
        let eq44_e1331_d_b5: f64 = (eq44_e1330_d_b5 * ddt_scale);
        let eq44_e1331_d_b6: f64 = (eq44_e1330_d_b6 * ddt_scale);
        let eq44_value: f64 = eq44_e1331;
        let eq44_node_derivatives: [f64; 13] = [eq44_e1331_d_n0, eq44_e1331_d_n1, eq44_e1331_d_n2, eq44_e1331_d_n3, eq44_e1331_d_n4, eq44_e1331_d_n5, eq44_e1331_d_n6, eq44_e1331_d_n7, eq44_e1331_d_n8, eq44_e1331_d_n9, eq44_e1331_d_n10, eq44_e1331_d_n11, eq44_e1331_d_n12];
        let eq44_branch_derivatives: [f64; 7] = [eq44_e1331_d_b0, eq44_e1331_d_b1, eq44_e1331_d_b2, eq44_e1331_d_b3, eq44_e1331_d_b4, eq44_e1331_d_b5, eq44_e1331_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let eq45_e1334: f64 = (s.v[0] * s.v[15]);
        let eq45_e1334_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq45_e1334_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq45_e1334_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq45_e1334_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq45_e1334_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq45_e1334_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq45_e1334_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq45_e1334_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq45_e1334_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq45_e1334_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq45_e1334_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq45_e1334_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq45_e1334_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq45_e1334_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq45_e1334_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq45_e1334_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq45_e1334_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq45_e1334_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq45_e1334_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq45_e1334_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq45_e1336: f64 = (eq45_e1334 * p.p33);
        let eq45_e1336_d_n0: f64 = (eq45_e1334_d_n0 * p.p33);
        let eq45_e1336_d_n1: f64 = (eq45_e1334_d_n1 * p.p33);
        let eq45_e1336_d_n2: f64 = (eq45_e1334_d_n2 * p.p33);
        let eq45_e1336_d_n3: f64 = (eq45_e1334_d_n3 * p.p33);
        let eq45_e1336_d_n4: f64 = (eq45_e1334_d_n4 * p.p33);
        let eq45_e1336_d_n5: f64 = (eq45_e1334_d_n5 * p.p33);
        let eq45_e1336_d_n6: f64 = (eq45_e1334_d_n6 * p.p33);
        let eq45_e1336_d_n7: f64 = (eq45_e1334_d_n7 * p.p33);
        let eq45_e1336_d_n8: f64 = (eq45_e1334_d_n8 * p.p33);
        let eq45_e1336_d_n9: f64 = (eq45_e1334_d_n9 * p.p33);
        let eq45_e1336_d_n10: f64 = (eq45_e1334_d_n10 * p.p33);
        let eq45_e1336_d_n11: f64 = (eq45_e1334_d_n11 * p.p33);
        let eq45_e1336_d_n12: f64 = (eq45_e1334_d_n12 * p.p33);
        let eq45_e1336_d_b0: f64 = (eq45_e1334_d_b0 * p.p33);
        let eq45_e1336_d_b1: f64 = (eq45_e1334_d_b1 * p.p33);
        let eq45_e1336_d_b2: f64 = (eq45_e1334_d_b2 * p.p33);
        let eq45_e1336_d_b3: f64 = (eq45_e1334_d_b3 * p.p33);
        let eq45_e1336_d_b4: f64 = (eq45_e1334_d_b4 * p.p33);
        let eq45_e1336_d_b5: f64 = (eq45_e1334_d_b5 * p.p33);
        let eq45_e1336_d_b6: f64 = (eq45_e1334_d_b6 * p.p33);
        let eq45_e1338: f64 = (eq45_e1336 * s.v[844]);
        let eq45_e1338_d_n0: f64 = ((eq45_e1336_d_n0 * s.v[844]) + (eq45_e1336 * s.dn[844][0]));
        let eq45_e1338_d_n1: f64 = ((eq45_e1336_d_n1 * s.v[844]) + (eq45_e1336 * s.dn[844][1]));
        let eq45_e1338_d_n2: f64 = ((eq45_e1336_d_n2 * s.v[844]) + (eq45_e1336 * s.dn[844][2]));
        let eq45_e1338_d_n3: f64 = ((eq45_e1336_d_n3 * s.v[844]) + (eq45_e1336 * s.dn[844][3]));
        let eq45_e1338_d_n4: f64 = ((eq45_e1336_d_n4 * s.v[844]) + (eq45_e1336 * s.dn[844][4]));
        let eq45_e1338_d_n5: f64 = ((eq45_e1336_d_n5 * s.v[844]) + (eq45_e1336 * s.dn[844][5]));
        let eq45_e1338_d_n6: f64 = ((eq45_e1336_d_n6 * s.v[844]) + (eq45_e1336 * s.dn[844][6]));
        let eq45_e1338_d_n7: f64 = ((eq45_e1336_d_n7 * s.v[844]) + (eq45_e1336 * s.dn[844][7]));
        let eq45_e1338_d_n8: f64 = ((eq45_e1336_d_n8 * s.v[844]) + (eq45_e1336 * s.dn[844][8]));
        let eq45_e1338_d_n9: f64 = ((eq45_e1336_d_n9 * s.v[844]) + (eq45_e1336 * s.dn[844][9]));
        let eq45_e1338_d_n10: f64 = ((eq45_e1336_d_n10 * s.v[844]) + (eq45_e1336 * s.dn[844][10]));
        let eq45_e1338_d_n11: f64 = ((eq45_e1336_d_n11 * s.v[844]) + (eq45_e1336 * s.dn[844][11]));
        let eq45_e1338_d_n12: f64 = ((eq45_e1336_d_n12 * s.v[844]) + (eq45_e1336 * s.dn[844][12]));
        let eq45_e1338_d_b0: f64 = ((eq45_e1336_d_b0 * s.v[844]) + (eq45_e1336 * s.db[844][0]));
        let eq45_e1338_d_b1: f64 = ((eq45_e1336_d_b1 * s.v[844]) + (eq45_e1336 * s.db[844][1]));
        let eq45_e1338_d_b2: f64 = ((eq45_e1336_d_b2 * s.v[844]) + (eq45_e1336 * s.db[844][2]));
        let eq45_e1338_d_b3: f64 = ((eq45_e1336_d_b3 * s.v[844]) + (eq45_e1336 * s.db[844][3]));
        let eq45_e1338_d_b4: f64 = ((eq45_e1336_d_b4 * s.v[844]) + (eq45_e1336 * s.db[844][4]));
        let eq45_e1338_d_b5: f64 = ((eq45_e1336_d_b5 * s.v[844]) + (eq45_e1336 * s.db[844][5]));
        let eq45_e1338_d_b6: f64 = ((eq45_e1336_d_b6 * s.v[844]) + (eq45_e1336 * s.db[844][6]));
        let eq45_e1339: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq45_e1338);
        let eq45_e1339_d_n0: f64 = (eq45_e1338_d_n0 * ddt_scale);
        let eq45_e1339_d_n1: f64 = (eq45_e1338_d_n1 * ddt_scale);
        let eq45_e1339_d_n2: f64 = (eq45_e1338_d_n2 * ddt_scale);
        let eq45_e1339_d_n3: f64 = (eq45_e1338_d_n3 * ddt_scale);
        let eq45_e1339_d_n4: f64 = (eq45_e1338_d_n4 * ddt_scale);
        let eq45_e1339_d_n5: f64 = (eq45_e1338_d_n5 * ddt_scale);
        let eq45_e1339_d_n6: f64 = (eq45_e1338_d_n6 * ddt_scale);
        let eq45_e1339_d_n7: f64 = (eq45_e1338_d_n7 * ddt_scale);
        let eq45_e1339_d_n8: f64 = (eq45_e1338_d_n8 * ddt_scale);
        let eq45_e1339_d_n9: f64 = (eq45_e1338_d_n9 * ddt_scale);
        let eq45_e1339_d_n10: f64 = (eq45_e1338_d_n10 * ddt_scale);
        let eq45_e1339_d_n11: f64 = (eq45_e1338_d_n11 * ddt_scale);
        let eq45_e1339_d_n12: f64 = (eq45_e1338_d_n12 * ddt_scale);
        let eq45_e1339_d_b0: f64 = (eq45_e1338_d_b0 * ddt_scale);
        let eq45_e1339_d_b1: f64 = (eq45_e1338_d_b1 * ddt_scale);
        let eq45_e1339_d_b2: f64 = (eq45_e1338_d_b2 * ddt_scale);
        let eq45_e1339_d_b3: f64 = (eq45_e1338_d_b3 * ddt_scale);
        let eq45_e1339_d_b4: f64 = (eq45_e1338_d_b4 * ddt_scale);
        let eq45_e1339_d_b5: f64 = (eq45_e1338_d_b5 * ddt_scale);
        let eq45_e1339_d_b6: f64 = (eq45_e1338_d_b6 * ddt_scale);
        let eq45_value: f64 = eq45_e1339;
        let eq45_node_derivatives: [f64; 13] = [eq45_e1339_d_n0, eq45_e1339_d_n1, eq45_e1339_d_n2, eq45_e1339_d_n3, eq45_e1339_d_n4, eq45_e1339_d_n5, eq45_e1339_d_n6, eq45_e1339_d_n7, eq45_e1339_d_n8, eq45_e1339_d_n9, eq45_e1339_d_n10, eq45_e1339_d_n11, eq45_e1339_d_n12];
        let eq45_branch_derivatives: [f64; 7] = [eq45_e1339_d_b0, eq45_e1339_d_b1, eq45_e1339_d_b2, eq45_e1339_d_b3, eq45_e1339_d_b4, eq45_e1339_d_b5, eq45_e1339_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
            multiplicity,
        );
        let eq46_e1342: f64 = (s.v[0] * s.v[15]);
        let eq46_e1342_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq46_e1342_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq46_e1342_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq46_e1342_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq46_e1342_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq46_e1342_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq46_e1342_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq46_e1342_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq46_e1342_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq46_e1342_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq46_e1342_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq46_e1342_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq46_e1342_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq46_e1342_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq46_e1342_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq46_e1342_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq46_e1342_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq46_e1342_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq46_e1342_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq46_e1342_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1344_d_n0: f64 = (eq46_e1342_d_n0 * p.p33);
        let eq46_e1344_d_n1: f64 = (eq46_e1342_d_n1 * p.p33);
        let eq46_e1344_d_n2: f64 = (eq46_e1342_d_n2 * p.p33);
        let eq46_e1344_d_n3: f64 = (eq46_e1342_d_n3 * p.p33);
        let eq46_e1344_d_n4: f64 = (eq46_e1342_d_n4 * p.p33);
        let eq46_e1344_d_n5: f64 = (eq46_e1342_d_n5 * p.p33);
        let eq46_e1344_d_n6: f64 = (eq46_e1342_d_n6 * p.p33);
        let eq46_e1344_d_n7: f64 = (eq46_e1342_d_n7 * p.p33);
        let eq46_e1344_d_n8: f64 = (eq46_e1342_d_n8 * p.p33);
        let eq46_e1344_d_n9: f64 = (eq46_e1342_d_n9 * p.p33);
        let eq46_e1344_d_n10: f64 = (eq46_e1342_d_n10 * p.p33);
        let eq46_e1344_d_n11: f64 = (eq46_e1342_d_n11 * p.p33);
        let eq46_e1344_d_n12: f64 = (eq46_e1342_d_n12 * p.p33);
        let eq46_e1344_d_b0: f64 = (eq46_e1342_d_b0 * p.p33);
        let eq46_e1344_d_b1: f64 = (eq46_e1342_d_b1 * p.p33);
        let eq46_e1344_d_b2: f64 = (eq46_e1342_d_b2 * p.p33);
        let eq46_e1344_d_b3: f64 = (eq46_e1342_d_b3 * p.p33);
        let eq46_e1344_d_b4: f64 = (eq46_e1342_d_b4 * p.p33);
        let eq46_e1344_d_b5: f64 = (eq46_e1342_d_b5 * p.p33);
        let eq46_e1344_d_b6: f64 = (eq46_e1342_d_b6 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * s.v[845]);
        let eq46_e1346_d_n0: f64 = ((eq46_e1344_d_n0 * s.v[845]) + (eq46_e1344 * s.dn[845][0]));
        let eq46_e1346_d_n1: f64 = ((eq46_e1344_d_n1 * s.v[845]) + (eq46_e1344 * s.dn[845][1]));
        let eq46_e1346_d_n2: f64 = ((eq46_e1344_d_n2 * s.v[845]) + (eq46_e1344 * s.dn[845][2]));
        let eq46_e1346_d_n3: f64 = ((eq46_e1344_d_n3 * s.v[845]) + (eq46_e1344 * s.dn[845][3]));
        let eq46_e1346_d_n4: f64 = ((eq46_e1344_d_n4 * s.v[845]) + (eq46_e1344 * s.dn[845][4]));
        let eq46_e1346_d_n5: f64 = ((eq46_e1344_d_n5 * s.v[845]) + (eq46_e1344 * s.dn[845][5]));
        let eq46_e1346_d_n6: f64 = ((eq46_e1344_d_n6 * s.v[845]) + (eq46_e1344 * s.dn[845][6]));
        let eq46_e1346_d_n7: f64 = ((eq46_e1344_d_n7 * s.v[845]) + (eq46_e1344 * s.dn[845][7]));
        let eq46_e1346_d_n8: f64 = ((eq46_e1344_d_n8 * s.v[845]) + (eq46_e1344 * s.dn[845][8]));
        let eq46_e1346_d_n9: f64 = ((eq46_e1344_d_n9 * s.v[845]) + (eq46_e1344 * s.dn[845][9]));
        let eq46_e1346_d_n10: f64 = ((eq46_e1344_d_n10 * s.v[845]) + (eq46_e1344 * s.dn[845][10]));
        let eq46_e1346_d_n11: f64 = ((eq46_e1344_d_n11 * s.v[845]) + (eq46_e1344 * s.dn[845][11]));
        let eq46_e1346_d_n12: f64 = ((eq46_e1344_d_n12 * s.v[845]) + (eq46_e1344 * s.dn[845][12]));
        let eq46_e1346_d_b0: f64 = ((eq46_e1344_d_b0 * s.v[845]) + (eq46_e1344 * s.db[845][0]));
        let eq46_e1346_d_b1: f64 = ((eq46_e1344_d_b1 * s.v[845]) + (eq46_e1344 * s.db[845][1]));
        let eq46_e1346_d_b2: f64 = ((eq46_e1344_d_b2 * s.v[845]) + (eq46_e1344 * s.db[845][2]));
        let eq46_e1346_d_b3: f64 = ((eq46_e1344_d_b3 * s.v[845]) + (eq46_e1344 * s.db[845][3]));
        let eq46_e1346_d_b4: f64 = ((eq46_e1344_d_b4 * s.v[845]) + (eq46_e1344 * s.db[845][4]));
        let eq46_e1346_d_b5: f64 = ((eq46_e1344_d_b5 * s.v[845]) + (eq46_e1344 * s.db[845][5]));
        let eq46_e1346_d_b6: f64 = ((eq46_e1344_d_b6 * s.v[845]) + (eq46_e1344 * s.db[845][6]));
        let eq46_e1347: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq46_e1346);
        let eq46_e1347_d_n0: f64 = (eq46_e1346_d_n0 * ddt_scale);
        let eq46_e1347_d_n1: f64 = (eq46_e1346_d_n1 * ddt_scale);
        let eq46_e1347_d_n2: f64 = (eq46_e1346_d_n2 * ddt_scale);
        let eq46_e1347_d_n3: f64 = (eq46_e1346_d_n3 * ddt_scale);
        let eq46_e1347_d_n4: f64 = (eq46_e1346_d_n4 * ddt_scale);
        let eq46_e1347_d_n5: f64 = (eq46_e1346_d_n5 * ddt_scale);
        let eq46_e1347_d_n6: f64 = (eq46_e1346_d_n6 * ddt_scale);
        let eq46_e1347_d_n7: f64 = (eq46_e1346_d_n7 * ddt_scale);
        let eq46_e1347_d_n8: f64 = (eq46_e1346_d_n8 * ddt_scale);
        let eq46_e1347_d_n9: f64 = (eq46_e1346_d_n9 * ddt_scale);
        let eq46_e1347_d_n10: f64 = (eq46_e1346_d_n10 * ddt_scale);
        let eq46_e1347_d_n11: f64 = (eq46_e1346_d_n11 * ddt_scale);
        let eq46_e1347_d_n12: f64 = (eq46_e1346_d_n12 * ddt_scale);
        let eq46_e1347_d_b0: f64 = (eq46_e1346_d_b0 * ddt_scale);
        let eq46_e1347_d_b1: f64 = (eq46_e1346_d_b1 * ddt_scale);
        let eq46_e1347_d_b2: f64 = (eq46_e1346_d_b2 * ddt_scale);
        let eq46_e1347_d_b3: f64 = (eq46_e1346_d_b3 * ddt_scale);
        let eq46_e1347_d_b4: f64 = (eq46_e1346_d_b4 * ddt_scale);
        let eq46_e1347_d_b5: f64 = (eq46_e1346_d_b5 * ddt_scale);
        let eq46_e1347_d_b6: f64 = (eq46_e1346_d_b6 * ddt_scale);
        let eq46_value: f64 = eq46_e1347;
        let eq46_node_derivatives: [f64; 13] = [eq46_e1347_d_n0, eq46_e1347_d_n1, eq46_e1347_d_n2, eq46_e1347_d_n3, eq46_e1347_d_n4, eq46_e1347_d_n5, eq46_e1347_d_n6, eq46_e1347_d_n7, eq46_e1347_d_n8, eq46_e1347_d_n9, eq46_e1347_d_n10, eq46_e1347_d_n11, eq46_e1347_d_n12];
        let eq46_branch_derivatives: [f64; 7] = [eq46_e1347_d_b0, eq46_e1347_d_b1, eq46_e1347_d_b2, eq46_e1347_d_b3, eq46_e1347_d_b4, eq46_e1347_d_b5, eq46_e1347_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(9),
            multiplicity * (eq46_value),
            &eq46_node_derivatives,
            &eq46_branch_derivatives,
            multiplicity,
        );
        let eq47_e1350: f64 = (s.v[0] * s.v[15]);
        let eq47_e1350_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq47_e1350_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq47_e1350_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq47_e1350_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq47_e1350_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq47_e1350_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq47_e1350_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq47_e1350_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq47_e1350_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq47_e1350_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq47_e1350_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq47_e1350_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq47_e1350_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq47_e1350_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq47_e1350_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq47_e1350_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq47_e1350_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq47_e1350_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq47_e1350_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq47_e1350_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq47_e1352: f64 = (eq47_e1350 * p.p33);
        let eq47_e1352_d_n0: f64 = (eq47_e1350_d_n0 * p.p33);
        let eq47_e1352_d_n1: f64 = (eq47_e1350_d_n1 * p.p33);
        let eq47_e1352_d_n2: f64 = (eq47_e1350_d_n2 * p.p33);
        let eq47_e1352_d_n3: f64 = (eq47_e1350_d_n3 * p.p33);
        let eq47_e1352_d_n4: f64 = (eq47_e1350_d_n4 * p.p33);
        let eq47_e1352_d_n5: f64 = (eq47_e1350_d_n5 * p.p33);
        let eq47_e1352_d_n6: f64 = (eq47_e1350_d_n6 * p.p33);
        let eq47_e1352_d_n7: f64 = (eq47_e1350_d_n7 * p.p33);
        let eq47_e1352_d_n8: f64 = (eq47_e1350_d_n8 * p.p33);
        let eq47_e1352_d_n9: f64 = (eq47_e1350_d_n9 * p.p33);
        let eq47_e1352_d_n10: f64 = (eq47_e1350_d_n10 * p.p33);
        let eq47_e1352_d_n11: f64 = (eq47_e1350_d_n11 * p.p33);
        let eq47_e1352_d_n12: f64 = (eq47_e1350_d_n12 * p.p33);
        let eq47_e1352_d_b0: f64 = (eq47_e1350_d_b0 * p.p33);
        let eq47_e1352_d_b1: f64 = (eq47_e1350_d_b1 * p.p33);
        let eq47_e1352_d_b2: f64 = (eq47_e1350_d_b2 * p.p33);
        let eq47_e1352_d_b3: f64 = (eq47_e1350_d_b3 * p.p33);
        let eq47_e1352_d_b4: f64 = (eq47_e1350_d_b4 * p.p33);
        let eq47_e1352_d_b5: f64 = (eq47_e1350_d_b5 * p.p33);
        let eq47_e1352_d_b6: f64 = (eq47_e1350_d_b6 * p.p33);
        let eq47_e1354: f64 = (eq47_e1352 * s.v[846]);
        let eq47_e1354_d_n0: f64 = ((eq47_e1352_d_n0 * s.v[846]) + (eq47_e1352 * s.dn[846][0]));
        let eq47_e1354_d_n1: f64 = ((eq47_e1352_d_n1 * s.v[846]) + (eq47_e1352 * s.dn[846][1]));
        let eq47_e1354_d_n2: f64 = ((eq47_e1352_d_n2 * s.v[846]) + (eq47_e1352 * s.dn[846][2]));
        let eq47_e1354_d_n3: f64 = ((eq47_e1352_d_n3 * s.v[846]) + (eq47_e1352 * s.dn[846][3]));
        let eq47_e1354_d_n4: f64 = ((eq47_e1352_d_n4 * s.v[846]) + (eq47_e1352 * s.dn[846][4]));
        let eq47_e1354_d_n5: f64 = ((eq47_e1352_d_n5 * s.v[846]) + (eq47_e1352 * s.dn[846][5]));
        let eq47_e1354_d_n6: f64 = ((eq47_e1352_d_n6 * s.v[846]) + (eq47_e1352 * s.dn[846][6]));
        let eq47_e1354_d_n7: f64 = ((eq47_e1352_d_n7 * s.v[846]) + (eq47_e1352 * s.dn[846][7]));
        let eq47_e1354_d_n8: f64 = ((eq47_e1352_d_n8 * s.v[846]) + (eq47_e1352 * s.dn[846][8]));
        let eq47_e1354_d_n9: f64 = ((eq47_e1352_d_n9 * s.v[846]) + (eq47_e1352 * s.dn[846][9]));
        let eq47_e1354_d_n10: f64 = ((eq47_e1352_d_n10 * s.v[846]) + (eq47_e1352 * s.dn[846][10]));
        let eq47_e1354_d_n11: f64 = ((eq47_e1352_d_n11 * s.v[846]) + (eq47_e1352 * s.dn[846][11]));
        let eq47_e1354_d_n12: f64 = ((eq47_e1352_d_n12 * s.v[846]) + (eq47_e1352 * s.dn[846][12]));
        let eq47_e1354_d_b0: f64 = ((eq47_e1352_d_b0 * s.v[846]) + (eq47_e1352 * s.db[846][0]));
        let eq47_e1354_d_b1: f64 = ((eq47_e1352_d_b1 * s.v[846]) + (eq47_e1352 * s.db[846][1]));
        let eq47_e1354_d_b2: f64 = ((eq47_e1352_d_b2 * s.v[846]) + (eq47_e1352 * s.db[846][2]));
        let eq47_e1354_d_b3: f64 = ((eq47_e1352_d_b3 * s.v[846]) + (eq47_e1352 * s.db[846][3]));
        let eq47_e1354_d_b4: f64 = ((eq47_e1352_d_b4 * s.v[846]) + (eq47_e1352 * s.db[846][4]));
        let eq47_e1354_d_b5: f64 = ((eq47_e1352_d_b5 * s.v[846]) + (eq47_e1352 * s.db[846][5]));
        let eq47_e1354_d_b6: f64 = ((eq47_e1352_d_b6 * s.v[846]) + (eq47_e1352 * s.db[846][6]));
        let eq47_e1355: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq47_e1354);
        let eq47_e1355_d_n0: f64 = (eq47_e1354_d_n0 * ddt_scale);
        let eq47_e1355_d_n1: f64 = (eq47_e1354_d_n1 * ddt_scale);
        let eq47_e1355_d_n2: f64 = (eq47_e1354_d_n2 * ddt_scale);
        let eq47_e1355_d_n3: f64 = (eq47_e1354_d_n3 * ddt_scale);
        let eq47_e1355_d_n4: f64 = (eq47_e1354_d_n4 * ddt_scale);
        let eq47_e1355_d_n5: f64 = (eq47_e1354_d_n5 * ddt_scale);
        let eq47_e1355_d_n6: f64 = (eq47_e1354_d_n6 * ddt_scale);
        let eq47_e1355_d_n7: f64 = (eq47_e1354_d_n7 * ddt_scale);
        let eq47_e1355_d_n8: f64 = (eq47_e1354_d_n8 * ddt_scale);
        let eq47_e1355_d_n9: f64 = (eq47_e1354_d_n9 * ddt_scale);
        let eq47_e1355_d_n10: f64 = (eq47_e1354_d_n10 * ddt_scale);
        let eq47_e1355_d_n11: f64 = (eq47_e1354_d_n11 * ddt_scale);
        let eq47_e1355_d_n12: f64 = (eq47_e1354_d_n12 * ddt_scale);
        let eq47_e1355_d_b0: f64 = (eq47_e1354_d_b0 * ddt_scale);
        let eq47_e1355_d_b1: f64 = (eq47_e1354_d_b1 * ddt_scale);
        let eq47_e1355_d_b2: f64 = (eq47_e1354_d_b2 * ddt_scale);
        let eq47_e1355_d_b3: f64 = (eq47_e1354_d_b3 * ddt_scale);
        let eq47_e1355_d_b4: f64 = (eq47_e1354_d_b4 * ddt_scale);
        let eq47_e1355_d_b5: f64 = (eq47_e1354_d_b5 * ddt_scale);
        let eq47_e1355_d_b6: f64 = (eq47_e1354_d_b6 * ddt_scale);
        let eq47_value: f64 = eq47_e1355;
        let eq47_node_derivatives: [f64; 13] = [eq47_e1355_d_n0, eq47_e1355_d_n1, eq47_e1355_d_n2, eq47_e1355_d_n3, eq47_e1355_d_n4, eq47_e1355_d_n5, eq47_e1355_d_n6, eq47_e1355_d_n7, eq47_e1355_d_n8, eq47_e1355_d_n9, eq47_e1355_d_n10, eq47_e1355_d_n11, eq47_e1355_d_n12];
        let eq47_branch_derivatives: [f64; 7] = [eq47_e1355_d_b0, eq47_e1355_d_b1, eq47_e1355_d_b2, eq47_e1355_d_b3, eq47_e1355_d_b4, eq47_e1355_d_b5, eq47_e1355_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq47_value),
            &eq47_node_derivatives,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let eq48_e1358: f64 = (s.v[0] * s.v[15]);
        let eq48_e1358_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq48_e1358_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq48_e1358_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq48_e1358_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq48_e1358_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq48_e1358_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq48_e1358_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq48_e1358_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq48_e1358_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq48_e1358_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq48_e1358_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq48_e1358_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq48_e1358_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq48_e1358_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq48_e1358_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq48_e1358_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq48_e1358_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq48_e1358_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq48_e1358_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq48_e1358_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq48_e1360: f64 = (eq48_e1358 * p.p33);
        let eq48_e1360_d_n0: f64 = (eq48_e1358_d_n0 * p.p33);
        let eq48_e1360_d_n1: f64 = (eq48_e1358_d_n1 * p.p33);
        let eq48_e1360_d_n2: f64 = (eq48_e1358_d_n2 * p.p33);
        let eq48_e1360_d_n3: f64 = (eq48_e1358_d_n3 * p.p33);
        let eq48_e1360_d_n4: f64 = (eq48_e1358_d_n4 * p.p33);
        let eq48_e1360_d_n5: f64 = (eq48_e1358_d_n5 * p.p33);
        let eq48_e1360_d_n6: f64 = (eq48_e1358_d_n6 * p.p33);
        let eq48_e1360_d_n7: f64 = (eq48_e1358_d_n7 * p.p33);
        let eq48_e1360_d_n8: f64 = (eq48_e1358_d_n8 * p.p33);
        let eq48_e1360_d_n9: f64 = (eq48_e1358_d_n9 * p.p33);
        let eq48_e1360_d_n10: f64 = (eq48_e1358_d_n10 * p.p33);
        let eq48_e1360_d_n11: f64 = (eq48_e1358_d_n11 * p.p33);
        let eq48_e1360_d_n12: f64 = (eq48_e1358_d_n12 * p.p33);
        let eq48_e1360_d_b0: f64 = (eq48_e1358_d_b0 * p.p33);
        let eq48_e1360_d_b1: f64 = (eq48_e1358_d_b1 * p.p33);
        let eq48_e1360_d_b2: f64 = (eq48_e1358_d_b2 * p.p33);
        let eq48_e1360_d_b3: f64 = (eq48_e1358_d_b3 * p.p33);
        let eq48_e1360_d_b4: f64 = (eq48_e1358_d_b4 * p.p33);
        let eq48_e1360_d_b5: f64 = (eq48_e1358_d_b5 * p.p33);
        let eq48_e1360_d_b6: f64 = (eq48_e1358_d_b6 * p.p33);
        let eq48_e1362: f64 = (eq48_e1360 * s.v[847]);
        let eq48_e1362_d_n0: f64 = ((eq48_e1360_d_n0 * s.v[847]) + (eq48_e1360 * s.dn[847][0]));
        let eq48_e1362_d_n1: f64 = ((eq48_e1360_d_n1 * s.v[847]) + (eq48_e1360 * s.dn[847][1]));
        let eq48_e1362_d_n2: f64 = ((eq48_e1360_d_n2 * s.v[847]) + (eq48_e1360 * s.dn[847][2]));
        let eq48_e1362_d_n3: f64 = ((eq48_e1360_d_n3 * s.v[847]) + (eq48_e1360 * s.dn[847][3]));
        let eq48_e1362_d_n4: f64 = ((eq48_e1360_d_n4 * s.v[847]) + (eq48_e1360 * s.dn[847][4]));
        let eq48_e1362_d_n5: f64 = ((eq48_e1360_d_n5 * s.v[847]) + (eq48_e1360 * s.dn[847][5]));
        let eq48_e1362_d_n6: f64 = ((eq48_e1360_d_n6 * s.v[847]) + (eq48_e1360 * s.dn[847][6]));
        let eq48_e1362_d_n7: f64 = ((eq48_e1360_d_n7 * s.v[847]) + (eq48_e1360 * s.dn[847][7]));
        let eq48_e1362_d_n8: f64 = ((eq48_e1360_d_n8 * s.v[847]) + (eq48_e1360 * s.dn[847][8]));
        let eq48_e1362_d_n9: f64 = ((eq48_e1360_d_n9 * s.v[847]) + (eq48_e1360 * s.dn[847][9]));
        let eq48_e1362_d_n10: f64 = ((eq48_e1360_d_n10 * s.v[847]) + (eq48_e1360 * s.dn[847][10]));
        let eq48_e1362_d_n11: f64 = ((eq48_e1360_d_n11 * s.v[847]) + (eq48_e1360 * s.dn[847][11]));
        let eq48_e1362_d_n12: f64 = ((eq48_e1360_d_n12 * s.v[847]) + (eq48_e1360 * s.dn[847][12]));
        let eq48_e1362_d_b0: f64 = ((eq48_e1360_d_b0 * s.v[847]) + (eq48_e1360 * s.db[847][0]));
        let eq48_e1362_d_b1: f64 = ((eq48_e1360_d_b1 * s.v[847]) + (eq48_e1360 * s.db[847][1]));
        let eq48_e1362_d_b2: f64 = ((eq48_e1360_d_b2 * s.v[847]) + (eq48_e1360 * s.db[847][2]));
        let eq48_e1362_d_b3: f64 = ((eq48_e1360_d_b3 * s.v[847]) + (eq48_e1360 * s.db[847][3]));
        let eq48_e1362_d_b4: f64 = ((eq48_e1360_d_b4 * s.v[847]) + (eq48_e1360 * s.db[847][4]));
        let eq48_e1362_d_b5: f64 = ((eq48_e1360_d_b5 * s.v[847]) + (eq48_e1360 * s.db[847][5]));
        let eq48_e1362_d_b6: f64 = ((eq48_e1360_d_b6 * s.v[847]) + (eq48_e1360 * s.db[847][6]));
        let eq48_e1363: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq48_e1362);
        let eq48_e1363_d_n0: f64 = (eq48_e1362_d_n0 * ddt_scale);
        let eq48_e1363_d_n1: f64 = (eq48_e1362_d_n1 * ddt_scale);
        let eq48_e1363_d_n2: f64 = (eq48_e1362_d_n2 * ddt_scale);
        let eq48_e1363_d_n3: f64 = (eq48_e1362_d_n3 * ddt_scale);
        let eq48_e1363_d_n4: f64 = (eq48_e1362_d_n4 * ddt_scale);
        let eq48_e1363_d_n5: f64 = (eq48_e1362_d_n5 * ddt_scale);
        let eq48_e1363_d_n6: f64 = (eq48_e1362_d_n6 * ddt_scale);
        let eq48_e1363_d_n7: f64 = (eq48_e1362_d_n7 * ddt_scale);
        let eq48_e1363_d_n8: f64 = (eq48_e1362_d_n8 * ddt_scale);
        let eq48_e1363_d_n9: f64 = (eq48_e1362_d_n9 * ddt_scale);
        let eq48_e1363_d_n10: f64 = (eq48_e1362_d_n10 * ddt_scale);
        let eq48_e1363_d_n11: f64 = (eq48_e1362_d_n11 * ddt_scale);
        let eq48_e1363_d_n12: f64 = (eq48_e1362_d_n12 * ddt_scale);
        let eq48_e1363_d_b0: f64 = (eq48_e1362_d_b0 * ddt_scale);
        let eq48_e1363_d_b1: f64 = (eq48_e1362_d_b1 * ddt_scale);
        let eq48_e1363_d_b2: f64 = (eq48_e1362_d_b2 * ddt_scale);
        let eq48_e1363_d_b3: f64 = (eq48_e1362_d_b3 * ddt_scale);
        let eq48_e1363_d_b4: f64 = (eq48_e1362_d_b4 * ddt_scale);
        let eq48_e1363_d_b5: f64 = (eq48_e1362_d_b5 * ddt_scale);
        let eq48_e1363_d_b6: f64 = (eq48_e1362_d_b6 * ddt_scale);
        let eq48_value: f64 = eq48_e1363;
        let eq48_node_derivatives: [f64; 13] = [eq48_e1363_d_n0, eq48_e1363_d_n1, eq48_e1363_d_n2, eq48_e1363_d_n3, eq48_e1363_d_n4, eq48_e1363_d_n5, eq48_e1363_d_n6, eq48_e1363_d_n7, eq48_e1363_d_n8, eq48_e1363_d_n9, eq48_e1363_d_n10, eq48_e1363_d_n11, eq48_e1363_d_n12];
        let eq48_branch_derivatives: [f64; 7] = [eq48_e1363_d_b0, eq48_e1363_d_b1, eq48_e1363_d_b2, eq48_e1363_d_b3, eq48_e1363_d_b4, eq48_e1363_d_b5, eq48_e1363_d_b6];
        stamper.stamp_current_dense_local(
            Some(12),
            Some(8),
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_6(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        multiplicity: f64,
        ddt_active: bool,
        ddt_scale: f64,
        ddt_state_current: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_previous: &mut [f64; Instance::DDT_STATE_COUNT],
        ddt_state_initialized: &mut [bool; Instance::DDT_STATE_COUNT],
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq50_e1371: f64 = ((nv5 - 0.0) / s.v[848]);
        let eq50_e1371_d_n0: f64 = (-(((nv5 - 0.0) * s.dn[848][0]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n1: f64 = (-(((nv5 - 0.0) * s.dn[848][1]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n2: f64 = (-(((nv5 - 0.0) * s.dn[848][2]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n3: f64 = (-(((nv5 - 0.0) * s.dn[848][3]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n4: f64 = (-(((nv5 - 0.0) * s.dn[848][4]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n5: f64 = ((s.v[848] - ((nv5 - 0.0) * s.dn[848][5])) / (s.v[848] * s.v[848]));
        let eq50_e1371_d_n6: f64 = (-(((nv5 - 0.0) * s.dn[848][6]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n7: f64 = (-(((nv5 - 0.0) * s.dn[848][7]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n8: f64 = (-(((nv5 - 0.0) * s.dn[848][8]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n9: f64 = (-(((nv5 - 0.0) * s.dn[848][9]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n10: f64 = (-(((nv5 - 0.0) * s.dn[848][10]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n11: f64 = (-(((nv5 - 0.0) * s.dn[848][11]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_n12: f64 = (-(((nv5 - 0.0) * s.dn[848][12]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b0: f64 = (-(((nv5 - 0.0) * s.db[848][0]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b1: f64 = (-(((nv5 - 0.0) * s.db[848][1]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b2: f64 = (-(((nv5 - 0.0) * s.db[848][2]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b3: f64 = (-(((nv5 - 0.0) * s.db[848][3]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b4: f64 = (-(((nv5 - 0.0) * s.db[848][4]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b5: f64 = (-(((nv5 - 0.0) * s.db[848][5]) / (s.v[848] * s.v[848])));
        let eq50_e1371_d_b6: f64 = (-(((nv5 - 0.0) * s.db[848][6]) / (s.v[848] * s.v[848])));
        let eq50_value: f64 = eq50_e1371;
        let eq50_node_derivatives: [f64; 13] = [eq50_e1371_d_n0, eq50_e1371_d_n1, eq50_e1371_d_n2, eq50_e1371_d_n3, eq50_e1371_d_n4, eq50_e1371_d_n5, eq50_e1371_d_n6, eq50_e1371_d_n7, eq50_e1371_d_n8, eq50_e1371_d_n9, eq50_e1371_d_n10, eq50_e1371_d_n11, eq50_e1371_d_n12];
        let eq50_branch_derivatives: [f64; 7] = [eq50_e1371_d_b0, eq50_e1371_d_b1, eq50_e1371_d_b2, eq50_e1371_d_b3, eq50_e1371_d_b4, eq50_e1371_d_b5, eq50_e1371_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
        let eq51_e1374: f64 = (s.v[849] * (nv5 - 0.0));
        let eq51_e1374_d_n0: f64 = (s.dn[849][0] * (nv5 - 0.0));
        let eq51_e1374_d_n1: f64 = (s.dn[849][1] * (nv5 - 0.0));
        let eq51_e1374_d_n2: f64 = (s.dn[849][2] * (nv5 - 0.0));
        let eq51_e1374_d_n3: f64 = (s.dn[849][3] * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (s.dn[849][4] * (nv5 - 0.0));
        let eq51_e1374_d_n5: f64 = ((s.dn[849][5] * (nv5 - 0.0)) + s.v[849]);
        let eq51_e1374_d_n6: f64 = (s.dn[849][6] * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (s.dn[849][7] * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (s.dn[849][8] * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (s.dn[849][9] * (nv5 - 0.0));
        let eq51_e1374_d_n10: f64 = (s.dn[849][10] * (nv5 - 0.0));
        let eq51_e1374_d_n11: f64 = (s.dn[849][11] * (nv5 - 0.0));
        let eq51_e1374_d_n12: f64 = (s.dn[849][12] * (nv5 - 0.0));
        let eq51_e1374_d_b0: f64 = (s.db[849][0] * (nv5 - 0.0));
        let eq51_e1374_d_b1: f64 = (s.db[849][1] * (nv5 - 0.0));
        let eq51_e1374_d_b2: f64 = (s.db[849][2] * (nv5 - 0.0));
        let eq51_e1374_d_b3: f64 = (s.db[849][3] * (nv5 - 0.0));
        let eq51_e1374_d_b4: f64 = (s.db[849][4] * (nv5 - 0.0));
        let eq51_e1374_d_b5: f64 = (s.db[849][5] * (nv5 - 0.0));
        let eq51_e1374_d_b6: f64 = (s.db[849][6] * (nv5 - 0.0));
        let eq51_e1375: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq51_e1374);
        let eq51_e1375_d_n0: f64 = (eq51_e1374_d_n0 * ddt_scale);
        let eq51_e1375_d_n1: f64 = (eq51_e1374_d_n1 * ddt_scale);
        let eq51_e1375_d_n2: f64 = (eq51_e1374_d_n2 * ddt_scale);
        let eq51_e1375_d_n3: f64 = (eq51_e1374_d_n3 * ddt_scale);
        let eq51_e1375_d_n4: f64 = (eq51_e1374_d_n4 * ddt_scale);
        let eq51_e1375_d_n5: f64 = (eq51_e1374_d_n5 * ddt_scale);
        let eq51_e1375_d_n6: f64 = (eq51_e1374_d_n6 * ddt_scale);
        let eq51_e1375_d_n7: f64 = (eq51_e1374_d_n7 * ddt_scale);
        let eq51_e1375_d_n8: f64 = (eq51_e1374_d_n8 * ddt_scale);
        let eq51_e1375_d_n9: f64 = (eq51_e1374_d_n9 * ddt_scale);
        let eq51_e1375_d_n10: f64 = (eq51_e1374_d_n10 * ddt_scale);
        let eq51_e1375_d_n11: f64 = (eq51_e1374_d_n11 * ddt_scale);
        let eq51_e1375_d_n12: f64 = (eq51_e1374_d_n12 * ddt_scale);
        let eq51_e1375_d_b0: f64 = (eq51_e1374_d_b0 * ddt_scale);
        let eq51_e1375_d_b1: f64 = (eq51_e1374_d_b1 * ddt_scale);
        let eq51_e1375_d_b2: f64 = (eq51_e1374_d_b2 * ddt_scale);
        let eq51_e1375_d_b3: f64 = (eq51_e1374_d_b3 * ddt_scale);
        let eq51_e1375_d_b4: f64 = (eq51_e1374_d_b4 * ddt_scale);
        let eq51_e1375_d_b5: f64 = (eq51_e1374_d_b5 * ddt_scale);
        let eq51_e1375_d_b6: f64 = (eq51_e1374_d_b6 * ddt_scale);
        let eq51_value: f64 = eq51_e1375;
        let eq51_node_derivatives: [f64; 13] = [eq51_e1375_d_n0, eq51_e1375_d_n1, eq51_e1375_d_n2, eq51_e1375_d_n3, eq51_e1375_d_n4, eq51_e1375_d_n5, eq51_e1375_d_n6, eq51_e1375_d_n7, eq51_e1375_d_n8, eq51_e1375_d_n9, eq51_e1375_d_n10, eq51_e1375_d_n11, eq51_e1375_d_n12];
        let eq51_branch_derivatives: [f64; 7] = [eq51_e1375_d_b0, eq51_e1375_d_b1, eq51_e1375_d_b2, eq51_e1375_d_b3, eq51_e1375_d_b4, eq51_e1375_d_b5, eq51_e1375_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            None,
            multiplicity * (eq51_value),
            &eq51_node_derivatives,
            &eq51_branch_derivatives,
            multiplicity,
        );
        let eq52_e1378: f64 = (s.v[15] * p.p32);
        let eq52_e1378_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq52_e1378_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq52_e1378_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq52_e1378_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq52_e1378_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq52_e1378_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq52_e1378_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq52_e1378_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq52_e1378_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq52_e1378_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq52_e1378_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq52_e1378_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq52_e1378_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq52_e1378_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq52_e1378_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq52_e1378_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq52_e1378_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq52_e1378_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq52_e1378_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq52_e1378_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let eq52_e1379_d_n0: f64 = (eq52_e1378_d_n0 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n1: f64 = (eq52_e1378_d_n1 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n2: f64 = (eq52_e1378_d_n2 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n3: f64 = (eq52_e1378_d_n3 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n4: f64 = (eq52_e1378_d_n4 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n5: f64 = (eq52_e1378_d_n5 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n6: f64 = (eq52_e1378_d_n6 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n7: f64 = (eq52_e1378_d_n7 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n8: f64 = (eq52_e1378_d_n8 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n9: f64 = (eq52_e1378_d_n9 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n10: f64 = (eq52_e1378_d_n10 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n11: f64 = (eq52_e1378_d_n11 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n12: f64 = (eq52_e1378_d_n12 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b0: f64 = (eq52_e1378_d_b0 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b1: f64 = (eq52_e1378_d_b1 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b2: f64 = (eq52_e1378_d_b2 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b3: f64 = (eq52_e1378_d_b3 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b4: f64 = (eq52_e1378_d_b4 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b5: f64 = (eq52_e1378_d_b5 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b6: f64 = (eq52_e1378_d_b6 / (2.0 * eq52_e1379));
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1381_d_n0: f64 = (eq52_e1379_d_n0 * 0.5);
        let eq52_e1381_d_n1: f64 = (eq52_e1379_d_n1 * 0.5);
        let eq52_e1381_d_n2: f64 = (eq52_e1379_d_n2 * 0.5);
        let eq52_e1381_d_n3: f64 = (eq52_e1379_d_n3 * 0.5);
        let eq52_e1381_d_n4: f64 = (eq52_e1379_d_n4 * 0.5);
        let eq52_e1381_d_n5: f64 = (eq52_e1379_d_n5 * 0.5);
        let eq52_e1381_d_n6: f64 = (eq52_e1379_d_n6 * 0.5);
        let eq52_e1381_d_n7: f64 = (eq52_e1379_d_n7 * 0.5);
        let eq52_e1381_d_n8: f64 = (eq52_e1379_d_n8 * 0.5);
        let eq52_e1381_d_n9: f64 = (eq52_e1379_d_n9 * 0.5);
        let eq52_e1381_d_n10: f64 = (eq52_e1379_d_n10 * 0.5);
        let eq52_e1381_d_n11: f64 = (eq52_e1379_d_n11 * 0.5);
        let eq52_e1381_d_n12: f64 = (eq52_e1379_d_n12 * 0.5);
        let eq52_e1381_d_b0: f64 = (eq52_e1379_d_b0 * 0.5);
        let eq52_e1381_d_b1: f64 = (eq52_e1379_d_b1 * 0.5);
        let eq52_e1381_d_b2: f64 = (eq52_e1379_d_b2 * 0.5);
        let eq52_e1381_d_b3: f64 = (eq52_e1379_d_b3 * 0.5);
        let eq52_e1381_d_b4: f64 = (eq52_e1379_d_b4 * 0.5);
        let eq52_e1381_d_b5: f64 = (eq52_e1379_d_b5 * 0.5);
        let eq52_e1381_d_b6: f64 = (eq52_e1379_d_b6 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * s.v[849]);
        let eq52_e1383_d_n0: f64 = ((eq52_e1381_d_n0 * s.v[849]) + (eq52_e1381 * s.dn[849][0]));
        let eq52_e1383_d_n1: f64 = ((eq52_e1381_d_n1 * s.v[849]) + (eq52_e1381 * s.dn[849][1]));
        let eq52_e1383_d_n2: f64 = ((eq52_e1381_d_n2 * s.v[849]) + (eq52_e1381 * s.dn[849][2]));
        let eq52_e1383_d_n3: f64 = ((eq52_e1381_d_n3 * s.v[849]) + (eq52_e1381 * s.dn[849][3]));
        let eq52_e1383_d_n4: f64 = ((eq52_e1381_d_n4 * s.v[849]) + (eq52_e1381 * s.dn[849][4]));
        let eq52_e1383_d_n5: f64 = ((eq52_e1381_d_n5 * s.v[849]) + (eq52_e1381 * s.dn[849][5]));
        let eq52_e1383_d_n6: f64 = ((eq52_e1381_d_n6 * s.v[849]) + (eq52_e1381 * s.dn[849][6]));
        let eq52_e1383_d_n7: f64 = ((eq52_e1381_d_n7 * s.v[849]) + (eq52_e1381 * s.dn[849][7]));
        let eq52_e1383_d_n8: f64 = ((eq52_e1381_d_n8 * s.v[849]) + (eq52_e1381 * s.dn[849][8]));
        let eq52_e1383_d_n9: f64 = ((eq52_e1381_d_n9 * s.v[849]) + (eq52_e1381 * s.dn[849][9]));
        let eq52_e1383_d_n10: f64 = ((eq52_e1381_d_n10 * s.v[849]) + (eq52_e1381 * s.dn[849][10]));
        let eq52_e1383_d_n11: f64 = ((eq52_e1381_d_n11 * s.v[849]) + (eq52_e1381 * s.dn[849][11]));
        let eq52_e1383_d_n12: f64 = ((eq52_e1381_d_n12 * s.v[849]) + (eq52_e1381 * s.dn[849][12]));
        let eq52_e1383_d_b0: f64 = ((eq52_e1381_d_b0 * s.v[849]) + (eq52_e1381 * s.db[849][0]));
        let eq52_e1383_d_b1: f64 = ((eq52_e1381_d_b1 * s.v[849]) + (eq52_e1381 * s.db[849][1]));
        let eq52_e1383_d_b2: f64 = ((eq52_e1381_d_b2 * s.v[849]) + (eq52_e1381 * s.db[849][2]));
        let eq52_e1383_d_b3: f64 = ((eq52_e1381_d_b3 * s.v[849]) + (eq52_e1381 * s.db[849][3]));
        let eq52_e1383_d_b4: f64 = ((eq52_e1381_d_b4 * s.v[849]) + (eq52_e1381 * s.db[849][4]));
        let eq52_e1383_d_b5: f64 = ((eq52_e1381_d_b5 * s.v[849]) + (eq52_e1381 * s.db[849][5]));
        let eq52_e1383_d_b6: f64 = ((eq52_e1381_d_b6 * s.v[849]) + (eq52_e1381 * s.db[849][6]));
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n0: f64 = (eq52_e1383_d_n0 * (nv5 - 0.0));
        let eq52_e1385_d_n1: f64 = (eq52_e1383_d_n1 * (nv5 - 0.0));
        let eq52_e1385_d_n2: f64 = (eq52_e1383_d_n2 * (nv5 - 0.0));
        let eq52_e1385_d_n3: f64 = (eq52_e1383_d_n3 * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));
        let eq52_e1385_d_n5: f64 = ((eq52_e1383_d_n5 * (nv5 - 0.0)) + eq52_e1383);
        let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));
        let eq52_e1385_d_n10: f64 = (eq52_e1383_d_n10 * (nv5 - 0.0));
        let eq52_e1385_d_n11: f64 = (eq52_e1383_d_n11 * (nv5 - 0.0));
        let eq52_e1385_d_n12: f64 = (eq52_e1383_d_n12 * (nv5 - 0.0));
        let eq52_e1385_d_b0: f64 = (eq52_e1383_d_b0 * (nv5 - 0.0));
        let eq52_e1385_d_b1: f64 = (eq52_e1383_d_b1 * (nv5 - 0.0));
        let eq52_e1385_d_b2: f64 = (eq52_e1383_d_b2 * (nv5 - 0.0));
        let eq52_e1385_d_b3: f64 = (eq52_e1383_d_b3 * (nv5 - 0.0));
        let eq52_e1385_d_b4: f64 = (eq52_e1383_d_b4 * (nv5 - 0.0));
        let eq52_e1385_d_b5: f64 = (eq52_e1383_d_b5 * (nv5 - 0.0));
        let eq52_e1385_d_b6: f64 = (eq52_e1383_d_b6 * (nv5 - 0.0));
        let eq52_e1386: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq52_e1385);
        let eq52_e1386_d_n0: f64 = (eq52_e1385_d_n0 * ddt_scale);
        let eq52_e1386_d_n1: f64 = (eq52_e1385_d_n1 * ddt_scale);
        let eq52_e1386_d_n2: f64 = (eq52_e1385_d_n2 * ddt_scale);
        let eq52_e1386_d_n3: f64 = (eq52_e1385_d_n3 * ddt_scale);
        let eq52_e1386_d_n4: f64 = (eq52_e1385_d_n4 * ddt_scale);
        let eq52_e1386_d_n5: f64 = (eq52_e1385_d_n5 * ddt_scale);
        let eq52_e1386_d_n6: f64 = (eq52_e1385_d_n6 * ddt_scale);
        let eq52_e1386_d_n7: f64 = (eq52_e1385_d_n7 * ddt_scale);
        let eq52_e1386_d_n8: f64 = (eq52_e1385_d_n8 * ddt_scale);
        let eq52_e1386_d_n9: f64 = (eq52_e1385_d_n9 * ddt_scale);
        let eq52_e1386_d_n10: f64 = (eq52_e1385_d_n10 * ddt_scale);
        let eq52_e1386_d_n11: f64 = (eq52_e1385_d_n11 * ddt_scale);
        let eq52_e1386_d_n12: f64 = (eq52_e1385_d_n12 * ddt_scale);
        let eq52_e1386_d_b0: f64 = (eq52_e1385_d_b0 * ddt_scale);
        let eq52_e1386_d_b1: f64 = (eq52_e1385_d_b1 * ddt_scale);
        let eq52_e1386_d_b2: f64 = (eq52_e1385_d_b2 * ddt_scale);
        let eq52_e1386_d_b3: f64 = (eq52_e1385_d_b3 * ddt_scale);
        let eq52_e1386_d_b4: f64 = (eq52_e1385_d_b4 * ddt_scale);
        let eq52_e1386_d_b5: f64 = (eq52_e1385_d_b5 * ddt_scale);
        let eq52_e1386_d_b6: f64 = (eq52_e1385_d_b6 * ddt_scale);
        let eq52_e1387: f64 = (-eq52_e1386);
        let eq52_e1387_d_n0: f64 = (-eq52_e1386_d_n0);
        let eq52_e1387_d_n1: f64 = (-eq52_e1386_d_n1);
        let eq52_e1387_d_n2: f64 = (-eq52_e1386_d_n2);
        let eq52_e1387_d_n3: f64 = (-eq52_e1386_d_n3);
        let eq52_e1387_d_n4: f64 = (-eq52_e1386_d_n4);
        let eq52_e1387_d_n5: f64 = (-eq52_e1386_d_n5);
        let eq52_e1387_d_n6: f64 = (-eq52_e1386_d_n6);
        let eq52_e1387_d_n7: f64 = (-eq52_e1386_d_n7);
        let eq52_e1387_d_n8: f64 = (-eq52_e1386_d_n8);
        let eq52_e1387_d_n9: f64 = (-eq52_e1386_d_n9);
        let eq52_e1387_d_n10: f64 = (-eq52_e1386_d_n10);
        let eq52_e1387_d_n11: f64 = (-eq52_e1386_d_n11);
        let eq52_e1387_d_n12: f64 = (-eq52_e1386_d_n12);
        let eq52_e1387_d_b0: f64 = (-eq52_e1386_d_b0);
        let eq52_e1387_d_b1: f64 = (-eq52_e1386_d_b1);
        let eq52_e1387_d_b2: f64 = (-eq52_e1386_d_b2);
        let eq52_e1387_d_b3: f64 = (-eq52_e1386_d_b3);
        let eq52_e1387_d_b4: f64 = (-eq52_e1386_d_b4);
        let eq52_e1387_d_b5: f64 = (-eq52_e1386_d_b5);
        let eq52_e1387_d_b6: f64 = (-eq52_e1386_d_b6);
        let eq52_value: f64 = eq52_e1387;
        let eq52_node_derivatives: [f64; 13] = [eq52_e1387_d_n0, eq52_e1387_d_n1, eq52_e1387_d_n2, eq52_e1387_d_n3, eq52_e1387_d_n4, eq52_e1387_d_n5, eq52_e1387_d_n6, eq52_e1387_d_n7, eq52_e1387_d_n8, eq52_e1387_d_n9, eq52_e1387_d_n10, eq52_e1387_d_n11, eq52_e1387_d_n12];
        let eq52_branch_derivatives: [f64; 7] = [eq52_e1387_d_b0, eq52_e1387_d_b1, eq52_e1387_d_b2, eq52_e1387_d_b3, eq52_e1387_d_b4, eq52_e1387_d_b5, eq52_e1387_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq52_value),
            &eq52_node_derivatives,
            &eq52_branch_derivatives,
            multiplicity,
        );
        let eq53_e1390: f64 = (s.v[15] * p.p32);
        let eq53_e1390_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq53_e1390_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq53_e1390_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq53_e1390_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq53_e1390_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq53_e1390_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq53_e1390_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq53_e1390_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq53_e1390_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq53_e1390_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq53_e1390_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq53_e1390_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq53_e1390_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq53_e1390_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq53_e1390_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq53_e1390_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq53_e1390_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq53_e1390_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq53_e1390_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq53_e1390_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let eq53_e1391_d_n0: f64 = (eq53_e1390_d_n0 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n1: f64 = (eq53_e1390_d_n1 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n2: f64 = (eq53_e1390_d_n2 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n3: f64 = (eq53_e1390_d_n3 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n4: f64 = (eq53_e1390_d_n4 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n5: f64 = (eq53_e1390_d_n5 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n6: f64 = (eq53_e1390_d_n6 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n7: f64 = (eq53_e1390_d_n7 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n8: f64 = (eq53_e1390_d_n8 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n9: f64 = (eq53_e1390_d_n9 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n10: f64 = (eq53_e1390_d_n10 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n11: f64 = (eq53_e1390_d_n11 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n12: f64 = (eq53_e1390_d_n12 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b0: f64 = (eq53_e1390_d_b0 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b1: f64 = (eq53_e1390_d_b1 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b2: f64 = (eq53_e1390_d_b2 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b3: f64 = (eq53_e1390_d_b3 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b4: f64 = (eq53_e1390_d_b4 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b5: f64 = (eq53_e1390_d_b5 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b6: f64 = (eq53_e1390_d_b6 / (2.0 * eq53_e1391));
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1393_d_n0: f64 = (eq53_e1391_d_n0 * 0.5);
        let eq53_e1393_d_n1: f64 = (eq53_e1391_d_n1 * 0.5);
        let eq53_e1393_d_n2: f64 = (eq53_e1391_d_n2 * 0.5);
        let eq53_e1393_d_n3: f64 = (eq53_e1391_d_n3 * 0.5);
        let eq53_e1393_d_n4: f64 = (eq53_e1391_d_n4 * 0.5);
        let eq53_e1393_d_n5: f64 = (eq53_e1391_d_n5 * 0.5);
        let eq53_e1393_d_n6: f64 = (eq53_e1391_d_n6 * 0.5);
        let eq53_e1393_d_n7: f64 = (eq53_e1391_d_n7 * 0.5);
        let eq53_e1393_d_n8: f64 = (eq53_e1391_d_n8 * 0.5);
        let eq53_e1393_d_n9: f64 = (eq53_e1391_d_n9 * 0.5);
        let eq53_e1393_d_n10: f64 = (eq53_e1391_d_n10 * 0.5);
        let eq53_e1393_d_n11: f64 = (eq53_e1391_d_n11 * 0.5);
        let eq53_e1393_d_n12: f64 = (eq53_e1391_d_n12 * 0.5);
        let eq53_e1393_d_b0: f64 = (eq53_e1391_d_b0 * 0.5);
        let eq53_e1393_d_b1: f64 = (eq53_e1391_d_b1 * 0.5);
        let eq53_e1393_d_b2: f64 = (eq53_e1391_d_b2 * 0.5);
        let eq53_e1393_d_b3: f64 = (eq53_e1391_d_b3 * 0.5);
        let eq53_e1393_d_b4: f64 = (eq53_e1391_d_b4 * 0.5);
        let eq53_e1393_d_b5: f64 = (eq53_e1391_d_b5 * 0.5);
        let eq53_e1393_d_b6: f64 = (eq53_e1391_d_b6 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * s.v[849]);
        let eq53_e1395_d_n0: f64 = ((eq53_e1393_d_n0 * s.v[849]) + (eq53_e1393 * s.dn[849][0]));
        let eq53_e1395_d_n1: f64 = ((eq53_e1393_d_n1 * s.v[849]) + (eq53_e1393 * s.dn[849][1]));
        let eq53_e1395_d_n2: f64 = ((eq53_e1393_d_n2 * s.v[849]) + (eq53_e1393 * s.dn[849][2]));
        let eq53_e1395_d_n3: f64 = ((eq53_e1393_d_n3 * s.v[849]) + (eq53_e1393 * s.dn[849][3]));
        let eq53_e1395_d_n4: f64 = ((eq53_e1393_d_n4 * s.v[849]) + (eq53_e1393 * s.dn[849][4]));
        let eq53_e1395_d_n5: f64 = ((eq53_e1393_d_n5 * s.v[849]) + (eq53_e1393 * s.dn[849][5]));
        let eq53_e1395_d_n6: f64 = ((eq53_e1393_d_n6 * s.v[849]) + (eq53_e1393 * s.dn[849][6]));
        let eq53_e1395_d_n7: f64 = ((eq53_e1393_d_n7 * s.v[849]) + (eq53_e1393 * s.dn[849][7]));
        let eq53_e1395_d_n8: f64 = ((eq53_e1393_d_n8 * s.v[849]) + (eq53_e1393 * s.dn[849][8]));
        let eq53_e1395_d_n9: f64 = ((eq53_e1393_d_n9 * s.v[849]) + (eq53_e1393 * s.dn[849][9]));
        let eq53_e1395_d_n10: f64 = ((eq53_e1393_d_n10 * s.v[849]) + (eq53_e1393 * s.dn[849][10]));
        let eq53_e1395_d_n11: f64 = ((eq53_e1393_d_n11 * s.v[849]) + (eq53_e1393 * s.dn[849][11]));
        let eq53_e1395_d_n12: f64 = ((eq53_e1393_d_n12 * s.v[849]) + (eq53_e1393 * s.dn[849][12]));
        let eq53_e1395_d_b0: f64 = ((eq53_e1393_d_b0 * s.v[849]) + (eq53_e1393 * s.db[849][0]));
        let eq53_e1395_d_b1: f64 = ((eq53_e1393_d_b1 * s.v[849]) + (eq53_e1393 * s.db[849][1]));
        let eq53_e1395_d_b2: f64 = ((eq53_e1393_d_b2 * s.v[849]) + (eq53_e1393 * s.db[849][2]));
        let eq53_e1395_d_b3: f64 = ((eq53_e1393_d_b3 * s.v[849]) + (eq53_e1393 * s.db[849][3]));
        let eq53_e1395_d_b4: f64 = ((eq53_e1393_d_b4 * s.v[849]) + (eq53_e1393 * s.db[849][4]));
        let eq53_e1395_d_b5: f64 = ((eq53_e1393_d_b5 * s.v[849]) + (eq53_e1393 * s.db[849][5]));
        let eq53_e1395_d_b6: f64 = ((eq53_e1393_d_b6 * s.v[849]) + (eq53_e1393 * s.db[849][6]));
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n0: f64 = (eq53_e1395_d_n0 * (nv5 - 0.0));
        let eq53_e1397_d_n1: f64 = (eq53_e1395_d_n1 * (nv5 - 0.0));
        let eq53_e1397_d_n2: f64 = (eq53_e1395_d_n2 * (nv5 - 0.0));
        let eq53_e1397_d_n3: f64 = (eq53_e1395_d_n3 * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));
        let eq53_e1397_d_n5: f64 = ((eq53_e1395_d_n5 * (nv5 - 0.0)) + eq53_e1395);
        let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));
        let eq53_e1397_d_n10: f64 = (eq53_e1395_d_n10 * (nv5 - 0.0));
        let eq53_e1397_d_n11: f64 = (eq53_e1395_d_n11 * (nv5 - 0.0));
        let eq53_e1397_d_n12: f64 = (eq53_e1395_d_n12 * (nv5 - 0.0));
        let eq53_e1397_d_b0: f64 = (eq53_e1395_d_b0 * (nv5 - 0.0));
        let eq53_e1397_d_b1: f64 = (eq53_e1395_d_b1 * (nv5 - 0.0));
        let eq53_e1397_d_b2: f64 = (eq53_e1395_d_b2 * (nv5 - 0.0));
        let eq53_e1397_d_b3: f64 = (eq53_e1395_d_b3 * (nv5 - 0.0));
        let eq53_e1397_d_b4: f64 = (eq53_e1395_d_b4 * (nv5 - 0.0));
        let eq53_e1397_d_b5: f64 = (eq53_e1395_d_b5 * (nv5 - 0.0));
        let eq53_e1397_d_b6: f64 = (eq53_e1395_d_b6 * (nv5 - 0.0));
        let eq53_e1398: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 11, eq53_e1397);
        let eq53_e1398_d_n0: f64 = (eq53_e1397_d_n0 * ddt_scale);
        let eq53_e1398_d_n1: f64 = (eq53_e1397_d_n1 * ddt_scale);
        let eq53_e1398_d_n2: f64 = (eq53_e1397_d_n2 * ddt_scale);
        let eq53_e1398_d_n3: f64 = (eq53_e1397_d_n3 * ddt_scale);
        let eq53_e1398_d_n4: f64 = (eq53_e1397_d_n4 * ddt_scale);
        let eq53_e1398_d_n5: f64 = (eq53_e1397_d_n5 * ddt_scale);
        let eq53_e1398_d_n6: f64 = (eq53_e1397_d_n6 * ddt_scale);
        let eq53_e1398_d_n7: f64 = (eq53_e1397_d_n7 * ddt_scale);
        let eq53_e1398_d_n8: f64 = (eq53_e1397_d_n8 * ddt_scale);
        let eq53_e1398_d_n9: f64 = (eq53_e1397_d_n9 * ddt_scale);
        let eq53_e1398_d_n10: f64 = (eq53_e1397_d_n10 * ddt_scale);
        let eq53_e1398_d_n11: f64 = (eq53_e1397_d_n11 * ddt_scale);
        let eq53_e1398_d_n12: f64 = (eq53_e1397_d_n12 * ddt_scale);
        let eq53_e1398_d_b0: f64 = (eq53_e1397_d_b0 * ddt_scale);
        let eq53_e1398_d_b1: f64 = (eq53_e1397_d_b1 * ddt_scale);
        let eq53_e1398_d_b2: f64 = (eq53_e1397_d_b2 * ddt_scale);
        let eq53_e1398_d_b3: f64 = (eq53_e1397_d_b3 * ddt_scale);
        let eq53_e1398_d_b4: f64 = (eq53_e1397_d_b4 * ddt_scale);
        let eq53_e1398_d_b5: f64 = (eq53_e1397_d_b5 * ddt_scale);
        let eq53_e1398_d_b6: f64 = (eq53_e1397_d_b6 * ddt_scale);
        let eq53_e1399: f64 = (-eq53_e1398);
        let eq53_e1399_d_n0: f64 = (-eq53_e1398_d_n0);
        let eq53_e1399_d_n1: f64 = (-eq53_e1398_d_n1);
        let eq53_e1399_d_n2: f64 = (-eq53_e1398_d_n2);
        let eq53_e1399_d_n3: f64 = (-eq53_e1398_d_n3);
        let eq53_e1399_d_n4: f64 = (-eq53_e1398_d_n4);
        let eq53_e1399_d_n5: f64 = (-eq53_e1398_d_n5);
        let eq53_e1399_d_n6: f64 = (-eq53_e1398_d_n6);
        let eq53_e1399_d_n7: f64 = (-eq53_e1398_d_n7);
        let eq53_e1399_d_n8: f64 = (-eq53_e1398_d_n8);
        let eq53_e1399_d_n9: f64 = (-eq53_e1398_d_n9);
        let eq53_e1399_d_n10: f64 = (-eq53_e1398_d_n10);
        let eq53_e1399_d_n11: f64 = (-eq53_e1398_d_n11);
        let eq53_e1399_d_n12: f64 = (-eq53_e1398_d_n12);
        let eq53_e1399_d_b0: f64 = (-eq53_e1398_d_b0);
        let eq53_e1399_d_b1: f64 = (-eq53_e1398_d_b1);
        let eq53_e1399_d_b2: f64 = (-eq53_e1398_d_b2);
        let eq53_e1399_d_b3: f64 = (-eq53_e1398_d_b3);
        let eq53_e1399_d_b4: f64 = (-eq53_e1398_d_b4);
        let eq53_e1399_d_b5: f64 = (-eq53_e1398_d_b5);
        let eq53_e1399_d_b6: f64 = (-eq53_e1398_d_b6);
        let eq53_value: f64 = eq53_e1399;
        let eq53_node_derivatives: [f64; 13] = [eq53_e1399_d_n0, eq53_e1399_d_n1, eq53_e1399_d_n2, eq53_e1399_d_n3, eq53_e1399_d_n4, eq53_e1399_d_n5, eq53_e1399_d_n6, eq53_e1399_d_n7, eq53_e1399_d_n8, eq53_e1399_d_n9, eq53_e1399_d_n10, eq53_e1399_d_n11, eq53_e1399_d_n12];
        let eq53_branch_derivatives: [f64; 7] = [eq53_e1399_d_b0, eq53_e1399_d_b1, eq53_e1399_d_b2, eq53_e1399_d_b3, eq53_e1399_d_b4, eq53_e1399_d_b5, eq53_e1399_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq53_value),
            &eq53_node_derivatives,
            &eq53_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq39_e1291: f64 = (s.v[15] * s.v[306]);
        let eq39_e1291_d_n0: f64 = ((s.dn[15][0] * s.v[306]) + (s.v[15] * s.dn[306][0]));
        let eq39_e1291_d_n1: f64 = ((s.dn[15][1] * s.v[306]) + (s.v[15] * s.dn[306][1]));
        let eq39_e1291_d_n2: f64 = ((s.dn[15][2] * s.v[306]) + (s.v[15] * s.dn[306][2]));
        let eq39_e1291_d_n3: f64 = ((s.dn[15][3] * s.v[306]) + (s.v[15] * s.dn[306][3]));
        let eq39_e1291_d_n4: f64 = ((s.dn[15][4] * s.v[306]) + (s.v[15] * s.dn[306][4]));
        let eq39_e1291_d_n5: f64 = ((s.dn[15][5] * s.v[306]) + (s.v[15] * s.dn[306][5]));
        let eq39_e1291_d_n6: f64 = ((s.dn[15][6] * s.v[306]) + (s.v[15] * s.dn[306][6]));
        let eq39_e1291_d_n7: f64 = ((s.dn[15][7] * s.v[306]) + (s.v[15] * s.dn[306][7]));
        let eq39_e1291_d_n8: f64 = ((s.dn[15][8] * s.v[306]) + (s.v[15] * s.dn[306][8]));
        let eq39_e1291_d_n9: f64 = ((s.dn[15][9] * s.v[306]) + (s.v[15] * s.dn[306][9]));
        let eq39_e1291_d_n10: f64 = ((s.dn[15][10] * s.v[306]) + (s.v[15] * s.dn[306][10]));
        let eq39_e1291_d_n11: f64 = ((s.dn[15][11] * s.v[306]) + (s.v[15] * s.dn[306][11]));
        let eq39_e1291_d_n12: f64 = ((s.dn[15][12] * s.v[306]) + (s.v[15] * s.dn[306][12]));
        let eq39_e1291_d_b0: f64 = ((s.db[15][0] * s.v[306]) + (s.v[15] * s.db[306][0]));
        let eq39_e1291_d_b1: f64 = ((s.db[15][1] * s.v[306]) + (s.v[15] * s.db[306][1]));
        let eq39_e1291_d_b2: f64 = ((s.db[15][2] * s.v[306]) + (s.v[15] * s.db[306][2]));
        let eq39_e1291_d_b3: f64 = ((s.db[15][3] * s.v[306]) + (s.v[15] * s.db[306][3]));
        let eq39_e1291_d_b4: f64 = ((s.db[15][4] * s.v[306]) + (s.v[15] * s.db[306][4]));
        let eq39_e1291_d_b5: f64 = ((s.db[15][5] * s.v[306]) + (s.v[15] * s.db[306][5]));
        let eq39_e1291_d_b6: f64 = ((s.db[15][6] * s.v[306]) + (s.v[15] * s.db[306][6]));
        let eq39_e1293: f64 = (eq39_e1291 * (nv4 - 0.0));
        let eq39_e1293_d_n0: f64 = (eq39_e1291_d_n0 * (nv4 - 0.0));
        let eq39_e1293_d_n1: f64 = (eq39_e1291_d_n1 * (nv4 - 0.0));
        let eq39_e1293_d_n2: f64 = (eq39_e1291_d_n2 * (nv4 - 0.0));
        let eq39_e1293_d_n3: f64 = (eq39_e1291_d_n3 * (nv4 - 0.0));
        let eq39_e1293_d_n4: f64 = ((eq39_e1291_d_n4 * (nv4 - 0.0)) + eq39_e1291);
        let eq39_e1293_d_n5: f64 = (eq39_e1291_d_n5 * (nv4 - 0.0));
        let eq39_e1293_d_n6: f64 = (eq39_e1291_d_n6 * (nv4 - 0.0));
        let eq39_e1293_d_n7: f64 = (eq39_e1291_d_n7 * (nv4 - 0.0));
        let eq39_e1293_d_n8: f64 = (eq39_e1291_d_n8 * (nv4 - 0.0));
        let eq39_e1293_d_n9: f64 = (eq39_e1291_d_n9 * (nv4 - 0.0));
        let eq39_e1293_d_n10: f64 = (eq39_e1291_d_n10 * (nv4 - 0.0));
        let eq39_e1293_d_n11: f64 = (eq39_e1291_d_n11 * (nv4 - 0.0));
        let eq39_e1293_d_n12: f64 = (eq39_e1291_d_n12 * (nv4 - 0.0));
        let eq39_e1293_d_b0: f64 = (eq39_e1291_d_b0 * (nv4 - 0.0));
        let eq39_e1293_d_b1: f64 = (eq39_e1291_d_b1 * (nv4 - 0.0));
        let eq39_e1293_d_b2: f64 = (eq39_e1291_d_b2 * (nv4 - 0.0));
        let eq39_e1293_d_b3: f64 = (eq39_e1291_d_b3 * (nv4 - 0.0));
        let eq39_e1293_d_b4: f64 = (eq39_e1291_d_b4 * (nv4 - 0.0));
        let eq39_e1293_d_b5: f64 = (eq39_e1291_d_b5 * (nv4 - 0.0));
        let eq39_e1293_d_b6: f64 = (eq39_e1291_d_b6 * (nv4 - 0.0));
        let eq39_e1294_q: f64 = eq39_e1293;
        let eq39_reactive_node_derivatives: [f64; 13] = [eq39_e1293_d_n0, eq39_e1293_d_n1, eq39_e1293_d_n2, eq39_e1293_d_n3, eq39_e1293_d_n4, eq39_e1293_d_n5, eq39_e1293_d_n6, eq39_e1293_d_n7, eq39_e1293_d_n8, eq39_e1293_d_n9, eq39_e1293_d_n10, eq39_e1293_d_n11, eq39_e1293_d_n12];
        let eq39_reactive_branch_derivatives: [f64; 7] = [eq39_e1293_d_b0, eq39_e1293_d_b1, eq39_e1293_d_b2, eq39_e1293_d_b3, eq39_e1293_d_b4, eq39_e1293_d_b5, eq39_e1293_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e1302: f64 = (s.v[0] * s.v[15]);
        let eq41_e1302_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq41_e1302_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq41_e1302_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq41_e1302_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq41_e1302_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq41_e1302_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq41_e1302_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq41_e1302_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq41_e1302_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq41_e1302_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq41_e1302_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq41_e1302_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq41_e1302_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq41_e1302_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq41_e1302_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq41_e1302_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq41_e1302_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq41_e1302_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq41_e1302_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq41_e1302_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq41_e1304: f64 = (eq41_e1302 * p.p33);
        let eq41_e1304_d_n0: f64 = (eq41_e1302_d_n0 * p.p33);
        let eq41_e1304_d_n1: f64 = (eq41_e1302_d_n1 * p.p33);
        let eq41_e1304_d_n2: f64 = (eq41_e1302_d_n2 * p.p33);
        let eq41_e1304_d_n3: f64 = (eq41_e1302_d_n3 * p.p33);
        let eq41_e1304_d_n4: f64 = (eq41_e1302_d_n4 * p.p33);
        let eq41_e1304_d_n5: f64 = (eq41_e1302_d_n5 * p.p33);
        let eq41_e1304_d_n6: f64 = (eq41_e1302_d_n6 * p.p33);
        let eq41_e1304_d_n7: f64 = (eq41_e1302_d_n7 * p.p33);
        let eq41_e1304_d_n8: f64 = (eq41_e1302_d_n8 * p.p33);
        let eq41_e1304_d_n9: f64 = (eq41_e1302_d_n9 * p.p33);
        let eq41_e1304_d_n10: f64 = (eq41_e1302_d_n10 * p.p33);
        let eq41_e1304_d_n11: f64 = (eq41_e1302_d_n11 * p.p33);
        let eq41_e1304_d_n12: f64 = (eq41_e1302_d_n12 * p.p33);
        let eq41_e1304_d_b0: f64 = (eq41_e1302_d_b0 * p.p33);
        let eq41_e1304_d_b1: f64 = (eq41_e1302_d_b1 * p.p33);
        let eq41_e1304_d_b2: f64 = (eq41_e1302_d_b2 * p.p33);
        let eq41_e1304_d_b3: f64 = (eq41_e1302_d_b3 * p.p33);
        let eq41_e1304_d_b4: f64 = (eq41_e1302_d_b4 * p.p33);
        let eq41_e1304_d_b5: f64 = (eq41_e1302_d_b5 * p.p33);
        let eq41_e1304_d_b6: f64 = (eq41_e1302_d_b6 * p.p33);
        let eq41_e1306: f64 = (eq41_e1304 * s.v[840]);
        let eq41_e1306_d_n0: f64 = ((eq41_e1304_d_n0 * s.v[840]) + (eq41_e1304 * s.dn[840][0]));
        let eq41_e1306_d_n1: f64 = ((eq41_e1304_d_n1 * s.v[840]) + (eq41_e1304 * s.dn[840][1]));
        let eq41_e1306_d_n2: f64 = ((eq41_e1304_d_n2 * s.v[840]) + (eq41_e1304 * s.dn[840][2]));
        let eq41_e1306_d_n3: f64 = ((eq41_e1304_d_n3 * s.v[840]) + (eq41_e1304 * s.dn[840][3]));
        let eq41_e1306_d_n4: f64 = ((eq41_e1304_d_n4 * s.v[840]) + (eq41_e1304 * s.dn[840][4]));
        let eq41_e1306_d_n5: f64 = ((eq41_e1304_d_n5 * s.v[840]) + (eq41_e1304 * s.dn[840][5]));
        let eq41_e1306_d_n6: f64 = ((eq41_e1304_d_n6 * s.v[840]) + (eq41_e1304 * s.dn[840][6]));
        let eq41_e1306_d_n7: f64 = ((eq41_e1304_d_n7 * s.v[840]) + (eq41_e1304 * s.dn[840][7]));
        let eq41_e1306_d_n8: f64 = ((eq41_e1304_d_n8 * s.v[840]) + (eq41_e1304 * s.dn[840][8]));
        let eq41_e1306_d_n9: f64 = ((eq41_e1304_d_n9 * s.v[840]) + (eq41_e1304 * s.dn[840][9]));
        let eq41_e1306_d_n10: f64 = ((eq41_e1304_d_n10 * s.v[840]) + (eq41_e1304 * s.dn[840][10]));
        let eq41_e1306_d_n11: f64 = ((eq41_e1304_d_n11 * s.v[840]) + (eq41_e1304 * s.dn[840][11]));
        let eq41_e1306_d_n12: f64 = ((eq41_e1304_d_n12 * s.v[840]) + (eq41_e1304 * s.dn[840][12]));
        let eq41_e1306_d_b0: f64 = ((eq41_e1304_d_b0 * s.v[840]) + (eq41_e1304 * s.db[840][0]));
        let eq41_e1306_d_b1: f64 = ((eq41_e1304_d_b1 * s.v[840]) + (eq41_e1304 * s.db[840][1]));
        let eq41_e1306_d_b2: f64 = ((eq41_e1304_d_b2 * s.v[840]) + (eq41_e1304 * s.db[840][2]));
        let eq41_e1306_d_b3: f64 = ((eq41_e1304_d_b3 * s.v[840]) + (eq41_e1304 * s.db[840][3]));
        let eq41_e1306_d_b4: f64 = ((eq41_e1304_d_b4 * s.v[840]) + (eq41_e1304 * s.db[840][4]));
        let eq41_e1306_d_b5: f64 = ((eq41_e1304_d_b5 * s.v[840]) + (eq41_e1304 * s.db[840][5]));
        let eq41_e1306_d_b6: f64 = ((eq41_e1304_d_b6 * s.v[840]) + (eq41_e1304 * s.db[840][6]));
        let eq41_e1307_q: f64 = eq41_e1306;
        let eq41_reactive_node_derivatives: [f64; 13] = [eq41_e1306_d_n0, eq41_e1306_d_n1, eq41_e1306_d_n2, eq41_e1306_d_n3, eq41_e1306_d_n4, eq41_e1306_d_n5, eq41_e1306_d_n6, eq41_e1306_d_n7, eq41_e1306_d_n8, eq41_e1306_d_n9, eq41_e1306_d_n10, eq41_e1306_d_n11, eq41_e1306_d_n12];
        let eq41_reactive_branch_derivatives: [f64; 7] = [eq41_e1306_d_b0, eq41_e1306_d_b1, eq41_e1306_d_b2, eq41_e1306_d_b3, eq41_e1306_d_b4, eq41_e1306_d_b5, eq41_e1306_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1310: f64 = (s.v[0] * s.v[15]);
        let eq42_e1310_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq42_e1310_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq42_e1310_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq42_e1310_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq42_e1310_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq42_e1310_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq42_e1310_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq42_e1310_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq42_e1310_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq42_e1310_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq42_e1310_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq42_e1310_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq42_e1310_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq42_e1310_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq42_e1310_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq42_e1310_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq42_e1310_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq42_e1310_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq42_e1310_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq42_e1310_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq42_e1312: f64 = (eq42_e1310 * p.p33);
        let eq42_e1312_d_n0: f64 = (eq42_e1310_d_n0 * p.p33);
        let eq42_e1312_d_n1: f64 = (eq42_e1310_d_n1 * p.p33);
        let eq42_e1312_d_n2: f64 = (eq42_e1310_d_n2 * p.p33);
        let eq42_e1312_d_n3: f64 = (eq42_e1310_d_n3 * p.p33);
        let eq42_e1312_d_n4: f64 = (eq42_e1310_d_n4 * p.p33);
        let eq42_e1312_d_n5: f64 = (eq42_e1310_d_n5 * p.p33);
        let eq42_e1312_d_n6: f64 = (eq42_e1310_d_n6 * p.p33);
        let eq42_e1312_d_n7: f64 = (eq42_e1310_d_n7 * p.p33);
        let eq42_e1312_d_n8: f64 = (eq42_e1310_d_n8 * p.p33);
        let eq42_e1312_d_n9: f64 = (eq42_e1310_d_n9 * p.p33);
        let eq42_e1312_d_n10: f64 = (eq42_e1310_d_n10 * p.p33);
        let eq42_e1312_d_n11: f64 = (eq42_e1310_d_n11 * p.p33);
        let eq42_e1312_d_n12: f64 = (eq42_e1310_d_n12 * p.p33);
        let eq42_e1312_d_b0: f64 = (eq42_e1310_d_b0 * p.p33);
        let eq42_e1312_d_b1: f64 = (eq42_e1310_d_b1 * p.p33);
        let eq42_e1312_d_b2: f64 = (eq42_e1310_d_b2 * p.p33);
        let eq42_e1312_d_b3: f64 = (eq42_e1310_d_b3 * p.p33);
        let eq42_e1312_d_b4: f64 = (eq42_e1310_d_b4 * p.p33);
        let eq42_e1312_d_b5: f64 = (eq42_e1310_d_b5 * p.p33);
        let eq42_e1312_d_b6: f64 = (eq42_e1310_d_b6 * p.p33);
        let eq42_e1314: f64 = (eq42_e1312 * s.v[841]);
        let eq42_e1314_d_n0: f64 = ((eq42_e1312_d_n0 * s.v[841]) + (eq42_e1312 * s.dn[841][0]));
        let eq42_e1314_d_n1: f64 = ((eq42_e1312_d_n1 * s.v[841]) + (eq42_e1312 * s.dn[841][1]));
        let eq42_e1314_d_n2: f64 = ((eq42_e1312_d_n2 * s.v[841]) + (eq42_e1312 * s.dn[841][2]));
        let eq42_e1314_d_n3: f64 = ((eq42_e1312_d_n3 * s.v[841]) + (eq42_e1312 * s.dn[841][3]));
        let eq42_e1314_d_n4: f64 = ((eq42_e1312_d_n4 * s.v[841]) + (eq42_e1312 * s.dn[841][4]));
        let eq42_e1314_d_n5: f64 = ((eq42_e1312_d_n5 * s.v[841]) + (eq42_e1312 * s.dn[841][5]));
        let eq42_e1314_d_n6: f64 = ((eq42_e1312_d_n6 * s.v[841]) + (eq42_e1312 * s.dn[841][6]));
        let eq42_e1314_d_n7: f64 = ((eq42_e1312_d_n7 * s.v[841]) + (eq42_e1312 * s.dn[841][7]));
        let eq42_e1314_d_n8: f64 = ((eq42_e1312_d_n8 * s.v[841]) + (eq42_e1312 * s.dn[841][8]));
        let eq42_e1314_d_n9: f64 = ((eq42_e1312_d_n9 * s.v[841]) + (eq42_e1312 * s.dn[841][9]));
        let eq42_e1314_d_n10: f64 = ((eq42_e1312_d_n10 * s.v[841]) + (eq42_e1312 * s.dn[841][10]));
        let eq42_e1314_d_n11: f64 = ((eq42_e1312_d_n11 * s.v[841]) + (eq42_e1312 * s.dn[841][11]));
        let eq42_e1314_d_n12: f64 = ((eq42_e1312_d_n12 * s.v[841]) + (eq42_e1312 * s.dn[841][12]));
        let eq42_e1314_d_b0: f64 = ((eq42_e1312_d_b0 * s.v[841]) + (eq42_e1312 * s.db[841][0]));
        let eq42_e1314_d_b1: f64 = ((eq42_e1312_d_b1 * s.v[841]) + (eq42_e1312 * s.db[841][1]));
        let eq42_e1314_d_b2: f64 = ((eq42_e1312_d_b2 * s.v[841]) + (eq42_e1312 * s.db[841][2]));
        let eq42_e1314_d_b3: f64 = ((eq42_e1312_d_b3 * s.v[841]) + (eq42_e1312 * s.db[841][3]));
        let eq42_e1314_d_b4: f64 = ((eq42_e1312_d_b4 * s.v[841]) + (eq42_e1312 * s.db[841][4]));
        let eq42_e1314_d_b5: f64 = ((eq42_e1312_d_b5 * s.v[841]) + (eq42_e1312 * s.db[841][5]));
        let eq42_e1314_d_b6: f64 = ((eq42_e1312_d_b6 * s.v[841]) + (eq42_e1312 * s.db[841][6]));
        let eq42_e1315_q: f64 = eq42_e1314;
        let eq42_reactive_node_derivatives: [f64; 13] = [eq42_e1314_d_n0, eq42_e1314_d_n1, eq42_e1314_d_n2, eq42_e1314_d_n3, eq42_e1314_d_n4, eq42_e1314_d_n5, eq42_e1314_d_n6, eq42_e1314_d_n7, eq42_e1314_d_n8, eq42_e1314_d_n9, eq42_e1314_d_n10, eq42_e1314_d_n11, eq42_e1314_d_n12];
        let eq42_reactive_branch_derivatives: [f64; 7] = [eq42_e1314_d_b0, eq42_e1314_d_b1, eq42_e1314_d_b2, eq42_e1314_d_b3, eq42_e1314_d_b4, eq42_e1314_d_b5, eq42_e1314_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[9]),
            Some(nodes[7]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e1318: f64 = (s.v[0] * s.v[15]);
        let eq43_e1318_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq43_e1318_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq43_e1318_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq43_e1318_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq43_e1318_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq43_e1318_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq43_e1318_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq43_e1318_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq43_e1318_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq43_e1318_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq43_e1318_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq43_e1318_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq43_e1318_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq43_e1318_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq43_e1318_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq43_e1318_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq43_e1318_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq43_e1318_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq43_e1318_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq43_e1318_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq43_e1320: f64 = (eq43_e1318 * p.p33);
        let eq43_e1320_d_n0: f64 = (eq43_e1318_d_n0 * p.p33);
        let eq43_e1320_d_n1: f64 = (eq43_e1318_d_n1 * p.p33);
        let eq43_e1320_d_n2: f64 = (eq43_e1318_d_n2 * p.p33);
        let eq43_e1320_d_n3: f64 = (eq43_e1318_d_n3 * p.p33);
        let eq43_e1320_d_n4: f64 = (eq43_e1318_d_n4 * p.p33);
        let eq43_e1320_d_n5: f64 = (eq43_e1318_d_n5 * p.p33);
        let eq43_e1320_d_n6: f64 = (eq43_e1318_d_n6 * p.p33);
        let eq43_e1320_d_n7: f64 = (eq43_e1318_d_n7 * p.p33);
        let eq43_e1320_d_n8: f64 = (eq43_e1318_d_n8 * p.p33);
        let eq43_e1320_d_n9: f64 = (eq43_e1318_d_n9 * p.p33);
        let eq43_e1320_d_n10: f64 = (eq43_e1318_d_n10 * p.p33);
        let eq43_e1320_d_n11: f64 = (eq43_e1318_d_n11 * p.p33);
        let eq43_e1320_d_n12: f64 = (eq43_e1318_d_n12 * p.p33);
        let eq43_e1320_d_b0: f64 = (eq43_e1318_d_b0 * p.p33);
        let eq43_e1320_d_b1: f64 = (eq43_e1318_d_b1 * p.p33);
        let eq43_e1320_d_b2: f64 = (eq43_e1318_d_b2 * p.p33);
        let eq43_e1320_d_b3: f64 = (eq43_e1318_d_b3 * p.p33);
        let eq43_e1320_d_b4: f64 = (eq43_e1318_d_b4 * p.p33);
        let eq43_e1320_d_b5: f64 = (eq43_e1318_d_b5 * p.p33);
        let eq43_e1320_d_b6: f64 = (eq43_e1318_d_b6 * p.p33);
        let eq43_e1322: f64 = (eq43_e1320 * s.v[842]);
        let eq43_e1322_d_n0: f64 = ((eq43_e1320_d_n0 * s.v[842]) + (eq43_e1320 * s.dn[842][0]));
        let eq43_e1322_d_n1: f64 = ((eq43_e1320_d_n1 * s.v[842]) + (eq43_e1320 * s.dn[842][1]));
        let eq43_e1322_d_n2: f64 = ((eq43_e1320_d_n2 * s.v[842]) + (eq43_e1320 * s.dn[842][2]));
        let eq43_e1322_d_n3: f64 = ((eq43_e1320_d_n3 * s.v[842]) + (eq43_e1320 * s.dn[842][3]));
        let eq43_e1322_d_n4: f64 = ((eq43_e1320_d_n4 * s.v[842]) + (eq43_e1320 * s.dn[842][4]));
        let eq43_e1322_d_n5: f64 = ((eq43_e1320_d_n5 * s.v[842]) + (eq43_e1320 * s.dn[842][5]));
        let eq43_e1322_d_n6: f64 = ((eq43_e1320_d_n6 * s.v[842]) + (eq43_e1320 * s.dn[842][6]));
        let eq43_e1322_d_n7: f64 = ((eq43_e1320_d_n7 * s.v[842]) + (eq43_e1320 * s.dn[842][7]));
        let eq43_e1322_d_n8: f64 = ((eq43_e1320_d_n8 * s.v[842]) + (eq43_e1320 * s.dn[842][8]));
        let eq43_e1322_d_n9: f64 = ((eq43_e1320_d_n9 * s.v[842]) + (eq43_e1320 * s.dn[842][9]));
        let eq43_e1322_d_n10: f64 = ((eq43_e1320_d_n10 * s.v[842]) + (eq43_e1320 * s.dn[842][10]));
        let eq43_e1322_d_n11: f64 = ((eq43_e1320_d_n11 * s.v[842]) + (eq43_e1320 * s.dn[842][11]));
        let eq43_e1322_d_n12: f64 = ((eq43_e1320_d_n12 * s.v[842]) + (eq43_e1320 * s.dn[842][12]));
        let eq43_e1322_d_b0: f64 = ((eq43_e1320_d_b0 * s.v[842]) + (eq43_e1320 * s.db[842][0]));
        let eq43_e1322_d_b1: f64 = ((eq43_e1320_d_b1 * s.v[842]) + (eq43_e1320 * s.db[842][1]));
        let eq43_e1322_d_b2: f64 = ((eq43_e1320_d_b2 * s.v[842]) + (eq43_e1320 * s.db[842][2]));
        let eq43_e1322_d_b3: f64 = ((eq43_e1320_d_b3 * s.v[842]) + (eq43_e1320 * s.db[842][3]));
        let eq43_e1322_d_b4: f64 = ((eq43_e1320_d_b4 * s.v[842]) + (eq43_e1320 * s.db[842][4]));
        let eq43_e1322_d_b5: f64 = ((eq43_e1320_d_b5 * s.v[842]) + (eq43_e1320 * s.db[842][5]));
        let eq43_e1322_d_b6: f64 = ((eq43_e1320_d_b6 * s.v[842]) + (eq43_e1320 * s.db[842][6]));
        let eq43_e1323_q: f64 = eq43_e1322;
        let eq43_reactive_node_derivatives: [f64; 13] = [eq43_e1322_d_n0, eq43_e1322_d_n1, eq43_e1322_d_n2, eq43_e1322_d_n3, eq43_e1322_d_n4, eq43_e1322_d_n5, eq43_e1322_d_n6, eq43_e1322_d_n7, eq43_e1322_d_n8, eq43_e1322_d_n9, eq43_e1322_d_n10, eq43_e1322_d_n11, eq43_e1322_d_n12];
        let eq43_reactive_branch_derivatives: [f64; 7] = [eq43_e1322_d_b0, eq43_e1322_d_b1, eq43_e1322_d_b2, eq43_e1322_d_b3, eq43_e1322_d_b4, eq43_e1322_d_b5, eq43_e1322_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[7]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let eq44_e1326: f64 = (s.v[0] * s.v[15]);
        let eq44_e1326_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq44_e1326_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq44_e1326_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq44_e1326_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq44_e1326_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq44_e1326_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq44_e1326_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq44_e1326_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq44_e1326_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq44_e1326_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq44_e1326_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq44_e1326_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq44_e1326_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq44_e1326_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq44_e1326_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq44_e1326_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq44_e1326_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq44_e1326_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq44_e1326_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq44_e1326_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq44_e1328: f64 = (eq44_e1326 * p.p33);
        let eq44_e1328_d_n0: f64 = (eq44_e1326_d_n0 * p.p33);
        let eq44_e1328_d_n1: f64 = (eq44_e1326_d_n1 * p.p33);
        let eq44_e1328_d_n2: f64 = (eq44_e1326_d_n2 * p.p33);
        let eq44_e1328_d_n3: f64 = (eq44_e1326_d_n3 * p.p33);
        let eq44_e1328_d_n4: f64 = (eq44_e1326_d_n4 * p.p33);
        let eq44_e1328_d_n5: f64 = (eq44_e1326_d_n5 * p.p33);
        let eq44_e1328_d_n6: f64 = (eq44_e1326_d_n6 * p.p33);
        let eq44_e1328_d_n7: f64 = (eq44_e1326_d_n7 * p.p33);
        let eq44_e1328_d_n8: f64 = (eq44_e1326_d_n8 * p.p33);
        let eq44_e1328_d_n9: f64 = (eq44_e1326_d_n9 * p.p33);
        let eq44_e1328_d_n10: f64 = (eq44_e1326_d_n10 * p.p33);
        let eq44_e1328_d_n11: f64 = (eq44_e1326_d_n11 * p.p33);
        let eq44_e1328_d_n12: f64 = (eq44_e1326_d_n12 * p.p33);
        let eq44_e1328_d_b0: f64 = (eq44_e1326_d_b0 * p.p33);
        let eq44_e1328_d_b1: f64 = (eq44_e1326_d_b1 * p.p33);
        let eq44_e1328_d_b2: f64 = (eq44_e1326_d_b2 * p.p33);
        let eq44_e1328_d_b3: f64 = (eq44_e1326_d_b3 * p.p33);
        let eq44_e1328_d_b4: f64 = (eq44_e1326_d_b4 * p.p33);
        let eq44_e1328_d_b5: f64 = (eq44_e1326_d_b5 * p.p33);
        let eq44_e1328_d_b6: f64 = (eq44_e1326_d_b6 * p.p33);
        let eq44_e1330: f64 = (eq44_e1328 * s.v[843]);
        let eq44_e1330_d_n0: f64 = ((eq44_e1328_d_n0 * s.v[843]) + (eq44_e1328 * s.dn[843][0]));
        let eq44_e1330_d_n1: f64 = ((eq44_e1328_d_n1 * s.v[843]) + (eq44_e1328 * s.dn[843][1]));
        let eq44_e1330_d_n2: f64 = ((eq44_e1328_d_n2 * s.v[843]) + (eq44_e1328 * s.dn[843][2]));
        let eq44_e1330_d_n3: f64 = ((eq44_e1328_d_n3 * s.v[843]) + (eq44_e1328 * s.dn[843][3]));
        let eq44_e1330_d_n4: f64 = ((eq44_e1328_d_n4 * s.v[843]) + (eq44_e1328 * s.dn[843][4]));
        let eq44_e1330_d_n5: f64 = ((eq44_e1328_d_n5 * s.v[843]) + (eq44_e1328 * s.dn[843][5]));
        let eq44_e1330_d_n6: f64 = ((eq44_e1328_d_n6 * s.v[843]) + (eq44_e1328 * s.dn[843][6]));
        let eq44_e1330_d_n7: f64 = ((eq44_e1328_d_n7 * s.v[843]) + (eq44_e1328 * s.dn[843][7]));
        let eq44_e1330_d_n8: f64 = ((eq44_e1328_d_n8 * s.v[843]) + (eq44_e1328 * s.dn[843][8]));
        let eq44_e1330_d_n9: f64 = ((eq44_e1328_d_n9 * s.v[843]) + (eq44_e1328 * s.dn[843][9]));
        let eq44_e1330_d_n10: f64 = ((eq44_e1328_d_n10 * s.v[843]) + (eq44_e1328 * s.dn[843][10]));
        let eq44_e1330_d_n11: f64 = ((eq44_e1328_d_n11 * s.v[843]) + (eq44_e1328 * s.dn[843][11]));
        let eq44_e1330_d_n12: f64 = ((eq44_e1328_d_n12 * s.v[843]) + (eq44_e1328 * s.dn[843][12]));
        let eq44_e1330_d_b0: f64 = ((eq44_e1328_d_b0 * s.v[843]) + (eq44_e1328 * s.db[843][0]));
        let eq44_e1330_d_b1: f64 = ((eq44_e1328_d_b1 * s.v[843]) + (eq44_e1328 * s.db[843][1]));
        let eq44_e1330_d_b2: f64 = ((eq44_e1328_d_b2 * s.v[843]) + (eq44_e1328 * s.db[843][2]));
        let eq44_e1330_d_b3: f64 = ((eq44_e1328_d_b3 * s.v[843]) + (eq44_e1328 * s.db[843][3]));
        let eq44_e1330_d_b4: f64 = ((eq44_e1328_d_b4 * s.v[843]) + (eq44_e1328 * s.db[843][4]));
        let eq44_e1330_d_b5: f64 = ((eq44_e1328_d_b5 * s.v[843]) + (eq44_e1328 * s.db[843][5]));
        let eq44_e1330_d_b6: f64 = ((eq44_e1328_d_b6 * s.v[843]) + (eq44_e1328 * s.db[843][6]));
        let eq44_e1331_q: f64 = eq44_e1330;
        let eq44_reactive_node_derivatives: [f64; 13] = [eq44_e1330_d_n0, eq44_e1330_d_n1, eq44_e1330_d_n2, eq44_e1330_d_n3, eq44_e1330_d_n4, eq44_e1330_d_n5, eq44_e1330_d_n6, eq44_e1330_d_n7, eq44_e1330_d_n8, eq44_e1330_d_n9, eq44_e1330_d_n10, eq44_e1330_d_n11, eq44_e1330_d_n12];
        let eq44_reactive_branch_derivatives: [f64; 7] = [eq44_e1330_d_b0, eq44_e1330_d_b1, eq44_e1330_d_b2, eq44_e1330_d_b3, eq44_e1330_d_b4, eq44_e1330_d_b5, eq44_e1330_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq44_reactive_node_derivatives,
            branches,
            &eq44_reactive_branch_derivatives,
            multiplicity,
        );
        let eq45_e1334: f64 = (s.v[0] * s.v[15]);
        let eq45_e1334_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq45_e1334_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq45_e1334_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq45_e1334_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq45_e1334_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq45_e1334_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq45_e1334_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq45_e1334_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq45_e1334_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq45_e1334_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq45_e1334_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq45_e1334_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq45_e1334_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq45_e1334_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq45_e1334_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq45_e1334_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq45_e1334_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq45_e1334_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq45_e1334_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq45_e1334_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq45_e1336: f64 = (eq45_e1334 * p.p33);
        let eq45_e1336_d_n0: f64 = (eq45_e1334_d_n0 * p.p33);
        let eq45_e1336_d_n1: f64 = (eq45_e1334_d_n1 * p.p33);
        let eq45_e1336_d_n2: f64 = (eq45_e1334_d_n2 * p.p33);
        let eq45_e1336_d_n3: f64 = (eq45_e1334_d_n3 * p.p33);
        let eq45_e1336_d_n4: f64 = (eq45_e1334_d_n4 * p.p33);
        let eq45_e1336_d_n5: f64 = (eq45_e1334_d_n5 * p.p33);
        let eq45_e1336_d_n6: f64 = (eq45_e1334_d_n6 * p.p33);
        let eq45_e1336_d_n7: f64 = (eq45_e1334_d_n7 * p.p33);
        let eq45_e1336_d_n8: f64 = (eq45_e1334_d_n8 * p.p33);
        let eq45_e1336_d_n9: f64 = (eq45_e1334_d_n9 * p.p33);
        let eq45_e1336_d_n10: f64 = (eq45_e1334_d_n10 * p.p33);
        let eq45_e1336_d_n11: f64 = (eq45_e1334_d_n11 * p.p33);
        let eq45_e1336_d_n12: f64 = (eq45_e1334_d_n12 * p.p33);
        let eq45_e1336_d_b0: f64 = (eq45_e1334_d_b0 * p.p33);
        let eq45_e1336_d_b1: f64 = (eq45_e1334_d_b1 * p.p33);
        let eq45_e1336_d_b2: f64 = (eq45_e1334_d_b2 * p.p33);
        let eq45_e1336_d_b3: f64 = (eq45_e1334_d_b3 * p.p33);
        let eq45_e1336_d_b4: f64 = (eq45_e1334_d_b4 * p.p33);
        let eq45_e1336_d_b5: f64 = (eq45_e1334_d_b5 * p.p33);
        let eq45_e1336_d_b6: f64 = (eq45_e1334_d_b6 * p.p33);
        let eq45_e1338: f64 = (eq45_e1336 * s.v[844]);
        let eq45_e1338_d_n0: f64 = ((eq45_e1336_d_n0 * s.v[844]) + (eq45_e1336 * s.dn[844][0]));
        let eq45_e1338_d_n1: f64 = ((eq45_e1336_d_n1 * s.v[844]) + (eq45_e1336 * s.dn[844][1]));
        let eq45_e1338_d_n2: f64 = ((eq45_e1336_d_n2 * s.v[844]) + (eq45_e1336 * s.dn[844][2]));
        let eq45_e1338_d_n3: f64 = ((eq45_e1336_d_n3 * s.v[844]) + (eq45_e1336 * s.dn[844][3]));
        let eq45_e1338_d_n4: f64 = ((eq45_e1336_d_n4 * s.v[844]) + (eq45_e1336 * s.dn[844][4]));
        let eq45_e1338_d_n5: f64 = ((eq45_e1336_d_n5 * s.v[844]) + (eq45_e1336 * s.dn[844][5]));
        let eq45_e1338_d_n6: f64 = ((eq45_e1336_d_n6 * s.v[844]) + (eq45_e1336 * s.dn[844][6]));
        let eq45_e1338_d_n7: f64 = ((eq45_e1336_d_n7 * s.v[844]) + (eq45_e1336 * s.dn[844][7]));
        let eq45_e1338_d_n8: f64 = ((eq45_e1336_d_n8 * s.v[844]) + (eq45_e1336 * s.dn[844][8]));
        let eq45_e1338_d_n9: f64 = ((eq45_e1336_d_n9 * s.v[844]) + (eq45_e1336 * s.dn[844][9]));
        let eq45_e1338_d_n10: f64 = ((eq45_e1336_d_n10 * s.v[844]) + (eq45_e1336 * s.dn[844][10]));
        let eq45_e1338_d_n11: f64 = ((eq45_e1336_d_n11 * s.v[844]) + (eq45_e1336 * s.dn[844][11]));
        let eq45_e1338_d_n12: f64 = ((eq45_e1336_d_n12 * s.v[844]) + (eq45_e1336 * s.dn[844][12]));
        let eq45_e1338_d_b0: f64 = ((eq45_e1336_d_b0 * s.v[844]) + (eq45_e1336 * s.db[844][0]));
        let eq45_e1338_d_b1: f64 = ((eq45_e1336_d_b1 * s.v[844]) + (eq45_e1336 * s.db[844][1]));
        let eq45_e1338_d_b2: f64 = ((eq45_e1336_d_b2 * s.v[844]) + (eq45_e1336 * s.db[844][2]));
        let eq45_e1338_d_b3: f64 = ((eq45_e1336_d_b3 * s.v[844]) + (eq45_e1336 * s.db[844][3]));
        let eq45_e1338_d_b4: f64 = ((eq45_e1336_d_b4 * s.v[844]) + (eq45_e1336 * s.db[844][4]));
        let eq45_e1338_d_b5: f64 = ((eq45_e1336_d_b5 * s.v[844]) + (eq45_e1336 * s.db[844][5]));
        let eq45_e1338_d_b6: f64 = ((eq45_e1336_d_b6 * s.v[844]) + (eq45_e1336 * s.db[844][6]));
        let eq45_e1339_q: f64 = eq45_e1338;
        let eq45_reactive_node_derivatives: [f64; 13] = [eq45_e1338_d_n0, eq45_e1338_d_n1, eq45_e1338_d_n2, eq45_e1338_d_n3, eq45_e1338_d_n4, eq45_e1338_d_n5, eq45_e1338_d_n6, eq45_e1338_d_n7, eq45_e1338_d_n8, eq45_e1338_d_n9, eq45_e1338_d_n10, eq45_e1338_d_n11, eq45_e1338_d_n12];
        let eq45_reactive_branch_derivatives: [f64; 7] = [eq45_e1338_d_b0, eq45_e1338_d_b1, eq45_e1338_d_b2, eq45_e1338_d_b3, eq45_e1338_d_b4, eq45_e1338_d_b5, eq45_e1338_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let eq46_e1342: f64 = (s.v[0] * s.v[15]);
        let eq46_e1342_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq46_e1342_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq46_e1342_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq46_e1342_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq46_e1342_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq46_e1342_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq46_e1342_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq46_e1342_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq46_e1342_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq46_e1342_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq46_e1342_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq46_e1342_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq46_e1342_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq46_e1342_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq46_e1342_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq46_e1342_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq46_e1342_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq46_e1342_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq46_e1342_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq46_e1342_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq46_e1344: f64 = (eq46_e1342 * p.p33);
        let eq46_e1344_d_n0: f64 = (eq46_e1342_d_n0 * p.p33);
        let eq46_e1344_d_n1: f64 = (eq46_e1342_d_n1 * p.p33);
        let eq46_e1344_d_n2: f64 = (eq46_e1342_d_n2 * p.p33);
        let eq46_e1344_d_n3: f64 = (eq46_e1342_d_n3 * p.p33);
        let eq46_e1344_d_n4: f64 = (eq46_e1342_d_n4 * p.p33);
        let eq46_e1344_d_n5: f64 = (eq46_e1342_d_n5 * p.p33);
        let eq46_e1344_d_n6: f64 = (eq46_e1342_d_n6 * p.p33);
        let eq46_e1344_d_n7: f64 = (eq46_e1342_d_n7 * p.p33);
        let eq46_e1344_d_n8: f64 = (eq46_e1342_d_n8 * p.p33);
        let eq46_e1344_d_n9: f64 = (eq46_e1342_d_n9 * p.p33);
        let eq46_e1344_d_n10: f64 = (eq46_e1342_d_n10 * p.p33);
        let eq46_e1344_d_n11: f64 = (eq46_e1342_d_n11 * p.p33);
        let eq46_e1344_d_n12: f64 = (eq46_e1342_d_n12 * p.p33);
        let eq46_e1344_d_b0: f64 = (eq46_e1342_d_b0 * p.p33);
        let eq46_e1344_d_b1: f64 = (eq46_e1342_d_b1 * p.p33);
        let eq46_e1344_d_b2: f64 = (eq46_e1342_d_b2 * p.p33);
        let eq46_e1344_d_b3: f64 = (eq46_e1342_d_b3 * p.p33);
        let eq46_e1344_d_b4: f64 = (eq46_e1342_d_b4 * p.p33);
        let eq46_e1344_d_b5: f64 = (eq46_e1342_d_b5 * p.p33);
        let eq46_e1344_d_b6: f64 = (eq46_e1342_d_b6 * p.p33);
        let eq46_e1346: f64 = (eq46_e1344 * s.v[845]);
        let eq46_e1346_d_n0: f64 = ((eq46_e1344_d_n0 * s.v[845]) + (eq46_e1344 * s.dn[845][0]));
        let eq46_e1346_d_n1: f64 = ((eq46_e1344_d_n1 * s.v[845]) + (eq46_e1344 * s.dn[845][1]));
        let eq46_e1346_d_n2: f64 = ((eq46_e1344_d_n2 * s.v[845]) + (eq46_e1344 * s.dn[845][2]));
        let eq46_e1346_d_n3: f64 = ((eq46_e1344_d_n3 * s.v[845]) + (eq46_e1344 * s.dn[845][3]));
        let eq46_e1346_d_n4: f64 = ((eq46_e1344_d_n4 * s.v[845]) + (eq46_e1344 * s.dn[845][4]));
        let eq46_e1346_d_n5: f64 = ((eq46_e1344_d_n5 * s.v[845]) + (eq46_e1344 * s.dn[845][5]));
        let eq46_e1346_d_n6: f64 = ((eq46_e1344_d_n6 * s.v[845]) + (eq46_e1344 * s.dn[845][6]));
        let eq46_e1346_d_n7: f64 = ((eq46_e1344_d_n7 * s.v[845]) + (eq46_e1344 * s.dn[845][7]));
        let eq46_e1346_d_n8: f64 = ((eq46_e1344_d_n8 * s.v[845]) + (eq46_e1344 * s.dn[845][8]));
        let eq46_e1346_d_n9: f64 = ((eq46_e1344_d_n9 * s.v[845]) + (eq46_e1344 * s.dn[845][9]));
        let eq46_e1346_d_n10: f64 = ((eq46_e1344_d_n10 * s.v[845]) + (eq46_e1344 * s.dn[845][10]));
        let eq46_e1346_d_n11: f64 = ((eq46_e1344_d_n11 * s.v[845]) + (eq46_e1344 * s.dn[845][11]));
        let eq46_e1346_d_n12: f64 = ((eq46_e1344_d_n12 * s.v[845]) + (eq46_e1344 * s.dn[845][12]));
        let eq46_e1346_d_b0: f64 = ((eq46_e1344_d_b0 * s.v[845]) + (eq46_e1344 * s.db[845][0]));
        let eq46_e1346_d_b1: f64 = ((eq46_e1344_d_b1 * s.v[845]) + (eq46_e1344 * s.db[845][1]));
        let eq46_e1346_d_b2: f64 = ((eq46_e1344_d_b2 * s.v[845]) + (eq46_e1344 * s.db[845][2]));
        let eq46_e1346_d_b3: f64 = ((eq46_e1344_d_b3 * s.v[845]) + (eq46_e1344 * s.db[845][3]));
        let eq46_e1346_d_b4: f64 = ((eq46_e1344_d_b4 * s.v[845]) + (eq46_e1344 * s.db[845][4]));
        let eq46_e1346_d_b5: f64 = ((eq46_e1344_d_b5 * s.v[845]) + (eq46_e1344 * s.db[845][5]));
        let eq46_e1346_d_b6: f64 = ((eq46_e1344_d_b6 * s.v[845]) + (eq46_e1344 * s.db[845][6]));
        let eq46_e1347_q: f64 = eq46_e1346;
        let eq46_reactive_node_derivatives: [f64; 13] = [eq46_e1346_d_n0, eq46_e1346_d_n1, eq46_e1346_d_n2, eq46_e1346_d_n3, eq46_e1346_d_n4, eq46_e1346_d_n5, eq46_e1346_d_n6, eq46_e1346_d_n7, eq46_e1346_d_n8, eq46_e1346_d_n9, eq46_e1346_d_n10, eq46_e1346_d_n11, eq46_e1346_d_n12];
        let eq46_reactive_branch_derivatives: [f64; 7] = [eq46_e1346_d_b0, eq46_e1346_d_b1, eq46_e1346_d_b2, eq46_e1346_d_b3, eq46_e1346_d_b4, eq46_e1346_d_b5, eq46_e1346_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[9]),
            nodes,
            &eq46_reactive_node_derivatives,
            branches,
            &eq46_reactive_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_reactive_equations_block_1(
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let nv5 = ctx.node_voltage(nodes[5]);
        let eq47_e1350: f64 = (s.v[0] * s.v[15]);
        let eq47_e1350_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq47_e1350_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq47_e1350_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq47_e1350_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq47_e1350_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq47_e1350_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq47_e1350_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq47_e1350_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq47_e1350_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq47_e1350_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq47_e1350_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq47_e1350_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq47_e1350_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq47_e1350_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq47_e1350_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq47_e1350_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq47_e1350_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq47_e1350_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq47_e1350_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq47_e1350_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq47_e1352: f64 = (eq47_e1350 * p.p33);
        let eq47_e1352_d_n0: f64 = (eq47_e1350_d_n0 * p.p33);
        let eq47_e1352_d_n1: f64 = (eq47_e1350_d_n1 * p.p33);
        let eq47_e1352_d_n2: f64 = (eq47_e1350_d_n2 * p.p33);
        let eq47_e1352_d_n3: f64 = (eq47_e1350_d_n3 * p.p33);
        let eq47_e1352_d_n4: f64 = (eq47_e1350_d_n4 * p.p33);
        let eq47_e1352_d_n5: f64 = (eq47_e1350_d_n5 * p.p33);
        let eq47_e1352_d_n6: f64 = (eq47_e1350_d_n6 * p.p33);
        let eq47_e1352_d_n7: f64 = (eq47_e1350_d_n7 * p.p33);
        let eq47_e1352_d_n8: f64 = (eq47_e1350_d_n8 * p.p33);
        let eq47_e1352_d_n9: f64 = (eq47_e1350_d_n9 * p.p33);
        let eq47_e1352_d_n10: f64 = (eq47_e1350_d_n10 * p.p33);
        let eq47_e1352_d_n11: f64 = (eq47_e1350_d_n11 * p.p33);
        let eq47_e1352_d_n12: f64 = (eq47_e1350_d_n12 * p.p33);
        let eq47_e1352_d_b0: f64 = (eq47_e1350_d_b0 * p.p33);
        let eq47_e1352_d_b1: f64 = (eq47_e1350_d_b1 * p.p33);
        let eq47_e1352_d_b2: f64 = (eq47_e1350_d_b2 * p.p33);
        let eq47_e1352_d_b3: f64 = (eq47_e1350_d_b3 * p.p33);
        let eq47_e1352_d_b4: f64 = (eq47_e1350_d_b4 * p.p33);
        let eq47_e1352_d_b5: f64 = (eq47_e1350_d_b5 * p.p33);
        let eq47_e1352_d_b6: f64 = (eq47_e1350_d_b6 * p.p33);
        let eq47_e1354: f64 = (eq47_e1352 * s.v[846]);
        let eq47_e1354_d_n0: f64 = ((eq47_e1352_d_n0 * s.v[846]) + (eq47_e1352 * s.dn[846][0]));
        let eq47_e1354_d_n1: f64 = ((eq47_e1352_d_n1 * s.v[846]) + (eq47_e1352 * s.dn[846][1]));
        let eq47_e1354_d_n2: f64 = ((eq47_e1352_d_n2 * s.v[846]) + (eq47_e1352 * s.dn[846][2]));
        let eq47_e1354_d_n3: f64 = ((eq47_e1352_d_n3 * s.v[846]) + (eq47_e1352 * s.dn[846][3]));
        let eq47_e1354_d_n4: f64 = ((eq47_e1352_d_n4 * s.v[846]) + (eq47_e1352 * s.dn[846][4]));
        let eq47_e1354_d_n5: f64 = ((eq47_e1352_d_n5 * s.v[846]) + (eq47_e1352 * s.dn[846][5]));
        let eq47_e1354_d_n6: f64 = ((eq47_e1352_d_n6 * s.v[846]) + (eq47_e1352 * s.dn[846][6]));
        let eq47_e1354_d_n7: f64 = ((eq47_e1352_d_n7 * s.v[846]) + (eq47_e1352 * s.dn[846][7]));
        let eq47_e1354_d_n8: f64 = ((eq47_e1352_d_n8 * s.v[846]) + (eq47_e1352 * s.dn[846][8]));
        let eq47_e1354_d_n9: f64 = ((eq47_e1352_d_n9 * s.v[846]) + (eq47_e1352 * s.dn[846][9]));
        let eq47_e1354_d_n10: f64 = ((eq47_e1352_d_n10 * s.v[846]) + (eq47_e1352 * s.dn[846][10]));
        let eq47_e1354_d_n11: f64 = ((eq47_e1352_d_n11 * s.v[846]) + (eq47_e1352 * s.dn[846][11]));
        let eq47_e1354_d_n12: f64 = ((eq47_e1352_d_n12 * s.v[846]) + (eq47_e1352 * s.dn[846][12]));
        let eq47_e1354_d_b0: f64 = ((eq47_e1352_d_b0 * s.v[846]) + (eq47_e1352 * s.db[846][0]));
        let eq47_e1354_d_b1: f64 = ((eq47_e1352_d_b1 * s.v[846]) + (eq47_e1352 * s.db[846][1]));
        let eq47_e1354_d_b2: f64 = ((eq47_e1352_d_b2 * s.v[846]) + (eq47_e1352 * s.db[846][2]));
        let eq47_e1354_d_b3: f64 = ((eq47_e1352_d_b3 * s.v[846]) + (eq47_e1352 * s.db[846][3]));
        let eq47_e1354_d_b4: f64 = ((eq47_e1352_d_b4 * s.v[846]) + (eq47_e1352 * s.db[846][4]));
        let eq47_e1354_d_b5: f64 = ((eq47_e1352_d_b5 * s.v[846]) + (eq47_e1352 * s.db[846][5]));
        let eq47_e1354_d_b6: f64 = ((eq47_e1352_d_b6 * s.v[846]) + (eq47_e1352 * s.db[846][6]));
        let eq47_e1355_q: f64 = eq47_e1354;
        let eq47_reactive_node_derivatives: [f64; 13] = [eq47_e1354_d_n0, eq47_e1354_d_n1, eq47_e1354_d_n2, eq47_e1354_d_n3, eq47_e1354_d_n4, eq47_e1354_d_n5, eq47_e1354_d_n6, eq47_e1354_d_n7, eq47_e1354_d_n8, eq47_e1354_d_n9, eq47_e1354_d_n10, eq47_e1354_d_n11, eq47_e1354_d_n12];
        let eq47_reactive_branch_derivatives: [f64; 7] = [eq47_e1354_d_b0, eq47_e1354_d_b1, eq47_e1354_d_b2, eq47_e1354_d_b3, eq47_e1354_d_b4, eq47_e1354_d_b5, eq47_e1354_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq47_reactive_node_derivatives,
            branches,
            &eq47_reactive_branch_derivatives,
            multiplicity,
        );
        let eq48_e1358: f64 = (s.v[0] * s.v[15]);
        let eq48_e1358_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq48_e1358_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq48_e1358_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq48_e1358_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq48_e1358_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq48_e1358_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq48_e1358_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq48_e1358_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq48_e1358_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq48_e1358_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq48_e1358_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq48_e1358_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq48_e1358_d_n12: f64 = ((s.dn[0][12] * s.v[15]) + (s.v[0] * s.dn[15][12]));
        let eq48_e1358_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq48_e1358_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq48_e1358_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq48_e1358_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq48_e1358_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq48_e1358_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq48_e1358_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq48_e1360: f64 = (eq48_e1358 * p.p33);
        let eq48_e1360_d_n0: f64 = (eq48_e1358_d_n0 * p.p33);
        let eq48_e1360_d_n1: f64 = (eq48_e1358_d_n1 * p.p33);
        let eq48_e1360_d_n2: f64 = (eq48_e1358_d_n2 * p.p33);
        let eq48_e1360_d_n3: f64 = (eq48_e1358_d_n3 * p.p33);
        let eq48_e1360_d_n4: f64 = (eq48_e1358_d_n4 * p.p33);
        let eq48_e1360_d_n5: f64 = (eq48_e1358_d_n5 * p.p33);
        let eq48_e1360_d_n6: f64 = (eq48_e1358_d_n6 * p.p33);
        let eq48_e1360_d_n7: f64 = (eq48_e1358_d_n7 * p.p33);
        let eq48_e1360_d_n8: f64 = (eq48_e1358_d_n8 * p.p33);
        let eq48_e1360_d_n9: f64 = (eq48_e1358_d_n9 * p.p33);
        let eq48_e1360_d_n10: f64 = (eq48_e1358_d_n10 * p.p33);
        let eq48_e1360_d_n11: f64 = (eq48_e1358_d_n11 * p.p33);
        let eq48_e1360_d_n12: f64 = (eq48_e1358_d_n12 * p.p33);
        let eq48_e1360_d_b0: f64 = (eq48_e1358_d_b0 * p.p33);
        let eq48_e1360_d_b1: f64 = (eq48_e1358_d_b1 * p.p33);
        let eq48_e1360_d_b2: f64 = (eq48_e1358_d_b2 * p.p33);
        let eq48_e1360_d_b3: f64 = (eq48_e1358_d_b3 * p.p33);
        let eq48_e1360_d_b4: f64 = (eq48_e1358_d_b4 * p.p33);
        let eq48_e1360_d_b5: f64 = (eq48_e1358_d_b5 * p.p33);
        let eq48_e1360_d_b6: f64 = (eq48_e1358_d_b6 * p.p33);
        let eq48_e1362: f64 = (eq48_e1360 * s.v[847]);
        let eq48_e1362_d_n0: f64 = ((eq48_e1360_d_n0 * s.v[847]) + (eq48_e1360 * s.dn[847][0]));
        let eq48_e1362_d_n1: f64 = ((eq48_e1360_d_n1 * s.v[847]) + (eq48_e1360 * s.dn[847][1]));
        let eq48_e1362_d_n2: f64 = ((eq48_e1360_d_n2 * s.v[847]) + (eq48_e1360 * s.dn[847][2]));
        let eq48_e1362_d_n3: f64 = ((eq48_e1360_d_n3 * s.v[847]) + (eq48_e1360 * s.dn[847][3]));
        let eq48_e1362_d_n4: f64 = ((eq48_e1360_d_n4 * s.v[847]) + (eq48_e1360 * s.dn[847][4]));
        let eq48_e1362_d_n5: f64 = ((eq48_e1360_d_n5 * s.v[847]) + (eq48_e1360 * s.dn[847][5]));
        let eq48_e1362_d_n6: f64 = ((eq48_e1360_d_n6 * s.v[847]) + (eq48_e1360 * s.dn[847][6]));
        let eq48_e1362_d_n7: f64 = ((eq48_e1360_d_n7 * s.v[847]) + (eq48_e1360 * s.dn[847][7]));
        let eq48_e1362_d_n8: f64 = ((eq48_e1360_d_n8 * s.v[847]) + (eq48_e1360 * s.dn[847][8]));
        let eq48_e1362_d_n9: f64 = ((eq48_e1360_d_n9 * s.v[847]) + (eq48_e1360 * s.dn[847][9]));
        let eq48_e1362_d_n10: f64 = ((eq48_e1360_d_n10 * s.v[847]) + (eq48_e1360 * s.dn[847][10]));
        let eq48_e1362_d_n11: f64 = ((eq48_e1360_d_n11 * s.v[847]) + (eq48_e1360 * s.dn[847][11]));
        let eq48_e1362_d_n12: f64 = ((eq48_e1360_d_n12 * s.v[847]) + (eq48_e1360 * s.dn[847][12]));
        let eq48_e1362_d_b0: f64 = ((eq48_e1360_d_b0 * s.v[847]) + (eq48_e1360 * s.db[847][0]));
        let eq48_e1362_d_b1: f64 = ((eq48_e1360_d_b1 * s.v[847]) + (eq48_e1360 * s.db[847][1]));
        let eq48_e1362_d_b2: f64 = ((eq48_e1360_d_b2 * s.v[847]) + (eq48_e1360 * s.db[847][2]));
        let eq48_e1362_d_b3: f64 = ((eq48_e1360_d_b3 * s.v[847]) + (eq48_e1360 * s.db[847][3]));
        let eq48_e1362_d_b4: f64 = ((eq48_e1360_d_b4 * s.v[847]) + (eq48_e1360 * s.db[847][4]));
        let eq48_e1362_d_b5: f64 = ((eq48_e1360_d_b5 * s.v[847]) + (eq48_e1360 * s.db[847][5]));
        let eq48_e1362_d_b6: f64 = ((eq48_e1360_d_b6 * s.v[847]) + (eq48_e1360 * s.db[847][6]));
        let eq48_e1363_q: f64 = eq48_e1362;
        let eq48_reactive_node_derivatives: [f64; 13] = [eq48_e1362_d_n0, eq48_e1362_d_n1, eq48_e1362_d_n2, eq48_e1362_d_n3, eq48_e1362_d_n4, eq48_e1362_d_n5, eq48_e1362_d_n6, eq48_e1362_d_n7, eq48_e1362_d_n8, eq48_e1362_d_n9, eq48_e1362_d_n10, eq48_e1362_d_n11, eq48_e1362_d_n12];
        let eq48_reactive_branch_derivatives: [f64; 7] = [eq48_e1362_d_b0, eq48_e1362_d_b1, eq48_e1362_d_b2, eq48_e1362_d_b3, eq48_e1362_d_b4, eq48_e1362_d_b5, eq48_e1362_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[12]),
            Some(nodes[8]),
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let eq51_e1374: f64 = (s.v[849] * (nv5 - 0.0));
        let eq51_e1374_d_n0: f64 = (s.dn[849][0] * (nv5 - 0.0));
        let eq51_e1374_d_n1: f64 = (s.dn[849][1] * (nv5 - 0.0));
        let eq51_e1374_d_n2: f64 = (s.dn[849][2] * (nv5 - 0.0));
        let eq51_e1374_d_n3: f64 = (s.dn[849][3] * (nv5 - 0.0));
        let eq51_e1374_d_n4: f64 = (s.dn[849][4] * (nv5 - 0.0));
        let eq51_e1374_d_n5: f64 = ((s.dn[849][5] * (nv5 - 0.0)) + s.v[849]);
        let eq51_e1374_d_n6: f64 = (s.dn[849][6] * (nv5 - 0.0));
        let eq51_e1374_d_n7: f64 = (s.dn[849][7] * (nv5 - 0.0));
        let eq51_e1374_d_n8: f64 = (s.dn[849][8] * (nv5 - 0.0));
        let eq51_e1374_d_n9: f64 = (s.dn[849][9] * (nv5 - 0.0));
        let eq51_e1374_d_n10: f64 = (s.dn[849][10] * (nv5 - 0.0));
        let eq51_e1374_d_n11: f64 = (s.dn[849][11] * (nv5 - 0.0));
        let eq51_e1374_d_n12: f64 = (s.dn[849][12] * (nv5 - 0.0));
        let eq51_e1374_d_b0: f64 = (s.db[849][0] * (nv5 - 0.0));
        let eq51_e1374_d_b1: f64 = (s.db[849][1] * (nv5 - 0.0));
        let eq51_e1374_d_b2: f64 = (s.db[849][2] * (nv5 - 0.0));
        let eq51_e1374_d_b3: f64 = (s.db[849][3] * (nv5 - 0.0));
        let eq51_e1374_d_b4: f64 = (s.db[849][4] * (nv5 - 0.0));
        let eq51_e1374_d_b5: f64 = (s.db[849][5] * (nv5 - 0.0));
        let eq51_e1374_d_b6: f64 = (s.db[849][6] * (nv5 - 0.0));
        let eq51_e1375_q: f64 = eq51_e1374;
        let eq51_reactive_node_derivatives: [f64; 13] = [eq51_e1374_d_n0, eq51_e1374_d_n1, eq51_e1374_d_n2, eq51_e1374_d_n3, eq51_e1374_d_n4, eq51_e1374_d_n5, eq51_e1374_d_n6, eq51_e1374_d_n7, eq51_e1374_d_n8, eq51_e1374_d_n9, eq51_e1374_d_n10, eq51_e1374_d_n11, eq51_e1374_d_n12];
        let eq51_reactive_branch_derivatives: [f64; 7] = [eq51_e1374_d_b0, eq51_e1374_d_b1, eq51_e1374_d_b2, eq51_e1374_d_b3, eq51_e1374_d_b4, eq51_e1374_d_b5, eq51_e1374_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            None,
            nodes,
            &eq51_reactive_node_derivatives,
            branches,
            &eq51_reactive_branch_derivatives,
            multiplicity,
        );
        let eq52_e1378: f64 = (s.v[15] * p.p32);
        let eq52_e1378_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq52_e1378_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq52_e1378_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq52_e1378_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq52_e1378_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq52_e1378_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq52_e1378_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq52_e1378_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq52_e1378_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq52_e1378_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq52_e1378_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq52_e1378_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq52_e1378_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq52_e1378_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq52_e1378_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq52_e1378_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq52_e1378_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq52_e1378_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq52_e1378_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq52_e1378_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq52_e1379: f64 = (eq52_e1378).sqrt();
        let eq52_e1379_d_n0: f64 = (eq52_e1378_d_n0 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n1: f64 = (eq52_e1378_d_n1 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n2: f64 = (eq52_e1378_d_n2 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n3: f64 = (eq52_e1378_d_n3 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n4: f64 = (eq52_e1378_d_n4 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n5: f64 = (eq52_e1378_d_n5 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n6: f64 = (eq52_e1378_d_n6 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n7: f64 = (eq52_e1378_d_n7 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n8: f64 = (eq52_e1378_d_n8 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n9: f64 = (eq52_e1378_d_n9 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n10: f64 = (eq52_e1378_d_n10 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n11: f64 = (eq52_e1378_d_n11 / (2.0 * eq52_e1379));
        let eq52_e1379_d_n12: f64 = (eq52_e1378_d_n12 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b0: f64 = (eq52_e1378_d_b0 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b1: f64 = (eq52_e1378_d_b1 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b2: f64 = (eq52_e1378_d_b2 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b3: f64 = (eq52_e1378_d_b3 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b4: f64 = (eq52_e1378_d_b4 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b5: f64 = (eq52_e1378_d_b5 / (2.0 * eq52_e1379));
        let eq52_e1379_d_b6: f64 = (eq52_e1378_d_b6 / (2.0 * eq52_e1379));
        let eq52_e1381: f64 = (eq52_e1379 * 0.5);
        let eq52_e1381_d_n0: f64 = (eq52_e1379_d_n0 * 0.5);
        let eq52_e1381_d_n1: f64 = (eq52_e1379_d_n1 * 0.5);
        let eq52_e1381_d_n2: f64 = (eq52_e1379_d_n2 * 0.5);
        let eq52_e1381_d_n3: f64 = (eq52_e1379_d_n3 * 0.5);
        let eq52_e1381_d_n4: f64 = (eq52_e1379_d_n4 * 0.5);
        let eq52_e1381_d_n5: f64 = (eq52_e1379_d_n5 * 0.5);
        let eq52_e1381_d_n6: f64 = (eq52_e1379_d_n6 * 0.5);
        let eq52_e1381_d_n7: f64 = (eq52_e1379_d_n7 * 0.5);
        let eq52_e1381_d_n8: f64 = (eq52_e1379_d_n8 * 0.5);
        let eq52_e1381_d_n9: f64 = (eq52_e1379_d_n9 * 0.5);
        let eq52_e1381_d_n10: f64 = (eq52_e1379_d_n10 * 0.5);
        let eq52_e1381_d_n11: f64 = (eq52_e1379_d_n11 * 0.5);
        let eq52_e1381_d_n12: f64 = (eq52_e1379_d_n12 * 0.5);
        let eq52_e1381_d_b0: f64 = (eq52_e1379_d_b0 * 0.5);
        let eq52_e1381_d_b1: f64 = (eq52_e1379_d_b1 * 0.5);
        let eq52_e1381_d_b2: f64 = (eq52_e1379_d_b2 * 0.5);
        let eq52_e1381_d_b3: f64 = (eq52_e1379_d_b3 * 0.5);
        let eq52_e1381_d_b4: f64 = (eq52_e1379_d_b4 * 0.5);
        let eq52_e1381_d_b5: f64 = (eq52_e1379_d_b5 * 0.5);
        let eq52_e1381_d_b6: f64 = (eq52_e1379_d_b6 * 0.5);
        let eq52_e1383: f64 = (eq52_e1381 * s.v[849]);
        let eq52_e1383_d_n0: f64 = ((eq52_e1381_d_n0 * s.v[849]) + (eq52_e1381 * s.dn[849][0]));
        let eq52_e1383_d_n1: f64 = ((eq52_e1381_d_n1 * s.v[849]) + (eq52_e1381 * s.dn[849][1]));
        let eq52_e1383_d_n2: f64 = ((eq52_e1381_d_n2 * s.v[849]) + (eq52_e1381 * s.dn[849][2]));
        let eq52_e1383_d_n3: f64 = ((eq52_e1381_d_n3 * s.v[849]) + (eq52_e1381 * s.dn[849][3]));
        let eq52_e1383_d_n4: f64 = ((eq52_e1381_d_n4 * s.v[849]) + (eq52_e1381 * s.dn[849][4]));
        let eq52_e1383_d_n5: f64 = ((eq52_e1381_d_n5 * s.v[849]) + (eq52_e1381 * s.dn[849][5]));
        let eq52_e1383_d_n6: f64 = ((eq52_e1381_d_n6 * s.v[849]) + (eq52_e1381 * s.dn[849][6]));
        let eq52_e1383_d_n7: f64 = ((eq52_e1381_d_n7 * s.v[849]) + (eq52_e1381 * s.dn[849][7]));
        let eq52_e1383_d_n8: f64 = ((eq52_e1381_d_n8 * s.v[849]) + (eq52_e1381 * s.dn[849][8]));
        let eq52_e1383_d_n9: f64 = ((eq52_e1381_d_n9 * s.v[849]) + (eq52_e1381 * s.dn[849][9]));
        let eq52_e1383_d_n10: f64 = ((eq52_e1381_d_n10 * s.v[849]) + (eq52_e1381 * s.dn[849][10]));
        let eq52_e1383_d_n11: f64 = ((eq52_e1381_d_n11 * s.v[849]) + (eq52_e1381 * s.dn[849][11]));
        let eq52_e1383_d_n12: f64 = ((eq52_e1381_d_n12 * s.v[849]) + (eq52_e1381 * s.dn[849][12]));
        let eq52_e1383_d_b0: f64 = ((eq52_e1381_d_b0 * s.v[849]) + (eq52_e1381 * s.db[849][0]));
        let eq52_e1383_d_b1: f64 = ((eq52_e1381_d_b1 * s.v[849]) + (eq52_e1381 * s.db[849][1]));
        let eq52_e1383_d_b2: f64 = ((eq52_e1381_d_b2 * s.v[849]) + (eq52_e1381 * s.db[849][2]));
        let eq52_e1383_d_b3: f64 = ((eq52_e1381_d_b3 * s.v[849]) + (eq52_e1381 * s.db[849][3]));
        let eq52_e1383_d_b4: f64 = ((eq52_e1381_d_b4 * s.v[849]) + (eq52_e1381 * s.db[849][4]));
        let eq52_e1383_d_b5: f64 = ((eq52_e1381_d_b5 * s.v[849]) + (eq52_e1381 * s.db[849][5]));
        let eq52_e1383_d_b6: f64 = ((eq52_e1381_d_b6 * s.v[849]) + (eq52_e1381 * s.db[849][6]));
        let eq52_e1385: f64 = (eq52_e1383 * (nv5 - 0.0));
        let eq52_e1385_d_n0: f64 = (eq52_e1383_d_n0 * (nv5 - 0.0));
        let eq52_e1385_d_n1: f64 = (eq52_e1383_d_n1 * (nv5 - 0.0));
        let eq52_e1385_d_n2: f64 = (eq52_e1383_d_n2 * (nv5 - 0.0));
        let eq52_e1385_d_n3: f64 = (eq52_e1383_d_n3 * (nv5 - 0.0));
        let eq52_e1385_d_n4: f64 = (eq52_e1383_d_n4 * (nv5 - 0.0));
        let eq52_e1385_d_n5: f64 = ((eq52_e1383_d_n5 * (nv5 - 0.0)) + eq52_e1383);
        let eq52_e1385_d_n6: f64 = (eq52_e1383_d_n6 * (nv5 - 0.0));
        let eq52_e1385_d_n7: f64 = (eq52_e1383_d_n7 * (nv5 - 0.0));
        let eq52_e1385_d_n8: f64 = (eq52_e1383_d_n8 * (nv5 - 0.0));
        let eq52_e1385_d_n9: f64 = (eq52_e1383_d_n9 * (nv5 - 0.0));
        let eq52_e1385_d_n10: f64 = (eq52_e1383_d_n10 * (nv5 - 0.0));
        let eq52_e1385_d_n11: f64 = (eq52_e1383_d_n11 * (nv5 - 0.0));
        let eq52_e1385_d_n12: f64 = (eq52_e1383_d_n12 * (nv5 - 0.0));
        let eq52_e1385_d_b0: f64 = (eq52_e1383_d_b0 * (nv5 - 0.0));
        let eq52_e1385_d_b1: f64 = (eq52_e1383_d_b1 * (nv5 - 0.0));
        let eq52_e1385_d_b2: f64 = (eq52_e1383_d_b2 * (nv5 - 0.0));
        let eq52_e1385_d_b3: f64 = (eq52_e1383_d_b3 * (nv5 - 0.0));
        let eq52_e1385_d_b4: f64 = (eq52_e1383_d_b4 * (nv5 - 0.0));
        let eq52_e1385_d_b5: f64 = (eq52_e1383_d_b5 * (nv5 - 0.0));
        let eq52_e1385_d_b6: f64 = (eq52_e1383_d_b6 * (nv5 - 0.0));
        let eq52_e1386_q: f64 = eq52_e1385;
        let eq52_e1387: f64 = (-eq52_e1385);
        let eq52_e1387_d_n0: f64 = (-eq52_e1385_d_n0);
        let eq52_e1387_d_n1: f64 = (-eq52_e1385_d_n1);
        let eq52_e1387_d_n2: f64 = (-eq52_e1385_d_n2);
        let eq52_e1387_d_n3: f64 = (-eq52_e1385_d_n3);
        let eq52_e1387_d_n4: f64 = (-eq52_e1385_d_n4);
        let eq52_e1387_d_n5: f64 = (-eq52_e1385_d_n5);
        let eq52_e1387_d_n6: f64 = (-eq52_e1385_d_n6);
        let eq52_e1387_d_n7: f64 = (-eq52_e1385_d_n7);
        let eq52_e1387_d_n8: f64 = (-eq52_e1385_d_n8);
        let eq52_e1387_d_n9: f64 = (-eq52_e1385_d_n9);
        let eq52_e1387_d_n10: f64 = (-eq52_e1385_d_n10);
        let eq52_e1387_d_n11: f64 = (-eq52_e1385_d_n11);
        let eq52_e1387_d_n12: f64 = (-eq52_e1385_d_n12);
        let eq52_e1387_d_b0: f64 = (-eq52_e1385_d_b0);
        let eq52_e1387_d_b1: f64 = (-eq52_e1385_d_b1);
        let eq52_e1387_d_b2: f64 = (-eq52_e1385_d_b2);
        let eq52_e1387_d_b3: f64 = (-eq52_e1385_d_b3);
        let eq52_e1387_d_b4: f64 = (-eq52_e1385_d_b4);
        let eq52_e1387_d_b5: f64 = (-eq52_e1385_d_b5);
        let eq52_e1387_d_b6: f64 = (-eq52_e1385_d_b6);
        let eq52_e1387_q: f64 = (-eq52_e1386_q);
        let eq52_e1387_q_d_n0: f64 = (-eq52_e1385_d_n0);
        let eq52_e1387_q_d_n1: f64 = (-eq52_e1385_d_n1);
        let eq52_e1387_q_d_n2: f64 = (-eq52_e1385_d_n2);
        let eq52_e1387_q_d_n3: f64 = (-eq52_e1385_d_n3);
        let eq52_e1387_q_d_n4: f64 = (-eq52_e1385_d_n4);
        let eq52_e1387_q_d_n5: f64 = (-eq52_e1385_d_n5);
        let eq52_e1387_q_d_n6: f64 = (-eq52_e1385_d_n6);
        let eq52_e1387_q_d_n7: f64 = (-eq52_e1385_d_n7);
        let eq52_e1387_q_d_n8: f64 = (-eq52_e1385_d_n8);
        let eq52_e1387_q_d_n9: f64 = (-eq52_e1385_d_n9);
        let eq52_e1387_q_d_n10: f64 = (-eq52_e1385_d_n10);
        let eq52_e1387_q_d_n11: f64 = (-eq52_e1385_d_n11);
        let eq52_e1387_q_d_n12: f64 = (-eq52_e1385_d_n12);
        let eq52_e1387_q_d_b0: f64 = (-eq52_e1385_d_b0);
        let eq52_e1387_q_d_b1: f64 = (-eq52_e1385_d_b1);
        let eq52_e1387_q_d_b2: f64 = (-eq52_e1385_d_b2);
        let eq52_e1387_q_d_b3: f64 = (-eq52_e1385_d_b3);
        let eq52_e1387_q_d_b4: f64 = (-eq52_e1385_d_b4);
        let eq52_e1387_q_d_b5: f64 = (-eq52_e1385_d_b5);
        let eq52_e1387_q_d_b6: f64 = (-eq52_e1385_d_b6);
        let eq52_reactive_node_derivatives: [f64; 13] = [eq52_e1387_q_d_n0, eq52_e1387_q_d_n1, eq52_e1387_q_d_n2, eq52_e1387_q_d_n3, eq52_e1387_q_d_n4, eq52_e1387_q_d_n5, eq52_e1387_q_d_n6, eq52_e1387_q_d_n7, eq52_e1387_q_d_n8, eq52_e1387_q_d_n9, eq52_e1387_q_d_n10, eq52_e1387_q_d_n11, eq52_e1387_q_d_n12];
        let eq52_reactive_branch_derivatives: [f64; 7] = [eq52_e1387_q_d_b0, eq52_e1387_q_d_b1, eq52_e1387_q_d_b2, eq52_e1387_q_d_b3, eq52_e1387_q_d_b4, eq52_e1387_q_d_b5, eq52_e1387_q_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[7]),
            nodes,
            &eq52_reactive_node_derivatives,
            branches,
            &eq52_reactive_branch_derivatives,
            multiplicity,
        );
        let eq53_e1390: f64 = (s.v[15] * p.p32);
        let eq53_e1390_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq53_e1390_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq53_e1390_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq53_e1390_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq53_e1390_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq53_e1390_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq53_e1390_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq53_e1390_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq53_e1390_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq53_e1390_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq53_e1390_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq53_e1390_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq53_e1390_d_n12: f64 = (s.dn[15][12] * p.p32);
        let eq53_e1390_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq53_e1390_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq53_e1390_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq53_e1390_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq53_e1390_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq53_e1390_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq53_e1390_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq53_e1391: f64 = (eq53_e1390).sqrt();
        let eq53_e1391_d_n0: f64 = (eq53_e1390_d_n0 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n1: f64 = (eq53_e1390_d_n1 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n2: f64 = (eq53_e1390_d_n2 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n3: f64 = (eq53_e1390_d_n3 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n4: f64 = (eq53_e1390_d_n4 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n5: f64 = (eq53_e1390_d_n5 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n6: f64 = (eq53_e1390_d_n6 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n7: f64 = (eq53_e1390_d_n7 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n8: f64 = (eq53_e1390_d_n8 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n9: f64 = (eq53_e1390_d_n9 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n10: f64 = (eq53_e1390_d_n10 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n11: f64 = (eq53_e1390_d_n11 / (2.0 * eq53_e1391));
        let eq53_e1391_d_n12: f64 = (eq53_e1390_d_n12 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b0: f64 = (eq53_e1390_d_b0 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b1: f64 = (eq53_e1390_d_b1 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b2: f64 = (eq53_e1390_d_b2 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b3: f64 = (eq53_e1390_d_b3 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b4: f64 = (eq53_e1390_d_b4 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b5: f64 = (eq53_e1390_d_b5 / (2.0 * eq53_e1391));
        let eq53_e1391_d_b6: f64 = (eq53_e1390_d_b6 / (2.0 * eq53_e1391));
        let eq53_e1393: f64 = (eq53_e1391 * 0.5);
        let eq53_e1393_d_n0: f64 = (eq53_e1391_d_n0 * 0.5);
        let eq53_e1393_d_n1: f64 = (eq53_e1391_d_n1 * 0.5);
        let eq53_e1393_d_n2: f64 = (eq53_e1391_d_n2 * 0.5);
        let eq53_e1393_d_n3: f64 = (eq53_e1391_d_n3 * 0.5);
        let eq53_e1393_d_n4: f64 = (eq53_e1391_d_n4 * 0.5);
        let eq53_e1393_d_n5: f64 = (eq53_e1391_d_n5 * 0.5);
        let eq53_e1393_d_n6: f64 = (eq53_e1391_d_n6 * 0.5);
        let eq53_e1393_d_n7: f64 = (eq53_e1391_d_n7 * 0.5);
        let eq53_e1393_d_n8: f64 = (eq53_e1391_d_n8 * 0.5);
        let eq53_e1393_d_n9: f64 = (eq53_e1391_d_n9 * 0.5);
        let eq53_e1393_d_n10: f64 = (eq53_e1391_d_n10 * 0.5);
        let eq53_e1393_d_n11: f64 = (eq53_e1391_d_n11 * 0.5);
        let eq53_e1393_d_n12: f64 = (eq53_e1391_d_n12 * 0.5);
        let eq53_e1393_d_b0: f64 = (eq53_e1391_d_b0 * 0.5);
        let eq53_e1393_d_b1: f64 = (eq53_e1391_d_b1 * 0.5);
        let eq53_e1393_d_b2: f64 = (eq53_e1391_d_b2 * 0.5);
        let eq53_e1393_d_b3: f64 = (eq53_e1391_d_b3 * 0.5);
        let eq53_e1393_d_b4: f64 = (eq53_e1391_d_b4 * 0.5);
        let eq53_e1393_d_b5: f64 = (eq53_e1391_d_b5 * 0.5);
        let eq53_e1393_d_b6: f64 = (eq53_e1391_d_b6 * 0.5);
        let eq53_e1395: f64 = (eq53_e1393 * s.v[849]);
        let eq53_e1395_d_n0: f64 = ((eq53_e1393_d_n0 * s.v[849]) + (eq53_e1393 * s.dn[849][0]));
        let eq53_e1395_d_n1: f64 = ((eq53_e1393_d_n1 * s.v[849]) + (eq53_e1393 * s.dn[849][1]));
        let eq53_e1395_d_n2: f64 = ((eq53_e1393_d_n2 * s.v[849]) + (eq53_e1393 * s.dn[849][2]));
        let eq53_e1395_d_n3: f64 = ((eq53_e1393_d_n3 * s.v[849]) + (eq53_e1393 * s.dn[849][3]));
        let eq53_e1395_d_n4: f64 = ((eq53_e1393_d_n4 * s.v[849]) + (eq53_e1393 * s.dn[849][4]));
        let eq53_e1395_d_n5: f64 = ((eq53_e1393_d_n5 * s.v[849]) + (eq53_e1393 * s.dn[849][5]));
        let eq53_e1395_d_n6: f64 = ((eq53_e1393_d_n6 * s.v[849]) + (eq53_e1393 * s.dn[849][6]));
        let eq53_e1395_d_n7: f64 = ((eq53_e1393_d_n7 * s.v[849]) + (eq53_e1393 * s.dn[849][7]));
        let eq53_e1395_d_n8: f64 = ((eq53_e1393_d_n8 * s.v[849]) + (eq53_e1393 * s.dn[849][8]));
        let eq53_e1395_d_n9: f64 = ((eq53_e1393_d_n9 * s.v[849]) + (eq53_e1393 * s.dn[849][9]));
        let eq53_e1395_d_n10: f64 = ((eq53_e1393_d_n10 * s.v[849]) + (eq53_e1393 * s.dn[849][10]));
        let eq53_e1395_d_n11: f64 = ((eq53_e1393_d_n11 * s.v[849]) + (eq53_e1393 * s.dn[849][11]));
        let eq53_e1395_d_n12: f64 = ((eq53_e1393_d_n12 * s.v[849]) + (eq53_e1393 * s.dn[849][12]));
        let eq53_e1395_d_b0: f64 = ((eq53_e1393_d_b0 * s.v[849]) + (eq53_e1393 * s.db[849][0]));
        let eq53_e1395_d_b1: f64 = ((eq53_e1393_d_b1 * s.v[849]) + (eq53_e1393 * s.db[849][1]));
        let eq53_e1395_d_b2: f64 = ((eq53_e1393_d_b2 * s.v[849]) + (eq53_e1393 * s.db[849][2]));
        let eq53_e1395_d_b3: f64 = ((eq53_e1393_d_b3 * s.v[849]) + (eq53_e1393 * s.db[849][3]));
        let eq53_e1395_d_b4: f64 = ((eq53_e1393_d_b4 * s.v[849]) + (eq53_e1393 * s.db[849][4]));
        let eq53_e1395_d_b5: f64 = ((eq53_e1393_d_b5 * s.v[849]) + (eq53_e1393 * s.db[849][5]));
        let eq53_e1395_d_b6: f64 = ((eq53_e1393_d_b6 * s.v[849]) + (eq53_e1393 * s.db[849][6]));
        let eq53_e1397: f64 = (eq53_e1395 * (nv5 - 0.0));
        let eq53_e1397_d_n0: f64 = (eq53_e1395_d_n0 * (nv5 - 0.0));
        let eq53_e1397_d_n1: f64 = (eq53_e1395_d_n1 * (nv5 - 0.0));
        let eq53_e1397_d_n2: f64 = (eq53_e1395_d_n2 * (nv5 - 0.0));
        let eq53_e1397_d_n3: f64 = (eq53_e1395_d_n3 * (nv5 - 0.0));
        let eq53_e1397_d_n4: f64 = (eq53_e1395_d_n4 * (nv5 - 0.0));
        let eq53_e1397_d_n5: f64 = ((eq53_e1395_d_n5 * (nv5 - 0.0)) + eq53_e1395);
        let eq53_e1397_d_n6: f64 = (eq53_e1395_d_n6 * (nv5 - 0.0));
        let eq53_e1397_d_n7: f64 = (eq53_e1395_d_n7 * (nv5 - 0.0));
        let eq53_e1397_d_n8: f64 = (eq53_e1395_d_n8 * (nv5 - 0.0));
        let eq53_e1397_d_n9: f64 = (eq53_e1395_d_n9 * (nv5 - 0.0));
        let eq53_e1397_d_n10: f64 = (eq53_e1395_d_n10 * (nv5 - 0.0));
        let eq53_e1397_d_n11: f64 = (eq53_e1395_d_n11 * (nv5 - 0.0));
        let eq53_e1397_d_n12: f64 = (eq53_e1395_d_n12 * (nv5 - 0.0));
        let eq53_e1397_d_b0: f64 = (eq53_e1395_d_b0 * (nv5 - 0.0));
        let eq53_e1397_d_b1: f64 = (eq53_e1395_d_b1 * (nv5 - 0.0));
        let eq53_e1397_d_b2: f64 = (eq53_e1395_d_b2 * (nv5 - 0.0));
        let eq53_e1397_d_b3: f64 = (eq53_e1395_d_b3 * (nv5 - 0.0));
        let eq53_e1397_d_b4: f64 = (eq53_e1395_d_b4 * (nv5 - 0.0));
        let eq53_e1397_d_b5: f64 = (eq53_e1395_d_b5 * (nv5 - 0.0));
        let eq53_e1397_d_b6: f64 = (eq53_e1395_d_b6 * (nv5 - 0.0));
        let eq53_e1398_q: f64 = eq53_e1397;
        let eq53_e1399: f64 = (-eq53_e1397);
        let eq53_e1399_d_n0: f64 = (-eq53_e1397_d_n0);
        let eq53_e1399_d_n1: f64 = (-eq53_e1397_d_n1);
        let eq53_e1399_d_n2: f64 = (-eq53_e1397_d_n2);
        let eq53_e1399_d_n3: f64 = (-eq53_e1397_d_n3);
        let eq53_e1399_d_n4: f64 = (-eq53_e1397_d_n4);
        let eq53_e1399_d_n5: f64 = (-eq53_e1397_d_n5);
        let eq53_e1399_d_n6: f64 = (-eq53_e1397_d_n6);
        let eq53_e1399_d_n7: f64 = (-eq53_e1397_d_n7);
        let eq53_e1399_d_n8: f64 = (-eq53_e1397_d_n8);
        let eq53_e1399_d_n9: f64 = (-eq53_e1397_d_n9);
        let eq53_e1399_d_n10: f64 = (-eq53_e1397_d_n10);
        let eq53_e1399_d_n11: f64 = (-eq53_e1397_d_n11);
        let eq53_e1399_d_n12: f64 = (-eq53_e1397_d_n12);
        let eq53_e1399_d_b0: f64 = (-eq53_e1397_d_b0);
        let eq53_e1399_d_b1: f64 = (-eq53_e1397_d_b1);
        let eq53_e1399_d_b2: f64 = (-eq53_e1397_d_b2);
        let eq53_e1399_d_b3: f64 = (-eq53_e1397_d_b3);
        let eq53_e1399_d_b4: f64 = (-eq53_e1397_d_b4);
        let eq53_e1399_d_b5: f64 = (-eq53_e1397_d_b5);
        let eq53_e1399_d_b6: f64 = (-eq53_e1397_d_b6);
        let eq53_e1399_q: f64 = (-eq53_e1398_q);
        let eq53_e1399_q_d_n0: f64 = (-eq53_e1397_d_n0);
        let eq53_e1399_q_d_n1: f64 = (-eq53_e1397_d_n1);
        let eq53_e1399_q_d_n2: f64 = (-eq53_e1397_d_n2);
        let eq53_e1399_q_d_n3: f64 = (-eq53_e1397_d_n3);
        let eq53_e1399_q_d_n4: f64 = (-eq53_e1397_d_n4);
        let eq53_e1399_q_d_n5: f64 = (-eq53_e1397_d_n5);
        let eq53_e1399_q_d_n6: f64 = (-eq53_e1397_d_n6);
        let eq53_e1399_q_d_n7: f64 = (-eq53_e1397_d_n7);
        let eq53_e1399_q_d_n8: f64 = (-eq53_e1397_d_n8);
        let eq53_e1399_q_d_n9: f64 = (-eq53_e1397_d_n9);
        let eq53_e1399_q_d_n10: f64 = (-eq53_e1397_d_n10);
        let eq53_e1399_q_d_n11: f64 = (-eq53_e1397_d_n11);
        let eq53_e1399_q_d_n12: f64 = (-eq53_e1397_d_n12);
        let eq53_e1399_q_d_b0: f64 = (-eq53_e1397_d_b0);
        let eq53_e1399_q_d_b1: f64 = (-eq53_e1397_d_b1);
        let eq53_e1399_q_d_b2: f64 = (-eq53_e1397_d_b2);
        let eq53_e1399_q_d_b3: f64 = (-eq53_e1397_d_b3);
        let eq53_e1399_q_d_b4: f64 = (-eq53_e1397_d_b4);
        let eq53_e1399_q_d_b5: f64 = (-eq53_e1397_d_b5);
        let eq53_e1399_q_d_b6: f64 = (-eq53_e1397_d_b6);
        let eq53_reactive_node_derivatives: [f64; 13] = [eq53_e1399_q_d_n0, eq53_e1399_q_d_n1, eq53_e1399_q_d_n2, eq53_e1399_q_d_n3, eq53_e1399_q_d_n4, eq53_e1399_q_d_n5, eq53_e1399_q_d_n6, eq53_e1399_q_d_n7, eq53_e1399_q_d_n8, eq53_e1399_q_d_n9, eq53_e1399_q_d_n10, eq53_e1399_q_d_n11, eq53_e1399_q_d_n12];
        let eq53_reactive_branch_derivatives: [f64; 7] = [eq53_e1399_q_d_b0, eq53_e1399_q_d_b1, eq53_e1399_q_d_b2, eq53_e1399_q_d_b3, eq53_e1399_q_d_b4, eq53_e1399_q_d_b5, eq53_e1399_q_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[6]),
            Some(nodes[8]),
            nodes,
            &eq53_reactive_node_derivatives,
            branches,
            &eq53_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
