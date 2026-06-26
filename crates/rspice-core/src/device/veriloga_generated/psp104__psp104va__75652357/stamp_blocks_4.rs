#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_reactive_block_19(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2555] && s.b[2556]) {
            s.store_scaled_mul(2511, 678, 678, 4.0);
            s.store_div(2512, 678, 679);
            s.store_add_scaled_product(2513, s.ad_value(827), 1.0, s.ad_value(678), s.ad_value(2512), 1.0);
            s.store_add(2514, 679, 2513);
            s.store_sub(2515, 679, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product(2560, s.ad_value(827), s.ad_value(679), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0);
        }

        s.b[2567] = (s.v[672] > 0.5);
        s.v[2567] = if s.b[2567] { 1.0 } else { 0.0 };

        s.b[2568] = (s.v[569] == 0.5);
        s.v[2568] = if s.b[2568] { 1.0 } else { 0.0 };

        if (((s.b[2555] && s.b[2556]) && s.b[2567]) && s.b[2568]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::mul(s.ad_value(2560), s.ad_value(566)));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2567]) && (!s.b[2568])) {
            s.store_pow_ad(2559, A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(566))), s.ad_value(569));
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2567]) {
            s.store_add_scaled_product(1909, A::mul_sub_from_scalar_rhs(s.ad_value(578), 1.0, s.ad_value(2559)), 1.0, s.ad_value(581), A::sub(s.ad_value(827), s.ad_value(2560)), 1.0);
        }

        s.b[2569] = (s.v[673] > 0.5);
        s.v[2569] = if s.b[2569] { 1.0 } else { 0.0 };

        s.b[2570] = (s.v[570] == 0.5);
        s.v[2570] = if s.b[2570] { 1.0 } else { 0.0 };

        if (((s.b[2555] && s.b[2556]) && s.b[2569]) && s.b[2570]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::mul(s.ad_value(2560), s.ad_value(567)));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2569]) && (!s.b[2570])) {
            s.store_pow_ad(2559, A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(567))), s.ad_value(570));
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2569]) {
            s.store_add_scaled_product(1910, A::mul_sub_from_scalar_rhs(s.ad_value(579), 1.0, s.ad_value(2559)), 1.0, s.ad_value(582), A::sub(s.ad_value(827), s.ad_value(2560)), 1.0);
        }

        s.b[2571] = (s.v[674] > 0.5);
        s.v[2571] = if s.b[2571] { 1.0 } else { 0.0 };

        s.b[2572] = (s.v[571] == 0.5);
        s.v[2572] = if s.b[2572] { 1.0 } else { 0.0 };

        if (((s.b[2555] && s.b[2556]) && s.b[2571]) && s.b[2572]) {
            s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::mul(s.ad_value(2560), s.ad_value(568)));
        }

        if (((s.b[2555] && s.b[2556]) && s.b[2571]) && (!s.b[2572])) {
            s.store_pow_ad(2559, A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(568))), s.ad_value(571));
        }

        if ((s.b[2555] && s.b[2556]) && s.b[2571]) {
            s.store_add_scaled_product(1911, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(2559)), 1.0, s.ad_value(583), A::sub(s.ad_value(827), s.ad_value(2560)), 1.0);
        }

        s.b[2573] = (p.p865 > 0.0);
        s.v[2573] = if s.b[2573] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2573]) {
            s.store_scaled_offset_ad(636, A::powf(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001))), 0.5), p.p866), (-(((0.5 * 0.001)) as f64).powf(p.p866)), p.p865);
            s.store_offset(634, 636, p.p855);
            s.store_div_from_scalar(444, 1.0, 634);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2573])) {
            s.store_scalar(634, p.p855);
        }

        s.b[2574] = (p.p867 > 0.0);
        s.v[2574] = if s.b[2574] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2574]) {
            s.store_scaled_offset_ad(638, A::powf(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001))), 0.5), p.p868), (-(((0.5 * 0.001)) as f64).powf(p.p868)), p.p867);
            s.store_mul_offset_rhs(437, 437, 638, 1.0);
        }

        if (s.b[2555] && (!s.b[2556])) {
            s.store_scalar(2524, 0.0);
            s.store_scalar(2521, 0.0);
        }

        s.b[2575] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));
        s.v[2575] = if s.b[2575] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2575]) {
            s.store_scaled_mul(2511, 651, 651, 4.0);
            s.store_div(2512, 651, 652);
            s.store_add_scaled_product(2513, s.ad_value(826), 1.0, s.ad_value(651), s.ad_value(2512), 1.0);
            s.store_add(2514, 652, 2513);
            s.store_sub(2515, 652, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product(2518, s.ad_value(826), s.ad_value(652), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0);
        }

        s.b[2576] = (s.v[826] < s.v[648]);
        s.v[2576] = if s.b[2576] { 1.0 } else { 0.0 };

        s.b[2577] = (((((-0.5) * (s.v[826] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[2577] = if s.b[2577] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) && s.b[2577]) {
            s.store_exp_scaled_input(2519, 826, (s.v[365] * (-0.5)));
        }

        s.b[2578] = (((-0.5) * (s.v[826] * s.v[365])) < 0.0);
        s.v[2578] = if s.b[2578] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) && (!s.b[2577])) && s.b[2578]) {
            s.store_div_from_scalar_offset_ad(2519, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(826), (s.v[365] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(826), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) && (!s.b[2577])) && (!s.b[2578])) {
            s.store_scaled_offset_ad(2519, A::mul_offset_rhs(A::scale_offset(s.ad_value(826), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(826), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(826), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) {
            s.store_div_from_scalar(2520, 1.0, 2519);
            s.store_square(2517, 2520);
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && (!s.b[2576])) {
            s.store_mul_offset_ad_lhs(2517, A::sub_scaled_inputs(s.ad_value(826), s.v[365], s.ad_value(648), s.v[365]), 1.0, 649);
            s.store_sqrt(2520, 2517);
            s.store_div_from_scalar(2519, 1.0, 2520);
        }

        if ((s.b[2555] && (!s.b[2556])) && s.b[2575]) {
            s.store_offset(2517, 2517, (-1.0));
        }

        s.b[2579] = (s.v[826] > 0.0);
        s.v[2579] = if s.b[2579] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2579]) {
            s.store_scaled_ln_ad(2521, A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2519), 1.0, A::offset(s.ad_value(2519), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && (!s.b[2579])) {
            s.store_sub_ad_lhs(2521, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2520), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2520), 1.0, A::scale_offset(s.ad_value(2520), 3.0, 1.0))))), (s.v[364] * 2.0)), 826);
        }

        if ((s.b[2555] && (!s.b[2556])) && s.b[2575]) {
            s.store_sub(2522, 650, 2521);
            s.store_add_scaled_inputs3(2523, s.ad_value(826), 0.5, s.ad_value(2522), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(826), s.ad_value(2522)), A::sub(s.ad_value(826), s.ad_value(2522))), ((4.0 * s.v[364]) * s.v[364]))), (-0.5));
            s.store_add_scaled_inputs3(2524, s.ad_value(826), 0.5, s.ad_value(653), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(826), s.ad_value(653)), A::sub(s.ad_value(826), s.ad_value(653))), ((4.0 * s.v[362]) * s.v[362]))), (-0.5));
            s.store_scaled_sub_ad_rhs(2525, 826, A::sqrt(A::offset(A::mul(s.ad_value(826), s.ad_value(826)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[2580] = (s.v[640] == 0.0);
        s.v[2580] = if s.b[2580] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2580]) {
            s.store_scalar(1906, 0.0);
        }

        s.b[2581] = ((p.p833 == 0.0) && (p.p838 == 0.0));
        s.v[2581] = if s.b[2581] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) {
            s.store_sub_from_scalar(2529, s.v[387], 2523);
        }

        s.b[2583] = (p.p824 == 0.5);
        s.v[2583] = if s.b[2583] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) && s.b[2583]) {
            s.store_sqrt_scaled_input(2526, 2529, s.v[423]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) && (!s.b[2583])) {
            s.store_powf_ad(2526, A::scale(s.ad_value(2529), s.v[423]), p.p824);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) {
            s.store_scale(2533, 2526, s.v[417]);
        }

        s.b[2584] = (p.p838 == 0.0);
        s.v[2584] = if s.b[2584] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) {
            s.store_div_scaled_inputs(2536, s.ad_value(2533), (s.v[402] * s.v[432]), s.ad_value(2529), 1.0);
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[429]), 2536);
            s.store_square(2538, 2537);
            s.store_sqrt_ad(2539, A::div_scaled_product_offset_denominator(s.ad_value(2538), s.ad_value(2538), 1.0, A::square(s.ad_value(2538)), 1.0, 1.0));
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
            s.store_sqrt_scaled_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_add_scaled_product(2545, s.ad_value(2539), (-1.0), s.ad_value(2537), s.ad_value(2540), 2.0);
            s.store_ad_value(2546, A::add_scaled_value_products(s.ad_value(2539), (-s.v[429]), s.ad_value(2537), s.ad_value(2540), s.v[429], s.ad_value(2536), s.ad_value(2541), 0.5));
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2587] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.v[2587] = if s.b[2587] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && s.b[2587]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2587])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2588] = (s.v[2547] > 0.0);
        s.v[2588] = if s.b[2588] { 1.0 } else { 0.0 };

        s.b[2589] = (s.v[2546] > (-230.25850929940458));
        s.v[2589] = if s.b[2589] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2588])) && s.b[2589]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2588])) && (!s.b[2589])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2546), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2546), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2590] = (p.p844 == 0.0);
        s.v[2590] = if s.b[2590] { 1.0 } else { 0.0 };

        s.b[2591] = (p.p824 == 0.5);
        s.v[2591] = if s.b[2591] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && s.b[2591]) {
            s.store_sqrt_scaled_ad(2526, A::sub_from_scalar(p.p821, s.ad_value(2524)), s.v[423]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && (!s.b[2591])) {
            s.store_powf_ad(2526, A::scale_offset(s.ad_value(2524), (-s.v[423]), ((p.p821) * (s.v[423]))), p.p824);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) {
            s.store_ad_value(2551, A::div_scaled_offset_numerator(s.ad_value(2524), ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), s.ad_value(2526), 1.0));
        }

        s.b[2592] = (((((-s.v[435]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.v[2592] = if s.b[2592] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && s.b[2592]) {
            s.store_exp_ad(2526, A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2593] = (((-s.v[435]) / s.v[2551]) < 0.0);
        s.v[2593] = if s.b[2593] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && (!s.b[2592])) && s.b[2593]) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && (!s.b[2592])) && (!s.b[2593])) {
            let assign57000_ad_e72320: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2526, assign57000_ad_e72320, 1e100);
        }

        s.b[2594] = (p.p853 > 1000.0);
        s.v[2594] = if s.b[2594] { 1.0 } else { 0.0 };

        s.b[2595] = (s.v[2525] > ((-s.v[438]) * p.p853));
        s.v[2595] = if s.b[2595] { 1.0 } else { 0.0 };

        s.b[2596] = (p.p856 == 4.0);
        s.v[2596] = if s.b[2596] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2594])) && s.b[2595]) && s.b[2596]) {
            s.store_mul_scaled_ad_lhs(2526, A::mul3_scaled_output(s.ad_value(2525), s.ad_value(2525), s.ad_value(2525), ((s.v[442] * s.v[442]) * s.v[442])), 2525, s.v[442]);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2594])) && s.b[2595]) && (!s.b[2596])) {
            s.store_powf_ad(2526, A::abs_scaled_input(s.ad_value(2525), s.v[442]), p.p856);
        }

        s.b[2597] = (s.v[402] == 0.5);
        s.v[2597] = if s.b[2597] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && s.b[2597]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[399]));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2597])) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[399])), s.v[402]);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) {
            s.store_add_scaled_inputs3_offset(1906, s.ad_value(2526), ((-s.v[411]) * p.p30), s.ad_value(826), (s.v[414] * p.p30), s.ad_value(2518), ((-s.v[414]) * p.p30), (s.v[411] * p.p30));
        }

        s.b[2598] = (s.v[641] == 0.0);
        s.v[2598] = if s.b[2598] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2598]) {
            s.store_scalar(1907, 0.0);
        }

        s.b[2599] = ((p.p834 == 0.0) && (p.p839 == 0.0));
        s.v[2599] = if s.b[2599] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) {
            s.store_sub_from_scalar(2529, s.v[388], 2523);
        }

        s.b[2601] = (p.p825 == 0.5);
        s.v[2601] = if s.b[2601] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) && s.b[2601]) {
            s.store_sqrt_scaled_input(2526, 2529, s.v[424]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) && (!s.b[2601])) {
            s.store_powf_ad(2526, A::scale(s.ad_value(2529), s.v[424]), p.p825);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) {
            s.store_scale(2533, 2526, s.v[418]);
        }

        s.b[2602] = (p.p839 == 0.0);
        s.v[2602] = if s.b[2602] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) {
            s.store_div_scaled_inputs(2536, s.ad_value(2533), (s.v[403] * s.v[433]), s.ad_value(2529), 1.0);
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[430]), 2536);
            s.store_square(2538, 2537);
            s.store_sqrt_ad(2539, A::div_scaled_product_offset_denominator(s.ad_value(2538), s.ad_value(2538), 1.0, A::square(s.ad_value(2538)), 1.0, 1.0));
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
            s.store_sqrt_scaled_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_add_scaled_product(2545, s.ad_value(2539), (-1.0), s.ad_value(2537), s.ad_value(2540), 2.0);
            s.store_ad_value(2546, A::add_scaled_value_products(s.ad_value(2539), (-s.v[430]), s.ad_value(2537), s.ad_value(2540), s.v[430], s.ad_value(2536), s.ad_value(2541), 0.5));
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2605] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.v[2605] = if s.b[2605] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && s.b[2605]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2605])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2606] = (s.v[2547] > 0.0);
        s.v[2606] = if s.b[2606] { 1.0 } else { 0.0 };

        s.b[2607] = (s.v[2546] > (-230.25850929940458));
        s.v[2607] = if s.b[2607] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2606])) && s.b[2607]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2606])) && (!s.b[2607])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2546), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2546), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2608] = (p.p845 == 0.0);
        s.v[2608] = if s.b[2608] { 1.0 } else { 0.0 };

        s.b[2609] = (p.p825 == 0.5);
        s.v[2609] = if s.b[2609] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && s.b[2609]) {
            s.store_sqrt_scaled_ad(2526, A::sub_from_scalar(p.p822, s.ad_value(2524)), s.v[424]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2609])) {
            s.store_powf_ad(2526, A::scale_offset(s.ad_value(2524), (-s.v[424]), ((p.p822) * (s.v[424]))), p.p825);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) {
            s.store_ad_value(2551, A::div_scaled_offset_numerator(s.ad_value(2524), ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), s.ad_value(2526), 1.0));
        }

    }

    pub(super) fn stamp_reactive_block_20(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2610] = (((((-s.v[436]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.v[2610] = if s.b[2610] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && s.b[2610]) {
            s.store_exp_ad(2526, A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2611] = (((-s.v[436]) / s.v[2551]) < 0.0);
        s.v[2611] = if s.b[2611] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2610])) && s.b[2611]) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2610])) && (!s.b[2611])) {
            let assign57750_ad_e73586: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2526, assign57750_ad_e73586, 1e100);
        }

        s.b[2612] = (p.p854 > 1000.0);
        s.v[2612] = if s.b[2612] { 1.0 } else { 0.0 };

        s.b[2613] = (s.v[2525] > ((-s.v[438]) * p.p854));
        s.v[2613] = if s.b[2613] { 1.0 } else { 0.0 };

        s.b[2614] = (p.p857 == 4.0);
        s.v[2614] = if s.b[2614] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && s.b[2613]) && s.b[2614]) {
            s.store_mul_scaled_ad_lhs(2526, A::mul3_scaled_output(s.ad_value(2525), s.ad_value(2525), s.ad_value(2525), ((s.v[443] * s.v[443]) * s.v[443])), 2525, s.v[443]);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && s.b[2613]) && (!s.b[2614])) {
            s.store_powf_ad(2526, A::abs_scaled_input(s.ad_value(2525), s.v[443]), p.p857);
        }

        s.b[2615] = (s.v[403] == 0.5);
        s.v[2615] = if s.b[2615] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && s.b[2615]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[400]));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2615])) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[400])), s.v[403]);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) {
            s.store_add_scaled_inputs3_offset(1907, s.ad_value(2526), ((-s.v[412]) * p.p30), s.ad_value(826), (s.v[415] * p.p30), s.ad_value(2518), ((-s.v[415]) * p.p30), (s.v[412] * p.p30));
        }

        s.b[2616] = (s.v[642] == 0.0);
        s.v[2616] = if s.b[2616] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2616]) {
            s.store_scalar(1908, 0.0);
        }

        s.b[2617] = ((p.p835 == 0.0) && (p.p840 == 0.0));
        s.v[2617] = if s.b[2617] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) {
            s.store_sub_from_scalar(2529, s.v[389], 2523);
        }

        s.b[2619] = (p.p826 == 0.5);
        s.v[2619] = if s.b[2619] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && s.b[2619]) {
            s.store_sqrt_scaled_input(2526, 2529, s.v[425]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && (!s.b[2619])) {
            s.store_powf_ad(2526, A::scale(s.ad_value(2529), s.v[425]), p.p826);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) {
            s.store_scale(2533, 2526, s.v[419]);
        }

        s.b[2620] = (p.p840 == 0.0);
        s.v[2620] = if s.b[2620] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) {
            s.store_div_scaled_inputs(2536, s.ad_value(2533), (s.v[404] * s.v[434]), s.ad_value(2529), 1.0);
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[431]), 2536);
            s.store_square(2538, 2537);
            s.store_sqrt_ad(2539, A::div_scaled_product_offset_denominator(s.ad_value(2538), s.ad_value(2538), 1.0, A::square(s.ad_value(2538)), 1.0, 1.0));
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
            s.store_sqrt_scaled_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_add_scaled_product(2545, s.ad_value(2539), (-1.0), s.ad_value(2537), s.ad_value(2540), 2.0);
            s.store_ad_value(2546, A::add_scaled_value_products(s.ad_value(2539), (-s.v[431]), s.ad_value(2537), s.ad_value(2540), s.v[431], s.ad_value(2536), s.ad_value(2541), 0.5));
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2623] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.v[2623] = if s.b[2623] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2623]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2623])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2624] = (s.v[2547] > 0.0);
        s.v[2624] = if s.b[2624] { 1.0 } else { 0.0 };

        s.b[2625] = (s.v[2546] > (-230.25850929940458));
        s.v[2625] = if s.b[2625] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2624])) && s.b[2625]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2624])) && (!s.b[2625])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2546), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2546), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2626] = (p.p846 == 0.0);
        s.v[2626] = if s.b[2626] { 1.0 } else { 0.0 };

        s.b[2627] = (p.p826 == 0.5);
        s.v[2627] = if s.b[2627] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && s.b[2627]) {
            s.store_sqrt_scaled_ad(2526, A::sub_from_scalar(p.p823, s.ad_value(2524)), s.v[425]);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2627])) {
            s.store_powf_ad(2526, A::scale_offset(s.ad_value(2524), (-s.v[425]), ((p.p823) * (s.v[425]))), p.p826);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) {
            s.store_ad_value(2551, A::div_scaled_offset_numerator(s.ad_value(2524), ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), s.ad_value(2526), 1.0));
        }

        s.b[2628] = (((((-s.v[437]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.v[2628] = if s.b[2628] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && s.b[2628]) {
            s.store_exp_ad(2526, A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2629] = (((-s.v[437]) / s.v[2551]) < 0.0);
        s.v[2629] = if s.b[2629] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2628])) && s.b[2629]) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2628])) && (!s.b[2629])) {
            let assign58500_ad_e74852: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2526, assign58500_ad_e74852, 1e100);
        }

        s.b[2630] = (s.v[634] > 1000.0);
        s.v[2630] = if s.b[2630] { 1.0 } else { 0.0 };

        s.b[2631] = (s.v[2525] > ((-s.v[438]) * s.v[634]));
        s.v[2631] = if s.b[2631] { 1.0 } else { 0.0 };

        s.b[2632] = (p.p858 == 4.0);
        s.v[2632] = if s.b[2632] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && s.b[2631]) && s.b[2632]) {
            s.store_mul_ad_product_lhs(2526, A::mul3(A::mul3(s.ad_value(2525), s.ad_value(444), A::mul(s.ad_value(2525), s.ad_value(444))), s.ad_value(2525), s.ad_value(444)), s.ad_value(2525), 444);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && s.b[2631]) && (!s.b[2632])) {
            s.store_powf_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(444))), p.p858);
        }

        s.b[2633] = (s.v[467] == 1.0);
        s.v[2633] = if s.b[2633] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            let assign58620_ad_e75078: A = {
                if (s.v[826] < p.p863) {
                    {
                        if (((s.v[826] - p.p863) / p.p864) < (-37.0)) {
                            A::constant(p.p863)
                        } else {
                            A::scale_offset(A::ln_one_plus_exp(A::scaled_offset(s.ad_value(826), (-p.p863), 1.0 / (p.p864))), p.p864, p.p863)
                        }
                    }
                } else {
                    {
                        if (((s.v[826] - p.p863) / p.p864) > 37.0) {
                            s.ad_value(826)
                        } else {
                            A::add_scaled_inputs(s.ad_value(826), 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(826), (-1.0 / (p.p864)), ((p.p863) * (1.0 / (p.p864))))), p.p864)
                        }
                    }
                }
            };
            s.store_ad_value(2553, assign58620_ad_e75078);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            s.store_scaled_mul(2511, 651, 651, 4.0);
            s.store_div(2512, 651, 652);
            s.store_add_scaled_product(2513, s.ad_value(2553), 1.0, s.ad_value(651), s.ad_value(2512), 1.0);
            s.store_add(2514, 652, 2513);
            s.store_sub(2515, 652, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product(2554, s.ad_value(2553), s.ad_value(652), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0);
        }

        s.b[2634] = (s.v[404] == 0.5);
        s.v[2634] = if s.b[2634] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && s.b[2634]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2554), s.v[401]));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && (!s.b[2634])) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2554), s.v[401])), s.v[404]);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            s.store_add_scaled_inputs3_offset(1908, s.ad_value(2526), ((-s.v[413]) * p.p30), s.ad_value(2553), (s.v[416] * p.p30), s.ad_value(2554), ((-s.v[416]) * p.p30), (s.v[413] * p.p30));
            s.store_sub_ad_lhs(2553, A::offset(s.ad_value(826), p.p863), 2553);
            s.store_scaled_mul(2511, 651, 651, 4.0);
            s.store_div(2512, 651, 652);
            s.store_add_scaled_product(2513, s.ad_value(2553), 1.0, s.ad_value(651), s.ad_value(2512), 1.0);
            s.store_add(2514, 652, 2513);
            s.store_sub(2515, 652, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product(2554, s.ad_value(2553), s.ad_value(652), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0);
        }

        s.b[2635] = (s.v[461] == 0.5);
        s.v[2635] = if s.b[2635] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && s.b[2635]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2554), s.ad_value(460)));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && (!s.b[2635])) {
            s.store_pow_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(460))), s.ad_value(461));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            s.store_add_scaled_product(466, A::mul_sub_from_scalar_rhs(s.ad_value(464), 1.0, s.ad_value(2526)), p.p30, s.ad_value(465), A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30);
            s.store_add(1908, 1908, 466);
        }

        s.b[2636] = (s.v[404] == 0.5);
        s.v[2636] = if s.b[2636] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) && s.b[2636]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[401]));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) && (!s.b[2636])) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[401])), s.v[404]);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) {
            s.store_add_scaled_inputs3_offset(1908, s.ad_value(2526), ((-s.v[413]) * p.p30), s.ad_value(826), (s.v[416] * p.p30), s.ad_value(2518), ((-s.v[416]) * p.p30), (s.v[413] * p.p30));
        }

        s.b[2637] = (s.v[630] > 0.0);
        s.v[2637] = if s.b[2637] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2637]) {
            s.store_mul_sub_ad_rhs(637, 630, A::pow(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001))), 0.5), s.ad_value(631)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(631)));
            s.store_add(635, 536, 637);
            s.store_div_from_scalar(610, 1.0, 635);
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2637])) {
            s.copy_ad(635, 536);
        }

        s.b[2638] = (s.v[632] > 0.0);
        s.v[2638] = if s.b[2638] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2638]) {
            s.store_mul_sub_ad_rhs(639, 632, A::pow(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001))), 0.5), s.ad_value(633)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(633)));
            s.store_mul_offset_rhs(604, 604, 639, 1.0);
        }

        if (s.b[2555] && (!s.b[2556])) {
            s.store_scalar(2524, 0.0);
            s.store_scalar(2521, 0.0);
        }

        s.b[2639] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));
        s.v[2639] = if s.b[2639] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {
            s.store_scaled_mul(2511, 678, 678, 4.0);
            s.store_div(2512, 678, 679);
            s.store_add_scaled_product(2513, s.ad_value(827), 1.0, s.ad_value(678), s.ad_value(2512), 1.0);
            s.store_add(2514, 679, 2513);
            s.store_sub(2515, 679, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product(2518, s.ad_value(827), s.ad_value(679), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0);
        }

        s.b[2640] = (s.v[827] < s.v[675]);
        s.v[2640] = if s.b[2640] { 1.0 } else { 0.0 };

        s.b[2641] = (((((-0.5) * (s.v[827] * s.v[365]))) as f64).abs() < 230.25850929940458);
        s.v[2641] = if s.b[2641] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) && s.b[2641]) {
            s.store_exp_scaled_input(2519, 827, (s.v[365] * (-0.5)));
        }

        s.b[2642] = (((-0.5) * (s.v[827] * s.v[365])) < 0.0);
        s.v[2642] = if s.b[2642] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) && (!s.b[2641])) && s.b[2642]) {
            s.store_div_from_scalar_offset_ad(2519, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::scale(s.ad_value(827), (s.v[365] * (-0.5))), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::scale(s.ad_value(827), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) && (!s.b[2641])) && (!s.b[2642])) {
            s.store_scaled_offset_ad(2519, A::mul_offset_rhs(A::scale_offset(s.ad_value(827), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(827), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(827), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) {
            s.store_div_from_scalar(2520, 1.0, 2519);
            s.store_square(2517, 2520);
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && (!s.b[2640])) {
            s.store_mul_offset_ad_lhs(2517, A::sub_scaled_inputs(s.ad_value(827), s.v[365], s.ad_value(675), s.v[365]), 1.0, 676);
            s.store_sqrt(2520, 2517);
            s.store_div_from_scalar(2519, 1.0, 2520);
        }

        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {
            s.store_offset(2517, 2517, (-1.0));
        }

        s.b[2643] = (s.v[827] > 0.0);
        s.v[2643] = if s.b[2643] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2643]) {
            s.store_scaled_ln_ad(2521, A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2519), 1.0, A::offset(s.ad_value(2519), 3.0)))), (s.v[364] * 2.0));
        }

        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && (!s.b[2643])) {
            s.store_sub_ad_lhs(2521, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2520), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2520), 1.0, A::scale_offset(s.ad_value(2520), 3.0, 1.0))))), (s.v[364] * 2.0)), 827);
        }

        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {
            s.store_sub(2522, 677, 2521);
            s.store_add_scaled_inputs3(2523, s.ad_value(827), 0.5, s.ad_value(2522), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(827), s.ad_value(2522)), A::sub(s.ad_value(827), s.ad_value(2522))), ((4.0 * s.v[364]) * s.v[364]))), (-0.5));
            s.store_add_scaled_inputs3(2524, s.ad_value(827), 0.5, s.ad_value(680), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(827), s.ad_value(680)), A::sub(s.ad_value(827), s.ad_value(680))), ((4.0 * s.v[362]) * s.v[362]))), (-0.5));
            s.store_scaled_sub_ad_rhs(2525, 827, A::sqrt(A::offset(A::mul(s.ad_value(827), s.ad_value(827)), ((4.0 * 1e-6) * 1e-6))), 0.5);
        }

        s.b[2644] = (s.v[667] == 0.0);
        s.v[2644] = if s.b[2644] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2644]) {
            s.store_scalar(1909, 0.0);
        }

        s.b[2645] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));
        s.v[2645] = if s.b[2645] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) {
            s.store_sub(2529, 563, 2523);
        }

        s.b[2647] = (s.v[505] == 0.5);
        s.v[2647] = if s.b[2647] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) && s.b[2647]) {
            s.store_sqrt_mul(2526, 2529, 590);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) && (!s.b[2647])) {
            s.store_pow_ad(2526, A::mul(s.ad_value(2529), s.ad_value(590)), s.ad_value(505));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) {
            s.store_mul(2533, 584, 2526);
        }

    }

    pub(super) fn stamp_reactive_block_21(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2648] = (s.v[519] == 0.0);
        s.v[2648] = if s.b[2648] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) {
            s.store_mul_ad_rhs(2536, 599, A::div_scaled_product(s.ad_value(2533), s.ad_value(569), 1.0, s.ad_value(2529), 1.0));
            s.store_div_scaled_inputs(2537, s.ad_value(596), 0.666666666666667, s.ad_value(2536), 1.0);
            s.store_square(2538, 2537);
            s.store_sqrt_ad(2539, A::div_scaled_product_offset_denominator(s.ad_value(2538), s.ad_value(2538), 1.0, A::square(s.ad_value(2538)), 1.0, 1.0));
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
            s.store_sqrt_scaled_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_add_scaled_product(2545, s.ad_value(2539), (-1.0), s.ad_value(2537), s.ad_value(2540), 2.0);
            s.store_ad_value(2546, A::add_scaled_value_products(A::mul3(s.ad_value(596), s.ad_value(2537), s.ad_value(2540)), 1.0, s.ad_value(596), s.ad_value(2539), (-1.0), s.ad_value(2536), s.ad_value(2541), 0.5));
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2651] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.v[2651] = if s.b[2651] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && s.b[2651]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2651])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2652] = (s.v[2547] > 0.0);
        s.v[2652] = if s.b[2652] { 1.0 } else { 0.0 };

        s.b[2653] = (s.v[2546] > (-230.25850929940458));
        s.v[2653] = if s.b[2653] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2652])) && s.b[2653]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2652])) && (!s.b[2653])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2546), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2546), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2654] = (s.v[525] == 0.0);
        s.v[2654] = if s.b[2654] { 1.0 } else { 0.0 };

        s.b[2655] = (s.v[505] == 0.5);
        s.v[2655] = if s.b[2655] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && s.b[2655]) {
            s.store_sqrt_mul_ad(2526, A::sub(s.ad_value(502), s.ad_value(2524)), s.ad_value(590));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && (!s.b[2655])) {
            s.store_pow_ad(2526, A::mul(A::sub(s.ad_value(502), s.ad_value(2524)), s.ad_value(590)), s.ad_value(505));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) {
            s.store_mul_ad_rhs(2551, 572, A::div_scaled_product(A::sub(s.ad_value(502), s.ad_value(2524)), s.ad_value(587), 1.0, s.ad_value(2526), 1.0));
        }

        s.b[2656] = (((((-s.v[602]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.v[2656] = if s.b[2656] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && s.b[2656]) {
            s.store_exp_ad(2526, A::div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2657] = (((-s.v[602]) / s.v[2551]) < 0.0);
        s.v[2657] = if s.b[2657] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && (!s.b[2656])) && s.b[2657]) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && (!s.b[2656])) && (!s.b[2657])) {
            let assign59900_ad_e77252: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2526, assign59900_ad_e77252, 1e100);
        }

        s.b[2658] = (s.v[534] > 1000.0);
        s.v[2658] = if s.b[2658] { 1.0 } else { 0.0 };

        s.b[2659] = (s.v[2525] > ((-s.v[438]) * s.v[534]));
        s.v[2659] = if s.b[2659] { 1.0 } else { 0.0 };

        s.b[2660] = (s.v[537] == 4.0);
        s.v[2660] = if s.b[2660] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2658])) && s.b[2659]) && s.b[2660]) {
            s.store_mul_ad_product_lhs(2526, A::mul3(A::mul3(s.ad_value(2525), s.ad_value(608), A::mul(s.ad_value(2525), s.ad_value(608))), s.ad_value(2525), s.ad_value(608)), s.ad_value(2525), 608);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2658])) && s.b[2659]) && (!s.b[2660])) {
            s.store_pow_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(608))), s.ad_value(537));
        }

        s.b[2661] = (s.v[569] == 0.5);
        s.v[2661] = if s.b[2661] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && s.b[2661]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2518), s.ad_value(566)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2661])) {
            s.store_pow_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(566))), s.ad_value(569));
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) {
            s.store_add_scaled_product(1909, A::mul_sub_from_scalar_rhs(s.ad_value(578), 1.0, s.ad_value(2526)), p.p30, s.ad_value(581), A::sub(s.ad_value(827), s.ad_value(2518)), p.p30);
        }

        s.b[2662] = (s.v[668] == 0.0);
        s.v[2662] = if s.b[2662] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2662]) {
            s.store_scalar(1910, 0.0);
        }

        s.b[2663] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));
        s.v[2663] = if s.b[2663] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) {
            s.store_sub(2529, 564, 2523);
        }

        s.b[2665] = (s.v[506] == 0.5);
        s.v[2665] = if s.b[2665] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) && s.b[2665]) {
            s.store_sqrt_mul(2526, 2529, 591);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) && (!s.b[2665])) {
            s.store_pow_ad(2526, A::mul(s.ad_value(2529), s.ad_value(591)), s.ad_value(506));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) {
            s.store_mul(2533, 585, 2526);
        }

        s.b[2666] = (s.v[520] == 0.0);
        s.v[2666] = if s.b[2666] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) {
            s.store_mul_ad_rhs(2536, 600, A::div_scaled_product(s.ad_value(2533), s.ad_value(570), 1.0, s.ad_value(2529), 1.0));
            s.store_div_scaled_inputs(2537, s.ad_value(597), 0.666666666666667, s.ad_value(2536), 1.0);
            s.store_square(2538, 2537);
            s.store_sqrt_ad(2539, A::div_scaled_product_offset_denominator(s.ad_value(2538), s.ad_value(2538), 1.0, A::square(s.ad_value(2538)), 1.0, 1.0));
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
            s.store_sqrt_scaled_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_add_scaled_product(2545, s.ad_value(2539), (-1.0), s.ad_value(2537), s.ad_value(2540), 2.0);
            s.store_ad_value(2546, A::add_scaled_value_products(A::mul3(s.ad_value(597), s.ad_value(2537), s.ad_value(2540)), 1.0, s.ad_value(597), s.ad_value(2539), (-1.0), s.ad_value(2536), s.ad_value(2541), 0.5));
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2669] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.v[2669] = if s.b[2669] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2669]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2669])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2670] = (s.v[2547] > 0.0);
        s.v[2670] = if s.b[2670] { 1.0 } else { 0.0 };

        s.b[2671] = (s.v[2546] > (-230.25850929940458));
        s.v[2671] = if s.b[2671] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2670])) && s.b[2671]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2670])) && (!s.b[2671])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2546), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2546), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2672] = (s.v[526] == 0.0);
        s.v[2672] = if s.b[2672] { 1.0 } else { 0.0 };

        s.b[2673] = (s.v[506] == 0.5);
        s.v[2673] = if s.b[2673] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && s.b[2673]) {
            s.store_sqrt_mul_ad(2526, A::sub(s.ad_value(503), s.ad_value(2524)), s.ad_value(591));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && (!s.b[2673])) {
            s.store_pow_ad(2526, A::mul(A::sub(s.ad_value(503), s.ad_value(2524)), s.ad_value(591)), s.ad_value(506));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) {
            s.store_mul_ad_rhs(2551, 573, A::div_scaled_product(A::sub(s.ad_value(503), s.ad_value(2524)), s.ad_value(588), 1.0, s.ad_value(2526), 1.0));
        }

        s.b[2674] = (((((-s.v[603]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.v[2674] = if s.b[2674] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && s.b[2674]) {
            s.store_exp_ad(2526, A::div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2675] = (((-s.v[603]) / s.v[2551]) < 0.0);
        s.v[2675] = if s.b[2675] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && (!s.b[2674])) && s.b[2675]) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && (!s.b[2674])) && (!s.b[2675])) {
            let assign60650_ad_e78518: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2526, assign60650_ad_e78518, 1e100);
        }

        s.b[2676] = (s.v[535] > 1000.0);
        s.v[2676] = if s.b[2676] { 1.0 } else { 0.0 };

        s.b[2677] = (s.v[2525] > ((-s.v[438]) * s.v[535]));
        s.v[2677] = if s.b[2677] { 1.0 } else { 0.0 };

        s.b[2678] = (s.v[538] == 4.0);
        s.v[2678] = if s.b[2678] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2676])) && s.b[2677]) && s.b[2678]) {
            s.store_mul_ad_product_lhs(2526, A::mul3(A::mul3(s.ad_value(2525), s.ad_value(609), A::mul(s.ad_value(2525), s.ad_value(609))), s.ad_value(2525), s.ad_value(609)), s.ad_value(2525), 609);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2676])) && s.b[2677]) && (!s.b[2678])) {
            s.store_pow_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(609))), s.ad_value(538));
        }

        s.b[2679] = (s.v[570] == 0.5);
        s.v[2679] = if s.b[2679] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && s.b[2679]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2518), s.ad_value(567)));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2679])) {
            s.store_pow_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(567))), s.ad_value(570));
        }

        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) {
            s.store_add_scaled_product(1910, A::mul_sub_from_scalar_rhs(s.ad_value(579), 1.0, s.ad_value(2526)), p.p30, s.ad_value(582), A::sub(s.ad_value(827), s.ad_value(2518)), p.p30);
        }

        s.b[2680] = (s.v[669] == 0.0);
        s.v[2680] = if s.b[2680] { 1.0 } else { 0.0 };

        if ((s.b[2555] && (!s.b[2556])) && s.b[2680]) {
            s.store_scalar(1911, 0.0);
        }

        s.b[2681] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));
        s.v[2681] = if s.b[2681] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) {
            s.store_sub(2529, 565, 2523);
        }

        s.b[2683] = (s.v[507] == 0.5);
        s.v[2683] = if s.b[2683] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) && s.b[2683]) {
            s.store_sqrt_mul(2526, 2529, 592);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) && (!s.b[2683])) {
            s.store_pow_ad(2526, A::mul(s.ad_value(2529), s.ad_value(592)), s.ad_value(507));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) {
            s.store_mul(2533, 586, 2526);
        }

        s.b[2684] = (s.v[521] == 0.0);
        s.v[2684] = if s.b[2684] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) {
            s.store_mul_ad_rhs(2536, 601, A::div_scaled_product(s.ad_value(2533), s.ad_value(571), 1.0, s.ad_value(2529), 1.0));
            s.store_div_scaled_inputs(2537, s.ad_value(598), 0.666666666666667, s.ad_value(2536), 1.0);
            s.store_square(2538, 2537);
            s.store_sqrt_ad(2539, A::div_scaled_product_offset_denominator(s.ad_value(2538), s.ad_value(2538), 1.0, A::square(s.ad_value(2538)), 1.0, 1.0));
            s.store_sqrt(2540, 2539);
            s.store_mul(2541, 2539, 2540);
            s.store_sqrt_scaled_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);
            s.store_add_scaled_product(2545, s.ad_value(2539), (-1.0), s.ad_value(2537), s.ad_value(2540), 2.0);
            s.store_ad_value(2546, A::add_scaled_value_products(A::mul3(s.ad_value(598), s.ad_value(2537), s.ad_value(2540)), 1.0, s.ad_value(598), s.ad_value(2539), (-1.0), s.ad_value(2536), s.ad_value(2541), 0.5));
            s.store_mul_offset_lhs(2547, 2545, (-1.0), 2544);
            s.store_square(2508, 2547);
        }

        s.b[2687] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));
        s.v[2687] = if s.b[2687] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2687]) {
            s.store_exp_sub(2526, 2546, 2508);
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2687])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2688] = (s.v[2547] > 0.0);
        s.v[2688] = if s.b[2688] { 1.0 } else { 0.0 };

        s.b[2689] = (s.v[2546] > (-230.25850929940458));
        s.v[2689] = if s.b[2689] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2688])) && s.b[2689]) {
            s.store_exp(2526, 2546);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2688])) && (!s.b[2689])) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), s.ad_value(2546), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), s.ad_value(2546), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        s.b[2690] = (s.v[527] == 0.0);
        s.v[2690] = if s.b[2690] { 1.0 } else { 0.0 };

        s.b[2691] = (s.v[507] == 0.5);
        s.v[2691] = if s.b[2691] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && s.b[2691]) {
            s.store_sqrt_mul_ad(2526, A::sub(s.ad_value(504), s.ad_value(2524)), s.ad_value(592));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && (!s.b[2691])) {
            s.store_pow_ad(2526, A::mul(A::sub(s.ad_value(504), s.ad_value(2524)), s.ad_value(592)), s.ad_value(507));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) {
            s.store_mul_ad_rhs(2551, 574, A::div_scaled_product(A::sub(s.ad_value(504), s.ad_value(2524)), s.ad_value(589), 1.0, s.ad_value(2526), 1.0));
        }

        s.b[2692] = (((((-s.v[604]) / s.v[2551])) as f64).abs() < 230.25850929940458);
        s.v[2692] = if s.b[2692] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && s.b[2692]) {
            s.store_exp_ad(2526, A::div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0));
        }

        s.b[2693] = (((-s.v[604]) / s.v[2551]) < 0.0);
        s.v[2693] = if s.b[2693] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && (!s.b[2692])) && s.b[2693]) {
            s.store_div_from_scalar_offset_ad(2526, 1e-100, A::mul_sub_from_scalar_lhs((-230.25850929940458), A::div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0), A::offset(A::mul_sub_from_scalar_scaled_offset_self((-230.25850929940458), A::div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, 1.0, 0.5), 1.0)), 1.0);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && (!s.b[2692])) && (!s.b[2693])) {
            let assign61400_ad_e79784: A = A::offset(A::mul_offset_lhs(A::div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::offset(A::mul_offset_lhs_scaled_output(A::div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0), (-230.25850929940458), A::scale_offset(A::div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0)), 1.0);
            s.store_scale_ad(2526, assign61400_ad_e79784, 1e100);
        }

        s.b[2694] = (s.v[635] > 1000.0);
        s.v[2694] = if s.b[2694] { 1.0 } else { 0.0 };

        s.b[2695] = (s.v[2525] > ((-s.v[438]) * s.v[635]));
        s.v[2695] = if s.b[2695] { 1.0 } else { 0.0 };

        s.b[2696] = (s.v[539] == 4.0);
        s.v[2696] = if s.b[2696] { 1.0 } else { 0.0 };

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2694])) && s.b[2695]) && s.b[2696]) {
            s.store_mul_ad_product_lhs(2526, A::mul3(A::mul3(s.ad_value(2525), s.ad_value(610), A::mul(s.ad_value(2525), s.ad_value(610))), s.ad_value(2525), s.ad_value(610)), s.ad_value(2525), 610);
        }

        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2694])) && s.b[2695]) && (!s.b[2696])) {
            s.store_pow_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(610))), s.ad_value(539));
        }

        s.b[2697] = (s.v[629] == 1.0);
        s.v[2697] = if s.b[2697] { 1.0 } else { 0.0 };

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            let assign61520_ad_e80010: A = {
                if (s.v[827] < s.v[544]) {
                    {
                        if (((s.v[827] - s.v[544]) / s.v[545]) < (-37.0)) {
                            s.ad_value(544)
                        } else {
                            A::add_scaled_product(s.ad_value(544), 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(827), 1.0, s.ad_value(544), (-1.0), s.ad_value(545), 1.0)), s.ad_value(545), 1.0)
                        }
                    }
                } else {
                    {
                        if (((s.v[827] - s.v[544]) / s.v[545]) > 37.0) {
                            s.ad_value(827)
                        } else {
                            A::add_scaled_product(s.ad_value(827), 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(544), 1.0, s.ad_value(827), (-1.0), s.ad_value(545), 1.0)), s.ad_value(545), 1.0)
                        }
                    }
                }
            };
            s.store_ad_value(2553, assign61520_ad_e80010);
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            s.store_scaled_mul(2511, 678, 678, 4.0);
            s.store_div(2512, 678, 679);
            s.store_add_scaled_product(2513, s.ad_value(2553), 1.0, s.ad_value(678), s.ad_value(2512), 1.0);
            s.store_add(2514, 679, 2513);
            s.store_sub(2515, 679, 2513);
        }

    }

    pub(super) fn stamp_reactive_block_22(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product(2554, s.ad_value(2553), s.ad_value(679), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0);
        }

        s.b[2698] = (s.v[571] == 0.5);
        s.v[2698] = if s.b[2698] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && s.b[2698]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2554), s.ad_value(568)));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && (!s.b[2698])) {
            s.store_pow_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(568))), s.ad_value(571));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            s.store_add_scaled_product(1911, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(2526)), p.p30, s.ad_value(583), A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30);
            s.store_add_scaled_inputs3(2553, s.ad_value(827), 1.0, s.ad_value(544), 1.0, s.ad_value(2553), -1.0);
            s.store_scaled_mul(2511, 678, 678, 4.0);
            s.store_div(2512, 678, 679);
            s.store_add_scaled_product(2513, s.ad_value(2553), 1.0, s.ad_value(678), s.ad_value(2512), 1.0);
            s.store_add(2514, 679, 2513);
            s.store_sub(2515, 679, 2513);
            s.store_sqrt_square_add(2516, 2515, 2511);
            s.store_div_scaled_product(2554, s.ad_value(2553), s.ad_value(679), 2.0, A::add(s.ad_value(2514), s.ad_value(2516)), 1.0);
        }

        s.b[2699] = (s.v[624] == 0.5);
        s.v[2699] = if s.b[2699] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && s.b[2699]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2554), s.ad_value(623)));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && (!s.b[2699])) {
            s.store_pow_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(623))), s.ad_value(624));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            s.store_add_scaled_product(466, A::mul_sub_from_scalar_rhs(s.ad_value(627), 1.0, s.ad_value(2526)), p.p30, s.ad_value(628), A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30);
            s.store_add(1911, 1911, 466);
        }

        s.b[2700] = (s.v[571] == 0.5);
        s.v[2700] = if s.b[2700] { 1.0 } else { 0.0 };

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2697])) && s.b[2700]) {
            s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2518), s.ad_value(568)));
        }

        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2697])) && (!s.b[2700])) {
            s.store_pow_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(568))), s.ad_value(571));
        }

        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2697])) {
            s.store_add_scaled_product(1911, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(2526)), p.p30, s.ad_value(583), A::sub(s.ad_value(827), s.ad_value(2518)), p.p30);
        }

        s.store_neg_ad(844, A::add_scaled_inputs3(s.ad_value(845), 1.0, s.ad_value(846), 1.0, s.ad_value(847), 1.0));

        s.store_add(848, 848, 1898);

        s.store_add(849, 849, 1899);

        s.store_add_scaled_products3(851, s.ad_value(640), s.ad_value(1906), 1.0, s.ad_value(641), s.ad_value(1907), 1.0, s.ad_value(642), s.ad_value(1908), 1.0);

        s.store_add_scaled_products3(852, s.ad_value(667), s.ad_value(1909), 1.0, s.ad_value(668), s.ad_value(1910), 1.0, s.ad_value(669), s.ad_value(1911), 1.0);

        s.b[2710] = (s.v[825] < 0.0);
        s.v[2710] = if s.b[2710] { 1.0 } else { 0.0 };

        if s.b[2710] {
            s.copy_ad(2709, 847);
            s.copy_ad(847, 844);
            s.copy_ad(844, 2709);
        }

        s.store_mul(854, 1892, 1883);

        s.b[2743] = ((s.v[1817] > 0.0) && (s.v[710] > 0.0));
        s.v[2743] = if s.b[2743] { 1.0 } else { 0.0 };

        s.b[2748] = ((((p.p50 == 1.0) && (s.v[713] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));
        s.v[2748] = if s.b[2748] { 1.0 } else { 0.0 };

        if (s.b[2743] && s.b[2748]) {
            s.store_ad_value(854, A::div_scaled_product3(A::square(s.ad_value(1896)), s.ad_value(1892), s.ad_value(1883), 1.0, A::square(s.ad_value(1894)), 1.0));
        }

        s.b[2752] = (((p.p46 != 0.0) && (s.v[282] > 0.0)) && (s.v[1868] > 0.0));
        s.v[2752] = if s.b[2752] { 1.0 } else { 0.0 };

        if s.b[2752] {
            s.store_div_scaled_inputs(1920, s.ad_value(1871), 4.0, s.ad_value(718), 1.0);
            s.store_scale(1920, 765, s.v[709]);
            s.store_mul(1920, 1852, 1865);
        }

    }

    pub(super) fn stamp_transient_equations_block_0(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq0_e948, eq0_e948_d_n0, eq0_e948_d_n1, eq0_e948_d_n2, eq0_e948_d_n3, eq0_e948_d_n4, eq0_e948_d_n5, eq0_e948_d_n6, eq0_e948_d_n7, eq0_e948_d_n8, eq0_e948_d_n9, eq0_e948_d_n10, eq0_e948_d_n11, eq0_e948_d_b0, eq0_e948_d_b1, eq0_e948_d_b2, eq0_e948_d_b3, eq0_e948_d_b4, eq0_e948_d_b5, eq0_e948_d_b6,) = {
    if s.b[2701] {
        let eq0_e942: f64 = (s.v[0] * s.v[15]);
        let eq0_e942_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq0_e942_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq0_e942_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq0_e942_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq0_e942_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq0_e942_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq0_e942_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq0_e942_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq0_e942_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq0_e942_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq0_e942_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq0_e942_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq0_e942_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq0_e942_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq0_e942_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq0_e942_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq0_e942_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq0_e942_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq0_e942_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq0_e944: f64 = (eq0_e942 * p.p32);
        let eq0_e944_d_n0: f64 = (eq0_e942_d_n0 * p.p32);
        let eq0_e944_d_n1: f64 = (eq0_e942_d_n1 * p.p32);
        let eq0_e944_d_n2: f64 = (eq0_e942_d_n2 * p.p32);
        let eq0_e944_d_n3: f64 = (eq0_e942_d_n3 * p.p32);
        let eq0_e944_d_n4: f64 = (eq0_e942_d_n4 * p.p32);
        let eq0_e944_d_n5: f64 = (eq0_e942_d_n5 * p.p32);
        let eq0_e944_d_n6: f64 = (eq0_e942_d_n6 * p.p32);
        let eq0_e944_d_n7: f64 = (eq0_e942_d_n7 * p.p32);
        let eq0_e944_d_n8: f64 = (eq0_e942_d_n8 * p.p32);
        let eq0_e944_d_n9: f64 = (eq0_e942_d_n9 * p.p32);
        let eq0_e944_d_n10: f64 = (eq0_e942_d_n10 * p.p32);
        let eq0_e944_d_n11: f64 = (eq0_e942_d_n11 * p.p32);
        let eq0_e944_d_b0: f64 = (eq0_e942_d_b0 * p.p32);
        let eq0_e944_d_b1: f64 = (eq0_e942_d_b1 * p.p32);
        let eq0_e944_d_b2: f64 = (eq0_e942_d_b2 * p.p32);
        let eq0_e944_d_b3: f64 = (eq0_e942_d_b3 * p.p32);
        let eq0_e944_d_b4: f64 = (eq0_e942_d_b4 * p.p32);
        let eq0_e944_d_b5: f64 = (eq0_e942_d_b5 * p.p32);
        let eq0_e944_d_b6: f64 = (eq0_e942_d_b6 * p.p32);
        let eq0_e946: f64 = (eq0_e944 * s.v[841]);
        let eq0_e946_d_n0: f64 = ((eq0_e944_d_n0 * s.v[841]) + (eq0_e944 * s.dn[841][0]));
        let eq0_e946_d_n1: f64 = ((eq0_e944_d_n1 * s.v[841]) + (eq0_e944 * s.dn[841][1]));
        let eq0_e946_d_n2: f64 = ((eq0_e944_d_n2 * s.v[841]) + (eq0_e944 * s.dn[841][2]));
        let eq0_e946_d_n3: f64 = ((eq0_e944_d_n3 * s.v[841]) + (eq0_e944 * s.dn[841][3]));
        let eq0_e946_d_n4: f64 = ((eq0_e944_d_n4 * s.v[841]) + (eq0_e944 * s.dn[841][4]));
        let eq0_e946_d_n5: f64 = ((eq0_e944_d_n5 * s.v[841]) + (eq0_e944 * s.dn[841][5]));
        let eq0_e946_d_n6: f64 = ((eq0_e944_d_n6 * s.v[841]) + (eq0_e944 * s.dn[841][6]));
        let eq0_e946_d_n7: f64 = ((eq0_e944_d_n7 * s.v[841]) + (eq0_e944 * s.dn[841][7]));
        let eq0_e946_d_n8: f64 = ((eq0_e944_d_n8 * s.v[841]) + (eq0_e944 * s.dn[841][8]));
        let eq0_e946_d_n9: f64 = ((eq0_e944_d_n9 * s.v[841]) + (eq0_e944 * s.dn[841][9]));
        let eq0_e946_d_n10: f64 = ((eq0_e944_d_n10 * s.v[841]) + (eq0_e944 * s.dn[841][10]));
        let eq0_e946_d_n11: f64 = ((eq0_e944_d_n11 * s.v[841]) + (eq0_e944 * s.dn[841][11]));
        let eq0_e946_d_b0: f64 = ((eq0_e944_d_b0 * s.v[841]) + (eq0_e944 * s.db[841][0]));
        let eq0_e946_d_b1: f64 = ((eq0_e944_d_b1 * s.v[841]) + (eq0_e944 * s.db[841][1]));
        let eq0_e946_d_b2: f64 = ((eq0_e944_d_b2 * s.v[841]) + (eq0_e944 * s.db[841][2]));
        let eq0_e946_d_b3: f64 = ((eq0_e944_d_b3 * s.v[841]) + (eq0_e944 * s.db[841][3]));
        let eq0_e946_d_b4: f64 = ((eq0_e944_d_b4 * s.v[841]) + (eq0_e944 * s.db[841][4]));
        let eq0_e946_d_b5: f64 = ((eq0_e944_d_b5 * s.v[841]) + (eq0_e944 * s.db[841][5]));
        let eq0_e946_d_b6: f64 = ((eq0_e944_d_b6 * s.v[841]) + (eq0_e944 * s.db[841][6]));
        (eq0_e946, eq0_e946_d_n0, eq0_e946_d_n1, eq0_e946_d_n2, eq0_e946_d_n3, eq0_e946_d_n4, eq0_e946_d_n5, eq0_e946_d_n6, eq0_e946_d_n7, eq0_e946_d_n8, eq0_e946_d_n9, eq0_e946_d_n10, eq0_e946_d_n11, eq0_e946_d_b0, eq0_e946_d_b1, eq0_e946_d_b2, eq0_e946_d_b3, eq0_e946_d_b4, eq0_e946_d_b5, eq0_e946_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq0_value: f64 = eq0_e948;
        let eq0_node_derivatives: [f64; 12] = [eq0_e948_d_n0, eq0_e948_d_n1, eq0_e948_d_n2, eq0_e948_d_n3, eq0_e948_d_n4, eq0_e948_d_n5, eq0_e948_d_n6, eq0_e948_d_n7, eq0_e948_d_n8, eq0_e948_d_n9, eq0_e948_d_n10, eq0_e948_d_n11];
        let eq0_branch_derivatives: [f64; 7] = [eq0_e948_d_b0, eq0_e948_d_b1, eq0_e948_d_b2, eq0_e948_d_b3, eq0_e948_d_b4, eq0_e948_d_b5, eq0_e948_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq0_value),
            &eq0_node_derivatives,
            &eq0_branch_derivatives,
            multiplicity,
        );
        let (eq1_e960, eq1_e960_d_n0, eq1_e960_d_n1, eq1_e960_d_n2, eq1_e960_d_n3, eq1_e960_d_n4, eq1_e960_d_n5, eq1_e960_d_n6, eq1_e960_d_n7, eq1_e960_d_n8, eq1_e960_d_n9, eq1_e960_d_n10, eq1_e960_d_n11, eq1_e960_d_b0, eq1_e960_d_b1, eq1_e960_d_b2, eq1_e960_d_b3, eq1_e960_d_b4, eq1_e960_d_b5, eq1_e960_d_b6,) = {
    if s.b[2701] {
        let eq1_e952: f64 = (s.v[0] * s.v[15]);
        let eq1_e952_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq1_e952_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq1_e952_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq1_e952_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq1_e952_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq1_e952_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq1_e952_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq1_e952_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq1_e952_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq1_e952_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq1_e952_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq1_e952_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq1_e952_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq1_e952_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq1_e952_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq1_e952_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq1_e952_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq1_e952_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq1_e952_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq1_e954: f64 = (eq1_e952 * p.p32);
        let eq1_e954_d_n0: f64 = (eq1_e952_d_n0 * p.p32);
        let eq1_e954_d_n1: f64 = (eq1_e952_d_n1 * p.p32);
        let eq1_e954_d_n2: f64 = (eq1_e952_d_n2 * p.p32);
        let eq1_e954_d_n3: f64 = (eq1_e952_d_n3 * p.p32);
        let eq1_e954_d_n4: f64 = (eq1_e952_d_n4 * p.p32);
        let eq1_e954_d_n5: f64 = (eq1_e952_d_n5 * p.p32);
        let eq1_e954_d_n6: f64 = (eq1_e952_d_n6 * p.p32);
        let eq1_e954_d_n7: f64 = (eq1_e952_d_n7 * p.p32);
        let eq1_e954_d_n8: f64 = (eq1_e952_d_n8 * p.p32);
        let eq1_e954_d_n9: f64 = (eq1_e952_d_n9 * p.p32);
        let eq1_e954_d_n10: f64 = (eq1_e952_d_n10 * p.p32);
        let eq1_e954_d_n11: f64 = (eq1_e952_d_n11 * p.p32);
        let eq1_e954_d_b0: f64 = (eq1_e952_d_b0 * p.p32);
        let eq1_e954_d_b1: f64 = (eq1_e952_d_b1 * p.p32);
        let eq1_e954_d_b2: f64 = (eq1_e952_d_b2 * p.p32);
        let eq1_e954_d_b3: f64 = (eq1_e952_d_b3 * p.p32);
        let eq1_e954_d_b4: f64 = (eq1_e952_d_b4 * p.p32);
        let eq1_e954_d_b5: f64 = (eq1_e952_d_b5 * p.p32);
        let eq1_e954_d_b6: f64 = (eq1_e952_d_b6 * p.p32);
        let eq1_e957: f64 = (s.v[832] + s.v[840]);
        let eq1_e957_d_n0: f64 = (s.dn[832][0] + s.dn[840][0]);
        let eq1_e957_d_n1: f64 = (s.dn[832][1] + s.dn[840][1]);
        let eq1_e957_d_n2: f64 = (s.dn[832][2] + s.dn[840][2]);
        let eq1_e957_d_n3: f64 = (s.dn[832][3] + s.dn[840][3]);
        let eq1_e957_d_n4: f64 = (s.dn[832][4] + s.dn[840][4]);
        let eq1_e957_d_n5: f64 = (s.dn[832][5] + s.dn[840][5]);
        let eq1_e957_d_n6: f64 = (s.dn[832][6] + s.dn[840][6]);
        let eq1_e957_d_n7: f64 = (s.dn[832][7] + s.dn[840][7]);
        let eq1_e957_d_n8: f64 = (s.dn[832][8] + s.dn[840][8]);
        let eq1_e957_d_n9: f64 = (s.dn[832][9] + s.dn[840][9]);
        let eq1_e957_d_n10: f64 = (s.dn[832][10] + s.dn[840][10]);
        let eq1_e957_d_n11: f64 = (s.dn[832][11] + s.dn[840][11]);
        let eq1_e957_d_b0: f64 = (s.db[832][0] + s.db[840][0]);
        let eq1_e957_d_b1: f64 = (s.db[832][1] + s.db[840][1]);
        let eq1_e957_d_b2: f64 = (s.db[832][2] + s.db[840][2]);
        let eq1_e957_d_b3: f64 = (s.db[832][3] + s.db[840][3]);
        let eq1_e957_d_b4: f64 = (s.db[832][4] + s.db[840][4]);
        let eq1_e957_d_b5: f64 = (s.db[832][5] + s.db[840][5]);
        let eq1_e957_d_b6: f64 = (s.db[832][6] + s.db[840][6]);
        let eq1_e958: f64 = (eq1_e954 * eq1_e957);
        let eq1_e958_d_n0: f64 = ((eq1_e954_d_n0 * eq1_e957) + (eq1_e954 * eq1_e957_d_n0));
        let eq1_e958_d_n1: f64 = ((eq1_e954_d_n1 * eq1_e957) + (eq1_e954 * eq1_e957_d_n1));
        let eq1_e958_d_n2: f64 = ((eq1_e954_d_n2 * eq1_e957) + (eq1_e954 * eq1_e957_d_n2));
        let eq1_e958_d_n3: f64 = ((eq1_e954_d_n3 * eq1_e957) + (eq1_e954 * eq1_e957_d_n3));
        let eq1_e958_d_n4: f64 = ((eq1_e954_d_n4 * eq1_e957) + (eq1_e954 * eq1_e957_d_n4));
        let eq1_e958_d_n5: f64 = ((eq1_e954_d_n5 * eq1_e957) + (eq1_e954 * eq1_e957_d_n5));
        let eq1_e958_d_n6: f64 = ((eq1_e954_d_n6 * eq1_e957) + (eq1_e954 * eq1_e957_d_n6));
        let eq1_e958_d_n7: f64 = ((eq1_e954_d_n7 * eq1_e957) + (eq1_e954 * eq1_e957_d_n7));
        let eq1_e958_d_n8: f64 = ((eq1_e954_d_n8 * eq1_e957) + (eq1_e954 * eq1_e957_d_n8));
        let eq1_e958_d_n9: f64 = ((eq1_e954_d_n9 * eq1_e957) + (eq1_e954 * eq1_e957_d_n9));
        let eq1_e958_d_n10: f64 = ((eq1_e954_d_n10 * eq1_e957) + (eq1_e954 * eq1_e957_d_n10));
        let eq1_e958_d_n11: f64 = ((eq1_e954_d_n11 * eq1_e957) + (eq1_e954 * eq1_e957_d_n11));
        let eq1_e958_d_b0: f64 = ((eq1_e954_d_b0 * eq1_e957) + (eq1_e954 * eq1_e957_d_b0));
        let eq1_e958_d_b1: f64 = ((eq1_e954_d_b1 * eq1_e957) + (eq1_e954 * eq1_e957_d_b1));
        let eq1_e958_d_b2: f64 = ((eq1_e954_d_b2 * eq1_e957) + (eq1_e954 * eq1_e957_d_b2));
        let eq1_e958_d_b3: f64 = ((eq1_e954_d_b3 * eq1_e957) + (eq1_e954 * eq1_e957_d_b3));
        let eq1_e958_d_b4: f64 = ((eq1_e954_d_b4 * eq1_e957) + (eq1_e954 * eq1_e957_d_b4));
        let eq1_e958_d_b5: f64 = ((eq1_e954_d_b5 * eq1_e957) + (eq1_e954 * eq1_e957_d_b5));
        let eq1_e958_d_b6: f64 = ((eq1_e954_d_b6 * eq1_e957) + (eq1_e954 * eq1_e957_d_b6));
        (eq1_e958, eq1_e958_d_n0, eq1_e958_d_n1, eq1_e958_d_n2, eq1_e958_d_n3, eq1_e958_d_n4, eq1_e958_d_n5, eq1_e958_d_n6, eq1_e958_d_n7, eq1_e958_d_n8, eq1_e958_d_n9, eq1_e958_d_n10, eq1_e958_d_n11, eq1_e958_d_b0, eq1_e958_d_b1, eq1_e958_d_b2, eq1_e958_d_b3, eq1_e958_d_b4, eq1_e958_d_b5, eq1_e958_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq1_value: f64 = eq1_e960;
        let eq1_node_derivatives: [f64; 12] = [eq1_e960_d_n0, eq1_e960_d_n1, eq1_e960_d_n2, eq1_e960_d_n3, eq1_e960_d_n4, eq1_e960_d_n5, eq1_e960_d_n6, eq1_e960_d_n7, eq1_e960_d_n8, eq1_e960_d_n9, eq1_e960_d_n10, eq1_e960_d_n11];
        let eq1_branch_derivatives: [f64; 7] = [eq1_e960_d_b0, eq1_e960_d_b1, eq1_e960_d_b2, eq1_e960_d_b3, eq1_e960_d_b4, eq1_e960_d_b5, eq1_e960_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq1_value),
            &eq1_node_derivatives,
            &eq1_branch_derivatives,
            multiplicity,
        );
        let (eq2_e970, eq2_e970_d_n0, eq2_e970_d_n1, eq2_e970_d_n2, eq2_e970_d_n3, eq2_e970_d_n4, eq2_e970_d_n5, eq2_e970_d_n6, eq2_e970_d_n7, eq2_e970_d_n8, eq2_e970_d_n9, eq2_e970_d_n10, eq2_e970_d_n11, eq2_e970_d_b0, eq2_e970_d_b1, eq2_e970_d_b2, eq2_e970_d_b3, eq2_e970_d_b4, eq2_e970_d_b5, eq2_e970_d_b6,) = {
    if s.b[2701] {
        let eq2_e964: f64 = (s.v[0] * s.v[15]);
        let eq2_e964_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq2_e964_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq2_e964_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq2_e964_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq2_e964_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq2_e964_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq2_e964_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq2_e964_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq2_e964_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq2_e964_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq2_e964_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq2_e964_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq2_e964_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq2_e964_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq2_e964_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq2_e964_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq2_e964_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq2_e964_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq2_e964_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq2_e966: f64 = (eq2_e964 * p.p32);
        let eq2_e966_d_n0: f64 = (eq2_e964_d_n0 * p.p32);
        let eq2_e966_d_n1: f64 = (eq2_e964_d_n1 * p.p32);
        let eq2_e966_d_n2: f64 = (eq2_e964_d_n2 * p.p32);
        let eq2_e966_d_n3: f64 = (eq2_e964_d_n3 * p.p32);
        let eq2_e966_d_n4: f64 = (eq2_e964_d_n4 * p.p32);
        let eq2_e966_d_n5: f64 = (eq2_e964_d_n5 * p.p32);
        let eq2_e966_d_n6: f64 = (eq2_e964_d_n6 * p.p32);
        let eq2_e966_d_n7: f64 = (eq2_e964_d_n7 * p.p32);
        let eq2_e966_d_n8: f64 = (eq2_e964_d_n8 * p.p32);
        let eq2_e966_d_n9: f64 = (eq2_e964_d_n9 * p.p32);
        let eq2_e966_d_n10: f64 = (eq2_e964_d_n10 * p.p32);
        let eq2_e966_d_n11: f64 = (eq2_e964_d_n11 * p.p32);
        let eq2_e966_d_b0: f64 = (eq2_e964_d_b0 * p.p32);
        let eq2_e966_d_b1: f64 = (eq2_e964_d_b1 * p.p32);
        let eq2_e966_d_b2: f64 = (eq2_e964_d_b2 * p.p32);
        let eq2_e966_d_b3: f64 = (eq2_e964_d_b3 * p.p32);
        let eq2_e966_d_b4: f64 = (eq2_e964_d_b4 * p.p32);
        let eq2_e966_d_b5: f64 = (eq2_e964_d_b5 * p.p32);
        let eq2_e966_d_b6: f64 = (eq2_e964_d_b6 * p.p32);
        let eq2_e968: f64 = (eq2_e966 * s.v[835]);
        let eq2_e968_d_n0: f64 = ((eq2_e966_d_n0 * s.v[835]) + (eq2_e966 * s.dn[835][0]));
        let eq2_e968_d_n1: f64 = ((eq2_e966_d_n1 * s.v[835]) + (eq2_e966 * s.dn[835][1]));
        let eq2_e968_d_n2: f64 = ((eq2_e966_d_n2 * s.v[835]) + (eq2_e966 * s.dn[835][2]));
        let eq2_e968_d_n3: f64 = ((eq2_e966_d_n3 * s.v[835]) + (eq2_e966 * s.dn[835][3]));
        let eq2_e968_d_n4: f64 = ((eq2_e966_d_n4 * s.v[835]) + (eq2_e966 * s.dn[835][4]));
        let eq2_e968_d_n5: f64 = ((eq2_e966_d_n5 * s.v[835]) + (eq2_e966 * s.dn[835][5]));
        let eq2_e968_d_n6: f64 = ((eq2_e966_d_n6 * s.v[835]) + (eq2_e966 * s.dn[835][6]));
        let eq2_e968_d_n7: f64 = ((eq2_e966_d_n7 * s.v[835]) + (eq2_e966 * s.dn[835][7]));
        let eq2_e968_d_n8: f64 = ((eq2_e966_d_n8 * s.v[835]) + (eq2_e966 * s.dn[835][8]));
        let eq2_e968_d_n9: f64 = ((eq2_e966_d_n9 * s.v[835]) + (eq2_e966 * s.dn[835][9]));
        let eq2_e968_d_n10: f64 = ((eq2_e966_d_n10 * s.v[835]) + (eq2_e966 * s.dn[835][10]));
        let eq2_e968_d_n11: f64 = ((eq2_e966_d_n11 * s.v[835]) + (eq2_e966 * s.dn[835][11]));
        let eq2_e968_d_b0: f64 = ((eq2_e966_d_b0 * s.v[835]) + (eq2_e966 * s.db[835][0]));
        let eq2_e968_d_b1: f64 = ((eq2_e966_d_b1 * s.v[835]) + (eq2_e966 * s.db[835][1]));
        let eq2_e968_d_b2: f64 = ((eq2_e966_d_b2 * s.v[835]) + (eq2_e966 * s.db[835][2]));
        let eq2_e968_d_b3: f64 = ((eq2_e966_d_b3 * s.v[835]) + (eq2_e966 * s.db[835][3]));
        let eq2_e968_d_b4: f64 = ((eq2_e966_d_b4 * s.v[835]) + (eq2_e966 * s.db[835][4]));
        let eq2_e968_d_b5: f64 = ((eq2_e966_d_b5 * s.v[835]) + (eq2_e966 * s.db[835][5]));
        let eq2_e968_d_b6: f64 = ((eq2_e966_d_b6 * s.v[835]) + (eq2_e966 * s.db[835][6]));
        (eq2_e968, eq2_e968_d_n0, eq2_e968_d_n1, eq2_e968_d_n2, eq2_e968_d_n3, eq2_e968_d_n4, eq2_e968_d_n5, eq2_e968_d_n6, eq2_e968_d_n7, eq2_e968_d_n8, eq2_e968_d_n9, eq2_e968_d_n10, eq2_e968_d_n11, eq2_e968_d_b0, eq2_e968_d_b1, eq2_e968_d_b2, eq2_e968_d_b3, eq2_e968_d_b4, eq2_e968_d_b5, eq2_e968_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq2_value: f64 = eq2_e970;
        let eq2_node_derivatives: [f64; 12] = [eq2_e970_d_n0, eq2_e970_d_n1, eq2_e970_d_n2, eq2_e970_d_n3, eq2_e970_d_n4, eq2_e970_d_n5, eq2_e970_d_n6, eq2_e970_d_n7, eq2_e970_d_n8, eq2_e970_d_n9, eq2_e970_d_n10, eq2_e970_d_n11];
        let eq2_branch_derivatives: [f64; 7] = [eq2_e970_d_b0, eq2_e970_d_b1, eq2_e970_d_b2, eq2_e970_d_b3, eq2_e970_d_b4, eq2_e970_d_b5, eq2_e970_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq2_value),
            &eq2_node_derivatives,
            &eq2_branch_derivatives,
            multiplicity,
        );
        let (eq3_e980, eq3_e980_d_n0, eq3_e980_d_n1, eq3_e980_d_n2, eq3_e980_d_n3, eq3_e980_d_n4, eq3_e980_d_n5, eq3_e980_d_n6, eq3_e980_d_n7, eq3_e980_d_n8, eq3_e980_d_n9, eq3_e980_d_n10, eq3_e980_d_n11, eq3_e980_d_b0, eq3_e980_d_b1, eq3_e980_d_b2, eq3_e980_d_b3, eq3_e980_d_b4, eq3_e980_d_b5, eq3_e980_d_b6,) = {
    if s.b[2701] {
        let eq3_e974: f64 = (s.v[0] * s.v[15]);
        let eq3_e974_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq3_e974_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq3_e974_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq3_e974_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq3_e974_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq3_e974_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq3_e974_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq3_e974_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq3_e974_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq3_e974_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq3_e974_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq3_e974_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq3_e974_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq3_e974_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq3_e974_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq3_e974_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq3_e974_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq3_e974_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq3_e974_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq3_e976: f64 = (eq3_e974 * p.p32);
        let eq3_e976_d_n0: f64 = (eq3_e974_d_n0 * p.p32);
        let eq3_e976_d_n1: f64 = (eq3_e974_d_n1 * p.p32);
        let eq3_e976_d_n2: f64 = (eq3_e974_d_n2 * p.p32);
        let eq3_e976_d_n3: f64 = (eq3_e974_d_n3 * p.p32);
        let eq3_e976_d_n4: f64 = (eq3_e974_d_n4 * p.p32);
        let eq3_e976_d_n5: f64 = (eq3_e974_d_n5 * p.p32);
        let eq3_e976_d_n6: f64 = (eq3_e974_d_n6 * p.p32);
        let eq3_e976_d_n7: f64 = (eq3_e974_d_n7 * p.p32);
        let eq3_e976_d_n8: f64 = (eq3_e974_d_n8 * p.p32);
        let eq3_e976_d_n9: f64 = (eq3_e974_d_n9 * p.p32);
        let eq3_e976_d_n10: f64 = (eq3_e974_d_n10 * p.p32);
        let eq3_e976_d_n11: f64 = (eq3_e974_d_n11 * p.p32);
        let eq3_e976_d_b0: f64 = (eq3_e974_d_b0 * p.p32);
        let eq3_e976_d_b1: f64 = (eq3_e974_d_b1 * p.p32);
        let eq3_e976_d_b2: f64 = (eq3_e974_d_b2 * p.p32);
        let eq3_e976_d_b3: f64 = (eq3_e974_d_b3 * p.p32);
        let eq3_e976_d_b4: f64 = (eq3_e974_d_b4 * p.p32);
        let eq3_e976_d_b5: f64 = (eq3_e974_d_b5 * p.p32);
        let eq3_e976_d_b6: f64 = (eq3_e974_d_b6 * p.p32);
        let eq3_e978: f64 = (eq3_e976 * s.v[836]);
        let eq3_e978_d_n0: f64 = ((eq3_e976_d_n0 * s.v[836]) + (eq3_e976 * s.dn[836][0]));
        let eq3_e978_d_n1: f64 = ((eq3_e976_d_n1 * s.v[836]) + (eq3_e976 * s.dn[836][1]));
        let eq3_e978_d_n2: f64 = ((eq3_e976_d_n2 * s.v[836]) + (eq3_e976 * s.dn[836][2]));
        let eq3_e978_d_n3: f64 = ((eq3_e976_d_n3 * s.v[836]) + (eq3_e976 * s.dn[836][3]));
        let eq3_e978_d_n4: f64 = ((eq3_e976_d_n4 * s.v[836]) + (eq3_e976 * s.dn[836][4]));
        let eq3_e978_d_n5: f64 = ((eq3_e976_d_n5 * s.v[836]) + (eq3_e976 * s.dn[836][5]));
        let eq3_e978_d_n6: f64 = ((eq3_e976_d_n6 * s.v[836]) + (eq3_e976 * s.dn[836][6]));
        let eq3_e978_d_n7: f64 = ((eq3_e976_d_n7 * s.v[836]) + (eq3_e976 * s.dn[836][7]));
        let eq3_e978_d_n8: f64 = ((eq3_e976_d_n8 * s.v[836]) + (eq3_e976 * s.dn[836][8]));
        let eq3_e978_d_n9: f64 = ((eq3_e976_d_n9 * s.v[836]) + (eq3_e976 * s.dn[836][9]));
        let eq3_e978_d_n10: f64 = ((eq3_e976_d_n10 * s.v[836]) + (eq3_e976 * s.dn[836][10]));
        let eq3_e978_d_n11: f64 = ((eq3_e976_d_n11 * s.v[836]) + (eq3_e976 * s.dn[836][11]));
        let eq3_e978_d_b0: f64 = ((eq3_e976_d_b0 * s.v[836]) + (eq3_e976 * s.db[836][0]));
        let eq3_e978_d_b1: f64 = ((eq3_e976_d_b1 * s.v[836]) + (eq3_e976 * s.db[836][1]));
        let eq3_e978_d_b2: f64 = ((eq3_e976_d_b2 * s.v[836]) + (eq3_e976 * s.db[836][2]));
        let eq3_e978_d_b3: f64 = ((eq3_e976_d_b3 * s.v[836]) + (eq3_e976 * s.db[836][3]));
        let eq3_e978_d_b4: f64 = ((eq3_e976_d_b4 * s.v[836]) + (eq3_e976 * s.db[836][4]));
        let eq3_e978_d_b5: f64 = ((eq3_e976_d_b5 * s.v[836]) + (eq3_e976 * s.db[836][5]));
        let eq3_e978_d_b6: f64 = ((eq3_e976_d_b6 * s.v[836]) + (eq3_e976 * s.db[836][6]));
        (eq3_e978, eq3_e978_d_n0, eq3_e978_d_n1, eq3_e978_d_n2, eq3_e978_d_n3, eq3_e978_d_n4, eq3_e978_d_n5, eq3_e978_d_n6, eq3_e978_d_n7, eq3_e978_d_n8, eq3_e978_d_n9, eq3_e978_d_n10, eq3_e978_d_n11, eq3_e978_d_b0, eq3_e978_d_b1, eq3_e978_d_b2, eq3_e978_d_b3, eq3_e978_d_b4, eq3_e978_d_b5, eq3_e978_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq3_value: f64 = eq3_e980;
        let eq3_node_derivatives: [f64; 12] = [eq3_e980_d_n0, eq3_e980_d_n1, eq3_e980_d_n2, eq3_e980_d_n3, eq3_e980_d_n4, eq3_e980_d_n5, eq3_e980_d_n6, eq3_e980_d_n7, eq3_e980_d_n8, eq3_e980_d_n9, eq3_e980_d_n10, eq3_e980_d_n11];
        let eq3_branch_derivatives: [f64; 7] = [eq3_e980_d_b0, eq3_e980_d_b1, eq3_e980_d_b2, eq3_e980_d_b3, eq3_e980_d_b4, eq3_e980_d_b5, eq3_e980_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq3_value),
            &eq3_node_derivatives,
            &eq3_branch_derivatives,
            multiplicity,
        );
        let (eq4_e991, eq4_e991_d_n0, eq4_e991_d_n1, eq4_e991_d_n2, eq4_e991_d_n3, eq4_e991_d_n4, eq4_e991_d_n5, eq4_e991_d_n6, eq4_e991_d_n7, eq4_e991_d_n8, eq4_e991_d_n9, eq4_e991_d_n10, eq4_e991_d_n11, eq4_e991_d_b0, eq4_e991_d_b1, eq4_e991_d_b2, eq4_e991_d_b3, eq4_e991_d_b4, eq4_e991_d_b5, eq4_e991_d_b6,) = {
    if (!s.b[2701]) {
        let eq4_e985: f64 = (s.v[0] * s.v[15]);
        let eq4_e985_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq4_e985_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq4_e985_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq4_e985_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq4_e985_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq4_e985_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq4_e985_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq4_e985_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq4_e985_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq4_e985_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq4_e985_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq4_e985_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq4_e985_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq4_e985_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq4_e985_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq4_e985_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq4_e985_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq4_e985_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq4_e985_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq4_e987: f64 = (eq4_e985 * p.p32);
        let eq4_e987_d_n0: f64 = (eq4_e985_d_n0 * p.p32);
        let eq4_e987_d_n1: f64 = (eq4_e985_d_n1 * p.p32);
        let eq4_e987_d_n2: f64 = (eq4_e985_d_n2 * p.p32);
        let eq4_e987_d_n3: f64 = (eq4_e985_d_n3 * p.p32);
        let eq4_e987_d_n4: f64 = (eq4_e985_d_n4 * p.p32);
        let eq4_e987_d_n5: f64 = (eq4_e985_d_n5 * p.p32);
        let eq4_e987_d_n6: f64 = (eq4_e985_d_n6 * p.p32);
        let eq4_e987_d_n7: f64 = (eq4_e985_d_n7 * p.p32);
        let eq4_e987_d_n8: f64 = (eq4_e985_d_n8 * p.p32);
        let eq4_e987_d_n9: f64 = (eq4_e985_d_n9 * p.p32);
        let eq4_e987_d_n10: f64 = (eq4_e985_d_n10 * p.p32);
        let eq4_e987_d_n11: f64 = (eq4_e985_d_n11 * p.p32);
        let eq4_e987_d_b0: f64 = (eq4_e985_d_b0 * p.p32);
        let eq4_e987_d_b1: f64 = (eq4_e985_d_b1 * p.p32);
        let eq4_e987_d_b2: f64 = (eq4_e985_d_b2 * p.p32);
        let eq4_e987_d_b3: f64 = (eq4_e985_d_b3 * p.p32);
        let eq4_e987_d_b4: f64 = (eq4_e985_d_b4 * p.p32);
        let eq4_e987_d_b5: f64 = (eq4_e985_d_b5 * p.p32);
        let eq4_e987_d_b6: f64 = (eq4_e985_d_b6 * p.p32);
        let eq4_e989: f64 = (eq4_e987 * s.v[841]);
        let eq4_e989_d_n0: f64 = ((eq4_e987_d_n0 * s.v[841]) + (eq4_e987 * s.dn[841][0]));
        let eq4_e989_d_n1: f64 = ((eq4_e987_d_n1 * s.v[841]) + (eq4_e987 * s.dn[841][1]));
        let eq4_e989_d_n2: f64 = ((eq4_e987_d_n2 * s.v[841]) + (eq4_e987 * s.dn[841][2]));
        let eq4_e989_d_n3: f64 = ((eq4_e987_d_n3 * s.v[841]) + (eq4_e987 * s.dn[841][3]));
        let eq4_e989_d_n4: f64 = ((eq4_e987_d_n4 * s.v[841]) + (eq4_e987 * s.dn[841][4]));
        let eq4_e989_d_n5: f64 = ((eq4_e987_d_n5 * s.v[841]) + (eq4_e987 * s.dn[841][5]));
        let eq4_e989_d_n6: f64 = ((eq4_e987_d_n6 * s.v[841]) + (eq4_e987 * s.dn[841][6]));
        let eq4_e989_d_n7: f64 = ((eq4_e987_d_n7 * s.v[841]) + (eq4_e987 * s.dn[841][7]));
        let eq4_e989_d_n8: f64 = ((eq4_e987_d_n8 * s.v[841]) + (eq4_e987 * s.dn[841][8]));
        let eq4_e989_d_n9: f64 = ((eq4_e987_d_n9 * s.v[841]) + (eq4_e987 * s.dn[841][9]));
        let eq4_e989_d_n10: f64 = ((eq4_e987_d_n10 * s.v[841]) + (eq4_e987 * s.dn[841][10]));
        let eq4_e989_d_n11: f64 = ((eq4_e987_d_n11 * s.v[841]) + (eq4_e987 * s.dn[841][11]));
        let eq4_e989_d_b0: f64 = ((eq4_e987_d_b0 * s.v[841]) + (eq4_e987 * s.db[841][0]));
        let eq4_e989_d_b1: f64 = ((eq4_e987_d_b1 * s.v[841]) + (eq4_e987 * s.db[841][1]));
        let eq4_e989_d_b2: f64 = ((eq4_e987_d_b2 * s.v[841]) + (eq4_e987 * s.db[841][2]));
        let eq4_e989_d_b3: f64 = ((eq4_e987_d_b3 * s.v[841]) + (eq4_e987 * s.db[841][3]));
        let eq4_e989_d_b4: f64 = ((eq4_e987_d_b4 * s.v[841]) + (eq4_e987 * s.db[841][4]));
        let eq4_e989_d_b5: f64 = ((eq4_e987_d_b5 * s.v[841]) + (eq4_e987 * s.db[841][5]));
        let eq4_e989_d_b6: f64 = ((eq4_e987_d_b6 * s.v[841]) + (eq4_e987 * s.db[841][6]));
        (eq4_e989, eq4_e989_d_n0, eq4_e989_d_n1, eq4_e989_d_n2, eq4_e989_d_n3, eq4_e989_d_n4, eq4_e989_d_n5, eq4_e989_d_n6, eq4_e989_d_n7, eq4_e989_d_n8, eq4_e989_d_n9, eq4_e989_d_n10, eq4_e989_d_n11, eq4_e989_d_b0, eq4_e989_d_b1, eq4_e989_d_b2, eq4_e989_d_b3, eq4_e989_d_b4, eq4_e989_d_b5, eq4_e989_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq4_value: f64 = eq4_e991;
        let eq4_node_derivatives: [f64; 12] = [eq4_e991_d_n0, eq4_e991_d_n1, eq4_e991_d_n2, eq4_e991_d_n3, eq4_e991_d_n4, eq4_e991_d_n5, eq4_e991_d_n6, eq4_e991_d_n7, eq4_e991_d_n8, eq4_e991_d_n9, eq4_e991_d_n10, eq4_e991_d_n11];
        let eq4_branch_derivatives: [f64; 7] = [eq4_e991_d_b0, eq4_e991_d_b1, eq4_e991_d_b2, eq4_e991_d_b3, eq4_e991_d_b4, eq4_e991_d_b5, eq4_e991_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq4_value),
            &eq4_node_derivatives,
            &eq4_branch_derivatives,
            multiplicity,
        );
        let (eq5_e1004, eq5_e1004_d_n0, eq5_e1004_d_n1, eq5_e1004_d_n2, eq5_e1004_d_n3, eq5_e1004_d_n4, eq5_e1004_d_n5, eq5_e1004_d_n6, eq5_e1004_d_n7, eq5_e1004_d_n8, eq5_e1004_d_n9, eq5_e1004_d_n10, eq5_e1004_d_n11, eq5_e1004_d_b0, eq5_e1004_d_b1, eq5_e1004_d_b2, eq5_e1004_d_b3, eq5_e1004_d_b4, eq5_e1004_d_b5, eq5_e1004_d_b6,) = {
    if (!s.b[2701]) {
        let eq5_e996: f64 = (s.v[0] * s.v[15]);
        let eq5_e996_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq5_e996_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq5_e996_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq5_e996_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq5_e996_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq5_e996_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq5_e996_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq5_e996_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq5_e996_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq5_e996_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq5_e996_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq5_e996_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq5_e996_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq5_e996_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq5_e996_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq5_e996_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq5_e996_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq5_e996_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq5_e996_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq5_e998: f64 = (eq5_e996 * p.p32);
        let eq5_e998_d_n0: f64 = (eq5_e996_d_n0 * p.p32);
        let eq5_e998_d_n1: f64 = (eq5_e996_d_n1 * p.p32);
        let eq5_e998_d_n2: f64 = (eq5_e996_d_n2 * p.p32);
        let eq5_e998_d_n3: f64 = (eq5_e996_d_n3 * p.p32);
        let eq5_e998_d_n4: f64 = (eq5_e996_d_n4 * p.p32);
        let eq5_e998_d_n5: f64 = (eq5_e996_d_n5 * p.p32);
        let eq5_e998_d_n6: f64 = (eq5_e996_d_n6 * p.p32);
        let eq5_e998_d_n7: f64 = (eq5_e996_d_n7 * p.p32);
        let eq5_e998_d_n8: f64 = (eq5_e996_d_n8 * p.p32);
        let eq5_e998_d_n9: f64 = (eq5_e996_d_n9 * p.p32);
        let eq5_e998_d_n10: f64 = (eq5_e996_d_n10 * p.p32);
        let eq5_e998_d_n11: f64 = (eq5_e996_d_n11 * p.p32);
        let eq5_e998_d_b0: f64 = (eq5_e996_d_b0 * p.p32);
        let eq5_e998_d_b1: f64 = (eq5_e996_d_b1 * p.p32);
        let eq5_e998_d_b2: f64 = (eq5_e996_d_b2 * p.p32);
        let eq5_e998_d_b3: f64 = (eq5_e996_d_b3 * p.p32);
        let eq5_e998_d_b4: f64 = (eq5_e996_d_b4 * p.p32);
        let eq5_e998_d_b5: f64 = (eq5_e996_d_b5 * p.p32);
        let eq5_e998_d_b6: f64 = (eq5_e996_d_b6 * p.p32);
        let eq5_e1001: f64 = (s.v[832] + s.v[840]);
        let eq5_e1001_d_n0: f64 = (s.dn[832][0] + s.dn[840][0]);
        let eq5_e1001_d_n1: f64 = (s.dn[832][1] + s.dn[840][1]);
        let eq5_e1001_d_n2: f64 = (s.dn[832][2] + s.dn[840][2]);
        let eq5_e1001_d_n3: f64 = (s.dn[832][3] + s.dn[840][3]);
        let eq5_e1001_d_n4: f64 = (s.dn[832][4] + s.dn[840][4]);
        let eq5_e1001_d_n5: f64 = (s.dn[832][5] + s.dn[840][5]);
        let eq5_e1001_d_n6: f64 = (s.dn[832][6] + s.dn[840][6]);
        let eq5_e1001_d_n7: f64 = (s.dn[832][7] + s.dn[840][7]);
        let eq5_e1001_d_n8: f64 = (s.dn[832][8] + s.dn[840][8]);
        let eq5_e1001_d_n9: f64 = (s.dn[832][9] + s.dn[840][9]);
        let eq5_e1001_d_n10: f64 = (s.dn[832][10] + s.dn[840][10]);
        let eq5_e1001_d_n11: f64 = (s.dn[832][11] + s.dn[840][11]);
        let eq5_e1001_d_b0: f64 = (s.db[832][0] + s.db[840][0]);
        let eq5_e1001_d_b1: f64 = (s.db[832][1] + s.db[840][1]);
        let eq5_e1001_d_b2: f64 = (s.db[832][2] + s.db[840][2]);
        let eq5_e1001_d_b3: f64 = (s.db[832][3] + s.db[840][3]);
        let eq5_e1001_d_b4: f64 = (s.db[832][4] + s.db[840][4]);
        let eq5_e1001_d_b5: f64 = (s.db[832][5] + s.db[840][5]);
        let eq5_e1001_d_b6: f64 = (s.db[832][6] + s.db[840][6]);
        let eq5_e1002: f64 = (eq5_e998 * eq5_e1001);
        let eq5_e1002_d_n0: f64 = ((eq5_e998_d_n0 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n0));
        let eq5_e1002_d_n1: f64 = ((eq5_e998_d_n1 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n1));
        let eq5_e1002_d_n2: f64 = ((eq5_e998_d_n2 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n2));
        let eq5_e1002_d_n3: f64 = ((eq5_e998_d_n3 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n3));
        let eq5_e1002_d_n4: f64 = ((eq5_e998_d_n4 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n4));
        let eq5_e1002_d_n5: f64 = ((eq5_e998_d_n5 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n5));
        let eq5_e1002_d_n6: f64 = ((eq5_e998_d_n6 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n6));
        let eq5_e1002_d_n7: f64 = ((eq5_e998_d_n7 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n7));
        let eq5_e1002_d_n8: f64 = ((eq5_e998_d_n8 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n8));
        let eq5_e1002_d_n9: f64 = ((eq5_e998_d_n9 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n9));
        let eq5_e1002_d_n10: f64 = ((eq5_e998_d_n10 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n10));
        let eq5_e1002_d_n11: f64 = ((eq5_e998_d_n11 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_n11));
        let eq5_e1002_d_b0: f64 = ((eq5_e998_d_b0 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_b0));
        let eq5_e1002_d_b1: f64 = ((eq5_e998_d_b1 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_b1));
        let eq5_e1002_d_b2: f64 = ((eq5_e998_d_b2 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_b2));
        let eq5_e1002_d_b3: f64 = ((eq5_e998_d_b3 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_b3));
        let eq5_e1002_d_b4: f64 = ((eq5_e998_d_b4 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_b4));
        let eq5_e1002_d_b5: f64 = ((eq5_e998_d_b5 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_b5));
        let eq5_e1002_d_b6: f64 = ((eq5_e998_d_b6 * eq5_e1001) + (eq5_e998 * eq5_e1001_d_b6));
        (eq5_e1002, eq5_e1002_d_n0, eq5_e1002_d_n1, eq5_e1002_d_n2, eq5_e1002_d_n3, eq5_e1002_d_n4, eq5_e1002_d_n5, eq5_e1002_d_n6, eq5_e1002_d_n7, eq5_e1002_d_n8, eq5_e1002_d_n9, eq5_e1002_d_n10, eq5_e1002_d_n11, eq5_e1002_d_b0, eq5_e1002_d_b1, eq5_e1002_d_b2, eq5_e1002_d_b3, eq5_e1002_d_b4, eq5_e1002_d_b5, eq5_e1002_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq5_value: f64 = eq5_e1004;
        let eq5_node_derivatives: [f64; 12] = [eq5_e1004_d_n0, eq5_e1004_d_n1, eq5_e1004_d_n2, eq5_e1004_d_n3, eq5_e1004_d_n4, eq5_e1004_d_n5, eq5_e1004_d_n6, eq5_e1004_d_n7, eq5_e1004_d_n8, eq5_e1004_d_n9, eq5_e1004_d_n10, eq5_e1004_d_n11];
        let eq5_branch_derivatives: [f64; 7] = [eq5_e1004_d_b0, eq5_e1004_d_b1, eq5_e1004_d_b2, eq5_e1004_d_b3, eq5_e1004_d_b4, eq5_e1004_d_b5, eq5_e1004_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(7),
            multiplicity * (eq5_value),
            &eq5_node_derivatives,
            &eq5_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_1(
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
        p: &Parameters,
        multiplicity: f64,
    ) {
        let (eq6_e1015, eq6_e1015_d_n0, eq6_e1015_d_n1, eq6_e1015_d_n2, eq6_e1015_d_n3, eq6_e1015_d_n4, eq6_e1015_d_n5, eq6_e1015_d_n6, eq6_e1015_d_n7, eq6_e1015_d_n8, eq6_e1015_d_n9, eq6_e1015_d_n10, eq6_e1015_d_n11, eq6_e1015_d_b0, eq6_e1015_d_b1, eq6_e1015_d_b2, eq6_e1015_d_b3, eq6_e1015_d_b4, eq6_e1015_d_b5, eq6_e1015_d_b6,) = {
    if (!s.b[2701]) {
        let eq6_e1009: f64 = (s.v[0] * s.v[15]);
        let eq6_e1009_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq6_e1009_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq6_e1009_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq6_e1009_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq6_e1009_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq6_e1009_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq6_e1009_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq6_e1009_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq6_e1009_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq6_e1009_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq6_e1009_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq6_e1009_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq6_e1009_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq6_e1009_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq6_e1009_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq6_e1009_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq6_e1009_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq6_e1009_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq6_e1009_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq6_e1011: f64 = (eq6_e1009 * p.p32);
        let eq6_e1011_d_n0: f64 = (eq6_e1009_d_n0 * p.p32);
        let eq6_e1011_d_n1: f64 = (eq6_e1009_d_n1 * p.p32);
        let eq6_e1011_d_n2: f64 = (eq6_e1009_d_n2 * p.p32);
        let eq6_e1011_d_n3: f64 = (eq6_e1009_d_n3 * p.p32);
        let eq6_e1011_d_n4: f64 = (eq6_e1009_d_n4 * p.p32);
        let eq6_e1011_d_n5: f64 = (eq6_e1009_d_n5 * p.p32);
        let eq6_e1011_d_n6: f64 = (eq6_e1009_d_n6 * p.p32);
        let eq6_e1011_d_n7: f64 = (eq6_e1009_d_n7 * p.p32);
        let eq6_e1011_d_n8: f64 = (eq6_e1009_d_n8 * p.p32);
        let eq6_e1011_d_n9: f64 = (eq6_e1009_d_n9 * p.p32);
        let eq6_e1011_d_n10: f64 = (eq6_e1009_d_n10 * p.p32);
        let eq6_e1011_d_n11: f64 = (eq6_e1009_d_n11 * p.p32);
        let eq6_e1011_d_b0: f64 = (eq6_e1009_d_b0 * p.p32);
        let eq6_e1011_d_b1: f64 = (eq6_e1009_d_b1 * p.p32);
        let eq6_e1011_d_b2: f64 = (eq6_e1009_d_b2 * p.p32);
        let eq6_e1011_d_b3: f64 = (eq6_e1009_d_b3 * p.p32);
        let eq6_e1011_d_b4: f64 = (eq6_e1009_d_b4 * p.p32);
        let eq6_e1011_d_b5: f64 = (eq6_e1009_d_b5 * p.p32);
        let eq6_e1011_d_b6: f64 = (eq6_e1009_d_b6 * p.p32);
        let eq6_e1013: f64 = (eq6_e1011 * s.v[835]);
        let eq6_e1013_d_n0: f64 = ((eq6_e1011_d_n0 * s.v[835]) + (eq6_e1011 * s.dn[835][0]));
        let eq6_e1013_d_n1: f64 = ((eq6_e1011_d_n1 * s.v[835]) + (eq6_e1011 * s.dn[835][1]));
        let eq6_e1013_d_n2: f64 = ((eq6_e1011_d_n2 * s.v[835]) + (eq6_e1011 * s.dn[835][2]));
        let eq6_e1013_d_n3: f64 = ((eq6_e1011_d_n3 * s.v[835]) + (eq6_e1011 * s.dn[835][3]));
        let eq6_e1013_d_n4: f64 = ((eq6_e1011_d_n4 * s.v[835]) + (eq6_e1011 * s.dn[835][4]));
        let eq6_e1013_d_n5: f64 = ((eq6_e1011_d_n5 * s.v[835]) + (eq6_e1011 * s.dn[835][5]));
        let eq6_e1013_d_n6: f64 = ((eq6_e1011_d_n6 * s.v[835]) + (eq6_e1011 * s.dn[835][6]));
        let eq6_e1013_d_n7: f64 = ((eq6_e1011_d_n7 * s.v[835]) + (eq6_e1011 * s.dn[835][7]));
        let eq6_e1013_d_n8: f64 = ((eq6_e1011_d_n8 * s.v[835]) + (eq6_e1011 * s.dn[835][8]));
        let eq6_e1013_d_n9: f64 = ((eq6_e1011_d_n9 * s.v[835]) + (eq6_e1011 * s.dn[835][9]));
        let eq6_e1013_d_n10: f64 = ((eq6_e1011_d_n10 * s.v[835]) + (eq6_e1011 * s.dn[835][10]));
        let eq6_e1013_d_n11: f64 = ((eq6_e1011_d_n11 * s.v[835]) + (eq6_e1011 * s.dn[835][11]));
        let eq6_e1013_d_b0: f64 = ((eq6_e1011_d_b0 * s.v[835]) + (eq6_e1011 * s.db[835][0]));
        let eq6_e1013_d_b1: f64 = ((eq6_e1011_d_b1 * s.v[835]) + (eq6_e1011 * s.db[835][1]));
        let eq6_e1013_d_b2: f64 = ((eq6_e1011_d_b2 * s.v[835]) + (eq6_e1011 * s.db[835][2]));
        let eq6_e1013_d_b3: f64 = ((eq6_e1011_d_b3 * s.v[835]) + (eq6_e1011 * s.db[835][3]));
        let eq6_e1013_d_b4: f64 = ((eq6_e1011_d_b4 * s.v[835]) + (eq6_e1011 * s.db[835][4]));
        let eq6_e1013_d_b5: f64 = ((eq6_e1011_d_b5 * s.v[835]) + (eq6_e1011 * s.db[835][5]));
        let eq6_e1013_d_b6: f64 = ((eq6_e1011_d_b6 * s.v[835]) + (eq6_e1011 * s.db[835][6]));
        (eq6_e1013, eq6_e1013_d_n0, eq6_e1013_d_n1, eq6_e1013_d_n2, eq6_e1013_d_n3, eq6_e1013_d_n4, eq6_e1013_d_n5, eq6_e1013_d_n6, eq6_e1013_d_n7, eq6_e1013_d_n8, eq6_e1013_d_n9, eq6_e1013_d_n10, eq6_e1013_d_n11, eq6_e1013_d_b0, eq6_e1013_d_b1, eq6_e1013_d_b2, eq6_e1013_d_b3, eq6_e1013_d_b4, eq6_e1013_d_b5, eq6_e1013_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq6_value: f64 = eq6_e1015;
        let eq6_node_derivatives: [f64; 12] = [eq6_e1015_d_n0, eq6_e1015_d_n1, eq6_e1015_d_n2, eq6_e1015_d_n3, eq6_e1015_d_n4, eq6_e1015_d_n5, eq6_e1015_d_n6, eq6_e1015_d_n7, eq6_e1015_d_n8, eq6_e1015_d_n9, eq6_e1015_d_n10, eq6_e1015_d_n11];
        let eq6_branch_derivatives: [f64; 7] = [eq6_e1015_d_b0, eq6_e1015_d_b1, eq6_e1015_d_b2, eq6_e1015_d_b3, eq6_e1015_d_b4, eq6_e1015_d_b5, eq6_e1015_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq6_value),
            &eq6_node_derivatives,
            &eq6_branch_derivatives,
            multiplicity,
        );
        let (eq7_e1026, eq7_e1026_d_n0, eq7_e1026_d_n1, eq7_e1026_d_n2, eq7_e1026_d_n3, eq7_e1026_d_n4, eq7_e1026_d_n5, eq7_e1026_d_n6, eq7_e1026_d_n7, eq7_e1026_d_n8, eq7_e1026_d_n9, eq7_e1026_d_n10, eq7_e1026_d_n11, eq7_e1026_d_b0, eq7_e1026_d_b1, eq7_e1026_d_b2, eq7_e1026_d_b3, eq7_e1026_d_b4, eq7_e1026_d_b5, eq7_e1026_d_b6,) = {
    if (!s.b[2701]) {
        let eq7_e1020: f64 = (s.v[0] * s.v[15]);
        let eq7_e1020_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq7_e1020_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq7_e1020_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq7_e1020_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq7_e1020_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq7_e1020_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq7_e1020_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq7_e1020_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq7_e1020_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq7_e1020_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq7_e1020_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq7_e1020_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq7_e1020_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq7_e1020_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq7_e1020_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq7_e1020_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq7_e1020_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq7_e1020_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq7_e1020_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq7_e1022: f64 = (eq7_e1020 * p.p32);
        let eq7_e1022_d_n0: f64 = (eq7_e1020_d_n0 * p.p32);
        let eq7_e1022_d_n1: f64 = (eq7_e1020_d_n1 * p.p32);
        let eq7_e1022_d_n2: f64 = (eq7_e1020_d_n2 * p.p32);
        let eq7_e1022_d_n3: f64 = (eq7_e1020_d_n3 * p.p32);
        let eq7_e1022_d_n4: f64 = (eq7_e1020_d_n4 * p.p32);
        let eq7_e1022_d_n5: f64 = (eq7_e1020_d_n5 * p.p32);
        let eq7_e1022_d_n6: f64 = (eq7_e1020_d_n6 * p.p32);
        let eq7_e1022_d_n7: f64 = (eq7_e1020_d_n7 * p.p32);
        let eq7_e1022_d_n8: f64 = (eq7_e1020_d_n8 * p.p32);
        let eq7_e1022_d_n9: f64 = (eq7_e1020_d_n9 * p.p32);
        let eq7_e1022_d_n10: f64 = (eq7_e1020_d_n10 * p.p32);
        let eq7_e1022_d_n11: f64 = (eq7_e1020_d_n11 * p.p32);
        let eq7_e1022_d_b0: f64 = (eq7_e1020_d_b0 * p.p32);
        let eq7_e1022_d_b1: f64 = (eq7_e1020_d_b1 * p.p32);
        let eq7_e1022_d_b2: f64 = (eq7_e1020_d_b2 * p.p32);
        let eq7_e1022_d_b3: f64 = (eq7_e1020_d_b3 * p.p32);
        let eq7_e1022_d_b4: f64 = (eq7_e1020_d_b4 * p.p32);
        let eq7_e1022_d_b5: f64 = (eq7_e1020_d_b5 * p.p32);
        let eq7_e1022_d_b6: f64 = (eq7_e1020_d_b6 * p.p32);
        let eq7_e1024: f64 = (eq7_e1022 * s.v[836]);
        let eq7_e1024_d_n0: f64 = ((eq7_e1022_d_n0 * s.v[836]) + (eq7_e1022 * s.dn[836][0]));
        let eq7_e1024_d_n1: f64 = ((eq7_e1022_d_n1 * s.v[836]) + (eq7_e1022 * s.dn[836][1]));
        let eq7_e1024_d_n2: f64 = ((eq7_e1022_d_n2 * s.v[836]) + (eq7_e1022 * s.dn[836][2]));
        let eq7_e1024_d_n3: f64 = ((eq7_e1022_d_n3 * s.v[836]) + (eq7_e1022 * s.dn[836][3]));
        let eq7_e1024_d_n4: f64 = ((eq7_e1022_d_n4 * s.v[836]) + (eq7_e1022 * s.dn[836][4]));
        let eq7_e1024_d_n5: f64 = ((eq7_e1022_d_n5 * s.v[836]) + (eq7_e1022 * s.dn[836][5]));
        let eq7_e1024_d_n6: f64 = ((eq7_e1022_d_n6 * s.v[836]) + (eq7_e1022 * s.dn[836][6]));
        let eq7_e1024_d_n7: f64 = ((eq7_e1022_d_n7 * s.v[836]) + (eq7_e1022 * s.dn[836][7]));
        let eq7_e1024_d_n8: f64 = ((eq7_e1022_d_n8 * s.v[836]) + (eq7_e1022 * s.dn[836][8]));
        let eq7_e1024_d_n9: f64 = ((eq7_e1022_d_n9 * s.v[836]) + (eq7_e1022 * s.dn[836][9]));
        let eq7_e1024_d_n10: f64 = ((eq7_e1022_d_n10 * s.v[836]) + (eq7_e1022 * s.dn[836][10]));
        let eq7_e1024_d_n11: f64 = ((eq7_e1022_d_n11 * s.v[836]) + (eq7_e1022 * s.dn[836][11]));
        let eq7_e1024_d_b0: f64 = ((eq7_e1022_d_b0 * s.v[836]) + (eq7_e1022 * s.db[836][0]));
        let eq7_e1024_d_b1: f64 = ((eq7_e1022_d_b1 * s.v[836]) + (eq7_e1022 * s.db[836][1]));
        let eq7_e1024_d_b2: f64 = ((eq7_e1022_d_b2 * s.v[836]) + (eq7_e1022 * s.db[836][2]));
        let eq7_e1024_d_b3: f64 = ((eq7_e1022_d_b3 * s.v[836]) + (eq7_e1022 * s.db[836][3]));
        let eq7_e1024_d_b4: f64 = ((eq7_e1022_d_b4 * s.v[836]) + (eq7_e1022 * s.db[836][4]));
        let eq7_e1024_d_b5: f64 = ((eq7_e1022_d_b5 * s.v[836]) + (eq7_e1022 * s.db[836][5]));
        let eq7_e1024_d_b6: f64 = ((eq7_e1022_d_b6 * s.v[836]) + (eq7_e1022 * s.db[836][6]));
        (eq7_e1024, eq7_e1024_d_n0, eq7_e1024_d_n1, eq7_e1024_d_n2, eq7_e1024_d_n3, eq7_e1024_d_n4, eq7_e1024_d_n5, eq7_e1024_d_n6, eq7_e1024_d_n7, eq7_e1024_d_n8, eq7_e1024_d_n9, eq7_e1024_d_n10, eq7_e1024_d_n11, eq7_e1024_d_b0, eq7_e1024_d_b1, eq7_e1024_d_b2, eq7_e1024_d_b3, eq7_e1024_d_b4, eq7_e1024_d_b5, eq7_e1024_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq7_value: f64 = eq7_e1026;
        let eq7_node_derivatives: [f64; 12] = [eq7_e1026_d_n0, eq7_e1026_d_n1, eq7_e1026_d_n2, eq7_e1026_d_n3, eq7_e1026_d_n4, eq7_e1026_d_n5, eq7_e1026_d_n6, eq7_e1026_d_n7, eq7_e1026_d_n8, eq7_e1026_d_n9, eq7_e1026_d_n10, eq7_e1026_d_n11];
        let eq7_branch_derivatives: [f64; 7] = [eq7_e1026_d_b0, eq7_e1026_d_b1, eq7_e1026_d_b2, eq7_e1026_d_b3, eq7_e1026_d_b4, eq7_e1026_d_b5, eq7_e1026_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq7_value),
            &eq7_node_derivatives,
            &eq7_branch_derivatives,
            multiplicity,
        );
        let eq8_e1029: f64 = (s.v[0] * s.v[15]);
        let eq8_e1029_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq8_e1029_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq8_e1029_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq8_e1029_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq8_e1029_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq8_e1029_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq8_e1029_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq8_e1029_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq8_e1029_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq8_e1029_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq8_e1029_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq8_e1029_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq8_e1029_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq8_e1029_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq8_e1029_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq8_e1029_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq8_e1029_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq8_e1029_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq8_e1029_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq8_e1031: f64 = (eq8_e1029 * p.p32);
        let eq8_e1031_d_n0: f64 = (eq8_e1029_d_n0 * p.p32);
        let eq8_e1031_d_n1: f64 = (eq8_e1029_d_n1 * p.p32);
        let eq8_e1031_d_n2: f64 = (eq8_e1029_d_n2 * p.p32);
        let eq8_e1031_d_n3: f64 = (eq8_e1029_d_n3 * p.p32);
        let eq8_e1031_d_n4: f64 = (eq8_e1029_d_n4 * p.p32);
        let eq8_e1031_d_n5: f64 = (eq8_e1029_d_n5 * p.p32);
        let eq8_e1031_d_n6: f64 = (eq8_e1029_d_n6 * p.p32);
        let eq8_e1031_d_n7: f64 = (eq8_e1029_d_n7 * p.p32);
        let eq8_e1031_d_n8: f64 = (eq8_e1029_d_n8 * p.p32);
        let eq8_e1031_d_n9: f64 = (eq8_e1029_d_n9 * p.p32);
        let eq8_e1031_d_n10: f64 = (eq8_e1029_d_n10 * p.p32);
        let eq8_e1031_d_n11: f64 = (eq8_e1029_d_n11 * p.p32);
        let eq8_e1031_d_b0: f64 = (eq8_e1029_d_b0 * p.p32);
        let eq8_e1031_d_b1: f64 = (eq8_e1029_d_b1 * p.p32);
        let eq8_e1031_d_b2: f64 = (eq8_e1029_d_b2 * p.p32);
        let eq8_e1031_d_b3: f64 = (eq8_e1029_d_b3 * p.p32);
        let eq8_e1031_d_b4: f64 = (eq8_e1029_d_b4 * p.p32);
        let eq8_e1031_d_b5: f64 = (eq8_e1029_d_b5 * p.p32);
        let eq8_e1031_d_b6: f64 = (eq8_e1029_d_b6 * p.p32);
        let eq8_e1033: f64 = (eq8_e1031 * s.v[837]);
        let eq8_e1033_d_n0: f64 = ((eq8_e1031_d_n0 * s.v[837]) + (eq8_e1031 * s.dn[837][0]));
        let eq8_e1033_d_n1: f64 = ((eq8_e1031_d_n1 * s.v[837]) + (eq8_e1031 * s.dn[837][1]));
        let eq8_e1033_d_n2: f64 = ((eq8_e1031_d_n2 * s.v[837]) + (eq8_e1031 * s.dn[837][2]));
        let eq8_e1033_d_n3: f64 = ((eq8_e1031_d_n3 * s.v[837]) + (eq8_e1031 * s.dn[837][3]));
        let eq8_e1033_d_n4: f64 = ((eq8_e1031_d_n4 * s.v[837]) + (eq8_e1031 * s.dn[837][4]));
        let eq8_e1033_d_n5: f64 = ((eq8_e1031_d_n5 * s.v[837]) + (eq8_e1031 * s.dn[837][5]));
        let eq8_e1033_d_n6: f64 = ((eq8_e1031_d_n6 * s.v[837]) + (eq8_e1031 * s.dn[837][6]));
        let eq8_e1033_d_n7: f64 = ((eq8_e1031_d_n7 * s.v[837]) + (eq8_e1031 * s.dn[837][7]));
        let eq8_e1033_d_n8: f64 = ((eq8_e1031_d_n8 * s.v[837]) + (eq8_e1031 * s.dn[837][8]));
        let eq8_e1033_d_n9: f64 = ((eq8_e1031_d_n9 * s.v[837]) + (eq8_e1031 * s.dn[837][9]));
        let eq8_e1033_d_n10: f64 = ((eq8_e1031_d_n10 * s.v[837]) + (eq8_e1031 * s.dn[837][10]));
        let eq8_e1033_d_n11: f64 = ((eq8_e1031_d_n11 * s.v[837]) + (eq8_e1031 * s.dn[837][11]));
        let eq8_e1033_d_b0: f64 = ((eq8_e1031_d_b0 * s.v[837]) + (eq8_e1031 * s.db[837][0]));
        let eq8_e1033_d_b1: f64 = ((eq8_e1031_d_b1 * s.v[837]) + (eq8_e1031 * s.db[837][1]));
        let eq8_e1033_d_b2: f64 = ((eq8_e1031_d_b2 * s.v[837]) + (eq8_e1031 * s.db[837][2]));
        let eq8_e1033_d_b3: f64 = ((eq8_e1031_d_b3 * s.v[837]) + (eq8_e1031 * s.db[837][3]));
        let eq8_e1033_d_b4: f64 = ((eq8_e1031_d_b4 * s.v[837]) + (eq8_e1031 * s.db[837][4]));
        let eq8_e1033_d_b5: f64 = ((eq8_e1031_d_b5 * s.v[837]) + (eq8_e1031 * s.db[837][5]));
        let eq8_e1033_d_b6: f64 = ((eq8_e1031_d_b6 * s.v[837]) + (eq8_e1031 * s.db[837][6]));
        let eq8_value: f64 = eq8_e1033;
        let eq8_node_derivatives: [f64; 12] = [eq8_e1033_d_n0, eq8_e1033_d_n1, eq8_e1033_d_n2, eq8_e1033_d_n3, eq8_e1033_d_n4, eq8_e1033_d_n5, eq8_e1033_d_n6, eq8_e1033_d_n7, eq8_e1033_d_n8, eq8_e1033_d_n9, eq8_e1033_d_n10, eq8_e1033_d_n11];
        let eq8_branch_derivatives: [f64; 7] = [eq8_e1033_d_b0, eq8_e1033_d_b1, eq8_e1033_d_b2, eq8_e1033_d_b3, eq8_e1033_d_b4, eq8_e1033_d_b5, eq8_e1033_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq8_value),
            &eq8_node_derivatives,
            &eq8_branch_derivatives,
            multiplicity,
        );
        let eq9_e1036: f64 = (s.v[0] * s.v[15]);
        let eq9_e1036_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq9_e1036_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq9_e1036_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq9_e1036_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq9_e1036_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq9_e1036_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq9_e1036_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq9_e1036_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq9_e1036_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq9_e1036_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq9_e1036_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq9_e1036_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq9_e1036_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq9_e1036_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq9_e1036_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq9_e1036_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq9_e1036_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq9_e1036_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq9_e1036_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq9_e1038: f64 = (eq9_e1036 * p.p32);
        let eq9_e1038_d_n0: f64 = (eq9_e1036_d_n0 * p.p32);
        let eq9_e1038_d_n1: f64 = (eq9_e1036_d_n1 * p.p32);
        let eq9_e1038_d_n2: f64 = (eq9_e1036_d_n2 * p.p32);
        let eq9_e1038_d_n3: f64 = (eq9_e1036_d_n3 * p.p32);
        let eq9_e1038_d_n4: f64 = (eq9_e1036_d_n4 * p.p32);
        let eq9_e1038_d_n5: f64 = (eq9_e1036_d_n5 * p.p32);
        let eq9_e1038_d_n6: f64 = (eq9_e1036_d_n6 * p.p32);
        let eq9_e1038_d_n7: f64 = (eq9_e1036_d_n7 * p.p32);
        let eq9_e1038_d_n8: f64 = (eq9_e1036_d_n8 * p.p32);
        let eq9_e1038_d_n9: f64 = (eq9_e1036_d_n9 * p.p32);
        let eq9_e1038_d_n10: f64 = (eq9_e1036_d_n10 * p.p32);
        let eq9_e1038_d_n11: f64 = (eq9_e1036_d_n11 * p.p32);
        let eq9_e1038_d_b0: f64 = (eq9_e1036_d_b0 * p.p32);
        let eq9_e1038_d_b1: f64 = (eq9_e1036_d_b1 * p.p32);
        let eq9_e1038_d_b2: f64 = (eq9_e1036_d_b2 * p.p32);
        let eq9_e1038_d_b3: f64 = (eq9_e1036_d_b3 * p.p32);
        let eq9_e1038_d_b4: f64 = (eq9_e1036_d_b4 * p.p32);
        let eq9_e1038_d_b5: f64 = (eq9_e1036_d_b5 * p.p32);
        let eq9_e1038_d_b6: f64 = (eq9_e1036_d_b6 * p.p32);
        let eq9_e1040: f64 = (eq9_e1038 * s.v[833]);
        let eq9_e1040_d_n0: f64 = ((eq9_e1038_d_n0 * s.v[833]) + (eq9_e1038 * s.dn[833][0]));
        let eq9_e1040_d_n1: f64 = ((eq9_e1038_d_n1 * s.v[833]) + (eq9_e1038 * s.dn[833][1]));
        let eq9_e1040_d_n2: f64 = ((eq9_e1038_d_n2 * s.v[833]) + (eq9_e1038 * s.dn[833][2]));
        let eq9_e1040_d_n3: f64 = ((eq9_e1038_d_n3 * s.v[833]) + (eq9_e1038 * s.dn[833][3]));
        let eq9_e1040_d_n4: f64 = ((eq9_e1038_d_n4 * s.v[833]) + (eq9_e1038 * s.dn[833][4]));
        let eq9_e1040_d_n5: f64 = ((eq9_e1038_d_n5 * s.v[833]) + (eq9_e1038 * s.dn[833][5]));
        let eq9_e1040_d_n6: f64 = ((eq9_e1038_d_n6 * s.v[833]) + (eq9_e1038 * s.dn[833][6]));
        let eq9_e1040_d_n7: f64 = ((eq9_e1038_d_n7 * s.v[833]) + (eq9_e1038 * s.dn[833][7]));
        let eq9_e1040_d_n8: f64 = ((eq9_e1038_d_n8 * s.v[833]) + (eq9_e1038 * s.dn[833][8]));
        let eq9_e1040_d_n9: f64 = ((eq9_e1038_d_n9 * s.v[833]) + (eq9_e1038 * s.dn[833][9]));
        let eq9_e1040_d_n10: f64 = ((eq9_e1038_d_n10 * s.v[833]) + (eq9_e1038 * s.dn[833][10]));
        let eq9_e1040_d_n11: f64 = ((eq9_e1038_d_n11 * s.v[833]) + (eq9_e1038 * s.dn[833][11]));
        let eq9_e1040_d_b0: f64 = ((eq9_e1038_d_b0 * s.v[833]) + (eq9_e1038 * s.db[833][0]));
        let eq9_e1040_d_b1: f64 = ((eq9_e1038_d_b1 * s.v[833]) + (eq9_e1038 * s.db[833][1]));
        let eq9_e1040_d_b2: f64 = ((eq9_e1038_d_b2 * s.v[833]) + (eq9_e1038 * s.db[833][2]));
        let eq9_e1040_d_b3: f64 = ((eq9_e1038_d_b3 * s.v[833]) + (eq9_e1038 * s.db[833][3]));
        let eq9_e1040_d_b4: f64 = ((eq9_e1038_d_b4 * s.v[833]) + (eq9_e1038 * s.db[833][4]));
        let eq9_e1040_d_b5: f64 = ((eq9_e1038_d_b5 * s.v[833]) + (eq9_e1038 * s.db[833][5]));
        let eq9_e1040_d_b6: f64 = ((eq9_e1038_d_b6 * s.v[833]) + (eq9_e1038 * s.db[833][6]));
        let eq9_value: f64 = eq9_e1040;
        let eq9_node_derivatives: [f64; 12] = [eq9_e1040_d_n0, eq9_e1040_d_n1, eq9_e1040_d_n2, eq9_e1040_d_n3, eq9_e1040_d_n4, eq9_e1040_d_n5, eq9_e1040_d_n6, eq9_e1040_d_n7, eq9_e1040_d_n8, eq9_e1040_d_n9, eq9_e1040_d_n10, eq9_e1040_d_n11];
        let eq9_branch_derivatives: [f64; 7] = [eq9_e1040_d_b0, eq9_e1040_d_b1, eq9_e1040_d_b2, eq9_e1040_d_b3, eq9_e1040_d_b4, eq9_e1040_d_b5, eq9_e1040_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq9_value),
            &eq9_node_derivatives,
            &eq9_branch_derivatives,
            multiplicity,
        );
        let eq10_e1043: f64 = (s.v[0] * s.v[15]);
        let eq10_e1043_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq10_e1043_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq10_e1043_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq10_e1043_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq10_e1043_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq10_e1043_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq10_e1043_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq10_e1043_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq10_e1043_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq10_e1043_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq10_e1043_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq10_e1043_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq10_e1043_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq10_e1043_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq10_e1043_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq10_e1043_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq10_e1043_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq10_e1043_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq10_e1043_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq10_e1045: f64 = (eq10_e1043 * p.p32);
        let eq10_e1045_d_n0: f64 = (eq10_e1043_d_n0 * p.p32);
        let eq10_e1045_d_n1: f64 = (eq10_e1043_d_n1 * p.p32);
        let eq10_e1045_d_n2: f64 = (eq10_e1043_d_n2 * p.p32);
        let eq10_e1045_d_n3: f64 = (eq10_e1043_d_n3 * p.p32);
        let eq10_e1045_d_n4: f64 = (eq10_e1043_d_n4 * p.p32);
        let eq10_e1045_d_n5: f64 = (eq10_e1043_d_n5 * p.p32);
        let eq10_e1045_d_n6: f64 = (eq10_e1043_d_n6 * p.p32);
        let eq10_e1045_d_n7: f64 = (eq10_e1043_d_n7 * p.p32);
        let eq10_e1045_d_n8: f64 = (eq10_e1043_d_n8 * p.p32);
        let eq10_e1045_d_n9: f64 = (eq10_e1043_d_n9 * p.p32);
        let eq10_e1045_d_n10: f64 = (eq10_e1043_d_n10 * p.p32);
        let eq10_e1045_d_n11: f64 = (eq10_e1043_d_n11 * p.p32);
        let eq10_e1045_d_b0: f64 = (eq10_e1043_d_b0 * p.p32);
        let eq10_e1045_d_b1: f64 = (eq10_e1043_d_b1 * p.p32);
        let eq10_e1045_d_b2: f64 = (eq10_e1043_d_b2 * p.p32);
        let eq10_e1045_d_b3: f64 = (eq10_e1043_d_b3 * p.p32);
        let eq10_e1045_d_b4: f64 = (eq10_e1043_d_b4 * p.p32);
        let eq10_e1045_d_b5: f64 = (eq10_e1043_d_b5 * p.p32);
        let eq10_e1045_d_b6: f64 = (eq10_e1043_d_b6 * p.p32);
        let eq10_e1047: f64 = (eq10_e1045 * s.v[834]);
        let eq10_e1047_d_n0: f64 = ((eq10_e1045_d_n0 * s.v[834]) + (eq10_e1045 * s.dn[834][0]));
        let eq10_e1047_d_n1: f64 = ((eq10_e1045_d_n1 * s.v[834]) + (eq10_e1045 * s.dn[834][1]));
        let eq10_e1047_d_n2: f64 = ((eq10_e1045_d_n2 * s.v[834]) + (eq10_e1045 * s.dn[834][2]));
        let eq10_e1047_d_n3: f64 = ((eq10_e1045_d_n3 * s.v[834]) + (eq10_e1045 * s.dn[834][3]));
        let eq10_e1047_d_n4: f64 = ((eq10_e1045_d_n4 * s.v[834]) + (eq10_e1045 * s.dn[834][4]));
        let eq10_e1047_d_n5: f64 = ((eq10_e1045_d_n5 * s.v[834]) + (eq10_e1045 * s.dn[834][5]));
        let eq10_e1047_d_n6: f64 = ((eq10_e1045_d_n6 * s.v[834]) + (eq10_e1045 * s.dn[834][6]));
        let eq10_e1047_d_n7: f64 = ((eq10_e1045_d_n7 * s.v[834]) + (eq10_e1045 * s.dn[834][7]));
        let eq10_e1047_d_n8: f64 = ((eq10_e1045_d_n8 * s.v[834]) + (eq10_e1045 * s.dn[834][8]));
        let eq10_e1047_d_n9: f64 = ((eq10_e1045_d_n9 * s.v[834]) + (eq10_e1045 * s.dn[834][9]));
        let eq10_e1047_d_n10: f64 = ((eq10_e1045_d_n10 * s.v[834]) + (eq10_e1045 * s.dn[834][10]));
        let eq10_e1047_d_n11: f64 = ((eq10_e1045_d_n11 * s.v[834]) + (eq10_e1045 * s.dn[834][11]));
        let eq10_e1047_d_b0: f64 = ((eq10_e1045_d_b0 * s.v[834]) + (eq10_e1045 * s.db[834][0]));
        let eq10_e1047_d_b1: f64 = ((eq10_e1045_d_b1 * s.v[834]) + (eq10_e1045 * s.db[834][1]));
        let eq10_e1047_d_b2: f64 = ((eq10_e1045_d_b2 * s.v[834]) + (eq10_e1045 * s.db[834][2]));
        let eq10_e1047_d_b3: f64 = ((eq10_e1045_d_b3 * s.v[834]) + (eq10_e1045 * s.db[834][3]));
        let eq10_e1047_d_b4: f64 = ((eq10_e1045_d_b4 * s.v[834]) + (eq10_e1045 * s.db[834][4]));
        let eq10_e1047_d_b5: f64 = ((eq10_e1045_d_b5 * s.v[834]) + (eq10_e1045 * s.db[834][5]));
        let eq10_e1047_d_b6: f64 = ((eq10_e1045_d_b6 * s.v[834]) + (eq10_e1045 * s.db[834][6]));
        let eq10_value: f64 = eq10_e1047;
        let eq10_node_derivatives: [f64; 12] = [eq10_e1047_d_n0, eq10_e1047_d_n1, eq10_e1047_d_n2, eq10_e1047_d_n3, eq10_e1047_d_n4, eq10_e1047_d_n5, eq10_e1047_d_n6, eq10_e1047_d_n7, eq10_e1047_d_n8, eq10_e1047_d_n9, eq10_e1047_d_n10, eq10_e1047_d_n11];
        let eq10_branch_derivatives: [f64; 7] = [eq10_e1047_d_b0, eq10_e1047_d_b1, eq10_e1047_d_b2, eq10_e1047_d_b3, eq10_e1047_d_b4, eq10_e1047_d_b5, eq10_e1047_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq10_value),
            &eq10_node_derivatives,
            &eq10_branch_derivatives,
            multiplicity,
        );
        let eq11_e1050: f64 = (s.v[0] * s.v[15]);
        let eq11_e1050_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq11_e1050_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq11_e1050_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq11_e1050_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq11_e1050_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq11_e1050_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq11_e1050_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq11_e1050_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq11_e1050_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq11_e1050_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq11_e1050_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq11_e1050_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq11_e1050_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq11_e1050_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq11_e1050_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq11_e1050_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq11_e1050_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq11_e1050_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq11_e1050_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq11_e1052: f64 = (eq11_e1050 * p.p32);
        let eq11_e1052_d_n0: f64 = (eq11_e1050_d_n0 * p.p32);
        let eq11_e1052_d_n1: f64 = (eq11_e1050_d_n1 * p.p32);
        let eq11_e1052_d_n2: f64 = (eq11_e1050_d_n2 * p.p32);
        let eq11_e1052_d_n3: f64 = (eq11_e1050_d_n3 * p.p32);
        let eq11_e1052_d_n4: f64 = (eq11_e1050_d_n4 * p.p32);
        let eq11_e1052_d_n5: f64 = (eq11_e1050_d_n5 * p.p32);
        let eq11_e1052_d_n6: f64 = (eq11_e1050_d_n6 * p.p32);
        let eq11_e1052_d_n7: f64 = (eq11_e1050_d_n7 * p.p32);
        let eq11_e1052_d_n8: f64 = (eq11_e1050_d_n8 * p.p32);
        let eq11_e1052_d_n9: f64 = (eq11_e1050_d_n9 * p.p32);
        let eq11_e1052_d_n10: f64 = (eq11_e1050_d_n10 * p.p32);
        let eq11_e1052_d_n11: f64 = (eq11_e1050_d_n11 * p.p32);
        let eq11_e1052_d_b0: f64 = (eq11_e1050_d_b0 * p.p32);
        let eq11_e1052_d_b1: f64 = (eq11_e1050_d_b1 * p.p32);
        let eq11_e1052_d_b2: f64 = (eq11_e1050_d_b2 * p.p32);
        let eq11_e1052_d_b3: f64 = (eq11_e1050_d_b3 * p.p32);
        let eq11_e1052_d_b4: f64 = (eq11_e1050_d_b4 * p.p32);
        let eq11_e1052_d_b5: f64 = (eq11_e1050_d_b5 * p.p32);
        let eq11_e1052_d_b6: f64 = (eq11_e1050_d_b6 * p.p32);
        let eq11_e1054: f64 = (eq11_e1052 * s.v[838]);
        let eq11_e1054_d_n0: f64 = ((eq11_e1052_d_n0 * s.v[838]) + (eq11_e1052 * s.dn[838][0]));
        let eq11_e1054_d_n1: f64 = ((eq11_e1052_d_n1 * s.v[838]) + (eq11_e1052 * s.dn[838][1]));
        let eq11_e1054_d_n2: f64 = ((eq11_e1052_d_n2 * s.v[838]) + (eq11_e1052 * s.dn[838][2]));
        let eq11_e1054_d_n3: f64 = ((eq11_e1052_d_n3 * s.v[838]) + (eq11_e1052 * s.dn[838][3]));
        let eq11_e1054_d_n4: f64 = ((eq11_e1052_d_n4 * s.v[838]) + (eq11_e1052 * s.dn[838][4]));
        let eq11_e1054_d_n5: f64 = ((eq11_e1052_d_n5 * s.v[838]) + (eq11_e1052 * s.dn[838][5]));
        let eq11_e1054_d_n6: f64 = ((eq11_e1052_d_n6 * s.v[838]) + (eq11_e1052 * s.dn[838][6]));
        let eq11_e1054_d_n7: f64 = ((eq11_e1052_d_n7 * s.v[838]) + (eq11_e1052 * s.dn[838][7]));
        let eq11_e1054_d_n8: f64 = ((eq11_e1052_d_n8 * s.v[838]) + (eq11_e1052 * s.dn[838][8]));
        let eq11_e1054_d_n9: f64 = ((eq11_e1052_d_n9 * s.v[838]) + (eq11_e1052 * s.dn[838][9]));
        let eq11_e1054_d_n10: f64 = ((eq11_e1052_d_n10 * s.v[838]) + (eq11_e1052 * s.dn[838][10]));
        let eq11_e1054_d_n11: f64 = ((eq11_e1052_d_n11 * s.v[838]) + (eq11_e1052 * s.dn[838][11]));
        let eq11_e1054_d_b0: f64 = ((eq11_e1052_d_b0 * s.v[838]) + (eq11_e1052 * s.db[838][0]));
        let eq11_e1054_d_b1: f64 = ((eq11_e1052_d_b1 * s.v[838]) + (eq11_e1052 * s.db[838][1]));
        let eq11_e1054_d_b2: f64 = ((eq11_e1052_d_b2 * s.v[838]) + (eq11_e1052 * s.db[838][2]));
        let eq11_e1054_d_b3: f64 = ((eq11_e1052_d_b3 * s.v[838]) + (eq11_e1052 * s.db[838][3]));
        let eq11_e1054_d_b4: f64 = ((eq11_e1052_d_b4 * s.v[838]) + (eq11_e1052 * s.db[838][4]));
        let eq11_e1054_d_b5: f64 = ((eq11_e1052_d_b5 * s.v[838]) + (eq11_e1052 * s.db[838][5]));
        let eq11_e1054_d_b6: f64 = ((eq11_e1052_d_b6 * s.v[838]) + (eq11_e1052 * s.db[838][6]));
        let eq11_value: f64 = eq11_e1054;
        let eq11_node_derivatives: [f64; 12] = [eq11_e1054_d_n0, eq11_e1054_d_n1, eq11_e1054_d_n2, eq11_e1054_d_n3, eq11_e1054_d_n4, eq11_e1054_d_n5, eq11_e1054_d_n6, eq11_e1054_d_n7, eq11_e1054_d_n8, eq11_e1054_d_n9, eq11_e1054_d_n10, eq11_e1054_d_n11];
        let eq11_branch_derivatives: [f64; 7] = [eq11_e1054_d_b0, eq11_e1054_d_b1, eq11_e1054_d_b2, eq11_e1054_d_b3, eq11_e1054_d_b4, eq11_e1054_d_b5, eq11_e1054_d_b6];
        stamper.stamp_current_dense_local(
            Some(6),
            Some(8),
            multiplicity * (eq11_value),
            &eq11_node_derivatives,
            &eq11_branch_derivatives,
            multiplicity,
        );
        let eq12_e1057: f64 = (s.v[0] * s.v[15]);
        let eq12_e1057_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq12_e1057_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq12_e1057_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq12_e1057_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq12_e1057_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq12_e1057_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq12_e1057_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq12_e1057_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq12_e1057_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq12_e1057_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq12_e1057_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq12_e1057_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq12_e1057_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq12_e1057_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq12_e1057_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq12_e1057_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq12_e1057_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq12_e1057_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq12_e1057_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq12_e1059: f64 = (eq12_e1057 * p.p32);
        let eq12_e1059_d_n0: f64 = (eq12_e1057_d_n0 * p.p32);
        let eq12_e1059_d_n1: f64 = (eq12_e1057_d_n1 * p.p32);
        let eq12_e1059_d_n2: f64 = (eq12_e1057_d_n2 * p.p32);
        let eq12_e1059_d_n3: f64 = (eq12_e1057_d_n3 * p.p32);
        let eq12_e1059_d_n4: f64 = (eq12_e1057_d_n4 * p.p32);
        let eq12_e1059_d_n5: f64 = (eq12_e1057_d_n5 * p.p32);
        let eq12_e1059_d_n6: f64 = (eq12_e1057_d_n6 * p.p32);
        let eq12_e1059_d_n7: f64 = (eq12_e1057_d_n7 * p.p32);
        let eq12_e1059_d_n8: f64 = (eq12_e1057_d_n8 * p.p32);
        let eq12_e1059_d_n9: f64 = (eq12_e1057_d_n9 * p.p32);
        let eq12_e1059_d_n10: f64 = (eq12_e1057_d_n10 * p.p32);
        let eq12_e1059_d_n11: f64 = (eq12_e1057_d_n11 * p.p32);
        let eq12_e1059_d_b0: f64 = (eq12_e1057_d_b0 * p.p32);
        let eq12_e1059_d_b1: f64 = (eq12_e1057_d_b1 * p.p32);
        let eq12_e1059_d_b2: f64 = (eq12_e1057_d_b2 * p.p32);
        let eq12_e1059_d_b3: f64 = (eq12_e1057_d_b3 * p.p32);
        let eq12_e1059_d_b4: f64 = (eq12_e1057_d_b4 * p.p32);
        let eq12_e1059_d_b5: f64 = (eq12_e1057_d_b5 * p.p32);
        let eq12_e1059_d_b6: f64 = (eq12_e1057_d_b6 * p.p32);
        let eq12_e1061: f64 = (eq12_e1059 * s.v[839]);
        let eq12_e1061_d_n0: f64 = ((eq12_e1059_d_n0 * s.v[839]) + (eq12_e1059 * s.dn[839][0]));
        let eq12_e1061_d_n1: f64 = ((eq12_e1059_d_n1 * s.v[839]) + (eq12_e1059 * s.dn[839][1]));
        let eq12_e1061_d_n2: f64 = ((eq12_e1059_d_n2 * s.v[839]) + (eq12_e1059 * s.dn[839][2]));
        let eq12_e1061_d_n3: f64 = ((eq12_e1059_d_n3 * s.v[839]) + (eq12_e1059 * s.dn[839][3]));
        let eq12_e1061_d_n4: f64 = ((eq12_e1059_d_n4 * s.v[839]) + (eq12_e1059 * s.dn[839][4]));
        let eq12_e1061_d_n5: f64 = ((eq12_e1059_d_n5 * s.v[839]) + (eq12_e1059 * s.dn[839][5]));
        let eq12_e1061_d_n6: f64 = ((eq12_e1059_d_n6 * s.v[839]) + (eq12_e1059 * s.dn[839][6]));
        let eq12_e1061_d_n7: f64 = ((eq12_e1059_d_n7 * s.v[839]) + (eq12_e1059 * s.dn[839][7]));
        let eq12_e1061_d_n8: f64 = ((eq12_e1059_d_n8 * s.v[839]) + (eq12_e1059 * s.dn[839][8]));
        let eq12_e1061_d_n9: f64 = ((eq12_e1059_d_n9 * s.v[839]) + (eq12_e1059 * s.dn[839][9]));
        let eq12_e1061_d_n10: f64 = ((eq12_e1059_d_n10 * s.v[839]) + (eq12_e1059 * s.dn[839][10]));
        let eq12_e1061_d_n11: f64 = ((eq12_e1059_d_n11 * s.v[839]) + (eq12_e1059 * s.dn[839][11]));
        let eq12_e1061_d_b0: f64 = ((eq12_e1059_d_b0 * s.v[839]) + (eq12_e1059 * s.db[839][0]));
        let eq12_e1061_d_b1: f64 = ((eq12_e1059_d_b1 * s.v[839]) + (eq12_e1059 * s.db[839][1]));
        let eq12_e1061_d_b2: f64 = ((eq12_e1059_d_b2 * s.v[839]) + (eq12_e1059 * s.db[839][2]));
        let eq12_e1061_d_b3: f64 = ((eq12_e1059_d_b3 * s.v[839]) + (eq12_e1059 * s.db[839][3]));
        let eq12_e1061_d_b4: f64 = ((eq12_e1059_d_b4 * s.v[839]) + (eq12_e1059 * s.db[839][4]));
        let eq12_e1061_d_b5: f64 = ((eq12_e1059_d_b5 * s.v[839]) + (eq12_e1059 * s.db[839][5]));
        let eq12_e1061_d_b6: f64 = ((eq12_e1059_d_b6 * s.v[839]) + (eq12_e1059 * s.db[839][6]));
        let eq12_value: f64 = eq12_e1061;
        let eq12_node_derivatives: [f64; 12] = [eq12_e1061_d_n0, eq12_e1061_d_n1, eq12_e1061_d_n2, eq12_e1061_d_n3, eq12_e1061_d_n4, eq12_e1061_d_n5, eq12_e1061_d_n6, eq12_e1061_d_n7, eq12_e1061_d_n8, eq12_e1061_d_n9, eq12_e1061_d_n10, eq12_e1061_d_n11];
        let eq12_branch_derivatives: [f64; 7] = [eq12_e1061_d_b0, eq12_e1061_d_b1, eq12_e1061_d_b2, eq12_e1061_d_b3, eq12_e1061_d_b4, eq12_e1061_d_b5, eq12_e1061_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(8),
            multiplicity * (eq12_value),
            &eq12_node_derivatives,
            &eq12_branch_derivatives,
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
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv1 = ctx.node_voltage(nodes[1]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv5 = ctx.node_voltage(nodes[5]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let eq13_e1064: f64 = (s.v[0] * s.v[15]);
        let eq13_e1064_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq13_e1064_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq13_e1064_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq13_e1064_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq13_e1064_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq13_e1064_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq13_e1064_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq13_e1064_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq13_e1064_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq13_e1064_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq13_e1064_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq13_e1064_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq13_e1064_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq13_e1064_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq13_e1064_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq13_e1064_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq13_e1064_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq13_e1064_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq13_e1064_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq13_e1066: f64 = (eq13_e1064 * p.p32);
        let eq13_e1066_d_n0: f64 = (eq13_e1064_d_n0 * p.p32);
        let eq13_e1066_d_n1: f64 = (eq13_e1064_d_n1 * p.p32);
        let eq13_e1066_d_n2: f64 = (eq13_e1064_d_n2 * p.p32);
        let eq13_e1066_d_n3: f64 = (eq13_e1064_d_n3 * p.p32);
        let eq13_e1066_d_n4: f64 = (eq13_e1064_d_n4 * p.p32);
        let eq13_e1066_d_n5: f64 = (eq13_e1064_d_n5 * p.p32);
        let eq13_e1066_d_n6: f64 = (eq13_e1064_d_n6 * p.p32);
        let eq13_e1066_d_n7: f64 = (eq13_e1064_d_n7 * p.p32);
        let eq13_e1066_d_n8: f64 = (eq13_e1064_d_n8 * p.p32);
        let eq13_e1066_d_n9: f64 = (eq13_e1064_d_n9 * p.p32);
        let eq13_e1066_d_n10: f64 = (eq13_e1064_d_n10 * p.p32);
        let eq13_e1066_d_n11: f64 = (eq13_e1064_d_n11 * p.p32);
        let eq13_e1066_d_b0: f64 = (eq13_e1064_d_b0 * p.p32);
        let eq13_e1066_d_b1: f64 = (eq13_e1064_d_b1 * p.p32);
        let eq13_e1066_d_b2: f64 = (eq13_e1064_d_b2 * p.p32);
        let eq13_e1066_d_b3: f64 = (eq13_e1064_d_b3 * p.p32);
        let eq13_e1066_d_b4: f64 = (eq13_e1064_d_b4 * p.p32);
        let eq13_e1066_d_b5: f64 = (eq13_e1064_d_b5 * p.p32);
        let eq13_e1066_d_b6: f64 = (eq13_e1064_d_b6 * p.p32);
        let eq13_e1068: f64 = (eq13_e1066 * s.v[842]);
        let eq13_e1068_d_n0: f64 = ((eq13_e1066_d_n0 * s.v[842]) + (eq13_e1066 * s.dn[842][0]));
        let eq13_e1068_d_n1: f64 = ((eq13_e1066_d_n1 * s.v[842]) + (eq13_e1066 * s.dn[842][1]));
        let eq13_e1068_d_n2: f64 = ((eq13_e1066_d_n2 * s.v[842]) + (eq13_e1066 * s.dn[842][2]));
        let eq13_e1068_d_n3: f64 = ((eq13_e1066_d_n3 * s.v[842]) + (eq13_e1066 * s.dn[842][3]));
        let eq13_e1068_d_n4: f64 = ((eq13_e1066_d_n4 * s.v[842]) + (eq13_e1066 * s.dn[842][4]));
        let eq13_e1068_d_n5: f64 = ((eq13_e1066_d_n5 * s.v[842]) + (eq13_e1066 * s.dn[842][5]));
        let eq13_e1068_d_n6: f64 = ((eq13_e1066_d_n6 * s.v[842]) + (eq13_e1066 * s.dn[842][6]));
        let eq13_e1068_d_n7: f64 = ((eq13_e1066_d_n7 * s.v[842]) + (eq13_e1066 * s.dn[842][7]));
        let eq13_e1068_d_n8: f64 = ((eq13_e1066_d_n8 * s.v[842]) + (eq13_e1066 * s.dn[842][8]));
        let eq13_e1068_d_n9: f64 = ((eq13_e1066_d_n9 * s.v[842]) + (eq13_e1066 * s.dn[842][9]));
        let eq13_e1068_d_n10: f64 = ((eq13_e1066_d_n10 * s.v[842]) + (eq13_e1066 * s.dn[842][10]));
        let eq13_e1068_d_n11: f64 = ((eq13_e1066_d_n11 * s.v[842]) + (eq13_e1066 * s.dn[842][11]));
        let eq13_e1068_d_b0: f64 = ((eq13_e1066_d_b0 * s.v[842]) + (eq13_e1066 * s.db[842][0]));
        let eq13_e1068_d_b1: f64 = ((eq13_e1066_d_b1 * s.v[842]) + (eq13_e1066 * s.db[842][1]));
        let eq13_e1068_d_b2: f64 = ((eq13_e1066_d_b2 * s.v[842]) + (eq13_e1066 * s.db[842][2]));
        let eq13_e1068_d_b3: f64 = ((eq13_e1066_d_b3 * s.v[842]) + (eq13_e1066 * s.db[842][3]));
        let eq13_e1068_d_b4: f64 = ((eq13_e1066_d_b4 * s.v[842]) + (eq13_e1066 * s.db[842][4]));
        let eq13_e1068_d_b5: f64 = ((eq13_e1066_d_b5 * s.v[842]) + (eq13_e1066 * s.db[842][5]));
        let eq13_e1068_d_b6: f64 = ((eq13_e1066_d_b6 * s.v[842]) + (eq13_e1066 * s.db[842][6]));
        let eq13_value: f64 = eq13_e1068;
        let eq13_node_derivatives: [f64; 12] = [eq13_e1068_d_n0, eq13_e1068_d_n1, eq13_e1068_d_n2, eq13_e1068_d_n3, eq13_e1068_d_n4, eq13_e1068_d_n5, eq13_e1068_d_n6, eq13_e1068_d_n7, eq13_e1068_d_n8, eq13_e1068_d_n9, eq13_e1068_d_n10, eq13_e1068_d_n11];
        let eq13_branch_derivatives: [f64; 7] = [eq13_e1068_d_b0, eq13_e1068_d_b1, eq13_e1068_d_b2, eq13_e1068_d_b3, eq13_e1068_d_b4, eq13_e1068_d_b5, eq13_e1068_d_b6];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq13_value),
            &eq13_node_derivatives,
            &eq13_branch_derivatives,
            multiplicity,
        );
        let eq14_e1071: f64 = (s.v[0] * s.v[15]);
        let eq14_e1071_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq14_e1071_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq14_e1071_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq14_e1071_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq14_e1071_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq14_e1071_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq14_e1071_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq14_e1071_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq14_e1071_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq14_e1071_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq14_e1071_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq14_e1071_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq14_e1071_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq14_e1071_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq14_e1071_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq14_e1071_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq14_e1071_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq14_e1071_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq14_e1071_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq14_e1073: f64 = (eq14_e1071 * p.p32);
        let eq14_e1073_d_n0: f64 = (eq14_e1071_d_n0 * p.p32);
        let eq14_e1073_d_n1: f64 = (eq14_e1071_d_n1 * p.p32);
        let eq14_e1073_d_n2: f64 = (eq14_e1071_d_n2 * p.p32);
        let eq14_e1073_d_n3: f64 = (eq14_e1071_d_n3 * p.p32);
        let eq14_e1073_d_n4: f64 = (eq14_e1071_d_n4 * p.p32);
        let eq14_e1073_d_n5: f64 = (eq14_e1071_d_n5 * p.p32);
        let eq14_e1073_d_n6: f64 = (eq14_e1071_d_n6 * p.p32);
        let eq14_e1073_d_n7: f64 = (eq14_e1071_d_n7 * p.p32);
        let eq14_e1073_d_n8: f64 = (eq14_e1071_d_n8 * p.p32);
        let eq14_e1073_d_n9: f64 = (eq14_e1071_d_n9 * p.p32);
        let eq14_e1073_d_n10: f64 = (eq14_e1071_d_n10 * p.p32);
        let eq14_e1073_d_n11: f64 = (eq14_e1071_d_n11 * p.p32);
        let eq14_e1073_d_b0: f64 = (eq14_e1071_d_b0 * p.p32);
        let eq14_e1073_d_b1: f64 = (eq14_e1071_d_b1 * p.p32);
        let eq14_e1073_d_b2: f64 = (eq14_e1071_d_b2 * p.p32);
        let eq14_e1073_d_b3: f64 = (eq14_e1071_d_b3 * p.p32);
        let eq14_e1073_d_b4: f64 = (eq14_e1071_d_b4 * p.p32);
        let eq14_e1073_d_b5: f64 = (eq14_e1071_d_b5 * p.p32);
        let eq14_e1073_d_b6: f64 = (eq14_e1071_d_b6 * p.p32);
        let eq14_e1075: f64 = (eq14_e1073 * s.v[843]);
        let eq14_e1075_d_n0: f64 = ((eq14_e1073_d_n0 * s.v[843]) + (eq14_e1073 * s.dn[843][0]));
        let eq14_e1075_d_n1: f64 = ((eq14_e1073_d_n1 * s.v[843]) + (eq14_e1073 * s.dn[843][1]));
        let eq14_e1075_d_n2: f64 = ((eq14_e1073_d_n2 * s.v[843]) + (eq14_e1073 * s.dn[843][2]));
        let eq14_e1075_d_n3: f64 = ((eq14_e1073_d_n3 * s.v[843]) + (eq14_e1073 * s.dn[843][3]));
        let eq14_e1075_d_n4: f64 = ((eq14_e1073_d_n4 * s.v[843]) + (eq14_e1073 * s.dn[843][4]));
        let eq14_e1075_d_n5: f64 = ((eq14_e1073_d_n5 * s.v[843]) + (eq14_e1073 * s.dn[843][5]));
        let eq14_e1075_d_n6: f64 = ((eq14_e1073_d_n6 * s.v[843]) + (eq14_e1073 * s.dn[843][6]));
        let eq14_e1075_d_n7: f64 = ((eq14_e1073_d_n7 * s.v[843]) + (eq14_e1073 * s.dn[843][7]));
        let eq14_e1075_d_n8: f64 = ((eq14_e1073_d_n8 * s.v[843]) + (eq14_e1073 * s.dn[843][8]));
        let eq14_e1075_d_n9: f64 = ((eq14_e1073_d_n9 * s.v[843]) + (eq14_e1073 * s.dn[843][9]));
        let eq14_e1075_d_n10: f64 = ((eq14_e1073_d_n10 * s.v[843]) + (eq14_e1073 * s.dn[843][10]));
        let eq14_e1075_d_n11: f64 = ((eq14_e1073_d_n11 * s.v[843]) + (eq14_e1073 * s.dn[843][11]));
        let eq14_e1075_d_b0: f64 = ((eq14_e1073_d_b0 * s.v[843]) + (eq14_e1073 * s.db[843][0]));
        let eq14_e1075_d_b1: f64 = ((eq14_e1073_d_b1 * s.v[843]) + (eq14_e1073 * s.db[843][1]));
        let eq14_e1075_d_b2: f64 = ((eq14_e1073_d_b2 * s.v[843]) + (eq14_e1073 * s.db[843][2]));
        let eq14_e1075_d_b3: f64 = ((eq14_e1073_d_b3 * s.v[843]) + (eq14_e1073 * s.db[843][3]));
        let eq14_e1075_d_b4: f64 = ((eq14_e1073_d_b4 * s.v[843]) + (eq14_e1073 * s.db[843][4]));
        let eq14_e1075_d_b5: f64 = ((eq14_e1073_d_b5 * s.v[843]) + (eq14_e1073 * s.db[843][5]));
        let eq14_e1075_d_b6: f64 = ((eq14_e1073_d_b6 * s.v[843]) + (eq14_e1073 * s.db[843][6]));
        let eq14_value: f64 = eq14_e1075;
        let eq14_node_derivatives: [f64; 12] = [eq14_e1075_d_n0, eq14_e1075_d_n1, eq14_e1075_d_n2, eq14_e1075_d_n3, eq14_e1075_d_n4, eq14_e1075_d_n5, eq14_e1075_d_n6, eq14_e1075_d_n7, eq14_e1075_d_n8, eq14_e1075_d_n9, eq14_e1075_d_n10, eq14_e1075_d_n11];
        let eq14_branch_derivatives: [f64; 7] = [eq14_e1075_d_b0, eq14_e1075_d_b1, eq14_e1075_d_b2, eq14_e1075_d_b3, eq14_e1075_d_b4, eq14_e1075_d_b5, eq14_e1075_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq14_value),
            &eq14_node_derivatives,
            &eq14_branch_derivatives,
            multiplicity,
        );
        let (eq15_e1085, eq15_e1085_d_n0, eq15_e1085_d_n1, eq15_e1085_d_n2, eq15_e1085_d_n3, eq15_e1085_d_n4, eq15_e1085_d_n5, eq15_e1085_d_n6, eq15_e1085_d_n7, eq15_e1085_d_n8, eq15_e1085_d_n9, eq15_e1085_d_n10, eq15_e1085_d_n11, eq15_e1085_d_b0, eq15_e1085_d_b1, eq15_e1085_d_b2, eq15_e1085_d_b3, eq15_e1085_d_b4, eq15_e1085_d_b5, eq15_e1085_d_b6,) = {
    if s.b[2702] {
        let eq15_e1079: f64 = (s.v[15] * p.p32);
        let eq15_e1079_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq15_e1079_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq15_e1079_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq15_e1079_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq15_e1079_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq15_e1079_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq15_e1079_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq15_e1079_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq15_e1079_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq15_e1079_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq15_e1079_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq15_e1079_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq15_e1079_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq15_e1079_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq15_e1079_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq15_e1079_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq15_e1079_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq15_e1079_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq15_e1079_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq15_e1081: f64 = (eq15_e1079 * s.v[805]);
        let eq15_e1081_d_n0: f64 = ((eq15_e1079_d_n0 * s.v[805]) + (eq15_e1079 * s.dn[805][0]));
        let eq15_e1081_d_n1: f64 = ((eq15_e1079_d_n1 * s.v[805]) + (eq15_e1079 * s.dn[805][1]));
        let eq15_e1081_d_n2: f64 = ((eq15_e1079_d_n2 * s.v[805]) + (eq15_e1079 * s.dn[805][2]));
        let eq15_e1081_d_n3: f64 = ((eq15_e1079_d_n3 * s.v[805]) + (eq15_e1079 * s.dn[805][3]));
        let eq15_e1081_d_n4: f64 = ((eq15_e1079_d_n4 * s.v[805]) + (eq15_e1079 * s.dn[805][4]));
        let eq15_e1081_d_n5: f64 = ((eq15_e1079_d_n5 * s.v[805]) + (eq15_e1079 * s.dn[805][5]));
        let eq15_e1081_d_n6: f64 = ((eq15_e1079_d_n6 * s.v[805]) + (eq15_e1079 * s.dn[805][6]));
        let eq15_e1081_d_n7: f64 = ((eq15_e1079_d_n7 * s.v[805]) + (eq15_e1079 * s.dn[805][7]));
        let eq15_e1081_d_n8: f64 = ((eq15_e1079_d_n8 * s.v[805]) + (eq15_e1079 * s.dn[805][8]));
        let eq15_e1081_d_n9: f64 = ((eq15_e1079_d_n9 * s.v[805]) + (eq15_e1079 * s.dn[805][9]));
        let eq15_e1081_d_n10: f64 = ((eq15_e1079_d_n10 * s.v[805]) + (eq15_e1079 * s.dn[805][10]));
        let eq15_e1081_d_n11: f64 = ((eq15_e1079_d_n11 * s.v[805]) + (eq15_e1079 * s.dn[805][11]));
        let eq15_e1081_d_b0: f64 = ((eq15_e1079_d_b0 * s.v[805]) + (eq15_e1079 * s.db[805][0]));
        let eq15_e1081_d_b1: f64 = ((eq15_e1079_d_b1 * s.v[805]) + (eq15_e1079 * s.db[805][1]));
        let eq15_e1081_d_b2: f64 = ((eq15_e1079_d_b2 * s.v[805]) + (eq15_e1079 * s.db[805][2]));
        let eq15_e1081_d_b3: f64 = ((eq15_e1079_d_b3 * s.v[805]) + (eq15_e1079 * s.db[805][3]));
        let eq15_e1081_d_b4: f64 = ((eq15_e1079_d_b4 * s.v[805]) + (eq15_e1079 * s.db[805][4]));
        let eq15_e1081_d_b5: f64 = ((eq15_e1079_d_b5 * s.v[805]) + (eq15_e1079 * s.db[805][5]));
        let eq15_e1081_d_b6: f64 = ((eq15_e1079_d_b6 * s.v[805]) + (eq15_e1079 * s.db[805][6]));
        let eq15_e1083: f64 = (eq15_e1081 * (nv1 - nv5));
        let eq15_e1083_d_n0: f64 = (eq15_e1081_d_n0 * (nv1 - nv5));
        let eq15_e1083_d_n1: f64 = ((eq15_e1081_d_n1 * (nv1 - nv5)) + eq15_e1081);
        let eq15_e1083_d_n2: f64 = (eq15_e1081_d_n2 * (nv1 - nv5));
        let eq15_e1083_d_n3: f64 = (eq15_e1081_d_n3 * (nv1 - nv5));
        let eq15_e1083_d_n4: f64 = (eq15_e1081_d_n4 * (nv1 - nv5));
        let eq15_e1083_d_n5: f64 = ((eq15_e1081_d_n5 * (nv1 - nv5)) + (-eq15_e1081));
        let eq15_e1083_d_n6: f64 = (eq15_e1081_d_n6 * (nv1 - nv5));
        let eq15_e1083_d_n7: f64 = (eq15_e1081_d_n7 * (nv1 - nv5));
        let eq15_e1083_d_n8: f64 = (eq15_e1081_d_n8 * (nv1 - nv5));
        let eq15_e1083_d_n9: f64 = (eq15_e1081_d_n9 * (nv1 - nv5));
        let eq15_e1083_d_n10: f64 = (eq15_e1081_d_n10 * (nv1 - nv5));
        let eq15_e1083_d_n11: f64 = (eq15_e1081_d_n11 * (nv1 - nv5));
        let eq15_e1083_d_b0: f64 = (eq15_e1081_d_b0 * (nv1 - nv5));
        let eq15_e1083_d_b1: f64 = (eq15_e1081_d_b1 * (nv1 - nv5));
        let eq15_e1083_d_b2: f64 = (eq15_e1081_d_b2 * (nv1 - nv5));
        let eq15_e1083_d_b3: f64 = (eq15_e1081_d_b3 * (nv1 - nv5));
        let eq15_e1083_d_b4: f64 = (eq15_e1081_d_b4 * (nv1 - nv5));
        let eq15_e1083_d_b5: f64 = (eq15_e1081_d_b5 * (nv1 - nv5));
        let eq15_e1083_d_b6: f64 = (eq15_e1081_d_b6 * (nv1 - nv5));
        (eq15_e1083, eq15_e1083_d_n0, eq15_e1083_d_n1, eq15_e1083_d_n2, eq15_e1083_d_n3, eq15_e1083_d_n4, eq15_e1083_d_n5, eq15_e1083_d_n6, eq15_e1083_d_n7, eq15_e1083_d_n8, eq15_e1083_d_n9, eq15_e1083_d_n10, eq15_e1083_d_n11, eq15_e1083_d_b0, eq15_e1083_d_b1, eq15_e1083_d_b2, eq15_e1083_d_b3, eq15_e1083_d_b4, eq15_e1083_d_b5, eq15_e1083_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq15_value: f64 = eq15_e1085;
        let eq15_node_derivatives: [f64; 12] = [eq15_e1085_d_n0, eq15_e1085_d_n1, eq15_e1085_d_n2, eq15_e1085_d_n3, eq15_e1085_d_n4, eq15_e1085_d_n5, eq15_e1085_d_n6, eq15_e1085_d_n7, eq15_e1085_d_n8, eq15_e1085_d_n9, eq15_e1085_d_n10, eq15_e1085_d_n11];
        let eq15_branch_derivatives: [f64; 7] = [eq15_e1085_d_b0, eq15_e1085_d_b1, eq15_e1085_d_b2, eq15_e1085_d_b3, eq15_e1085_d_b4, eq15_e1085_d_b5, eq15_e1085_d_b6];
        stamper.stamp_current_dense_local(
            Some(1),
            Some(5),
            multiplicity * (eq15_value),
            &eq15_node_derivatives,
            &eq15_branch_derivatives,
            multiplicity,
        );
        let (eq16_e1095,) = {
    if s.b[2702] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq16_value: f64 = eq16_e1095;
        stamper.stamp_current_const_local(
            Some(1),
            Some(5),
            multiplicity * (eq16_value),
        );
        let (eq17_e1100,) = {
    if (!s.b[2702]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq17_value: f64 = eq17_e1100;
        stamper.stamp_potential_const_local(
            0,
            eq17_value,
        );
        let (eq18_e1110, eq18_e1110_d_n0, eq18_e1110_d_n1, eq18_e1110_d_n2, eq18_e1110_d_n3, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n11, eq18_e1110_d_b0, eq18_e1110_d_b1, eq18_e1110_d_b2, eq18_e1110_d_b3, eq18_e1110_d_b4, eq18_e1110_d_b5, eq18_e1110_d_b6,) = {
    if s.b[2703] {
        let eq18_e1104: f64 = (s.v[15] * p.p32);
        let eq18_e1104_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq18_e1104_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq18_e1104_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq18_e1104_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq18_e1104_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq18_e1104_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq18_e1104_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq18_e1104_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq18_e1104_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq18_e1104_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq18_e1104_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq18_e1104_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq18_e1104_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq18_e1104_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq18_e1104_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq18_e1104_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq18_e1104_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq18_e1104_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq18_e1104_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq18_e1106: f64 = (eq18_e1104 * s.v[806]);
        let eq18_e1106_d_n0: f64 = ((eq18_e1104_d_n0 * s.v[806]) + (eq18_e1104 * s.dn[806][0]));
        let eq18_e1106_d_n1: f64 = ((eq18_e1104_d_n1 * s.v[806]) + (eq18_e1104 * s.dn[806][1]));
        let eq18_e1106_d_n2: f64 = ((eq18_e1104_d_n2 * s.v[806]) + (eq18_e1104 * s.dn[806][2]));
        let eq18_e1106_d_n3: f64 = ((eq18_e1104_d_n3 * s.v[806]) + (eq18_e1104 * s.dn[806][3]));
        let eq18_e1106_d_n4: f64 = ((eq18_e1104_d_n4 * s.v[806]) + (eq18_e1104 * s.dn[806][4]));
        let eq18_e1106_d_n5: f64 = ((eq18_e1104_d_n5 * s.v[806]) + (eq18_e1104 * s.dn[806][5]));
        let eq18_e1106_d_n6: f64 = ((eq18_e1104_d_n6 * s.v[806]) + (eq18_e1104 * s.dn[806][6]));
        let eq18_e1106_d_n7: f64 = ((eq18_e1104_d_n7 * s.v[806]) + (eq18_e1104 * s.dn[806][7]));
        let eq18_e1106_d_n8: f64 = ((eq18_e1104_d_n8 * s.v[806]) + (eq18_e1104 * s.dn[806][8]));
        let eq18_e1106_d_n9: f64 = ((eq18_e1104_d_n9 * s.v[806]) + (eq18_e1104 * s.dn[806][9]));
        let eq18_e1106_d_n10: f64 = ((eq18_e1104_d_n10 * s.v[806]) + (eq18_e1104 * s.dn[806][10]));
        let eq18_e1106_d_n11: f64 = ((eq18_e1104_d_n11 * s.v[806]) + (eq18_e1104 * s.dn[806][11]));
        let eq18_e1106_d_b0: f64 = ((eq18_e1104_d_b0 * s.v[806]) + (eq18_e1104 * s.db[806][0]));
        let eq18_e1106_d_b1: f64 = ((eq18_e1104_d_b1 * s.v[806]) + (eq18_e1104 * s.db[806][1]));
        let eq18_e1106_d_b2: f64 = ((eq18_e1104_d_b2 * s.v[806]) + (eq18_e1104 * s.db[806][2]));
        let eq18_e1106_d_b3: f64 = ((eq18_e1104_d_b3 * s.v[806]) + (eq18_e1104 * s.db[806][3]));
        let eq18_e1106_d_b4: f64 = ((eq18_e1104_d_b4 * s.v[806]) + (eq18_e1104 * s.db[806][4]));
        let eq18_e1106_d_b5: f64 = ((eq18_e1104_d_b5 * s.v[806]) + (eq18_e1104 * s.db[806][5]));
        let eq18_e1106_d_b6: f64 = ((eq18_e1104_d_b6 * s.v[806]) + (eq18_e1104 * s.db[806][6]));
        let eq18_e1108: f64 = (eq18_e1106 * (nv2 - nv6));
        let eq18_e1108_d_n0: f64 = (eq18_e1106_d_n0 * (nv2 - nv6));
        let eq18_e1108_d_n1: f64 = (eq18_e1106_d_n1 * (nv2 - nv6));
        let eq18_e1108_d_n2: f64 = ((eq18_e1106_d_n2 * (nv2 - nv6)) + eq18_e1106);
        let eq18_e1108_d_n3: f64 = (eq18_e1106_d_n3 * (nv2 - nv6));
        let eq18_e1108_d_n4: f64 = (eq18_e1106_d_n4 * (nv2 - nv6));
        let eq18_e1108_d_n5: f64 = (eq18_e1106_d_n5 * (nv2 - nv6));
        let eq18_e1108_d_n6: f64 = ((eq18_e1106_d_n6 * (nv2 - nv6)) + (-eq18_e1106));
        let eq18_e1108_d_n7: f64 = (eq18_e1106_d_n7 * (nv2 - nv6));
        let eq18_e1108_d_n8: f64 = (eq18_e1106_d_n8 * (nv2 - nv6));
        let eq18_e1108_d_n9: f64 = (eq18_e1106_d_n9 * (nv2 - nv6));
        let eq18_e1108_d_n10: f64 = (eq18_e1106_d_n10 * (nv2 - nv6));
        let eq18_e1108_d_n11: f64 = (eq18_e1106_d_n11 * (nv2 - nv6));
        let eq18_e1108_d_b0: f64 = (eq18_e1106_d_b0 * (nv2 - nv6));
        let eq18_e1108_d_b1: f64 = (eq18_e1106_d_b1 * (nv2 - nv6));
        let eq18_e1108_d_b2: f64 = (eq18_e1106_d_b2 * (nv2 - nv6));
        let eq18_e1108_d_b3: f64 = (eq18_e1106_d_b3 * (nv2 - nv6));
        let eq18_e1108_d_b4: f64 = (eq18_e1106_d_b4 * (nv2 - nv6));
        let eq18_e1108_d_b5: f64 = (eq18_e1106_d_b5 * (nv2 - nv6));
        let eq18_e1108_d_b6: f64 = (eq18_e1106_d_b6 * (nv2 - nv6));
        (eq18_e1108, eq18_e1108_d_n0, eq18_e1108_d_n1, eq18_e1108_d_n2, eq18_e1108_d_n3, eq18_e1108_d_n4, eq18_e1108_d_n5, eq18_e1108_d_n6, eq18_e1108_d_n7, eq18_e1108_d_n8, eq18_e1108_d_n9, eq18_e1108_d_n10, eq18_e1108_d_n11, eq18_e1108_d_b0, eq18_e1108_d_b1, eq18_e1108_d_b2, eq18_e1108_d_b3, eq18_e1108_d_b4, eq18_e1108_d_b5, eq18_e1108_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq18_value: f64 = eq18_e1110;
        let eq18_node_derivatives: [f64; 12] = [eq18_e1110_d_n0, eq18_e1110_d_n1, eq18_e1110_d_n2, eq18_e1110_d_n3, eq18_e1110_d_n4, eq18_e1110_d_n5, eq18_e1110_d_n6, eq18_e1110_d_n7, eq18_e1110_d_n8, eq18_e1110_d_n9, eq18_e1110_d_n10, eq18_e1110_d_n11];
        let eq18_branch_derivatives: [f64; 7] = [eq18_e1110_d_b0, eq18_e1110_d_b1, eq18_e1110_d_b2, eq18_e1110_d_b3, eq18_e1110_d_b4, eq18_e1110_d_b5, eq18_e1110_d_b6];
        stamper.stamp_current_dense_local(
            Some(2),
            Some(6),
            multiplicity * (eq18_value),
            &eq18_node_derivatives,
            &eq18_branch_derivatives,
            multiplicity,
        );
        let (eq19_e1120,) = {
    if s.b[2703] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq19_value: f64 = eq19_e1120;
        stamper.stamp_current_const_local(
            Some(2),
            Some(6),
            multiplicity * (eq19_value),
        );
        let (eq20_e1125,) = {
    if (!s.b[2703]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq20_value: f64 = eq20_e1125;
        stamper.stamp_potential_const_local(
            1,
            eq20_value,
        );
        let (eq21_e1135, eq21_e1135_d_n0, eq21_e1135_d_n1, eq21_e1135_d_n2, eq21_e1135_d_n3, eq21_e1135_d_n4, eq21_e1135_d_n5, eq21_e1135_d_n6, eq21_e1135_d_n7, eq21_e1135_d_n8, eq21_e1135_d_n9, eq21_e1135_d_n10, eq21_e1135_d_n11, eq21_e1135_d_b0, eq21_e1135_d_b1, eq21_e1135_d_b2, eq21_e1135_d_b3, eq21_e1135_d_b4, eq21_e1135_d_b5, eq21_e1135_d_b6,) = {
    if s.b[2704] {
        let eq21_e1129: f64 = (s.v[15] * p.p32);
        let eq21_e1129_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq21_e1129_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq21_e1129_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq21_e1129_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq21_e1129_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq21_e1129_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq21_e1129_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq21_e1129_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq21_e1129_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq21_e1129_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq21_e1129_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq21_e1129_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq21_e1129_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq21_e1129_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq21_e1129_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq21_e1129_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq21_e1129_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq21_e1129_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq21_e1129_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq21_e1131: f64 = (eq21_e1129 * s.v[807]);
        let eq21_e1131_d_n0: f64 = ((eq21_e1129_d_n0 * s.v[807]) + (eq21_e1129 * s.dn[807][0]));
        let eq21_e1131_d_n1: f64 = ((eq21_e1129_d_n1 * s.v[807]) + (eq21_e1129 * s.dn[807][1]));
        let eq21_e1131_d_n2: f64 = ((eq21_e1129_d_n2 * s.v[807]) + (eq21_e1129 * s.dn[807][2]));
        let eq21_e1131_d_n3: f64 = ((eq21_e1129_d_n3 * s.v[807]) + (eq21_e1129 * s.dn[807][3]));
        let eq21_e1131_d_n4: f64 = ((eq21_e1129_d_n4 * s.v[807]) + (eq21_e1129 * s.dn[807][4]));
        let eq21_e1131_d_n5: f64 = ((eq21_e1129_d_n5 * s.v[807]) + (eq21_e1129 * s.dn[807][5]));
        let eq21_e1131_d_n6: f64 = ((eq21_e1129_d_n6 * s.v[807]) + (eq21_e1129 * s.dn[807][6]));
        let eq21_e1131_d_n7: f64 = ((eq21_e1129_d_n7 * s.v[807]) + (eq21_e1129 * s.dn[807][7]));
        let eq21_e1131_d_n8: f64 = ((eq21_e1129_d_n8 * s.v[807]) + (eq21_e1129 * s.dn[807][8]));
        let eq21_e1131_d_n9: f64 = ((eq21_e1129_d_n9 * s.v[807]) + (eq21_e1129 * s.dn[807][9]));
        let eq21_e1131_d_n10: f64 = ((eq21_e1129_d_n10 * s.v[807]) + (eq21_e1129 * s.dn[807][10]));
        let eq21_e1131_d_n11: f64 = ((eq21_e1129_d_n11 * s.v[807]) + (eq21_e1129 * s.dn[807][11]));
        let eq21_e1131_d_b0: f64 = ((eq21_e1129_d_b0 * s.v[807]) + (eq21_e1129 * s.db[807][0]));
        let eq21_e1131_d_b1: f64 = ((eq21_e1129_d_b1 * s.v[807]) + (eq21_e1129 * s.db[807][1]));
        let eq21_e1131_d_b2: f64 = ((eq21_e1129_d_b2 * s.v[807]) + (eq21_e1129 * s.db[807][2]));
        let eq21_e1131_d_b3: f64 = ((eq21_e1129_d_b3 * s.v[807]) + (eq21_e1129 * s.db[807][3]));
        let eq21_e1131_d_b4: f64 = ((eq21_e1129_d_b4 * s.v[807]) + (eq21_e1129 * s.db[807][4]));
        let eq21_e1131_d_b5: f64 = ((eq21_e1129_d_b5 * s.v[807]) + (eq21_e1129 * s.db[807][5]));
        let eq21_e1131_d_b6: f64 = ((eq21_e1129_d_b6 * s.v[807]) + (eq21_e1129 * s.db[807][6]));
        let eq21_e1133: f64 = (eq21_e1131 * (nv0 - nv7));
        let eq21_e1133_d_n0: f64 = ((eq21_e1131_d_n0 * (nv0 - nv7)) + eq21_e1131);
        let eq21_e1133_d_n1: f64 = (eq21_e1131_d_n1 * (nv0 - nv7));
        let eq21_e1133_d_n2: f64 = (eq21_e1131_d_n2 * (nv0 - nv7));
        let eq21_e1133_d_n3: f64 = (eq21_e1131_d_n3 * (nv0 - nv7));
        let eq21_e1133_d_n4: f64 = (eq21_e1131_d_n4 * (nv0 - nv7));
        let eq21_e1133_d_n5: f64 = (eq21_e1131_d_n5 * (nv0 - nv7));
        let eq21_e1133_d_n6: f64 = (eq21_e1131_d_n6 * (nv0 - nv7));
        let eq21_e1133_d_n7: f64 = ((eq21_e1131_d_n7 * (nv0 - nv7)) + (-eq21_e1131));
        let eq21_e1133_d_n8: f64 = (eq21_e1131_d_n8 * (nv0 - nv7));
        let eq21_e1133_d_n9: f64 = (eq21_e1131_d_n9 * (nv0 - nv7));
        let eq21_e1133_d_n10: f64 = (eq21_e1131_d_n10 * (nv0 - nv7));
        let eq21_e1133_d_n11: f64 = (eq21_e1131_d_n11 * (nv0 - nv7));
        let eq21_e1133_d_b0: f64 = (eq21_e1131_d_b0 * (nv0 - nv7));
        let eq21_e1133_d_b1: f64 = (eq21_e1131_d_b1 * (nv0 - nv7));
        let eq21_e1133_d_b2: f64 = (eq21_e1131_d_b2 * (nv0 - nv7));
        let eq21_e1133_d_b3: f64 = (eq21_e1131_d_b3 * (nv0 - nv7));
        let eq21_e1133_d_b4: f64 = (eq21_e1131_d_b4 * (nv0 - nv7));
        let eq21_e1133_d_b5: f64 = (eq21_e1131_d_b5 * (nv0 - nv7));
        let eq21_e1133_d_b6: f64 = (eq21_e1131_d_b6 * (nv0 - nv7));
        (eq21_e1133, eq21_e1133_d_n0, eq21_e1133_d_n1, eq21_e1133_d_n2, eq21_e1133_d_n3, eq21_e1133_d_n4, eq21_e1133_d_n5, eq21_e1133_d_n6, eq21_e1133_d_n7, eq21_e1133_d_n8, eq21_e1133_d_n9, eq21_e1133_d_n10, eq21_e1133_d_n11, eq21_e1133_d_b0, eq21_e1133_d_b1, eq21_e1133_d_b2, eq21_e1133_d_b3, eq21_e1133_d_b4, eq21_e1133_d_b5, eq21_e1133_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq21_value: f64 = eq21_e1135;
        let eq21_node_derivatives: [f64; 12] = [eq21_e1135_d_n0, eq21_e1135_d_n1, eq21_e1135_d_n2, eq21_e1135_d_n3, eq21_e1135_d_n4, eq21_e1135_d_n5, eq21_e1135_d_n6, eq21_e1135_d_n7, eq21_e1135_d_n8, eq21_e1135_d_n9, eq21_e1135_d_n10, eq21_e1135_d_n11];
        let eq21_branch_derivatives: [f64; 7] = [eq21_e1135_d_b0, eq21_e1135_d_b1, eq21_e1135_d_b2, eq21_e1135_d_b3, eq21_e1135_d_b4, eq21_e1135_d_b5, eq21_e1135_d_b6];
        stamper.stamp_current_dense_local(
            Some(0),
            Some(7),
            multiplicity * (eq21_value),
            &eq21_node_derivatives,
            &eq21_branch_derivatives,
            multiplicity,
        );
        let (eq22_e1145,) = {
    if s.b[2704] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq22_value: f64 = eq22_e1145;
        stamper.stamp_current_const_local(
            Some(0),
            Some(7),
            multiplicity * (eq22_value),
        );
        let (eq23_e1150,) = {
    if (!s.b[2704]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq23_value: f64 = eq23_e1150;
        stamper.stamp_potential_const_local(
            2,
            eq23_value,
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
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        let nv7 = ctx.node_voltage(nodes[7]);
        let nv8 = ctx.node_voltage(nodes[8]);
        let nv9 = ctx.node_voltage(nodes[9]);
        let nv10 = ctx.node_voltage(nodes[10]);
        let nv11 = ctx.node_voltage(nodes[11]);
        let (eq24_e1160, eq24_e1160_d_n0, eq24_e1160_d_n1, eq24_e1160_d_n2, eq24_e1160_d_n3, eq24_e1160_d_n4, eq24_e1160_d_n5, eq24_e1160_d_n6, eq24_e1160_d_n7, eq24_e1160_d_n8, eq24_e1160_d_n9, eq24_e1160_d_n10, eq24_e1160_d_n11, eq24_e1160_d_b0, eq24_e1160_d_b1, eq24_e1160_d_b2, eq24_e1160_d_b3, eq24_e1160_d_b4, eq24_e1160_d_b5, eq24_e1160_d_b6,) = {
    if s.b[2705] {
        let eq24_e1154: f64 = (s.v[15] * p.p32);
        let eq24_e1154_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq24_e1154_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq24_e1154_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq24_e1154_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq24_e1154_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq24_e1154_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq24_e1154_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq24_e1154_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq24_e1154_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq24_e1154_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq24_e1154_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq24_e1154_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq24_e1154_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq24_e1154_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq24_e1154_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq24_e1154_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq24_e1154_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq24_e1154_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq24_e1154_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq24_e1156: f64 = (eq24_e1154 * s.v[808]);
        let eq24_e1156_d_n0: f64 = ((eq24_e1154_d_n0 * s.v[808]) + (eq24_e1154 * s.dn[808][0]));
        let eq24_e1156_d_n1: f64 = ((eq24_e1154_d_n1 * s.v[808]) + (eq24_e1154 * s.dn[808][1]));
        let eq24_e1156_d_n2: f64 = ((eq24_e1154_d_n2 * s.v[808]) + (eq24_e1154 * s.dn[808][2]));
        let eq24_e1156_d_n3: f64 = ((eq24_e1154_d_n3 * s.v[808]) + (eq24_e1154 * s.dn[808][3]));
        let eq24_e1156_d_n4: f64 = ((eq24_e1154_d_n4 * s.v[808]) + (eq24_e1154 * s.dn[808][4]));
        let eq24_e1156_d_n5: f64 = ((eq24_e1154_d_n5 * s.v[808]) + (eq24_e1154 * s.dn[808][5]));
        let eq24_e1156_d_n6: f64 = ((eq24_e1154_d_n6 * s.v[808]) + (eq24_e1154 * s.dn[808][6]));
        let eq24_e1156_d_n7: f64 = ((eq24_e1154_d_n7 * s.v[808]) + (eq24_e1154 * s.dn[808][7]));
        let eq24_e1156_d_n8: f64 = ((eq24_e1154_d_n8 * s.v[808]) + (eq24_e1154 * s.dn[808][8]));
        let eq24_e1156_d_n9: f64 = ((eq24_e1154_d_n9 * s.v[808]) + (eq24_e1154 * s.dn[808][9]));
        let eq24_e1156_d_n10: f64 = ((eq24_e1154_d_n10 * s.v[808]) + (eq24_e1154 * s.dn[808][10]));
        let eq24_e1156_d_n11: f64 = ((eq24_e1154_d_n11 * s.v[808]) + (eq24_e1154 * s.dn[808][11]));
        let eq24_e1156_d_b0: f64 = ((eq24_e1154_d_b0 * s.v[808]) + (eq24_e1154 * s.db[808][0]));
        let eq24_e1156_d_b1: f64 = ((eq24_e1154_d_b1 * s.v[808]) + (eq24_e1154 * s.db[808][1]));
        let eq24_e1156_d_b2: f64 = ((eq24_e1154_d_b2 * s.v[808]) + (eq24_e1154 * s.db[808][2]));
        let eq24_e1156_d_b3: f64 = ((eq24_e1154_d_b3 * s.v[808]) + (eq24_e1154 * s.db[808][3]));
        let eq24_e1156_d_b4: f64 = ((eq24_e1154_d_b4 * s.v[808]) + (eq24_e1154 * s.db[808][4]));
        let eq24_e1156_d_b5: f64 = ((eq24_e1154_d_b5 * s.v[808]) + (eq24_e1154 * s.db[808][5]));
        let eq24_e1156_d_b6: f64 = ((eq24_e1154_d_b6 * s.v[808]) + (eq24_e1154 * s.db[808][6]));
        let eq24_e1158: f64 = (eq24_e1156 * (nv8 - nv9));
        let eq24_e1158_d_n0: f64 = (eq24_e1156_d_n0 * (nv8 - nv9));
        let eq24_e1158_d_n1: f64 = (eq24_e1156_d_n1 * (nv8 - nv9));
        let eq24_e1158_d_n2: f64 = (eq24_e1156_d_n2 * (nv8 - nv9));
        let eq24_e1158_d_n3: f64 = (eq24_e1156_d_n3 * (nv8 - nv9));
        let eq24_e1158_d_n4: f64 = (eq24_e1156_d_n4 * (nv8 - nv9));
        let eq24_e1158_d_n5: f64 = (eq24_e1156_d_n5 * (nv8 - nv9));
        let eq24_e1158_d_n6: f64 = (eq24_e1156_d_n6 * (nv8 - nv9));
        let eq24_e1158_d_n7: f64 = (eq24_e1156_d_n7 * (nv8 - nv9));
        let eq24_e1158_d_n8: f64 = ((eq24_e1156_d_n8 * (nv8 - nv9)) + eq24_e1156);
        let eq24_e1158_d_n9: f64 = ((eq24_e1156_d_n9 * (nv8 - nv9)) + (-eq24_e1156));
        let eq24_e1158_d_n10: f64 = (eq24_e1156_d_n10 * (nv8 - nv9));
        let eq24_e1158_d_n11: f64 = (eq24_e1156_d_n11 * (nv8 - nv9));
        let eq24_e1158_d_b0: f64 = (eq24_e1156_d_b0 * (nv8 - nv9));
        let eq24_e1158_d_b1: f64 = (eq24_e1156_d_b1 * (nv8 - nv9));
        let eq24_e1158_d_b2: f64 = (eq24_e1156_d_b2 * (nv8 - nv9));
        let eq24_e1158_d_b3: f64 = (eq24_e1156_d_b3 * (nv8 - nv9));
        let eq24_e1158_d_b4: f64 = (eq24_e1156_d_b4 * (nv8 - nv9));
        let eq24_e1158_d_b5: f64 = (eq24_e1156_d_b5 * (nv8 - nv9));
        let eq24_e1158_d_b6: f64 = (eq24_e1156_d_b6 * (nv8 - nv9));
        (eq24_e1158, eq24_e1158_d_n0, eq24_e1158_d_n1, eq24_e1158_d_n2, eq24_e1158_d_n3, eq24_e1158_d_n4, eq24_e1158_d_n5, eq24_e1158_d_n6, eq24_e1158_d_n7, eq24_e1158_d_n8, eq24_e1158_d_n9, eq24_e1158_d_n10, eq24_e1158_d_n11, eq24_e1158_d_b0, eq24_e1158_d_b1, eq24_e1158_d_b2, eq24_e1158_d_b3, eq24_e1158_d_b4, eq24_e1158_d_b5, eq24_e1158_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq24_value: f64 = eq24_e1160;
        let eq24_node_derivatives: [f64; 12] = [eq24_e1160_d_n0, eq24_e1160_d_n1, eq24_e1160_d_n2, eq24_e1160_d_n3, eq24_e1160_d_n4, eq24_e1160_d_n5, eq24_e1160_d_n6, eq24_e1160_d_n7, eq24_e1160_d_n8, eq24_e1160_d_n9, eq24_e1160_d_n10, eq24_e1160_d_n11];
        let eq24_branch_derivatives: [f64; 7] = [eq24_e1160_d_b0, eq24_e1160_d_b1, eq24_e1160_d_b2, eq24_e1160_d_b3, eq24_e1160_d_b4, eq24_e1160_d_b5, eq24_e1160_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(9),
            multiplicity * (eq24_value),
            &eq24_node_derivatives,
            &eq24_branch_derivatives,
            multiplicity,
        );
        let (eq25_e1170,) = {
    if s.b[2705] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq25_value: f64 = eq25_e1170;
        stamper.stamp_current_const_local(
            Some(8),
            Some(9),
            multiplicity * (eq25_value),
        );
        let (eq26_e1175,) = {
    if (!s.b[2705]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq26_value: f64 = eq26_e1175;
        stamper.stamp_potential_const_local(
            3,
            eq26_value,
        );
        let (eq27_e1185, eq27_e1185_d_n0, eq27_e1185_d_n1, eq27_e1185_d_n2, eq27_e1185_d_n3, eq27_e1185_d_n4, eq27_e1185_d_n5, eq27_e1185_d_n6, eq27_e1185_d_n7, eq27_e1185_d_n8, eq27_e1185_d_n9, eq27_e1185_d_n10, eq27_e1185_d_n11, eq27_e1185_d_b0, eq27_e1185_d_b1, eq27_e1185_d_b2, eq27_e1185_d_b3, eq27_e1185_d_b4, eq27_e1185_d_b5, eq27_e1185_d_b6,) = {
    if s.b[2706] {
        let eq27_e1179: f64 = (s.v[15] * p.p32);
        let eq27_e1179_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq27_e1179_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq27_e1179_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq27_e1179_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq27_e1179_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq27_e1179_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq27_e1179_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq27_e1179_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq27_e1179_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq27_e1179_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq27_e1179_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq27_e1179_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq27_e1179_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq27_e1179_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq27_e1179_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq27_e1179_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq27_e1179_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq27_e1179_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq27_e1179_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq27_e1181: f64 = (eq27_e1179 * s.v[809]);
        let eq27_e1181_d_n0: f64 = ((eq27_e1179_d_n0 * s.v[809]) + (eq27_e1179 * s.dn[809][0]));
        let eq27_e1181_d_n1: f64 = ((eq27_e1179_d_n1 * s.v[809]) + (eq27_e1179 * s.dn[809][1]));
        let eq27_e1181_d_n2: f64 = ((eq27_e1179_d_n2 * s.v[809]) + (eq27_e1179 * s.dn[809][2]));
        let eq27_e1181_d_n3: f64 = ((eq27_e1179_d_n3 * s.v[809]) + (eq27_e1179 * s.dn[809][3]));
        let eq27_e1181_d_n4: f64 = ((eq27_e1179_d_n4 * s.v[809]) + (eq27_e1179 * s.dn[809][4]));
        let eq27_e1181_d_n5: f64 = ((eq27_e1179_d_n5 * s.v[809]) + (eq27_e1179 * s.dn[809][5]));
        let eq27_e1181_d_n6: f64 = ((eq27_e1179_d_n6 * s.v[809]) + (eq27_e1179 * s.dn[809][6]));
        let eq27_e1181_d_n7: f64 = ((eq27_e1179_d_n7 * s.v[809]) + (eq27_e1179 * s.dn[809][7]));
        let eq27_e1181_d_n8: f64 = ((eq27_e1179_d_n8 * s.v[809]) + (eq27_e1179 * s.dn[809][8]));
        let eq27_e1181_d_n9: f64 = ((eq27_e1179_d_n9 * s.v[809]) + (eq27_e1179 * s.dn[809][9]));
        let eq27_e1181_d_n10: f64 = ((eq27_e1179_d_n10 * s.v[809]) + (eq27_e1179 * s.dn[809][10]));
        let eq27_e1181_d_n11: f64 = ((eq27_e1179_d_n11 * s.v[809]) + (eq27_e1179 * s.dn[809][11]));
        let eq27_e1181_d_b0: f64 = ((eq27_e1179_d_b0 * s.v[809]) + (eq27_e1179 * s.db[809][0]));
        let eq27_e1181_d_b1: f64 = ((eq27_e1179_d_b1 * s.v[809]) + (eq27_e1179 * s.db[809][1]));
        let eq27_e1181_d_b2: f64 = ((eq27_e1179_d_b2 * s.v[809]) + (eq27_e1179 * s.db[809][2]));
        let eq27_e1181_d_b3: f64 = ((eq27_e1179_d_b3 * s.v[809]) + (eq27_e1179 * s.db[809][3]));
        let eq27_e1181_d_b4: f64 = ((eq27_e1179_d_b4 * s.v[809]) + (eq27_e1179 * s.db[809][4]));
        let eq27_e1181_d_b5: f64 = ((eq27_e1179_d_b5 * s.v[809]) + (eq27_e1179 * s.db[809][5]));
        let eq27_e1181_d_b6: f64 = ((eq27_e1179_d_b6 * s.v[809]) + (eq27_e1179 * s.db[809][6]));
        let eq27_e1183: f64 = (eq27_e1181 * (nv10 - nv9));
        let eq27_e1183_d_n0: f64 = (eq27_e1181_d_n0 * (nv10 - nv9));
        let eq27_e1183_d_n1: f64 = (eq27_e1181_d_n1 * (nv10 - nv9));
        let eq27_e1183_d_n2: f64 = (eq27_e1181_d_n2 * (nv10 - nv9));
        let eq27_e1183_d_n3: f64 = (eq27_e1181_d_n3 * (nv10 - nv9));
        let eq27_e1183_d_n4: f64 = (eq27_e1181_d_n4 * (nv10 - nv9));
        let eq27_e1183_d_n5: f64 = (eq27_e1181_d_n5 * (nv10 - nv9));
        let eq27_e1183_d_n6: f64 = (eq27_e1181_d_n6 * (nv10 - nv9));
        let eq27_e1183_d_n7: f64 = (eq27_e1181_d_n7 * (nv10 - nv9));
        let eq27_e1183_d_n8: f64 = (eq27_e1181_d_n8 * (nv10 - nv9));
        let eq27_e1183_d_n9: f64 = ((eq27_e1181_d_n9 * (nv10 - nv9)) + (-eq27_e1181));
        let eq27_e1183_d_n10: f64 = ((eq27_e1181_d_n10 * (nv10 - nv9)) + eq27_e1181);
        let eq27_e1183_d_n11: f64 = (eq27_e1181_d_n11 * (nv10 - nv9));
        let eq27_e1183_d_b0: f64 = (eq27_e1181_d_b0 * (nv10 - nv9));
        let eq27_e1183_d_b1: f64 = (eq27_e1181_d_b1 * (nv10 - nv9));
        let eq27_e1183_d_b2: f64 = (eq27_e1181_d_b2 * (nv10 - nv9));
        let eq27_e1183_d_b3: f64 = (eq27_e1181_d_b3 * (nv10 - nv9));
        let eq27_e1183_d_b4: f64 = (eq27_e1181_d_b4 * (nv10 - nv9));
        let eq27_e1183_d_b5: f64 = (eq27_e1181_d_b5 * (nv10 - nv9));
        let eq27_e1183_d_b6: f64 = (eq27_e1181_d_b6 * (nv10 - nv9));
        (eq27_e1183, eq27_e1183_d_n0, eq27_e1183_d_n1, eq27_e1183_d_n2, eq27_e1183_d_n3, eq27_e1183_d_n4, eq27_e1183_d_n5, eq27_e1183_d_n6, eq27_e1183_d_n7, eq27_e1183_d_n8, eq27_e1183_d_n9, eq27_e1183_d_n10, eq27_e1183_d_n11, eq27_e1183_d_b0, eq27_e1183_d_b1, eq27_e1183_d_b2, eq27_e1183_d_b3, eq27_e1183_d_b4, eq27_e1183_d_b5, eq27_e1183_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq27_value: f64 = eq27_e1185;
        let eq27_node_derivatives: [f64; 12] = [eq27_e1185_d_n0, eq27_e1185_d_n1, eq27_e1185_d_n2, eq27_e1185_d_n3, eq27_e1185_d_n4, eq27_e1185_d_n5, eq27_e1185_d_n6, eq27_e1185_d_n7, eq27_e1185_d_n8, eq27_e1185_d_n9, eq27_e1185_d_n10, eq27_e1185_d_n11];
        let eq27_branch_derivatives: [f64; 7] = [eq27_e1185_d_b0, eq27_e1185_d_b1, eq27_e1185_d_b2, eq27_e1185_d_b3, eq27_e1185_d_b4, eq27_e1185_d_b5, eq27_e1185_d_b6];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(9),
            multiplicity * (eq27_value),
            &eq27_node_derivatives,
            &eq27_branch_derivatives,
            multiplicity,
        );
        let (eq28_e1195,) = {
    if s.b[2706] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq28_value: f64 = eq28_e1195;
        stamper.stamp_current_const_local(
            Some(10),
            Some(9),
            multiplicity * (eq28_value),
        );
        let (eq29_e1200,) = {
    if (!s.b[2706]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq29_value: f64 = eq29_e1200;
        stamper.stamp_potential_const_local(
            4,
            eq29_value,
        );
        let (eq30_e1210, eq30_e1210_d_n0, eq30_e1210_d_n1, eq30_e1210_d_n2, eq30_e1210_d_n3, eq30_e1210_d_n4, eq30_e1210_d_n5, eq30_e1210_d_n6, eq30_e1210_d_n7, eq30_e1210_d_n8, eq30_e1210_d_n9, eq30_e1210_d_n10, eq30_e1210_d_n11, eq30_e1210_d_b0, eq30_e1210_d_b1, eq30_e1210_d_b2, eq30_e1210_d_b3, eq30_e1210_d_b4, eq30_e1210_d_b5, eq30_e1210_d_b6,) = {
    if s.b[2707] {
        let eq30_e1204: f64 = (s.v[15] * p.p32);
        let eq30_e1204_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq30_e1204_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq30_e1204_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq30_e1204_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq30_e1204_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq30_e1204_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq30_e1204_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq30_e1204_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq30_e1204_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq30_e1204_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq30_e1204_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq30_e1204_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq30_e1204_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq30_e1204_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq30_e1204_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq30_e1204_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq30_e1204_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq30_e1204_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq30_e1204_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq30_e1206: f64 = (eq30_e1204 * s.v[810]);
        let eq30_e1206_d_n0: f64 = ((eq30_e1204_d_n0 * s.v[810]) + (eq30_e1204 * s.dn[810][0]));
        let eq30_e1206_d_n1: f64 = ((eq30_e1204_d_n1 * s.v[810]) + (eq30_e1204 * s.dn[810][1]));
        let eq30_e1206_d_n2: f64 = ((eq30_e1204_d_n2 * s.v[810]) + (eq30_e1204 * s.dn[810][2]));
        let eq30_e1206_d_n3: f64 = ((eq30_e1204_d_n3 * s.v[810]) + (eq30_e1204 * s.dn[810][3]));
        let eq30_e1206_d_n4: f64 = ((eq30_e1204_d_n4 * s.v[810]) + (eq30_e1204 * s.dn[810][4]));
        let eq30_e1206_d_n5: f64 = ((eq30_e1204_d_n5 * s.v[810]) + (eq30_e1204 * s.dn[810][5]));
        let eq30_e1206_d_n6: f64 = ((eq30_e1204_d_n6 * s.v[810]) + (eq30_e1204 * s.dn[810][6]));
        let eq30_e1206_d_n7: f64 = ((eq30_e1204_d_n7 * s.v[810]) + (eq30_e1204 * s.dn[810][7]));
        let eq30_e1206_d_n8: f64 = ((eq30_e1204_d_n8 * s.v[810]) + (eq30_e1204 * s.dn[810][8]));
        let eq30_e1206_d_n9: f64 = ((eq30_e1204_d_n9 * s.v[810]) + (eq30_e1204 * s.dn[810][9]));
        let eq30_e1206_d_n10: f64 = ((eq30_e1204_d_n10 * s.v[810]) + (eq30_e1204 * s.dn[810][10]));
        let eq30_e1206_d_n11: f64 = ((eq30_e1204_d_n11 * s.v[810]) + (eq30_e1204 * s.dn[810][11]));
        let eq30_e1206_d_b0: f64 = ((eq30_e1204_d_b0 * s.v[810]) + (eq30_e1204 * s.db[810][0]));
        let eq30_e1206_d_b1: f64 = ((eq30_e1204_d_b1 * s.v[810]) + (eq30_e1204 * s.db[810][1]));
        let eq30_e1206_d_b2: f64 = ((eq30_e1204_d_b2 * s.v[810]) + (eq30_e1204 * s.db[810][2]));
        let eq30_e1206_d_b3: f64 = ((eq30_e1204_d_b3 * s.v[810]) + (eq30_e1204 * s.db[810][3]));
        let eq30_e1206_d_b4: f64 = ((eq30_e1204_d_b4 * s.v[810]) + (eq30_e1204 * s.db[810][4]));
        let eq30_e1206_d_b5: f64 = ((eq30_e1204_d_b5 * s.v[810]) + (eq30_e1204 * s.db[810][5]));
        let eq30_e1206_d_b6: f64 = ((eq30_e1204_d_b6 * s.v[810]) + (eq30_e1204 * s.db[810][6]));
        let eq30_e1208: f64 = (eq30_e1206 * (nv11 - nv9));
        let eq30_e1208_d_n0: f64 = (eq30_e1206_d_n0 * (nv11 - nv9));
        let eq30_e1208_d_n1: f64 = (eq30_e1206_d_n1 * (nv11 - nv9));
        let eq30_e1208_d_n2: f64 = (eq30_e1206_d_n2 * (nv11 - nv9));
        let eq30_e1208_d_n3: f64 = (eq30_e1206_d_n3 * (nv11 - nv9));
        let eq30_e1208_d_n4: f64 = (eq30_e1206_d_n4 * (nv11 - nv9));
        let eq30_e1208_d_n5: f64 = (eq30_e1206_d_n5 * (nv11 - nv9));
        let eq30_e1208_d_n6: f64 = (eq30_e1206_d_n6 * (nv11 - nv9));
        let eq30_e1208_d_n7: f64 = (eq30_e1206_d_n7 * (nv11 - nv9));
        let eq30_e1208_d_n8: f64 = (eq30_e1206_d_n8 * (nv11 - nv9));
        let eq30_e1208_d_n9: f64 = ((eq30_e1206_d_n9 * (nv11 - nv9)) + (-eq30_e1206));
        let eq30_e1208_d_n10: f64 = (eq30_e1206_d_n10 * (nv11 - nv9));
        let eq30_e1208_d_n11: f64 = ((eq30_e1206_d_n11 * (nv11 - nv9)) + eq30_e1206);
        let eq30_e1208_d_b0: f64 = (eq30_e1206_d_b0 * (nv11 - nv9));
        let eq30_e1208_d_b1: f64 = (eq30_e1206_d_b1 * (nv11 - nv9));
        let eq30_e1208_d_b2: f64 = (eq30_e1206_d_b2 * (nv11 - nv9));
        let eq30_e1208_d_b3: f64 = (eq30_e1206_d_b3 * (nv11 - nv9));
        let eq30_e1208_d_b4: f64 = (eq30_e1206_d_b4 * (nv11 - nv9));
        let eq30_e1208_d_b5: f64 = (eq30_e1206_d_b5 * (nv11 - nv9));
        let eq30_e1208_d_b6: f64 = (eq30_e1206_d_b6 * (nv11 - nv9));
        (eq30_e1208, eq30_e1208_d_n0, eq30_e1208_d_n1, eq30_e1208_d_n2, eq30_e1208_d_n3, eq30_e1208_d_n4, eq30_e1208_d_n5, eq30_e1208_d_n6, eq30_e1208_d_n7, eq30_e1208_d_n8, eq30_e1208_d_n9, eq30_e1208_d_n10, eq30_e1208_d_n11, eq30_e1208_d_b0, eq30_e1208_d_b1, eq30_e1208_d_b2, eq30_e1208_d_b3, eq30_e1208_d_b4, eq30_e1208_d_b5, eq30_e1208_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq30_value: f64 = eq30_e1210;
        let eq30_node_derivatives: [f64; 12] = [eq30_e1210_d_n0, eq30_e1210_d_n1, eq30_e1210_d_n2, eq30_e1210_d_n3, eq30_e1210_d_n4, eq30_e1210_d_n5, eq30_e1210_d_n6, eq30_e1210_d_n7, eq30_e1210_d_n8, eq30_e1210_d_n9, eq30_e1210_d_n10, eq30_e1210_d_n11];
        let eq30_branch_derivatives: [f64; 7] = [eq30_e1210_d_b0, eq30_e1210_d_b1, eq30_e1210_d_b2, eq30_e1210_d_b3, eq30_e1210_d_b4, eq30_e1210_d_b5, eq30_e1210_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(9),
            multiplicity * (eq30_value),
            &eq30_node_derivatives,
            &eq30_branch_derivatives,
            multiplicity,
        );
        let (eq31_e1220,) = {
    if s.b[2707] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq31_value: f64 = eq31_e1220;
        stamper.stamp_current_const_local(
            Some(11),
            Some(9),
            multiplicity * (eq31_value),
        );
        let (eq32_e1225,) = {
    if (!s.b[2707]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq32_value: f64 = eq32_e1225;
        stamper.stamp_potential_const_local(
            5,
            eq32_value,
        );
        let (eq33_e1235, eq33_e1235_d_n0, eq33_e1235_d_n1, eq33_e1235_d_n2, eq33_e1235_d_n3, eq33_e1235_d_n4, eq33_e1235_d_n5, eq33_e1235_d_n6, eq33_e1235_d_n7, eq33_e1235_d_n8, eq33_e1235_d_n9, eq33_e1235_d_n10, eq33_e1235_d_n11, eq33_e1235_d_b0, eq33_e1235_d_b1, eq33_e1235_d_b2, eq33_e1235_d_b3, eq33_e1235_d_b4, eq33_e1235_d_b5, eq33_e1235_d_b6,) = {
    if s.b[2708] {
        let eq33_e1229: f64 = (s.v[15] * p.p32);
        let eq33_e1229_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq33_e1229_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq33_e1229_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq33_e1229_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq33_e1229_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq33_e1229_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq33_e1229_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq33_e1229_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq33_e1229_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq33_e1229_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq33_e1229_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq33_e1229_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq33_e1229_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq33_e1229_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq33_e1229_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq33_e1229_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq33_e1229_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq33_e1229_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq33_e1229_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq33_e1231: f64 = (eq33_e1229 * s.v[811]);
        let eq33_e1231_d_n0: f64 = ((eq33_e1229_d_n0 * s.v[811]) + (eq33_e1229 * s.dn[811][0]));
        let eq33_e1231_d_n1: f64 = ((eq33_e1229_d_n1 * s.v[811]) + (eq33_e1229 * s.dn[811][1]));
        let eq33_e1231_d_n2: f64 = ((eq33_e1229_d_n2 * s.v[811]) + (eq33_e1229 * s.dn[811][2]));
        let eq33_e1231_d_n3: f64 = ((eq33_e1229_d_n3 * s.v[811]) + (eq33_e1229 * s.dn[811][3]));
        let eq33_e1231_d_n4: f64 = ((eq33_e1229_d_n4 * s.v[811]) + (eq33_e1229 * s.dn[811][4]));
        let eq33_e1231_d_n5: f64 = ((eq33_e1229_d_n5 * s.v[811]) + (eq33_e1229 * s.dn[811][5]));
        let eq33_e1231_d_n6: f64 = ((eq33_e1229_d_n6 * s.v[811]) + (eq33_e1229 * s.dn[811][6]));
        let eq33_e1231_d_n7: f64 = ((eq33_e1229_d_n7 * s.v[811]) + (eq33_e1229 * s.dn[811][7]));
        let eq33_e1231_d_n8: f64 = ((eq33_e1229_d_n8 * s.v[811]) + (eq33_e1229 * s.dn[811][8]));
        let eq33_e1231_d_n9: f64 = ((eq33_e1229_d_n9 * s.v[811]) + (eq33_e1229 * s.dn[811][9]));
        let eq33_e1231_d_n10: f64 = ((eq33_e1229_d_n10 * s.v[811]) + (eq33_e1229 * s.dn[811][10]));
        let eq33_e1231_d_n11: f64 = ((eq33_e1229_d_n11 * s.v[811]) + (eq33_e1229 * s.dn[811][11]));
        let eq33_e1231_d_b0: f64 = ((eq33_e1229_d_b0 * s.v[811]) + (eq33_e1229 * s.db[811][0]));
        let eq33_e1231_d_b1: f64 = ((eq33_e1229_d_b1 * s.v[811]) + (eq33_e1229 * s.db[811][1]));
        let eq33_e1231_d_b2: f64 = ((eq33_e1229_d_b2 * s.v[811]) + (eq33_e1229 * s.db[811][2]));
        let eq33_e1231_d_b3: f64 = ((eq33_e1229_d_b3 * s.v[811]) + (eq33_e1229 * s.db[811][3]));
        let eq33_e1231_d_b4: f64 = ((eq33_e1229_d_b4 * s.v[811]) + (eq33_e1229 * s.db[811][4]));
        let eq33_e1231_d_b5: f64 = ((eq33_e1229_d_b5 * s.v[811]) + (eq33_e1229 * s.db[811][5]));
        let eq33_e1231_d_b6: f64 = ((eq33_e1229_d_b6 * s.v[811]) + (eq33_e1229 * s.db[811][6]));
        let eq33_e1233: f64 = (eq33_e1231 * (nv3 - nv9));
        let eq33_e1233_d_n0: f64 = (eq33_e1231_d_n0 * (nv3 - nv9));
        let eq33_e1233_d_n1: f64 = (eq33_e1231_d_n1 * (nv3 - nv9));
        let eq33_e1233_d_n2: f64 = (eq33_e1231_d_n2 * (nv3 - nv9));
        let eq33_e1233_d_n3: f64 = ((eq33_e1231_d_n3 * (nv3 - nv9)) + eq33_e1231);
        let eq33_e1233_d_n4: f64 = (eq33_e1231_d_n4 * (nv3 - nv9));
        let eq33_e1233_d_n5: f64 = (eq33_e1231_d_n5 * (nv3 - nv9));
        let eq33_e1233_d_n6: f64 = (eq33_e1231_d_n6 * (nv3 - nv9));
        let eq33_e1233_d_n7: f64 = (eq33_e1231_d_n7 * (nv3 - nv9));
        let eq33_e1233_d_n8: f64 = (eq33_e1231_d_n8 * (nv3 - nv9));
        let eq33_e1233_d_n9: f64 = ((eq33_e1231_d_n9 * (nv3 - nv9)) + (-eq33_e1231));
        let eq33_e1233_d_n10: f64 = (eq33_e1231_d_n10 * (nv3 - nv9));
        let eq33_e1233_d_n11: f64 = (eq33_e1231_d_n11 * (nv3 - nv9));
        let eq33_e1233_d_b0: f64 = (eq33_e1231_d_b0 * (nv3 - nv9));
        let eq33_e1233_d_b1: f64 = (eq33_e1231_d_b1 * (nv3 - nv9));
        let eq33_e1233_d_b2: f64 = (eq33_e1231_d_b2 * (nv3 - nv9));
        let eq33_e1233_d_b3: f64 = (eq33_e1231_d_b3 * (nv3 - nv9));
        let eq33_e1233_d_b4: f64 = (eq33_e1231_d_b4 * (nv3 - nv9));
        let eq33_e1233_d_b5: f64 = (eq33_e1231_d_b5 * (nv3 - nv9));
        let eq33_e1233_d_b6: f64 = (eq33_e1231_d_b6 * (nv3 - nv9));
        (eq33_e1233, eq33_e1233_d_n0, eq33_e1233_d_n1, eq33_e1233_d_n2, eq33_e1233_d_n3, eq33_e1233_d_n4, eq33_e1233_d_n5, eq33_e1233_d_n6, eq33_e1233_d_n7, eq33_e1233_d_n8, eq33_e1233_d_n9, eq33_e1233_d_n10, eq33_e1233_d_n11, eq33_e1233_d_b0, eq33_e1233_d_b1, eq33_e1233_d_b2, eq33_e1233_d_b3, eq33_e1233_d_b4, eq33_e1233_d_b5, eq33_e1233_d_b6,)
    } else {
        (0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0,)
    }
};
        let eq33_value: f64 = eq33_e1235;
        let eq33_node_derivatives: [f64; 12] = [eq33_e1235_d_n0, eq33_e1235_d_n1, eq33_e1235_d_n2, eq33_e1235_d_n3, eq33_e1235_d_n4, eq33_e1235_d_n5, eq33_e1235_d_n6, eq33_e1235_d_n7, eq33_e1235_d_n8, eq33_e1235_d_n9, eq33_e1235_d_n10, eq33_e1235_d_n11];
        let eq33_branch_derivatives: [f64; 7] = [eq33_e1235_d_b0, eq33_e1235_d_b1, eq33_e1235_d_b2, eq33_e1235_d_b3, eq33_e1235_d_b4, eq33_e1235_d_b5, eq33_e1235_d_b6];
        stamper.stamp_current_dense_local(
            Some(3),
            Some(9),
            multiplicity * (eq33_value),
            &eq33_node_derivatives,
            &eq33_branch_derivatives,
            multiplicity,
        );
        let (eq34_e1245,) = {
    if s.b[2708] {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq34_value: f64 = eq34_e1245;
        stamper.stamp_current_const_local(
            Some(3),
            Some(9),
            multiplicity * (eq34_value),
        );
        let (eq35_e1250,) = {
    if (!s.b[2708]) {
        (0.0,)
    } else {
        (0.0,)
    }
};
        let eq35_value: f64 = eq35_e1250;
        stamper.stamp_potential_const_local(
            6,
            eq35_value,
        );
        let eq36_e1253: f64 = (p.p32 * s.v[872]);
        let eq36_e1255: f64 = (eq36_e1253 * (nv7 - nv8));
        let eq36_e1255_d_n8: f64 = (-eq36_e1253);
        let eq36_value: f64 = eq36_e1255;
        stamper.stamp_current_node2_local(
            Some(7),
            Some(8),
            multiplicity * (eq36_value),
            7,
            multiplicity * (eq36_e1253),
            8,
            multiplicity * (eq36_e1255_d_n8),
        );
        let eq37_e1258: f64 = (p.p32 * s.v[872]);
        let eq37_e1260: f64 = (eq37_e1258 * (nv6 - nv8));
        let eq37_e1260_d_n8: f64 = (-eq37_e1258);
        let eq37_value: f64 = eq37_e1260;
        stamper.stamp_current_node2_local(
            Some(6),
            Some(8),
            multiplicity * (eq37_value),
            6,
            multiplicity * (eq37_e1258),
            8,
            multiplicity * (eq37_e1260_d_n8),
        );
    }

    pub(super) fn stamp_transient_equations_block_4(
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
        let eq38_e1263: f64 = (s.v[0] * s.v[15]);
        let eq38_e1263_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq38_e1263_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq38_e1263_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq38_e1263_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq38_e1263_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq38_e1263_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq38_e1263_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq38_e1263_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq38_e1263_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq38_e1263_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq38_e1263_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq38_e1263_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq38_e1263_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq38_e1263_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq38_e1263_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq38_e1263_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq38_e1263_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq38_e1263_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq38_e1263_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq38_e1265: f64 = (eq38_e1263 * p.p33);
        let eq38_e1265_d_n0: f64 = (eq38_e1263_d_n0 * p.p33);
        let eq38_e1265_d_n1: f64 = (eq38_e1263_d_n1 * p.p33);
        let eq38_e1265_d_n2: f64 = (eq38_e1263_d_n2 * p.p33);
        let eq38_e1265_d_n3: f64 = (eq38_e1263_d_n3 * p.p33);
        let eq38_e1265_d_n4: f64 = (eq38_e1263_d_n4 * p.p33);
        let eq38_e1265_d_n5: f64 = (eq38_e1263_d_n5 * p.p33);
        let eq38_e1265_d_n6: f64 = (eq38_e1263_d_n6 * p.p33);
        let eq38_e1265_d_n7: f64 = (eq38_e1263_d_n7 * p.p33);
        let eq38_e1265_d_n8: f64 = (eq38_e1263_d_n8 * p.p33);
        let eq38_e1265_d_n9: f64 = (eq38_e1263_d_n9 * p.p33);
        let eq38_e1265_d_n10: f64 = (eq38_e1263_d_n10 * p.p33);
        let eq38_e1265_d_n11: f64 = (eq38_e1263_d_n11 * p.p33);
        let eq38_e1265_d_b0: f64 = (eq38_e1263_d_b0 * p.p33);
        let eq38_e1265_d_b1: f64 = (eq38_e1263_d_b1 * p.p33);
        let eq38_e1265_d_b2: f64 = (eq38_e1263_d_b2 * p.p33);
        let eq38_e1265_d_b3: f64 = (eq38_e1263_d_b3 * p.p33);
        let eq38_e1265_d_b4: f64 = (eq38_e1263_d_b4 * p.p33);
        let eq38_e1265_d_b5: f64 = (eq38_e1263_d_b5 * p.p33);
        let eq38_e1265_d_b6: f64 = (eq38_e1263_d_b6 * p.p33);
        let eq38_e1267: f64 = (eq38_e1265 * s.v[845]);
        let eq38_e1267_d_n0: f64 = ((eq38_e1265_d_n0 * s.v[845]) + (eq38_e1265 * s.dn[845][0]));
        let eq38_e1267_d_n1: f64 = ((eq38_e1265_d_n1 * s.v[845]) + (eq38_e1265 * s.dn[845][1]));
        let eq38_e1267_d_n2: f64 = ((eq38_e1265_d_n2 * s.v[845]) + (eq38_e1265 * s.dn[845][2]));
        let eq38_e1267_d_n3: f64 = ((eq38_e1265_d_n3 * s.v[845]) + (eq38_e1265 * s.dn[845][3]));
        let eq38_e1267_d_n4: f64 = ((eq38_e1265_d_n4 * s.v[845]) + (eq38_e1265 * s.dn[845][4]));
        let eq38_e1267_d_n5: f64 = ((eq38_e1265_d_n5 * s.v[845]) + (eq38_e1265 * s.dn[845][5]));
        let eq38_e1267_d_n6: f64 = ((eq38_e1265_d_n6 * s.v[845]) + (eq38_e1265 * s.dn[845][6]));
        let eq38_e1267_d_n7: f64 = ((eq38_e1265_d_n7 * s.v[845]) + (eq38_e1265 * s.dn[845][7]));
        let eq38_e1267_d_n8: f64 = ((eq38_e1265_d_n8 * s.v[845]) + (eq38_e1265 * s.dn[845][8]));
        let eq38_e1267_d_n9: f64 = ((eq38_e1265_d_n9 * s.v[845]) + (eq38_e1265 * s.dn[845][9]));
        let eq38_e1267_d_n10: f64 = ((eq38_e1265_d_n10 * s.v[845]) + (eq38_e1265 * s.dn[845][10]));
        let eq38_e1267_d_n11: f64 = ((eq38_e1265_d_n11 * s.v[845]) + (eq38_e1265 * s.dn[845][11]));
        let eq38_e1267_d_b0: f64 = ((eq38_e1265_d_b0 * s.v[845]) + (eq38_e1265 * s.db[845][0]));
        let eq38_e1267_d_b1: f64 = ((eq38_e1265_d_b1 * s.v[845]) + (eq38_e1265 * s.db[845][1]));
        let eq38_e1267_d_b2: f64 = ((eq38_e1265_d_b2 * s.v[845]) + (eq38_e1265 * s.db[845][2]));
        let eq38_e1267_d_b3: f64 = ((eq38_e1265_d_b3 * s.v[845]) + (eq38_e1265 * s.db[845][3]));
        let eq38_e1267_d_b4: f64 = ((eq38_e1265_d_b4 * s.v[845]) + (eq38_e1265 * s.db[845][4]));
        let eq38_e1267_d_b5: f64 = ((eq38_e1265_d_b5 * s.v[845]) + (eq38_e1265 * s.db[845][5]));
        let eq38_e1267_d_b6: f64 = ((eq38_e1265_d_b6 * s.v[845]) + (eq38_e1265 * s.db[845][6]));
        let eq38_e1268: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 0, eq38_e1267);
        let eq38_e1268_d_n0: f64 = (eq38_e1267_d_n0 * ddt_scale);
        let eq38_e1268_d_n1: f64 = (eq38_e1267_d_n1 * ddt_scale);
        let eq38_e1268_d_n2: f64 = (eq38_e1267_d_n2 * ddt_scale);
        let eq38_e1268_d_n3: f64 = (eq38_e1267_d_n3 * ddt_scale);
        let eq38_e1268_d_n4: f64 = (eq38_e1267_d_n4 * ddt_scale);
        let eq38_e1268_d_n5: f64 = (eq38_e1267_d_n5 * ddt_scale);
        let eq38_e1268_d_n6: f64 = (eq38_e1267_d_n6 * ddt_scale);
        let eq38_e1268_d_n7: f64 = (eq38_e1267_d_n7 * ddt_scale);
        let eq38_e1268_d_n8: f64 = (eq38_e1267_d_n8 * ddt_scale);
        let eq38_e1268_d_n9: f64 = (eq38_e1267_d_n9 * ddt_scale);
        let eq38_e1268_d_n10: f64 = (eq38_e1267_d_n10 * ddt_scale);
        let eq38_e1268_d_n11: f64 = (eq38_e1267_d_n11 * ddt_scale);
        let eq38_e1268_d_b0: f64 = (eq38_e1267_d_b0 * ddt_scale);
        let eq38_e1268_d_b1: f64 = (eq38_e1267_d_b1 * ddt_scale);
        let eq38_e1268_d_b2: f64 = (eq38_e1267_d_b2 * ddt_scale);
        let eq38_e1268_d_b3: f64 = (eq38_e1267_d_b3 * ddt_scale);
        let eq38_e1268_d_b4: f64 = (eq38_e1267_d_b4 * ddt_scale);
        let eq38_e1268_d_b5: f64 = (eq38_e1267_d_b5 * ddt_scale);
        let eq38_e1268_d_b6: f64 = (eq38_e1267_d_b6 * ddt_scale);
        let eq38_value: f64 = eq38_e1268;
        let eq38_node_derivatives: [f64; 12] = [eq38_e1268_d_n0, eq38_e1268_d_n1, eq38_e1268_d_n2, eq38_e1268_d_n3, eq38_e1268_d_n4, eq38_e1268_d_n5, eq38_e1268_d_n6, eq38_e1268_d_n7, eq38_e1268_d_n8, eq38_e1268_d_n9, eq38_e1268_d_n10, eq38_e1268_d_n11];
        let eq38_branch_derivatives: [f64; 7] = [eq38_e1268_d_b0, eq38_e1268_d_b1, eq38_e1268_d_b2, eq38_e1268_d_b3, eq38_e1268_d_b4, eq38_e1268_d_b5, eq38_e1268_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq38_value),
            &eq38_node_derivatives,
            &eq38_branch_derivatives,
            multiplicity,
        );
        let eq39_e1271: f64 = (s.v[0] * s.v[15]);
        let eq39_e1271_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq39_e1271_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq39_e1271_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq39_e1271_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq39_e1271_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq39_e1271_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq39_e1271_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq39_e1271_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq39_e1271_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq39_e1271_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq39_e1271_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq39_e1271_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq39_e1271_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq39_e1271_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq39_e1271_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq39_e1271_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq39_e1271_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq39_e1271_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq39_e1271_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq39_e1273: f64 = (eq39_e1271 * p.p33);
        let eq39_e1273_d_n0: f64 = (eq39_e1271_d_n0 * p.p33);
        let eq39_e1273_d_n1: f64 = (eq39_e1271_d_n1 * p.p33);
        let eq39_e1273_d_n2: f64 = (eq39_e1271_d_n2 * p.p33);
        let eq39_e1273_d_n3: f64 = (eq39_e1271_d_n3 * p.p33);
        let eq39_e1273_d_n4: f64 = (eq39_e1271_d_n4 * p.p33);
        let eq39_e1273_d_n5: f64 = (eq39_e1271_d_n5 * p.p33);
        let eq39_e1273_d_n6: f64 = (eq39_e1271_d_n6 * p.p33);
        let eq39_e1273_d_n7: f64 = (eq39_e1271_d_n7 * p.p33);
        let eq39_e1273_d_n8: f64 = (eq39_e1271_d_n8 * p.p33);
        let eq39_e1273_d_n9: f64 = (eq39_e1271_d_n9 * p.p33);
        let eq39_e1273_d_n10: f64 = (eq39_e1271_d_n10 * p.p33);
        let eq39_e1273_d_n11: f64 = (eq39_e1271_d_n11 * p.p33);
        let eq39_e1273_d_b0: f64 = (eq39_e1271_d_b0 * p.p33);
        let eq39_e1273_d_b1: f64 = (eq39_e1271_d_b1 * p.p33);
        let eq39_e1273_d_b2: f64 = (eq39_e1271_d_b2 * p.p33);
        let eq39_e1273_d_b3: f64 = (eq39_e1271_d_b3 * p.p33);
        let eq39_e1273_d_b4: f64 = (eq39_e1271_d_b4 * p.p33);
        let eq39_e1273_d_b5: f64 = (eq39_e1271_d_b5 * p.p33);
        let eq39_e1273_d_b6: f64 = (eq39_e1271_d_b6 * p.p33);
        let eq39_e1275: f64 = (eq39_e1273 * s.v[846]);
        let eq39_e1275_d_n0: f64 = ((eq39_e1273_d_n0 * s.v[846]) + (eq39_e1273 * s.dn[846][0]));
        let eq39_e1275_d_n1: f64 = ((eq39_e1273_d_n1 * s.v[846]) + (eq39_e1273 * s.dn[846][1]));
        let eq39_e1275_d_n2: f64 = ((eq39_e1273_d_n2 * s.v[846]) + (eq39_e1273 * s.dn[846][2]));
        let eq39_e1275_d_n3: f64 = ((eq39_e1273_d_n3 * s.v[846]) + (eq39_e1273 * s.dn[846][3]));
        let eq39_e1275_d_n4: f64 = ((eq39_e1273_d_n4 * s.v[846]) + (eq39_e1273 * s.dn[846][4]));
        let eq39_e1275_d_n5: f64 = ((eq39_e1273_d_n5 * s.v[846]) + (eq39_e1273 * s.dn[846][5]));
        let eq39_e1275_d_n6: f64 = ((eq39_e1273_d_n6 * s.v[846]) + (eq39_e1273 * s.dn[846][6]));
        let eq39_e1275_d_n7: f64 = ((eq39_e1273_d_n7 * s.v[846]) + (eq39_e1273 * s.dn[846][7]));
        let eq39_e1275_d_n8: f64 = ((eq39_e1273_d_n8 * s.v[846]) + (eq39_e1273 * s.dn[846][8]));
        let eq39_e1275_d_n9: f64 = ((eq39_e1273_d_n9 * s.v[846]) + (eq39_e1273 * s.dn[846][9]));
        let eq39_e1275_d_n10: f64 = ((eq39_e1273_d_n10 * s.v[846]) + (eq39_e1273 * s.dn[846][10]));
        let eq39_e1275_d_n11: f64 = ((eq39_e1273_d_n11 * s.v[846]) + (eq39_e1273 * s.dn[846][11]));
        let eq39_e1275_d_b0: f64 = ((eq39_e1273_d_b0 * s.v[846]) + (eq39_e1273 * s.db[846][0]));
        let eq39_e1275_d_b1: f64 = ((eq39_e1273_d_b1 * s.v[846]) + (eq39_e1273 * s.db[846][1]));
        let eq39_e1275_d_b2: f64 = ((eq39_e1273_d_b2 * s.v[846]) + (eq39_e1273 * s.db[846][2]));
        let eq39_e1275_d_b3: f64 = ((eq39_e1273_d_b3 * s.v[846]) + (eq39_e1273 * s.db[846][3]));
        let eq39_e1275_d_b4: f64 = ((eq39_e1273_d_b4 * s.v[846]) + (eq39_e1273 * s.db[846][4]));
        let eq39_e1275_d_b5: f64 = ((eq39_e1273_d_b5 * s.v[846]) + (eq39_e1273 * s.db[846][5]));
        let eq39_e1275_d_b6: f64 = ((eq39_e1273_d_b6 * s.v[846]) + (eq39_e1273 * s.db[846][6]));
        let eq39_e1276: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 1, eq39_e1275);
        let eq39_e1276_d_n0: f64 = (eq39_e1275_d_n0 * ddt_scale);
        let eq39_e1276_d_n1: f64 = (eq39_e1275_d_n1 * ddt_scale);
        let eq39_e1276_d_n2: f64 = (eq39_e1275_d_n2 * ddt_scale);
        let eq39_e1276_d_n3: f64 = (eq39_e1275_d_n3 * ddt_scale);
        let eq39_e1276_d_n4: f64 = (eq39_e1275_d_n4 * ddt_scale);
        let eq39_e1276_d_n5: f64 = (eq39_e1275_d_n5 * ddt_scale);
        let eq39_e1276_d_n6: f64 = (eq39_e1275_d_n6 * ddt_scale);
        let eq39_e1276_d_n7: f64 = (eq39_e1275_d_n7 * ddt_scale);
        let eq39_e1276_d_n8: f64 = (eq39_e1275_d_n8 * ddt_scale);
        let eq39_e1276_d_n9: f64 = (eq39_e1275_d_n9 * ddt_scale);
        let eq39_e1276_d_n10: f64 = (eq39_e1275_d_n10 * ddt_scale);
        let eq39_e1276_d_n11: f64 = (eq39_e1275_d_n11 * ddt_scale);
        let eq39_e1276_d_b0: f64 = (eq39_e1275_d_b0 * ddt_scale);
        let eq39_e1276_d_b1: f64 = (eq39_e1275_d_b1 * ddt_scale);
        let eq39_e1276_d_b2: f64 = (eq39_e1275_d_b2 * ddt_scale);
        let eq39_e1276_d_b3: f64 = (eq39_e1275_d_b3 * ddt_scale);
        let eq39_e1276_d_b4: f64 = (eq39_e1275_d_b4 * ddt_scale);
        let eq39_e1276_d_b5: f64 = (eq39_e1275_d_b5 * ddt_scale);
        let eq39_e1276_d_b6: f64 = (eq39_e1275_d_b6 * ddt_scale);
        let eq39_value: f64 = eq39_e1276;
        let eq39_node_derivatives: [f64; 12] = [eq39_e1276_d_n0, eq39_e1276_d_n1, eq39_e1276_d_n2, eq39_e1276_d_n3, eq39_e1276_d_n4, eq39_e1276_d_n5, eq39_e1276_d_n6, eq39_e1276_d_n7, eq39_e1276_d_n8, eq39_e1276_d_n9, eq39_e1276_d_n10, eq39_e1276_d_n11];
        let eq39_branch_derivatives: [f64; 7] = [eq39_e1276_d_b0, eq39_e1276_d_b1, eq39_e1276_d_b2, eq39_e1276_d_b3, eq39_e1276_d_b4, eq39_e1276_d_b5, eq39_e1276_d_b6];
        stamper.stamp_current_dense_local(
            Some(8),
            Some(6),
            multiplicity * (eq39_value),
            &eq39_node_derivatives,
            &eq39_branch_derivatives,
            multiplicity,
        );
        let eq40_e1279: f64 = (s.v[0] * s.v[15]);
        let eq40_e1279_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq40_e1279_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq40_e1279_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq40_e1279_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq40_e1279_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq40_e1279_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq40_e1279_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq40_e1279_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq40_e1279_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq40_e1279_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq40_e1279_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq40_e1279_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq40_e1279_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq40_e1279_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq40_e1279_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq40_e1279_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq40_e1279_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq40_e1279_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq40_e1279_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq40_e1281: f64 = (eq40_e1279 * p.p33);
        let eq40_e1281_d_n0: f64 = (eq40_e1279_d_n0 * p.p33);
        let eq40_e1281_d_n1: f64 = (eq40_e1279_d_n1 * p.p33);
        let eq40_e1281_d_n2: f64 = (eq40_e1279_d_n2 * p.p33);
        let eq40_e1281_d_n3: f64 = (eq40_e1279_d_n3 * p.p33);
        let eq40_e1281_d_n4: f64 = (eq40_e1279_d_n4 * p.p33);
        let eq40_e1281_d_n5: f64 = (eq40_e1279_d_n5 * p.p33);
        let eq40_e1281_d_n6: f64 = (eq40_e1279_d_n6 * p.p33);
        let eq40_e1281_d_n7: f64 = (eq40_e1279_d_n7 * p.p33);
        let eq40_e1281_d_n8: f64 = (eq40_e1279_d_n8 * p.p33);
        let eq40_e1281_d_n9: f64 = (eq40_e1279_d_n9 * p.p33);
        let eq40_e1281_d_n10: f64 = (eq40_e1279_d_n10 * p.p33);
        let eq40_e1281_d_n11: f64 = (eq40_e1279_d_n11 * p.p33);
        let eq40_e1281_d_b0: f64 = (eq40_e1279_d_b0 * p.p33);
        let eq40_e1281_d_b1: f64 = (eq40_e1279_d_b1 * p.p33);
        let eq40_e1281_d_b2: f64 = (eq40_e1279_d_b2 * p.p33);
        let eq40_e1281_d_b3: f64 = (eq40_e1279_d_b3 * p.p33);
        let eq40_e1281_d_b4: f64 = (eq40_e1279_d_b4 * p.p33);
        let eq40_e1281_d_b5: f64 = (eq40_e1279_d_b5 * p.p33);
        let eq40_e1281_d_b6: f64 = (eq40_e1279_d_b6 * p.p33);
        let eq40_e1283: f64 = (eq40_e1281 * s.v[847]);
        let eq40_e1283_d_n0: f64 = ((eq40_e1281_d_n0 * s.v[847]) + (eq40_e1281 * s.dn[847][0]));
        let eq40_e1283_d_n1: f64 = ((eq40_e1281_d_n1 * s.v[847]) + (eq40_e1281 * s.dn[847][1]));
        let eq40_e1283_d_n2: f64 = ((eq40_e1281_d_n2 * s.v[847]) + (eq40_e1281 * s.dn[847][2]));
        let eq40_e1283_d_n3: f64 = ((eq40_e1281_d_n3 * s.v[847]) + (eq40_e1281 * s.dn[847][3]));
        let eq40_e1283_d_n4: f64 = ((eq40_e1281_d_n4 * s.v[847]) + (eq40_e1281 * s.dn[847][4]));
        let eq40_e1283_d_n5: f64 = ((eq40_e1281_d_n5 * s.v[847]) + (eq40_e1281 * s.dn[847][5]));
        let eq40_e1283_d_n6: f64 = ((eq40_e1281_d_n6 * s.v[847]) + (eq40_e1281 * s.dn[847][6]));
        let eq40_e1283_d_n7: f64 = ((eq40_e1281_d_n7 * s.v[847]) + (eq40_e1281 * s.dn[847][7]));
        let eq40_e1283_d_n8: f64 = ((eq40_e1281_d_n8 * s.v[847]) + (eq40_e1281 * s.dn[847][8]));
        let eq40_e1283_d_n9: f64 = ((eq40_e1281_d_n9 * s.v[847]) + (eq40_e1281 * s.dn[847][9]));
        let eq40_e1283_d_n10: f64 = ((eq40_e1281_d_n10 * s.v[847]) + (eq40_e1281 * s.dn[847][10]));
        let eq40_e1283_d_n11: f64 = ((eq40_e1281_d_n11 * s.v[847]) + (eq40_e1281 * s.dn[847][11]));
        let eq40_e1283_d_b0: f64 = ((eq40_e1281_d_b0 * s.v[847]) + (eq40_e1281 * s.db[847][0]));
        let eq40_e1283_d_b1: f64 = ((eq40_e1281_d_b1 * s.v[847]) + (eq40_e1281 * s.db[847][1]));
        let eq40_e1283_d_b2: f64 = ((eq40_e1281_d_b2 * s.v[847]) + (eq40_e1281 * s.db[847][2]));
        let eq40_e1283_d_b3: f64 = ((eq40_e1281_d_b3 * s.v[847]) + (eq40_e1281 * s.db[847][3]));
        let eq40_e1283_d_b4: f64 = ((eq40_e1281_d_b4 * s.v[847]) + (eq40_e1281 * s.db[847][4]));
        let eq40_e1283_d_b5: f64 = ((eq40_e1281_d_b5 * s.v[847]) + (eq40_e1281 * s.db[847][5]));
        let eq40_e1283_d_b6: f64 = ((eq40_e1281_d_b6 * s.v[847]) + (eq40_e1281 * s.db[847][6]));
        let eq40_e1284: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 2, eq40_e1283);
        let eq40_e1284_d_n0: f64 = (eq40_e1283_d_n0 * ddt_scale);
        let eq40_e1284_d_n1: f64 = (eq40_e1283_d_n1 * ddt_scale);
        let eq40_e1284_d_n2: f64 = (eq40_e1283_d_n2 * ddt_scale);
        let eq40_e1284_d_n3: f64 = (eq40_e1283_d_n3 * ddt_scale);
        let eq40_e1284_d_n4: f64 = (eq40_e1283_d_n4 * ddt_scale);
        let eq40_e1284_d_n5: f64 = (eq40_e1283_d_n5 * ddt_scale);
        let eq40_e1284_d_n6: f64 = (eq40_e1283_d_n6 * ddt_scale);
        let eq40_e1284_d_n7: f64 = (eq40_e1283_d_n7 * ddt_scale);
        let eq40_e1284_d_n8: f64 = (eq40_e1283_d_n8 * ddt_scale);
        let eq40_e1284_d_n9: f64 = (eq40_e1283_d_n9 * ddt_scale);
        let eq40_e1284_d_n10: f64 = (eq40_e1283_d_n10 * ddt_scale);
        let eq40_e1284_d_n11: f64 = (eq40_e1283_d_n11 * ddt_scale);
        let eq40_e1284_d_b0: f64 = (eq40_e1283_d_b0 * ddt_scale);
        let eq40_e1284_d_b1: f64 = (eq40_e1283_d_b1 * ddt_scale);
        let eq40_e1284_d_b2: f64 = (eq40_e1283_d_b2 * ddt_scale);
        let eq40_e1284_d_b3: f64 = (eq40_e1283_d_b3 * ddt_scale);
        let eq40_e1284_d_b4: f64 = (eq40_e1283_d_b4 * ddt_scale);
        let eq40_e1284_d_b5: f64 = (eq40_e1283_d_b5 * ddt_scale);
        let eq40_e1284_d_b6: f64 = (eq40_e1283_d_b6 * ddt_scale);
        let eq40_value: f64 = eq40_e1284;
        let eq40_node_derivatives: [f64; 12] = [eq40_e1284_d_n0, eq40_e1284_d_n1, eq40_e1284_d_n2, eq40_e1284_d_n3, eq40_e1284_d_n4, eq40_e1284_d_n5, eq40_e1284_d_n6, eq40_e1284_d_n7, eq40_e1284_d_n8, eq40_e1284_d_n9, eq40_e1284_d_n10, eq40_e1284_d_n11];
        let eq40_branch_derivatives: [f64; 7] = [eq40_e1284_d_b0, eq40_e1284_d_b1, eq40_e1284_d_b2, eq40_e1284_d_b3, eq40_e1284_d_b4, eq40_e1284_d_b5, eq40_e1284_d_b6];
        stamper.stamp_current_dense_local(
            Some(7),
            Some(6),
            multiplicity * (eq40_value),
            &eq40_node_derivatives,
            &eq40_branch_derivatives,
            multiplicity,
        );
        let eq41_e1287: f64 = (s.v[0] * s.v[15]);
        let eq41_e1287_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq41_e1287_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq41_e1287_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq41_e1287_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq41_e1287_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq41_e1287_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq41_e1287_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq41_e1287_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq41_e1287_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq41_e1287_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq41_e1287_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq41_e1287_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq41_e1287_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq41_e1287_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq41_e1287_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq41_e1287_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq41_e1287_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq41_e1287_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq41_e1287_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq41_e1289: f64 = (eq41_e1287 * p.p33);
        let eq41_e1289_d_n0: f64 = (eq41_e1287_d_n0 * p.p33);
        let eq41_e1289_d_n1: f64 = (eq41_e1287_d_n1 * p.p33);
        let eq41_e1289_d_n2: f64 = (eq41_e1287_d_n2 * p.p33);
        let eq41_e1289_d_n3: f64 = (eq41_e1287_d_n3 * p.p33);
        let eq41_e1289_d_n4: f64 = (eq41_e1287_d_n4 * p.p33);
        let eq41_e1289_d_n5: f64 = (eq41_e1287_d_n5 * p.p33);
        let eq41_e1289_d_n6: f64 = (eq41_e1287_d_n6 * p.p33);
        let eq41_e1289_d_n7: f64 = (eq41_e1287_d_n7 * p.p33);
        let eq41_e1289_d_n8: f64 = (eq41_e1287_d_n8 * p.p33);
        let eq41_e1289_d_n9: f64 = (eq41_e1287_d_n9 * p.p33);
        let eq41_e1289_d_n10: f64 = (eq41_e1287_d_n10 * p.p33);
        let eq41_e1289_d_n11: f64 = (eq41_e1287_d_n11 * p.p33);
        let eq41_e1289_d_b0: f64 = (eq41_e1287_d_b0 * p.p33);
        let eq41_e1289_d_b1: f64 = (eq41_e1287_d_b1 * p.p33);
        let eq41_e1289_d_b2: f64 = (eq41_e1287_d_b2 * p.p33);
        let eq41_e1289_d_b3: f64 = (eq41_e1287_d_b3 * p.p33);
        let eq41_e1289_d_b4: f64 = (eq41_e1287_d_b4 * p.p33);
        let eq41_e1289_d_b5: f64 = (eq41_e1287_d_b5 * p.p33);
        let eq41_e1289_d_b6: f64 = (eq41_e1287_d_b6 * p.p33);
        let eq41_e1291: f64 = (eq41_e1289 * s.v[848]);
        let eq41_e1291_d_n0: f64 = ((eq41_e1289_d_n0 * s.v[848]) + (eq41_e1289 * s.dn[848][0]));
        let eq41_e1291_d_n1: f64 = ((eq41_e1289_d_n1 * s.v[848]) + (eq41_e1289 * s.dn[848][1]));
        let eq41_e1291_d_n2: f64 = ((eq41_e1289_d_n2 * s.v[848]) + (eq41_e1289 * s.dn[848][2]));
        let eq41_e1291_d_n3: f64 = ((eq41_e1289_d_n3 * s.v[848]) + (eq41_e1289 * s.dn[848][3]));
        let eq41_e1291_d_n4: f64 = ((eq41_e1289_d_n4 * s.v[848]) + (eq41_e1289 * s.dn[848][4]));
        let eq41_e1291_d_n5: f64 = ((eq41_e1289_d_n5 * s.v[848]) + (eq41_e1289 * s.dn[848][5]));
        let eq41_e1291_d_n6: f64 = ((eq41_e1289_d_n6 * s.v[848]) + (eq41_e1289 * s.dn[848][6]));
        let eq41_e1291_d_n7: f64 = ((eq41_e1289_d_n7 * s.v[848]) + (eq41_e1289 * s.dn[848][7]));
        let eq41_e1291_d_n8: f64 = ((eq41_e1289_d_n8 * s.v[848]) + (eq41_e1289 * s.dn[848][8]));
        let eq41_e1291_d_n9: f64 = ((eq41_e1289_d_n9 * s.v[848]) + (eq41_e1289 * s.dn[848][9]));
        let eq41_e1291_d_n10: f64 = ((eq41_e1289_d_n10 * s.v[848]) + (eq41_e1289 * s.dn[848][10]));
        let eq41_e1291_d_n11: f64 = ((eq41_e1289_d_n11 * s.v[848]) + (eq41_e1289 * s.dn[848][11]));
        let eq41_e1291_d_b0: f64 = ((eq41_e1289_d_b0 * s.v[848]) + (eq41_e1289 * s.db[848][0]));
        let eq41_e1291_d_b1: f64 = ((eq41_e1289_d_b1 * s.v[848]) + (eq41_e1289 * s.db[848][1]));
        let eq41_e1291_d_b2: f64 = ((eq41_e1289_d_b2 * s.v[848]) + (eq41_e1289 * s.db[848][2]));
        let eq41_e1291_d_b3: f64 = ((eq41_e1289_d_b3 * s.v[848]) + (eq41_e1289 * s.db[848][3]));
        let eq41_e1291_d_b4: f64 = ((eq41_e1289_d_b4 * s.v[848]) + (eq41_e1289 * s.db[848][4]));
        let eq41_e1291_d_b5: f64 = ((eq41_e1289_d_b5 * s.v[848]) + (eq41_e1289 * s.db[848][5]));
        let eq41_e1291_d_b6: f64 = ((eq41_e1289_d_b6 * s.v[848]) + (eq41_e1289 * s.db[848][6]));
        let eq41_e1292: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 3, eq41_e1291);
        let eq41_e1292_d_n0: f64 = (eq41_e1291_d_n0 * ddt_scale);
        let eq41_e1292_d_n1: f64 = (eq41_e1291_d_n1 * ddt_scale);
        let eq41_e1292_d_n2: f64 = (eq41_e1291_d_n2 * ddt_scale);
        let eq41_e1292_d_n3: f64 = (eq41_e1291_d_n3 * ddt_scale);
        let eq41_e1292_d_n4: f64 = (eq41_e1291_d_n4 * ddt_scale);
        let eq41_e1292_d_n5: f64 = (eq41_e1291_d_n5 * ddt_scale);
        let eq41_e1292_d_n6: f64 = (eq41_e1291_d_n6 * ddt_scale);
        let eq41_e1292_d_n7: f64 = (eq41_e1291_d_n7 * ddt_scale);
        let eq41_e1292_d_n8: f64 = (eq41_e1291_d_n8 * ddt_scale);
        let eq41_e1292_d_n9: f64 = (eq41_e1291_d_n9 * ddt_scale);
        let eq41_e1292_d_n10: f64 = (eq41_e1291_d_n10 * ddt_scale);
        let eq41_e1292_d_n11: f64 = (eq41_e1291_d_n11 * ddt_scale);
        let eq41_e1292_d_b0: f64 = (eq41_e1291_d_b0 * ddt_scale);
        let eq41_e1292_d_b1: f64 = (eq41_e1291_d_b1 * ddt_scale);
        let eq41_e1292_d_b2: f64 = (eq41_e1291_d_b2 * ddt_scale);
        let eq41_e1292_d_b3: f64 = (eq41_e1291_d_b3 * ddt_scale);
        let eq41_e1292_d_b4: f64 = (eq41_e1291_d_b4 * ddt_scale);
        let eq41_e1292_d_b5: f64 = (eq41_e1291_d_b5 * ddt_scale);
        let eq41_e1292_d_b6: f64 = (eq41_e1291_d_b6 * ddt_scale);
        let eq41_value: f64 = eq41_e1292;
        let eq41_node_derivatives: [f64; 12] = [eq41_e1292_d_n0, eq41_e1292_d_n1, eq41_e1292_d_n2, eq41_e1292_d_n3, eq41_e1292_d_n4, eq41_e1292_d_n5, eq41_e1292_d_n6, eq41_e1292_d_n7, eq41_e1292_d_n8, eq41_e1292_d_n9, eq41_e1292_d_n10, eq41_e1292_d_n11];
        let eq41_branch_derivatives: [f64; 7] = [eq41_e1292_d_b0, eq41_e1292_d_b1, eq41_e1292_d_b2, eq41_e1292_d_b3, eq41_e1292_d_b4, eq41_e1292_d_b5, eq41_e1292_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq41_value),
            &eq41_node_derivatives,
            &eq41_branch_derivatives,
            multiplicity,
        );
        let eq42_e1295: f64 = (s.v[0] * s.v[15]);
        let eq42_e1295_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq42_e1295_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq42_e1295_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq42_e1295_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq42_e1295_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq42_e1295_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq42_e1295_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq42_e1295_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq42_e1295_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq42_e1295_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq42_e1295_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq42_e1295_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq42_e1295_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq42_e1295_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq42_e1295_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq42_e1295_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq42_e1295_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq42_e1295_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq42_e1295_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq42_e1297: f64 = (eq42_e1295 * p.p33);
        let eq42_e1297_d_n0: f64 = (eq42_e1295_d_n0 * p.p33);
        let eq42_e1297_d_n1: f64 = (eq42_e1295_d_n1 * p.p33);
        let eq42_e1297_d_n2: f64 = (eq42_e1295_d_n2 * p.p33);
        let eq42_e1297_d_n3: f64 = (eq42_e1295_d_n3 * p.p33);
        let eq42_e1297_d_n4: f64 = (eq42_e1295_d_n4 * p.p33);
        let eq42_e1297_d_n5: f64 = (eq42_e1295_d_n5 * p.p33);
        let eq42_e1297_d_n6: f64 = (eq42_e1295_d_n6 * p.p33);
        let eq42_e1297_d_n7: f64 = (eq42_e1295_d_n7 * p.p33);
        let eq42_e1297_d_n8: f64 = (eq42_e1295_d_n8 * p.p33);
        let eq42_e1297_d_n9: f64 = (eq42_e1295_d_n9 * p.p33);
        let eq42_e1297_d_n10: f64 = (eq42_e1295_d_n10 * p.p33);
        let eq42_e1297_d_n11: f64 = (eq42_e1295_d_n11 * p.p33);
        let eq42_e1297_d_b0: f64 = (eq42_e1295_d_b0 * p.p33);
        let eq42_e1297_d_b1: f64 = (eq42_e1295_d_b1 * p.p33);
        let eq42_e1297_d_b2: f64 = (eq42_e1295_d_b2 * p.p33);
        let eq42_e1297_d_b3: f64 = (eq42_e1295_d_b3 * p.p33);
        let eq42_e1297_d_b4: f64 = (eq42_e1295_d_b4 * p.p33);
        let eq42_e1297_d_b5: f64 = (eq42_e1295_d_b5 * p.p33);
        let eq42_e1297_d_b6: f64 = (eq42_e1295_d_b6 * p.p33);
        let eq42_e1299: f64 = (eq42_e1297 * s.v[849]);
        let eq42_e1299_d_n0: f64 = ((eq42_e1297_d_n0 * s.v[849]) + (eq42_e1297 * s.dn[849][0]));
        let eq42_e1299_d_n1: f64 = ((eq42_e1297_d_n1 * s.v[849]) + (eq42_e1297 * s.dn[849][1]));
        let eq42_e1299_d_n2: f64 = ((eq42_e1297_d_n2 * s.v[849]) + (eq42_e1297 * s.dn[849][2]));
        let eq42_e1299_d_n3: f64 = ((eq42_e1297_d_n3 * s.v[849]) + (eq42_e1297 * s.dn[849][3]));
        let eq42_e1299_d_n4: f64 = ((eq42_e1297_d_n4 * s.v[849]) + (eq42_e1297 * s.dn[849][4]));
        let eq42_e1299_d_n5: f64 = ((eq42_e1297_d_n5 * s.v[849]) + (eq42_e1297 * s.dn[849][5]));
        let eq42_e1299_d_n6: f64 = ((eq42_e1297_d_n6 * s.v[849]) + (eq42_e1297 * s.dn[849][6]));
        let eq42_e1299_d_n7: f64 = ((eq42_e1297_d_n7 * s.v[849]) + (eq42_e1297 * s.dn[849][7]));
        let eq42_e1299_d_n8: f64 = ((eq42_e1297_d_n8 * s.v[849]) + (eq42_e1297 * s.dn[849][8]));
        let eq42_e1299_d_n9: f64 = ((eq42_e1297_d_n9 * s.v[849]) + (eq42_e1297 * s.dn[849][9]));
        let eq42_e1299_d_n10: f64 = ((eq42_e1297_d_n10 * s.v[849]) + (eq42_e1297 * s.dn[849][10]));
        let eq42_e1299_d_n11: f64 = ((eq42_e1297_d_n11 * s.v[849]) + (eq42_e1297 * s.dn[849][11]));
        let eq42_e1299_d_b0: f64 = ((eq42_e1297_d_b0 * s.v[849]) + (eq42_e1297 * s.db[849][0]));
        let eq42_e1299_d_b1: f64 = ((eq42_e1297_d_b1 * s.v[849]) + (eq42_e1297 * s.db[849][1]));
        let eq42_e1299_d_b2: f64 = ((eq42_e1297_d_b2 * s.v[849]) + (eq42_e1297 * s.db[849][2]));
        let eq42_e1299_d_b3: f64 = ((eq42_e1297_d_b3 * s.v[849]) + (eq42_e1297 * s.db[849][3]));
        let eq42_e1299_d_b4: f64 = ((eq42_e1297_d_b4 * s.v[849]) + (eq42_e1297 * s.db[849][4]));
        let eq42_e1299_d_b5: f64 = ((eq42_e1297_d_b5 * s.v[849]) + (eq42_e1297 * s.db[849][5]));
        let eq42_e1299_d_b6: f64 = ((eq42_e1297_d_b6 * s.v[849]) + (eq42_e1297 * s.db[849][6]));
        let eq42_e1300: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 4, eq42_e1299);
        let eq42_e1300_d_n0: f64 = (eq42_e1299_d_n0 * ddt_scale);
        let eq42_e1300_d_n1: f64 = (eq42_e1299_d_n1 * ddt_scale);
        let eq42_e1300_d_n2: f64 = (eq42_e1299_d_n2 * ddt_scale);
        let eq42_e1300_d_n3: f64 = (eq42_e1299_d_n3 * ddt_scale);
        let eq42_e1300_d_n4: f64 = (eq42_e1299_d_n4 * ddt_scale);
        let eq42_e1300_d_n5: f64 = (eq42_e1299_d_n5 * ddt_scale);
        let eq42_e1300_d_n6: f64 = (eq42_e1299_d_n6 * ddt_scale);
        let eq42_e1300_d_n7: f64 = (eq42_e1299_d_n7 * ddt_scale);
        let eq42_e1300_d_n8: f64 = (eq42_e1299_d_n8 * ddt_scale);
        let eq42_e1300_d_n9: f64 = (eq42_e1299_d_n9 * ddt_scale);
        let eq42_e1300_d_n10: f64 = (eq42_e1299_d_n10 * ddt_scale);
        let eq42_e1300_d_n11: f64 = (eq42_e1299_d_n11 * ddt_scale);
        let eq42_e1300_d_b0: f64 = (eq42_e1299_d_b0 * ddt_scale);
        let eq42_e1300_d_b1: f64 = (eq42_e1299_d_b1 * ddt_scale);
        let eq42_e1300_d_b2: f64 = (eq42_e1299_d_b2 * ddt_scale);
        let eq42_e1300_d_b3: f64 = (eq42_e1299_d_b3 * ddt_scale);
        let eq42_e1300_d_b4: f64 = (eq42_e1299_d_b4 * ddt_scale);
        let eq42_e1300_d_b5: f64 = (eq42_e1299_d_b5 * ddt_scale);
        let eq42_e1300_d_b6: f64 = (eq42_e1299_d_b6 * ddt_scale);
        let eq42_value: f64 = eq42_e1300;
        let eq42_node_derivatives: [f64; 12] = [eq42_e1300_d_n0, eq42_e1300_d_n1, eq42_e1300_d_n2, eq42_e1300_d_n3, eq42_e1300_d_n4, eq42_e1300_d_n5, eq42_e1300_d_n6, eq42_e1300_d_n7, eq42_e1300_d_n8, eq42_e1300_d_n9, eq42_e1300_d_n10, eq42_e1300_d_n11];
        let eq42_branch_derivatives: [f64; 7] = [eq42_e1300_d_b0, eq42_e1300_d_b1, eq42_e1300_d_b2, eq42_e1300_d_b3, eq42_e1300_d_b4, eq42_e1300_d_b5, eq42_e1300_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq42_value),
            &eq42_node_derivatives,
            &eq42_branch_derivatives,
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
        let eq43_e1303: f64 = (s.v[0] * s.v[15]);
        let eq43_e1303_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq43_e1303_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq43_e1303_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq43_e1303_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq43_e1303_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq43_e1303_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq43_e1303_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq43_e1303_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq43_e1303_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq43_e1303_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq43_e1303_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq43_e1303_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq43_e1303_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq43_e1303_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq43_e1303_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq43_e1303_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq43_e1303_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq43_e1303_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq43_e1303_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq43_e1305: f64 = (eq43_e1303 * p.p33);
        let eq43_e1305_d_n0: f64 = (eq43_e1303_d_n0 * p.p33);
        let eq43_e1305_d_n1: f64 = (eq43_e1303_d_n1 * p.p33);
        let eq43_e1305_d_n2: f64 = (eq43_e1303_d_n2 * p.p33);
        let eq43_e1305_d_n3: f64 = (eq43_e1303_d_n3 * p.p33);
        let eq43_e1305_d_n4: f64 = (eq43_e1303_d_n4 * p.p33);
        let eq43_e1305_d_n5: f64 = (eq43_e1303_d_n5 * p.p33);
        let eq43_e1305_d_n6: f64 = (eq43_e1303_d_n6 * p.p33);
        let eq43_e1305_d_n7: f64 = (eq43_e1303_d_n7 * p.p33);
        let eq43_e1305_d_n8: f64 = (eq43_e1303_d_n8 * p.p33);
        let eq43_e1305_d_n9: f64 = (eq43_e1303_d_n9 * p.p33);
        let eq43_e1305_d_n10: f64 = (eq43_e1303_d_n10 * p.p33);
        let eq43_e1305_d_n11: f64 = (eq43_e1303_d_n11 * p.p33);
        let eq43_e1305_d_b0: f64 = (eq43_e1303_d_b0 * p.p33);
        let eq43_e1305_d_b1: f64 = (eq43_e1303_d_b1 * p.p33);
        let eq43_e1305_d_b2: f64 = (eq43_e1303_d_b2 * p.p33);
        let eq43_e1305_d_b3: f64 = (eq43_e1303_d_b3 * p.p33);
        let eq43_e1305_d_b4: f64 = (eq43_e1303_d_b4 * p.p33);
        let eq43_e1305_d_b5: f64 = (eq43_e1303_d_b5 * p.p33);
        let eq43_e1305_d_b6: f64 = (eq43_e1303_d_b6 * p.p33);
        let eq43_e1307: f64 = (eq43_e1305 * s.v[850]);
        let eq43_e1307_d_n0: f64 = ((eq43_e1305_d_n0 * s.v[850]) + (eq43_e1305 * s.dn[850][0]));
        let eq43_e1307_d_n1: f64 = ((eq43_e1305_d_n1 * s.v[850]) + (eq43_e1305 * s.dn[850][1]));
        let eq43_e1307_d_n2: f64 = ((eq43_e1305_d_n2 * s.v[850]) + (eq43_e1305 * s.dn[850][2]));
        let eq43_e1307_d_n3: f64 = ((eq43_e1305_d_n3 * s.v[850]) + (eq43_e1305 * s.dn[850][3]));
        let eq43_e1307_d_n4: f64 = ((eq43_e1305_d_n4 * s.v[850]) + (eq43_e1305 * s.dn[850][4]));
        let eq43_e1307_d_n5: f64 = ((eq43_e1305_d_n5 * s.v[850]) + (eq43_e1305 * s.dn[850][5]));
        let eq43_e1307_d_n6: f64 = ((eq43_e1305_d_n6 * s.v[850]) + (eq43_e1305 * s.dn[850][6]));
        let eq43_e1307_d_n7: f64 = ((eq43_e1305_d_n7 * s.v[850]) + (eq43_e1305 * s.dn[850][7]));
        let eq43_e1307_d_n8: f64 = ((eq43_e1305_d_n8 * s.v[850]) + (eq43_e1305 * s.dn[850][8]));
        let eq43_e1307_d_n9: f64 = ((eq43_e1305_d_n9 * s.v[850]) + (eq43_e1305 * s.dn[850][9]));
        let eq43_e1307_d_n10: f64 = ((eq43_e1305_d_n10 * s.v[850]) + (eq43_e1305 * s.dn[850][10]));
        let eq43_e1307_d_n11: f64 = ((eq43_e1305_d_n11 * s.v[850]) + (eq43_e1305 * s.dn[850][11]));
        let eq43_e1307_d_b0: f64 = ((eq43_e1305_d_b0 * s.v[850]) + (eq43_e1305 * s.db[850][0]));
        let eq43_e1307_d_b1: f64 = ((eq43_e1305_d_b1 * s.v[850]) + (eq43_e1305 * s.db[850][1]));
        let eq43_e1307_d_b2: f64 = ((eq43_e1305_d_b2 * s.v[850]) + (eq43_e1305 * s.db[850][2]));
        let eq43_e1307_d_b3: f64 = ((eq43_e1305_d_b3 * s.v[850]) + (eq43_e1305 * s.db[850][3]));
        let eq43_e1307_d_b4: f64 = ((eq43_e1305_d_b4 * s.v[850]) + (eq43_e1305 * s.db[850][4]));
        let eq43_e1307_d_b5: f64 = ((eq43_e1305_d_b5 * s.v[850]) + (eq43_e1305 * s.db[850][5]));
        let eq43_e1307_d_b6: f64 = ((eq43_e1305_d_b6 * s.v[850]) + (eq43_e1305 * s.db[850][6]));
        let eq43_e1308: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 5, eq43_e1307);
        let eq43_e1308_d_n0: f64 = (eq43_e1307_d_n0 * ddt_scale);
        let eq43_e1308_d_n1: f64 = (eq43_e1307_d_n1 * ddt_scale);
        let eq43_e1308_d_n2: f64 = (eq43_e1307_d_n2 * ddt_scale);
        let eq43_e1308_d_n3: f64 = (eq43_e1307_d_n3 * ddt_scale);
        let eq43_e1308_d_n4: f64 = (eq43_e1307_d_n4 * ddt_scale);
        let eq43_e1308_d_n5: f64 = (eq43_e1307_d_n5 * ddt_scale);
        let eq43_e1308_d_n6: f64 = (eq43_e1307_d_n6 * ddt_scale);
        let eq43_e1308_d_n7: f64 = (eq43_e1307_d_n7 * ddt_scale);
        let eq43_e1308_d_n8: f64 = (eq43_e1307_d_n8 * ddt_scale);
        let eq43_e1308_d_n9: f64 = (eq43_e1307_d_n9 * ddt_scale);
        let eq43_e1308_d_n10: f64 = (eq43_e1307_d_n10 * ddt_scale);
        let eq43_e1308_d_n11: f64 = (eq43_e1307_d_n11 * ddt_scale);
        let eq43_e1308_d_b0: f64 = (eq43_e1307_d_b0 * ddt_scale);
        let eq43_e1308_d_b1: f64 = (eq43_e1307_d_b1 * ddt_scale);
        let eq43_e1308_d_b2: f64 = (eq43_e1307_d_b2 * ddt_scale);
        let eq43_e1308_d_b3: f64 = (eq43_e1307_d_b3 * ddt_scale);
        let eq43_e1308_d_b4: f64 = (eq43_e1307_d_b4 * ddt_scale);
        let eq43_e1308_d_b5: f64 = (eq43_e1307_d_b5 * ddt_scale);
        let eq43_e1308_d_b6: f64 = (eq43_e1307_d_b6 * ddt_scale);
        let eq43_value: f64 = eq43_e1308;
        let eq43_node_derivatives: [f64; 12] = [eq43_e1308_d_n0, eq43_e1308_d_n1, eq43_e1308_d_n2, eq43_e1308_d_n3, eq43_e1308_d_n4, eq43_e1308_d_n5, eq43_e1308_d_n6, eq43_e1308_d_n7, eq43_e1308_d_n8, eq43_e1308_d_n9, eq43_e1308_d_n10, eq43_e1308_d_n11];
        let eq43_branch_derivatives: [f64; 7] = [eq43_e1308_d_b0, eq43_e1308_d_b1, eq43_e1308_d_b2, eq43_e1308_d_b3, eq43_e1308_d_b4, eq43_e1308_d_b5, eq43_e1308_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(8),
            multiplicity * (eq43_value),
            &eq43_node_derivatives,
            &eq43_branch_derivatives,
            multiplicity,
        );
        let eq44_e1311: f64 = (s.v[0] * s.v[15]);
        let eq44_e1311_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq44_e1311_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq44_e1311_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq44_e1311_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq44_e1311_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq44_e1311_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq44_e1311_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq44_e1311_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq44_e1311_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq44_e1311_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq44_e1311_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq44_e1311_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq44_e1311_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq44_e1311_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq44_e1311_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq44_e1311_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq44_e1311_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq44_e1311_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq44_e1311_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq44_e1313: f64 = (eq44_e1311 * p.p33);
        let eq44_e1313_d_n0: f64 = (eq44_e1311_d_n0 * p.p33);
        let eq44_e1313_d_n1: f64 = (eq44_e1311_d_n1 * p.p33);
        let eq44_e1313_d_n2: f64 = (eq44_e1311_d_n2 * p.p33);
        let eq44_e1313_d_n3: f64 = (eq44_e1311_d_n3 * p.p33);
        let eq44_e1313_d_n4: f64 = (eq44_e1311_d_n4 * p.p33);
        let eq44_e1313_d_n5: f64 = (eq44_e1311_d_n5 * p.p33);
        let eq44_e1313_d_n6: f64 = (eq44_e1311_d_n6 * p.p33);
        let eq44_e1313_d_n7: f64 = (eq44_e1311_d_n7 * p.p33);
        let eq44_e1313_d_n8: f64 = (eq44_e1311_d_n8 * p.p33);
        let eq44_e1313_d_n9: f64 = (eq44_e1311_d_n9 * p.p33);
        let eq44_e1313_d_n10: f64 = (eq44_e1311_d_n10 * p.p33);
        let eq44_e1313_d_n11: f64 = (eq44_e1311_d_n11 * p.p33);
        let eq44_e1313_d_b0: f64 = (eq44_e1311_d_b0 * p.p33);
        let eq44_e1313_d_b1: f64 = (eq44_e1311_d_b1 * p.p33);
        let eq44_e1313_d_b2: f64 = (eq44_e1311_d_b2 * p.p33);
        let eq44_e1313_d_b3: f64 = (eq44_e1311_d_b3 * p.p33);
        let eq44_e1313_d_b4: f64 = (eq44_e1311_d_b4 * p.p33);
        let eq44_e1313_d_b5: f64 = (eq44_e1311_d_b5 * p.p33);
        let eq44_e1313_d_b6: f64 = (eq44_e1311_d_b6 * p.p33);
        let eq44_e1315: f64 = (eq44_e1313 * s.v[851]);
        let eq44_e1315_d_n0: f64 = ((eq44_e1313_d_n0 * s.v[851]) + (eq44_e1313 * s.dn[851][0]));
        let eq44_e1315_d_n1: f64 = ((eq44_e1313_d_n1 * s.v[851]) + (eq44_e1313 * s.dn[851][1]));
        let eq44_e1315_d_n2: f64 = ((eq44_e1313_d_n2 * s.v[851]) + (eq44_e1313 * s.dn[851][2]));
        let eq44_e1315_d_n3: f64 = ((eq44_e1313_d_n3 * s.v[851]) + (eq44_e1313 * s.dn[851][3]));
        let eq44_e1315_d_n4: f64 = ((eq44_e1313_d_n4 * s.v[851]) + (eq44_e1313 * s.dn[851][4]));
        let eq44_e1315_d_n5: f64 = ((eq44_e1313_d_n5 * s.v[851]) + (eq44_e1313 * s.dn[851][5]));
        let eq44_e1315_d_n6: f64 = ((eq44_e1313_d_n6 * s.v[851]) + (eq44_e1313 * s.dn[851][6]));
        let eq44_e1315_d_n7: f64 = ((eq44_e1313_d_n7 * s.v[851]) + (eq44_e1313 * s.dn[851][7]));
        let eq44_e1315_d_n8: f64 = ((eq44_e1313_d_n8 * s.v[851]) + (eq44_e1313 * s.dn[851][8]));
        let eq44_e1315_d_n9: f64 = ((eq44_e1313_d_n9 * s.v[851]) + (eq44_e1313 * s.dn[851][9]));
        let eq44_e1315_d_n10: f64 = ((eq44_e1313_d_n10 * s.v[851]) + (eq44_e1313 * s.dn[851][10]));
        let eq44_e1315_d_n11: f64 = ((eq44_e1313_d_n11 * s.v[851]) + (eq44_e1313 * s.dn[851][11]));
        let eq44_e1315_d_b0: f64 = ((eq44_e1313_d_b0 * s.v[851]) + (eq44_e1313 * s.db[851][0]));
        let eq44_e1315_d_b1: f64 = ((eq44_e1313_d_b1 * s.v[851]) + (eq44_e1313 * s.db[851][1]));
        let eq44_e1315_d_b2: f64 = ((eq44_e1313_d_b2 * s.v[851]) + (eq44_e1313 * s.db[851][2]));
        let eq44_e1315_d_b3: f64 = ((eq44_e1313_d_b3 * s.v[851]) + (eq44_e1313 * s.db[851][3]));
        let eq44_e1315_d_b4: f64 = ((eq44_e1313_d_b4 * s.v[851]) + (eq44_e1313 * s.db[851][4]));
        let eq44_e1315_d_b5: f64 = ((eq44_e1313_d_b5 * s.v[851]) + (eq44_e1313 * s.db[851][5]));
        let eq44_e1315_d_b6: f64 = ((eq44_e1313_d_b6 * s.v[851]) + (eq44_e1313 * s.db[851][6]));
        let eq44_e1316: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 6, eq44_e1315);
        let eq44_e1316_d_n0: f64 = (eq44_e1315_d_n0 * ddt_scale);
        let eq44_e1316_d_n1: f64 = (eq44_e1315_d_n1 * ddt_scale);
        let eq44_e1316_d_n2: f64 = (eq44_e1315_d_n2 * ddt_scale);
        let eq44_e1316_d_n3: f64 = (eq44_e1315_d_n3 * ddt_scale);
        let eq44_e1316_d_n4: f64 = (eq44_e1315_d_n4 * ddt_scale);
        let eq44_e1316_d_n5: f64 = (eq44_e1315_d_n5 * ddt_scale);
        let eq44_e1316_d_n6: f64 = (eq44_e1315_d_n6 * ddt_scale);
        let eq44_e1316_d_n7: f64 = (eq44_e1315_d_n7 * ddt_scale);
        let eq44_e1316_d_n8: f64 = (eq44_e1315_d_n8 * ddt_scale);
        let eq44_e1316_d_n9: f64 = (eq44_e1315_d_n9 * ddt_scale);
        let eq44_e1316_d_n10: f64 = (eq44_e1315_d_n10 * ddt_scale);
        let eq44_e1316_d_n11: f64 = (eq44_e1315_d_n11 * ddt_scale);
        let eq44_e1316_d_b0: f64 = (eq44_e1315_d_b0 * ddt_scale);
        let eq44_e1316_d_b1: f64 = (eq44_e1315_d_b1 * ddt_scale);
        let eq44_e1316_d_b2: f64 = (eq44_e1315_d_b2 * ddt_scale);
        let eq44_e1316_d_b3: f64 = (eq44_e1315_d_b3 * ddt_scale);
        let eq44_e1316_d_b4: f64 = (eq44_e1315_d_b4 * ddt_scale);
        let eq44_e1316_d_b5: f64 = (eq44_e1315_d_b5 * ddt_scale);
        let eq44_e1316_d_b6: f64 = (eq44_e1315_d_b6 * ddt_scale);
        let eq44_value: f64 = eq44_e1316;
        let eq44_node_derivatives: [f64; 12] = [eq44_e1316_d_n0, eq44_e1316_d_n1, eq44_e1316_d_n2, eq44_e1316_d_n3, eq44_e1316_d_n4, eq44_e1316_d_n5, eq44_e1316_d_n6, eq44_e1316_d_n7, eq44_e1316_d_n8, eq44_e1316_d_n9, eq44_e1316_d_n10, eq44_e1316_d_n11];
        let eq44_branch_derivatives: [f64; 7] = [eq44_e1316_d_b0, eq44_e1316_d_b1, eq44_e1316_d_b2, eq44_e1316_d_b3, eq44_e1316_d_b4, eq44_e1316_d_b5, eq44_e1316_d_b6];
        stamper.stamp_current_dense_local(
            Some(10),
            Some(6),
            multiplicity * (eq44_value),
            &eq44_node_derivatives,
            &eq44_branch_derivatives,
            multiplicity,
        );
        let eq45_e1319: f64 = (s.v[0] * s.v[15]);
        let eq45_e1319_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq45_e1319_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq45_e1319_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq45_e1319_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq45_e1319_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq45_e1319_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq45_e1319_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq45_e1319_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq45_e1319_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq45_e1319_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq45_e1319_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq45_e1319_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq45_e1319_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq45_e1319_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq45_e1319_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq45_e1319_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq45_e1319_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq45_e1319_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq45_e1319_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq45_e1321: f64 = (eq45_e1319 * p.p33);
        let eq45_e1321_d_n0: f64 = (eq45_e1319_d_n0 * p.p33);
        let eq45_e1321_d_n1: f64 = (eq45_e1319_d_n1 * p.p33);
        let eq45_e1321_d_n2: f64 = (eq45_e1319_d_n2 * p.p33);
        let eq45_e1321_d_n3: f64 = (eq45_e1319_d_n3 * p.p33);
        let eq45_e1321_d_n4: f64 = (eq45_e1319_d_n4 * p.p33);
        let eq45_e1321_d_n5: f64 = (eq45_e1319_d_n5 * p.p33);
        let eq45_e1321_d_n6: f64 = (eq45_e1319_d_n6 * p.p33);
        let eq45_e1321_d_n7: f64 = (eq45_e1319_d_n7 * p.p33);
        let eq45_e1321_d_n8: f64 = (eq45_e1319_d_n8 * p.p33);
        let eq45_e1321_d_n9: f64 = (eq45_e1319_d_n9 * p.p33);
        let eq45_e1321_d_n10: f64 = (eq45_e1319_d_n10 * p.p33);
        let eq45_e1321_d_n11: f64 = (eq45_e1319_d_n11 * p.p33);
        let eq45_e1321_d_b0: f64 = (eq45_e1319_d_b0 * p.p33);
        let eq45_e1321_d_b1: f64 = (eq45_e1319_d_b1 * p.p33);
        let eq45_e1321_d_b2: f64 = (eq45_e1319_d_b2 * p.p33);
        let eq45_e1321_d_b3: f64 = (eq45_e1319_d_b3 * p.p33);
        let eq45_e1321_d_b4: f64 = (eq45_e1319_d_b4 * p.p33);
        let eq45_e1321_d_b5: f64 = (eq45_e1319_d_b5 * p.p33);
        let eq45_e1321_d_b6: f64 = (eq45_e1319_d_b6 * p.p33);
        let eq45_e1323: f64 = (eq45_e1321 * s.v[852]);
        let eq45_e1323_d_n0: f64 = ((eq45_e1321_d_n0 * s.v[852]) + (eq45_e1321 * s.dn[852][0]));
        let eq45_e1323_d_n1: f64 = ((eq45_e1321_d_n1 * s.v[852]) + (eq45_e1321 * s.dn[852][1]));
        let eq45_e1323_d_n2: f64 = ((eq45_e1321_d_n2 * s.v[852]) + (eq45_e1321 * s.dn[852][2]));
        let eq45_e1323_d_n3: f64 = ((eq45_e1321_d_n3 * s.v[852]) + (eq45_e1321 * s.dn[852][3]));
        let eq45_e1323_d_n4: f64 = ((eq45_e1321_d_n4 * s.v[852]) + (eq45_e1321 * s.dn[852][4]));
        let eq45_e1323_d_n5: f64 = ((eq45_e1321_d_n5 * s.v[852]) + (eq45_e1321 * s.dn[852][5]));
        let eq45_e1323_d_n6: f64 = ((eq45_e1321_d_n6 * s.v[852]) + (eq45_e1321 * s.dn[852][6]));
        let eq45_e1323_d_n7: f64 = ((eq45_e1321_d_n7 * s.v[852]) + (eq45_e1321 * s.dn[852][7]));
        let eq45_e1323_d_n8: f64 = ((eq45_e1321_d_n8 * s.v[852]) + (eq45_e1321 * s.dn[852][8]));
        let eq45_e1323_d_n9: f64 = ((eq45_e1321_d_n9 * s.v[852]) + (eq45_e1321 * s.dn[852][9]));
        let eq45_e1323_d_n10: f64 = ((eq45_e1321_d_n10 * s.v[852]) + (eq45_e1321 * s.dn[852][10]));
        let eq45_e1323_d_n11: f64 = ((eq45_e1321_d_n11 * s.v[852]) + (eq45_e1321 * s.dn[852][11]));
        let eq45_e1323_d_b0: f64 = ((eq45_e1321_d_b0 * s.v[852]) + (eq45_e1321 * s.db[852][0]));
        let eq45_e1323_d_b1: f64 = ((eq45_e1321_d_b1 * s.v[852]) + (eq45_e1321 * s.db[852][1]));
        let eq45_e1323_d_b2: f64 = ((eq45_e1321_d_b2 * s.v[852]) + (eq45_e1321 * s.db[852][2]));
        let eq45_e1323_d_b3: f64 = ((eq45_e1321_d_b3 * s.v[852]) + (eq45_e1321 * s.db[852][3]));
        let eq45_e1323_d_b4: f64 = ((eq45_e1321_d_b4 * s.v[852]) + (eq45_e1321 * s.db[852][4]));
        let eq45_e1323_d_b5: f64 = ((eq45_e1321_d_b5 * s.v[852]) + (eq45_e1321 * s.db[852][5]));
        let eq45_e1323_d_b6: f64 = ((eq45_e1321_d_b6 * s.v[852]) + (eq45_e1321 * s.db[852][6]));
        let eq45_e1324: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 7, eq45_e1323);
        let eq45_e1324_d_n0: f64 = (eq45_e1323_d_n0 * ddt_scale);
        let eq45_e1324_d_n1: f64 = (eq45_e1323_d_n1 * ddt_scale);
        let eq45_e1324_d_n2: f64 = (eq45_e1323_d_n2 * ddt_scale);
        let eq45_e1324_d_n3: f64 = (eq45_e1323_d_n3 * ddt_scale);
        let eq45_e1324_d_n4: f64 = (eq45_e1323_d_n4 * ddt_scale);
        let eq45_e1324_d_n5: f64 = (eq45_e1323_d_n5 * ddt_scale);
        let eq45_e1324_d_n6: f64 = (eq45_e1323_d_n6 * ddt_scale);
        let eq45_e1324_d_n7: f64 = (eq45_e1323_d_n7 * ddt_scale);
        let eq45_e1324_d_n8: f64 = (eq45_e1323_d_n8 * ddt_scale);
        let eq45_e1324_d_n9: f64 = (eq45_e1323_d_n9 * ddt_scale);
        let eq45_e1324_d_n10: f64 = (eq45_e1323_d_n10 * ddt_scale);
        let eq45_e1324_d_n11: f64 = (eq45_e1323_d_n11 * ddt_scale);
        let eq45_e1324_d_b0: f64 = (eq45_e1323_d_b0 * ddt_scale);
        let eq45_e1324_d_b1: f64 = (eq45_e1323_d_b1 * ddt_scale);
        let eq45_e1324_d_b2: f64 = (eq45_e1323_d_b2 * ddt_scale);
        let eq45_e1324_d_b3: f64 = (eq45_e1323_d_b3 * ddt_scale);
        let eq45_e1324_d_b4: f64 = (eq45_e1323_d_b4 * ddt_scale);
        let eq45_e1324_d_b5: f64 = (eq45_e1323_d_b5 * ddt_scale);
        let eq45_e1324_d_b6: f64 = (eq45_e1323_d_b6 * ddt_scale);
        let eq45_value: f64 = eq45_e1324;
        let eq45_node_derivatives: [f64; 12] = [eq45_e1324_d_n0, eq45_e1324_d_n1, eq45_e1324_d_n2, eq45_e1324_d_n3, eq45_e1324_d_n4, eq45_e1324_d_n5, eq45_e1324_d_n6, eq45_e1324_d_n7, eq45_e1324_d_n8, eq45_e1324_d_n9, eq45_e1324_d_n10, eq45_e1324_d_n11];
        let eq45_branch_derivatives: [f64; 7] = [eq45_e1324_d_b0, eq45_e1324_d_b1, eq45_e1324_d_b2, eq45_e1324_d_b3, eq45_e1324_d_b4, eq45_e1324_d_b5, eq45_e1324_d_b6];
        stamper.stamp_current_dense_local(
            Some(11),
            Some(7),
            multiplicity * (eq45_value),
            &eq45_node_derivatives,
            &eq45_branch_derivatives,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq47_e1332: f64 = ((nv4 - 0.0) / s.v[853]);
        let eq47_e1332_d_n0: f64 = (-(((nv4 - 0.0) * s.dn[853][0]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n1: f64 = (-(((nv4 - 0.0) * s.dn[853][1]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n2: f64 = (-(((nv4 - 0.0) * s.dn[853][2]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n3: f64 = (-(((nv4 - 0.0) * s.dn[853][3]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n4: f64 = ((s.v[853] - ((nv4 - 0.0) * s.dn[853][4])) / (s.v[853] * s.v[853]));
        let eq47_e1332_d_n5: f64 = (-(((nv4 - 0.0) * s.dn[853][5]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n6: f64 = (-(((nv4 - 0.0) * s.dn[853][6]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n7: f64 = (-(((nv4 - 0.0) * s.dn[853][7]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n8: f64 = (-(((nv4 - 0.0) * s.dn[853][8]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n9: f64 = (-(((nv4 - 0.0) * s.dn[853][9]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n10: f64 = (-(((nv4 - 0.0) * s.dn[853][10]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_n11: f64 = (-(((nv4 - 0.0) * s.dn[853][11]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_b0: f64 = (-(((nv4 - 0.0) * s.db[853][0]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_b1: f64 = (-(((nv4 - 0.0) * s.db[853][1]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_b2: f64 = (-(((nv4 - 0.0) * s.db[853][2]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_b3: f64 = (-(((nv4 - 0.0) * s.db[853][3]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_b4: f64 = (-(((nv4 - 0.0) * s.db[853][4]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_b5: f64 = (-(((nv4 - 0.0) * s.db[853][5]) / (s.v[853] * s.v[853])));
        let eq47_e1332_d_b6: f64 = (-(((nv4 - 0.0) * s.db[853][6]) / (s.v[853] * s.v[853])));
        let eq47_value: f64 = eq47_e1332;
        let eq47_node_derivatives: [f64; 12] = [eq47_e1332_d_n0, eq47_e1332_d_n1, eq47_e1332_d_n2, eq47_e1332_d_n3, eq47_e1332_d_n4, eq47_e1332_d_n5, eq47_e1332_d_n6, eq47_e1332_d_n7, eq47_e1332_d_n8, eq47_e1332_d_n9, eq47_e1332_d_n10, eq47_e1332_d_n11];
        let eq47_branch_derivatives: [f64; 7] = [eq47_e1332_d_b0, eq47_e1332_d_b1, eq47_e1332_d_b2, eq47_e1332_d_b3, eq47_e1332_d_b4, eq47_e1332_d_b5, eq47_e1332_d_b6];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq47_value),
            &eq47_node_derivatives,
            &eq47_branch_derivatives,
            multiplicity,
        );
        let eq48_e1335: f64 = (s.v[854] * (nv4 - 0.0));
        let eq48_e1335_d_n0: f64 = (s.dn[854][0] * (nv4 - 0.0));
        let eq48_e1335_d_n1: f64 = (s.dn[854][1] * (nv4 - 0.0));
        let eq48_e1335_d_n2: f64 = (s.dn[854][2] * (nv4 - 0.0));
        let eq48_e1335_d_n3: f64 = (s.dn[854][3] * (nv4 - 0.0));
        let eq48_e1335_d_n4: f64 = ((s.dn[854][4] * (nv4 - 0.0)) + s.v[854]);
        let eq48_e1335_d_n5: f64 = (s.dn[854][5] * (nv4 - 0.0));
        let eq48_e1335_d_n6: f64 = (s.dn[854][6] * (nv4 - 0.0));
        let eq48_e1335_d_n7: f64 = (s.dn[854][7] * (nv4 - 0.0));
        let eq48_e1335_d_n8: f64 = (s.dn[854][8] * (nv4 - 0.0));
        let eq48_e1335_d_n9: f64 = (s.dn[854][9] * (nv4 - 0.0));
        let eq48_e1335_d_n10: f64 = (s.dn[854][10] * (nv4 - 0.0));
        let eq48_e1335_d_n11: f64 = (s.dn[854][11] * (nv4 - 0.0));
        let eq48_e1335_d_b0: f64 = (s.db[854][0] * (nv4 - 0.0));
        let eq48_e1335_d_b1: f64 = (s.db[854][1] * (nv4 - 0.0));
        let eq48_e1335_d_b2: f64 = (s.db[854][2] * (nv4 - 0.0));
        let eq48_e1335_d_b3: f64 = (s.db[854][3] * (nv4 - 0.0));
        let eq48_e1335_d_b4: f64 = (s.db[854][4] * (nv4 - 0.0));
        let eq48_e1335_d_b5: f64 = (s.db[854][5] * (nv4 - 0.0));
        let eq48_e1335_d_b6: f64 = (s.db[854][6] * (nv4 - 0.0));
        let eq48_e1336: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 8, eq48_e1335);
        let eq48_e1336_d_n0: f64 = (eq48_e1335_d_n0 * ddt_scale);
        let eq48_e1336_d_n1: f64 = (eq48_e1335_d_n1 * ddt_scale);
        let eq48_e1336_d_n2: f64 = (eq48_e1335_d_n2 * ddt_scale);
        let eq48_e1336_d_n3: f64 = (eq48_e1335_d_n3 * ddt_scale);
        let eq48_e1336_d_n4: f64 = (eq48_e1335_d_n4 * ddt_scale);
        let eq48_e1336_d_n5: f64 = (eq48_e1335_d_n5 * ddt_scale);
        let eq48_e1336_d_n6: f64 = (eq48_e1335_d_n6 * ddt_scale);
        let eq48_e1336_d_n7: f64 = (eq48_e1335_d_n7 * ddt_scale);
        let eq48_e1336_d_n8: f64 = (eq48_e1335_d_n8 * ddt_scale);
        let eq48_e1336_d_n9: f64 = (eq48_e1335_d_n9 * ddt_scale);
        let eq48_e1336_d_n10: f64 = (eq48_e1335_d_n10 * ddt_scale);
        let eq48_e1336_d_n11: f64 = (eq48_e1335_d_n11 * ddt_scale);
        let eq48_e1336_d_b0: f64 = (eq48_e1335_d_b0 * ddt_scale);
        let eq48_e1336_d_b1: f64 = (eq48_e1335_d_b1 * ddt_scale);
        let eq48_e1336_d_b2: f64 = (eq48_e1335_d_b2 * ddt_scale);
        let eq48_e1336_d_b3: f64 = (eq48_e1335_d_b3 * ddt_scale);
        let eq48_e1336_d_b4: f64 = (eq48_e1335_d_b4 * ddt_scale);
        let eq48_e1336_d_b5: f64 = (eq48_e1335_d_b5 * ddt_scale);
        let eq48_e1336_d_b6: f64 = (eq48_e1335_d_b6 * ddt_scale);
        let eq48_value: f64 = eq48_e1336;
        let eq48_node_derivatives: [f64; 12] = [eq48_e1336_d_n0, eq48_e1336_d_n1, eq48_e1336_d_n2, eq48_e1336_d_n3, eq48_e1336_d_n4, eq48_e1336_d_n5, eq48_e1336_d_n6, eq48_e1336_d_n7, eq48_e1336_d_n8, eq48_e1336_d_n9, eq48_e1336_d_n10, eq48_e1336_d_n11];
        let eq48_branch_derivatives: [f64; 7] = [eq48_e1336_d_b0, eq48_e1336_d_b1, eq48_e1336_d_b2, eq48_e1336_d_b3, eq48_e1336_d_b4, eq48_e1336_d_b5, eq48_e1336_d_b6];
        stamper.stamp_current_dense_local(
            Some(4),
            None,
            multiplicity * (eq48_value),
            &eq48_node_derivatives,
            &eq48_branch_derivatives,
            multiplicity,
        );
        let eq49_e1339: f64 = (s.v[15] * p.p32);
        let eq49_e1339_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq49_e1339_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq49_e1339_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq49_e1339_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq49_e1339_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq49_e1339_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq49_e1339_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq49_e1339_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq49_e1339_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq49_e1339_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq49_e1339_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq49_e1339_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq49_e1339_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq49_e1339_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq49_e1339_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq49_e1339_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq49_e1339_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq49_e1339_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq49_e1339_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq49_e1340: f64 = (eq49_e1339).sqrt();
        let eq49_e1340_d_n0: f64 = (eq49_e1339_d_n0 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n1: f64 = (eq49_e1339_d_n1 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n2: f64 = (eq49_e1339_d_n2 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n3: f64 = (eq49_e1339_d_n3 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n4: f64 = (eq49_e1339_d_n4 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n5: f64 = (eq49_e1339_d_n5 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n6: f64 = (eq49_e1339_d_n6 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n7: f64 = (eq49_e1339_d_n7 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n8: f64 = (eq49_e1339_d_n8 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n9: f64 = (eq49_e1339_d_n9 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n10: f64 = (eq49_e1339_d_n10 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n11: f64 = (eq49_e1339_d_n11 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b0: f64 = (eq49_e1339_d_b0 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b1: f64 = (eq49_e1339_d_b1 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b2: f64 = (eq49_e1339_d_b2 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b3: f64 = (eq49_e1339_d_b3 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b4: f64 = (eq49_e1339_d_b4 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b5: f64 = (eq49_e1339_d_b5 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b6: f64 = (eq49_e1339_d_b6 / (2.0 * eq49_e1340));
        let eq49_e1342: f64 = (eq49_e1340 * 0.5);
        let eq49_e1342_d_n0: f64 = (eq49_e1340_d_n0 * 0.5);
        let eq49_e1342_d_n1: f64 = (eq49_e1340_d_n1 * 0.5);
        let eq49_e1342_d_n2: f64 = (eq49_e1340_d_n2 * 0.5);
        let eq49_e1342_d_n3: f64 = (eq49_e1340_d_n3 * 0.5);
        let eq49_e1342_d_n4: f64 = (eq49_e1340_d_n4 * 0.5);
        let eq49_e1342_d_n5: f64 = (eq49_e1340_d_n5 * 0.5);
        let eq49_e1342_d_n6: f64 = (eq49_e1340_d_n6 * 0.5);
        let eq49_e1342_d_n7: f64 = (eq49_e1340_d_n7 * 0.5);
        let eq49_e1342_d_n8: f64 = (eq49_e1340_d_n8 * 0.5);
        let eq49_e1342_d_n9: f64 = (eq49_e1340_d_n9 * 0.5);
        let eq49_e1342_d_n10: f64 = (eq49_e1340_d_n10 * 0.5);
        let eq49_e1342_d_n11: f64 = (eq49_e1340_d_n11 * 0.5);
        let eq49_e1342_d_b0: f64 = (eq49_e1340_d_b0 * 0.5);
        let eq49_e1342_d_b1: f64 = (eq49_e1340_d_b1 * 0.5);
        let eq49_e1342_d_b2: f64 = (eq49_e1340_d_b2 * 0.5);
        let eq49_e1342_d_b3: f64 = (eq49_e1340_d_b3 * 0.5);
        let eq49_e1342_d_b4: f64 = (eq49_e1340_d_b4 * 0.5);
        let eq49_e1342_d_b5: f64 = (eq49_e1340_d_b5 * 0.5);
        let eq49_e1342_d_b6: f64 = (eq49_e1340_d_b6 * 0.5);
        let eq49_e1344: f64 = (eq49_e1342 * s.v[854]);
        let eq49_e1344_d_n0: f64 = ((eq49_e1342_d_n0 * s.v[854]) + (eq49_e1342 * s.dn[854][0]));
        let eq49_e1344_d_n1: f64 = ((eq49_e1342_d_n1 * s.v[854]) + (eq49_e1342 * s.dn[854][1]));
        let eq49_e1344_d_n2: f64 = ((eq49_e1342_d_n2 * s.v[854]) + (eq49_e1342 * s.dn[854][2]));
        let eq49_e1344_d_n3: f64 = ((eq49_e1342_d_n3 * s.v[854]) + (eq49_e1342 * s.dn[854][3]));
        let eq49_e1344_d_n4: f64 = ((eq49_e1342_d_n4 * s.v[854]) + (eq49_e1342 * s.dn[854][4]));
        let eq49_e1344_d_n5: f64 = ((eq49_e1342_d_n5 * s.v[854]) + (eq49_e1342 * s.dn[854][5]));
        let eq49_e1344_d_n6: f64 = ((eq49_e1342_d_n6 * s.v[854]) + (eq49_e1342 * s.dn[854][6]));
        let eq49_e1344_d_n7: f64 = ((eq49_e1342_d_n7 * s.v[854]) + (eq49_e1342 * s.dn[854][7]));
        let eq49_e1344_d_n8: f64 = ((eq49_e1342_d_n8 * s.v[854]) + (eq49_e1342 * s.dn[854][8]));
        let eq49_e1344_d_n9: f64 = ((eq49_e1342_d_n9 * s.v[854]) + (eq49_e1342 * s.dn[854][9]));
        let eq49_e1344_d_n10: f64 = ((eq49_e1342_d_n10 * s.v[854]) + (eq49_e1342 * s.dn[854][10]));
        let eq49_e1344_d_n11: f64 = ((eq49_e1342_d_n11 * s.v[854]) + (eq49_e1342 * s.dn[854][11]));
        let eq49_e1344_d_b0: f64 = ((eq49_e1342_d_b0 * s.v[854]) + (eq49_e1342 * s.db[854][0]));
        let eq49_e1344_d_b1: f64 = ((eq49_e1342_d_b1 * s.v[854]) + (eq49_e1342 * s.db[854][1]));
        let eq49_e1344_d_b2: f64 = ((eq49_e1342_d_b2 * s.v[854]) + (eq49_e1342 * s.db[854][2]));
        let eq49_e1344_d_b3: f64 = ((eq49_e1342_d_b3 * s.v[854]) + (eq49_e1342 * s.db[854][3]));
        let eq49_e1344_d_b4: f64 = ((eq49_e1342_d_b4 * s.v[854]) + (eq49_e1342 * s.db[854][4]));
        let eq49_e1344_d_b5: f64 = ((eq49_e1342_d_b5 * s.v[854]) + (eq49_e1342 * s.db[854][5]));
        let eq49_e1344_d_b6: f64 = ((eq49_e1342_d_b6 * s.v[854]) + (eq49_e1342 * s.db[854][6]));
        let eq49_e1346: f64 = (eq49_e1344 * (nv4 - 0.0));
        let eq49_e1346_d_n0: f64 = (eq49_e1344_d_n0 * (nv4 - 0.0));
        let eq49_e1346_d_n1: f64 = (eq49_e1344_d_n1 * (nv4 - 0.0));
        let eq49_e1346_d_n2: f64 = (eq49_e1344_d_n2 * (nv4 - 0.0));
        let eq49_e1346_d_n3: f64 = (eq49_e1344_d_n3 * (nv4 - 0.0));
        let eq49_e1346_d_n4: f64 = ((eq49_e1344_d_n4 * (nv4 - 0.0)) + eq49_e1344);
        let eq49_e1346_d_n5: f64 = (eq49_e1344_d_n5 * (nv4 - 0.0));
        let eq49_e1346_d_n6: f64 = (eq49_e1344_d_n6 * (nv4 - 0.0));
        let eq49_e1346_d_n7: f64 = (eq49_e1344_d_n7 * (nv4 - 0.0));
        let eq49_e1346_d_n8: f64 = (eq49_e1344_d_n8 * (nv4 - 0.0));
        let eq49_e1346_d_n9: f64 = (eq49_e1344_d_n9 * (nv4 - 0.0));
        let eq49_e1346_d_n10: f64 = (eq49_e1344_d_n10 * (nv4 - 0.0));
        let eq49_e1346_d_n11: f64 = (eq49_e1344_d_n11 * (nv4 - 0.0));
        let eq49_e1346_d_b0: f64 = (eq49_e1344_d_b0 * (nv4 - 0.0));
        let eq49_e1346_d_b1: f64 = (eq49_e1344_d_b1 * (nv4 - 0.0));
        let eq49_e1346_d_b2: f64 = (eq49_e1344_d_b2 * (nv4 - 0.0));
        let eq49_e1346_d_b3: f64 = (eq49_e1344_d_b3 * (nv4 - 0.0));
        let eq49_e1346_d_b4: f64 = (eq49_e1344_d_b4 * (nv4 - 0.0));
        let eq49_e1346_d_b5: f64 = (eq49_e1344_d_b5 * (nv4 - 0.0));
        let eq49_e1346_d_b6: f64 = (eq49_e1344_d_b6 * (nv4 - 0.0));
        let eq49_e1347: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 9, eq49_e1346);
        let eq49_e1347_d_n0: f64 = (eq49_e1346_d_n0 * ddt_scale);
        let eq49_e1347_d_n1: f64 = (eq49_e1346_d_n1 * ddt_scale);
        let eq49_e1347_d_n2: f64 = (eq49_e1346_d_n2 * ddt_scale);
        let eq49_e1347_d_n3: f64 = (eq49_e1346_d_n3 * ddt_scale);
        let eq49_e1347_d_n4: f64 = (eq49_e1346_d_n4 * ddt_scale);
        let eq49_e1347_d_n5: f64 = (eq49_e1346_d_n5 * ddt_scale);
        let eq49_e1347_d_n6: f64 = (eq49_e1346_d_n6 * ddt_scale);
        let eq49_e1347_d_n7: f64 = (eq49_e1346_d_n7 * ddt_scale);
        let eq49_e1347_d_n8: f64 = (eq49_e1346_d_n8 * ddt_scale);
        let eq49_e1347_d_n9: f64 = (eq49_e1346_d_n9 * ddt_scale);
        let eq49_e1347_d_n10: f64 = (eq49_e1346_d_n10 * ddt_scale);
        let eq49_e1347_d_n11: f64 = (eq49_e1346_d_n11 * ddt_scale);
        let eq49_e1347_d_b0: f64 = (eq49_e1346_d_b0 * ddt_scale);
        let eq49_e1347_d_b1: f64 = (eq49_e1346_d_b1 * ddt_scale);
        let eq49_e1347_d_b2: f64 = (eq49_e1346_d_b2 * ddt_scale);
        let eq49_e1347_d_b3: f64 = (eq49_e1346_d_b3 * ddt_scale);
        let eq49_e1347_d_b4: f64 = (eq49_e1346_d_b4 * ddt_scale);
        let eq49_e1347_d_b5: f64 = (eq49_e1346_d_b5 * ddt_scale);
        let eq49_e1347_d_b6: f64 = (eq49_e1346_d_b6 * ddt_scale);
        let eq49_e1348: f64 = (-eq49_e1347);
        let eq49_e1348_d_n0: f64 = (-eq49_e1347_d_n0);
        let eq49_e1348_d_n1: f64 = (-eq49_e1347_d_n1);
        let eq49_e1348_d_n2: f64 = (-eq49_e1347_d_n2);
        let eq49_e1348_d_n3: f64 = (-eq49_e1347_d_n3);
        let eq49_e1348_d_n4: f64 = (-eq49_e1347_d_n4);
        let eq49_e1348_d_n5: f64 = (-eq49_e1347_d_n5);
        let eq49_e1348_d_n6: f64 = (-eq49_e1347_d_n6);
        let eq49_e1348_d_n7: f64 = (-eq49_e1347_d_n7);
        let eq49_e1348_d_n8: f64 = (-eq49_e1347_d_n8);
        let eq49_e1348_d_n9: f64 = (-eq49_e1347_d_n9);
        let eq49_e1348_d_n10: f64 = (-eq49_e1347_d_n10);
        let eq49_e1348_d_n11: f64 = (-eq49_e1347_d_n11);
        let eq49_e1348_d_b0: f64 = (-eq49_e1347_d_b0);
        let eq49_e1348_d_b1: f64 = (-eq49_e1347_d_b1);
        let eq49_e1348_d_b2: f64 = (-eq49_e1347_d_b2);
        let eq49_e1348_d_b3: f64 = (-eq49_e1347_d_b3);
        let eq49_e1348_d_b4: f64 = (-eq49_e1347_d_b4);
        let eq49_e1348_d_b5: f64 = (-eq49_e1347_d_b5);
        let eq49_e1348_d_b6: f64 = (-eq49_e1347_d_b6);
        let eq49_value: f64 = eq49_e1348;
        let eq49_node_derivatives: [f64; 12] = [eq49_e1348_d_n0, eq49_e1348_d_n1, eq49_e1348_d_n2, eq49_e1348_d_n3, eq49_e1348_d_n4, eq49_e1348_d_n5, eq49_e1348_d_n6, eq49_e1348_d_n7, eq49_e1348_d_n8, eq49_e1348_d_n9, eq49_e1348_d_n10, eq49_e1348_d_n11];
        let eq49_branch_derivatives: [f64; 7] = [eq49_e1348_d_b0, eq49_e1348_d_b1, eq49_e1348_d_b2, eq49_e1348_d_b3, eq49_e1348_d_b4, eq49_e1348_d_b5, eq49_e1348_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(6),
            multiplicity * (eq49_value),
            &eq49_node_derivatives,
            &eq49_branch_derivatives,
            multiplicity,
        );
        let eq50_e1351: f64 = (s.v[15] * p.p32);
        let eq50_e1351_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq50_e1351_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq50_e1351_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq50_e1351_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq50_e1351_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq50_e1351_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq50_e1351_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq50_e1351_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq50_e1351_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq50_e1351_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq50_e1351_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq50_e1351_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq50_e1351_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq50_e1351_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq50_e1351_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq50_e1351_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq50_e1351_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq50_e1351_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq50_e1351_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq50_e1352: f64 = (eq50_e1351).sqrt();
        let eq50_e1352_d_n0: f64 = (eq50_e1351_d_n0 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n1: f64 = (eq50_e1351_d_n1 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n2: f64 = (eq50_e1351_d_n2 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n3: f64 = (eq50_e1351_d_n3 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n4: f64 = (eq50_e1351_d_n4 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n5: f64 = (eq50_e1351_d_n5 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n6: f64 = (eq50_e1351_d_n6 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n7: f64 = (eq50_e1351_d_n7 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n8: f64 = (eq50_e1351_d_n8 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n9: f64 = (eq50_e1351_d_n9 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n10: f64 = (eq50_e1351_d_n10 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n11: f64 = (eq50_e1351_d_n11 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b0: f64 = (eq50_e1351_d_b0 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b1: f64 = (eq50_e1351_d_b1 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b2: f64 = (eq50_e1351_d_b2 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b3: f64 = (eq50_e1351_d_b3 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b4: f64 = (eq50_e1351_d_b4 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b5: f64 = (eq50_e1351_d_b5 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b6: f64 = (eq50_e1351_d_b6 / (2.0 * eq50_e1352));
        let eq50_e1354: f64 = (eq50_e1352 * 0.5);
        let eq50_e1354_d_n0: f64 = (eq50_e1352_d_n0 * 0.5);
        let eq50_e1354_d_n1: f64 = (eq50_e1352_d_n1 * 0.5);
        let eq50_e1354_d_n2: f64 = (eq50_e1352_d_n2 * 0.5);
        let eq50_e1354_d_n3: f64 = (eq50_e1352_d_n3 * 0.5);
        let eq50_e1354_d_n4: f64 = (eq50_e1352_d_n4 * 0.5);
        let eq50_e1354_d_n5: f64 = (eq50_e1352_d_n5 * 0.5);
        let eq50_e1354_d_n6: f64 = (eq50_e1352_d_n6 * 0.5);
        let eq50_e1354_d_n7: f64 = (eq50_e1352_d_n7 * 0.5);
        let eq50_e1354_d_n8: f64 = (eq50_e1352_d_n8 * 0.5);
        let eq50_e1354_d_n9: f64 = (eq50_e1352_d_n9 * 0.5);
        let eq50_e1354_d_n10: f64 = (eq50_e1352_d_n10 * 0.5);
        let eq50_e1354_d_n11: f64 = (eq50_e1352_d_n11 * 0.5);
        let eq50_e1354_d_b0: f64 = (eq50_e1352_d_b0 * 0.5);
        let eq50_e1354_d_b1: f64 = (eq50_e1352_d_b1 * 0.5);
        let eq50_e1354_d_b2: f64 = (eq50_e1352_d_b2 * 0.5);
        let eq50_e1354_d_b3: f64 = (eq50_e1352_d_b3 * 0.5);
        let eq50_e1354_d_b4: f64 = (eq50_e1352_d_b4 * 0.5);
        let eq50_e1354_d_b5: f64 = (eq50_e1352_d_b5 * 0.5);
        let eq50_e1354_d_b6: f64 = (eq50_e1352_d_b6 * 0.5);
        let eq50_e1356: f64 = (eq50_e1354 * s.v[854]);
        let eq50_e1356_d_n0: f64 = ((eq50_e1354_d_n0 * s.v[854]) + (eq50_e1354 * s.dn[854][0]));
        let eq50_e1356_d_n1: f64 = ((eq50_e1354_d_n1 * s.v[854]) + (eq50_e1354 * s.dn[854][1]));
        let eq50_e1356_d_n2: f64 = ((eq50_e1354_d_n2 * s.v[854]) + (eq50_e1354 * s.dn[854][2]));
        let eq50_e1356_d_n3: f64 = ((eq50_e1354_d_n3 * s.v[854]) + (eq50_e1354 * s.dn[854][3]));
        let eq50_e1356_d_n4: f64 = ((eq50_e1354_d_n4 * s.v[854]) + (eq50_e1354 * s.dn[854][4]));
        let eq50_e1356_d_n5: f64 = ((eq50_e1354_d_n5 * s.v[854]) + (eq50_e1354 * s.dn[854][5]));
        let eq50_e1356_d_n6: f64 = ((eq50_e1354_d_n6 * s.v[854]) + (eq50_e1354 * s.dn[854][6]));
        let eq50_e1356_d_n7: f64 = ((eq50_e1354_d_n7 * s.v[854]) + (eq50_e1354 * s.dn[854][7]));
        let eq50_e1356_d_n8: f64 = ((eq50_e1354_d_n8 * s.v[854]) + (eq50_e1354 * s.dn[854][8]));
        let eq50_e1356_d_n9: f64 = ((eq50_e1354_d_n9 * s.v[854]) + (eq50_e1354 * s.dn[854][9]));
        let eq50_e1356_d_n10: f64 = ((eq50_e1354_d_n10 * s.v[854]) + (eq50_e1354 * s.dn[854][10]));
        let eq50_e1356_d_n11: f64 = ((eq50_e1354_d_n11 * s.v[854]) + (eq50_e1354 * s.dn[854][11]));
        let eq50_e1356_d_b0: f64 = ((eq50_e1354_d_b0 * s.v[854]) + (eq50_e1354 * s.db[854][0]));
        let eq50_e1356_d_b1: f64 = ((eq50_e1354_d_b1 * s.v[854]) + (eq50_e1354 * s.db[854][1]));
        let eq50_e1356_d_b2: f64 = ((eq50_e1354_d_b2 * s.v[854]) + (eq50_e1354 * s.db[854][2]));
        let eq50_e1356_d_b3: f64 = ((eq50_e1354_d_b3 * s.v[854]) + (eq50_e1354 * s.db[854][3]));
        let eq50_e1356_d_b4: f64 = ((eq50_e1354_d_b4 * s.v[854]) + (eq50_e1354 * s.db[854][4]));
        let eq50_e1356_d_b5: f64 = ((eq50_e1354_d_b5 * s.v[854]) + (eq50_e1354 * s.db[854][5]));
        let eq50_e1356_d_b6: f64 = ((eq50_e1354_d_b6 * s.v[854]) + (eq50_e1354 * s.db[854][6]));
        let eq50_e1358: f64 = (eq50_e1356 * (nv4 - 0.0));
        let eq50_e1358_d_n0: f64 = (eq50_e1356_d_n0 * (nv4 - 0.0));
        let eq50_e1358_d_n1: f64 = (eq50_e1356_d_n1 * (nv4 - 0.0));
        let eq50_e1358_d_n2: f64 = (eq50_e1356_d_n2 * (nv4 - 0.0));
        let eq50_e1358_d_n3: f64 = (eq50_e1356_d_n3 * (nv4 - 0.0));
        let eq50_e1358_d_n4: f64 = ((eq50_e1356_d_n4 * (nv4 - 0.0)) + eq50_e1356);
        let eq50_e1358_d_n5: f64 = (eq50_e1356_d_n5 * (nv4 - 0.0));
        let eq50_e1358_d_n6: f64 = (eq50_e1356_d_n6 * (nv4 - 0.0));
        let eq50_e1358_d_n7: f64 = (eq50_e1356_d_n7 * (nv4 - 0.0));
        let eq50_e1358_d_n8: f64 = (eq50_e1356_d_n8 * (nv4 - 0.0));
        let eq50_e1358_d_n9: f64 = (eq50_e1356_d_n9 * (nv4 - 0.0));
        let eq50_e1358_d_n10: f64 = (eq50_e1356_d_n10 * (nv4 - 0.0));
        let eq50_e1358_d_n11: f64 = (eq50_e1356_d_n11 * (nv4 - 0.0));
        let eq50_e1358_d_b0: f64 = (eq50_e1356_d_b0 * (nv4 - 0.0));
        let eq50_e1358_d_b1: f64 = (eq50_e1356_d_b1 * (nv4 - 0.0));
        let eq50_e1358_d_b2: f64 = (eq50_e1356_d_b2 * (nv4 - 0.0));
        let eq50_e1358_d_b3: f64 = (eq50_e1356_d_b3 * (nv4 - 0.0));
        let eq50_e1358_d_b4: f64 = (eq50_e1356_d_b4 * (nv4 - 0.0));
        let eq50_e1358_d_b5: f64 = (eq50_e1356_d_b5 * (nv4 - 0.0));
        let eq50_e1358_d_b6: f64 = (eq50_e1356_d_b6 * (nv4 - 0.0));
        let eq50_e1359: f64 = eval_ddt(ddt_state_current, ddt_state_previous, ddt_state_initialized, ddt_active, ddt_scale, 10, eq50_e1358);
        let eq50_e1359_d_n0: f64 = (eq50_e1358_d_n0 * ddt_scale);
        let eq50_e1359_d_n1: f64 = (eq50_e1358_d_n1 * ddt_scale);
        let eq50_e1359_d_n2: f64 = (eq50_e1358_d_n2 * ddt_scale);
        let eq50_e1359_d_n3: f64 = (eq50_e1358_d_n3 * ddt_scale);
        let eq50_e1359_d_n4: f64 = (eq50_e1358_d_n4 * ddt_scale);
        let eq50_e1359_d_n5: f64 = (eq50_e1358_d_n5 * ddt_scale);
        let eq50_e1359_d_n6: f64 = (eq50_e1358_d_n6 * ddt_scale);
        let eq50_e1359_d_n7: f64 = (eq50_e1358_d_n7 * ddt_scale);
        let eq50_e1359_d_n8: f64 = (eq50_e1358_d_n8 * ddt_scale);
        let eq50_e1359_d_n9: f64 = (eq50_e1358_d_n9 * ddt_scale);
        let eq50_e1359_d_n10: f64 = (eq50_e1358_d_n10 * ddt_scale);
        let eq50_e1359_d_n11: f64 = (eq50_e1358_d_n11 * ddt_scale);
        let eq50_e1359_d_b0: f64 = (eq50_e1358_d_b0 * ddt_scale);
        let eq50_e1359_d_b1: f64 = (eq50_e1358_d_b1 * ddt_scale);
        let eq50_e1359_d_b2: f64 = (eq50_e1358_d_b2 * ddt_scale);
        let eq50_e1359_d_b3: f64 = (eq50_e1358_d_b3 * ddt_scale);
        let eq50_e1359_d_b4: f64 = (eq50_e1358_d_b4 * ddt_scale);
        let eq50_e1359_d_b5: f64 = (eq50_e1358_d_b5 * ddt_scale);
        let eq50_e1359_d_b6: f64 = (eq50_e1358_d_b6 * ddt_scale);
        let eq50_e1360: f64 = (-eq50_e1359);
        let eq50_e1360_d_n0: f64 = (-eq50_e1359_d_n0);
        let eq50_e1360_d_n1: f64 = (-eq50_e1359_d_n1);
        let eq50_e1360_d_n2: f64 = (-eq50_e1359_d_n2);
        let eq50_e1360_d_n3: f64 = (-eq50_e1359_d_n3);
        let eq50_e1360_d_n4: f64 = (-eq50_e1359_d_n4);
        let eq50_e1360_d_n5: f64 = (-eq50_e1359_d_n5);
        let eq50_e1360_d_n6: f64 = (-eq50_e1359_d_n6);
        let eq50_e1360_d_n7: f64 = (-eq50_e1359_d_n7);
        let eq50_e1360_d_n8: f64 = (-eq50_e1359_d_n8);
        let eq50_e1360_d_n9: f64 = (-eq50_e1359_d_n9);
        let eq50_e1360_d_n10: f64 = (-eq50_e1359_d_n10);
        let eq50_e1360_d_n11: f64 = (-eq50_e1359_d_n11);
        let eq50_e1360_d_b0: f64 = (-eq50_e1359_d_b0);
        let eq50_e1360_d_b1: f64 = (-eq50_e1359_d_b1);
        let eq50_e1360_d_b2: f64 = (-eq50_e1359_d_b2);
        let eq50_e1360_d_b3: f64 = (-eq50_e1359_d_b3);
        let eq50_e1360_d_b4: f64 = (-eq50_e1359_d_b4);
        let eq50_e1360_d_b5: f64 = (-eq50_e1359_d_b5);
        let eq50_e1360_d_b6: f64 = (-eq50_e1359_d_b6);
        let eq50_value: f64 = eq50_e1360;
        let eq50_node_derivatives: [f64; 12] = [eq50_e1360_d_n0, eq50_e1360_d_n1, eq50_e1360_d_n2, eq50_e1360_d_n3, eq50_e1360_d_n4, eq50_e1360_d_n5, eq50_e1360_d_n6, eq50_e1360_d_n7, eq50_e1360_d_n8, eq50_e1360_d_n9, eq50_e1360_d_n10, eq50_e1360_d_n11];
        let eq50_branch_derivatives: [f64; 7] = [eq50_e1360_d_b0, eq50_e1360_d_b1, eq50_e1360_d_b2, eq50_e1360_d_b3, eq50_e1360_d_b4, eq50_e1360_d_b5, eq50_e1360_d_b6];
        stamper.stamp_current_dense_local(
            Some(5),
            Some(7),
            multiplicity * (eq50_value),
            &eq50_node_derivatives,
            &eq50_branch_derivatives,
            multiplicity,
        );
    }

    pub(super) fn stamp_transient_equations_block_7(
        stamper: &mut GeneratedStamper<'_>,
        multiplicity: f64,
    ) {
        let eq52_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (eq52_value),
        );
        let eq53_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (eq53_value),
        );
        let eq54_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(6),
            multiplicity * (eq54_value),
        );
        let eq55_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(5),
            Some(7),
            multiplicity * (eq55_value),
        );
        let eq56_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(10),
            Some(6),
            multiplicity * (eq56_value),
        );
        let eq57_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(11),
            Some(7),
            multiplicity * (eq57_value),
        );
        let eq58_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (eq58_value),
        );
        let eq59_value: f64 = 0.0;
        stamper.stamp_current_const_local(
            Some(7),
            Some(6),
            multiplicity * (eq59_value),
        );
    }

    pub(super) fn stamp_reactive_equations_block_0(
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
        branches: &[usize; Instance::BRANCH_COUNT],
        multiplicity: f64,
    ) {
        let eq38_e1263: f64 = (s.v[0] * s.v[15]);
        let eq38_e1263_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq38_e1263_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq38_e1263_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq38_e1263_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq38_e1263_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq38_e1263_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq38_e1263_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq38_e1263_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq38_e1263_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq38_e1263_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq38_e1263_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq38_e1263_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq38_e1263_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq38_e1263_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq38_e1263_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq38_e1263_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq38_e1263_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq38_e1263_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq38_e1263_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq38_e1265: f64 = (eq38_e1263 * p.p33);
        let eq38_e1265_d_n0: f64 = (eq38_e1263_d_n0 * p.p33);
        let eq38_e1265_d_n1: f64 = (eq38_e1263_d_n1 * p.p33);
        let eq38_e1265_d_n2: f64 = (eq38_e1263_d_n2 * p.p33);
        let eq38_e1265_d_n3: f64 = (eq38_e1263_d_n3 * p.p33);
        let eq38_e1265_d_n4: f64 = (eq38_e1263_d_n4 * p.p33);
        let eq38_e1265_d_n5: f64 = (eq38_e1263_d_n5 * p.p33);
        let eq38_e1265_d_n6: f64 = (eq38_e1263_d_n6 * p.p33);
        let eq38_e1265_d_n7: f64 = (eq38_e1263_d_n7 * p.p33);
        let eq38_e1265_d_n8: f64 = (eq38_e1263_d_n8 * p.p33);
        let eq38_e1265_d_n9: f64 = (eq38_e1263_d_n9 * p.p33);
        let eq38_e1265_d_n10: f64 = (eq38_e1263_d_n10 * p.p33);
        let eq38_e1265_d_n11: f64 = (eq38_e1263_d_n11 * p.p33);
        let eq38_e1265_d_b0: f64 = (eq38_e1263_d_b0 * p.p33);
        let eq38_e1265_d_b1: f64 = (eq38_e1263_d_b1 * p.p33);
        let eq38_e1265_d_b2: f64 = (eq38_e1263_d_b2 * p.p33);
        let eq38_e1265_d_b3: f64 = (eq38_e1263_d_b3 * p.p33);
        let eq38_e1265_d_b4: f64 = (eq38_e1263_d_b4 * p.p33);
        let eq38_e1265_d_b5: f64 = (eq38_e1263_d_b5 * p.p33);
        let eq38_e1265_d_b6: f64 = (eq38_e1263_d_b6 * p.p33);
        let eq38_e1267: f64 = (eq38_e1265 * s.v[845]);
        let eq38_e1267_d_n0: f64 = ((eq38_e1265_d_n0 * s.v[845]) + (eq38_e1265 * s.dn[845][0]));
        let eq38_e1267_d_n1: f64 = ((eq38_e1265_d_n1 * s.v[845]) + (eq38_e1265 * s.dn[845][1]));
        let eq38_e1267_d_n2: f64 = ((eq38_e1265_d_n2 * s.v[845]) + (eq38_e1265 * s.dn[845][2]));
        let eq38_e1267_d_n3: f64 = ((eq38_e1265_d_n3 * s.v[845]) + (eq38_e1265 * s.dn[845][3]));
        let eq38_e1267_d_n4: f64 = ((eq38_e1265_d_n4 * s.v[845]) + (eq38_e1265 * s.dn[845][4]));
        let eq38_e1267_d_n5: f64 = ((eq38_e1265_d_n5 * s.v[845]) + (eq38_e1265 * s.dn[845][5]));
        let eq38_e1267_d_n6: f64 = ((eq38_e1265_d_n6 * s.v[845]) + (eq38_e1265 * s.dn[845][6]));
        let eq38_e1267_d_n7: f64 = ((eq38_e1265_d_n7 * s.v[845]) + (eq38_e1265 * s.dn[845][7]));
        let eq38_e1267_d_n8: f64 = ((eq38_e1265_d_n8 * s.v[845]) + (eq38_e1265 * s.dn[845][8]));
        let eq38_e1267_d_n9: f64 = ((eq38_e1265_d_n9 * s.v[845]) + (eq38_e1265 * s.dn[845][9]));
        let eq38_e1267_d_n10: f64 = ((eq38_e1265_d_n10 * s.v[845]) + (eq38_e1265 * s.dn[845][10]));
        let eq38_e1267_d_n11: f64 = ((eq38_e1265_d_n11 * s.v[845]) + (eq38_e1265 * s.dn[845][11]));
        let eq38_e1267_d_b0: f64 = ((eq38_e1265_d_b0 * s.v[845]) + (eq38_e1265 * s.db[845][0]));
        let eq38_e1267_d_b1: f64 = ((eq38_e1265_d_b1 * s.v[845]) + (eq38_e1265 * s.db[845][1]));
        let eq38_e1267_d_b2: f64 = ((eq38_e1265_d_b2 * s.v[845]) + (eq38_e1265 * s.db[845][2]));
        let eq38_e1267_d_b3: f64 = ((eq38_e1265_d_b3 * s.v[845]) + (eq38_e1265 * s.db[845][3]));
        let eq38_e1267_d_b4: f64 = ((eq38_e1265_d_b4 * s.v[845]) + (eq38_e1265 * s.db[845][4]));
        let eq38_e1267_d_b5: f64 = ((eq38_e1265_d_b5 * s.v[845]) + (eq38_e1265 * s.db[845][5]));
        let eq38_e1267_d_b6: f64 = ((eq38_e1265_d_b6 * s.v[845]) + (eq38_e1265 * s.db[845][6]));
        let eq38_e1268_q: f64 = eq38_e1267;
        let eq38_reactive_node_derivatives: [f64; 12] = [eq38_e1267_d_n0, eq38_e1267_d_n1, eq38_e1267_d_n2, eq38_e1267_d_n3, eq38_e1267_d_n4, eq38_e1267_d_n5, eq38_e1267_d_n6, eq38_e1267_d_n7, eq38_e1267_d_n8, eq38_e1267_d_n9, eq38_e1267_d_n10, eq38_e1267_d_n11];
        let eq38_reactive_branch_derivatives: [f64; 7] = [eq38_e1267_d_b0, eq38_e1267_d_b1, eq38_e1267_d_b2, eq38_e1267_d_b3, eq38_e1267_d_b4, eq38_e1267_d_b5, eq38_e1267_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq38_reactive_node_derivatives,
            branches,
            &eq38_reactive_branch_derivatives,
            multiplicity,
        );
        let eq39_e1271: f64 = (s.v[0] * s.v[15]);
        let eq39_e1271_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq39_e1271_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq39_e1271_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq39_e1271_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq39_e1271_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq39_e1271_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq39_e1271_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq39_e1271_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq39_e1271_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq39_e1271_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq39_e1271_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq39_e1271_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq39_e1271_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq39_e1271_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq39_e1271_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq39_e1271_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq39_e1271_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq39_e1271_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq39_e1271_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq39_e1273: f64 = (eq39_e1271 * p.p33);
        let eq39_e1273_d_n0: f64 = (eq39_e1271_d_n0 * p.p33);
        let eq39_e1273_d_n1: f64 = (eq39_e1271_d_n1 * p.p33);
        let eq39_e1273_d_n2: f64 = (eq39_e1271_d_n2 * p.p33);
        let eq39_e1273_d_n3: f64 = (eq39_e1271_d_n3 * p.p33);
        let eq39_e1273_d_n4: f64 = (eq39_e1271_d_n4 * p.p33);
        let eq39_e1273_d_n5: f64 = (eq39_e1271_d_n5 * p.p33);
        let eq39_e1273_d_n6: f64 = (eq39_e1271_d_n6 * p.p33);
        let eq39_e1273_d_n7: f64 = (eq39_e1271_d_n7 * p.p33);
        let eq39_e1273_d_n8: f64 = (eq39_e1271_d_n8 * p.p33);
        let eq39_e1273_d_n9: f64 = (eq39_e1271_d_n9 * p.p33);
        let eq39_e1273_d_n10: f64 = (eq39_e1271_d_n10 * p.p33);
        let eq39_e1273_d_n11: f64 = (eq39_e1271_d_n11 * p.p33);
        let eq39_e1273_d_b0: f64 = (eq39_e1271_d_b0 * p.p33);
        let eq39_e1273_d_b1: f64 = (eq39_e1271_d_b1 * p.p33);
        let eq39_e1273_d_b2: f64 = (eq39_e1271_d_b2 * p.p33);
        let eq39_e1273_d_b3: f64 = (eq39_e1271_d_b3 * p.p33);
        let eq39_e1273_d_b4: f64 = (eq39_e1271_d_b4 * p.p33);
        let eq39_e1273_d_b5: f64 = (eq39_e1271_d_b5 * p.p33);
        let eq39_e1273_d_b6: f64 = (eq39_e1271_d_b6 * p.p33);
        let eq39_e1275: f64 = (eq39_e1273 * s.v[846]);
        let eq39_e1275_d_n0: f64 = ((eq39_e1273_d_n0 * s.v[846]) + (eq39_e1273 * s.dn[846][0]));
        let eq39_e1275_d_n1: f64 = ((eq39_e1273_d_n1 * s.v[846]) + (eq39_e1273 * s.dn[846][1]));
        let eq39_e1275_d_n2: f64 = ((eq39_e1273_d_n2 * s.v[846]) + (eq39_e1273 * s.dn[846][2]));
        let eq39_e1275_d_n3: f64 = ((eq39_e1273_d_n3 * s.v[846]) + (eq39_e1273 * s.dn[846][3]));
        let eq39_e1275_d_n4: f64 = ((eq39_e1273_d_n4 * s.v[846]) + (eq39_e1273 * s.dn[846][4]));
        let eq39_e1275_d_n5: f64 = ((eq39_e1273_d_n5 * s.v[846]) + (eq39_e1273 * s.dn[846][5]));
        let eq39_e1275_d_n6: f64 = ((eq39_e1273_d_n6 * s.v[846]) + (eq39_e1273 * s.dn[846][6]));
        let eq39_e1275_d_n7: f64 = ((eq39_e1273_d_n7 * s.v[846]) + (eq39_e1273 * s.dn[846][7]));
        let eq39_e1275_d_n8: f64 = ((eq39_e1273_d_n8 * s.v[846]) + (eq39_e1273 * s.dn[846][8]));
        let eq39_e1275_d_n9: f64 = ((eq39_e1273_d_n9 * s.v[846]) + (eq39_e1273 * s.dn[846][9]));
        let eq39_e1275_d_n10: f64 = ((eq39_e1273_d_n10 * s.v[846]) + (eq39_e1273 * s.dn[846][10]));
        let eq39_e1275_d_n11: f64 = ((eq39_e1273_d_n11 * s.v[846]) + (eq39_e1273 * s.dn[846][11]));
        let eq39_e1275_d_b0: f64 = ((eq39_e1273_d_b0 * s.v[846]) + (eq39_e1273 * s.db[846][0]));
        let eq39_e1275_d_b1: f64 = ((eq39_e1273_d_b1 * s.v[846]) + (eq39_e1273 * s.db[846][1]));
        let eq39_e1275_d_b2: f64 = ((eq39_e1273_d_b2 * s.v[846]) + (eq39_e1273 * s.db[846][2]));
        let eq39_e1275_d_b3: f64 = ((eq39_e1273_d_b3 * s.v[846]) + (eq39_e1273 * s.db[846][3]));
        let eq39_e1275_d_b4: f64 = ((eq39_e1273_d_b4 * s.v[846]) + (eq39_e1273 * s.db[846][4]));
        let eq39_e1275_d_b5: f64 = ((eq39_e1273_d_b5 * s.v[846]) + (eq39_e1273 * s.db[846][5]));
        let eq39_e1275_d_b6: f64 = ((eq39_e1273_d_b6 * s.v[846]) + (eq39_e1273 * s.db[846][6]));
        let eq39_e1276_q: f64 = eq39_e1275;
        let eq39_reactive_node_derivatives: [f64; 12] = [eq39_e1275_d_n0, eq39_e1275_d_n1, eq39_e1275_d_n2, eq39_e1275_d_n3, eq39_e1275_d_n4, eq39_e1275_d_n5, eq39_e1275_d_n6, eq39_e1275_d_n7, eq39_e1275_d_n8, eq39_e1275_d_n9, eq39_e1275_d_n10, eq39_e1275_d_n11];
        let eq39_reactive_branch_derivatives: [f64; 7] = [eq39_e1275_d_b0, eq39_e1275_d_b1, eq39_e1275_d_b2, eq39_e1275_d_b3, eq39_e1275_d_b4, eq39_e1275_d_b5, eq39_e1275_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[8]),
            Some(nodes[6]),
            nodes,
            &eq39_reactive_node_derivatives,
            branches,
            &eq39_reactive_branch_derivatives,
            multiplicity,
        );
        let eq40_e1279: f64 = (s.v[0] * s.v[15]);
        let eq40_e1279_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq40_e1279_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq40_e1279_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq40_e1279_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq40_e1279_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq40_e1279_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq40_e1279_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq40_e1279_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq40_e1279_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq40_e1279_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq40_e1279_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq40_e1279_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq40_e1279_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq40_e1279_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq40_e1279_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq40_e1279_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq40_e1279_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq40_e1279_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq40_e1279_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq40_e1281: f64 = (eq40_e1279 * p.p33);
        let eq40_e1281_d_n0: f64 = (eq40_e1279_d_n0 * p.p33);
        let eq40_e1281_d_n1: f64 = (eq40_e1279_d_n1 * p.p33);
        let eq40_e1281_d_n2: f64 = (eq40_e1279_d_n2 * p.p33);
        let eq40_e1281_d_n3: f64 = (eq40_e1279_d_n3 * p.p33);
        let eq40_e1281_d_n4: f64 = (eq40_e1279_d_n4 * p.p33);
        let eq40_e1281_d_n5: f64 = (eq40_e1279_d_n5 * p.p33);
        let eq40_e1281_d_n6: f64 = (eq40_e1279_d_n6 * p.p33);
        let eq40_e1281_d_n7: f64 = (eq40_e1279_d_n7 * p.p33);
        let eq40_e1281_d_n8: f64 = (eq40_e1279_d_n8 * p.p33);
        let eq40_e1281_d_n9: f64 = (eq40_e1279_d_n9 * p.p33);
        let eq40_e1281_d_n10: f64 = (eq40_e1279_d_n10 * p.p33);
        let eq40_e1281_d_n11: f64 = (eq40_e1279_d_n11 * p.p33);
        let eq40_e1281_d_b0: f64 = (eq40_e1279_d_b0 * p.p33);
        let eq40_e1281_d_b1: f64 = (eq40_e1279_d_b1 * p.p33);
        let eq40_e1281_d_b2: f64 = (eq40_e1279_d_b2 * p.p33);
        let eq40_e1281_d_b3: f64 = (eq40_e1279_d_b3 * p.p33);
        let eq40_e1281_d_b4: f64 = (eq40_e1279_d_b4 * p.p33);
        let eq40_e1281_d_b5: f64 = (eq40_e1279_d_b5 * p.p33);
        let eq40_e1281_d_b6: f64 = (eq40_e1279_d_b6 * p.p33);
        let eq40_e1283: f64 = (eq40_e1281 * s.v[847]);
        let eq40_e1283_d_n0: f64 = ((eq40_e1281_d_n0 * s.v[847]) + (eq40_e1281 * s.dn[847][0]));
        let eq40_e1283_d_n1: f64 = ((eq40_e1281_d_n1 * s.v[847]) + (eq40_e1281 * s.dn[847][1]));
        let eq40_e1283_d_n2: f64 = ((eq40_e1281_d_n2 * s.v[847]) + (eq40_e1281 * s.dn[847][2]));
        let eq40_e1283_d_n3: f64 = ((eq40_e1281_d_n3 * s.v[847]) + (eq40_e1281 * s.dn[847][3]));
        let eq40_e1283_d_n4: f64 = ((eq40_e1281_d_n4 * s.v[847]) + (eq40_e1281 * s.dn[847][4]));
        let eq40_e1283_d_n5: f64 = ((eq40_e1281_d_n5 * s.v[847]) + (eq40_e1281 * s.dn[847][5]));
        let eq40_e1283_d_n6: f64 = ((eq40_e1281_d_n6 * s.v[847]) + (eq40_e1281 * s.dn[847][6]));
        let eq40_e1283_d_n7: f64 = ((eq40_e1281_d_n7 * s.v[847]) + (eq40_e1281 * s.dn[847][7]));
        let eq40_e1283_d_n8: f64 = ((eq40_e1281_d_n8 * s.v[847]) + (eq40_e1281 * s.dn[847][8]));
        let eq40_e1283_d_n9: f64 = ((eq40_e1281_d_n9 * s.v[847]) + (eq40_e1281 * s.dn[847][9]));
        let eq40_e1283_d_n10: f64 = ((eq40_e1281_d_n10 * s.v[847]) + (eq40_e1281 * s.dn[847][10]));
        let eq40_e1283_d_n11: f64 = ((eq40_e1281_d_n11 * s.v[847]) + (eq40_e1281 * s.dn[847][11]));
        let eq40_e1283_d_b0: f64 = ((eq40_e1281_d_b0 * s.v[847]) + (eq40_e1281 * s.db[847][0]));
        let eq40_e1283_d_b1: f64 = ((eq40_e1281_d_b1 * s.v[847]) + (eq40_e1281 * s.db[847][1]));
        let eq40_e1283_d_b2: f64 = ((eq40_e1281_d_b2 * s.v[847]) + (eq40_e1281 * s.db[847][2]));
        let eq40_e1283_d_b3: f64 = ((eq40_e1281_d_b3 * s.v[847]) + (eq40_e1281 * s.db[847][3]));
        let eq40_e1283_d_b4: f64 = ((eq40_e1281_d_b4 * s.v[847]) + (eq40_e1281 * s.db[847][4]));
        let eq40_e1283_d_b5: f64 = ((eq40_e1281_d_b5 * s.v[847]) + (eq40_e1281 * s.db[847][5]));
        let eq40_e1283_d_b6: f64 = ((eq40_e1281_d_b6 * s.v[847]) + (eq40_e1281 * s.db[847][6]));
        let eq40_e1284_q: f64 = eq40_e1283;
        let eq40_reactive_node_derivatives: [f64; 12] = [eq40_e1283_d_n0, eq40_e1283_d_n1, eq40_e1283_d_n2, eq40_e1283_d_n3, eq40_e1283_d_n4, eq40_e1283_d_n5, eq40_e1283_d_n6, eq40_e1283_d_n7, eq40_e1283_d_n8, eq40_e1283_d_n9, eq40_e1283_d_n10, eq40_e1283_d_n11];
        let eq40_reactive_branch_derivatives: [f64; 7] = [eq40_e1283_d_b0, eq40_e1283_d_b1, eq40_e1283_d_b2, eq40_e1283_d_b3, eq40_e1283_d_b4, eq40_e1283_d_b5, eq40_e1283_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[7]),
            Some(nodes[6]),
            nodes,
            &eq40_reactive_node_derivatives,
            branches,
            &eq40_reactive_branch_derivatives,
            multiplicity,
        );
        let eq41_e1287: f64 = (s.v[0] * s.v[15]);
        let eq41_e1287_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq41_e1287_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq41_e1287_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq41_e1287_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq41_e1287_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq41_e1287_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq41_e1287_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq41_e1287_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq41_e1287_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq41_e1287_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq41_e1287_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq41_e1287_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq41_e1287_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq41_e1287_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq41_e1287_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq41_e1287_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq41_e1287_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq41_e1287_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq41_e1287_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq41_e1289: f64 = (eq41_e1287 * p.p33);
        let eq41_e1289_d_n0: f64 = (eq41_e1287_d_n0 * p.p33);
        let eq41_e1289_d_n1: f64 = (eq41_e1287_d_n1 * p.p33);
        let eq41_e1289_d_n2: f64 = (eq41_e1287_d_n2 * p.p33);
        let eq41_e1289_d_n3: f64 = (eq41_e1287_d_n3 * p.p33);
        let eq41_e1289_d_n4: f64 = (eq41_e1287_d_n4 * p.p33);
        let eq41_e1289_d_n5: f64 = (eq41_e1287_d_n5 * p.p33);
        let eq41_e1289_d_n6: f64 = (eq41_e1287_d_n6 * p.p33);
        let eq41_e1289_d_n7: f64 = (eq41_e1287_d_n7 * p.p33);
        let eq41_e1289_d_n8: f64 = (eq41_e1287_d_n8 * p.p33);
        let eq41_e1289_d_n9: f64 = (eq41_e1287_d_n9 * p.p33);
        let eq41_e1289_d_n10: f64 = (eq41_e1287_d_n10 * p.p33);
        let eq41_e1289_d_n11: f64 = (eq41_e1287_d_n11 * p.p33);
        let eq41_e1289_d_b0: f64 = (eq41_e1287_d_b0 * p.p33);
        let eq41_e1289_d_b1: f64 = (eq41_e1287_d_b1 * p.p33);
        let eq41_e1289_d_b2: f64 = (eq41_e1287_d_b2 * p.p33);
        let eq41_e1289_d_b3: f64 = (eq41_e1287_d_b3 * p.p33);
        let eq41_e1289_d_b4: f64 = (eq41_e1287_d_b4 * p.p33);
        let eq41_e1289_d_b5: f64 = (eq41_e1287_d_b5 * p.p33);
        let eq41_e1289_d_b6: f64 = (eq41_e1287_d_b6 * p.p33);
        let eq41_e1291: f64 = (eq41_e1289 * s.v[848]);
        let eq41_e1291_d_n0: f64 = ((eq41_e1289_d_n0 * s.v[848]) + (eq41_e1289 * s.dn[848][0]));
        let eq41_e1291_d_n1: f64 = ((eq41_e1289_d_n1 * s.v[848]) + (eq41_e1289 * s.dn[848][1]));
        let eq41_e1291_d_n2: f64 = ((eq41_e1289_d_n2 * s.v[848]) + (eq41_e1289 * s.dn[848][2]));
        let eq41_e1291_d_n3: f64 = ((eq41_e1289_d_n3 * s.v[848]) + (eq41_e1289 * s.dn[848][3]));
        let eq41_e1291_d_n4: f64 = ((eq41_e1289_d_n4 * s.v[848]) + (eq41_e1289 * s.dn[848][4]));
        let eq41_e1291_d_n5: f64 = ((eq41_e1289_d_n5 * s.v[848]) + (eq41_e1289 * s.dn[848][5]));
        let eq41_e1291_d_n6: f64 = ((eq41_e1289_d_n6 * s.v[848]) + (eq41_e1289 * s.dn[848][6]));
        let eq41_e1291_d_n7: f64 = ((eq41_e1289_d_n7 * s.v[848]) + (eq41_e1289 * s.dn[848][7]));
        let eq41_e1291_d_n8: f64 = ((eq41_e1289_d_n8 * s.v[848]) + (eq41_e1289 * s.dn[848][8]));
        let eq41_e1291_d_n9: f64 = ((eq41_e1289_d_n9 * s.v[848]) + (eq41_e1289 * s.dn[848][9]));
        let eq41_e1291_d_n10: f64 = ((eq41_e1289_d_n10 * s.v[848]) + (eq41_e1289 * s.dn[848][10]));
        let eq41_e1291_d_n11: f64 = ((eq41_e1289_d_n11 * s.v[848]) + (eq41_e1289 * s.dn[848][11]));
        let eq41_e1291_d_b0: f64 = ((eq41_e1289_d_b0 * s.v[848]) + (eq41_e1289 * s.db[848][0]));
        let eq41_e1291_d_b1: f64 = ((eq41_e1289_d_b1 * s.v[848]) + (eq41_e1289 * s.db[848][1]));
        let eq41_e1291_d_b2: f64 = ((eq41_e1289_d_b2 * s.v[848]) + (eq41_e1289 * s.db[848][2]));
        let eq41_e1291_d_b3: f64 = ((eq41_e1289_d_b3 * s.v[848]) + (eq41_e1289 * s.db[848][3]));
        let eq41_e1291_d_b4: f64 = ((eq41_e1289_d_b4 * s.v[848]) + (eq41_e1289 * s.db[848][4]));
        let eq41_e1291_d_b5: f64 = ((eq41_e1289_d_b5 * s.v[848]) + (eq41_e1289 * s.db[848][5]));
        let eq41_e1291_d_b6: f64 = ((eq41_e1289_d_b6 * s.v[848]) + (eq41_e1289 * s.db[848][6]));
        let eq41_e1292_q: f64 = eq41_e1291;
        let eq41_reactive_node_derivatives: [f64; 12] = [eq41_e1291_d_n0, eq41_e1291_d_n1, eq41_e1291_d_n2, eq41_e1291_d_n3, eq41_e1291_d_n4, eq41_e1291_d_n5, eq41_e1291_d_n6, eq41_e1291_d_n7, eq41_e1291_d_n8, eq41_e1291_d_n9, eq41_e1291_d_n10, eq41_e1291_d_n11];
        let eq41_reactive_branch_derivatives: [f64; 7] = [eq41_e1291_d_b0, eq41_e1291_d_b1, eq41_e1291_d_b2, eq41_e1291_d_b3, eq41_e1291_d_b4, eq41_e1291_d_b5, eq41_e1291_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq41_reactive_node_derivatives,
            branches,
            &eq41_reactive_branch_derivatives,
            multiplicity,
        );
        let eq42_e1295: f64 = (s.v[0] * s.v[15]);
        let eq42_e1295_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq42_e1295_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq42_e1295_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq42_e1295_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq42_e1295_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq42_e1295_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq42_e1295_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq42_e1295_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq42_e1295_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq42_e1295_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq42_e1295_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq42_e1295_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq42_e1295_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq42_e1295_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq42_e1295_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq42_e1295_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq42_e1295_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq42_e1295_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq42_e1295_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq42_e1297: f64 = (eq42_e1295 * p.p33);
        let eq42_e1297_d_n0: f64 = (eq42_e1295_d_n0 * p.p33);
        let eq42_e1297_d_n1: f64 = (eq42_e1295_d_n1 * p.p33);
        let eq42_e1297_d_n2: f64 = (eq42_e1295_d_n2 * p.p33);
        let eq42_e1297_d_n3: f64 = (eq42_e1295_d_n3 * p.p33);
        let eq42_e1297_d_n4: f64 = (eq42_e1295_d_n4 * p.p33);
        let eq42_e1297_d_n5: f64 = (eq42_e1295_d_n5 * p.p33);
        let eq42_e1297_d_n6: f64 = (eq42_e1295_d_n6 * p.p33);
        let eq42_e1297_d_n7: f64 = (eq42_e1295_d_n7 * p.p33);
        let eq42_e1297_d_n8: f64 = (eq42_e1295_d_n8 * p.p33);
        let eq42_e1297_d_n9: f64 = (eq42_e1295_d_n9 * p.p33);
        let eq42_e1297_d_n10: f64 = (eq42_e1295_d_n10 * p.p33);
        let eq42_e1297_d_n11: f64 = (eq42_e1295_d_n11 * p.p33);
        let eq42_e1297_d_b0: f64 = (eq42_e1295_d_b0 * p.p33);
        let eq42_e1297_d_b1: f64 = (eq42_e1295_d_b1 * p.p33);
        let eq42_e1297_d_b2: f64 = (eq42_e1295_d_b2 * p.p33);
        let eq42_e1297_d_b3: f64 = (eq42_e1295_d_b3 * p.p33);
        let eq42_e1297_d_b4: f64 = (eq42_e1295_d_b4 * p.p33);
        let eq42_e1297_d_b5: f64 = (eq42_e1295_d_b5 * p.p33);
        let eq42_e1297_d_b6: f64 = (eq42_e1295_d_b6 * p.p33);
        let eq42_e1299: f64 = (eq42_e1297 * s.v[849]);
        let eq42_e1299_d_n0: f64 = ((eq42_e1297_d_n0 * s.v[849]) + (eq42_e1297 * s.dn[849][0]));
        let eq42_e1299_d_n1: f64 = ((eq42_e1297_d_n1 * s.v[849]) + (eq42_e1297 * s.dn[849][1]));
        let eq42_e1299_d_n2: f64 = ((eq42_e1297_d_n2 * s.v[849]) + (eq42_e1297 * s.dn[849][2]));
        let eq42_e1299_d_n3: f64 = ((eq42_e1297_d_n3 * s.v[849]) + (eq42_e1297 * s.dn[849][3]));
        let eq42_e1299_d_n4: f64 = ((eq42_e1297_d_n4 * s.v[849]) + (eq42_e1297 * s.dn[849][4]));
        let eq42_e1299_d_n5: f64 = ((eq42_e1297_d_n5 * s.v[849]) + (eq42_e1297 * s.dn[849][5]));
        let eq42_e1299_d_n6: f64 = ((eq42_e1297_d_n6 * s.v[849]) + (eq42_e1297 * s.dn[849][6]));
        let eq42_e1299_d_n7: f64 = ((eq42_e1297_d_n7 * s.v[849]) + (eq42_e1297 * s.dn[849][7]));
        let eq42_e1299_d_n8: f64 = ((eq42_e1297_d_n8 * s.v[849]) + (eq42_e1297 * s.dn[849][8]));
        let eq42_e1299_d_n9: f64 = ((eq42_e1297_d_n9 * s.v[849]) + (eq42_e1297 * s.dn[849][9]));
        let eq42_e1299_d_n10: f64 = ((eq42_e1297_d_n10 * s.v[849]) + (eq42_e1297 * s.dn[849][10]));
        let eq42_e1299_d_n11: f64 = ((eq42_e1297_d_n11 * s.v[849]) + (eq42_e1297 * s.dn[849][11]));
        let eq42_e1299_d_b0: f64 = ((eq42_e1297_d_b0 * s.v[849]) + (eq42_e1297 * s.db[849][0]));
        let eq42_e1299_d_b1: f64 = ((eq42_e1297_d_b1 * s.v[849]) + (eq42_e1297 * s.db[849][1]));
        let eq42_e1299_d_b2: f64 = ((eq42_e1297_d_b2 * s.v[849]) + (eq42_e1297 * s.db[849][2]));
        let eq42_e1299_d_b3: f64 = ((eq42_e1297_d_b3 * s.v[849]) + (eq42_e1297 * s.db[849][3]));
        let eq42_e1299_d_b4: f64 = ((eq42_e1297_d_b4 * s.v[849]) + (eq42_e1297 * s.db[849][4]));
        let eq42_e1299_d_b5: f64 = ((eq42_e1297_d_b5 * s.v[849]) + (eq42_e1297 * s.db[849][5]));
        let eq42_e1299_d_b6: f64 = ((eq42_e1297_d_b6 * s.v[849]) + (eq42_e1297 * s.db[849][6]));
        let eq42_e1300_q: f64 = eq42_e1299;
        let eq42_reactive_node_derivatives: [f64; 12] = [eq42_e1299_d_n0, eq42_e1299_d_n1, eq42_e1299_d_n2, eq42_e1299_d_n3, eq42_e1299_d_n4, eq42_e1299_d_n5, eq42_e1299_d_n6, eq42_e1299_d_n7, eq42_e1299_d_n8, eq42_e1299_d_n9, eq42_e1299_d_n10, eq42_e1299_d_n11];
        let eq42_reactive_branch_derivatives: [f64; 7] = [eq42_e1299_d_b0, eq42_e1299_d_b1, eq42_e1299_d_b2, eq42_e1299_d_b3, eq42_e1299_d_b4, eq42_e1299_d_b5, eq42_e1299_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq42_reactive_node_derivatives,
            branches,
            &eq42_reactive_branch_derivatives,
            multiplicity,
        );
        let eq43_e1303: f64 = (s.v[0] * s.v[15]);
        let eq43_e1303_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq43_e1303_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq43_e1303_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq43_e1303_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq43_e1303_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq43_e1303_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq43_e1303_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq43_e1303_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq43_e1303_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq43_e1303_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq43_e1303_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq43_e1303_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq43_e1303_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq43_e1303_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq43_e1303_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq43_e1303_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq43_e1303_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq43_e1303_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq43_e1303_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq43_e1305: f64 = (eq43_e1303 * p.p33);
        let eq43_e1305_d_n0: f64 = (eq43_e1303_d_n0 * p.p33);
        let eq43_e1305_d_n1: f64 = (eq43_e1303_d_n1 * p.p33);
        let eq43_e1305_d_n2: f64 = (eq43_e1303_d_n2 * p.p33);
        let eq43_e1305_d_n3: f64 = (eq43_e1303_d_n3 * p.p33);
        let eq43_e1305_d_n4: f64 = (eq43_e1303_d_n4 * p.p33);
        let eq43_e1305_d_n5: f64 = (eq43_e1303_d_n5 * p.p33);
        let eq43_e1305_d_n6: f64 = (eq43_e1303_d_n6 * p.p33);
        let eq43_e1305_d_n7: f64 = (eq43_e1303_d_n7 * p.p33);
        let eq43_e1305_d_n8: f64 = (eq43_e1303_d_n8 * p.p33);
        let eq43_e1305_d_n9: f64 = (eq43_e1303_d_n9 * p.p33);
        let eq43_e1305_d_n10: f64 = (eq43_e1303_d_n10 * p.p33);
        let eq43_e1305_d_n11: f64 = (eq43_e1303_d_n11 * p.p33);
        let eq43_e1305_d_b0: f64 = (eq43_e1303_d_b0 * p.p33);
        let eq43_e1305_d_b1: f64 = (eq43_e1303_d_b1 * p.p33);
        let eq43_e1305_d_b2: f64 = (eq43_e1303_d_b2 * p.p33);
        let eq43_e1305_d_b3: f64 = (eq43_e1303_d_b3 * p.p33);
        let eq43_e1305_d_b4: f64 = (eq43_e1303_d_b4 * p.p33);
        let eq43_e1305_d_b5: f64 = (eq43_e1303_d_b5 * p.p33);
        let eq43_e1305_d_b6: f64 = (eq43_e1303_d_b6 * p.p33);
        let eq43_e1307: f64 = (eq43_e1305 * s.v[850]);
        let eq43_e1307_d_n0: f64 = ((eq43_e1305_d_n0 * s.v[850]) + (eq43_e1305 * s.dn[850][0]));
        let eq43_e1307_d_n1: f64 = ((eq43_e1305_d_n1 * s.v[850]) + (eq43_e1305 * s.dn[850][1]));
        let eq43_e1307_d_n2: f64 = ((eq43_e1305_d_n2 * s.v[850]) + (eq43_e1305 * s.dn[850][2]));
        let eq43_e1307_d_n3: f64 = ((eq43_e1305_d_n3 * s.v[850]) + (eq43_e1305 * s.dn[850][3]));
        let eq43_e1307_d_n4: f64 = ((eq43_e1305_d_n4 * s.v[850]) + (eq43_e1305 * s.dn[850][4]));
        let eq43_e1307_d_n5: f64 = ((eq43_e1305_d_n5 * s.v[850]) + (eq43_e1305 * s.dn[850][5]));
        let eq43_e1307_d_n6: f64 = ((eq43_e1305_d_n6 * s.v[850]) + (eq43_e1305 * s.dn[850][6]));
        let eq43_e1307_d_n7: f64 = ((eq43_e1305_d_n7 * s.v[850]) + (eq43_e1305 * s.dn[850][7]));
        let eq43_e1307_d_n8: f64 = ((eq43_e1305_d_n8 * s.v[850]) + (eq43_e1305 * s.dn[850][8]));
        let eq43_e1307_d_n9: f64 = ((eq43_e1305_d_n9 * s.v[850]) + (eq43_e1305 * s.dn[850][9]));
        let eq43_e1307_d_n10: f64 = ((eq43_e1305_d_n10 * s.v[850]) + (eq43_e1305 * s.dn[850][10]));
        let eq43_e1307_d_n11: f64 = ((eq43_e1305_d_n11 * s.v[850]) + (eq43_e1305 * s.dn[850][11]));
        let eq43_e1307_d_b0: f64 = ((eq43_e1305_d_b0 * s.v[850]) + (eq43_e1305 * s.db[850][0]));
        let eq43_e1307_d_b1: f64 = ((eq43_e1305_d_b1 * s.v[850]) + (eq43_e1305 * s.db[850][1]));
        let eq43_e1307_d_b2: f64 = ((eq43_e1305_d_b2 * s.v[850]) + (eq43_e1305 * s.db[850][2]));
        let eq43_e1307_d_b3: f64 = ((eq43_e1305_d_b3 * s.v[850]) + (eq43_e1305 * s.db[850][3]));
        let eq43_e1307_d_b4: f64 = ((eq43_e1305_d_b4 * s.v[850]) + (eq43_e1305 * s.db[850][4]));
        let eq43_e1307_d_b5: f64 = ((eq43_e1305_d_b5 * s.v[850]) + (eq43_e1305 * s.db[850][5]));
        let eq43_e1307_d_b6: f64 = ((eq43_e1305_d_b6 * s.v[850]) + (eq43_e1305 * s.db[850][6]));
        let eq43_e1308_q: f64 = eq43_e1307;
        let eq43_reactive_node_derivatives: [f64; 12] = [eq43_e1307_d_n0, eq43_e1307_d_n1, eq43_e1307_d_n2, eq43_e1307_d_n3, eq43_e1307_d_n4, eq43_e1307_d_n5, eq43_e1307_d_n6, eq43_e1307_d_n7, eq43_e1307_d_n8, eq43_e1307_d_n9, eq43_e1307_d_n10, eq43_e1307_d_n11];
        let eq43_reactive_branch_derivatives: [f64; 7] = [eq43_e1307_d_b0, eq43_e1307_d_b1, eq43_e1307_d_b2, eq43_e1307_d_b3, eq43_e1307_d_b4, eq43_e1307_d_b5, eq43_e1307_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[8]),
            nodes,
            &eq43_reactive_node_derivatives,
            branches,
            &eq43_reactive_branch_derivatives,
            multiplicity,
        );
        let eq44_e1311: f64 = (s.v[0] * s.v[15]);
        let eq44_e1311_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq44_e1311_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq44_e1311_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq44_e1311_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq44_e1311_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq44_e1311_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq44_e1311_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq44_e1311_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq44_e1311_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq44_e1311_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq44_e1311_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq44_e1311_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq44_e1311_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq44_e1311_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq44_e1311_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq44_e1311_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq44_e1311_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq44_e1311_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq44_e1311_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq44_e1313: f64 = (eq44_e1311 * p.p33);
        let eq44_e1313_d_n0: f64 = (eq44_e1311_d_n0 * p.p33);
        let eq44_e1313_d_n1: f64 = (eq44_e1311_d_n1 * p.p33);
        let eq44_e1313_d_n2: f64 = (eq44_e1311_d_n2 * p.p33);
        let eq44_e1313_d_n3: f64 = (eq44_e1311_d_n3 * p.p33);
        let eq44_e1313_d_n4: f64 = (eq44_e1311_d_n4 * p.p33);
        let eq44_e1313_d_n5: f64 = (eq44_e1311_d_n5 * p.p33);
        let eq44_e1313_d_n6: f64 = (eq44_e1311_d_n6 * p.p33);
        let eq44_e1313_d_n7: f64 = (eq44_e1311_d_n7 * p.p33);
        let eq44_e1313_d_n8: f64 = (eq44_e1311_d_n8 * p.p33);
        let eq44_e1313_d_n9: f64 = (eq44_e1311_d_n9 * p.p33);
        let eq44_e1313_d_n10: f64 = (eq44_e1311_d_n10 * p.p33);
        let eq44_e1313_d_n11: f64 = (eq44_e1311_d_n11 * p.p33);
        let eq44_e1313_d_b0: f64 = (eq44_e1311_d_b0 * p.p33);
        let eq44_e1313_d_b1: f64 = (eq44_e1311_d_b1 * p.p33);
        let eq44_e1313_d_b2: f64 = (eq44_e1311_d_b2 * p.p33);
        let eq44_e1313_d_b3: f64 = (eq44_e1311_d_b3 * p.p33);
        let eq44_e1313_d_b4: f64 = (eq44_e1311_d_b4 * p.p33);
        let eq44_e1313_d_b5: f64 = (eq44_e1311_d_b5 * p.p33);
        let eq44_e1313_d_b6: f64 = (eq44_e1311_d_b6 * p.p33);
        let eq44_e1315: f64 = (eq44_e1313 * s.v[851]);
        let eq44_e1315_d_n0: f64 = ((eq44_e1313_d_n0 * s.v[851]) + (eq44_e1313 * s.dn[851][0]));
        let eq44_e1315_d_n1: f64 = ((eq44_e1313_d_n1 * s.v[851]) + (eq44_e1313 * s.dn[851][1]));
        let eq44_e1315_d_n2: f64 = ((eq44_e1313_d_n2 * s.v[851]) + (eq44_e1313 * s.dn[851][2]));
        let eq44_e1315_d_n3: f64 = ((eq44_e1313_d_n3 * s.v[851]) + (eq44_e1313 * s.dn[851][3]));
        let eq44_e1315_d_n4: f64 = ((eq44_e1313_d_n4 * s.v[851]) + (eq44_e1313 * s.dn[851][4]));
        let eq44_e1315_d_n5: f64 = ((eq44_e1313_d_n5 * s.v[851]) + (eq44_e1313 * s.dn[851][5]));
        let eq44_e1315_d_n6: f64 = ((eq44_e1313_d_n6 * s.v[851]) + (eq44_e1313 * s.dn[851][6]));
        let eq44_e1315_d_n7: f64 = ((eq44_e1313_d_n7 * s.v[851]) + (eq44_e1313 * s.dn[851][7]));
        let eq44_e1315_d_n8: f64 = ((eq44_e1313_d_n8 * s.v[851]) + (eq44_e1313 * s.dn[851][8]));
        let eq44_e1315_d_n9: f64 = ((eq44_e1313_d_n9 * s.v[851]) + (eq44_e1313 * s.dn[851][9]));
        let eq44_e1315_d_n10: f64 = ((eq44_e1313_d_n10 * s.v[851]) + (eq44_e1313 * s.dn[851][10]));
        let eq44_e1315_d_n11: f64 = ((eq44_e1313_d_n11 * s.v[851]) + (eq44_e1313 * s.dn[851][11]));
        let eq44_e1315_d_b0: f64 = ((eq44_e1313_d_b0 * s.v[851]) + (eq44_e1313 * s.db[851][0]));
        let eq44_e1315_d_b1: f64 = ((eq44_e1313_d_b1 * s.v[851]) + (eq44_e1313 * s.db[851][1]));
        let eq44_e1315_d_b2: f64 = ((eq44_e1313_d_b2 * s.v[851]) + (eq44_e1313 * s.db[851][2]));
        let eq44_e1315_d_b3: f64 = ((eq44_e1313_d_b3 * s.v[851]) + (eq44_e1313 * s.db[851][3]));
        let eq44_e1315_d_b4: f64 = ((eq44_e1313_d_b4 * s.v[851]) + (eq44_e1313 * s.db[851][4]));
        let eq44_e1315_d_b5: f64 = ((eq44_e1313_d_b5 * s.v[851]) + (eq44_e1313 * s.db[851][5]));
        let eq44_e1315_d_b6: f64 = ((eq44_e1313_d_b6 * s.v[851]) + (eq44_e1313 * s.db[851][6]));
        let eq44_e1316_q: f64 = eq44_e1315;
        let eq44_reactive_node_derivatives: [f64; 12] = [eq44_e1315_d_n0, eq44_e1315_d_n1, eq44_e1315_d_n2, eq44_e1315_d_n3, eq44_e1315_d_n4, eq44_e1315_d_n5, eq44_e1315_d_n6, eq44_e1315_d_n7, eq44_e1315_d_n8, eq44_e1315_d_n9, eq44_e1315_d_n10, eq44_e1315_d_n11];
        let eq44_reactive_branch_derivatives: [f64; 7] = [eq44_e1315_d_b0, eq44_e1315_d_b1, eq44_e1315_d_b2, eq44_e1315_d_b3, eq44_e1315_d_b4, eq44_e1315_d_b5, eq44_e1315_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[10]),
            Some(nodes[6]),
            nodes,
            &eq44_reactive_node_derivatives,
            branches,
            &eq44_reactive_branch_derivatives,
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
        let nv4 = ctx.node_voltage(nodes[4]);
        let eq45_e1319: f64 = (s.v[0] * s.v[15]);
        let eq45_e1319_d_n0: f64 = ((s.dn[0][0] * s.v[15]) + (s.v[0] * s.dn[15][0]));
        let eq45_e1319_d_n1: f64 = ((s.dn[0][1] * s.v[15]) + (s.v[0] * s.dn[15][1]));
        let eq45_e1319_d_n2: f64 = ((s.dn[0][2] * s.v[15]) + (s.v[0] * s.dn[15][2]));
        let eq45_e1319_d_n3: f64 = ((s.dn[0][3] * s.v[15]) + (s.v[0] * s.dn[15][3]));
        let eq45_e1319_d_n4: f64 = ((s.dn[0][4] * s.v[15]) + (s.v[0] * s.dn[15][4]));
        let eq45_e1319_d_n5: f64 = ((s.dn[0][5] * s.v[15]) + (s.v[0] * s.dn[15][5]));
        let eq45_e1319_d_n6: f64 = ((s.dn[0][6] * s.v[15]) + (s.v[0] * s.dn[15][6]));
        let eq45_e1319_d_n7: f64 = ((s.dn[0][7] * s.v[15]) + (s.v[0] * s.dn[15][7]));
        let eq45_e1319_d_n8: f64 = ((s.dn[0][8] * s.v[15]) + (s.v[0] * s.dn[15][8]));
        let eq45_e1319_d_n9: f64 = ((s.dn[0][9] * s.v[15]) + (s.v[0] * s.dn[15][9]));
        let eq45_e1319_d_n10: f64 = ((s.dn[0][10] * s.v[15]) + (s.v[0] * s.dn[15][10]));
        let eq45_e1319_d_n11: f64 = ((s.dn[0][11] * s.v[15]) + (s.v[0] * s.dn[15][11]));
        let eq45_e1319_d_b0: f64 = ((s.db[0][0] * s.v[15]) + (s.v[0] * s.db[15][0]));
        let eq45_e1319_d_b1: f64 = ((s.db[0][1] * s.v[15]) + (s.v[0] * s.db[15][1]));
        let eq45_e1319_d_b2: f64 = ((s.db[0][2] * s.v[15]) + (s.v[0] * s.db[15][2]));
        let eq45_e1319_d_b3: f64 = ((s.db[0][3] * s.v[15]) + (s.v[0] * s.db[15][3]));
        let eq45_e1319_d_b4: f64 = ((s.db[0][4] * s.v[15]) + (s.v[0] * s.db[15][4]));
        let eq45_e1319_d_b5: f64 = ((s.db[0][5] * s.v[15]) + (s.v[0] * s.db[15][5]));
        let eq45_e1319_d_b6: f64 = ((s.db[0][6] * s.v[15]) + (s.v[0] * s.db[15][6]));
        let eq45_e1321: f64 = (eq45_e1319 * p.p33);
        let eq45_e1321_d_n0: f64 = (eq45_e1319_d_n0 * p.p33);
        let eq45_e1321_d_n1: f64 = (eq45_e1319_d_n1 * p.p33);
        let eq45_e1321_d_n2: f64 = (eq45_e1319_d_n2 * p.p33);
        let eq45_e1321_d_n3: f64 = (eq45_e1319_d_n3 * p.p33);
        let eq45_e1321_d_n4: f64 = (eq45_e1319_d_n4 * p.p33);
        let eq45_e1321_d_n5: f64 = (eq45_e1319_d_n5 * p.p33);
        let eq45_e1321_d_n6: f64 = (eq45_e1319_d_n6 * p.p33);
        let eq45_e1321_d_n7: f64 = (eq45_e1319_d_n7 * p.p33);
        let eq45_e1321_d_n8: f64 = (eq45_e1319_d_n8 * p.p33);
        let eq45_e1321_d_n9: f64 = (eq45_e1319_d_n9 * p.p33);
        let eq45_e1321_d_n10: f64 = (eq45_e1319_d_n10 * p.p33);
        let eq45_e1321_d_n11: f64 = (eq45_e1319_d_n11 * p.p33);
        let eq45_e1321_d_b0: f64 = (eq45_e1319_d_b0 * p.p33);
        let eq45_e1321_d_b1: f64 = (eq45_e1319_d_b1 * p.p33);
        let eq45_e1321_d_b2: f64 = (eq45_e1319_d_b2 * p.p33);
        let eq45_e1321_d_b3: f64 = (eq45_e1319_d_b3 * p.p33);
        let eq45_e1321_d_b4: f64 = (eq45_e1319_d_b4 * p.p33);
        let eq45_e1321_d_b5: f64 = (eq45_e1319_d_b5 * p.p33);
        let eq45_e1321_d_b6: f64 = (eq45_e1319_d_b6 * p.p33);
        let eq45_e1323: f64 = (eq45_e1321 * s.v[852]);
        let eq45_e1323_d_n0: f64 = ((eq45_e1321_d_n0 * s.v[852]) + (eq45_e1321 * s.dn[852][0]));
        let eq45_e1323_d_n1: f64 = ((eq45_e1321_d_n1 * s.v[852]) + (eq45_e1321 * s.dn[852][1]));
        let eq45_e1323_d_n2: f64 = ((eq45_e1321_d_n2 * s.v[852]) + (eq45_e1321 * s.dn[852][2]));
        let eq45_e1323_d_n3: f64 = ((eq45_e1321_d_n3 * s.v[852]) + (eq45_e1321 * s.dn[852][3]));
        let eq45_e1323_d_n4: f64 = ((eq45_e1321_d_n4 * s.v[852]) + (eq45_e1321 * s.dn[852][4]));
        let eq45_e1323_d_n5: f64 = ((eq45_e1321_d_n5 * s.v[852]) + (eq45_e1321 * s.dn[852][5]));
        let eq45_e1323_d_n6: f64 = ((eq45_e1321_d_n6 * s.v[852]) + (eq45_e1321 * s.dn[852][6]));
        let eq45_e1323_d_n7: f64 = ((eq45_e1321_d_n7 * s.v[852]) + (eq45_e1321 * s.dn[852][7]));
        let eq45_e1323_d_n8: f64 = ((eq45_e1321_d_n8 * s.v[852]) + (eq45_e1321 * s.dn[852][8]));
        let eq45_e1323_d_n9: f64 = ((eq45_e1321_d_n9 * s.v[852]) + (eq45_e1321 * s.dn[852][9]));
        let eq45_e1323_d_n10: f64 = ((eq45_e1321_d_n10 * s.v[852]) + (eq45_e1321 * s.dn[852][10]));
        let eq45_e1323_d_n11: f64 = ((eq45_e1321_d_n11 * s.v[852]) + (eq45_e1321 * s.dn[852][11]));
        let eq45_e1323_d_b0: f64 = ((eq45_e1321_d_b0 * s.v[852]) + (eq45_e1321 * s.db[852][0]));
        let eq45_e1323_d_b1: f64 = ((eq45_e1321_d_b1 * s.v[852]) + (eq45_e1321 * s.db[852][1]));
        let eq45_e1323_d_b2: f64 = ((eq45_e1321_d_b2 * s.v[852]) + (eq45_e1321 * s.db[852][2]));
        let eq45_e1323_d_b3: f64 = ((eq45_e1321_d_b3 * s.v[852]) + (eq45_e1321 * s.db[852][3]));
        let eq45_e1323_d_b4: f64 = ((eq45_e1321_d_b4 * s.v[852]) + (eq45_e1321 * s.db[852][4]));
        let eq45_e1323_d_b5: f64 = ((eq45_e1321_d_b5 * s.v[852]) + (eq45_e1321 * s.db[852][5]));
        let eq45_e1323_d_b6: f64 = ((eq45_e1321_d_b6 * s.v[852]) + (eq45_e1321 * s.db[852][6]));
        let eq45_e1324_q: f64 = eq45_e1323;
        let eq45_reactive_node_derivatives: [f64; 12] = [eq45_e1323_d_n0, eq45_e1323_d_n1, eq45_e1323_d_n2, eq45_e1323_d_n3, eq45_e1323_d_n4, eq45_e1323_d_n5, eq45_e1323_d_n6, eq45_e1323_d_n7, eq45_e1323_d_n8, eq45_e1323_d_n9, eq45_e1323_d_n10, eq45_e1323_d_n11];
        let eq45_reactive_branch_derivatives: [f64; 7] = [eq45_e1323_d_b0, eq45_e1323_d_b1, eq45_e1323_d_b2, eq45_e1323_d_b3, eq45_e1323_d_b4, eq45_e1323_d_b5, eq45_e1323_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[11]),
            Some(nodes[7]),
            nodes,
            &eq45_reactive_node_derivatives,
            branches,
            &eq45_reactive_branch_derivatives,
            multiplicity,
        );
        let eq48_e1335: f64 = (s.v[854] * (nv4 - 0.0));
        let eq48_e1335_d_n0: f64 = (s.dn[854][0] * (nv4 - 0.0));
        let eq48_e1335_d_n1: f64 = (s.dn[854][1] * (nv4 - 0.0));
        let eq48_e1335_d_n2: f64 = (s.dn[854][2] * (nv4 - 0.0));
        let eq48_e1335_d_n3: f64 = (s.dn[854][3] * (nv4 - 0.0));
        let eq48_e1335_d_n4: f64 = ((s.dn[854][4] * (nv4 - 0.0)) + s.v[854]);
        let eq48_e1335_d_n5: f64 = (s.dn[854][5] * (nv4 - 0.0));
        let eq48_e1335_d_n6: f64 = (s.dn[854][6] * (nv4 - 0.0));
        let eq48_e1335_d_n7: f64 = (s.dn[854][7] * (nv4 - 0.0));
        let eq48_e1335_d_n8: f64 = (s.dn[854][8] * (nv4 - 0.0));
        let eq48_e1335_d_n9: f64 = (s.dn[854][9] * (nv4 - 0.0));
        let eq48_e1335_d_n10: f64 = (s.dn[854][10] * (nv4 - 0.0));
        let eq48_e1335_d_n11: f64 = (s.dn[854][11] * (nv4 - 0.0));
        let eq48_e1335_d_b0: f64 = (s.db[854][0] * (nv4 - 0.0));
        let eq48_e1335_d_b1: f64 = (s.db[854][1] * (nv4 - 0.0));
        let eq48_e1335_d_b2: f64 = (s.db[854][2] * (nv4 - 0.0));
        let eq48_e1335_d_b3: f64 = (s.db[854][3] * (nv4 - 0.0));
        let eq48_e1335_d_b4: f64 = (s.db[854][4] * (nv4 - 0.0));
        let eq48_e1335_d_b5: f64 = (s.db[854][5] * (nv4 - 0.0));
        let eq48_e1335_d_b6: f64 = (s.db[854][6] * (nv4 - 0.0));
        let eq48_e1336_q: f64 = eq48_e1335;
        let eq48_reactive_node_derivatives: [f64; 12] = [eq48_e1335_d_n0, eq48_e1335_d_n1, eq48_e1335_d_n2, eq48_e1335_d_n3, eq48_e1335_d_n4, eq48_e1335_d_n5, eq48_e1335_d_n6, eq48_e1335_d_n7, eq48_e1335_d_n8, eq48_e1335_d_n9, eq48_e1335_d_n10, eq48_e1335_d_n11];
        let eq48_reactive_branch_derivatives: [f64; 7] = [eq48_e1335_d_b0, eq48_e1335_d_b1, eq48_e1335_d_b2, eq48_e1335_d_b3, eq48_e1335_d_b4, eq48_e1335_d_b5, eq48_e1335_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[4]),
            None,
            nodes,
            &eq48_reactive_node_derivatives,
            branches,
            &eq48_reactive_branch_derivatives,
            multiplicity,
        );
        let eq49_e1339: f64 = (s.v[15] * p.p32);
        let eq49_e1339_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq49_e1339_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq49_e1339_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq49_e1339_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq49_e1339_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq49_e1339_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq49_e1339_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq49_e1339_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq49_e1339_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq49_e1339_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq49_e1339_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq49_e1339_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq49_e1339_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq49_e1339_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq49_e1339_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq49_e1339_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq49_e1339_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq49_e1339_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq49_e1339_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq49_e1340: f64 = (eq49_e1339).sqrt();
        let eq49_e1340_d_n0: f64 = (eq49_e1339_d_n0 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n1: f64 = (eq49_e1339_d_n1 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n2: f64 = (eq49_e1339_d_n2 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n3: f64 = (eq49_e1339_d_n3 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n4: f64 = (eq49_e1339_d_n4 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n5: f64 = (eq49_e1339_d_n5 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n6: f64 = (eq49_e1339_d_n6 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n7: f64 = (eq49_e1339_d_n7 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n8: f64 = (eq49_e1339_d_n8 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n9: f64 = (eq49_e1339_d_n9 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n10: f64 = (eq49_e1339_d_n10 / (2.0 * eq49_e1340));
        let eq49_e1340_d_n11: f64 = (eq49_e1339_d_n11 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b0: f64 = (eq49_e1339_d_b0 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b1: f64 = (eq49_e1339_d_b1 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b2: f64 = (eq49_e1339_d_b2 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b3: f64 = (eq49_e1339_d_b3 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b4: f64 = (eq49_e1339_d_b4 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b5: f64 = (eq49_e1339_d_b5 / (2.0 * eq49_e1340));
        let eq49_e1340_d_b6: f64 = (eq49_e1339_d_b6 / (2.0 * eq49_e1340));
        let eq49_e1342: f64 = (eq49_e1340 * 0.5);
        let eq49_e1342_d_n0: f64 = (eq49_e1340_d_n0 * 0.5);
        let eq49_e1342_d_n1: f64 = (eq49_e1340_d_n1 * 0.5);
        let eq49_e1342_d_n2: f64 = (eq49_e1340_d_n2 * 0.5);
        let eq49_e1342_d_n3: f64 = (eq49_e1340_d_n3 * 0.5);
        let eq49_e1342_d_n4: f64 = (eq49_e1340_d_n4 * 0.5);
        let eq49_e1342_d_n5: f64 = (eq49_e1340_d_n5 * 0.5);
        let eq49_e1342_d_n6: f64 = (eq49_e1340_d_n6 * 0.5);
        let eq49_e1342_d_n7: f64 = (eq49_e1340_d_n7 * 0.5);
        let eq49_e1342_d_n8: f64 = (eq49_e1340_d_n8 * 0.5);
        let eq49_e1342_d_n9: f64 = (eq49_e1340_d_n9 * 0.5);
        let eq49_e1342_d_n10: f64 = (eq49_e1340_d_n10 * 0.5);
        let eq49_e1342_d_n11: f64 = (eq49_e1340_d_n11 * 0.5);
        let eq49_e1342_d_b0: f64 = (eq49_e1340_d_b0 * 0.5);
        let eq49_e1342_d_b1: f64 = (eq49_e1340_d_b1 * 0.5);
        let eq49_e1342_d_b2: f64 = (eq49_e1340_d_b2 * 0.5);
        let eq49_e1342_d_b3: f64 = (eq49_e1340_d_b3 * 0.5);
        let eq49_e1342_d_b4: f64 = (eq49_e1340_d_b4 * 0.5);
        let eq49_e1342_d_b5: f64 = (eq49_e1340_d_b5 * 0.5);
        let eq49_e1342_d_b6: f64 = (eq49_e1340_d_b6 * 0.5);
        let eq49_e1344: f64 = (eq49_e1342 * s.v[854]);
        let eq49_e1344_d_n0: f64 = ((eq49_e1342_d_n0 * s.v[854]) + (eq49_e1342 * s.dn[854][0]));
        let eq49_e1344_d_n1: f64 = ((eq49_e1342_d_n1 * s.v[854]) + (eq49_e1342 * s.dn[854][1]));
        let eq49_e1344_d_n2: f64 = ((eq49_e1342_d_n2 * s.v[854]) + (eq49_e1342 * s.dn[854][2]));
        let eq49_e1344_d_n3: f64 = ((eq49_e1342_d_n3 * s.v[854]) + (eq49_e1342 * s.dn[854][3]));
        let eq49_e1344_d_n4: f64 = ((eq49_e1342_d_n4 * s.v[854]) + (eq49_e1342 * s.dn[854][4]));
        let eq49_e1344_d_n5: f64 = ((eq49_e1342_d_n5 * s.v[854]) + (eq49_e1342 * s.dn[854][5]));
        let eq49_e1344_d_n6: f64 = ((eq49_e1342_d_n6 * s.v[854]) + (eq49_e1342 * s.dn[854][6]));
        let eq49_e1344_d_n7: f64 = ((eq49_e1342_d_n7 * s.v[854]) + (eq49_e1342 * s.dn[854][7]));
        let eq49_e1344_d_n8: f64 = ((eq49_e1342_d_n8 * s.v[854]) + (eq49_e1342 * s.dn[854][8]));
        let eq49_e1344_d_n9: f64 = ((eq49_e1342_d_n9 * s.v[854]) + (eq49_e1342 * s.dn[854][9]));
        let eq49_e1344_d_n10: f64 = ((eq49_e1342_d_n10 * s.v[854]) + (eq49_e1342 * s.dn[854][10]));
        let eq49_e1344_d_n11: f64 = ((eq49_e1342_d_n11 * s.v[854]) + (eq49_e1342 * s.dn[854][11]));
        let eq49_e1344_d_b0: f64 = ((eq49_e1342_d_b0 * s.v[854]) + (eq49_e1342 * s.db[854][0]));
        let eq49_e1344_d_b1: f64 = ((eq49_e1342_d_b1 * s.v[854]) + (eq49_e1342 * s.db[854][1]));
        let eq49_e1344_d_b2: f64 = ((eq49_e1342_d_b2 * s.v[854]) + (eq49_e1342 * s.db[854][2]));
        let eq49_e1344_d_b3: f64 = ((eq49_e1342_d_b3 * s.v[854]) + (eq49_e1342 * s.db[854][3]));
        let eq49_e1344_d_b4: f64 = ((eq49_e1342_d_b4 * s.v[854]) + (eq49_e1342 * s.db[854][4]));
        let eq49_e1344_d_b5: f64 = ((eq49_e1342_d_b5 * s.v[854]) + (eq49_e1342 * s.db[854][5]));
        let eq49_e1344_d_b6: f64 = ((eq49_e1342_d_b6 * s.v[854]) + (eq49_e1342 * s.db[854][6]));
        let eq49_e1346: f64 = (eq49_e1344 * (nv4 - 0.0));
        let eq49_e1346_d_n0: f64 = (eq49_e1344_d_n0 * (nv4 - 0.0));
        let eq49_e1346_d_n1: f64 = (eq49_e1344_d_n1 * (nv4 - 0.0));
        let eq49_e1346_d_n2: f64 = (eq49_e1344_d_n2 * (nv4 - 0.0));
        let eq49_e1346_d_n3: f64 = (eq49_e1344_d_n3 * (nv4 - 0.0));
        let eq49_e1346_d_n4: f64 = ((eq49_e1344_d_n4 * (nv4 - 0.0)) + eq49_e1344);
        let eq49_e1346_d_n5: f64 = (eq49_e1344_d_n5 * (nv4 - 0.0));
        let eq49_e1346_d_n6: f64 = (eq49_e1344_d_n6 * (nv4 - 0.0));
        let eq49_e1346_d_n7: f64 = (eq49_e1344_d_n7 * (nv4 - 0.0));
        let eq49_e1346_d_n8: f64 = (eq49_e1344_d_n8 * (nv4 - 0.0));
        let eq49_e1346_d_n9: f64 = (eq49_e1344_d_n9 * (nv4 - 0.0));
        let eq49_e1346_d_n10: f64 = (eq49_e1344_d_n10 * (nv4 - 0.0));
        let eq49_e1346_d_n11: f64 = (eq49_e1344_d_n11 * (nv4 - 0.0));
        let eq49_e1346_d_b0: f64 = (eq49_e1344_d_b0 * (nv4 - 0.0));
        let eq49_e1346_d_b1: f64 = (eq49_e1344_d_b1 * (nv4 - 0.0));
        let eq49_e1346_d_b2: f64 = (eq49_e1344_d_b2 * (nv4 - 0.0));
        let eq49_e1346_d_b3: f64 = (eq49_e1344_d_b3 * (nv4 - 0.0));
        let eq49_e1346_d_b4: f64 = (eq49_e1344_d_b4 * (nv4 - 0.0));
        let eq49_e1346_d_b5: f64 = (eq49_e1344_d_b5 * (nv4 - 0.0));
        let eq49_e1346_d_b6: f64 = (eq49_e1344_d_b6 * (nv4 - 0.0));
        let eq49_e1347_q: f64 = eq49_e1346;
        let eq49_e1348: f64 = (-eq49_e1346);
        let eq49_e1348_d_n0: f64 = (-eq49_e1346_d_n0);
        let eq49_e1348_d_n1: f64 = (-eq49_e1346_d_n1);
        let eq49_e1348_d_n2: f64 = (-eq49_e1346_d_n2);
        let eq49_e1348_d_n3: f64 = (-eq49_e1346_d_n3);
        let eq49_e1348_d_n4: f64 = (-eq49_e1346_d_n4);
        let eq49_e1348_d_n5: f64 = (-eq49_e1346_d_n5);
        let eq49_e1348_d_n6: f64 = (-eq49_e1346_d_n6);
        let eq49_e1348_d_n7: f64 = (-eq49_e1346_d_n7);
        let eq49_e1348_d_n8: f64 = (-eq49_e1346_d_n8);
        let eq49_e1348_d_n9: f64 = (-eq49_e1346_d_n9);
        let eq49_e1348_d_n10: f64 = (-eq49_e1346_d_n10);
        let eq49_e1348_d_n11: f64 = (-eq49_e1346_d_n11);
        let eq49_e1348_d_b0: f64 = (-eq49_e1346_d_b0);
        let eq49_e1348_d_b1: f64 = (-eq49_e1346_d_b1);
        let eq49_e1348_d_b2: f64 = (-eq49_e1346_d_b2);
        let eq49_e1348_d_b3: f64 = (-eq49_e1346_d_b3);
        let eq49_e1348_d_b4: f64 = (-eq49_e1346_d_b4);
        let eq49_e1348_d_b5: f64 = (-eq49_e1346_d_b5);
        let eq49_e1348_d_b6: f64 = (-eq49_e1346_d_b6);
        let eq49_e1348_q: f64 = (-eq49_e1347_q);
        let eq49_e1348_q_d_n0: f64 = (-eq49_e1346_d_n0);
        let eq49_e1348_q_d_n1: f64 = (-eq49_e1346_d_n1);
        let eq49_e1348_q_d_n2: f64 = (-eq49_e1346_d_n2);
        let eq49_e1348_q_d_n3: f64 = (-eq49_e1346_d_n3);
        let eq49_e1348_q_d_n4: f64 = (-eq49_e1346_d_n4);
        let eq49_e1348_q_d_n5: f64 = (-eq49_e1346_d_n5);
        let eq49_e1348_q_d_n6: f64 = (-eq49_e1346_d_n6);
        let eq49_e1348_q_d_n7: f64 = (-eq49_e1346_d_n7);
        let eq49_e1348_q_d_n8: f64 = (-eq49_e1346_d_n8);
        let eq49_e1348_q_d_n9: f64 = (-eq49_e1346_d_n9);
        let eq49_e1348_q_d_n10: f64 = (-eq49_e1346_d_n10);
        let eq49_e1348_q_d_n11: f64 = (-eq49_e1346_d_n11);
        let eq49_e1348_q_d_b0: f64 = (-eq49_e1346_d_b0);
        let eq49_e1348_q_d_b1: f64 = (-eq49_e1346_d_b1);
        let eq49_e1348_q_d_b2: f64 = (-eq49_e1346_d_b2);
        let eq49_e1348_q_d_b3: f64 = (-eq49_e1346_d_b3);
        let eq49_e1348_q_d_b4: f64 = (-eq49_e1346_d_b4);
        let eq49_e1348_q_d_b5: f64 = (-eq49_e1346_d_b5);
        let eq49_e1348_q_d_b6: f64 = (-eq49_e1346_d_b6);
        let eq49_reactive_node_derivatives: [f64; 12] = [eq49_e1348_q_d_n0, eq49_e1348_q_d_n1, eq49_e1348_q_d_n2, eq49_e1348_q_d_n3, eq49_e1348_q_d_n4, eq49_e1348_q_d_n5, eq49_e1348_q_d_n6, eq49_e1348_q_d_n7, eq49_e1348_q_d_n8, eq49_e1348_q_d_n9, eq49_e1348_q_d_n10, eq49_e1348_q_d_n11];
        let eq49_reactive_branch_derivatives: [f64; 7] = [eq49_e1348_q_d_b0, eq49_e1348_q_d_b1, eq49_e1348_q_d_b2, eq49_e1348_q_d_b3, eq49_e1348_q_d_b4, eq49_e1348_q_d_b5, eq49_e1348_q_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[6]),
            nodes,
            &eq49_reactive_node_derivatives,
            branches,
            &eq49_reactive_branch_derivatives,
            multiplicity,
        );
        let eq50_e1351: f64 = (s.v[15] * p.p32);
        let eq50_e1351_d_n0: f64 = (s.dn[15][0] * p.p32);
        let eq50_e1351_d_n1: f64 = (s.dn[15][1] * p.p32);
        let eq50_e1351_d_n2: f64 = (s.dn[15][2] * p.p32);
        let eq50_e1351_d_n3: f64 = (s.dn[15][3] * p.p32);
        let eq50_e1351_d_n4: f64 = (s.dn[15][4] * p.p32);
        let eq50_e1351_d_n5: f64 = (s.dn[15][5] * p.p32);
        let eq50_e1351_d_n6: f64 = (s.dn[15][6] * p.p32);
        let eq50_e1351_d_n7: f64 = (s.dn[15][7] * p.p32);
        let eq50_e1351_d_n8: f64 = (s.dn[15][8] * p.p32);
        let eq50_e1351_d_n9: f64 = (s.dn[15][9] * p.p32);
        let eq50_e1351_d_n10: f64 = (s.dn[15][10] * p.p32);
        let eq50_e1351_d_n11: f64 = (s.dn[15][11] * p.p32);
        let eq50_e1351_d_b0: f64 = (s.db[15][0] * p.p32);
        let eq50_e1351_d_b1: f64 = (s.db[15][1] * p.p32);
        let eq50_e1351_d_b2: f64 = (s.db[15][2] * p.p32);
        let eq50_e1351_d_b3: f64 = (s.db[15][3] * p.p32);
        let eq50_e1351_d_b4: f64 = (s.db[15][4] * p.p32);
        let eq50_e1351_d_b5: f64 = (s.db[15][5] * p.p32);
        let eq50_e1351_d_b6: f64 = (s.db[15][6] * p.p32);
        let eq50_e1352: f64 = (eq50_e1351).sqrt();
        let eq50_e1352_d_n0: f64 = (eq50_e1351_d_n0 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n1: f64 = (eq50_e1351_d_n1 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n2: f64 = (eq50_e1351_d_n2 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n3: f64 = (eq50_e1351_d_n3 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n4: f64 = (eq50_e1351_d_n4 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n5: f64 = (eq50_e1351_d_n5 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n6: f64 = (eq50_e1351_d_n6 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n7: f64 = (eq50_e1351_d_n7 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n8: f64 = (eq50_e1351_d_n8 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n9: f64 = (eq50_e1351_d_n9 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n10: f64 = (eq50_e1351_d_n10 / (2.0 * eq50_e1352));
        let eq50_e1352_d_n11: f64 = (eq50_e1351_d_n11 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b0: f64 = (eq50_e1351_d_b0 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b1: f64 = (eq50_e1351_d_b1 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b2: f64 = (eq50_e1351_d_b2 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b3: f64 = (eq50_e1351_d_b3 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b4: f64 = (eq50_e1351_d_b4 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b5: f64 = (eq50_e1351_d_b5 / (2.0 * eq50_e1352));
        let eq50_e1352_d_b6: f64 = (eq50_e1351_d_b6 / (2.0 * eq50_e1352));
        let eq50_e1354: f64 = (eq50_e1352 * 0.5);
        let eq50_e1354_d_n0: f64 = (eq50_e1352_d_n0 * 0.5);
        let eq50_e1354_d_n1: f64 = (eq50_e1352_d_n1 * 0.5);
        let eq50_e1354_d_n2: f64 = (eq50_e1352_d_n2 * 0.5);
        let eq50_e1354_d_n3: f64 = (eq50_e1352_d_n3 * 0.5);
        let eq50_e1354_d_n4: f64 = (eq50_e1352_d_n4 * 0.5);
        let eq50_e1354_d_n5: f64 = (eq50_e1352_d_n5 * 0.5);
        let eq50_e1354_d_n6: f64 = (eq50_e1352_d_n6 * 0.5);
        let eq50_e1354_d_n7: f64 = (eq50_e1352_d_n7 * 0.5);
        let eq50_e1354_d_n8: f64 = (eq50_e1352_d_n8 * 0.5);
        let eq50_e1354_d_n9: f64 = (eq50_e1352_d_n9 * 0.5);
        let eq50_e1354_d_n10: f64 = (eq50_e1352_d_n10 * 0.5);
        let eq50_e1354_d_n11: f64 = (eq50_e1352_d_n11 * 0.5);
        let eq50_e1354_d_b0: f64 = (eq50_e1352_d_b0 * 0.5);
        let eq50_e1354_d_b1: f64 = (eq50_e1352_d_b1 * 0.5);
        let eq50_e1354_d_b2: f64 = (eq50_e1352_d_b2 * 0.5);
        let eq50_e1354_d_b3: f64 = (eq50_e1352_d_b3 * 0.5);
        let eq50_e1354_d_b4: f64 = (eq50_e1352_d_b4 * 0.5);
        let eq50_e1354_d_b5: f64 = (eq50_e1352_d_b5 * 0.5);
        let eq50_e1354_d_b6: f64 = (eq50_e1352_d_b6 * 0.5);
        let eq50_e1356: f64 = (eq50_e1354 * s.v[854]);
        let eq50_e1356_d_n0: f64 = ((eq50_e1354_d_n0 * s.v[854]) + (eq50_e1354 * s.dn[854][0]));
        let eq50_e1356_d_n1: f64 = ((eq50_e1354_d_n1 * s.v[854]) + (eq50_e1354 * s.dn[854][1]));
        let eq50_e1356_d_n2: f64 = ((eq50_e1354_d_n2 * s.v[854]) + (eq50_e1354 * s.dn[854][2]));
        let eq50_e1356_d_n3: f64 = ((eq50_e1354_d_n3 * s.v[854]) + (eq50_e1354 * s.dn[854][3]));
        let eq50_e1356_d_n4: f64 = ((eq50_e1354_d_n4 * s.v[854]) + (eq50_e1354 * s.dn[854][4]));
        let eq50_e1356_d_n5: f64 = ((eq50_e1354_d_n5 * s.v[854]) + (eq50_e1354 * s.dn[854][5]));
        let eq50_e1356_d_n6: f64 = ((eq50_e1354_d_n6 * s.v[854]) + (eq50_e1354 * s.dn[854][6]));
        let eq50_e1356_d_n7: f64 = ((eq50_e1354_d_n7 * s.v[854]) + (eq50_e1354 * s.dn[854][7]));
        let eq50_e1356_d_n8: f64 = ((eq50_e1354_d_n8 * s.v[854]) + (eq50_e1354 * s.dn[854][8]));
        let eq50_e1356_d_n9: f64 = ((eq50_e1354_d_n9 * s.v[854]) + (eq50_e1354 * s.dn[854][9]));
        let eq50_e1356_d_n10: f64 = ((eq50_e1354_d_n10 * s.v[854]) + (eq50_e1354 * s.dn[854][10]));
        let eq50_e1356_d_n11: f64 = ((eq50_e1354_d_n11 * s.v[854]) + (eq50_e1354 * s.dn[854][11]));
        let eq50_e1356_d_b0: f64 = ((eq50_e1354_d_b0 * s.v[854]) + (eq50_e1354 * s.db[854][0]));
        let eq50_e1356_d_b1: f64 = ((eq50_e1354_d_b1 * s.v[854]) + (eq50_e1354 * s.db[854][1]));
        let eq50_e1356_d_b2: f64 = ((eq50_e1354_d_b2 * s.v[854]) + (eq50_e1354 * s.db[854][2]));
        let eq50_e1356_d_b3: f64 = ((eq50_e1354_d_b3 * s.v[854]) + (eq50_e1354 * s.db[854][3]));
        let eq50_e1356_d_b4: f64 = ((eq50_e1354_d_b4 * s.v[854]) + (eq50_e1354 * s.db[854][4]));
        let eq50_e1356_d_b5: f64 = ((eq50_e1354_d_b5 * s.v[854]) + (eq50_e1354 * s.db[854][5]));
        let eq50_e1356_d_b6: f64 = ((eq50_e1354_d_b6 * s.v[854]) + (eq50_e1354 * s.db[854][6]));
        let eq50_e1358: f64 = (eq50_e1356 * (nv4 - 0.0));
        let eq50_e1358_d_n0: f64 = (eq50_e1356_d_n0 * (nv4 - 0.0));
        let eq50_e1358_d_n1: f64 = (eq50_e1356_d_n1 * (nv4 - 0.0));
        let eq50_e1358_d_n2: f64 = (eq50_e1356_d_n2 * (nv4 - 0.0));
        let eq50_e1358_d_n3: f64 = (eq50_e1356_d_n3 * (nv4 - 0.0));
        let eq50_e1358_d_n4: f64 = ((eq50_e1356_d_n4 * (nv4 - 0.0)) + eq50_e1356);
        let eq50_e1358_d_n5: f64 = (eq50_e1356_d_n5 * (nv4 - 0.0));
        let eq50_e1358_d_n6: f64 = (eq50_e1356_d_n6 * (nv4 - 0.0));
        let eq50_e1358_d_n7: f64 = (eq50_e1356_d_n7 * (nv4 - 0.0));
        let eq50_e1358_d_n8: f64 = (eq50_e1356_d_n8 * (nv4 - 0.0));
        let eq50_e1358_d_n9: f64 = (eq50_e1356_d_n9 * (nv4 - 0.0));
        let eq50_e1358_d_n10: f64 = (eq50_e1356_d_n10 * (nv4 - 0.0));
        let eq50_e1358_d_n11: f64 = (eq50_e1356_d_n11 * (nv4 - 0.0));
        let eq50_e1358_d_b0: f64 = (eq50_e1356_d_b0 * (nv4 - 0.0));
        let eq50_e1358_d_b1: f64 = (eq50_e1356_d_b1 * (nv4 - 0.0));
        let eq50_e1358_d_b2: f64 = (eq50_e1356_d_b2 * (nv4 - 0.0));
        let eq50_e1358_d_b3: f64 = (eq50_e1356_d_b3 * (nv4 - 0.0));
        let eq50_e1358_d_b4: f64 = (eq50_e1356_d_b4 * (nv4 - 0.0));
        let eq50_e1358_d_b5: f64 = (eq50_e1356_d_b5 * (nv4 - 0.0));
        let eq50_e1358_d_b6: f64 = (eq50_e1356_d_b6 * (nv4 - 0.0));
        let eq50_e1359_q: f64 = eq50_e1358;
        let eq50_e1360: f64 = (-eq50_e1358);
        let eq50_e1360_d_n0: f64 = (-eq50_e1358_d_n0);
        let eq50_e1360_d_n1: f64 = (-eq50_e1358_d_n1);
        let eq50_e1360_d_n2: f64 = (-eq50_e1358_d_n2);
        let eq50_e1360_d_n3: f64 = (-eq50_e1358_d_n3);
        let eq50_e1360_d_n4: f64 = (-eq50_e1358_d_n4);
        let eq50_e1360_d_n5: f64 = (-eq50_e1358_d_n5);
        let eq50_e1360_d_n6: f64 = (-eq50_e1358_d_n6);
        let eq50_e1360_d_n7: f64 = (-eq50_e1358_d_n7);
        let eq50_e1360_d_n8: f64 = (-eq50_e1358_d_n8);
        let eq50_e1360_d_n9: f64 = (-eq50_e1358_d_n9);
        let eq50_e1360_d_n10: f64 = (-eq50_e1358_d_n10);
        let eq50_e1360_d_n11: f64 = (-eq50_e1358_d_n11);
        let eq50_e1360_d_b0: f64 = (-eq50_e1358_d_b0);
        let eq50_e1360_d_b1: f64 = (-eq50_e1358_d_b1);
        let eq50_e1360_d_b2: f64 = (-eq50_e1358_d_b2);
        let eq50_e1360_d_b3: f64 = (-eq50_e1358_d_b3);
        let eq50_e1360_d_b4: f64 = (-eq50_e1358_d_b4);
        let eq50_e1360_d_b5: f64 = (-eq50_e1358_d_b5);
        let eq50_e1360_d_b6: f64 = (-eq50_e1358_d_b6);
        let eq50_e1360_q: f64 = (-eq50_e1359_q);
        let eq50_e1360_q_d_n0: f64 = (-eq50_e1358_d_n0);
        let eq50_e1360_q_d_n1: f64 = (-eq50_e1358_d_n1);
        let eq50_e1360_q_d_n2: f64 = (-eq50_e1358_d_n2);
        let eq50_e1360_q_d_n3: f64 = (-eq50_e1358_d_n3);
        let eq50_e1360_q_d_n4: f64 = (-eq50_e1358_d_n4);
        let eq50_e1360_q_d_n5: f64 = (-eq50_e1358_d_n5);
        let eq50_e1360_q_d_n6: f64 = (-eq50_e1358_d_n6);
        let eq50_e1360_q_d_n7: f64 = (-eq50_e1358_d_n7);
        let eq50_e1360_q_d_n8: f64 = (-eq50_e1358_d_n8);
        let eq50_e1360_q_d_n9: f64 = (-eq50_e1358_d_n9);
        let eq50_e1360_q_d_n10: f64 = (-eq50_e1358_d_n10);
        let eq50_e1360_q_d_n11: f64 = (-eq50_e1358_d_n11);
        let eq50_e1360_q_d_b0: f64 = (-eq50_e1358_d_b0);
        let eq50_e1360_q_d_b1: f64 = (-eq50_e1358_d_b1);
        let eq50_e1360_q_d_b2: f64 = (-eq50_e1358_d_b2);
        let eq50_e1360_q_d_b3: f64 = (-eq50_e1358_d_b3);
        let eq50_e1360_q_d_b4: f64 = (-eq50_e1358_d_b4);
        let eq50_e1360_q_d_b5: f64 = (-eq50_e1358_d_b5);
        let eq50_e1360_q_d_b6: f64 = (-eq50_e1358_d_b6);
        let eq50_reactive_node_derivatives: [f64; 12] = [eq50_e1360_q_d_n0, eq50_e1360_q_d_n1, eq50_e1360_q_d_n2, eq50_e1360_q_d_n3, eq50_e1360_q_d_n4, eq50_e1360_q_d_n5, eq50_e1360_q_d_n6, eq50_e1360_q_d_n7, eq50_e1360_q_d_n8, eq50_e1360_q_d_n9, eq50_e1360_q_d_n10, eq50_e1360_q_d_n11];
        let eq50_reactive_branch_derivatives: [f64; 7] = [eq50_e1360_q_d_b0, eq50_e1360_q_d_b1, eq50_e1360_q_d_b2, eq50_e1360_q_d_b3, eq50_e1360_q_d_b4, eq50_e1360_q_d_b5, eq50_e1360_q_d_b6];
        stamper.stamp_current_reactive_dense(
            Some(nodes[5]),
            Some(nodes[7]),
            nodes,
            &eq50_reactive_node_derivatives,
            branches,
            &eq50_reactive_branch_derivatives,
            multiplicity,
        );
    }
}
