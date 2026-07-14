#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_128(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2605])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) {s.store_mul_mixed_ai(2510, A::add_scaled_inputs_product(s.ad_value(2509), 0.29214664, A::square(s.ad_value(2509)), s.v[367], A::square(s.ad_value(2509)), s.ad_value(2509), s.v[368]), 2526);}
        s.b[2606] = (s.v[2547] > 0.0);s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && s.b[2606]) {s.copy_ad(2548, 2510);}
        s.b[2607] = (s.v[2546] > (-230.25850929940458));s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2606])) && s.b[2607]) {s.store_exp(2526, 2546);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2606])) && (!s.b[2607])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2606])) {s.store_sub_scaled_inputs(2548, 2526, 2.0, 2510, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) {s.store_div_scaled_inputs_indices(2549, 2548, (s.v[430] * (1.772453850905516 * 0.5)), 2544, 1.0);s.store_mul3_affine_lhs(2535, 2534, 2549, p.p839, 0.0, 2543);}
        s.b[2608] = (p.p845 == 0.0);s.store_scalar(2608, if s.b[2608] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && s.b[2608]) {s.store_scalar(2550, 0.0);}
        s.b[2609] = (p.p825 == 0.5);s.store_scalar(2609, if s.b[2609] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && s.b[2609]) {s.store_sqrt_scaled_input_ad(2526, A::sub_from_scalar(p.p822, s.ad_value(2524)), s.v[424]);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2609])) {s.store_powf_scale_offset_input(2526, 2524, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) {s.store_div_scaled_offset_numerator_indices(2551, 2524, ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), 2526, 1.0);}
        s.b[2610] = (((((-s.v[436]) / s.v[2551])) as f64).abs() < 230.25850929940458);s.store_scalar(2610, if s.b[2610] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && s.b[2610]) {s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0));}
        s.b[2611] = (((-s.v[436]) / s.v[2551]) < 0.0);s.store_scalar(2611, if s.b[2611] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2610])) && s.b[2611]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 436, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2610])) && (!s.b[2611])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 436, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) {s.store_mul_scale_offset_mixed_ai(2550, A::mul3(s.ad_value(826), s.ad_value(2551), s.ad_value(2551)), 2526, p.p845, 0.0);}
        s.b[2612] = (p.p854 > 1000.0);s.store_scalar(2612, if s.b[2612] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && s.b[2612]) {s.store_scalar(2552, 1.0);}
        s.b[2613] = (s.v[2525] > ((-s.v[438]) * p.p854));s.store_scalar(2613, if s.b[2613] { 1.0 } else { 0.0 });s.b[2614] = (p.p857 == 4.0);s.store_scalar(2614, if s.b[2614] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && s.b[2613]) && s.b[2614]) {s.store_mul_scale_offset_mixed_ai(2526, A::mul3_scaled_output(s.ad_value(2525), s.ad_value(2525), s.ad_value(2525), ((s.v[443] * s.v[443]) * s.v[443])), 2525, s.v[443], 0.0);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && s.b[2613]) && (!s.b[2614])) {s.store_powf_ad(2526, A::abs_scaled_input(s.ad_value(2525), s.v[443]), p.p857);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && s.b[2613]) {s.store_div_from_scalar_sub_from_scalar_ad(2552, 1.0, 1.0, s.ad_value(2526));}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && (!s.b[2613])) {s.store_offset_scaled(2552, 2525, s.v[446], (((((s.v[438] * p.p854)) * (s.v[446]))) + (s.v[440])));}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) {s.store_mul_scale_offset_mixed_ia(1901, 2552, A::add_scaled_inputs4(s.ad_value(2527), 1.0, s.ad_value(2528), 1.0, s.ad_value(2535), 1.0, s.ad_value(2550), 1.0), p.p29, 0.0);}
        s.b[2615] = (s.v[403] == 0.5);s.store_scalar(2615, if s.b[2615] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && s.b[2615]) {s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[400]));}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2615])) {s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[400])), s.v[403]);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) {s.store_add_scaled_inputs3_offset_indices(1907, 2526, ((-s.v[412]) * p.p30), 826, (s.v[415] * p.p30), 2518, ((-s.v[415]) * p.p30), (s.v[412] * p.p30));}
        s.b[2616] = (s.v[642] == 0.0);s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2616]) {s.store_scalar(1902, 0.0);s.store_scalar(1908, 0.0);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) {s.store_scale(2527, 2517, s.v[383]);}
        s.b[2617] = ((p.p835 == 0.0) && (p.p840 == 0.0));s.store_scalar(2617, if s.b[2617] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2617]) {s.store_scalar(2528, 0.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) {s.store_sub_from_scalar(2529, s.v[389], 2523);s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_129(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2618] = (p.p826 == 0.5);s.store_scalar(2618, if s.b[2618] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && s.b[2618]) {s.store_scalar(2531, 0.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && (!s.b[2618])) {s.store_scaled_add_mixed_ai(2531, A::div_scaled_product(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2530)), 1.0), 2530, (1.0 - (2.0 * p.p826)));}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) {s.store_add(2532, 2530, 2531);}
        s.b[2619] = (p.p826 == 0.5);s.store_scalar(2619, if s.b[2619] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && s.b[2619]) {s.store_sqrt_scaled_input(2526, 2529, s.v[425]);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && (!s.b[2619])) {s.store_powf_scaled_input(2526, 2529, s.v[425], p.p826);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) {s.store_scale(2533, 2526, s.v[419]);s.store_mul_scale_offset_indices(2534, 2533, 2520, s.v[380], ((-1.0)) * (s.v[380]));s.store_scaled_mul(2528, 2534, 2532, p.p835);}
        s.b[2620] = (p.p840 == 0.0);s.store_scalar(2620, if s.b[2620] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2620]) {s.store_scalar(2535, 0.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) {s.store_div_scaled_inputs_indices(2536, 2533, (s.v[404] * s.v[434]), 2529, 1.0);s.store_div_from_scalar(2537, (0.666666666666667 * s.v[431]), 2536);s.store_square(2538, 2537);s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);s.store_sqrt(2540, 2539);s.store_mul(2541, 2539, 2540);}
        s.b[2621] = (((-p.p826) * s.v[407]) == (-1.0));s.store_scalar(2621, if s.b[2621] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2621]) {s.store_div_from_scalar_offset_product(2542, 1.0, 2536, 2541, 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2621])) {s.store_powf_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), ((-p.p826) * s.v[407]));}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) {s.store_div_scaled_product_add_scaled_denominator_indices(2543, 2532, 2542, 1.0, 2532, 1.0, 2542, 1.0, 1.0);s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);s.store_add_scaled_value_products_indices(2546, 2539, (-s.v[431]), 2537, 2540, s.v[431], 2536, 2541, 0.5);s.store_mul_scale_offset_indices(2547, 2544, 2545, 1.0, (-1.0));s.store_square(2508, 2547);}
        s.b[2622] = (s.v[2547] > 0.0);s.store_scalar(2622, if s.b[2622] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2622]) {s.store_div_from_scalar_offset_scaled_input(2509, 1.0, 2547, s.v[366], 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2622])) {s.store_div_from_scalar_sub_from_scalar_ad(2509, 1.0, 1.0, A::scale(s.ad_value(2547), s.v[366]));}
        s.b[2623] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));s.store_scalar(2623, if s.b[2623] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2623]) {s.store_exp_sub(2526, 2546, 2508);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2623])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) {s.store_mul_mixed_ai(2510, A::add_scaled_inputs_product(s.ad_value(2509), 0.29214664, A::square(s.ad_value(2509)), s.v[367], A::square(s.ad_value(2509)), s.ad_value(2509), s.v[368]), 2526);}
        s.b[2624] = (s.v[2547] > 0.0);s.store_scalar(2624, if s.b[2624] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2624]) {s.copy_ad(2548, 2510);}
        s.b[2625] = (s.v[2546] > (-230.25850929940458));s.store_scalar(2625, if s.b[2625] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2624])) && s.b[2625]) {s.store_exp(2526, 2546);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2624])) && (!s.b[2625])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2624])) {s.store_sub_scaled_inputs(2548, 2526, 2.0, 2510, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) {s.store_div_scaled_inputs_indices(2549, 2548, (s.v[431] * (1.772453850905516 * 0.5)), 2544, 1.0);s.store_mul3_affine_lhs(2535, 2534, 2549, p.p840, 0.0, 2543);}
        s.b[2626] = (p.p846 == 0.0);s.store_scalar(2626, if s.b[2626] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2626]) {s.store_scalar(2550, 0.0);}
        s.b[2627] = (p.p826 == 0.5);s.store_scalar(2627, if s.b[2627] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && s.b[2627]) {s.store_sqrt_scaled_input_ad(2526, A::sub_from_scalar(p.p823, s.ad_value(2524)), s.v[425]);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2627])) {s.store_powf_scale_offset_input(2526, 2524, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) {s.store_div_scaled_offset_numerator_indices(2551, 2524, ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), 2526, 1.0);}
        s.b[2628] = (((((-s.v[437]) / s.v[2551])) as f64).abs() < 230.25850929940458);s.store_scalar(2628, if s.b[2628] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && s.b[2628]) {s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0));}
        s.b[2629] = (((-s.v[437]) / s.v[2551]) < 0.0);s.store_scalar(2629, if s.b[2629] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2628])) && s.b[2629]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 437, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2628])) && (!s.b[2629])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 437, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_130(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) {s.store_mul_scale_offset_mixed_ai(2550, A::mul3(s.ad_value(826), s.ad_value(2551), s.ad_value(2551)), 2526, p.p846, 0.0);}
        s.b[2630] = (s.v[634] > 1000.0);s.store_scalar(2630, if s.b[2630] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2630]) {s.store_scalar(2552, 1.0);}
        s.b[2631] = (s.v[2525] > ((-s.v[438]) * s.v[634]));s.store_scalar(2631, if s.b[2631] { 1.0 } else { 0.0 });s.b[2632] = (p.p858 == 4.0);s.store_scalar(2632, if s.b[2632] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && s.b[2631]) && s.b[2632]) {s.store_mul_ad_product_lhs_mixed_ai(2526, A::mul3(A::square(A::mul(s.ad_value(2525), s.ad_value(444))), s.ad_value(2525), s.ad_value(444)), 2525, 444);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && s.b[2631]) && (!s.b[2632])) {s.store_powf_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(444))), p.p858);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && s.b[2631]) {s.store_div_from_scalar_sub_from_scalar_ad(2552, 1.0, 1.0, s.ad_value(2526));}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && (!s.b[2631])) {s.store_offset_mul_ad(2552, A::add_scaled_inputs(s.ad_value(2525), 1.0, s.ad_value(634), s.v[438]), s.ad_value(447), s.v[441]);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) {s.store_mul_scale_offset_mixed_ia(1902, 2552, A::add_scaled_inputs4(s.ad_value(2527), 1.0, s.ad_value(2528), 1.0, s.ad_value(2535), 1.0, s.ad_value(2550), 1.0), p.p29, 0.0);}
        s.b[2633] = (s.v[467] == 1.0);s.store_scalar(2633, if s.b[2633] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {
            if (s.v[826] < p.p863) {
                if (((s.v[826] - p.p863) / p.p864) < (-37.0)) {
                    s.store_scalar(2553, p.p863);
                } else {
                    s.store_offset_scaled_ad(2553, A::ln_one_plus_exp(A::scaled_offset(s.ad_value(826), (-p.p863), 1.0 / (p.p864))), p.p864, p.p863);
                }
            } else {
                if (((s.v[826] - p.p863) / p.p864) > 37.0) {
                    s.copy_ad(2553, 826);
                } else {
                    s.store_add_scaled_inputs_mixed_ia(2553, 826, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(826), (-1.0 / (p.p864)), ((p.p863) * (1.0 / (p.p864))))), p.p864);
                }
            }
        }
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {s.store_primal_scaled_mul(2511, 651, 651, 4.0);s.store_primal_div(2512, 651, 652);s.store_add_scaled_product_indices(2513, 2553, 1.0, 651, 2512, 1.0);s.store_add(2514, 652, 2513);s.store_sub(2515, 652, 2513);s.store_sqrt_square_add(2516, 2515, 2511);s.store_div_scaled_product_add_scaled_denominator_indices(2554, 2553, 652, 2.0, 2514, 1.0, 2516, 1.0, 1.0);}
        s.b[2634] = (s.v[404] == 0.5);s.store_scalar(2634, if s.b[2634] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && s.b[2634]) {s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2554), s.v[401]));}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && (!s.b[2634])) {s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2554), s.v[401])), s.v[404]);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {s.store_add_scaled_inputs3_offset_indices(1908, 2526, ((-s.v[413]) * p.p30), 2553, (s.v[416] * p.p30), 2554, ((-s.v[416]) * p.p30), (s.v[413] * p.p30));s.store_sub_offset_lhs(2553, 826, p.p863, 2553);s.store_primal_scaled_mul(2511, 651, 651, 4.0);s.store_primal_div(2512, 651, 652);s.store_add_scaled_product_indices(2513, 2553, 1.0, 651, 2512, 1.0);s.store_add(2514, 652, 2513);s.store_sub(2515, 652, 2513);s.store_sqrt_square_add(2516, 2515, 2511);s.store_div_scaled_product_add_scaled_denominator_indices(2554, 2553, 652, 2.0, 2514, 1.0, 2516, 1.0, 1.0);}
        s.b[2635] = (s.v[461] == 0.5);s.store_scalar(2635, if s.b[2635] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && s.b[2635]) {s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2554), s.ad_value(460)));}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) && (!s.b[2635])) {s.store_pow_sub_from_scalar_mul_base_indices(2526, 1.0, 2554, 460, 461);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {s.store_add_scaled_product_mixed_aia(466, A::mul_sub_from_scalar_rhs(s.ad_value(464), 1.0, s.ad_value(2526)), p.p30, 465, A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_131(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {s.store_add(1908, 1908, 466);}
        s.b[2636] = (s.v[404] == 0.5);s.store_scalar(2636, if s.b[2636] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) && s.b[2636]) {s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[401]));}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) && (!s.b[2636])) {s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[401])), s.v[404]);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) {s.store_add_scaled_inputs3_offset_indices(1908, 2526, ((-s.v[413]) * p.p30), 826, (s.v[416] * p.p30), 2518, ((-s.v[416]) * p.p30), (s.v[413] * p.p30));}
        if (s.b[2555] && (!s.b[2556])) {s.store_add_scaled_products3_indices(842, 640, 1900, 1.0, 641, 1901, 1.0, 642, 1902, 1.0);}
        s.b[2637] = (s.v[630] > 0.0);s.store_scalar(2637, if s.b[2637] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2637]) {s.store_mul_sub_mixed_iaa(637, 630, A::pow(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt_square_offset(A::add(s.ad_value(819), s.ad_value(821)), (0.001 * 0.001)), 0.5), s.ad_value(631)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(631)));s.store_add(635, 536, 637);s.store_div_from_scalar(610, 1.0, 635);s.store_div_scaled_value_offset_denominator(613, s.ad_value(613), 1.0, A::div(s.ad_value(637), s.ad_value(536)), 1.0, 1.0);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2637])) {s.copy_ad(635, 536);}
        s.b[2638] = (s.v[632] > 0.0);s.store_scalar(2638, if s.b[2638] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2638]) {s.store_mul_sub_mixed_iaa(639, 632, A::pow(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt_square_offset(A::add(s.ad_value(819), s.ad_value(821)), (0.001 * 0.001)), 0.5), s.ad_value(633)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(633)));s.store_mul_scale_offset_indices(604, 604, 639, 1.0, 1.0);}
        if (s.b[2555] && (!s.b[2556])) {s.store_scalar(2524, 0.0);s.store_scalar(2521, 0.0);}
        s.b[2639] = (!(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)));s.store_scalar(2639, if s.b[2639] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {s.store_primal_scaled_mul(2511, 678, 678, 4.0);s.store_primal_div(2512, 678, 679);s.store_add_scaled_product_indices(2513, 827, 1.0, 678, 2512, 1.0);s.store_add(2514, 679, 2513);s.store_sub(2515, 679, 2513);s.store_sqrt_square_add(2516, 2515, 2511);s.store_div_scaled_product_add_scaled_denominator_indices(2518, 827, 679, 2.0, 2514, 1.0, 2516, 1.0, 1.0);}
        s.b[2640] = (s.v[827] < s.v[675]);s.store_scalar(2640, if s.b[2640] { 1.0 } else { 0.0 });s.b[2641] = (((((-0.5) * (s.v[827] * s.v[365]))) as f64).abs() < 230.25850929940458);s.store_scalar(2641, if s.b[2641] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) && s.b[2641]) {s.store_exp_scaled_input(2519, 827, (s.v[365] * (-0.5)));}
        s.b[2642] = (((-0.5) * (s.v[827] * s.v[365])) < 0.0);s.store_scalar(2642, if s.b[2642] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) && (!s.b[2641])) && s.b[2642]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2519, 1e-100, (-230.25850929940458), A::scale(s.ad_value(827), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) && (!s.b[2641])) && (!s.b[2642])) {s.store_scaled_offset_ad(2519, A::mul_offset_rhs(A::scale_offset(s.ad_value(827), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(827), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(827), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2640]) {s.store_div_from_scalar(2520, 1.0, 2519);s.store_square(2517, 2520);}
        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && (!s.b[2640])) {s.store_mul_scale_offset_mixed_ia(2517, 676, A::sub_scaled_inputs(s.ad_value(827), s.v[365], s.ad_value(675), s.v[365]), 1.0, 1.0);s.store_sqrt(2520, 2517);s.store_div_from_scalar(2519, 1.0, 2520);}
        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {s.store_offset(2517, 2517, (-1.0));}
        s.b[2643] = (s.v[827] > 0.0);s.store_scalar(2643, if s.b[2643] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_132(
        s: &mut Scratch,
    ) {
        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2643]) {s.store_scaled_ln_ad(2521, A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2519), 1.0, A::offset(s.ad_value(2519), 3.0)))), (s.v[364] * 2.0));}
        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && (!s.b[2643])) {s.store_sub_mixed_ai(2521, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2520), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2520), 1.0, A::scale_offset(s.ad_value(2520), 3.0, 1.0))))), (s.v[364] * 2.0)), 827);}
        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {s.store_sub(2522, 677, 2521);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2523, 827, 0.5, 2522, 0.5, 827, 2522, ((4.0 * s.v[364]) * s.v[364]), (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2524, 827, 0.5, 680, 0.5, 827, 680, ((4.0 * s.v[362]) * s.v[362]), (-0.5));s.store_scaled_sub_mixed_ia(2525, 827, A::sqrt_square_offset(s.ad_value(827), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[2644] = (s.v[667] == 0.0);s.store_scalar(2644, if s.b[2644] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2644]) {s.store_scalar(1903, 0.0);s.store_scalar(1909, 0.0);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) {s.store_mul(2527, 557, 2517);}
        s.b[2645] = ((s.v[516] == 0.0) && (s.v[519] == 0.0));s.store_scalar(2645, if s.b[2645] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && s.b[2645]) {s.store_scalar(2528, 0.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) {s.store_sub(2529, 563, 2523);s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));}
        s.b[2646] = (s.v[505] == 0.5);s.store_scalar(2646, if s.b[2646] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) && s.b[2646]) {s.store_scalar(2531, 0.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) && (!s.b[2646])) {s.store_mul_scale_offset(2531, A::add(A::div_scaled_product(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2530)), 1.0), s.ad_value(2530)), A::scale(s.ad_value(505), 2.0), -1.0, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) {s.store_add(2532, 2530, 2531);}
        s.b[2647] = (s.v[505] == 0.5);s.store_scalar(2647, if s.b[2647] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) && s.b[2647]) {s.store_sqrt_mul(2526, 2529, 590);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) && (!s.b[2647])) {s.store_pow_mul_base_indices(2526, 2529, 590, 505);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2645])) {s.store_mul(2533, 584, 2526);s.store_mul_ad_product_lhs_mixed_ia(2534, 554, A::offset(s.ad_value(2520), (-1.0)), 2533);s.store_mul3_lhs(2528, 516, 2534, 2532);}
        s.b[2648] = (s.v[519] == 0.0);s.store_scalar(2648, if s.b[2648] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && s.b[2648]) {s.store_scalar(2535, 0.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) {s.store_mul_div_scaled_product_indices(2536, 599, 2533, 569, 1.0, 2529, 1.0);s.store_div_scaled_inputs_indices(2537, 596, 0.666666666666667, 2536, 1.0);s.store_square(2538, 2537);s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);s.store_sqrt(2540, 2539);s.store_mul(2541, 2539, 2540);}
        s.b[2649] = (((-s.v[505]) * s.v[572]) == (-1.0));s.store_scalar(2649, if s.b[2649] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && s.b[2649]) {s.store_div_from_scalar_offset_product(2542, 1.0, 2536, 2541, 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2649])) {s.store_pow_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), A::mul_scaled_lhs(s.ad_value(505), -1.0, s.ad_value(572)));}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) {s.store_div_scaled_product_add_scaled_denominator_indices(2543, 2532, 2542, 1.0, 2532, 1.0, 2542, 1.0, 1.0);s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_133(
        s: &mut Scratch,
    ) {
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) {s.store_add_scaled_value_products_mixed_aiiii(2546, A::mul3(s.ad_value(596), s.ad_value(2537), s.ad_value(2540)), 1.0, 596, 2539, (-1.0), 2536, 2541, 0.5);s.store_mul_scale_offset_indices(2547, 2544, 2545, 1.0, (-1.0));s.store_square(2508, 2547);}
        s.b[2650] = (s.v[2547] > 0.0);s.store_scalar(2650, if s.b[2650] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && s.b[2650]) {s.store_div_from_scalar_offset_scaled_input(2509, 1.0, 2547, s.v[366], 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2650])) {s.store_div_from_scalar_sub_from_scalar_ad(2509, 1.0, 1.0, A::scale(s.ad_value(2547), s.v[366]));}
        s.b[2651] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));s.store_scalar(2651, if s.b[2651] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && s.b[2651]) {s.store_exp_sub(2526, 2546, 2508);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2651])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) {s.store_mul_mixed_ai(2510, A::add_scaled_inputs_product(s.ad_value(2509), 0.29214664, A::square(s.ad_value(2509)), s.v[367], A::square(s.ad_value(2509)), s.ad_value(2509), s.v[368]), 2526);}
        s.b[2652] = (s.v[2547] > 0.0);s.store_scalar(2652, if s.b[2652] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && s.b[2652]) {s.copy_ad(2548, 2510);}
        s.b[2653] = (s.v[2546] > (-230.25850929940458));s.store_scalar(2653, if s.b[2653] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2652])) && s.b[2653]) {s.store_exp(2526, 2546);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2652])) && (!s.b[2653])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) && (!s.b[2652])) {s.store_sub_scaled_inputs(2548, 2526, 2.0, 2510, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2648])) {s.store_div_scaled_product_indices(2549, 596, 2548, (1.772453850905516 * 0.5), 2544, 1.0);s.store_mul_product3_indices(2535, 519, 2534, 2549, 2543, 1.0);}
        s.b[2654] = (s.v[525] == 0.0);s.store_scalar(2654, if s.b[2654] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && s.b[2654]) {s.store_scalar(2550, 0.0);}
        s.b[2655] = (s.v[505] == 0.5);s.store_scalar(2655, if s.b[2655] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && s.b[2655]) {s.store_sqrt_mul_sub_lhs(2526, 502, 2524, 590);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && (!s.b[2655])) {s.store_pow_mul_base_mixed_ai(2526, A::sub(s.ad_value(502), s.ad_value(2524)), 590, 505);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) {s.store_mul_div_scaled_product_mixed_iaii(2551, 572, A::sub(s.ad_value(502), s.ad_value(2524)), 587, 1.0, 2526, 1.0);}
        s.b[2656] = (((((-s.v[602]) / s.v[2551])) as f64).abs() < 230.25850929940458);s.store_scalar(2656, if s.b[2656] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && s.b[2656]) {s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(602), -1.0, s.ad_value(2551), 1.0));}
        s.b[2657] = (((-s.v[602]) / s.v[2551]) < 0.0);s.store_scalar(2657, if s.b[2657] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && (!s.b[2656])) && s.b[2657]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 602, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) && (!s.b[2656])) && (!s.b[2657])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 602, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2654])) {s.store_mul_ad_product_lhs_mixed_ia(2550, 525, A::mul3(s.ad_value(827), s.ad_value(2551), s.ad_value(2551)), 2526);}
        s.b[2658] = (s.v[534] > 1000.0);s.store_scalar(2658, if s.b[2658] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && s.b[2658]) {s.store_scalar(2552, 1.0);}
        s.b[2659] = (s.v[2525] > ((-s.v[438]) * s.v[534]));s.store_scalar(2659, if s.b[2659] { 1.0 } else { 0.0 });s.b[2660] = (s.v[537] == 4.0);s.store_scalar(2660, if s.b[2660] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2658])) && s.b[2659]) && s.b[2660]) {s.store_mul_ad_product_lhs_mixed_ai(2526, A::mul3(A::square(A::mul(s.ad_value(2525), s.ad_value(608))), s.ad_value(2525), s.ad_value(608)), 2525, 608);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2658])) && s.b[2659]) && (!s.b[2660])) {s.store_pow_abs_mul_base_indices(2526, 2525, 608, 537);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2658])) && s.b[2659]) {s.store_div_from_scalar_sub_from_scalar_ad(2552, 1.0, 1.0, s.ad_value(2526));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_134(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2658])) && (!s.b[2659])) {s.store_add_scaled_product_mixed_iai(2552, 605, 1.0, A::add_scaled_inputs(s.ad_value(2525), 1.0, s.ad_value(534), s.v[438]), 611, 1.0);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) {s.store_mul_scale_offset_mixed_ia(1903, 2552, A::add_scaled_inputs4(s.ad_value(2527), 1.0, s.ad_value(2528), 1.0, s.ad_value(2535), 1.0, s.ad_value(2550), 1.0), p.p29, 0.0);}
        s.b[2661] = (s.v[569] == 0.5);s.store_scalar(2661, if s.b[2661] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && s.b[2661]) {s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2518), s.ad_value(566)));}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) && (!s.b[2661])) {s.store_pow_sub_from_scalar_mul_base_indices(2526, 1.0, 2518, 566, 569);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2644])) {s.store_add_scaled_product_mixed_aia(1909, A::mul_sub_from_scalar_rhs(s.ad_value(578), 1.0, s.ad_value(2526)), p.p30, 581, A::sub(s.ad_value(827), s.ad_value(2518)), p.p30);}
        s.b[2662] = (s.v[668] == 0.0);s.store_scalar(2662, if s.b[2662] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2662]) {s.store_scalar(1904, 0.0);s.store_scalar(1910, 0.0);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) {s.store_mul(2527, 558, 2517);}
        s.b[2663] = ((s.v[517] == 0.0) && (s.v[520] == 0.0));s.store_scalar(2663, if s.b[2663] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && s.b[2663]) {s.store_scalar(2528, 0.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) {s.store_sub(2529, 564, 2523);s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));}
        s.b[2664] = (s.v[506] == 0.5);s.store_scalar(2664, if s.b[2664] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) && s.b[2664]) {s.store_scalar(2531, 0.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) && (!s.b[2664])) {s.store_mul_scale_offset(2531, A::add(A::div_scaled_product(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2530)), 1.0), s.ad_value(2530)), A::scale(s.ad_value(506), 2.0), -1.0, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) {s.store_add(2532, 2530, 2531);}
        s.b[2665] = (s.v[506] == 0.5);s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) && s.b[2665]) {s.store_sqrt_mul(2526, 2529, 591);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) && (!s.b[2665])) {s.store_pow_mul_base_indices(2526, 2529, 591, 506);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2663])) {s.store_mul(2533, 585, 2526);s.store_mul_ad_product_lhs_mixed_ia(2534, 555, A::offset(s.ad_value(2520), (-1.0)), 2533);s.store_mul3_lhs(2528, 517, 2534, 2532);}
        s.b[2666] = (s.v[520] == 0.0);s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && s.b[2666]) {s.store_scalar(2535, 0.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) {s.store_mul_div_scaled_product_indices(2536, 600, 2533, 570, 1.0, 2529, 1.0);s.store_div_scaled_inputs_indices(2537, 597, 0.666666666666667, 2536, 1.0);s.store_square(2538, 2537);s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);s.store_sqrt(2540, 2539);s.store_mul(2541, 2539, 2540);}
        s.b[2667] = (((-s.v[506]) * s.v[573]) == (-1.0));s.store_scalar(2667, if s.b[2667] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2667]) {s.store_div_from_scalar_offset_product(2542, 1.0, 2536, 2541, 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2667])) {s.store_pow_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), A::mul_scaled_lhs(s.ad_value(506), -1.0, s.ad_value(573)));}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) {s.store_div_scaled_product_add_scaled_denominator_indices(2543, 2532, 2542, 1.0, 2532, 1.0, 2542, 1.0, 1.0);s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_135(
        s: &mut Scratch,
    ) {
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) {s.store_add_scaled_value_products_mixed_aiiii(2546, A::mul3(s.ad_value(597), s.ad_value(2537), s.ad_value(2540)), 1.0, 597, 2539, (-1.0), 2536, 2541, 0.5);s.store_mul_scale_offset_indices(2547, 2544, 2545, 1.0, (-1.0));s.store_square(2508, 2547);}
        s.b[2668] = (s.v[2547] > 0.0);s.store_scalar(2668, if s.b[2668] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2668]) {s.store_div_from_scalar_offset_scaled_input(2509, 1.0, 2547, s.v[366], 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2668])) {s.store_div_from_scalar_sub_from_scalar_ad(2509, 1.0, 1.0, A::scale(s.ad_value(2547), s.v[366]));}
        s.b[2669] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));s.store_scalar(2669, if s.b[2669] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2669]) {s.store_exp_sub(2526, 2546, 2508);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2669])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) {s.store_mul_mixed_ai(2510, A::add_scaled_inputs_product(s.ad_value(2509), 0.29214664, A::square(s.ad_value(2509)), s.v[367], A::square(s.ad_value(2509)), s.ad_value(2509), s.v[368]), 2526);}
        s.b[2670] = (s.v[2547] > 0.0);s.store_scalar(2670, if s.b[2670] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2670]) {s.copy_ad(2548, 2510);}
        s.b[2671] = (s.v[2546] > (-230.25850929940458));s.store_scalar(2671, if s.b[2671] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2670])) && s.b[2671]) {s.store_exp(2526, 2546);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2670])) && (!s.b[2671])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2670])) {s.store_sub_scaled_inputs(2548, 2526, 2.0, 2510, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2666])) {s.store_div_scaled_product_indices(2549, 597, 2548, (1.772453850905516 * 0.5), 2544, 1.0);s.store_mul_product3_indices(2535, 520, 2534, 2549, 2543, 1.0);}
        s.b[2672] = (s.v[526] == 0.0);s.store_scalar(2672, if s.b[2672] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && s.b[2672]) {s.store_scalar(2550, 0.0);}
        s.b[2673] = (s.v[506] == 0.5);s.store_scalar(2673, if s.b[2673] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && s.b[2673]) {s.store_sqrt_mul_sub_lhs(2526, 503, 2524, 591);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && (!s.b[2673])) {s.store_pow_mul_base_mixed_ai(2526, A::sub(s.ad_value(503), s.ad_value(2524)), 591, 506);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) {s.store_mul_div_scaled_product_mixed_iaii(2551, 573, A::sub(s.ad_value(503), s.ad_value(2524)), 588, 1.0, 2526, 1.0);}
        s.b[2674] = (((((-s.v[603]) / s.v[2551])) as f64).abs() < 230.25850929940458);s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && s.b[2674]) {s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(603), -1.0, s.ad_value(2551), 1.0));}
        s.b[2675] = (((-s.v[603]) / s.v[2551]) < 0.0);s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && (!s.b[2674])) && s.b[2675]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 603, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) && (!s.b[2674])) && (!s.b[2675])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 603, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2672])) {s.store_mul_ad_product_lhs_mixed_ia(2550, 526, A::mul3(s.ad_value(827), s.ad_value(2551), s.ad_value(2551)), 2526);}
        s.b[2676] = (s.v[535] > 1000.0);s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && s.b[2676]) {s.store_scalar(2552, 1.0);}
        s.b[2677] = (s.v[2525] > ((-s.v[438]) * s.v[535]));s.store_scalar(2677, if s.b[2677] { 1.0 } else { 0.0 });s.b[2678] = (s.v[538] == 4.0);s.store_scalar(2678, if s.b[2678] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2676])) && s.b[2677]) && s.b[2678]) {s.store_mul_ad_product_lhs_mixed_ai(2526, A::mul3(A::square(A::mul(s.ad_value(2525), s.ad_value(609))), s.ad_value(2525), s.ad_value(609)), 2525, 609);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2676])) && s.b[2677]) && (!s.b[2678])) {s.store_pow_abs_mul_base_indices(2526, 2525, 609, 538);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2676])) && s.b[2677]) {s.store_div_from_scalar_sub_from_scalar_ad(2552, 1.0, 1.0, s.ad_value(2526));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_136(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2676])) && (!s.b[2677])) {s.store_add_scaled_product_mixed_iai(2552, 606, 1.0, A::add_scaled_inputs(s.ad_value(2525), 1.0, s.ad_value(535), s.v[438]), 612, 1.0);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) {s.store_mul_scale_offset_mixed_ia(1904, 2552, A::add_scaled_inputs4(s.ad_value(2527), 1.0, s.ad_value(2528), 1.0, s.ad_value(2535), 1.0, s.ad_value(2550), 1.0), p.p29, 0.0);}
        s.b[2679] = (s.v[570] == 0.5);s.store_scalar(2679, if s.b[2679] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && s.b[2679]) {s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2518), s.ad_value(567)));}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) && (!s.b[2679])) {s.store_pow_sub_from_scalar_mul_base_indices(2526, 1.0, 2518, 567, 570);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2662])) {s.store_add_scaled_product_mixed_aia(1910, A::mul_sub_from_scalar_rhs(s.ad_value(579), 1.0, s.ad_value(2526)), p.p30, 582, A::sub(s.ad_value(827), s.ad_value(2518)), p.p30);}
        s.b[2680] = (s.v[669] == 0.0);s.store_scalar(2680, if s.b[2680] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2680]) {s.store_scalar(1905, 0.0);s.store_scalar(1911, 0.0);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) {s.store_mul(2527, 559, 2517);}
        s.b[2681] = ((s.v[518] == 0.0) && (s.v[521] == 0.0));s.store_scalar(2681, if s.b[2681] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2681]) {s.store_scalar(2528, 0.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) {s.store_sub(2529, 565, 2523);s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));}
        s.b[2682] = (s.v[507] == 0.5);s.store_scalar(2682, if s.b[2682] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) && s.b[2682]) {s.store_scalar(2531, 0.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) && (!s.b[2682])) {s.store_mul_scale_offset(2531, A::add(A::div_scaled_product(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2530)), 1.0), s.ad_value(2530)), A::scale(s.ad_value(507), 2.0), -1.0, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) {s.store_add(2532, 2530, 2531);}
        s.b[2683] = (s.v[507] == 0.5);s.store_scalar(2683, if s.b[2683] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) && s.b[2683]) {s.store_sqrt_mul(2526, 2529, 592);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) && (!s.b[2683])) {s.store_pow_mul_base_indices(2526, 2529, 592, 507);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2681])) {s.store_mul(2533, 586, 2526);s.store_mul_ad_product_lhs_mixed_ia(2534, 556, A::offset(s.ad_value(2520), (-1.0)), 2533);s.store_mul3_lhs(2528, 518, 2534, 2532);}
        s.b[2684] = (s.v[521] == 0.0);s.store_scalar(2684, if s.b[2684] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2684]) {s.store_scalar(2535, 0.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) {s.store_mul_div_scaled_product_indices(2536, 601, 2533, 571, 1.0, 2529, 1.0);s.store_div_scaled_inputs_indices(2537, 598, 0.666666666666667, 2536, 1.0);s.store_square(2538, 2537);s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);s.store_sqrt(2540, 2539);s.store_mul(2541, 2539, 2540);}
        s.b[2685] = (((-s.v[507]) * s.v[574]) == (-1.0));s.store_scalar(2685, if s.b[2685] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2685]) {s.store_div_from_scalar_offset_product(2542, 1.0, 2536, 2541, 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2685])) {s.store_pow_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), A::mul_scaled_lhs(s.ad_value(507), -1.0, s.ad_value(574)));}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) {s.store_div_scaled_product_add_scaled_denominator_indices(2543, 2532, 2542, 1.0, 2532, 1.0, 2542, 1.0, 1.0);s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_137(
        s: &mut Scratch,
    ) {
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) {s.store_add_scaled_value_products_mixed_aiiii(2546, A::mul3(s.ad_value(598), s.ad_value(2537), s.ad_value(2540)), 1.0, 598, 2539, (-1.0), 2536, 2541, 0.5);s.store_mul_scale_offset_indices(2547, 2544, 2545, 1.0, (-1.0));s.store_square(2508, 2547);}
        s.b[2686] = (s.v[2547] > 0.0);s.store_scalar(2686, if s.b[2686] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2686]) {s.store_div_from_scalar_offset_scaled_input(2509, 1.0, 2547, s.v[366], 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2686])) {s.store_div_from_scalar_sub_from_scalar_ad(2509, 1.0, 1.0, A::scale(s.ad_value(2547), s.v[366]));}
        s.b[2687] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));s.store_scalar(2687, if s.b[2687] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2687]) {s.store_exp_sub(2526, 2546, 2508);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2687])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) {s.store_mul_mixed_ai(2510, A::add_scaled_inputs_product(s.ad_value(2509), 0.29214664, A::square(s.ad_value(2509)), s.v[367], A::square(s.ad_value(2509)), s.ad_value(2509), s.v[368]), 2526);}
        s.b[2688] = (s.v[2547] > 0.0);s.store_scalar(2688, if s.b[2688] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2688]) {s.copy_ad(2548, 2510);}
        s.b[2689] = (s.v[2546] > (-230.25850929940458));s.store_scalar(2689, if s.b[2689] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2688])) && s.b[2689]) {s.store_exp(2526, 2546);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2688])) && (!s.b[2689])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2688])) {s.store_sub_scaled_inputs(2548, 2526, 2.0, 2510, 1.0);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2684])) {s.store_div_scaled_product_indices(2549, 598, 2548, (1.772453850905516 * 0.5), 2544, 1.0);s.store_mul_product3_indices(2535, 521, 2534, 2549, 2543, 1.0);}
        s.b[2690] = (s.v[527] == 0.0);s.store_scalar(2690, if s.b[2690] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2690]) {s.store_scalar(2550, 0.0);}
        s.b[2691] = (s.v[507] == 0.5);s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && s.b[2691]) {s.store_sqrt_mul_sub_lhs(2526, 504, 2524, 592);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && (!s.b[2691])) {s.store_pow_mul_base_mixed_ai(2526, A::sub(s.ad_value(504), s.ad_value(2524)), 592, 507);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) {s.store_mul_div_scaled_product_mixed_iaii(2551, 574, A::sub(s.ad_value(504), s.ad_value(2524)), 589, 1.0, 2526, 1.0);}
        s.b[2692] = (((((-s.v[604]) / s.v[2551])) as f64).abs() < 230.25850929940458);s.store_scalar(2692, if s.b[2692] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && s.b[2692]) {s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(604), -1.0, s.ad_value(2551), 1.0));}
        s.b[2693] = (((-s.v[604]) / s.v[2551]) < 0.0);s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && (!s.b[2692])) && s.b[2693]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 604, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) && (!s.b[2692])) && (!s.b[2693])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 604, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2690])) {s.store_mul_ad_product_lhs_mixed_ia(2550, 527, A::mul3(s.ad_value(827), s.ad_value(2551), s.ad_value(2551)), 2526);}
        s.b[2694] = (s.v[635] > 1000.0);s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2694]) {s.store_scalar(2552, 1.0);}
        s.b[2695] = (s.v[2525] > ((-s.v[438]) * s.v[635]));s.store_scalar(2695, if s.b[2695] { 1.0 } else { 0.0 });s.b[2696] = (s.v[539] == 4.0);s.store_scalar(2696, if s.b[2696] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2694])) && s.b[2695]) && s.b[2696]) {s.store_mul_ad_product_lhs_mixed_ai(2526, A::mul3(A::square(A::mul(s.ad_value(2525), s.ad_value(610))), s.ad_value(2525), s.ad_value(610)), 2525, 610);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2694])) && s.b[2695]) && (!s.b[2696])) {s.store_pow_abs_mul_base_indices(2526, 2525, 610, 539);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2694])) && s.b[2695]) {s.store_div_from_scalar_sub_from_scalar_ad(2552, 1.0, 1.0, s.ad_value(2526));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_138(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2694])) && (!s.b[2695])) {s.store_add_scaled_product_mixed_iai(2552, 607, 1.0, A::add_scaled_inputs(s.ad_value(2525), 1.0, s.ad_value(635), s.v[438]), 613, 1.0);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) {s.store_mul_scale_offset_mixed_ia(1905, 2552, A::add_scaled_inputs4(s.ad_value(2527), 1.0, s.ad_value(2528), 1.0, s.ad_value(2535), 1.0, s.ad_value(2550), 1.0), p.p29, 0.0);}
        s.b[2697] = (s.v[629] == 1.0);s.store_scalar(2697, if s.b[2697] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {
            if (s.v[827] < s.v[544]) {
                if (((s.v[827] - s.v[544]) / s.v[545]) < (-37.0)) {
                    s.copy_ad(2553, 544);
                } else {
                    s.store_add_scaled_product_mixed_iai(2553, 544, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(827), 1.0, s.ad_value(544), (-1.0), s.ad_value(545), 1.0)), 545, 1.0);
                }
            } else {
                if (((s.v[827] - s.v[544]) / s.v[545]) > 37.0) {
                    s.copy_ad(2553, 827);
                } else {
                    s.store_add_scaled_product_mixed_iai(2553, 827, 1.0, A::ln_one_plus_exp(A::div_scaled_inputs2(s.ad_value(544), 1.0, s.ad_value(827), (-1.0), s.ad_value(545), 1.0)), 545, 1.0);
                }
            }
        }
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {s.store_primal_scaled_mul(2511, 678, 678, 4.0);s.store_primal_div(2512, 678, 679);s.store_add_scaled_product_indices(2513, 2553, 1.0, 678, 2512, 1.0);s.store_add(2514, 679, 2513);s.store_sub(2515, 679, 2513);s.store_sqrt_square_add(2516, 2515, 2511);s.store_div_scaled_product_add_scaled_denominator_indices(2554, 2553, 679, 2.0, 2514, 1.0, 2516, 1.0, 1.0);}
        s.b[2698] = (s.v[571] == 0.5);s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && s.b[2698]) {s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2554), s.ad_value(568)));}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && (!s.b[2698])) {s.store_pow_sub_from_scalar_mul_base_indices(2526, 1.0, 2554, 568, 571);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {s.store_add_scaled_product_mixed_aia(1911, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(2526)), p.p30, 583, A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30);s.store_add_scaled_inputs3_indices(2553, 827, 1.0, 544, 1.0, 2553, -1.0);s.store_primal_scaled_mul(2511, 678, 678, 4.0);s.store_primal_div(2512, 678, 679);s.store_add_scaled_product_indices(2513, 2553, 1.0, 678, 2512, 1.0);s.store_add(2514, 679, 2513);s.store_sub(2515, 679, 2513);s.store_sqrt_square_add(2516, 2515, 2511);s.store_div_scaled_product_add_scaled_denominator_indices(2554, 2553, 679, 2.0, 2514, 1.0, 2516, 1.0, 1.0);}
        s.b[2699] = (s.v[624] == 0.5);s.store_scalar(2699, if s.b[2699] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && s.b[2699]) {s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2554), s.ad_value(623)));}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) && (!s.b[2699])) {s.store_pow_sub_from_scalar_mul_base_indices(2526, 1.0, 2554, 623, 624);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && s.b[2697]) {s.store_add_scaled_product_mixed_aia(466, A::mul_sub_from_scalar_rhs(s.ad_value(627), 1.0, s.ad_value(2526)), p.p30, 628, A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30);s.store_add(1911, 1911, 466);}
        s.b[2700] = (s.v[571] == 0.5);s.store_scalar(2700, if s.b[2700] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2697])) && s.b[2700]) {s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::mul(s.ad_value(2518), s.ad_value(568)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_139(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2697])) && (!s.b[2700])) {s.store_pow_sub_from_scalar_mul_base_indices(2526, 1.0, 2518, 568, 571);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2680])) && (!s.b[2697])) {s.store_add_scaled_product_mixed_aia(1911, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(2526)), p.p30, 583, A::sub(s.ad_value(827), s.ad_value(2518)), p.p30);}
        if (s.b[2555] && (!s.b[2556])) {s.store_add_scaled_products3_indices(843, 667, 1903, 1.0, 668, 1904, 1.0, 669, 1905, 1.0);}
        s.b[2701] = (s.v[825] > 0.0);s.store_scalar(2701, if s.b[2701] { 1.0 } else { 0.0 });s.b[2702] = (s.v[295] > 0.0);s.store_scalar(2702, if s.b[2702] { 1.0 } else { 0.0 });s.b[2703] = (s.v[296] > 0.0);s.store_scalar(2703, if s.b[2703] { 1.0 } else { 0.0 });s.b[2704] = (s.v[297] > 0.0);s.store_scalar(2704, if s.b[2704] { 1.0 } else { 0.0 });s.b[2705] = (s.v[298] > 0.0);s.store_scalar(2705, if s.b[2705] { 1.0 } else { 0.0 });s.b[2706] = (s.v[299] > 0.0);s.store_scalar(2706, if s.b[2706] { 1.0 } else { 0.0 });s.b[2707] = (s.v[300] > 0.0);s.store_scalar(2707, if s.b[2707] { 1.0 } else { 0.0 });s.b[2708] = (s.v[301] > 0.0);s.store_scalar(2708, if s.b[2708] { 1.0 } else { 0.0 });s.store_add_scaled_inputs3_indices(844, 845, (-1.0), 846, (-1.0), 847, (-1.0));s.store_add(848, 848, 1898);s.store_add(849, 849, 1899);s.store_add_scaled_products3_indices(851, 640, 1906, 1.0, 641, 1907, 1.0, 642, 1908, 1.0);s.store_add_scaled_products3_indices(852, 667, 1909, 1.0, 668, 1910, 1.0, 669, 1911, 1.0);s.b[2710] = (s.v[825] < 0.0);s.store_scalar(2710, if s.b[2710] { 1.0 } else { 0.0 });
        if s.b[2710] {s.copy_ad(2709, 847);s.copy_ad(847, 844);s.copy_ad(844, 2709);}
        s.store_scalar(2727, 0.0);s.store_scalar(2722, 0.0);s.store_scalar(853, 1e-40);s.store_scalar(855, 0.0);s.store_scalar(857, 0.0);s.store_mul(854, 1892, 1883);s.store_scalar(856, 0.0);s.store_scalar(2729, 0.0);s.b[2743] = ((s.v[1817] > 0.0) && (s.v[710] > 0.0));s.store_scalar(2743, if s.b[2743] { 1.0 } else { 0.0 });s.b[2745] = (p.p32 > 0.0);s.store_scalar(2745, if s.b[2745] { 1.0 } else { 0.0 });
        if (s.b[2743] && s.b[2745]) {s.store_div(2714, 1854, 1852);s.store_div(2715, 1853, 1854);s.store_scaled_div(2716, 1848, 2714, (0.5 * 0.16666666666666666));s.store_square(2717, 2716);s.store_offset_div(2718, 2714, 1865, (-1.0));}
        if (s.b[2743] && s.b[2745]) {
            if ((1.0 - (12.0 * (s.v[2718] * s.v[2717]))) > 1e-20) {
                s.store_sub_from_scalar_scaled_mul(2719, 1.0, 2718, 2717, 12.0);
            } else {
                s.store_scalar(2719, 1e-20);
            }
        }
        if (s.b[2743] && s.b[2745]) {s.store_div_from_scalar_square_ad(2720, 1.0, s.ad_value(2719));s.store_mul3_lhs(2721, 710, 1854, 1864);s.store_add_scaled_inputs3_mixed_iia(2722, 2715, 1.0, 2717, 12.0, A::mul3_scaled_output(A::offset(s.ad_value(2715), 1.0), s.ad_value(2717), s.ad_value(2718), 24.0), -1.0);}
        if (s.b[2743] && s.b[2745]) {
            if (s.v[2722] > 1e-40) {
            } else {
                s.store_scalar(2722, 1e-40);
            }
        }
        if (s.b[2743] && s.b[2745]) {s.store_mul3_lhs(2722, 2721, 2720, 2722);}
        s.b[2746] = (s.v[272] > 0.0);s.store_scalar(2746, if s.b[2746] { 1.0 } else { 0.0 });
        if ((s.b[2743] && s.b[2745]) && s.b[2746]) {s.store_div(2723, 1858, 1857);s.store_mul_ad_product_lhs_mixed_ai(2724, A::square(s.ad_value(2723)), 1848, 1848);}
        s.b[2747] = (s.v[0] == (-1.0));s.store_scalar(2747, if s.b[2747] { 1.0 } else { 0.0 });
        if (((s.b[2743] && s.b[2745]) && s.b[2746]) && s.b[2747]) {s.store_div_scaled_value_offset_denominator(2724, s.ad_value(2724), 1.0, A::mul(s.ad_value(2723), s.ad_value(1848)), 1.0, 1.0);}
        if ((s.b[2743] && s.b[2745]) && s.b[2746]) {s.store_mul_scale_offset_mixed_ia(2725, 1857, A::sqrt(A::scale_offset(s.ad_value(2724), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div_scaled_value_by_product_indices(2726, 1857, 1.0, 2725, 2719, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_140(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2743] && s.b[2745]) && s.b[2746]) {s.store_mul_ad_product_lhs_mixed_ai(2727, A::mul3(s.ad_value(804), s.ad_value(832), s.ad_value(1845)), 2726, 2726);s.store_add_scaled_inputs(2722, 2722, 1.0, 2727, 1.0 / (s.v[712]));}
        if (s.b[2743] && s.b[2745]) {s.store_sqrt_mul(856, 713, 2722);}
        s.b[2748] = ((((p.p50 == 1.0) && (s.v[713] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));s.store_scalar(2748, if s.b[2748] { 1.0 } else { 0.0 });
        if (s.b[2743] && s.b[2748]) {s.store_sub_ad(853, A::add_scaled_product(s.ad_value(2715), 0.08333333333333333, s.ad_value(2717), A::sub_scaled_inputs(A::offset(s.ad_value(2715), 0.2), 1.0, s.ad_value(2717), 12.0), (-1.0)), A::mul3_scaled_output(s.ad_value(2717), A::sub_scaled_inputs(A::offset(s.ad_value(2715), 1.0), 1.0, s.ad_value(2717), 12.0), s.ad_value(2718), 1.6));}
        if (s.b[2743] && s.b[2748]) {
            if (s.v[853] > 1e-40) {
            } else {
                s.store_scalar(853, 1e-40);
            }
        }
        if (s.b[2743] && s.b[2748]) {s.store_mul_div_lhs(853, 2720, 2721, 853);s.store_mul_ad_product_rhs_mixed_ia(2728, 2720, 2716, A::add_scaled_sub_value_product(1.0, A::scale(s.ad_value(2717), 12.0), 1.0, A::add_scaled_inputs_product(s.ad_value(2715), 1.0, s.ad_value(2717), 19.2, s.ad_value(2715), s.ad_value(2717), (-12.0)), s.ad_value(2718), (-1.0)));s.store_div_scaled_product3_mixed_aiia(854, A::square(s.ad_value(1896)), 1892, 1883, 1.0, A::square(s.ad_value(1894)), 1.0);}
        s.b[2749] = (s.v[272] > 0.0);s.store_scalar(2749, if s.b[2749] { 1.0 } else { 0.0 });
        if ((s.b[2743] && s.b[2748]) && s.b[2749]) {s.store_add_mixed_ia(853, 853, A::div_scaled_product_by_product(s.ad_value(2727), A::scale_offset(s.ad_value(2717), 12.0, 1.0), 1.0, s.ad_value(2721), s.ad_value(2721), (12.0 * s.v[712])));s.store_sub_mixed_ia(2728, 2728, A::div_scaled_product3(s.ad_value(2727), s.ad_value(2716), A::offset(s.ad_value(2718), 1.0), 1.0, s.ad_value(2721), s.v[712]));}
        if (s.b[2743] && s.b[2748]) {s.store_sqrt_div(2729, 713, 853);}
        s.b[2750] = (s.v[856] <= 0.0);s.store_scalar(2750, if s.b[2750] { 1.0 } else { 0.0 });
        if ((s.b[2743] && s.b[2748]) && s.b[2750]) {s.store_scalar(857, 0.0);}
        if ((s.b[2743] && s.b[2748]) && (!s.b[2750])) {s.store_div_scaled_product_indices(857, 2728, 2729, 1.0, 856, 1.0);}
        if (s.b[2743] && s.b[2748]) {
            if (s.v[857] > 0.0) {
                if (s.v[857] < 1.0) {
                } else {
                    s.store_scalar(857, 1.0);
                }
            } else {
                s.store_scalar(857, 0.0);
            }
        }
        if (s.b[2743] && s.b[2748]) {s.store_div_scaled_product_indices(855, 857, 856, 1.0, 2729, 1.0);}
        s.b[2752] = (((p.p46 != 0.0) && (s.v[282] > 0.0)) && (s.v[1868] > 0.0));s.store_scalar(2752, if s.b[2752] { 1.0 } else { 0.0 });
        if s.b[2752] {s.store_div_scaled_inputs_indices(1920, 1871, 4.0, 718, 1.0);s.store_scale(1920, 765, s.v[709]);s.store_mul(1920, 1852, 1865);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[984] = (p.p37 >= 0.0);s.store_scalar(984, if s.b[984] { 1.0 } else { 0.0 });
        if s.b[984] {s.store_scalar(0, 1.0);}
        if (!s.b[984]) {s.store_scalar(0, (-1.0));}
        s.store_scalar(761, (8.8541878176e-12 * 11.8));s.store_scalar(344, (273.15 + p.p38));s.store_scalar(468, 0.0);s.b[985] = (p.p920 > 0.5);s.store_scalar(985, if s.b[985] { 1.0 } else { 0.0 });
        if s.b[985] {s.store_scalar(468, 1.0);}
        if (!s.b[985]) {s.store_scalar(468, 0.0);}
        s.store_scalar(358, (273.15 + p.p816));s.store_scalar(361, (1.3806505e-23 / 1.6021918e-19));s.store_scalar(362, (s.v[361] * s.v[358]));s.store_scalar(363, (1.0 / s.v[362]));s.store_scalar(369, ((-((0.000702 * s.v[358]) * s.v[358])) / (1108.0 + s.v[358])));s.store_scalar(372, (p.p827 + s.v[369]));s.store_scalar(373, (p.p828 + s.v[369]));s.store_scalar(374, (p.p829 + s.v[369]));s.store_scalar(402, (1.0 - p.p824));s.store_scalar(403, (1.0 - p.p825));s.store_scalar(404, (1.0 - p.p826));s.store_scalar(405, (1.0 / s.v[402]));s.store_scalar(406, (1.0 / s.v[403]));s.store_scalar(407, (1.0 / s.v[404]));s.store_scalar(417, (s.v[761] / p.p818));s.store_scalar(418, ((p.p836 * s.v[761]) / p.p819));s.store_scalar(419, ((p.p837 * s.v[761]) / p.p820));s.store_scalar(420, (1.0 / s.v[417]));s.store_scalar(421, (1.0 / s.v[418]));s.store_scalar(422, (1.0 / s.v[419]));s.store_scalar(423, (1.0 / p.p821));s.store_scalar(424, (1.0 / p.p822));s.store_scalar(425, (1.0 / p.p823));s.store_scalar(438, (1.0 - (1.0 / p.p817)));s.store_scalar(442, (1.0 / p.p853));s.store_scalar(443, (1.0 / p.p854));s.store_scalar(444, (1.0 / p.p855));s.b[986] = ((((p.p859 != 1.0) || (p.p860 != 1.0)) || (p.p861 != 1.0)) || (p.p862 != 1.0));s.store_scalar(986, if s.b[986] { 1.0 } else { 0.0 });
        if s.b[986] {s.store_scalar(467, 1.0);}
        if (!s.b[986]) {s.store_scalar(467, 0.0);}
        s.b[987] = (s.v[467] == 1.0);s.store_scalar(987, if s.b[987] { 1.0 } else { 0.0 });
        if s.b[987] {s.store_scalar(451, (if ((p.p820 * p.p859) > 1e-18) { (p.p820 * p.p859) } else { 1e-18 }));}
        if s.b[987] {s.store_scalar(452, (if ((p.p823 * p.p860) > 0.05) { (p.p823 * p.p860) } else { 0.05 }));}
        if s.b[987] {s.store_scalar(453, (if ((if ((p.p826 * p.p861) > 0.05) { (p.p826 * p.p861) } else { 0.05 }) < 0.95) { (if ((p.p826 * p.p861) > 0.05) { (p.p826 * p.p861) } else { 0.05 }) } else { 0.95 }));}
        if s.b[987] {s.store_scalar(454, (p.p829 * p.p862));s.store_primal_offset(456, 454, s.v[369]);s.store_primal_sub_from_scalar(461, 1.0, 453);s.store_primal_div_from_scalar(462, 1.0, 461);}
        s.b[988] = (p.p44 == 0.0);s.store_scalar(988, if s.b[988] { 1.0 } else { 0.0 });
        if s.b[988] {s.store_scalar(499, p.p818);s.store_scalar(500, p.p819);s.store_scalar(501, p.p820);s.store_scalar(502, p.p821);s.store_scalar(503, p.p822);s.store_scalar(504, p.p823);s.store_scalar(505, p.p824);s.store_scalar(506, p.p825);s.store_scalar(507, p.p826);s.store_scalar(508, p.p827);s.store_scalar(509, p.p828);s.store_scalar(510, p.p829);s.store_scalar(511, p.p830);s.store_scalar(512, p.p831);s.store_scalar(513, p.p832);s.store_scalar(516, p.p833);s.store_scalar(517, p.p834);s.store_scalar(518, p.p835);s.store_scalar(514, p.p836);s.store_scalar(515, p.p837);s.store_scalar(519, p.p838);s.store_scalar(520, p.p839);s.store_scalar(521, p.p840);s.store_scalar(522, p.p841);s.store_scalar(523, p.p842);s.store_scalar(524, p.p843);s.store_scalar(525, p.p844);s.store_scalar(526, p.p845);s.store_scalar(527, p.p846);s.store_scalar(528, p.p847);s.store_scalar(529, p.p848);s.store_scalar(530, p.p849);s.store_scalar(531, p.p850);s.store_scalar(532, p.p851);s.store_scalar(533, p.p852);s.store_scalar(534, p.p853);s.store_scalar(535, p.p854);s.store_scalar(536, p.p855);s.store_scalar(537, p.p856);s.store_scalar(538, p.p857);s.store_scalar(539, p.p858);s.store_scalar(547, p.p922);s.store_scalar(630, p.p865);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_1(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[988] {s.store_scalar(631, p.p866);s.store_scalar(632, p.p867);s.store_scalar(633, p.p868);s.store_scalar(540, p.p859);s.store_scalar(541, p.p860);s.store_scalar(542, p.p861);s.store_scalar(543, p.p862);s.store_scalar(544, p.p863);s.store_scalar(545, p.p864);}
        if (!s.b[988]) {s.store_scalar(499, p.p869);s.store_scalar(500, p.p870);s.store_scalar(501, p.p871);s.store_scalar(502, p.p872);s.store_scalar(503, p.p873);s.store_scalar(504, p.p874);s.store_scalar(505, p.p875);s.store_scalar(506, p.p876);s.store_scalar(507, p.p877);s.store_scalar(508, p.p878);s.store_scalar(509, p.p879);s.store_scalar(510, p.p880);s.store_scalar(511, p.p881);s.store_scalar(512, p.p882);s.store_scalar(513, p.p883);s.store_scalar(516, p.p884);s.store_scalar(517, p.p885);s.store_scalar(518, p.p886);s.store_scalar(514, p.p887);s.store_scalar(515, p.p888);s.store_scalar(519, p.p889);s.store_scalar(520, p.p890);s.store_scalar(521, p.p891);s.store_scalar(522, p.p892);s.store_scalar(523, p.p893);s.store_scalar(524, p.p894);s.store_scalar(525, p.p895);s.store_scalar(526, p.p896);s.store_scalar(527, p.p897);s.store_scalar(528, p.p898);s.store_scalar(529, p.p899);s.store_scalar(530, p.p900);s.store_scalar(531, p.p901);s.store_scalar(532, p.p902);s.store_scalar(533, p.p903);s.store_scalar(534, p.p904);s.store_scalar(535, p.p905);s.store_scalar(536, p.p906);s.store_scalar(537, p.p907);s.store_scalar(538, p.p908);s.store_scalar(539, p.p909);s.store_scalar(547, p.p924);s.store_scalar(630, p.p916);s.store_scalar(631, p.p917);s.store_scalar(632, p.p918);s.store_scalar(633, p.p919);s.store_scalar(540, p.p910);s.store_scalar(541, p.p911);s.store_scalar(542, p.p912);s.store_scalar(543, p.p913);s.store_scalar(544, p.p914);s.store_scalar(545, p.p915);}
        s.store_primal_offset(548, 508, s.v[369]);s.store_primal_offset(549, 509, s.v[369]);s.store_primal_offset(550, 510, s.v[369]);s.store_primal_sub_from_scalar(569, 1.0, 505);s.store_primal_sub_from_scalar(570, 1.0, 506);s.store_primal_sub_from_scalar(571, 1.0, 507);s.store_primal_div_from_scalar(572, 1.0, 569);s.store_primal_div_from_scalar(573, 1.0, 570);s.store_primal_div_from_scalar(574, 1.0, 571);s.store_primal_div_from_scalar(584, s.v[761], 499);s.store_primal_div_scaled_inputs_indices(585, 514, s.v[761], 500, 1.0);s.store_primal_div_scaled_inputs_indices(586, 515, s.v[761], 501, 1.0);s.store_primal_div_from_scalar(587, 1.0, 584);s.store_primal_div_from_scalar(588, 1.0, 585);s.store_primal_div_from_scalar(589, 1.0, 586);s.store_primal_div_from_scalar(590, 1.0, 502);s.store_primal_div_from_scalar(591, 1.0, 503);s.store_primal_div_from_scalar(592, 1.0, 504);s.store_primal_div_from_scalar(608, 1.0, 534);s.store_primal_div_from_scalar(609, 1.0, 535);s.store_div_from_scalar(610, 1.0, 536);s.b[989] = ((((s.v[540] != 1.0) || (s.v[541] != 1.0)) || (s.v[542] != 1.0)) || (s.v[543] != 1.0));s.store_scalar(989, if s.b[989] { 1.0 } else { 0.0 });
        if s.b[989] {s.store_scalar(629, 1.0);}
        if (!s.b[989]) {s.store_scalar(629, 0.0);}
        s.b[990] = (s.v[629] == 1.0);s.store_scalar(990, if s.b[990] { 1.0 } else { 0.0 });
        if s.b[990] {
            if ((s.v[501] * s.v[540]) > 1e-18) {
                s.store_primal_mul(614, 501, 540);
            } else {
                s.store_scalar(614, 1e-18);
            }
        }
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_2(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        let ctx_temp = ctx.temperature();
        if s.b[990] {
            if ((s.v[504] * s.v[541]) > 0.05) {
                s.store_primal_mul(615, 504, 541);
            } else {
                s.store_scalar(615, 0.05);
            }
        }
        if s.b[990] {
            if ((if ((s.v[507] * s.v[542]) > 0.05) { (s.v[507] * s.v[542]) } else { 0.05 }) < 0.95) {
                if ((s.v[507] * s.v[542]) > 0.05) {
                    s.store_primal_mul(616, 507, 542);
                } else {
                    s.store_scalar(616, 0.05);
                }
            } else {
                s.store_scalar(616, 0.95);
            }
        }
        if s.b[990] {s.store_primal_mul(617, 510, 543);s.store_primal_offset(619, 617, s.v[369]);s.store_primal_sub_from_scalar(624, 1.0, 616);s.store_primal_div_from_scalar(625, 1.0, 624);}
        s.store_scalar(345, ((ctx_temp + p.p55) + p.p35));s.store_scalar(346, (s.v[345] / s.v[344]));s.store_scalar(347, (s.v[345] - s.v[344]));s.store_scalar(348, ((s.v[345] * 1.3806505e-23) / 1.6021918e-19));s.store_scalar(349, (1.0 / s.v[348]));s.store_scalar(350, s.v[345]);s.store_scalar(351, (s.v[350] * s.v[350]));s.store_scalar(352, (s.v[350] - s.v[344]));s.store_scalar(353, (s.v[344] / s.v[350]));s.store_scalar(354, ((s.v[353]) as f64).ln());s.store_scalar(709, ((s.v[350] * 1.3806505e-23) / 1.6021918e-19));s.store_scalar(355, (1.0 / s.v[709]));s.store_scalar(356, ((1.179 - (9.025e-5 * s.v[350])) - (3.05e-7 * s.v[351])));s.store_scalar(357, ((((1.045 + (0.00045 * s.v[350])) * ((0.523 + (0.0014 * s.v[350])) - (1.48e-6 * s.v[351]))) * s.v[351]) / 90000.0));
        if (!(s.v[357] > 0.001)) {s.store_scalar(357, 0.001);}
        s.store_scalar(359, (((ctx_temp + p.p55) + p.p35)).max((273.15 + (-250.0))));s.store_scalar(360, (s.v[359] / s.v[358]));s.store_scalar(364, (s.v[361] * s.v[359]));s.store_scalar(365, (1.0 / s.v[364]));s.store_scalar(370, ((-((0.000702 * s.v[359]) * s.v[359])) / (1108.0 + s.v[359])));s.store_scalar(375, (p.p827 + s.v[370]));s.store_scalar(376, (p.p828 + s.v[370]));s.store_scalar(377, (p.p829 + s.v[370]));s.store_scalar(378, (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[372] * s.v[363]) - (s.v[375] * s.v[365])))) as f64).exp()));s.store_scalar(379, (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[373] * s.v[363]) - (s.v[376] * s.v[365])))) as f64).exp()));s.store_scalar(380, (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[374] * s.v[363]) - (s.v[377] * s.v[365])))) as f64).exp()));s.store_scalar(381, ((p.p830 * s.v[378]) * s.v[378]));s.store_scalar(382, ((p.p831 * s.v[379]) * s.v[379]));s.store_scalar(383, ((p.p832 * s.v[380]) * s.v[380]));s.store_scalar(384, ((p.p821 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[378]) as f64).ln())));s.store_scalar(385, ((p.p822 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[379]) as f64).ln())));s.store_scalar(386, ((p.p823 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[380]) as f64).ln())));s.store_scalar(387, (s.v[384] + (s.v[364] * (((1.0 + ((((0.05 - s.v[384]) * s.v[365])) as f64).exp())) as f64).ln())));s.store_scalar(388, (s.v[385] + (s.v[364] * (((1.0 + ((((0.05 - s.v[385]) * s.v[365])) as f64).exp())) as f64).ln())));s.store_scalar(389, (s.v[386] + (s.v[364] * (((1.0 + ((((0.05 - s.v[386]) * s.v[365])) as f64).exp())) as f64).ln())));s.store_scalar(399, (1.0 / s.v[387]));s.store_scalar(400, (1.0 / s.v[388]));s.store_scalar(401, (1.0 / s.v[389]));s.store_scalar(408, (p.p818 * (((p.p821 * s.v[399])) as f64).powf(p.p824)));s.store_scalar(409, (p.p819 * (((p.p822 * s.v[400])) as f64).powf(p.p825)));s.store_scalar(410, (p.p820 * (((p.p823 * s.v[401])) as f64).powf(p.p826)));s.store_scalar(411, ((s.v[408] * s.v[387]) * s.v[405]));s.store_scalar(412, ((s.v[409] * s.v[388]) * s.v[406]));s.store_scalar(413, ((s.v[410] * s.v[389]) * s.v[407]));s.store_scalar(414, (2.0 * s.v[408]));s.store_scalar(415, (2.0 * s.v[409]));s.store_scalar(416, (2.0 * s.v[410]));s.store_scalar(426, ((0.5 * s.v[375])).max(s.v[364]));s.store_scalar(427, ((0.5 * s.v[376])).max(s.v[364]));s.store_scalar(428, ((0.5 * s.v[377])).max(s.v[364]));s.store_scalar(429, (s.v[426] * s.v[365]));s.store_scalar(430, (s.v[427] * s.v[365]));s.store_scalar(431, (s.v[428] * s.v[365]));s.store_scalar(432, (((((((32.0 * p.p841) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[426] * s.v[426]) * s.v[426]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(433, (((((((32.0 * p.p842) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[427] * s.v[427]) * s.v[427]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(434, (((((((32.0 * p.p843) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[428] * s.v[428]) * s.v[428]))) as f64).sqrt() / (3.0 * 1.05457168e-34)));s.store_scalar(435, (p.p847 * (1.0 + (p.p850 * (s.v[359] - s.v[358])))));s.store_scalar(436, (p.p848 * (1.0 + (p.p851 * (s.v[359] - s.v[358])))));s.store_scalar(437, (p.p849 * (1.0 + (p.p852 * (s.v[359] - s.v[358])))));
        if (!(s.v[435] > 0.0)) {s.store_scalar(435, 0.0);}
        if (!(s.v[436] > 0.0)) {s.store_scalar(436, 0.0);}
        if (!(s.v[437] > 0.0)) {s.store_scalar(437, 0.0);}
        s.b[1010] = (s.v[467] == 1.0);s.store_scalar(1010, if s.b[1010] { 1.0 } else { 0.0 });
        if s.b[1010] {s.store_primal_offset(455, 454, s.v[370]);s.store_primal_scale_ad(457, A::exp_scaled_input(A::sub_scaled_inputs(s.ad_value(456), s.v[363], s.ad_value(455), s.v[365]), 0.5), ((s.v[360]) as f64).powf(1.5));s.store_primal_sub_scaled_inputs_ln_rhs(458, 452, s.v[360], 457, (2.0 * s.v[364]));s.store_primal_add_scaled_inputs_mixed_ia(459, 458, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(458), (-s.v[365]), ((0.05) * (s.v[365])))), s.v[364]);s.store_primal_div_from_scalar(460, 1.0, 459);s.store_primal_mul_pow_mixed_iai(463, 451, A::mul(s.ad_value(452), s.ad_value(460)), 453);s.store_primal_mul3_lhs(464, 463, 459, 462);}
    }
}
