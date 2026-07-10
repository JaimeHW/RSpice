#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_117(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2505] = (4.0 == 1.0);s.store_scalar(2505, if s.b[2505] { 1.0 } else { 0.0 });
        if (((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && s.b[2505]) {s.store_scalar(720, 1.0);}
        s.b[2506] = (4.0 == 2.0);s.store_scalar(2506, if s.b[2506] { 1.0 } else { 0.0 });
        if ((((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && (!s.b[2505])) && s.b[2506]) {s.store_scalar(720, 2.0);}
        s.b[2507] = (4.0 == 4.0);s.store_scalar(2507, if s.b[2507] { 1.0 } else { 0.0 });
        if (((((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && (!s.b[2505])) && (!s.b[2506])) && s.b[2507]) {s.store_scalar(720, 3.0);}
        s.b[2508] = (4.0 == 8.0);s.store_scalar(2508, if s.b[2508] { 1.0 } else { 0.0 });
        if ((((((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && (!s.b[2505])) && (!s.b[2506])) && (!s.b[2507])) && s.b[2508]) {s.store_scalar(720, 4.0);}
        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) {s.store_scalar(719, 0.0);}
        let mut t1: usize = 0;
        while {
            let t0: f64 = if (((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t0 != 0.0
        } {
            t1 += 1;assert!(t1 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && s.b[2504]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((!s.b[1439]) && s.b[2426]) && s.b[2499]) && (!s.b[2504])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((!s.b[1439]) && s.b[2426]) && s.b[2499]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(333, 332, 726, 1.0);s.store_div_scaled_product_indices(336, 725, 726, 1.0, 770, 1.0);s.store_scale(145, 155, ((2.0 * s.v[495]) * p.p7));s.copy_ad(335, 170);s.store_div_scaled_product_mixed_aii(153, A::mul3(s.ad_value(145), s.ad_value(253), s.ad_value(152)), 333, 1.0, 335, 1.0);s.store_add(134, 134, 153);}
        s.b[2509] = (((p.p31 != 0.0) && (p.p30 != 0.0)) && (s.v[963] == 0.0));s.store_scalar(2509, if s.b[2509] { 1.0 } else { 0.0 });
        if (((!s.b[1439]) && s.b[2426]) && s.b[2509]) {s.store_square(317, 127);s.store_mul3_affine_lhs(318, 155, 186, 2.0, 0.0, 248);s.store_sub(319, 317, 318);s.store_sqrt_square_offset(782, 317, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(334, 317, 782, 0.5, 0.5);s.store_scaled_add(317, 317, 782, 0.5);}
        s.b[2510] = (s.v[317] < 0.0);s.store_scalar(2510, if s.b[2510] { 1.0 } else { 0.0 });
        if ((((!s.b[1439]) && s.b[2426]) && s.b[2509]) && s.b[2510]) {s.store_scalar(317, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1439]) && s.b[2426]) && s.b[2509]) {s.store_sqrt_square_offset(782, 319, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(334, 319, 782, 0.5, 0.5);s.store_scaled_add(319, 319, 782, 0.5);}
        s.b[2511] = (s.v[319] < 0.0);s.store_scalar(2511, if s.b[2511] { 1.0 } else { 0.0 });
        if ((((!s.b[1439]) && s.b[2426]) && s.b[2509]) && s.b[2511]) {s.store_scalar(319, 0.0);s.store_scalar(334, 0.0);}
        if (((!s.b[1439]) && s.b[2426]) && s.b[2509]) {s.store_sub(320, 317, 319);}
        s.b[2512] = ((s.v[238] < (10.0 * 2.220446049250313e-16)) || (s.v[320] < (10.0 * 2.220446049250313e-16)));s.store_scalar(2512, if s.b[2512] { 1.0 } else { 0.0 });
        if ((((!s.b[1439]) && s.b[2426]) && s.b[2509]) && s.b[2512]) {s.store_scalar(321, 0.0);}
        if ((((!s.b[1439]) && s.b[2426]) && s.b[2509]) && (!s.b[2512])) {s.store_scalar(321, 1.0);}
        if ((!s.b[1439]) && (s.v[946] != 0.0)) {s.store_scalar(946, 0.0);}
        s.b[2513] = ((s.v[78] == 0.0) && (s.v[127] > 1e-12));s.store_scalar(2513, if s.b[2513] { 1.0 } else { 0.0 });
        if ((!s.b[1439]) && s.b[2513]) {s.store_div_scaled_product_indices(130, 212, 154, 1.0, 100, 2.0);s.store_add_mixed_ai(128, A::div_scaled_value_offset_denominator(s.ad_value(127), 1.0, s.ad_value(130), 1.0, 1.0), 87);}
        if ((!s.b[1439]) && (!s.b[2513])) {s.store_scalar(128, 0.0);}
        if (!s.b[1439]) {s.copy_ad(136, 134);s.store_scalar(46, 0.0);}
        s.b[2515] = ((p.p450 > 0.0) && (p.p454 > 0.0));s.store_scalar(2515, if s.b[2515] { 1.0 } else { 0.0 });
        if ((!s.b[1439]) && s.b[2515]) {s.store_scalar(2520, 1e-5);s.store_offset_add_scaled_inputs3_offset_indices(2521, 791, 1.0, 120, 1.0, 182, -1.0, (-s.v[160]), (-p.p455));s.store_offset(2522, 118, p.p455);s.store_sqrt_offset_ad(781, A::square(A::sub(s.ad_value(960), s.ad_value(1431))), ((4.0 * 0.01) * 0.01));s.store_add_scaled_inputs3_indices(2532, 960, 0.5, 1431, ((-1.0) * 0.5), 781, 0.5);s.store_sqrt_ad(2516, A::div_scaled_product_offset_denominator(s.ad_value(2532), s.ad_value(586), (((2.0 * 1.6021918e-19) * 1.0 / (1.034943e-10)) * s.v[489]), s.ad_value(586), s.v[489], 1.0));s.store_mul(2518, 2516, 162);s.store_div_scaled_product_add_scaled_denominator_indices(993, 2518, 2518, (-0.25), 790, 1.0, 2518, 1.0, 1.0);}
        s.b[2534] = (p.p457 > 0.0);s.store_scalar(2534, if s.b[2534] { 1.0 } else { 0.0 });
        if (((!s.b[1439]) && s.b[2515]) && s.b[2534]) {s.store_scalar(2519, p.p457);}
        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {s.copy_ad(2535, 993);s.copy_ad(2536, 2522);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_118(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::sub(s.ad_value(2521), s.ad_value(2535))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);}
        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {
            if (s.v[332] >= (10.0 * 2.220446049250313e-16)) {
            } else {
                s.store_scalar(332, (10.0 * 2.220446049250313e-16));
            }
        }
        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {s.store_add_product3_rhs_mixed_iia(89, 2521, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 0.5);s.store_mul_sub_rhs(116, 154, 89, 2535);}
        s.b[2537] = (s.v[116] < 3.0);s.store_scalar(2537, if s.b[2537] { 1.0 } else { 0.0 });
        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2537]) {s.store_mul_sub_rhs(333, 154, 2521, 2535);s.store_div_scalar_by_product_indices(335, 1.0, 154, 212, (1.414213562373095 / 108.0));s.store_offset_scaled(336, 335, 3.0, 81.0);s.store_add_scaled_sub_value_product_mixed_aii(337, (-2916.0), A::scale(s.ad_value(335), 81.0), 1.0, 335, 333, 27.0);s.store_add_scaled_sub_value_product_mixed_aii(338, 1458.0, A::scaled_offset(s.ad_value(335), 54.0, 81.0), 1.0, 335, 333, 27.0);s.store_square(338, 338);}
        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2537]) {
            if ((s.v[337] + ((((((4.0 * s.v[336]) * s.v[336]) * s.v[336]) + s.v[338])) as f64).sqrt()) == 0.0) {
                s.store_scalar(339, 0.0);
            } else {
                s.store_powf_ad(339, A::add(s.ad_value(337), A::sqrt(A::add(A::mul3_scaled_output(s.ad_value(336), s.ad_value(336), s.ad_value(336), 4.0), s.ad_value(338)))), 0.3333333333333333);
            }
        }
        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2537]) {s.store_add_scaled_inputs_mixed_ai(332, A::sub_from_scalar(3.0, A::div_scaled_inputs(s.ad_value(336), 1.259921049894873, s.ad_value(339), 3.0)), 1.0, 339, (1.0 / (3.0 * 1.259921049894873)));s.store_add_scaled_product_indices(89, 2535, 1.0, 332, 155, 1.0);s.copy_ad(88, 89);}
        s.b[2538] = (s.v[791] <= s.v[2536]);s.store_scalar(2538, if s.b[2538] { 1.0 } else { 0.0 });
        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && (!s.b[2537])) && s.b[2538]) {s.copy_ad(88, 89);}
        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && (!s.b[2537])) && (!s.b[2538])) {s.store_div_scalar_by_product_indices(335, 1.0, 210, 211, 1.0);s.store_mul3_lhs(336, 335, 2521, 2521);s.store_add_div_from_scalar_rhs(337, 154, 2.0, 2521);s.store_offset_div_ad(90, A::ln(s.ad_value(336)), s.ad_value(337), p.p456);s.store_offset_sub(781, 90, 89, (-0.0008));s.store_scale(782, 90, (4.0 * 0.0008));}
        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && (!s.b[2537])) && (!s.b[2538])) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && (!s.b[2537])) && (!s.b[2538])) {s.store_sqrt_square_add(782, 781, 782);s.store_add_scaled_inputs3_indices(88, 90, 1.0, 781, (-0.5), 782, (-0.5));}
        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {s.store_offset(332, 2535, (1e-12 / 2.0));}
        s.b[2539] = (s.v[88] < s.v[332]);s.store_scalar(2539, if s.b[2539] { 1.0 } else { 0.0 });
        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2539]) {s.copy_ad(88, 332);}
        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) {s.copy_ad(2519, 88);}
        s.b[2540] = (p.p451 == 1.0);s.store_scalar(2540, if s.b[2540] { 1.0 } else { 0.0 });
        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) {s.copy_ad(88, 2519);s.copy_ad(2541, 993);s.store_offset_add_scaled_inputs3_offset_indices(86, 120, (-1.0), 182, 1.0, 2541, 1.0, s.v[160], p.p455);}
        s.b[2550] = (s.v[791] < s.v[86]);s.store_scalar(2550, if s.b[2550] { 1.0 } else { 0.0 });
        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) {s.store_scalar(347, (-1.0));s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_sub_rhs(332, 154, 2521, 2541);s.store_div_scalar_by_product_indices(335, 1.0, 154, 209, 1.0);s.store_mul(333, 335, 185);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_offset(338, 332, (-2.0));s.store_scaled_mul(339, 333, 338, 9.0);s.store_sub_from_scalar(278, (7.0 * 1.414213562373095), 339);s.store_square(276, 278);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_119(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2551] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(2551, if s.b[2551] { 1.0 } else { 0.0 });
        if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) && s.b[2551]) {s.store_add_scaled_inputs3_offset_mixed_iai(274, 278, 1.0, A::div_scaled_inputs(s.ad_value(277), 0.5, s.ad_value(278), 1.0), 1.0, 339, 1.0, ((-7.0) * 1.414213562373095));}
        if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) && (!s.b[2551])) {s.store_sqrt_add(275, 277, 276);s.store_add_offset_lhs(274, 275, ((-7.0) * 1.414213562373095), 339);}
        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) {
            if (s.v[274] == 0.0) {
                s.store_scalar(273, 0.0);
            } else {
                s.store_powf(273, 274, 0.3333333333333333);
            }
        }
        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && s.b[2550]) {s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div_from_scalar(335, 1.0, 273);s.store_mul(116, 272, 335);s.store_add_scaled_product_indices(167, 2541, 1.0, 116, 155, 1.0);s.store_sub(335, 167, 2541);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_add_div_lhs_indices(2519, 335, 337, 2541);}
        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {s.store_exp_ad(230, A::mul_offset_rhs(s.ad_value(154), s.ad_value(2541), (-p.p456)));s.store_scalar(79, 0.0);s.copy_ad(2542, 88);s.store_mul3_affine_lhs(2543, 166, 2520, (0.5 * 9662367879.197212), 0.0, 2520);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 2543);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(2544, 335, 2543);s.store_scalar(97, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_120(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let mut t4: usize = 0;
        while {
            let t2: f64 = (s.v[421] + 1.0);let t3: f64 = if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (s.v[97] <= t2)) { 1.0 } else { 0.0 };
            t3 != 0.0
        } {
            t4 += 1;assert!(t4 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {s.store_sub(2545, 2542, 2541);s.store_mul(116, 154, 2545);s.store_mul_sub_rhs(333, 2544, 2545, 2543);}
            s.b[2552] = (s.v[333] < 60.0);s.store_scalar(2552, if s.b[2552] { 1.0 } else { 0.0 });
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2552]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 2544, -1.0, 2543);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(2547, 336, 1.0, 2544);s.store_div_scaled_value_offset_denominator(2548, s.ad_value(335), 1.0, s.ad_value(336), 1.0, 1.0);}
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2552])) {s.store_sub(2547, 2545, 2543);s.store_scalar(2548, 1.0);}
            if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {s.store_mul(2546, 154, 2547);}
            s.b[2553] = (((s.v[116]) as f64).abs() < 1e-16);s.store_scalar(2553, if s.b[2553] { 1.0 } else { 0.0 });
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2553]) {s.store_sqrt_scaled_input_ad(334, A::sub_from_scalar(1.0, A::square(s.ad_value(2548))), 1.0 / (2.0));s.store_mul(223, 116, 334);s.store_mul(2549, 154, 334);}
            s.b[2554] = (s.v[116] < 0.0);s.store_scalar(2554, if s.b[2554] { 1.0 } else { 0.0 });
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2553]) && s.b[2554]) {s.store_neg(223, 223);s.store_neg(2549, 2549);}
            s.b[2555] = (((s.v[116]) as f64).abs() < 0.005);s.store_scalar(2555, if s.b[2555] { 1.0 } else { 0.0 });
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2553])) && s.b[2555]) {s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(334, 116, 1.0, 116, 1.0, 116, 1.0, 116, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(335, 116, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(116), 1.0, A::scale(s.ad_value(116), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_square_mul_sub_from_scalar_double_scaled_sub_rhs_scaled_output(336, 2546, 1.0, 2546, 1.0, 2546, 1.0, 2546, 0.2, 1.0 / (4.0), 1.0 / (3.0), 1.0 / (2.0));s.store_mul_scale_offset_mixed_ia(337, 2546, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2546), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2546), 1.0, A::scale(s.ad_value(2546), 0.25), 1.0 / (3.0)), 1.0 / (2.0)), -1.0, 1.0);s.store_sqrt_sub(223, 334, 336);s.store_div_scaled_product_mixed_iai(2549, 154, A::add_scaled_product(s.ad_value(335), 1.0, s.ad_value(2548), s.ad_value(337), (-1.0)), 0.5, 223, 1.0);}
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2553])) && (!s.b[2555])) {s.store_exp_neg_input(334, 116);s.store_exp_neg_input(335, 2546);s.store_sqrt_ad(223, A::add_scaled_inputs4(s.ad_value(116), 1.0, s.ad_value(2546), (-1.0), s.ad_value(334), 1.0, s.ad_value(335), (-1.0)));s.store_div_scaled_product_mixed_iai(2549, 154, A::sub(A::sub_from_scalar(1.0, s.ad_value(334)), A::mul_sub_from_scalar_rhs(s.ad_value(2548), 1.0, s.ad_value(335))), 0.5, 223, 1.0);}
            s.b[2556] = ((s.v[79] == 1.0) && (s.v[116] < 0.0));s.store_scalar(2556, if s.b[2556] { 1.0 } else { 0.0 });
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2556]) {s.store_scalar(347, (-1.0));}
            s.b[2557] = (s.v[116] < 0.0);s.store_scalar(2557, if s.b[2557] { 1.0 } else { 0.0 });
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2557]) {s.store_neg(216, 223);s.store_neg(217, 2549);}
            s.b[2558] = (s.v[116] < 1e-7);s.store_scalar(2558, if s.b[2558] { 1.0 } else { 0.0 });
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2557])) && s.b[2558]) {s.copy_ad(216, 223);s.copy_ad(217, 2549);}
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2557])) && (!s.b[2558])) {s.store_mul_scale_offset_indices(117, 154, 2542, 1.0, (-p.p456));s.store_exp(228, 117);s.store_mul_mixed_ia(214, 210, A::add_scaled_offset_product_rhs(s.ad_value(228), 1.0, s.ad_value(230), s.ad_value(116), 1.0, (-1.0)));s.store_mul_ad_product_rhs_mixed_ia(215, 210, 154, A::sub(s.ad_value(228), s.ad_value(230)));s.store_sqrt_square_add(216, 223, 214);s.store_div_scaled_add_product_indices(217, 215, 0.5, 2549, 223, (2.0 * 0.5), 216, 1.0);}
            if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {s.store_add_scaled_inputs_product_indices(232, 2542, 1.0, 2521, (-1.0), 212, 216, 1.0);s.store_offset_mul(233, 212, 217, 1.0);}
            s.b[2559] = (s.v[79] == 1.0);s.store_scalar(2559, if s.b[2559] { 1.0 } else { 0.0 });
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && s.b[2559]) {s.store_scalar(97, (s.v[421] + 1.0));}
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) {s.store_div_scaled_inputs_indices(236, 232, -1.0, 233, 1.0);}
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) {
                s.store_scaled_offset_ad(93, {
                    if (1.0 >= ((s.v[2542]) as f64).abs()) {
                        A::constant(1.0)
                    } else {
                        A::abs(s.ad_value(2542))
                    }
                }, 1.0, (0.5 * 0.1));
            }
            s.b[2560] = (((s.v[236]) as f64).abs() > s.v[93]);s.store_scalar(2560, if s.b[2560] { 1.0 } else { 0.0 });
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) && s.b[2560]) {s.store_scale(236, 93, (if (s.v[236] >= 0.0) { 1.0 } else { (-1.0) }));}
            if ((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) {s.store_add(2542, 2542, 236);}
            s.b[2561] = ((((s.v[236]) as f64).abs() <= 1e-12) && (((s.v[232]) as f64).abs() <= 1e-8));s.store_scalar(2561, if s.b[2561] { 1.0 } else { 0.0 });
            if (((((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) && (!s.b[2559])) && s.b[2561]) {s.store_scalar(79, 1.0);}
            if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {s.store_primal_offset(97, 97, 1.0);}
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_121(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((!s.b[1439]) && s.b[2515]) && (!s.b[2534])) && s.b[2540]) && (!s.b[2550])) {s.copy_ad(2519, 2542);}
        if ((!s.b[1439]) && s.b[2515]) {s.store_mul_sub_scaled_inputs_rhs_indices(339, 154, 2519, -1.0, 993, -1.0);s.store_abs(2531, 339);s.store_exp(340, 339);s.store_sub_offset_lhs(341, 340, (-1.0), 339);}
        s.b[2562] = (s.v[339] > 1e-7);s.store_scalar(2562, if s.b[2562] { 1.0 } else { 0.0 });
        if (((!s.b[1439]) && s.b[2515]) && s.b[2562]) {s.store_mul_scaled_sqrt_rhs(2533, 209, -1.0, 341);}
        s.b[2563] = (s.v[2531] > 1e-7);s.store_scalar(2563, if s.b[2563] { 1.0 } else { 0.0 });
        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2562])) && s.b[2563]) {s.store_mul_sqrt_rhs(2533, 209, 341);}
        if ((((!s.b[1439]) && s.b[2515]) && (!s.b[2562])) && (!s.b[2563])) {s.store_mul_scaled_sqrt_ad_rhs(2533, 339, (-0.7071067811865475), A::offset(A::mul_scaled_lhs(s.ad_value(2531), 0.3333333333333333, A::scale_offset(s.ad_value(2531), 0.25, 1.0)), 1.0));}
        if ((!s.b[1439]) && s.b[2515]) {s.store_sqrt_square_offset(781, 2533, ((4.0 * 1e-6) * 1e-6));s.store_scaled_add(2528, 2533, 781, 0.5);s.store_div_scaled_inputs_indices(2529, 2528, 1.0, 586, 1.6021918e-19);s.store_offset(335, 2529, (-p.p452));s.store_scale(2530, 2529, 0.01);s.store_sqrt_add_scaled_square_product(781, 335, 1.0, 2530, 2530, 4.0);s.store_scaled_add(336, 335, 781, 0.5);s.store_div_scaled_product_by_product_indices(2527, 336, 336, 1.0, 2529, 2529, 1.0);s.store_add_scaled_product_mixed_iai(994, 993, 1.0, A::sub(s.ad_value(2519), s.ad_value(993)), 2527, 1.0);s.store_mul_scale_offset(333, A::exp(A::mul(s.ad_value(154), A::add_scaled_inputs3(s.ad_value(994), 1.0, s.ad_value(960), -1.0, s.ad_value(1431), 1.0))), A::exp(A::mul_scaled_lhs(s.ad_value(154), -1.0, s.ad_value(790))), -1.0, 1.0);s.store_scalar(2523, (((((2.0 * 1.6021918e-19) * s.v[489]) * 1.034943e-10)) as f64).sqrt());s.store_mul_sqrt_rhs(2524, 2523, 155);s.store_mul_sub_rhs(2517, 154, 994, 993);}
        s.b[2564] = ((s.v[2517] < (0.2 * s.v[154])) && ((0.2 * s.v[154]) >= 0.0));s.store_scalar(2564, if s.b[2564] { 1.0 } else { 0.0 });
        if (((!s.b[1439]) && s.b[2515]) && s.b[2564]) {s.store_sub_scaled_inputs(781, 154, 0.2, 2517, 1.0);s.store_square(722, 781);s.store_scaled_mul(723, 154, 154, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2565] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2565, if s.b[2565] { 1.0 } else { 0.0 });s.b[2566] = (1.0 == 1.0);s.store_scalar(2566, if s.b[2566] { 1.0 } else { 0.0 });
        if (((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && s.b[2566]) {s.store_scalar(720, 1.0);}
        s.b[2567] = (1.0 == 2.0);s.store_scalar(2567, if s.b[2567] { 1.0 } else { 0.0 });
        if ((((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && (!s.b[2566])) && s.b[2567]) {s.store_scalar(720, 2.0);}
        s.b[2568] = (1.0 == 4.0);s.store_scalar(2568, if s.b[2568] { 1.0 } else { 0.0 });
        if (((((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && (!s.b[2566])) && (!s.b[2567])) && s.b[2568]) {s.store_scalar(720, 3.0);}
        s.b[2569] = (1.0 == 8.0);s.store_scalar(2569, if s.b[2569] { 1.0 } else { 0.0 });
        if ((((((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && (!s.b[2566])) && (!s.b[2567])) && (!s.b[2568])) && s.b[2569]) {s.store_scalar(720, 4.0);}
        if ((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) {s.store_scalar(719, 0.0);}
        let mut t6: usize = 0;
        while {
            let t5: f64 = if (((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t5 != 0.0
        } {
            t6 += 1;assert!(t6 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && s.b[2565]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((!s.b[1439]) && s.b[2515]) && s.b[2564]) && (!s.b[2565])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((!s.b[1439]) && s.b[2515]) && s.b[2564]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 154, 0.2, 0.0, 726);s.store_div_scaled_product3_indices(334, 154, 725, 726, 0.2, 770, 1.0);s.store_sub_scaled_inputs(335, 154, 0.2, 780, 1.0);}
        if (((!s.b[1439]) && s.b[2515]) && s.b[2564]) {
        }
        if (((!s.b[1439]) && s.b[2515]) && (!s.b[2564])) {s.copy_ad(335, 2517);s.store_scalar(334, 1.0);}
        if ((!s.b[1439]) && s.b[2515]) {s.store_sqrt_offset_input(2525, 335, (10.0 * 2.220446049250313e-16));s.store_mul(2526, 2524, 2525);s.store_mul_scale_offset_mixed_ai(995, A::div_scaled_inputs(s.ad_value(155), 2.0, s.ad_value(162), 1.0), 2526, p.p454, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_122(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[1439]) && s.b[2515]) {s.store_scaled_mul(46, 995, 333, s.v[632]);s.store_add(134, 136, 46);}
        if (!s.b[1439]) {s.store_add(134, 136, 46);s.copy_ad(978, 133);}
        s.store_scale(335, 162, (-s.v[635]));s.store_mul(20, 335, 131);s.store_mul(132, 335, 133);s.store_mul(19, 132, 247);s.store_mul(979, 335, 978);s.store_scaled_sub(335, 790, 94, 0.5);s.store_scale(781, 335, (2.0 * 1.0 / (p.p263)));s.store_offset_mul_offset_rhs_mixed_ia(782, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 5040.0), (1.0 / 720.0))), (1.0 / 120.0)), (1.0 / 24.0)), (1.0 / 6.0)), (1.0 / 2.0), 1.0);s.store_offset_mul_offset_rhs_mixed_ia(783, 781, A::mul_offset_rhs(s.ad_value(781), A::mul_offset_rhs(s.ad_value(781), A::mul(s.ad_value(781), A::scale_offset(s.ad_value(781), (1.0 / 840.0), (1.0 / 144.0))), (1.0 / 30.0)), (1.0 / 8.0)), (1.0 / 3.0), (1.0 / 2.0));s.store_div_from_scalar(110, p.p263, 782);s.store_div_scaled_inputs_square_rhs(336, 783, (-2.0), 782, 1.0);s.b[2570] = ((s.v[110] < ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16))) && ((10.0 * 2.220446049250313e-16) >= 0.0));s.store_scalar(2570, if s.b[2570] { 1.0 } else { 0.0 });
        if s.b[2570] {s.store_sub_from_scalar(781, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 110);s.store_square(722, 781);s.store_scalar(723, ((10.0 * 2.220446049250313e-16) * (10.0 * 2.220446049250313e-16)));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2571] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2571, if s.b[2571] { 1.0 } else { 0.0 });s.b[2572] = (2.0 == 1.0);s.store_scalar(2572, if s.b[2572] { 1.0 } else { 0.0 });
        if ((s.b[2570] && s.b[2571]) && s.b[2572]) {s.store_scalar(720, 1.0);}
        s.b[2573] = (2.0 == 2.0);s.store_scalar(2573, if s.b[2573] { 1.0 } else { 0.0 });
        if (((s.b[2570] && s.b[2571]) && (!s.b[2572])) && s.b[2573]) {s.store_scalar(720, 2.0);}
        s.b[2574] = (2.0 == 4.0);s.store_scalar(2574, if s.b[2574] { 1.0 } else { 0.0 });
        if ((((s.b[2570] && s.b[2571]) && (!s.b[2572])) && (!s.b[2573])) && s.b[2574]) {s.store_scalar(720, 3.0);}
        s.b[2575] = (2.0 == 8.0);s.store_scalar(2575, if s.b[2575] { 1.0 } else { 0.0 });
        if (((((s.b[2570] && s.b[2571]) && (!s.b[2572])) && (!s.b[2573])) && (!s.b[2574])) && s.b[2575]) {s.store_scalar(720, 4.0);}
        if (s.b[2570] && s.b[2571]) {s.store_scalar(719, 0.0);}
        let mut t8: usize = 0;
        while {
            let t7: f64 = if ((s.b[2570] && s.b[2571]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t7 != 0.0
        } {
            t8 += 1;assert!(t8 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if (s.b[2570] && s.b[2571]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if (s.b[2570] && (!s.b[2571])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if s.b[2570] {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, (10.0 * 2.220446049250313e-16));s.store_div_scaled_product_indices(334, 725, 726, (10.0 * 2.220446049250313e-16), 770, 1.0);s.store_sub_from_scalar(110, ((10.0 * 2.220446049250313e-16) + (10.0 * 2.220446049250313e-16)), 780);}
        if s.b[2570] {
        }
        if (!s.b[2570]) {
        }
        if (!s.b[2570]) {s.store_scalar(334, 1.0);}
        s.store_add(109, 87, 110);s.store_add_scaled_product_mixed_iai(134, 134, 1.0, A::div_from_scalar(s.v[163], s.ad_value(162)), 790, p.p435);s.b[2576] = (p.p23 == 0.0);s.store_scalar(2576, if s.b[2576] { 1.0 } else { 0.0 });
        if s.b[2576] {s.store_scalar(280, 0.0);s.store_scalar(288, 0.0);}
        s.b[2577] = ((s.v[481] > 0.0) && (s.v[454] > 0.0));s.store_scalar(2577, if s.b[2577] { 1.0 } else { 0.0 });
        if ((!s.b[2576]) && s.b[2577]) {s.store_mul(335, 659, 85);s.store_scale(337, 636, 1.0 / ((s.v[188] * s.v[188])));s.store_scale_ad(338, A::div_from_scalar(2.0, s.ad_value(636)), (s.v[188] * s.v[188]));s.store_add_scaled_inputs_product_indices(339, 335, 1.0, 155, (-1.0), 660, 1434, (-1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_123(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[2576]) && s.b[2577]) {s.store_offset_mul(340, 338, 339, 1.0);s.store_scaled_offset(341, 338, 1.0, 2.0);}
        s.b[2578] = ((s.v[340] < (1e-6 + s.v[341])) && (s.v[341] >= 0.0));s.store_scalar(2578, if s.b[2578] { 1.0 } else { 0.0 });
        if (((!s.b[2576]) && s.b[2577]) && s.b[2578]) {s.store_sub_offset_lhs(781, 341, 1e-6, 340);s.store_square(722, 781);s.store_square(723, 341);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2579] = ((((4.0 == 1.0) || (4.0 == 2.0)) || (4.0 == 4.0)) || (4.0 == 8.0));s.store_scalar(2579, if s.b[2579] { 1.0 } else { 0.0 });s.b[2580] = (4.0 == 1.0);s.store_scalar(2580, if s.b[2580] { 1.0 } else { 0.0 });
        if (((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && s.b[2580]) {s.store_scalar(720, 1.0);}
        s.b[2581] = (4.0 == 2.0);s.store_scalar(2581, if s.b[2581] { 1.0 } else { 0.0 });
        if ((((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && (!s.b[2580])) && s.b[2581]) {s.store_scalar(720, 2.0);}
        s.b[2582] = (4.0 == 4.0);s.store_scalar(2582, if s.b[2582] { 1.0 } else { 0.0 });
        if (((((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && (!s.b[2580])) && (!s.b[2581])) && s.b[2582]) {s.store_scalar(720, 3.0);}
        s.b[2583] = (4.0 == 8.0);s.store_scalar(2583, if s.b[2583] { 1.0 } else { 0.0 });
        if ((((((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && (!s.b[2580])) && (!s.b[2581])) && (!s.b[2582])) && s.b[2583]) {s.store_scalar(720, 4.0);}
        if ((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) {s.store_scalar(719, 0.0);}
        let mut ta: usize = 0;
        while {
            let t9: f64 = if (((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t9 != 0.0
        } {
            ta += 1;assert!(ta <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && s.b[2579]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((!s.b[2576]) && s.b[2577]) && s.b[2578]) && (!s.b[2579])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 4.0)));
            }
        }
        if (((!s.b[2576]) && s.b[2577]) && s.b[2578]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 341, 726);s.store_div_scaled_product3_indices(334, 341, 725, 726, 1.0, 770, 1.0);s.store_sub_offset_lhs(340, 341, 1e-6, 780);}
        if (((!s.b[2576]) && s.b[2577]) && s.b[2578]) {
        }
        if (((!s.b[2576]) && s.b[2577]) && (!s.b[2578])) {
        }
        if (((!s.b[2576]) && s.b[2577]) && (!s.b[2578])) {s.store_scalar(334, 1.0);}
        if ((!s.b[2576]) && s.b[2577]) {s.store_sqrt(340, 340);s.store_add_mul_sub_from_scalar_rhs_indices(282, 335, 337, 1.0, 340);s.store_div_from_scalar_offset_input(336, s.v[582], 661, s.v[582]);s.store_add_scaled_inputs_product_indices(283, 1435, s.v[483], 109, 1.0, 336, 282, (-1.0));s.store_sqrt_square_offset(782, 283, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(343, 283, 782, 0.5, 0.5);s.store_scaled_add(283, 283, 782, 0.5);}
        s.b[2584] = (s.v[283] < 0.0);s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });
        if (((!s.b[2576]) && s.b[2577]) && s.b[2584]) {s.store_scalar(283, 0.0);s.store_scalar(343, 0.0);}
        if ((!s.b[2576]) && s.b[2577]) {s.store_offset(283, 283, 1e-25);s.store_offset_mul_offset_rhs(958, 957, 387, (-s.v[764]), 1.0);}
        if ((!s.b[2576]) && s.b[2577]) {
            if (s.v[958] <= 0.001) {
                s.store_scalar(958, 0.001);
            } else {
            }
        }
        if ((!s.b[2576]) && s.b[2577]) {s.store_div(339, 662, 958);s.store_mul(340, 663, 958);s.store_ad_value(336, A::exp_div_scaled_inputs(s.ad_value(340), -1.0, s.ad_value(283), 1.0));s.store_mul_product3_indices(280, 336, 339, 283, 134, 1.0);s.store_mul3_lhs(288, 339, 283, 336);}
        if ((!s.b[2576]) && (!s.b[2577])) {s.store_scalar(280, 0.0);}
        s.b[2585] = (s.v[664] != 0.0);s.store_scalar(2585, if s.b[2585] { 1.0 } else { 0.0 });
        if ((!s.b[2576]) && s.b[2585]) {s.copy_ad(334, 799);s.store_sqrt_square_offset(782, 334, ((4.0 * 1e-6) * 1e-6));s.store_offset_scaled_div(335, 334, 782, 0.5, 0.5);s.store_scaled_add(334, 334, 782, 0.5);}
        s.b[2586] = (s.v[334] < 0.0);s.store_scalar(2586, if s.b[2586] { 1.0 } else { 0.0 });
        if (((!s.b[2576]) && s.b[2585]) && s.b[2586]) {s.store_scalar(334, 0.0);s.store_scalar(335, 0.0);}
        if ((!s.b[2576]) && s.b[2585]) {s.store_sqrt_offset_input(335, 127, 1e-25);s.store_div_from_scalar_scaled_input(337, 1.0, 335, 2.0);s.store_sub_mixed_ia(338, 334, A::scale_offset(s.ad_value(791), ((p.p106) * (p.p105)), p.p105));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_124(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[2576]) && s.b[2585]) {s.store_sqrt_square_offset(782, 338, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 338, 782, 0.5, 0.5);s.store_scaled_add(338, 338, 782, 0.5);}
        s.b[2587] = (s.v[338] < 0.0);s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });
        if (((!s.b[2576]) && s.b[2585]) && s.b[2587]) {s.store_scalar(338, 0.0);s.store_scalar(343, 0.0);}
        if ((!s.b[2576]) && s.b[2585]) {s.store_offset(338, 338, 1e-25);s.store_mul_ad_product_rhs_mixed_ia(344, 450, 451, A::exp(A::div_from_scalar((-1.0), s.ad_value(338))));s.store_mul_scale_offset_mixed_ia(345, 344, A::div_from_scalar(1.0, s.ad_value(338)), 1.0, 1.0);s.store_mul(337, 338, 344);s.store_sub(334, 334, 337);s.store_sqrt_square_offset(782, 334, ((4.0 * 0.01) * 0.01));s.store_offset_scaled_div(343, 334, 782, 0.5, 0.5);s.store_scaled_add(334, 334, 782, 0.5);}
        s.b[2588] = (s.v[334] < 0.0);s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });
        if (((!s.b[2576]) && s.b[2585]) && s.b[2588]) {s.store_scalar(334, 0.0);s.store_scalar(343, 0.0);}
        if ((!s.b[2576]) && s.b[2585]) {s.store_offset(334, 334, 1e-25);s.store_div_scalar_by_product_indices(338, 1.0, 334, 335, 1.0);s.store_scalar(341, (s.v[165] * s.v[554]));s.store_exp_mul_scaled_lhs_indices(336, 341, -1.0, 338);s.store_mul_product3_indices(340, 338, 341, 336, 338, 1.0);s.store_mul_product3_indices(281, 336, 664, 134, 334, 1.0);}
        s.b[2589] = (p.p45 == 0.0);s.store_scalar(2589, if s.b[2589] { 1.0 } else { 0.0 });
        if s.b[2589] {s.store_scalar(423, 0.0);}
        s.b[2590] = ((p.p45 * (s.v[796] - p.p446)) < 0.0);s.store_scalar(2590, if s.b[2590] { 1.0 } else { 0.0 });
        if ((!s.b[2589]) && s.b[2590]) {s.copy_ad(426, 427);}
        if ((!s.b[2589]) && (!s.b[2590])) {s.store_add_scaled_inputs_mixed_ai(426, A::square(A::offset(s.ad_value(796), (-p.p446))), p.p445, 427, 1.0);}
        if (!s.b[2589]) {s.store_scaled_limited_exp_ad(423, A::mul(s.ad_value(154), A::sub(s.ad_value(793), s.ad_value(426))), p.p449);}
        s.b[2591] = (s.v[423] > 0.0);s.store_scalar(2591, if s.b[2591] { 1.0 } else { 0.0 });s.b[2592] = ((s.v[423] > (100000.0 - 50000.0)) && (50000.0 >= 0.0));s.store_scalar(2592, if s.b[2592] { 1.0 } else { 0.0 });
        if (s.b[2591] && s.b[2592]) {s.store_offset(781, 423, (((-100000.0)) + (50000.0)));s.store_square(722, 781);s.store_scalar(723, (50000.0 * 50000.0));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2593] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });s.b[2594] = (1.0 == 1.0);s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });
        if (((s.b[2591] && s.b[2592]) && s.b[2593]) && s.b[2594]) {s.store_scalar(720, 1.0);}
        s.b[2595] = (1.0 == 2.0);s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });
        if ((((s.b[2591] && s.b[2592]) && s.b[2593]) && (!s.b[2594])) && s.b[2595]) {s.store_scalar(720, 2.0);}
        s.b[2596] = (1.0 == 4.0);s.store_scalar(2596, if s.b[2596] { 1.0 } else { 0.0 });
        if (((((s.b[2591] && s.b[2592]) && s.b[2593]) && (!s.b[2594])) && (!s.b[2595])) && s.b[2596]) {s.store_scalar(720, 3.0);}
        s.b[2597] = (1.0 == 8.0);s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });
        if ((((((s.b[2591] && s.b[2592]) && s.b[2593]) && (!s.b[2594])) && (!s.b[2595])) && (!s.b[2596])) && s.b[2597]) {s.store_scalar(720, 4.0);}
        if ((s.b[2591] && s.b[2592]) && s.b[2593]) {s.store_scalar(719, 0.0);}
        let mut tc: usize = 0;
        while {
            let tb: f64 = if (((s.b[2591] && s.b[2592]) && s.b[2593]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tb != 0.0
        } {
            tc += 1;assert!(tc <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.b[2591] && s.b[2592]) && s.b[2593]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((s.b[2591] && s.b[2592]) && (!s.b[2593])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (s.b[2591] && s.b[2592]) {s.store_div_from_scalar(726, 1.0, 726);s.store_scaled_mul(780, 781, 726, 50000.0);s.store_div_scaled_product_indices(334, 725, 726, 50000.0, 770, 1.0);s.store_offset(336, 780, (100000.0 - 50000.0));}
        if (s.b[2591] && s.b[2592]) {
        }
        if (s.b[2591] && (!s.b[2592])) {s.copy_ad(336, 423);s.store_scalar(334, 1.0);}
        s.b[2598] = ((((s.v[280] + s.v[281]) > 0.0) && (s.v[523] != 0.0)) && (s.v[963] == 0.0));s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_125(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[2598] {s.store_offset_scaled(334, 120, s.v[524], 1.0);s.store_add(335, 280, 281);s.store_scaled_mul(111, 334, 335, s.v[523]);s.store_div_from_scalar(344, 1.0, 99);s.store_mul3_lhs(335, 154, 111, 344);s.store_square(345, 344);s.store_div_from_scalar(344, 1.0, 102);s.store_mul3_lhs(336, 154, 111, 344);s.store_square(345, 344);s.store_mul_mixed_ia(112, 209, A::add_scaled_products(s.ad_value(104), s.ad_value(336), 1.0, s.ad_value(101), s.ad_value(335), (-1.0)));s.store_mul_add_scaled_products_indices_rhs(113, 209, 103, 336, ((-1.0) * (0.5)), 100, 335, 0.5);s.store_add(114, 112, 113);s.store_mul3_lhs(400, 115, 114, 253);s.store_mul(287, 288, 400);}
        s.b[2599] = (p.p24 != 0.0);s.store_scalar(2599, if s.b[2599] { 1.0 } else { 0.0 });s.b[2600] = (s.v[78] == 0.0);s.store_scalar(2600, if s.b[2600] { 1.0 } else { 0.0 });
        if (s.b[2599] && s.b[2600]) {s.store_offset_add(191, 109, 1435, (-(10.0 * 2.220446049250313e-16)));s.store_sub_scaled_inputs_mixed_ai(335, A::add_scaled_product(A::offset(s.ad_value(1436), (-s.v[160])), 1.0, A::sub(s.ad_value(120), s.ad_value(182)), s.ad_value(162), s.v[560]), 1.0, 191, s.v[515]);s.store_square(335, 335);s.store_scalar(337, (1.0 / s.v[187]));s.store_mul(336, 335, 337);s.store_scalar(337, (1.0 / s.v[561]));s.store_offset_mul(341, 255, 337, 1.0);s.store_mul(195, 336, 341);s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);s.store_scaled_add(195, 195, 782, 0.5);}
        s.b[2601] = (s.v[195] < 0.0);s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });
        if ((s.b[2599] && s.b[2600]) && s.b[2601]) {s.store_scalar(195, 0.0);s.store_scalar(339, 0.0);}
        if (s.b[2599] && s.b[2600]) {s.store_sqrt_square_offset(782, 1436, ((4.0 * 0.001) * 0.001));s.store_offset_scaled_div(338, 1436, 782, 0.5, 0.5);s.store_scaled_add(337, 1436, 782, 0.5);}
        s.b[2602] = (s.v[337] < 0.0);s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });
        if ((s.b[2599] && s.b[2600]) && s.b[2602]) {s.store_scalar(337, 0.0);s.store_scalar(338, 0.0);}
        if (s.b[2599] && s.b[2600]) {s.store_offset(337, 337, (-p.p262));s.store_scale(332, 337, 10.0);s.store_offset_square(336, 332, 1.0);s.store_sub_from_scalar_ad(335, 1.0, A::div_from_scalar(1.0, s.ad_value(336)));s.store_mul(195, 195, 335);s.store_scale(334, 162, s.v[632]);s.store_div_from_scalar_offset_input(341, s.v[562], 334, s.v[562]);s.store_scalar(340, s.v[516]);s.store_div_add_scaled_inputs_rhs_indices(343, 340, 340, 1.0, 1435, 1.0);s.store_div_from_scalar_offset_input(338, 1.0, 195, 1e-25);s.store_scaled_mul(335, 193, 338, (-s.v[514]));s.store_scaled_mul(337, 338, 338, s.v[514]);}
        s.b[2603] = (s.v[335] < (-34.0));s.store_scalar(2603, if s.b[2603] { 1.0 } else { 0.0 });
        if ((s.b[2599] && s.b[2600]) && (!s.b[2603])) {s.store_exp(336, 335);s.store_mul_scale_offset_mixed_ia(337, 334, A::div_from_scalar(s.v[513], s.ad_value(192)), 1.6021918e-19, 0.0);s.store_div_from_scalar(339, 1.0, 209);s.store_sqrt_ad(340, A::mul_offset_lhs(s.ad_value(978), (s.v[188] * 1e-12), s.ad_value(339)));s.store_mul3_lhs(338, 336, 337, 340);s.store_mul(339, 338, 195);s.store_mul(344, 339, 195);}
        if s.b[2599] {s.store_offset_scaled(334, 791, (-s.v[518]), s.v[559]);s.store_exp_scaled_input(336, 334, s.v[187]);s.store_scale(334, 791, (1.0 / (s.v[187]) * 1.0 / (s.v[187])));s.store_mul(337, 791, 334);s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));s.store_sub(335, 791, 790);s.store_offset_scaled(334, 335, (-s.v[518]), s.v[559]);s.store_exp_scaled_input(336, 334, s.v[187]);s.store_scale(334, 335, (1.0 / (s.v[187]) * 1.0 / (s.v[187])));s.store_mul(337, 335, 334);s.store_scalar(338, ((s.v[517] / 1000000.0) * s.v[632]));s.store_scaled_offset_ad(195, A::neg(A::sub(s.ad_value(791), s.ad_value(792))), ((s.v[160]) + (p.p258)), 1.0 / (s.v[187]));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_126(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[2599] {s.store_sqrt_square_offset(782, 195, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));s.store_offset_scaled_div(339, 195, 782, 0.5, 0.5);s.store_scaled_add(195, 195, 782, 0.5);}
        s.b[2606] = (s.v[195] < 0.0);s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });
        if (s.b[2599] && s.b[2606]) {s.store_scalar(195, 0.0);s.store_scalar(339, 0.0);}
        if s.b[2599] {s.store_offset(195, 195, 1e-25);s.store_div_from_scalar(335, (-s.v[520]), 195);}
        s.b[2607] = (s.v[335] < (-34.0));s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });
        if (s.b[2599] && (!s.b[2607])) {s.store_exp(336, 335);s.store_mul_div_from_scalar_lhs_ad_mixed_ai(337, s.v[520], A::square(s.ad_value(195)), 336);s.store_scale(337, 162, (s.v[519] * s.v[632]));}
        if s.b[2599] {s.copy_ad(285, 677);s.store_mul(286, 393, 285);s.store_scaled_offset_ad(336, A::add_scaled_inputs4(s.ad_value(1434), s.v[493], s.ad_value(1436), (-1.0), s.ad_value(122), 1.0, s.ad_value(174), 1.0), (-s.v[492]), (-1.0 / (s.v[187])));s.store_square(334, 336);s.store_scale(335, 286, s.v[491]);s.store_div_scaled_inputs_indices(337, 335, -1.0, 336, 1.0);}
        s.b[2608] = (s.v[337] < (-34.0));s.store_scalar(2608, if s.b[2608] { 1.0 } else { 0.0 });
        if (s.b[2599] && s.b[2608]) {s.store_scalar(339, 0.0);}
        if (s.b[2599] && (!s.b[2608])) {s.store_exp(339, 337);}
        if s.b[2599] {s.store_div_from_scalar(338, (((1.6021918e-19 * s.v[490]) * s.v[632]) * s.v[582]), 285);}
        s.b[2610] = (p.p25 != 0.0);s.store_scalar(2610, if s.b[2610] { 1.0 } else { 0.0 });
        if s.b[2610] {s.store_offset_ad(335, A::mul_sub_from_scalar_rhs(s.ad_value(790), 1.0, A::scale(s.ad_value(790), 100.0)), (-1e-5));s.store_sqrt_add_scaled_square_input(336, 335, 1.0, 790, (4.0 * 1e-5));s.store_add_scaled_inputs3_indices(196, 790, 1.0, 335, (-0.5), 336, (-0.5));}
        s.b[2611] = (p.p25 == 0.0);s.store_scalar(2611, if s.b[2611] { 1.0 } else { 0.0 });
        if (!s.b[2611]) {s.store_add_scaled_inputs4_offset_indices(335, 196, p.p242, 791, (-1.0), 122, p.p244, 174, p.p244, (p.p243 * p.p242));s.store_scalar(336, (1.0 / s.v[187]));s.store_mul(194, 335, 336);s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);s.store_scaled_add(197, 194, 782, 0.5);}
        s.b[2612] = (s.v[197] < 0.0);s.store_scalar(2612, if s.b[2612] { 1.0 } else { 0.0 });
        if ((!s.b[2611]) && s.b[2612]) {s.store_scalar(197, 0.0);s.store_scalar(339, 0.0);}
        if (!s.b[2611]) {s.store_div_from_scalar_offset_input(337, 1.0, 197, 1e-25);s.store_scaled_mul(334, 193, 337, (-s.v[512]));}
        s.b[2613] = (s.v[334] < (-34.0));s.store_scalar(2613, if s.b[2613] { 1.0 } else { 0.0 });
        if ((!s.b[2611]) && (!s.b[2613])) {s.store_exp(335, 334);s.store_scale_ad(336, A::div_from_scalar(s.v[511], s.ad_value(192)), (1.6021918e-19 * s.v[632]));}
        if (!s.b[2611]) {s.store_sub(205, 790, 792);}
        s.b[2614] = (s.v[205] > 0.0);s.store_scalar(2614, if s.b[2614] { 1.0 } else { 0.0 });
        if ((!s.b[2611]) && s.b[2614]) {s.store_square(336, 205);s.store_mul(338, 336, 205);s.store_offset(334, 338, 0.5);s.store_div(339, 338, 334);s.store_div_square_rhs_mixed_ai(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), 334);}
        s.b[2615] = (p.p25 == 0.0);s.store_scalar(2615, if s.b[2615] { 1.0 } else { 0.0 });
        if (!s.b[2615]) {s.store_add_scaled_inputs3_mixed_aii(335, A::add_scaled_inputs3_offset(s.ad_value(196), (-p.p242), s.ad_value(791), -1.0, s.ad_value(196), 1.0, ((p.p243) * (p.p242))), 1.0, 122, p.p244, 174, p.p244);s.store_scalar(336, (1.0 / s.v[187]));s.store_mul(194, 335, 336);s.store_sqrt_square_offset(782, 194, ((4.0 * (0.01 / 0.01)) * (0.01 / 0.01)));s.store_offset_scaled_div(339, 194, 782, 0.5, 0.5);s.store_scaled_add(198, 194, 782, 0.5);}
        s.b[2616] = (s.v[198] < 0.0);s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });
        if ((!s.b[2615]) && s.b[2616]) {s.store_scalar(198, 0.0);s.store_scalar(339, 0.0);}
        if (!s.b[2615]) {s.store_div_from_scalar_offset_input(337, 1.0, 198, 1e-25);s.store_scaled_mul(334, 193, 337, (-s.v[512]));}
        s.b[2617] = (s.v[334] < (-34.0));s.store_scalar(2617, if s.b[2617] { 1.0 } else { 0.0 });
        if ((!s.b[2615]) && (!s.b[2617])) {s.store_exp(335, 334);s.store_div_from_scalar(337, 1.0, 192);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_127(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((!s.b[2615]) && (!s.b[2617])) {s.store_scale(336, 337, (s.v[511] * (1.6021918e-19 * s.v[632])));}
        if (!s.b[2615]) {s.store_neg(206, 792);}
        s.b[2618] = (s.v[206] > 0.0);s.store_scalar(2618, if s.b[2618] { 1.0 } else { 0.0 });
        if ((!s.b[2615]) && s.b[2618]) {s.store_square(336, 206);s.store_mul(338, 336, 206);s.store_offset(334, 338, 0.5);s.store_div(339, 338, 334);s.store_div_square_rhs_mixed_ai(341, A::add_scaled_products(s.ad_value(336), s.ad_value(334), 3.0, s.ad_value(338), s.ad_value(336), (-3.0)), 334);}
        s.store_scalar(2619, 0.0);s.store_scalar(2622, 0.0);s.store_scalar(2621, 0.0);s.store_scalar(406, 0.0);s.store_scalar(2621, 0.0);s.b[2623] = (1.0 == 1.0);s.store_scalar(2623, if s.b[2623] { 1.0 } else { 0.0 });s.b[2624] = (1.0 == 2.0);s.store_scalar(2624, if s.b[2624] { 1.0 } else { 0.0 });s.b[2625] = (1.0 == 3.0);s.store_scalar(2625, if s.b[2625] { 1.0 } else { 0.0 });s.b[2626] = (1.0 == 4.0);s.store_scalar(2626, if s.b[2626] { 1.0 } else { 0.0 });s.b[2627] = (((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0));s.store_scalar(2627, if s.b[2627] { 1.0 } else { 0.0 });
        if (s.b[2623] && s.b[2627]) {s.store_scalar(2621, 1.0);s.store_scalar(2619, 1.0);s.store_sub(395, 731, 728);s.store_neg(396, 728);s.store_scalar(409, s.v[460]);s.store_scalar(407, p.p66);s.store_scalar(411, 0.0);s.copy_ad(410, 687);s.store_scalar(413, s.v[188]);}
        s.b[2628] = (((((p.p36 == 1.0) && (p.p66 > 0.0)) && (s.v[460] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));s.store_scalar(2628, if s.b[2628] { 1.0 } else { 0.0 });
        if ((s.b[2624] && (!s.b[2623])) && s.b[2628]) {s.store_scalar(2621, 1.0);s.store_sub(395, 734, 735);s.store_neg(396, 735);}
        s.b[2629] = (((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0));s.store_scalar(2629, if s.b[2629] { 1.0 } else { 0.0 });
        if ((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) {s.store_scalar(2621, 1.0);s.store_scalar(2622, 1.0);s.store_sub(395, 731, 728);s.store_sub(396, 729, 728);s.store_scalar(409, s.v[459]);s.store_scalar(407, (p.p63 + (p.p64 * p.p55)));s.copy_ad(411, 384);s.copy_ad(410, 686);s.copy_ad(413, 412);s.store_neg(407, 407);}
        s.b[2630] = (((s.v[407] < 0.0) && (p.p432 > 0.0)) && (p.p55 == 1.0));s.store_scalar(2630, if s.b[2630] { 1.0 } else { 0.0 });
        if (((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) {s.store_neg(407, 407);s.store_scalar(335, p.p63);s.store_offset_div_scaled_product_indices(996, 335, 335, 1.0, 651, 1.0, (-p.p137));}
        s.b[2631] = (p.p113 > 0.0);s.store_scalar(2631, if s.b[2631] { 1.0 } else { 0.0 });s.b[2632] = ((s.v[396] == 0.0) || (p.p113 <= 0.0));s.store_scalar(2632, if s.b[2632] { 1.0 } else { 0.0 });
        if (((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) && s.b[2632]) {
        }
        if (((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) && (!s.b[2632])) {s.store_scalar(783, (if (s.v[396] < 0.0) { (-1.0) } else { 1.0 }));}
        if (((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) && (!s.b[2632])) {s.store_mul(784, 783, 396);s.store_offset_powf_ad(781, A::div(s.ad_value(784), s.ad_value(996)), p.p113, 1.0);s.store_powf(782, 781, (1.0 / p.p113));s.store_div_scaled_product_indices(396, 783, 784, 1.0, 782, 1.0);}
        if ((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) {s.store_sqrt_offset_square_offset(782, 396, p.p137, ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(396), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(396), p.p137), 782, 0.5);}
        s.b[2633] = (s.v[336] < 0.0);s.store_scalar(2633, if s.b[2633] { 1.0 } else { 0.0 });
        if (((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) && s.b[2633]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.b[2625] && (!(s.b[2623] || s.b[2624]))) && s.b[2629]) && s.b[2630]) && s.b[2631]) {s.store_scaled_sqrt_mul(600, 651, 336, p.p432);s.store_sub(407, 407, 600);}
        s.b[2634] = (((((p.p35 == 1.0) && (p.p63 > 0.0)) && (s.v[459] > 0.0)) && (s.v[526] != 0.0)) && (p.p55 != 1.0));s.store_scalar(2634, if s.b[2634] { 1.0 } else { 0.0 });
        if ((s.b[2626] && (!((s.b[2623] || s.b[2624]) || s.b[2625]))) && s.b[2634]) {s.store_scalar(2621, 1.0);s.store_sub(395, 734, 735);s.store_sub(396, 733, 735);}
        if (s.v[2621] != 0.0) {s.store_scalar(2642, 0.4);s.store_scalar(2643, 0.0);s.store_scalar(223, 0.0);s.store_scalar(214, 0.0);s.store_scalar(216, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_128(
        s: &mut ReactiveScratch,
        p: &Parameters,
        param_given: &[bool; Instance::PARAMETER_COUNT],
    ) {
        if (s.v[2621] != 0.0) {s.store_scalar(232, 0.0);s.store_scalar(236, 0.0);s.store_scalar(233, 0.0);s.store_scalar(217, 0.0);s.store_scalar(420, 0.0);s.store_scalar(215, 0.0);s.store_scalar(447, 0.0);s.store_scalar(445, 0.0);s.store_scalar(446, 0.0);s.store_scalar(79, (-1.0));s.store_scalar(2644, 0.0);s.store_scalar(2645, 0.0);s.store_mul_scaled_ln_ad_rhs(2640, 155, 2.0, A::div(s.ad_value(409), s.ad_value(394)));s.store_offset_sub_from_scalar_ad(781, 0.8, s.ad_value(2640), (-0.1));s.store_scalar(782, ((4.0 * 0.8) * 0.1));}
        if (s.v[2621] != 0.0) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if (s.v[2621] != 0.0) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(2641, 781, (-0.5), 782, (-0.5), 0.8);}
        s.b[2647] = (s.v[2642] > (s.v[2641] * 0.5));s.store_scalar(2647, if s.b[2647] { 1.0 } else { 0.0 });
        if ((s.v[2621] != 0.0) && s.b[2647]) {s.store_scale(2642, 2641, 0.5);}
        s.b[2648] = param_given[338];s.store_scalar(2648, if s.b[2648] { 1.0 } else { 0.0 });
        if ((s.v[2621] != 0.0) && s.b[2648]) {s.store_scalar(2641, p.p338);}
        s.b[2649] = param_given[339];s.store_scalar(2649, if s.b[2649] { 1.0 } else { 0.0 });
        if ((s.v[2621] != 0.0) && s.b[2649]) {s.store_scalar(2642, p.p339);}
        s.b[2650] = param_given[338];s.store_scalar(2650, if s.b[2650] { 1.0 } else { 0.0 });
        if (((s.v[2621] != 0.0) && (!s.b[2649])) && s.b[2650]) {s.store_scale(2642, 2641, 0.5);}
        s.b[2651] = (s.v[2642] > (s.v[2641] * 0.5));s.store_scalar(2651, if s.b[2651] { 1.0 } else { 0.0 });
        if ((s.v[2621] != 0.0) && s.b[2651]) {s.store_scale(2642, 2641, 0.5);}
        s.b[2652] = (p.p38 == 1.0);s.store_scalar(2652, if s.b[2652] { 1.0 } else { 0.0 });
        if ((s.v[2621] != 0.0) && s.b[2652]) {s.store_neg(334, 396);}
        s.b[2653] = (s.v[334] > s.v[2642]);s.store_scalar(2653, if s.b[2653] { 1.0 } else { 0.0 });
        if (((s.v[2621] != 0.0) && s.b[2652]) && s.b[2653]) {s.store_sub(335, 334, 2642);s.store_sub(336, 2641, 2642);s.store_div(781, 335, 336);s.store_square(782, 781);s.store_mul(783, 782, 781);s.store_square(784, 782);s.store_div_from_scalar_ad(780, 1.0, A::add_scaled_inputs4_offset(s.ad_value(781), 1.0, s.ad_value(782), 1.0, s.ad_value(783), 1.0, s.ad_value(784), 1.0, 1.0));s.store_mul_ad_affine_product_lhs(345, A::add_scaled_inputs3_offset(s.ad_value(781), 2.0, s.ad_value(782), 3.0, s.ad_value(783), 4.0, 1.0), s.ad_value(780), -1.0, 0.0, 780);s.store_mul_scale_offset_indices(333, 336, 780, -1.0, 1.0);s.store_add_scaled_sub_value_product_indices(334, 1.0, 780, 1.0, 781, 345, 1.0);s.store_neg(345, 345);s.store_add(344, 2642, 333);}
        if (((s.v[2621] != 0.0) && s.b[2652]) && (!s.b[2653])) {s.copy_ad(344, 334);}
        if ((s.v[2621] != 0.0) && s.b[2652]) {s.store_neg(397, 344);}
        if ((s.v[2621] != 0.0) && (!s.b[2652])) {s.copy_ad(397, 396);}
        if (s.v[2621] != 0.0) {s.store_div(212, 410, 413);s.store_square(213, 212);s.store_sub_from_scalar(402, s.v[458], 395);s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);s.store_scalar(2636, 0.0);s.store_primal_scale(2637, 409, 1.6021918e-19);s.store_div(334, 394, 409);s.store_square(405, 334);}
        s.b[2654] = ((s.v[154] * (-s.v[397])) >= 500.0);s.store_scalar(2654, if s.b[2654] { 1.0 } else { 0.0 });
        if ((s.v[2621] != 0.0) && s.b[2654]) {s.store_scaled_offset_ad(229, A::mul_scaled_rhs(s.ad_value(154), s.ad_value(397), -1.0), ((1.0) + ((-500.0))), 1.403592217853e217);s.store_scalar(334, 1.403592217853e217);}
        if ((s.v[2621] != 0.0) && (!s.b[2654])) {s.store_mul_scale_offset_indices(781, 154, 397, -1.0, 0.0);s.store_scalar(229, 1.0);}
        let mut te: usize = 0;
        while {
            let td: f64 = if (((s.v[2621] != 0.0) && (!s.b[2654])) && (s.v[781] >= 60.0)) { 1.0 } else { 0.0 };
            td != 0.0
        } {
            te += 1;assert!(te <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((s.v[2621] != 0.0) && (!s.b[2654])) {s.store_scale(229, 229, 1.14200738981568e26);s.store_offset(781, 781, (-60.0));}
        }
        if ((s.v[2621] != 0.0) && (!s.b[2654])) {s.store_mul_exp_rhs(229, 229, 781);s.copy_ad(334, 229);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_129(
        s: &mut ReactiveScratch,
    ) {
        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {s.store_offset_scaled(781, 402, (-0.5), (((-0.5)) + ((-1.0))));s.store_scalar(782, (4.0 * 0.5));}
        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(334, 781, 782, 0.5, 0.5);s.store_offset_add_scaled_inputs_indices(335, 781, 0.5, 782, 0.5, 0.5);}
        s.b[2655] = (((s.v[402] + s.v[397]) > (-s.v[335])) && (s.v[335] >= 0.0));s.store_scalar(2655, if s.b[2655] { 1.0 } else { 0.0 });
        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {s.store_add_scaled_inputs3_indices(781, 402, 1.0, 397, 1.0, 335, 1.0);s.store_square(722, 781);s.store_square(723, 335);s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2656] = ((((1.0 == 1.0) || (1.0 == 2.0)) || (1.0 == 4.0)) || (1.0 == 8.0));s.store_scalar(2656, if s.b[2656] { 1.0 } else { 0.0 });s.b[2657] = (1.0 == 1.0);s.store_scalar(2657, if s.b[2657] { 1.0 } else { 0.0 });
        if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && s.b[2657]) {s.store_scalar(720, 1.0);}
        s.b[2658] = (1.0 == 2.0);s.store_scalar(2658, if s.b[2658] { 1.0 } else { 0.0 });
        if ((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && (!s.b[2657])) && s.b[2658]) {s.store_scalar(720, 2.0);}
        s.b[2659] = (1.0 == 4.0);s.store_scalar(2659, if s.b[2659] { 1.0 } else { 0.0 });
        if (((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && (!s.b[2657])) && (!s.b[2658])) && s.b[2659]) {s.store_scalar(720, 3.0);}
        s.b[2660] = (1.0 == 8.0);s.store_scalar(2660, if s.b[2660] { 1.0 } else { 0.0 });
        if ((((((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && (!s.b[2657])) && (!s.b[2658])) && (!s.b[2659])) && s.b[2660]) {s.store_scalar(720, 4.0);}
        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) {s.store_scalar(719, 0.0);}
        let mut t10: usize = 0;
        while {
            let tf: f64 = if (((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            tf != 0.0
        } {
            t10 += 1;assert!(t10 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && s.b[2656]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) && (!s.b[2656])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / 2.0));
            }
        }
        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_lhs(780, 781, 335, 726);s.store_div_scaled_product3_indices(334, 335, 725, 726, 1.0, 770, 1.0);s.store_add_scaled_inputs(335, 335, -1.0, 780, 1.0);}
        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && s.b[2655]) {
        }
        if (((s.v[2621] != 0.0) && (s.v[406] != 0.0)) && (!s.b[2655])) {s.store_add(335, 402, 397);s.store_scalar(334, 1.0);}
        if ((s.v[2621] != 0.0) && (s.v[406] != 0.0)) {s.store_sub(397, 335, 402);s.store_sub_from_scalar(403, (10.0 * 2.220446049250313e-16), 397);}
        s.b[2661] = (s.v[402] < s.v[403]);s.store_scalar(2661, if s.b[2661] { 1.0 } else { 0.0 });
        if ((s.v[2621] != 0.0) && s.b[2661]) {s.store_mul_scaled_ln_ad_rhs(271, 155, 2.0, A::div_from_scalar((-s.v[270]), s.ad_value(212)));s.store_mul_add_rhs(332, 154, 402, 397);s.store_div_scalar_by_product_indices(335, 1.0, 154, 410, 1.0);s.store_mul(333, 335, 413);s.store_offset_scaled(279, 333, (3.0 * 1.414213562373095), 2.0);s.store_mul3_affine_lhs(277, 279, 279, 8.0, 0.0, 279);s.store_sub_from_scalar_scaled_mul_mixed_ia(278, (7.0 * 1.414213562373095), 333, A::offset(s.ad_value(332), (-2.0)), 9.0);s.store_square(276, 278);}
        s.b[2662] = (s.v[277] < (s.v[276] * 1e-8));s.store_scalar(2662, if s.b[2662] { 1.0 } else { 0.0 });
        if (((s.v[2621] != 0.0) && s.b[2661]) && s.b[2662]) {s.store_div_scaled_inputs_indices(274, 277, 0.5, 278, 1.0);}
        if (((s.v[2621] != 0.0) && s.b[2661]) && (!s.b[2662])) {s.store_sqrt_add(275, 277, 276);s.store_sub(274, 275, 278);}
        if ((s.v[2621] != 0.0) && s.b[2661]) {s.store_powf(273, 274, 0.3333333333333333);s.store_add_scaled_inputs_product_mixed_aiii(272, A::sub_from_scalar(((-4.0) * 1.414213562373095), A::scale(s.ad_value(333), 12.0)), 1.0, 273, 2.0, 273, 273, 1.414213562373095);s.store_div(116, 272, 273);s.store_mul(335, 116, 155);s.store_div(336, 335, 271);s.store_sqrt_square_offset(337, 336, 1.0);s.store_sub_div_lhs_indices(404, 335, 337, 397);s.store_sub(336, 402, 404);s.store_mul(398, 413, 336);s.copy_ad(354, 398);s.copy_ad(2644, 404);}
        s.b[2663] = ((s.v[154] * (s.v[402] + s.v[397])) < (1.0 + (((((10.0 * 2.220446049250313e-16) - 1.0) * s.v[213]) * s.v[156]) / 4.0)));s.store_scalar(2663, if s.b[2663] { 1.0 } else { 0.0 });
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2663]) {s.store_add_scaled_product_indices(89, 402, 1.0, 213, 154, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_130(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2663])) {s.store_offset_div_scaled_offset_numerator(332, A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), 4.0, ((-1.0) * 4.0), A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));}
        if ((s.v[2621] != 0.0) && (!s.b[2661])) {s.store_mul_add_rhs(116, 154, 89, 397);}
        s.b[2664] = (s.v[116] >= 3.0);s.store_scalar(2664, if s.b[2664] { 1.0 } else { 0.0 });
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2664]) {s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);s.store_exp_neg_input(333, 116);s.store_offset_div_scaled_inputs2_mixed_aia(332, A::offset(A::mul(s.ad_value(154), A::add(s.ad_value(402), s.ad_value(397))), (-1.0)), 4.0, 333, 4.0, A::mul(s.ad_value(213), s.ad_value(156)), 1.0, 1.0);s.store_add_product3_rhs_mixed_iia(89, 402, 213, 154, A::sub_from_scalar(1.0, A::sqrt(s.ad_value(332))), 1.0 / (2.0));s.store_mul_add_rhs(116, 154, 89, 397);}
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2664])) {s.store_scalar(434, ((1.0 / (9.0 * ((2.0) as f64).sqrt())) - ((5.0 + (7.0 * (((-3.0)) as f64).exp())) / (54.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt()))));s.store_scalar(435, (((1.0 + (((-3.0)) as f64).exp()) / (2.0 * (((2.0 + (((-3.0)) as f64).exp())) as f64).sqrt())) - (((2.0) as f64).sqrt() / 3.0)));s.store_offset_div_from_scalar_ad(436, 1.0, A::mul(s.ad_value(154), s.ad_value(212)), (1.0 / ((2.0) as f64).sqrt()));s.store_div_scaled_inputs2_indices(437, 402, -1.0, 397, -1.0, 212, 1.0);s.store_add_scaled_inputs3_div_scaled_third_mixed_aaii(441, A::div_scaled_product(A::square(s.ad_value(435)), s.ad_value(435), 1.0, A::mul3_scaled_output(s.ad_value(434), s.ad_value(434), s.ad_value(434), 27.0), 1.0), 1.0, A::div_scaled_product_by_product(s.ad_value(435), s.ad_value(436), 1.0, s.ad_value(434), s.ad_value(434), 6.0), (-1.0), 437, 1.0, 434, 2.0, 1.0);s.store_div_scaled_value_by_product_mixed_aii(440, A::add_scaled_square_product(s.ad_value(435), (-1.0), s.ad_value(434), s.ad_value(436), 3.0), 1.0, 434, 434, 9.0);s.store_sqrt_add_scaled_square_cube_product(339, 441, 1.0, 440, 1.0);s.store_powf_ad(439, A::sub(s.ad_value(339), s.ad_value(441)), 0.3333333333333333);s.store_neg_powf_add_input(438, 441, 339, 0.3333333333333333);s.store_add_scaled_inputs3_div_scaled_third_indices(116, 439, 1.0, 438, 1.0, 435, 1.0, 434, 3.0, -1.0);s.store_add_scaled_product_indices(89, 397, (-1.0), 116, 155, 1.0);}
        s.b[2665] = (p.p33 > 0.0);s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {s.store_offset_add(442, 402, 397, 0.1);s.store_mul(222, 405, 229);s.store_mul(443, 405, 229);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_131(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {s.store_mul(334, 156, 213);s.store_mul(444, 154, 442);s.store_add_scaled_inputs_product_mixed_aaii(447, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), 1.0, A::ln(A::mul(s.ad_value(405), s.ad_value(334))), (-1.0), 154, 397, 1.0);}
        s.b[2666] = (p.p33 == 2.0);s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2666]) {s.store_offset_sub(781, 444, 447, (-1.0));s.store_scale(782, 444, 4.0);}
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2666]) {
            if (s.v[782] > 0.0) {
            } else {
                s.store_neg(782, 782);
            }
        }
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2666]) {s.store_sqrt_square_add(782, 781, 782);s.store_offset_scaled_div(335, 781, 782, 0.5, 0.5);s.store_add_scaled_inputs3_indices(447, 444, 1.0, 781, (-0.5), 782, (-0.5));}
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && (!s.b[2666])) {
            if (s.v[447] <= s.v[444]) {
            } else {
                s.copy_ad(447, 444);
            }
        }
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {
            if (s.v[447] >= 0.0) {
            } else {
                s.store_scalar(447, 0.0);
            }
        }
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {s.store_sub(444, 444, 447);s.store_add_scaled_inputs(444, 444, 1.0, 154, 0.1);s.store_sub_ad(335, A::ln(A::add_scaled_square_product(s.ad_value(444), 1.0, s.ad_value(443), s.ad_value(334), 1.0)), A::ln(A::mul(s.ad_value(405), s.ad_value(334))));s.store_add_scaled_product_indices(446, 335, 1.0, 154, 397, 1.0);}
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {
            if (s.v[446] >= 0.0) {
            } else {
                s.store_scalar(446, 0.0);
            }
        }
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) {s.copy_ad(445, 116);}
        s.b[2667] = (p.p33 == 2.0);s.store_scalar(2667, if s.b[2667] { 1.0 } else { 0.0 });s.b[2668] = ((s.v[445] > (s.v[446] - (0.2 * s.v[446]))) && ((0.2 * s.v[446]) >= 0.0));s.store_scalar(2668, if s.b[2668] { 1.0 } else { 0.0 });
        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {s.store_add_scaled_inputs3_indices(781, 445, 1.0, 446, (-1.0), 446, 0.2);s.store_square(722, 781);s.store_scaled_mul(723, 446, 446, (0.2 * 0.2));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2669] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2669, if s.b[2669] { 1.0 } else { 0.0 });s.b[2670] = (2.0 == 1.0);s.store_scalar(2670, if s.b[2670] { 1.0 } else { 0.0 });
        if (((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && s.b[2670]) {s.store_scalar(720, 1.0);}
        s.b[2671] = (2.0 == 2.0);s.store_scalar(2671, if s.b[2671] { 1.0 } else { 0.0 });
        if ((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && (!s.b[2670])) && s.b[2671]) {s.store_scalar(720, 2.0);}
        s.b[2672] = (2.0 == 4.0);s.store_scalar(2672, if s.b[2672] { 1.0 } else { 0.0 });
        if (((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && (!s.b[2670])) && (!s.b[2671])) && s.b[2672]) {s.store_scalar(720, 3.0);}
        s.b[2673] = (2.0 == 8.0);s.store_scalar(2673, if s.b[2673] { 1.0 } else { 0.0 });
        if ((((((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && (!s.b[2670])) && (!s.b[2671])) && (!s.b[2672])) && s.b[2673]) {s.store_scalar(720, 4.0);}
        if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) {s.store_scalar(719, 0.0);}
        let mut t12: usize = 0;
        while {
            let t11: f64 = if (((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) && (s.v[719] < s.v[720])) { 1.0 } else { 0.0 };
            t11 != 0.0
        } {
            t12 += 1;assert!(t12 <= Self::MAX_ANALOG_LOOP_ITERATIONS, "generated Verilog-A analog loop exceeded iteration guard");
            if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && s.b[2669]) {s.store_sqrt(726, 726);s.store_primal_offset(719, 719, 1.0);}
        }
        if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) && (!s.b[2669])) {
            if (s.v[726] == 0.0) {
                s.store_scalar(726, 0.0);
            } else {
                s.store_powf(726, 726, (1.0 / (2.0 * 2.0)));
            }
        }
        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {s.store_div_from_scalar(726, 1.0, 726);s.store_mul3_affine_lhs(780, 781, 446, 0.2, 0.0, 726);s.store_div_scaled_product3_indices(335, 446, 725, 726, 0.2, 770, 1.0);s.store_add_scaled_inputs3_indices(116, 446, 1.0, 446, (-0.2), 780, 1.0);}
        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && s.b[2668]) {
        }
        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && s.b[2667]) && (!s.b[2668])) {s.copy_ad(116, 445);s.store_scalar(335, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_132(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2665]) && (!s.b[2667])) {
            if (s.v[445] <= s.v[446]) {
                s.copy_ad(116, 445);
            } else {
                s.copy_ad(116, 446);
            }
        }
        s.b[2674] = (p.p33 == 1.0);s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[2675] = (s.v[411] > 0.0);s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2675]) {s.store_sub_from_scalar(336, p.p334, 411);}
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2675])) {s.store_sqrt_offset_square_offset(782, 729, p.p137, ((4.0 * 0.1) * 0.1));s.store_scaled_offset_ad(343, A::div_scaled_offset_numerator(s.ad_value(729), 1.0, p.p137, s.ad_value(782), 1.0), 1.0, 0.5);s.store_scaled_add_mixed_ai(336, A::offset(s.ad_value(729), p.p137), 782, 0.5);}
        s.b[2676] = (s.v[336] < 0.0);s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });
        if (((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2675])) && s.b[2676]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2675])) {s.store_scaled_sqrt_mul(600, 651, 336, p.p432);s.store_sub_from_scalar(336, p.p334, 600);}
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) {s.store_sqrt_square_offset(782, 336, ((4.0 * (p.p334 * 0.01)) * (p.p334 * 0.01)));s.store_offset_scaled_div(343, 336, 782, 0.5, 0.5);s.store_scaled_add(336, 336, 782, 0.5);}
        s.b[2677] = (s.v[336] < 0.0);s.store_scalar(2677, if s.b[2677] { 1.0 } else { 0.0 });
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2677]) {s.store_scalar(336, 0.0);s.store_scalar(343, 0.0);}
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) {s.copy_ad(386, 336);s.store_mul3_affine_lhs(418, 2637, 386, (0.5 * 9662367879.197212), 0.0, 386);s.store_sqrt_mul_scaled_lhs(334, 154, 2.0, 418);s.store_scaled_add_ad(335, A::limited_exp(s.ad_value(334)), A::limited_exp_scaled_input(s.ad_value(334), -1.0), 0.5);s.store_div_ln_lhs(419, 335, 418);s.store_add(414, 404, 397);s.store_mul_sub_rhs(333, 419, 414, 418);}
        s.b[2678] = (s.v[333] < 60.0);s.store_scalar(2678, if s.b[2678] { 1.0 } else { 0.0 });
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2678]) {s.store_exp(335, 333);s.store_exp_mul_scaled_lhs_indices(334, 419, -1.0, 418);s.store_sub(336, 335, 334);s.store_div_ln_offset_lhs(416, 336, 1.0, 419);}
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && (!s.b[2678])) {s.store_sub(416, 414, 418);}
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) {s.store_mul(415, 154, 416);}
        s.b[2679] = ((s.v[415] > (s.v[116] / 100.0)) && (s.v[415] > 0.0));s.store_scalar(2679, if s.b[2679] { 1.0 } else { 0.0 });
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2674]) && s.b[2679]) {s.store_primal_offset(2643, 2643, 1.0);s.copy_ad(116, 447);}
        if ((s.v[2621] != 0.0) && (!s.b[2661])) {s.store_add_scaled_product_indices(404, 397, (-1.0), 116, 155, 1.0);}
        s.b[2680] = (((s.v[116]) as f64).abs() > 1e-6);s.store_scalar(2680, if s.b[2680] { 1.0 } else { 0.0 });
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2680]) {s.store_add_offset_lhs_mixed_ia(335, 116, (-1.0), A::exp_scaled_input(s.ad_value(116), -1.0));s.store_sqrt(336, 335);}
        if (((s.v[2621] != 0.0) && (!s.b[2661])) && (!s.b[2680])) {s.store_mul_scaled_sqrt_ad_rhs(336, 116, 0.7071067811865475, A::sub_from_scalar(1.0, A::scale(s.ad_value(116), 0.3333333333333333)));}
        if ((s.v[2621] != 0.0) && (!s.b[2661])) {s.store_mul(354, 410, 336);s.store_mul_sub_rhs(398, 413, 402, 404);s.store_div(2681, 354, 2637);}
        s.b[2683] = (p.p33 == 2.0);s.store_scalar(2683, if s.b[2683] { 1.0 } else { 0.0 });s.b[2684] = ((s.v[2681] > (s.v[386] - (s.v[386] * 0.1))) && ((s.v[386] * 0.1) >= 0.0));s.store_scalar(2684, if s.b[2684] { 1.0 } else { 0.0 });
        if ((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) {s.store_add_scaled_inputs3_indices(781, 2681, 1.0, 386, (-1.0), 386, 0.1);s.store_square(722, 781);s.store_scaled_mul(723, 386, 386, (0.1 * 0.1));s.store_scalar(724, 1.0);s.store_scalar(725, 1.0);s.store_scalar(719, 0.0);s.store_scalar(720, 0.0);s.store_scalar(770, 0.0);s.store_scalar(726, 0.0);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_mul(724, 724, 722);s.store_mul(725, 725, 723);s.store_add(770, 724, 725);s.copy_ad(726, 770);}
        s.b[2685] = ((((2.0 == 1.0) || (2.0 == 2.0)) || (2.0 == 4.0)) || (2.0 == 8.0));s.store_scalar(2685, if s.b[2685] { 1.0 } else { 0.0 });s.b[2686] = (2.0 == 1.0);s.store_scalar(2686, if s.b[2686] { 1.0 } else { 0.0 });
        if ((((((s.v[2621] != 0.0) && (!s.b[2661])) && s.b[2683]) && s.b[2684]) && s.b[2685]) && s.b[2686]) {s.store_scalar(720, 1.0);}
    }
}
