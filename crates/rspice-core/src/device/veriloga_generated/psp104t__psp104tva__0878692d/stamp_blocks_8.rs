#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_128(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2608])) && (!s.b[2609])) {s.store_offset_scaled(2566, 2539, s.v[452], (((((s.v[445] * p.p877)) * (s.v[452]))) + (s.v[446])));}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) {s.store_mul_scale_offset_mixed_ia(1896, 2566, A::add_scaled_inputs4(s.ad_value(2541), 1.0, s.ad_value(2542), 1.0, s.ad_value(2549), 1.0, s.ad_value(2564), 1.0), p.p29, 0.0);}
        s.b[2611] = (s.v[409] == 0.5);s.store_scalar(2611, if s.b[2611] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && s.b[2611]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2532), s.v[406]));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2611])) {s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[406])), s.v[409]);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) {s.store_add_scaled_inputs3_offset_indices(1902, 2540, ((-s.v[418]) * p.p30), 821, (s.v[421] * p.p30), 2532, ((-s.v[421]) * p.p30), (s.v[418] * p.p30));}
        s.b[2612] = (s.v[648] == 0.0);s.store_scalar(2612, if s.b[2612] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2612]) {s.store_scalar(1897, 0.0);s.store_scalar(1903, 0.0);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) {s.store_scale(2541, 2531, s.v[389]);}
        s.b[2613] = ((p.p858 == 0.0) && (p.p863 == 0.0));s.store_scalar(2613, if s.b[2613] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && s.b[2613]) {s.store_scalar(2542, 0.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) {s.store_sub_from_scalar(2543, s.v[395], 2537);s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));}
        s.b[2614] = (p.p849 == 0.5);s.store_scalar(2614, if s.b[2614] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) && s.b[2614]) {s.store_scalar(2545, 0.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) && (!s.b[2614])) {s.store_scaled_add_mixed_ai(2545, A::div_scaled_product(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2544)), 1.0), 2544, (1.0 - (2.0 * p.p849)));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) {s.store_add(2546, 2544, 2545);}
        s.b[2615] = (p.p849 == 0.5);s.store_scalar(2615, if s.b[2615] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) && s.b[2615]) {s.store_sqrt_scaled_input(2540, 2543, s.v[431]);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) && (!s.b[2615])) {s.store_powf_scaled_input(2540, 2543, s.v[431], p.p849);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2613])) {s.store_scale(2547, 2540, s.v[425]);s.store_mul_scale_offset_indices(2548, 2547, 2534, s.v[386], ((-1.0)) * (s.v[386]));s.store_scaled_mul(2542, 2548, 2546, p.p858);}
        s.b[2616] = (p.p863 == 0.0);s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && s.b[2616]) {s.store_scalar(2549, 0.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) {s.store_div_scaled_inputs_indices(2550, 2547, (s.v[410] * s.v[440]), 2543, 1.0);s.store_div_from_scalar(2551, (0.666666666666667 * s.v[437]), 2550);s.store_square(2552, 2551);s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);s.store_sqrt(2554, 2553);s.store_mul(2555, 2553, 2554);}
        s.b[2617] = (((-p.p849) * s.v[413]) == (-1.0));s.store_scalar(2617, if s.b[2617] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && s.b[2617]) {s.store_div_from_scalar_offset_product(2556, 1.0, 2550, 2555, 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2617])) {s.store_powf_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), ((-p.p849) * s.v[413]));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) {s.store_div_scaled_product_add_scaled_denominator_indices(2557, 2546, 2556, 1.0, 2546, 1.0, 2556, 1.0, 1.0);s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);s.store_add_scaled_value_products_indices(2560, 2553, (-s.v[437]), 2551, 2554, s.v[437], 2550, 2555, 0.5);s.store_mul_scale_offset_indices(2561, 2558, 2559, 1.0, (-1.0));s.store_square(2522, 2561);}
        s.b[2618] = (s.v[2561] > 0.0);s.store_scalar(2618, if s.b[2618] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && s.b[2618]) {s.store_div_from_scalar_offset_scaled_input(2523, 1.0, 2561, s.v[373], 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2618])) {s.store_div_from_scalar_sub_from_scalar_ad(2523, 1.0, 1.0, A::scale(s.ad_value(2561), s.v[373]));}
        s.b[2619] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));s.store_scalar(2619, if s.b[2619] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && s.b[2619]) {s.store_exp_sub(2540, 2560, 2522);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2619])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_129(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) {s.store_mul_mixed_ai(2524, A::add_scaled_inputs_product(s.ad_value(2523), 0.29214664, A::square(s.ad_value(2523)), s.v[374], A::square(s.ad_value(2523)), s.ad_value(2523), s.v[375]), 2540);}
        s.b[2620] = (s.v[2561] > 0.0);s.store_scalar(2620, if s.b[2620] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && s.b[2620]) {s.copy_ad(2562, 2524);}
        s.b[2621] = (s.v[2560] > (-230.25850929940458));s.store_scalar(2621, if s.b[2621] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2621]) {s.store_exp(2540, 2560);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2621])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) && (!s.b[2620])) {s.store_sub_scaled_inputs(2562, 2540, 2.0, 2524, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2616])) {s.store_div_scaled_inputs_indices(2563, 2562, (s.v[437] * (1.772453850905516 * 0.5)), 2558, 1.0);s.store_mul3_affine_lhs(2549, 2548, 2563, p.p863, 0.0, 2557);}
        s.b[2622] = (p.p869 == 0.0);s.store_scalar(2622, if s.b[2622] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && s.b[2622]) {s.store_scalar(2564, 0.0);}
        s.b[2623] = (p.p849 == 0.5);s.store_scalar(2623, if s.b[2623] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && s.b[2623]) {s.store_sqrt_scaled_input_ad(2540, A::sub_from_scalar(p.p846, s.ad_value(2538)), s.v[431]);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && (!s.b[2623])) {s.store_powf_scale_offset_input(2540, 2538, (-s.v[431]), ((p.p846) * (s.v[431])), p.p849);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) {s.store_div_scaled_offset_numerator_indices(2565, 2538, ((-s.v[428]) * s.v[413]), (((p.p846) * (s.v[428])) * s.v[413]), 2540, 1.0);}
        s.b[2624] = (((((-s.v[443]) / s.v[2565])) as f64).abs() < 230.25850929940458);s.store_scalar(2624, if s.b[2624] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && s.b[2624]) {s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2565), 1.0));}
        s.b[2625] = (((-s.v[443]) / s.v[2565]) < 0.0);s.store_scalar(2625, if s.b[2625] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && (!s.b[2624])) && s.b[2625]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 443, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) && (!s.b[2624])) && (!s.b[2625])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 443, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2622])) {s.store_mul_scale_offset_mixed_ai(2564, A::mul3(s.ad_value(821), s.ad_value(2565), s.ad_value(2565)), 2540, p.p869, 0.0);}
        s.b[2626] = (p.p878 > 1000.0);s.store_scalar(2626, if s.b[2626] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && s.b[2626]) {s.store_scalar(2566, 1.0);}
        s.b[2627] = (s.v[2539] > ((-s.v[445]) * p.p878));s.store_scalar(2627, if s.b[2627] { 1.0 } else { 0.0 });s.b[2628] = (p.p881 == 4.0);s.store_scalar(2628, if s.b[2628] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2626])) && s.b[2627]) && s.b[2628]) {s.store_mul_scale_offset_mixed_ai(2540, A::mul3_scaled_output(s.ad_value(2539), s.ad_value(2539), s.ad_value(2539), ((s.v[450] * s.v[450]) * s.v[450])), 2539, s.v[450], 0.0);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2626])) && s.b[2627]) && (!s.b[2628])) {s.store_powf_ad(2540, A::abs_scaled_input(s.ad_value(2539), s.v[450]), p.p881);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2626])) && s.b[2627]) {s.store_div_from_scalar_sub_from_scalar_ad(2566, 1.0, 1.0, s.ad_value(2540));}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2626])) && (!s.b[2627])) {s.store_offset_scaled(2566, 2539, s.v[453], (((((s.v[445] * p.p878)) * (s.v[453]))) + (s.v[447])));}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) {s.store_mul_scale_offset_mixed_ia(1897, 2566, A::add_scaled_inputs4(s.ad_value(2541), 1.0, s.ad_value(2542), 1.0, s.ad_value(2549), 1.0, s.ad_value(2564), 1.0), p.p29, 0.0);}
        s.b[2629] = (s.v[410] == 0.5);s.store_scalar(2629, if s.b[2629] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && s.b[2629]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2532), s.v[407]));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) && (!s.b[2629])) {s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[407])), s.v[410]);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2612])) {s.store_add_scaled_inputs3_offset_indices(1903, 2540, ((-s.v[419]) * p.p30), 821, (s.v[422] * p.p30), 2532, ((-s.v[422]) * p.p30), (s.v[419] * p.p30));}
        s.b[2630] = (s.v[649] == 0.0);s.store_scalar(2630, if s.b[2630] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2630]) {s.store_scalar(1898, 0.0);s.store_scalar(1904, 0.0);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) {s.store_scale(2541, 2531, s.v[390]);}
        s.b[2631] = ((p.p859 == 0.0) && (p.p864 == 0.0));s.store_scalar(2631, if s.b[2631] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2631]) {s.store_scalar(2542, 0.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) {s.store_sub_from_scalar(2543, s.v[396], 2537);s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));}
        s.b[2632] = (p.p850 == 0.5);s.store_scalar(2632, if s.b[2632] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) && s.b[2632]) {s.store_scalar(2545, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_130(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) && (!s.b[2632])) {s.store_scaled_add_mixed_ai(2545, A::div_scaled_product(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2544)), 1.0), 2544, (1.0 - (2.0 * p.p850)));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) {s.store_add(2546, 2544, 2545);}
        s.b[2633] = (p.p850 == 0.5);s.store_scalar(2633, if s.b[2633] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) && s.b[2633]) {s.store_sqrt_scaled_input(2540, 2543, s.v[432]);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) && (!s.b[2633])) {s.store_powf_scaled_input(2540, 2543, s.v[432], p.p850);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2631])) {s.store_scale(2547, 2540, s.v[426]);s.store_mul_scale_offset_indices(2548, 2547, 2534, s.v[387], ((-1.0)) * (s.v[387]));s.store_scaled_mul(2542, 2548, 2546, p.p859);}
        s.b[2634] = (p.p864 == 0.0);s.store_scalar(2634, if s.b[2634] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2634]) {s.store_scalar(2549, 0.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) {s.store_div_scaled_inputs_indices(2550, 2547, (s.v[411] * s.v[441]), 2543, 1.0);s.store_div_from_scalar(2551, (0.666666666666667 * s.v[438]), 2550);s.store_square(2552, 2551);s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);s.store_sqrt(2554, 2553);s.store_mul(2555, 2553, 2554);}
        s.b[2635] = (((-p.p850) * s.v[414]) == (-1.0));s.store_scalar(2635, if s.b[2635] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && s.b[2635]) {s.store_div_from_scalar_offset_product(2556, 1.0, 2550, 2555, 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2635])) {s.store_powf_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), ((-p.p850) * s.v[414]));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) {s.store_div_scaled_product_add_scaled_denominator_indices(2557, 2546, 2556, 1.0, 2546, 1.0, 2556, 1.0, 1.0);s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);s.store_add_scaled_value_products_indices(2560, 2553, (-s.v[438]), 2551, 2554, s.v[438], 2550, 2555, 0.5);s.store_mul_scale_offset_indices(2561, 2558, 2559, 1.0, (-1.0));s.store_square(2522, 2561);}
        s.b[2636] = (s.v[2561] > 0.0);s.store_scalar(2636, if s.b[2636] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && s.b[2636]) {s.store_div_from_scalar_offset_scaled_input(2523, 1.0, 2561, s.v[373], 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2636])) {s.store_div_from_scalar_sub_from_scalar_ad(2523, 1.0, 1.0, A::scale(s.ad_value(2561), s.v[373]));}
        s.b[2637] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));s.store_scalar(2637, if s.b[2637] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && s.b[2637]) {s.store_exp_sub(2540, 2560, 2522);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2637])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) {s.store_mul_mixed_ai(2524, A::add_scaled_inputs_product(s.ad_value(2523), 0.29214664, A::square(s.ad_value(2523)), s.v[374], A::square(s.ad_value(2523)), s.ad_value(2523), s.v[375]), 2540);}
        s.b[2638] = (s.v[2561] > 0.0);s.store_scalar(2638, if s.b[2638] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && s.b[2638]) {s.copy_ad(2562, 2524);}
        s.b[2639] = (s.v[2560] > (-230.25850929940458));s.store_scalar(2639, if s.b[2639] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2638])) && s.b[2639]) {s.store_exp(2540, 2560);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2638])) && (!s.b[2639])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) && (!s.b[2638])) {s.store_sub_scaled_inputs(2562, 2540, 2.0, 2524, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2634])) {s.store_div_scaled_inputs_indices(2563, 2562, (s.v[438] * (1.772453850905516 * 0.5)), 2558, 1.0);s.store_mul3_affine_lhs(2549, 2548, 2563, p.p864, 0.0, 2557);}
        s.b[2640] = (p.p870 == 0.0);s.store_scalar(2640, if s.b[2640] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2640]) {s.store_scalar(2564, 0.0);}
        s.b[2641] = (p.p850 == 0.5);s.store_scalar(2641, if s.b[2641] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && s.b[2641]) {s.store_sqrt_scaled_input_ad(2540, A::sub_from_scalar(p.p847, s.ad_value(2538)), s.v[432]);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && (!s.b[2641])) {s.store_powf_scale_offset_input(2540, 2538, (-s.v[432]), ((p.p847) * (s.v[432])), p.p850);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) {s.store_div_scaled_offset_numerator_indices(2565, 2538, ((-s.v[429]) * s.v[414]), (((p.p847) * (s.v[429])) * s.v[414]), 2540, 1.0);}
        s.b[2642] = (((((-s.v[444]) / s.v[2565])) as f64).abs() < 230.25850929940458);s.store_scalar(2642, if s.b[2642] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && s.b[2642]) {s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(444), -1.0, s.ad_value(2565), 1.0));}
        s.b[2643] = (((-s.v[444]) / s.v[2565]) < 0.0);s.store_scalar(2643, if s.b[2643] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && (!s.b[2642])) && s.b[2643]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 444, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) && (!s.b[2642])) && (!s.b[2643])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 444, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_131(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2640])) {s.store_mul_scale_offset_mixed_ai(2564, A::mul3(s.ad_value(821), s.ad_value(2565), s.ad_value(2565)), 2540, p.p870, 0.0);}
        s.b[2644] = (s.v[641] > 1000.0);s.store_scalar(2644, if s.b[2644] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2644]) {s.store_scalar(2566, 1.0);}
        s.b[2645] = (s.v[2539] > ((-s.v[445]) * s.v[641]));s.store_scalar(2645, if s.b[2645] { 1.0 } else { 0.0 });s.b[2646] = (p.p882 == 4.0);s.store_scalar(2646, if s.b[2646] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && s.b[2645]) && s.b[2646]) {s.store_mul_ad_product_lhs_mixed_ai(2540, A::mul3(A::square(A::mul(s.ad_value(2539), s.ad_value(451))), s.ad_value(2539), s.ad_value(451)), 2539, 451);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && s.b[2645]) && (!s.b[2646])) {s.store_powf_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(451))), p.p882);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && s.b[2645]) {s.store_div_from_scalar_sub_from_scalar_ad(2566, 1.0, 1.0, s.ad_value(2540));}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2644])) && (!s.b[2645])) {s.store_offset_mul_ad(2566, A::add_scaled_inputs(s.ad_value(2539), 1.0, s.ad_value(641), s.v[445]), s.ad_value(454), s.v[448]);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) {s.store_mul_scale_offset_mixed_ia(1898, 2566, A::add_scaled_inputs4(s.ad_value(2541), 1.0, s.ad_value(2542), 1.0, s.ad_value(2549), 1.0, s.ad_value(2564), 1.0), p.p29, 0.0);}
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
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {s.store_add_scaled_product_mixed_aia(473, A::mul_sub_from_scalar_rhs(s.ad_value(471), 1.0, s.ad_value(2540)), p.p30, 472, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_132(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && s.b[2647]) {s.store_add(1904, 1904, 473);}
        s.b[2650] = (s.v[411] == 0.5);s.store_scalar(2650, if s.b[2650] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) && s.b[2650]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::scale(s.ad_value(2532), s.v[408]));}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) && (!s.b[2650])) {s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[408])), s.v[411]);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2630])) && (!s.b[2647])) {s.store_add_scaled_inputs3_offset_indices(1904, 2540, ((-s.v[420]) * p.p30), 821, (s.v[423] * p.p30), 2532, ((-s.v[423]) * p.p30), (s.v[420] * p.p30));}
        if (s.b[2569] && (!s.b[2570])) {s.store_add_scaled_products3_indices(837, 647, 1896, 1.0, 648, 1897, 1.0, 649, 1898, 1.0);}
        s.b[2651] = (s.v[637] > 0.0);s.store_scalar(2651, if s.b[2651] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2651]) {s.store_mul_sub_mixed_iaa(644, 637, A::pow(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), s.ad_value(638)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(638)));s.store_add(642, 543, 644);s.store_div_from_scalar(617, 1.0, 642);s.store_div_scaled_value_offset_denominator(620, s.ad_value(620), 1.0, A::div(s.ad_value(644), s.ad_value(543)), 1.0, 1.0);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_133(
        s: &mut Scratch,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && s.b[2657]) {s.store_scaled_ln_ad(2535, A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2533), 1.0, A::offset(s.ad_value(2533), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[2569] && (!s.b[2570])) && s.b[2653]) && (!s.b[2657])) {s.store_sub_mixed_ai(2535, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2534), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2534), 1.0, A::scale_offset(s.ad_value(2534), 3.0, 1.0))))), (s.v[371] * 2.0)), 822);}
        if ((s.b[2569] && (!s.b[2570])) && s.b[2653]) {s.store_sub(2536, 684, 2535);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2537, 822, 0.5, 2536, 0.5, 822, 2536, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2538, 822, 0.5, 687, 0.5, 822, 687, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_scaled_sub_mixed_ia(2539, 822, A::sqrt_square_offset(s.ad_value(822), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[2658] = (s.v[674] == 0.0);s.store_scalar(2658, if s.b[2658] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2658]) {s.store_scalar(1899, 0.0);s.store_scalar(1905, 0.0);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) {s.store_mul(2541, 564, 2531);}
        s.b[2659] = ((s.v[523] == 0.0) && (s.v[526] == 0.0));s.store_scalar(2659, if s.b[2659] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2659]) {s.store_scalar(2542, 0.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) {s.store_sub(2543, 570, 2537);s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));}
        s.b[2660] = (s.v[512] == 0.5);s.store_scalar(2660, if s.b[2660] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && s.b[2660]) {s.store_scalar(2545, 0.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && (!s.b[2660])) {s.store_mul_scale_offset(2545, A::add(A::div_scaled_product(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2544)), 1.0), s.ad_value(2544)), A::scale(s.ad_value(512), 2.0), -1.0, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) {s.store_add(2546, 2544, 2545);}
        s.b[2661] = (s.v[512] == 0.5);s.store_scalar(2661, if s.b[2661] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && s.b[2661]) {s.store_sqrt_mul(2540, 2543, 597);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) && (!s.b[2661])) {s.store_pow_mul_base_indices(2540, 2543, 597, 512);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2659])) {s.store_mul(2547, 591, 2540);s.store_mul_ad_product_lhs_mixed_ia(2548, 561, A::offset(s.ad_value(2534), (-1.0)), 2547);s.store_mul3_lhs(2542, 523, 2548, 2546);}
        s.b[2662] = (s.v[526] == 0.0);s.store_scalar(2662, if s.b[2662] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2662]) {s.store_scalar(2549, 0.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) {s.store_mul_div_scaled_product_indices(2550, 606, 2547, 576, 1.0, 2543, 1.0);s.store_div_scaled_inputs_indices(2551, 603, 0.666666666666667, 2550, 1.0);s.store_square(2552, 2551);s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);s.store_sqrt(2554, 2553);s.store_mul(2555, 2553, 2554);}
        s.b[2663] = (((-s.v[512]) * s.v[579]) == (-1.0));s.store_scalar(2663, if s.b[2663] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && s.b[2663]) {s.store_div_from_scalar_offset_product(2556, 1.0, 2550, 2555, 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2663])) {s.store_pow_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), A::mul_scaled_lhs(s.ad_value(512), -1.0, s.ad_value(579)));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) {s.store_div_scaled_product_add_scaled_denominator_indices(2557, 2546, 2556, 1.0, 2546, 1.0, 2556, 1.0, 1.0);s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_134(
        s: &mut Scratch,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) {s.store_add_scaled_value_products_mixed_aiiii(2560, A::mul3(s.ad_value(603), s.ad_value(2551), s.ad_value(2554)), 1.0, 603, 2553, (-1.0), 2550, 2555, 0.5);s.store_mul_scale_offset_indices(2561, 2558, 2559, 1.0, (-1.0));s.store_square(2522, 2561);}
        s.b[2664] = (s.v[2561] > 0.0);s.store_scalar(2664, if s.b[2664] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && s.b[2664]) {s.store_div_from_scalar_offset_scaled_input(2523, 1.0, 2561, s.v[373], 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2664])) {s.store_div_from_scalar_sub_from_scalar_ad(2523, 1.0, 1.0, A::scale(s.ad_value(2561), s.v[373]));}
        s.b[2665] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && s.b[2665]) {s.store_exp_sub(2540, 2560, 2522);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2665])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) {s.store_mul_mixed_ai(2524, A::add_scaled_inputs_product(s.ad_value(2523), 0.29214664, A::square(s.ad_value(2523)), s.v[374], A::square(s.ad_value(2523)), s.ad_value(2523), s.v[375]), 2540);}
        s.b[2666] = (s.v[2561] > 0.0);s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && s.b[2666]) {s.copy_ad(2562, 2524);}
        s.b[2667] = (s.v[2560] > (-230.25850929940458));s.store_scalar(2667, if s.b[2667] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2666])) && s.b[2667]) {s.store_exp(2540, 2560);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2666])) && (!s.b[2667])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) && (!s.b[2666])) {s.store_sub_scaled_inputs(2562, 2540, 2.0, 2524, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2662])) {s.store_div_scaled_product_indices(2563, 603, 2562, (1.772453850905516 * 0.5), 2558, 1.0);s.store_mul_product3_indices(2549, 526, 2548, 2563, 2557, 1.0);}
        s.b[2668] = (s.v[532] == 0.0);s.store_scalar(2668, if s.b[2668] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2668]) {s.store_scalar(2564, 0.0);}
        s.b[2669] = (s.v[512] == 0.5);s.store_scalar(2669, if s.b[2669] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && s.b[2669]) {s.store_sqrt_mul_sub_lhs(2540, 509, 2538, 597);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2669])) {s.store_pow_mul_base_mixed_ai(2540, A::sub(s.ad_value(509), s.ad_value(2538)), 597, 512);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) {s.store_mul_div_scaled_product_mixed_iaii(2565, 579, A::sub(s.ad_value(509), s.ad_value(2538)), 594, 1.0, 2540, 1.0);}
        s.b[2670] = (((((-s.v[609]) / s.v[2565])) as f64).abs() < 230.25850929940458);s.store_scalar(2670, if s.b[2670] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && s.b[2670]) {s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(609), -1.0, s.ad_value(2565), 1.0));}
        s.b[2671] = (((-s.v[609]) / s.v[2565]) < 0.0);s.store_scalar(2671, if s.b[2671] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2670])) && s.b[2671]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 609, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) && (!s.b[2670])) && (!s.b[2671])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 609, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2668])) {s.store_mul_ad_product_lhs_mixed_ia(2564, 532, A::mul3(s.ad_value(822), s.ad_value(2565), s.ad_value(2565)), 2540);}
        s.b[2672] = (s.v[541] > 1000.0);s.store_scalar(2672, if s.b[2672] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2672]) {s.store_scalar(2566, 1.0);}
        s.b[2673] = (s.v[2539] > ((-s.v[445]) * s.v[541]));s.store_scalar(2673, if s.b[2673] { 1.0 } else { 0.0 });s.b[2674] = (s.v[544] == 4.0);s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && s.b[2673]) && s.b[2674]) {s.store_mul_ad_product_lhs_mixed_ai(2540, A::mul3(A::square(A::mul(s.ad_value(2539), s.ad_value(615))), s.ad_value(2539), s.ad_value(615)), 2539, 615);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && s.b[2673]) && (!s.b[2674])) {s.store_pow_abs_mul_base_indices(2540, 2539, 615, 544);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && s.b[2673]) {s.store_div_from_scalar_sub_from_scalar_ad(2566, 1.0, 1.0, s.ad_value(2540));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_135(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2672])) && (!s.b[2673])) {s.store_add_scaled_product_mixed_iai(2566, 612, 1.0, A::add_scaled_inputs(s.ad_value(2539), 1.0, s.ad_value(541), s.v[445]), 618, 1.0);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) {s.store_mul_scale_offset_mixed_ia(1899, 2566, A::add_scaled_inputs4(s.ad_value(2541), 1.0, s.ad_value(2542), 1.0, s.ad_value(2549), 1.0, s.ad_value(2564), 1.0), p.p29, 0.0);}
        s.b[2675] = (s.v[576] == 0.5);s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && s.b[2675]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(573)));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) && (!s.b[2675])) {s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2532, 573, 576);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2658])) {s.store_add_scaled_product_mixed_aia(1905, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2540)), p.p30, 588, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);}
        s.b[2676] = (s.v[675] == 0.0);s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2676]) {s.store_scalar(1900, 0.0);s.store_scalar(1906, 0.0);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) {s.store_mul(2541, 565, 2531);}
        s.b[2677] = ((s.v[524] == 0.0) && (s.v[527] == 0.0));s.store_scalar(2677, if s.b[2677] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2677]) {s.store_scalar(2542, 0.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) {s.store_sub(2543, 571, 2537);s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));}
        s.b[2678] = (s.v[513] == 0.5);s.store_scalar(2678, if s.b[2678] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && s.b[2678]) {s.store_scalar(2545, 0.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && (!s.b[2678])) {s.store_mul_scale_offset(2545, A::add(A::div_scaled_product(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2544)), 1.0), s.ad_value(2544)), A::scale(s.ad_value(513), 2.0), -1.0, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) {s.store_add(2546, 2544, 2545);}
        s.b[2679] = (s.v[513] == 0.5);s.store_scalar(2679, if s.b[2679] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && s.b[2679]) {s.store_sqrt_mul(2540, 2543, 598);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) && (!s.b[2679])) {s.store_pow_mul_base_indices(2540, 2543, 598, 513);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2677])) {s.store_mul(2547, 592, 2540);s.store_mul_ad_product_lhs_mixed_ia(2548, 562, A::offset(s.ad_value(2534), (-1.0)), 2547);s.store_mul3_lhs(2542, 524, 2548, 2546);}
        s.b[2680] = (s.v[527] == 0.0);s.store_scalar(2680, if s.b[2680] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2680]) {s.store_scalar(2549, 0.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) {s.store_mul_div_scaled_product_indices(2550, 607, 2547, 577, 1.0, 2543, 1.0);s.store_div_scaled_inputs_indices(2551, 604, 0.666666666666667, 2550, 1.0);s.store_square(2552, 2551);s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);s.store_sqrt(2554, 2553);s.store_mul(2555, 2553, 2554);}
        s.b[2681] = (((-s.v[513]) * s.v[580]) == (-1.0));s.store_scalar(2681, if s.b[2681] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && s.b[2681]) {s.store_div_from_scalar_offset_product(2556, 1.0, 2550, 2555, 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2681])) {s.store_pow_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), A::mul_scaled_lhs(s.ad_value(513), -1.0, s.ad_value(580)));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) {s.store_div_scaled_product_add_scaled_denominator_indices(2557, 2546, 2556, 1.0, 2546, 1.0, 2556, 1.0, 1.0);s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_136(
        s: &mut Scratch,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) {s.store_add_scaled_value_products_mixed_aiiii(2560, A::mul3(s.ad_value(604), s.ad_value(2551), s.ad_value(2554)), 1.0, 604, 2553, (-1.0), 2550, 2555, 0.5);s.store_mul_scale_offset_indices(2561, 2558, 2559, 1.0, (-1.0));s.store_square(2522, 2561);}
        s.b[2682] = (s.v[2561] > 0.0);s.store_scalar(2682, if s.b[2682] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && s.b[2682]) {s.store_div_from_scalar_offset_scaled_input(2523, 1.0, 2561, s.v[373], 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2682])) {s.store_div_from_scalar_sub_from_scalar_ad(2523, 1.0, 1.0, A::scale(s.ad_value(2561), s.v[373]));}
        s.b[2683] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));s.store_scalar(2683, if s.b[2683] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && s.b[2683]) {s.store_exp_sub(2540, 2560, 2522);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2683])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) {s.store_mul_mixed_ai(2524, A::add_scaled_inputs_product(s.ad_value(2523), 0.29214664, A::square(s.ad_value(2523)), s.v[374], A::square(s.ad_value(2523)), s.ad_value(2523), s.v[375]), 2540);}
        s.b[2684] = (s.v[2561] > 0.0);s.store_scalar(2684, if s.b[2684] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && s.b[2684]) {s.copy_ad(2562, 2524);}
        s.b[2685] = (s.v[2560] > (-230.25850929940458));s.store_scalar(2685, if s.b[2685] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2684])) && s.b[2685]) {s.store_exp(2540, 2560);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2684])) && (!s.b[2685])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) && (!s.b[2684])) {s.store_sub_scaled_inputs(2562, 2540, 2.0, 2524, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2680])) {s.store_div_scaled_product_indices(2563, 604, 2562, (1.772453850905516 * 0.5), 2558, 1.0);s.store_mul_product3_indices(2549, 527, 2548, 2563, 2557, 1.0);}
        s.b[2686] = (s.v[533] == 0.0);s.store_scalar(2686, if s.b[2686] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2686]) {s.store_scalar(2564, 0.0);}
        s.b[2687] = (s.v[513] == 0.5);s.store_scalar(2687, if s.b[2687] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && s.b[2687]) {s.store_sqrt_mul_sub_lhs(2540, 510, 2538, 598);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2687])) {s.store_pow_mul_base_mixed_ai(2540, A::sub(s.ad_value(510), s.ad_value(2538)), 598, 513);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) {s.store_mul_div_scaled_product_mixed_iaii(2565, 580, A::sub(s.ad_value(510), s.ad_value(2538)), 595, 1.0, 2540, 1.0);}
        s.b[2688] = (((((-s.v[610]) / s.v[2565])) as f64).abs() < 230.25850929940458);s.store_scalar(2688, if s.b[2688] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && s.b[2688]) {s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(610), -1.0, s.ad_value(2565), 1.0));}
        s.b[2689] = (((-s.v[610]) / s.v[2565]) < 0.0);s.store_scalar(2689, if s.b[2689] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2688])) && s.b[2689]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 610, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) && (!s.b[2688])) && (!s.b[2689])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 610, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2686])) {s.store_mul_ad_product_lhs_mixed_ia(2564, 533, A::mul3(s.ad_value(822), s.ad_value(2565), s.ad_value(2565)), 2540);}
        s.b[2690] = (s.v[542] > 1000.0);s.store_scalar(2690, if s.b[2690] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2690]) {s.store_scalar(2566, 1.0);}
        s.b[2691] = (s.v[2539] > ((-s.v[445]) * s.v[542]));s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });s.b[2692] = (s.v[545] == 4.0);s.store_scalar(2692, if s.b[2692] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && s.b[2691]) && s.b[2692]) {s.store_mul_ad_product_lhs_mixed_ai(2540, A::mul3(A::square(A::mul(s.ad_value(2539), s.ad_value(616))), s.ad_value(2539), s.ad_value(616)), 2539, 616);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && s.b[2691]) && (!s.b[2692])) {s.store_pow_abs_mul_base_indices(2540, 2539, 616, 545);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && s.b[2691]) {s.store_div_from_scalar_sub_from_scalar_ad(2566, 1.0, 1.0, s.ad_value(2540));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_137(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2690])) && (!s.b[2691])) {s.store_add_scaled_product_mixed_iai(2566, 613, 1.0, A::add_scaled_inputs(s.ad_value(2539), 1.0, s.ad_value(542), s.v[445]), 619, 1.0);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) {s.store_mul_scale_offset_mixed_ia(1900, 2566, A::add_scaled_inputs4(s.ad_value(2541), 1.0, s.ad_value(2542), 1.0, s.ad_value(2549), 1.0, s.ad_value(2564), 1.0), p.p29, 0.0);}
        s.b[2693] = (s.v[577] == 0.5);s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && s.b[2693]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(574)));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) && (!s.b[2693])) {s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2532, 574, 577);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2676])) {s.store_add_scaled_product_mixed_aia(1906, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2540)), p.p30, 589, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);}
        s.b[2694] = (s.v[676] == 0.0);s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2694]) {s.store_scalar(1901, 0.0);s.store_scalar(1907, 0.0);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) {s.store_mul(2541, 566, 2531);}
        s.b[2695] = ((s.v[525] == 0.0) && (s.v[528] == 0.0));s.store_scalar(2695, if s.b[2695] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2695]) {s.store_scalar(2542, 0.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) {s.store_sub(2543, 572, 2537);s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));}
        s.b[2696] = (s.v[514] == 0.5);s.store_scalar(2696, if s.b[2696] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && s.b[2696]) {s.store_scalar(2545, 0.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && (!s.b[2696])) {s.store_mul_scale_offset(2545, A::add(A::div_scaled_product(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2544)), 1.0), s.ad_value(2544)), A::scale(s.ad_value(514), 2.0), -1.0, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) {s.store_add(2546, 2544, 2545);}
        s.b[2697] = (s.v[514] == 0.5);s.store_scalar(2697, if s.b[2697] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && s.b[2697]) {s.store_sqrt_mul(2540, 2543, 599);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) && (!s.b[2697])) {s.store_pow_mul_base_indices(2540, 2543, 599, 514);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2695])) {s.store_mul(2547, 593, 2540);s.store_mul_ad_product_lhs_mixed_ia(2548, 563, A::offset(s.ad_value(2534), (-1.0)), 2547);s.store_mul3_lhs(2542, 525, 2548, 2546);}
        s.b[2698] = (s.v[528] == 0.0);s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2698]) {s.store_scalar(2549, 0.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) {s.store_mul_div_scaled_product_indices(2550, 608, 2547, 578, 1.0, 2543, 1.0);s.store_div_scaled_inputs_indices(2551, 605, 0.666666666666667, 2550, 1.0);s.store_square(2552, 2551);s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);s.store_sqrt(2554, 2553);s.store_mul(2555, 2553, 2554);}
        s.b[2699] = (((-s.v[514]) * s.v[581]) == (-1.0));s.store_scalar(2699, if s.b[2699] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2699]) {s.store_div_from_scalar_offset_product(2556, 1.0, 2550, 2555, 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2699])) {s.store_pow_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), A::mul_scaled_lhs(s.ad_value(514), -1.0, s.ad_value(581)));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) {s.store_div_scaled_product_add_scaled_denominator_indices(2557, 2546, 2556, 1.0, 2546, 1.0, 2556, 1.0, 1.0);s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_138(
        s: &mut Scratch,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) {s.store_add_scaled_value_products_mixed_aiiii(2560, A::mul3(s.ad_value(605), s.ad_value(2551), s.ad_value(2554)), 1.0, 605, 2553, (-1.0), 2550, 2555, 0.5);s.store_mul_scale_offset_indices(2561, 2558, 2559, 1.0, (-1.0));s.store_square(2522, 2561);}
        s.b[2700] = (s.v[2561] > 0.0);s.store_scalar(2700, if s.b[2700] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2700]) {s.store_div_from_scalar_offset_scaled_input(2523, 1.0, 2561, s.v[373], 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2700])) {s.store_div_from_scalar_sub_from_scalar_ad(2523, 1.0, 1.0, A::scale(s.ad_value(2561), s.v[373]));}
        s.b[2701] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));s.store_scalar(2701, if s.b[2701] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2701]) {s.store_exp_sub(2540, 2560, 2522);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2701])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) {s.store_mul_mixed_ai(2524, A::add_scaled_inputs_product(s.ad_value(2523), 0.29214664, A::square(s.ad_value(2523)), s.v[374], A::square(s.ad_value(2523)), s.ad_value(2523), s.v[375]), 2540);}
        s.b[2702] = (s.v[2561] > 0.0);s.store_scalar(2702, if s.b[2702] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2702]) {s.copy_ad(2562, 2524);}
        s.b[2703] = (s.v[2560] > (-230.25850929940458));s.store_scalar(2703, if s.b[2703] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2702])) && s.b[2703]) {s.store_exp(2540, 2560);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2702])) && (!s.b[2703])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2702])) {s.store_sub_scaled_inputs(2562, 2540, 2.0, 2524, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2698])) {s.store_div_scaled_product_indices(2563, 605, 2562, (1.772453850905516 * 0.5), 2558, 1.0);s.store_mul_product3_indices(2549, 528, 2548, 2563, 2557, 1.0);}
        s.b[2704] = (s.v[534] == 0.0);s.store_scalar(2704, if s.b[2704] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2704]) {s.store_scalar(2564, 0.0);}
        s.b[2705] = (s.v[514] == 0.5);s.store_scalar(2705, if s.b[2705] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && s.b[2705]) {s.store_sqrt_mul_sub_lhs(2540, 511, 2538, 599);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2705])) {s.store_pow_mul_base_mixed_ai(2540, A::sub(s.ad_value(511), s.ad_value(2538)), 599, 514);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) {s.store_mul_div_scaled_product_mixed_iaii(2565, 581, A::sub(s.ad_value(511), s.ad_value(2538)), 596, 1.0, 2540, 1.0);}
        s.b[2706] = (((((-s.v[611]) / s.v[2565])) as f64).abs() < 230.25850929940458);s.store_scalar(2706, if s.b[2706] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && s.b[2706]) {s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(611), -1.0, s.ad_value(2565), 1.0));}
        s.b[2707] = (((-s.v[611]) / s.v[2565]) < 0.0);s.store_scalar(2707, if s.b[2707] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2706])) && s.b[2707]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 611, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) && (!s.b[2706])) && (!s.b[2707])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 611, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2704])) {s.store_mul_ad_product_lhs_mixed_ia(2564, 534, A::mul3(s.ad_value(822), s.ad_value(2565), s.ad_value(2565)), 2540);}
        s.b[2708] = (s.v[642] > 1000.0);s.store_scalar(2708, if s.b[2708] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2708]) {s.store_scalar(2566, 1.0);}
        s.b[2709] = (s.v[2539] > ((-s.v[445]) * s.v[642]));s.store_scalar(2709, if s.b[2709] { 1.0 } else { 0.0 });s.b[2710] = (s.v[546] == 4.0);s.store_scalar(2710, if s.b[2710] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && s.b[2709]) && s.b[2710]) {s.store_mul_ad_product_lhs_mixed_ai(2540, A::mul3(A::square(A::mul(s.ad_value(2539), s.ad_value(617))), s.ad_value(2539), s.ad_value(617)), 2539, 617);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && s.b[2709]) && (!s.b[2710])) {s.store_pow_abs_mul_base_indices(2540, 2539, 617, 546);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && s.b[2709]) {s.store_div_from_scalar_sub_from_scalar_ad(2566, 1.0, 1.0, s.ad_value(2540));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_139(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2708])) && (!s.b[2709])) {s.store_add_scaled_product_mixed_iai(2566, 614, 1.0, A::add_scaled_inputs(s.ad_value(2539), 1.0, s.ad_value(642), s.v[445]), 620, 1.0);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) {s.store_mul_scale_offset_mixed_ia(1901, 2566, A::add_scaled_inputs4(s.ad_value(2541), 1.0, s.ad_value(2542), 1.0, s.ad_value(2549), 1.0, s.ad_value(2564), 1.0), p.p29, 0.0);}
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
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) && (!s.b[2713])) {s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2568, 630, 631);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && s.b[2711]) {s.store_add_scaled_product_mixed_aia(473, A::mul_sub_from_scalar_rhs(s.ad_value(634), 1.0, s.ad_value(2540)), p.p30, 635, A::sub(s.ad_value(2567), s.ad_value(2568)), p.p30);s.store_add(1907, 1907, 473);}
        s.b[2714] = (s.v[578] == 0.5);s.store_scalar(2714, if s.b[2714] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) && s.b[2714]) {s.store_sqrt_sub_from_scalar_ad(2540, 1.0, A::mul(s.ad_value(2532), s.ad_value(575)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_140(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) && (!s.b[2714])) {s.store_pow_sub_from_scalar_mul_base_indices(2540, 1.0, 2532, 575, 578);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2694])) && (!s.b[2711])) {s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2540)), p.p30, 590, A::sub(s.ad_value(822), s.ad_value(2532)), p.p30);}
        if (s.b[2569] && (!s.b[2570])) {s.store_add_scaled_products3_indices(838, 674, 1899, 1.0, 675, 1900, 1.0, 676, 1901, 1.0);}
        s.b[2715] = (s.v[820] > 0.0);s.store_scalar(2715, if s.b[2715] { 1.0 } else { 0.0 });s.b[2716] = (s.v[298] > 0.0);s.store_scalar(2716, if s.b[2716] { 1.0 } else { 0.0 });s.b[2717] = (s.v[299] > 0.0);s.store_scalar(2717, if s.b[2717] { 1.0 } else { 0.0 });s.b[2718] = (s.v[300] > 0.0);s.store_scalar(2718, if s.b[2718] { 1.0 } else { 0.0 });s.b[2719] = (s.v[301] > 0.0);s.store_scalar(2719, if s.b[2719] { 1.0 } else { 0.0 });s.b[2720] = (s.v[302] > 0.0);s.store_scalar(2720, if s.b[2720] { 1.0 } else { 0.0 });s.b[2721] = (s.v[303] > 0.0);s.store_scalar(2721, if s.b[2721] { 1.0 } else { 0.0 });s.b[2722] = (s.v[304] > 0.0);s.store_scalar(2722, if s.b[2722] { 1.0 } else { 0.0 });s.store_scalar(1915, 0.0);s.store_scalar(2723, 0.0);s.store_scalar(2724, 0.0);s.b[2725] = (s.v[299] > 0.0);s.store_scalar(2725, if s.b[2725] { 1.0 } else { 0.0 });
        if s.b[2725] {s.store_mul_ad_product_rhs(2723, 801, A::voltage(ctx, nodes, Some(2), Some(7)), A::voltage(ctx, nodes, Some(2), Some(7)));}
        s.b[2726] = (s.v[300] > 0.0);s.store_scalar(2726, if s.b[2726] { 1.0 } else { 0.0 });
        if s.b[2726] {s.store_mul_ad_product_rhs(2724, 802, A::voltage(ctx, nodes, Some(0), Some(8)), A::voltage(ctx, nodes, Some(0), Some(8)));}
        s.b[2727] = (s.v[172] > 0.001);s.store_scalar(2727, if s.b[2727] { 1.0 } else { 0.0 });
        if s.b[2727] {s.store_add_scaled_inputs3_mixed_aii(1915, A::add_scaled_products(A::add(s.ad_value(827), s.ad_value(835)), s.ad_value(815), 1.0, s.ad_value(836), A::add(s.ad_value(815), s.ad_value(816)), 1.0), 1.0, 2723, 1.0, 2724, 1.0);}
        s.store_add_scaled_inputs3_indices(839, 840, (-1.0), 841, (-1.0), 842, (-1.0));s.store_add(843, 843, 1894);s.store_add(844, 844, 1895);s.store_add_scaled_products3_indices(846, 647, 1902, 1.0, 648, 1903, 1.0, 649, 1904, 1.0);s.store_add_scaled_products3_indices(847, 674, 1905, 1.0, 675, 1906, 1.0, 676, 1907, 1.0);s.b[2729] = (s.v[820] < 0.0);s.store_scalar(2729, if s.b[2729] { 1.0 } else { 0.0 });
        if s.b[2729] {s.copy_ad(2728, 842);s.copy_ad(842, 839);s.copy_ad(839, 2728);}
        s.store_scalar(2746, 0.0);s.store_scalar(2741, 0.0);s.store_scalar(848, 1e-40);s.store_scalar(850, 0.0);s.store_scalar(852, 0.0);s.store_mul(849, 1888, 1879);s.store_scalar(851, 0.0);s.store_scalar(2748, 0.0);s.b[2762] = ((s.v[1813] > 0.0) && (s.v[1917] > 0.0));s.store_scalar(2762, if s.b[2762] { 1.0 } else { 0.0 });s.b[2764] = (p.p32 > 0.0);s.store_scalar(2764, if s.b[2764] { 1.0 } else { 0.0 });
        if (s.b[2762] && s.b[2764]) {s.store_div(2733, 1850, 1848);s.store_div(2734, 1849, 1850);s.store_scaled_div(2735, 1844, 2733, (0.5 * 0.16666666666666666));s.store_square(2736, 2735);s.store_offset_div(2737, 2733, 1861, (-1.0));}
        if (s.b[2762] && s.b[2764]) {
            if ((1.0 - (12.0 * (s.v[2737] * s.v[2736]))) > 1e-20) {
                s.store_sub_from_scalar_scaled_mul(2738, 1.0, 2737, 2736, 12.0);
            } else {
                s.store_scalar(2738, 1e-20);
            }
        }
        if (s.b[2762] && s.b[2764]) {s.store_div_from_scalar_square_ad(2739, 1.0, s.ad_value(2738));s.store_mul3_lhs(2740, 1917, 1850, 1860);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_141(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2762] && s.b[2764]) {s.store_add_scaled_inputs3_mixed_iia(2741, 2734, 1.0, 2736, 12.0, A::mul3_scaled_output(A::offset(s.ad_value(2734), 1.0), s.ad_value(2736), s.ad_value(2737), 24.0), -1.0);}
        if (s.b[2762] && s.b[2764]) {
            if (s.v[2741] > 1e-40) {
            } else {
                s.store_scalar(2741, 1e-40);
            }
        }
        if (s.b[2762] && s.b[2764]) {s.store_mul3_lhs(2741, 2740, 2739, 2741);}
        s.b[2765] = (s.v[275] > 0.0);s.store_scalar(2765, if s.b[2765] { 1.0 } else { 0.0 });
        if ((s.b[2762] && s.b[2764]) && s.b[2765]) {s.store_div(2742, 1854, 1853);s.store_mul_ad_product_lhs_mixed_ai(2743, A::square(s.ad_value(2742)), 1844, 1844);}
        s.b[2766] = (s.v[0] == (-1.0));s.store_scalar(2766, if s.b[2766] { 1.0 } else { 0.0 });
        if (((s.b[2762] && s.b[2764]) && s.b[2765]) && s.b[2766]) {s.store_div_scaled_value_offset_denominator(2743, s.ad_value(2743), 1.0, A::mul(s.ad_value(2742), s.ad_value(1844)), 1.0, 1.0);}
        if ((s.b[2762] && s.b[2764]) && s.b[2765]) {s.store_mul_scale_offset_mixed_ia(2744, 1853, A::sqrt(A::scale_offset(s.ad_value(2743), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div_scaled_value_by_product_indices(2745, 1853, 1.0, 2744, 2738, 1.0);s.store_mul_ad_product_lhs_mixed_ai(2746, A::mul3(s.ad_value(799), s.ad_value(827), s.ad_value(1841)), 2745, 2745);s.store_add_div_rhs_indices(2741, 2741, 2746, 1919);}
        if (s.b[2762] && s.b[2764]) {s.store_sqrt_mul(851, 1920, 2741);}
        s.b[2767] = ((((p.p50 == 1.0) && (s.v[1920] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0));s.store_scalar(2767, if s.b[2767] { 1.0 } else { 0.0 });
        if (s.b[2762] && s.b[2767]) {s.store_sub_ad(848, A::add_scaled_product(s.ad_value(2734), 0.08333333333333333, s.ad_value(2736), A::sub_scaled_inputs(A::offset(s.ad_value(2734), 0.2), 1.0, s.ad_value(2736), 12.0), (-1.0)), A::mul3_scaled_output(s.ad_value(2736), A::sub_scaled_inputs(A::offset(s.ad_value(2734), 1.0), 1.0, s.ad_value(2736), 12.0), s.ad_value(2737), 1.6));}
        if (s.b[2762] && s.b[2767]) {
            if (s.v[848] > 1e-40) {
            } else {
                s.store_scalar(848, 1e-40);
            }
        }
        if (s.b[2762] && s.b[2767]) {s.store_mul_div_lhs(848, 2739, 2740, 848);s.store_mul_ad_product_rhs_mixed_ia(2747, 2739, 2735, A::add_scaled_sub_value_product(1.0, A::scale(s.ad_value(2736), 12.0), 1.0, A::add_scaled_inputs_product(s.ad_value(2734), 1.0, s.ad_value(2736), 19.2, s.ad_value(2734), s.ad_value(2736), (-12.0)), s.ad_value(2737), (-1.0)));s.store_div_scaled_product3_mixed_aiia(849, A::square(s.ad_value(1892)), 1888, 1879, 1.0, A::square(s.ad_value(1890)), 1.0);}
        s.b[2768] = (s.v[275] > 0.0);s.store_scalar(2768, if s.b[2768] { 1.0 } else { 0.0 });
        if ((s.b[2762] && s.b[2767]) && s.b[2768]) {s.store_add_mixed_ia(848, 848, A::div_scaled_product(s.ad_value(2746), A::scale_offset(s.ad_value(2736), 12.0, 1.0), 1.0, A::mul3_scaled_output(s.ad_value(2740), s.ad_value(2740), s.ad_value(1919), 12.0), 1.0));s.store_sub_mixed_ia(2747, 2747, A::div_scaled_product3_by_product(s.ad_value(2746), s.ad_value(2735), A::offset(s.ad_value(2737), 1.0), 1.0, s.ad_value(2740), s.ad_value(1919), 1.0));}
        if (s.b[2762] && s.b[2767]) {s.store_sqrt_div(2748, 1920, 848);}
        s.b[2769] = (s.v[851] <= 0.0);s.store_scalar(2769, if s.b[2769] { 1.0 } else { 0.0 });
        if ((s.b[2762] && s.b[2767]) && s.b[2769]) {s.store_scalar(852, 0.0);}
        if ((s.b[2762] && s.b[2767]) && (!s.b[2769])) {s.store_div_scaled_product_indices(852, 2747, 2748, 1.0, 851, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_142(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2762] && s.b[2767]) {
            if (s.v[852] > 0.0) {
                if (s.v[852] < 1.0) {
                } else {
                    s.store_scalar(852, 1.0);
                }
            } else {
                s.store_scalar(852, 0.0);
            }
        }
        if (s.b[2762] && s.b[2767]) {s.store_div_scaled_product_indices(850, 852, 851, 1.0, 2748, 1.0);}
        s.b[2771] = (((p.p46 != 0.0) && (s.v[285] > 0.0)) && (s.v[1864] > 0.0));s.store_scalar(2771, if s.b[2771] { 1.0 } else { 0.0 });
        if s.b[2771] {s.store_div_scaled_inputs_indices(1930, 1867, 4.0, 1925, 1.0);s.store_mul(1930, 760, 1916);s.store_mul(1930, 1848, 1861);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_0(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[981] = (p.p37 >= 0.0);s.store_scalar(981, if s.b[981] { 1.0 } else { 0.0 });
        if s.b[981] {s.store_scalar(0, 1.0);}
        if (!s.b[981]) {s.store_scalar(0, (-1.0));}
        s.store_scalar(756, (8.8541878176e-12 * 11.8));s.store_scalar(351, (273.15 + p.p38));s.store_scalar(475, 0.0);s.b[982] = (p.p944 > 0.5);s.store_scalar(982, if s.b[982] { 1.0 } else { 0.0 });
        if s.b[982] {s.store_scalar(475, 1.0);}
        if (!s.b[982]) {s.store_scalar(475, 0.0);}
        s.store_scalar(365, (273.15 + p.p840));s.store_scalar(368, (1.3806505e-23 / 1.6021918e-19));s.store_scalar(369, (s.v[368] * s.v[365]));s.store_scalar(370, (1.0 / s.v[369]));s.store_scalar(376, ((-((0.000702 * s.v[365]) * s.v[365])) / (1108.0 + s.v[365])));s.store_scalar(379, (p.p851 + s.v[376]));s.store_scalar(380, (p.p852 + s.v[376]));s.store_scalar(381, (p.p853 + s.v[376]));s.store_scalar(409, (1.0 - p.p848));s.store_scalar(410, (1.0 - p.p849));s.store_scalar(411, (1.0 - p.p850));s.store_scalar(412, (1.0 / s.v[409]));s.store_scalar(413, (1.0 / s.v[410]));s.store_scalar(414, (1.0 / s.v[411]));s.store_scalar(424, (s.v[756] / p.p842));s.store_scalar(425, ((p.p860 * s.v[756]) / p.p843));s.store_scalar(426, ((p.p861 * s.v[756]) / p.p844));s.store_scalar(427, (1.0 / s.v[424]));s.store_scalar(428, (1.0 / s.v[425]));s.store_scalar(429, (1.0 / s.v[426]));s.store_scalar(430, (1.0 / p.p845));s.store_scalar(431, (1.0 / p.p846));s.store_scalar(432, (1.0 / p.p847));s.store_scalar(445, (1.0 - (1.0 / p.p841)));s.store_scalar(449, (1.0 / p.p877));s.store_scalar(450, (1.0 / p.p878));s.store_scalar(451, (1.0 / p.p879));s.b[983] = ((((p.p883 != 1.0) || (p.p884 != 1.0)) || (p.p885 != 1.0)) || (p.p886 != 1.0));s.store_scalar(983, if s.b[983] { 1.0 } else { 0.0 });
        if s.b[983] {s.store_scalar(474, 1.0);}
        if (!s.b[983]) {s.store_scalar(474, 0.0);}
        s.b[984] = (s.v[474] == 1.0);s.store_scalar(984, if s.b[984] { 1.0 } else { 0.0 });
        if s.b[984] {s.store_scalar(458, (if ((p.p844 * p.p883) > 1e-18) { (p.p844 * p.p883) } else { 1e-18 }));}
        if s.b[984] {s.store_scalar(459, (if ((p.p847 * p.p884) > 0.05) { (p.p847 * p.p884) } else { 0.05 }));}
        if s.b[984] {s.store_scalar(460, (if ((if ((p.p850 * p.p885) > 0.05) { (p.p850 * p.p885) } else { 0.05 }) < 0.95) { (if ((p.p850 * p.p885) > 0.05) { (p.p850 * p.p885) } else { 0.05 }) } else { 0.95 }));}
        if s.b[984] {s.store_scalar(461, (p.p853 * p.p886));s.store_primal_offset(463, 461, s.v[376]);s.store_primal_sub_from_scalar(468, 1.0, 460);s.store_primal_div_from_scalar(469, 1.0, 468);}
        s.b[985] = (p.p44 == 0.0);s.store_scalar(985, if s.b[985] { 1.0 } else { 0.0 });
        if s.b[985] {s.store_scalar(506, p.p842);s.store_scalar(507, p.p843);s.store_scalar(508, p.p844);s.store_scalar(509, p.p845);s.store_scalar(510, p.p846);s.store_scalar(511, p.p847);s.store_scalar(512, p.p848);s.store_scalar(513, p.p849);s.store_scalar(514, p.p850);s.store_scalar(515, p.p851);s.store_scalar(516, p.p852);s.store_scalar(517, p.p853);s.store_scalar(518, p.p854);s.store_scalar(519, p.p855);s.store_scalar(520, p.p856);s.store_scalar(523, p.p857);s.store_scalar(524, p.p858);s.store_scalar(525, p.p859);s.store_scalar(521, p.p860);s.store_scalar(522, p.p861);s.store_scalar(526, p.p862);s.store_scalar(527, p.p863);s.store_scalar(528, p.p864);s.store_scalar(529, p.p865);s.store_scalar(530, p.p866);s.store_scalar(531, p.p867);s.store_scalar(532, p.p868);s.store_scalar(533, p.p869);s.store_scalar(534, p.p870);s.store_scalar(535, p.p871);s.store_scalar(536, p.p872);s.store_scalar(537, p.p873);s.store_scalar(538, p.p874);s.store_scalar(539, p.p875);s.store_scalar(540, p.p876);s.store_scalar(541, p.p877);s.store_scalar(542, p.p878);s.store_scalar(543, p.p879);s.store_scalar(544, p.p880);s.store_scalar(545, p.p881);s.store_scalar(546, p.p882);s.store_scalar(554, p.p946);s.store_scalar(637, p.p889);}
    }
}
