#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_112(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {s.store_mul3_lhs(2451, 2446, 2398, 2413);}
        s.b[2570] = (s.v[217] < 0.0);s.store_scalar(2570, if s.b[2570] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2570]) {s.store_div_from_scalar_sub_from_scalar_ad(2452, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2397)));}
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2570])) {s.store_offset_mul(2452, 217, 2397, 1.0);}
        s.b[2571] = (s.v[218] < 0.0);s.store_scalar(2571, if s.b[2571] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2571]) {s.store_sub_from_scalar_scaled_mul(2453, 1.0, 218, 2450, 1.0);}
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2571])) {s.store_div_from_scalar_offset_product(2453, 1.0, 218, 2450, 1.0);}
        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {s.store_mul_product3_indices(2454, 2450, 757, 2452, 2453, 1.0);s.store_mul_add_scaled_product_rhs_indices(2455, 774, 2451, 1.0, 775, 2450, 1.0);s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2445), 1.0, A::add(s.ad_value(2445), s.ad_value(2444)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2456, A::pow(A::mul(s.ad_value(2455), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);s.store_mul_add_mixed_iai(2457, 2448, A::offset(s.ad_value(2456), 1.0), 2454);}
        s.b[2572] = (s.v[221] < 0.0);s.store_scalar(2572, if s.b[2572] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2572]) {s.store_div_from_scalar_sub_from_scalar_ad(2458, 1.0, 1.0, A::mul(s.ad_value(221), s.ad_value(2397)));}
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2572])) {s.store_offset_mul(2458, 221, 2397, 1.0);}
        if (((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) {s.store_mul(2029, 2450, 2458);s.store_div_add_scaled_inputs_rhs_indices(2459, 2029, 223, 1.0, 2029, 1.0);}
        s.b[2573] = (s.v[222] < 0.0);s.store_scalar(2573, if s.b[2573] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && s.b[2573]) {s.store_div_from_scalar_sub_from_scalar_ad(2460, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2459)));}
        if ((((s.b[2547] && s.b[2548]) && s.b[2565]) && s.b[2569]) && (!s.b[2573])) {s.store_offset_mul(2460, 222, 2459, 1.0);}
        if (s.b[2547] && (!s.b[2548])) {s.copy_ad(2395, 1822);s.copy_ad(2397, 1823);s.copy_ad(2413, 1824);s.copy_ad(2414, 1825);s.copy_ad(2398, 1826);s.copy_ad(2399, 1827);s.copy_ad(2415, 1828);s.copy_ad(2417, 1829);s.copy_ad(2422, 1830);s.copy_ad(2423, 1831);s.copy_ad(2434, 1832);}
        let (t0,) = {
    if (s.b[2547] && (!s.b[2548])) {
        (s.v[1833],)
    } else {
        (s.v[2435],)
    }
};
        s.store_scalar(2435, t0);
        if (s.b[2547] && (!s.b[2548])) {s.copy_ad(2436, 1834);s.copy_ad(2543, 1835);s.copy_ad(2438, 1836);s.copy_ad(2437, 1837);s.copy_ad(2440, 1838);s.copy_ad(2441, 1839);s.copy_ad(2442, 1840);s.copy_ad(2443, 1841);s.copy_ad(2445, 1842);s.copy_ad(2444, 1843);s.copy_ad(2446, 1844);s.copy_ad(2447, 1845);s.copy_ad(2448, 1846);s.copy_ad(2449, 1847);s.copy_ad(2450, 1848);s.copy_ad(2451, 1849);s.copy_ad(2452, 1850);s.copy_ad(2453, 1851);s.copy_ad(2457, 1852);s.copy_ad(2458, 1853);s.copy_ad(2460, 1854);}
        if s.b[2547] {s.copy_ad(2393, 720);s.copy_ad(2394, 777);}
        s.b[2574] = (p.p48 != 0.0);s.store_scalar(2574, if s.b[2574] { 1.0 } else { 0.0 });
        if (s.b[2547] && s.b[2574]) {s.copy_ad(2393, 721);s.copy_ad(2394, 778);}
        if s.b[2547] {s.store_scalar(2462, 0.0);s.store_scale(2461, 2413, 4.60517018598809);s.copy_ad(2478, 2461);s.copy_ad(2479, 826);s.store_mul(2480, 826, 2414);s.copy_ad(2484, 2437);s.store_scalar(2485, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_113(
        s: &mut Scratch,
    ) {
        if s.b[2547] {s.store_scalar(2488, 0.0);s.copy_ad(2490, 2443);s.copy_ad(2491, 2445);s.copy_ad(2493, 2444);s.copy_ad(2494, 2451);s.copy_ad(2495, 2437);s.copy_ad(2496, 2443);s.copy_ad(2498, 2444);s.copy_ad(2499, 2445);s.store_sub(2500, 2417, 2437);s.store_scalar(2501, 1.0);s.store_scalar(2503, 1.0);s.store_scalar(2502, 0.0);s.copy_ad(2512, 2450);s.store_mul(2516, 2500, 2413);s.store_scalar(2513, 0.0);s.copy_ad(2514, 2451);s.store_scalar(2519, 0.0);s.store_scalar(2518, 1.0);s.copy_ad(2521, 2393);s.copy_ad(2520, 2516);}
        s.b[2575] = (s.v[2417] > 0.0);s.store_scalar(2575, if s.b[2575] { 1.0 } else { 0.0 });s.b[2576] = (s.v[2444] > 1e-100);s.store_scalar(2576, if s.b[2576] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {s.store_mul(2521, 2393, 2460);s.store_div(2462, 2521, 2457);s.store_add_scaled_inputs(2463, 2449, 1.0, 2399, 0.5);s.store_div_scaled_product_by_product_indices(2027, 2399, 2442, 1.0, 2463, 2463, 1.0);}
        s.b[2577] = (s.v[2027] > 0.0001);s.store_scalar(2577, if s.b[2577] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) {s.store_sub_from_scalar(2028, 1.0, 2027);}
        s.b[2578] = (s.v[2028] < 1e-10);s.store_scalar(2578, if s.b[2578] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) && s.b[2578]) {s.store_scalar(2029, 1.0);}
        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2577]) && (!s.b[2578])) {s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));}
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && (!s.b[2577])) {s.store_scale(2029, 2027, 0.5);}
        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {s.store_mul(2464, 2029, 2463);}
        s.b[2579] = ((s.v[706] > 0.0) && (s.v[707] > 0.0));s.store_scalar(2579, if s.b[2579] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) {s.store_scaled_mul(2465, 2413, 2464, 0.475);s.store_add_scaled_product_indices(2027, 2450, 1.0, 2447, 2465, (-1.0));s.store_scaled_add_mixed_ia(2466, 2027, A::sqrt_square_offset(s.ad_value(2027), 1e-12), 0.5);s.store_add_scaled_value_products_mixed_iiiai(2467, 2450, (-1.0), 2413, 2449, 1.0, A::offset(s.ad_value(2447), (-1.0)), 2465, 1.0);s.store_offset_div_scaled_product_indices(2468, 2399, 2413, 0.5, 2467, 1.0, 1.0);s.store_add_scaled_product_indices(2027, 2467, 1.0, 775, 2466, 1.0);s.store_pow_ad(2469, A::mul3(s.ad_value(774), s.ad_value(2027), s.ad_value(704)), s.ad_value(705));s.store_mul_mixed_ai(2028, A::div_scaled_product_offset_rhs(s.ad_value(705), A::mul_sub_from_scalar_rhs(s.ad_value(2468), 1.0, s.ad_value(775)), (-1.0), 1.0, s.ad_value(2027), 1.0), 2469);s.store_div(2027, 2466, 2467);s.store_mul_pow_mixed_iaa(2470, 706, A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707)));s.store_mul_div_scaled_product_mixed_iiai(2029, 2470, 707, A::add(A::offset(s.ad_value(2468), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(2027), 1.0, 1.0)), 1.0, 2467, 1.0);s.store_mul_product3_indices(2471, 2466, 757, 2452, 2453, 1.0);s.store_offset_ad(2027, A::div_scaled_add_product(s.ad_value(2028), 1.0, A::mul3(s.ad_value(757), s.ad_value(2452), s.ad_value(2453)), s.ad_value(2468), (-1.0), s.ad_value(2029), 1.0), 1.0);}
        s.b[2580] = (s.v[2027] < 230.25850929940458);s.store_scalar(2580, if s.b[2580] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) && s.b[2580]) {s.store_scaled_ln_one_plus_exp_scaled_input(2028, 2027, 2.0, 0.5);}
        if ((((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) && (!s.b[2580])) {s.copy_ad(2028, 2027);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_114(
        s: &mut Scratch,
    ) {
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2579]) {s.store_div_scaled_product3_mixed_iiia(2472, 2465, 2029, 2028, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2469), 1.0, s.ad_value(2470), 1.0, s.ad_value(2471), 1.0, 1.0), 1.0);s.store_mul_scale_offset_mixed_ia(2473, 2464, A::div_scaled_value_offset_denominator(s.ad_value(2472), 1.0, A::sqrt_square_offset(s.ad_value(2472), 1.0), 1.0, 1.0), 1.0, 1.0);}
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && (!s.b[2579])) {s.copy_ad(2473, 2464);}
        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {s.store_mul3_affine_lhs(2474, 2413, 2462, 0.7071067811865475, 0.0, 2473);}
        s.b[2581] = (s.v[0] == (-1.0));s.store_scalar(2581, if s.b[2581] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && s.b[2576]) && s.b[2581]) {s.store_div_mixed_ia(2474, 2474, A::sqrt(A::offset(s.ad_value(2474), 1.0)));}
        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {s.store_div_from_scalar_offset_ad(2475, 2.0, A::sqrt(A::scale_offset(s.ad_value(2474), 4.0, 1.0)), 1.0);s.store_mul(2027, 2475, 2474);s.store_mul_ad_product_rhs_mixed_ia(2476, 2473, 2475, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 1.0, A::mul(s.ad_value(2027), s.ad_value(2475)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(2027), s.ad_value(2027), s.ad_value(2475), 4.0), 1.0)), 1.0));s.store_scale(2477, 2476, 0.99);s.store_div_scaled_product3_mixed_iaii(2027, 2477, A::sub_scaled_inputs(s.ad_value(2477), 1.0, s.ad_value(2463), 2.0), 2415, 1.0, 2444, 1.0);}
        if ((s.b[2547] && s.b[2575]) && s.b[2576]) {
            s.store_mul_sub_mixed_iia(2478, 2413, 2477, A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }
        if ((s.b[2547] && s.b[2575]) && (!s.b[2576])) {s.copy_ad(2478, 2461);}
        if (s.b[2547] && s.b[2575]) {s.store_offset(2027, 2394, 1.0);s.store_div_scaled_product_mixed_aii(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 2478, 1.0);s.store_add_mixed_ai(2029, A::square(s.ad_value(2028)), 2027);s.store_scale(2027, 2028, 2.0);s.store_div_scaled_product_add_scaled_denominator(2479, 2478, 2027, 1.0, A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), 1.0, A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027))), 1.0, 1.0);s.store_mul(2480, 2479, 2414);s.store_add(2481, 2423, 2480);}
        s.b[2582] = (s.v[2480] < 460.51701859880916);s.store_scalar(2582, if s.b[2582] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2582]) {s.store_exp_neg_input(2482, 2480);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2582])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2482, 1e-200, 2480, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2547] && s.b[2575]) {s.store_mul(2483, 2438, 2482);}
        s.b[2583] = (((s.v[2417]) as f64).abs() <= s.v[2435]);s.store_scalar(2583, if s.b[2583] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2583]) {s.store_scaled_square(2523, 2436, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2484, 2417, 2436, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2417), 1.0, s.ad_value(2483)), s.ad_value(2398), s.ad_value(2523)), 1.0));}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {s.store_offset(2544, 2481, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_115(
        s: &mut Scratch,
    ) {
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {s.store_sub_ad(2527, A::add_scaled_inputs3(s.ad_value(2543), 0.5, s.ad_value(2544), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2543), s.ad_value(2544)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2544), 0.5, A::sqrt_square_offset(s.ad_value(2544), 5.0), 0.5));s.store_sub(2522, 2417, 2527);s.store_exp_neg_input(2523, 2527);s.store_div_from_scalar_offset_square(2524, 1.0, 2527, 2.0);s.store_mul_square_lhs(2534, 2527, 2524);s.store_mul3_affine_lhs(2535, 2527, 2524, 4.0, 0.0, 2524);s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2524), 8.0, s.ad_value(2534), 12.0), 2524, 2524);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {
            if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2483] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                s.store_scalar(2528, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2528, 2522, 1.0, 2399, A::add_scaled_product(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));
            }
        }
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2545, 1.0, 2399, A::add_scaled_product(s.ad_value(2523), 1.0, s.ad_value(2483), s.ad_value(2536), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2529, 2522, 2.0, 2399, A::add_scaled_sub_value_product(1.0, s.ad_value(2523), 1.0, s.ad_value(2483), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2530, 2481, 1.0, 2527, (-1.0), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))), 1.0);s.store_add(824, 2528, 2529);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2530, A::add_scaled_square_product(s.ad_value(2529), 0.5, s.ad_value(2528), s.ad_value(2545), (-1.0)), 1.0);s.store_add_mixed_ia(2546, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::add_scaled_square_product(s.ad_value(2529), 0.3333333333333333, s.ad_value(2528), s.ad_value(2545), (-1.0)))), 1.0));}
        s.b[2584] = (s.v[2546] < 230.25850929940458);s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && (!s.b[2583])) && s.b[2584]) {s.store_exp(2532, 2546);s.store_div_from_scalar(2533, 1.0, 2532);s.store_mul(2532, 2483, 2532);}
        s.b[2585] = (s.v[2546] > (s.v[2481] - 230.25850929940458));s.store_scalar(2585, if s.b[2585] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2575]) && (!s.b[2583])) && (!s.b[2584])) && s.b[2585]) {s.store_exp_sub(2532, 2546, 2481);s.store_div(2533, 2483, 2532);}
        if ((((s.b[2547] && s.b[2575]) && (!s.b[2583])) && (!s.b[2584])) && (!s.b[2585])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2532, 1e-100, A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2533, 1e-100, 2546, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {s.store_div_from_scalar_offset_square(2522, 1.0, 2546, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_116(
        s: &mut Scratch,
    ) {
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {s.store_mul_square_lhs(2534, 2546, 2522);s.store_mul3_affine_lhs(2535, 2546, 2522, 4.0, 0.0, 2522);s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), 2522, 2522);s.store_sub(2522, 2417, 2546);s.store_add_scaled_product_mixed_iia(2537, 2522, 2.0, 2399, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2533)), 1.0, s.ad_value(2532), 1.0, s.ad_value(2483), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2538, 2522, 1.0, 2399, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2533), 1.0, s.ad_value(2546), 1.0, s.ad_value(2532), 1.0, (-1.0)), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2522, 2.0, 2399, A::add_scaled_inputs_product(s.ad_value(2533), 1.0, s.ad_value(2532), 1.0, s.ad_value(2483), s.ad_value(2536), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2522, 2537, 1.0, 2538, 2522, (-2.0));s.store_add_scaled_inputs_mixed_ia(2484, 2546, 1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0);}
        if (s.b[2547] && s.b[2575]) {s.store_sub(2485, 2484, 2437);}
        s.b[2586] = (s.v[2485] < 1e-10);s.store_scalar(2586, if s.b[2586] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2586]) {s.store_add_scaled_inputs_product_mixed_iiia(2486, 2417, 2.0, 2437, (-2.0), 2399, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2443), 1.0, s.ad_value(2442), s.ad_value(2482), 1.0), 1.0, s.ad_value(2483), s.ad_value(2440), 1.0, (-1.0)), 1.0);s.store_mul_mixed_ai(2487, A::mul_sub_from_scalar_rhs(s.ad_value(2399), 1.0, s.ad_value(2482)), 2444);s.store_sub_from_scalar_scaled_mul_mixed_ia(2027, 2.0, 2399, A::add_scaled_value_products(s.ad_value(2443), 1.0, s.ad_value(2442), s.ad_value(2482), 1.0, s.ad_value(2483), s.ad_value(2441), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2027, 2486, 1.0, 2027, 2487, (-2.0));s.store_scaled_div_mixed_ia(2485, 2487, A::add(s.ad_value(2486), A::sqrt(s.ad_value(2027))), 2.0);s.store_add(2484, 2437, 2485);}
        if (s.b[2547] && s.b[2575]) {s.store_mul(2488, 2485, 2413);s.store_div_scaled_product_offset_denominator_mixed_iia(2489, 2484, 2484, 1.0, A::square(s.ad_value(2484)), 2.0, 1.0);}
        s.b[2587] = (s.v[2484] < 230.25850929940458);s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2587]) {s.store_exp_neg_input(2490, 2484);}
        s.b[2588] = (s.v[2484] < 1e-5);s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && s.b[2588]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2491, 2484, 1.0, 2484, 1.0, 2484, 0.25, 0.3333333333333333, 0.5);s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2484), 1.0, A::scale(s.ad_value(2484), 0.25), 0.3333333333333333));s.store_scaled_mul(2492, 2484, 2027, 0.7071067811865475);s.store_mul3_ad_middle(2493, A::mul3_scaled_output(s.ad_value(2483), s.ad_value(2484), s.ad_value(2484), 0.16666666666666666), 2484, A::scale_offset(s.ad_value(2484), 1.75, 1.0));}
        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && (!s.b[2588])) {s.store_add_offset_lhs(2491, 2484, (-1.0), 2490);s.store_sqrt(2492, 2491);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_117(
        s: &mut Scratch,
    ) {
        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && (!s.b[2588])) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(2493, 2483, A::div_from_scalar(1.0, s.ad_value(2490)), 1.0, 2484, (-1.0), 2489, -1.0, (-1.0));}
        s.b[2589] = (s.v[2484] > (s.v[2481] - 230.25850929940458));s.store_scalar(2589, if s.b[2589] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && (!s.b[2587])) && s.b[2589]) {s.store_exp_sub(2027, 2484, 2481);s.store_div(2490, 2483, 2027);s.store_add_scaled_product_mixed_iia(2493, 2027, 1.0, 2483, A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489)), (-1.0));}
        if (((s.b[2547] && s.b[2575]) && (!s.b[2587])) && (!s.b[2589])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2490, 1e-100, 2484, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2027, 1e-100, A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(2493, 2027, 1.0, 2483, A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489)), (-1.0));}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2587])) {s.store_add_offset_lhs(2491, 2484, (-1.0), 2490);s.store_sqrt(2492, 2491);}
        if (s.b[2547] && s.b[2575]) {s.store_mul3_lhs(2494, 2492, 2398, 2413);s.store_scaled_add(2495, 2437, 2484, 0.5);s.store_scalar(2496, 0.0);s.store_mul(2027, 2490, 2443);}
        s.b[2590] = (s.v[2027] > 0.0);s.store_scalar(2590, if s.b[2590] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2590]) {s.store_sqrt(2496, 2027);}
        if (s.b[2547] && s.b[2575]) {s.store_scaled_add(2497, 2444, 2493, 0.5);s.store_add_scaled_product_mixed_iaa(2498, 2497, 1.0, A::square(s.ad_value(2485)), A::sub_scaled_inputs(s.ad_value(2496), 1.0, s.ad_value(2415), 2.0), 0.125);}
        s.b[2591] = (s.v[2495] < 1e-5);s.store_scalar(2591, if s.b[2591] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2591]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2499, 2495, 1.0, 2495, 1.0, 2495, 0.25, 0.3333333333333333, 0.5);s.store_mul_sqrt_mixed_ia(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));}
        s.b[2592] = (s.v[730] > 0.0);s.store_scalar(2592, if s.b[2592] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && s.b[2591]) && s.b[2592]) {s.store_div_from_scalar_sqrt_ad(2501, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0));}
        if ((s.b[2547] && s.b[2575]) && s.b[2591]) {s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2495), 1.0, A::scale(s.ad_value(2495), 0.25), 0.3333333333333333));s.store_scaled_mul(2502, 2495, 2027, 0.7071067811865475);s.store_add_mixed_ia(2503, 2501, A::div_scaled_product(s.ad_value(2398), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2495), 0.5)), 1.0, A::square(s.ad_value(2495)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0));}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2591])) {s.store_add_offset_lhs(2499, 2495, (-1.0), 2496);s.store_mul_sqrt_mixed_ia(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));}
        s.b[2593] = (s.v[730] > 0.0);s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && (!s.b[2591])) && s.b[2593]) {s.store_add_scaled_sub_value_product_indices(2504, 1.0, 2496, 1.0, 2500, 2415, 2.0);s.store_div_from_scalar_sqrt_ad(2501, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0));s.store_div_scaled_value_offset_denominator(2027, s.ad_value(2501), 1.0, s.ad_value(2501), 1.0, 1.0);s.store_mul_product3_mixed_iaii(2505, 730, A::square(s.ad_value(2027)), 2399, 2498, 1.0);s.store_add_scaled_inputs_product_mixed_iiia(2506, 2500, 2.0, 2505, (-2.0), 2399, A::add(A::sub_from_scalar(1.0, s.ad_value(2496)), s.ad_value(2498)), 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(2507, 2505, 2505, 1.0, 2500, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_118(
        s: &mut Scratch,
    ) {
        if (((s.b[2547] && s.b[2575]) && (!s.b[2591])) && s.b[2593]) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2508, 1.0, 2399, A::add(s.ad_value(2496), s.ad_value(2498)), 0.5);s.store_div_scaled_product_mixed_iia(2509, 2507, 2506, 1.0, A::add_scaled_square_product(s.ad_value(2506), 1.0, s.ad_value(2508), s.ad_value(2507), (-1.0)), 1.0);s.store_add(2495, 2495, 2509);s.store_exp(2510, 2509);s.store_div(2496, 2496, 2510);s.store_mul(2498, 2498, 2510);s.store_add_offset_lhs(2499, 2495, (-1.0), 2496);s.store_mul_sqrt_mixed_ia(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));s.store_add_ad(2511, A::sub_from_scalar(1.0, s.ad_value(2496)), A::mul3_scaled_output(s.ad_value(2500), s.ad_value(2501), s.ad_value(2415), 2.0));s.store_div_scaled_product3_mixed_iiaa(2485, 2485, 2510, A::add(s.ad_value(2504), s.ad_value(2497)), 1.0, A::add_scaled_product(s.ad_value(2511), 1.0, s.ad_value(2510), s.ad_value(2497), 1.0), 1.0);s.store_mul(2488, 2485, 2413);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2591])) {s.store_sqrt(2502, 2499);s.store_add_scaled_inputs_mixed_ia(2503, 2501, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, s.ad_value(2496)), s.ad_value(2502)), 0.5);}
        if (s.b[2547] && s.b[2575]) {s.store_mul_div_scaled_product_mixed_iiia(2512, 2413, 2399, 2498, 1.0, A::add_scaled_product(s.ad_value(2500), 1.0, s.ad_value(2398), s.ad_value(2502), 1.0), 1.0);s.store_add_scaled_product_indices(2513, 2512, 1.0, 2413, 2503, 1.0);s.store_mul3_lhs(2514, 2502, 2398, 2413);}
        s.b[2594] = (s.v[218] < 0.0);s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2594]) {s.store_sub_from_scalar_scaled_mul(2453, 1.0, 218, 2512, 1.0);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2594])) {s.store_div_from_scalar_offset_product(2453, 1.0, 218, 2512, 1.0);}
        if (s.b[2547] && s.b[2575]) {s.store_mul_product3_indices(2454, 2512, 757, 2452, 2453, 1.0);s.store_add_scaled_product_indices(2515, 2514, 1.0, 775, 2512, 1.0);s.store_add_scaled_product_indices(2516, 2514, 1.0, 776, 2512, 1.0);s.store_mul(2517, 774, 2515);s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2499), 1.0, A::add(s.ad_value(2499), s.ad_value(2498)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2456, A::pow(A::mul(s.ad_value(2517), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);s.store_mul_add_mixed_iai(2518, 2448, A::offset(s.ad_value(2456), 1.0), 2454);s.store_ln_ad(2519, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(826), s.ad_value(2488)), s.ad_value(779)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2479), s.ad_value(2488)), s.ad_value(779)), 1.0), 1.0));s.store_mul(2029, 2512, 2458);s.store_div_add_scaled_inputs_rhs_indices(2459, 2029, 223, 1.0, 2029, 1.0);}
        s.b[2595] = (s.v[222] < 0.0);s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2595]) {s.store_div_from_scalar_sub_from_scalar_ad(2460, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2459)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_119(
        s: &mut Scratch,
    ) {
        if ((s.b[2547] && s.b[2575]) && (!s.b[2595])) {s.store_offset_mul(2460, 222, 2459, 1.0);}
        if (s.b[2547] && s.b[2575]) {s.store_mul(2521, 2393, 2460);s.store_mul(2520, 2500, 2413);}
        if s.b[2547] {s.copy_ad(1887, 2395);s.copy_ad(1888, 2413);s.copy_ad(1889, 2398);s.copy_ad(1890, 2417);s.copy_ad(1891, 2422);s.copy_ad(1892, 2451);s.copy_ad(1893, 2488);s.copy_ad(1894, 2494);s.copy_ad(1895, 2501);s.copy_ad(1896, 2503);s.copy_ad(1897, 2512);s.copy_ad(1898, 2513);s.copy_ad(1899, 2516);s.copy_ad(1900, 2518);s.copy_ad(1901, 2519);s.copy_ad(1902, 2521);s.copy_ad(1903, 2520);s.copy_ad(1932, 2414);}
        let (t1,) = {
    if s.b[2547] {
        (s.v[2435],)
    } else {
        (s.v[1933],)
    }
};
        s.store_scalar(1933, t1);
        if s.b[2547] {s.copy_ad(1934, 2495);s.copy_ad(1935, 2500);}
        if (!s.b[2547]) {s.copy_ad(745, 728);s.copy_ad(1887, 1822);s.copy_ad(1888, 1824);s.copy_ad(1889, 1826);s.copy_ad(1890, 1829);s.copy_ad(1891, 1830);s.copy_ad(1892, 1849);s.copy_ad(1893, 1860);s.copy_ad(1894, 1861);s.copy_ad(1895, 1863);s.copy_ad(1896, 1864);s.copy_ad(1897, 1865);s.copy_ad(1898, 1866);s.copy_ad(1899, 1868);s.copy_ad(1900, 1869);s.copy_ad(1901, 1871);s.copy_ad(1902, 1870);s.copy_ad(1903, 1872);s.copy_ad(1932, 1825);}
        let (t2,) = {
    if (!s.b[2547]) {
        (s.v[1833],)
    } else {
        (s.v[1933],)
    }
};
        s.store_scalar(1933, t2);
        if (!s.b[2547]) {s.copy_ad(1934, 1862);s.copy_ad(1935, 1931);}
        s.copy_ad(1904, 255);s.b[2596] = (s.v[773] > 0.0);s.store_scalar(2596, if s.b[2596] { 1.0 } else { 0.0 });
        if s.b[2596] {s.store_div_scaled_value_offset_denominator(1904, s.ad_value(255), 1.0, A::mul(s.ad_value(773), A::powf(A::offset(A::square(s.ad_value(1899)), s.v[733]), ((-1.0) * 0.16666666666666666))), 1.0, 1.0);}
        s.store_scalar(1905, 1.0);s.store_scalar(1906, 1.0);s.store_scalar(1907, 0.0);s.store_scalar(1908, 1.0);s.store_scalar(1909, 1.0);s.copy_ad(2359, 1903);s.store_scalar(2362, 0.0);s.store_scalar(2361, 0.0);s.copy_ad(2363, 2359);s.b[2597] = (s.v[1890] > 0.0);s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });
        if s.b[2597] {s.store_mul_div_scaled_product_mixed_iaii(2354, 1901, A::add(s.ad_value(260), A::div(s.ad_value(261), s.ad_value(1898))), 1897, 1.0, 1898, 1.0);}
        s.b[2598] = (s.v[2354] > 0.0);s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });
        if (s.b[2597] && s.b[2598]) {s.store_div_from_scalar_add_ad(1905, 1.0, A::offset(s.ad_value(2354), 1.0), A::square(s.ad_value(2354)));}
        if (s.b[2597] && (!s.b[2598])) {s.store_sub_from_scalar(1905, 1.0, 2354);}
        if s.b[2597] {s.store_mul(1906, 1900, 1905);s.store_div(1907, 1902, 1906);s.store_mul_ad_product_lhs_mixed_ai(2355, A::square(s.ad_value(1907)), 1893, 1893);}
        s.b[2599] = (s.v[0] == (-1.0));s.store_scalar(2599, if s.b[2599] { 1.0 } else { 0.0 });
        if (s.b[2597] && s.b[2599]) {s.store_div_scaled_value_offset_denominator(2355, s.ad_value(2355), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_120(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[2597] {s.store_mul_scale_offset_mixed_ia(1908, 1906, A::sqrt(A::scale_offset(s.ad_value(2355), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div(2027, 1906, 1908);s.store_mul_scale_offset_mixed_ia(2356, 1896, A::mul3_scaled_output(s.ad_value(2355), s.ad_value(2027), s.ad_value(2027), 0.5), 1.0, 1.0);s.store_div_scaled_product_indices(1909, 2027, 1898, 1.0, 2356, 1.0);s.store_scaled_div(2357, 1893, 1909, 0.5);s.store_square(2358, 2357);s.store_add_product3_rhs_mixed_iia(2359, 1903, 1895, 1893, A::add(A::offset(A::mul_scaled_output(s.ad_value(2357), s.ad_value(1905), 0.3333333333333333), (-1.0)), s.ad_value(1905)), 0.5);s.store_scaled_mul(2027, 1896, 1893, 0.16666666666666666);}
        s.b[2600] = (p.p49 == 1.0);s.store_scalar(2600, if s.b[2600] { 1.0 } else { 0.0 });
        if (s.b[2597] && s.b[2600]) {s.store_scalar(2360, 0.0);s.store_mul_ad_affine_product_rhs(2361, 1905, s.ad_value(1905), A::sub(s.ad_value(1897), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 2.0, s.ad_value(2357), 3.0)), 0.5, 0.0);}
        if (s.b[2597] && (!s.b[2600])) {s.store_mul_scale_offset_mixed_ai(2360, A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(1896), s.ad_value(1893), (-0.5)), 1905, -1.0, 1.0);s.store_add_scaled_products_mixed_aaia(2361, A::square(s.ad_value(1905)), A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(2027), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2357)), 1.0, s.ad_value(2358), 0.2), (-1.0)), 0.5, 2360, A::offset(s.ad_value(1905), 1.0), 0.5);}
        if s.b[2597] {s.store_add_scaled_product_mixed_iia(2362, 2360, 1.0, 1905, A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(2027), s.ad_value(2357), 1.0), 1.0);s.store_sub(2363, 2359, 2362);}
        s.store_mul(851, 2359, 1904);s.store_mul_scale_offset_indices(853, 1904, 2361, -1.0, 0.0);s.store_mul_scale_offset_indices(852, 1904, 2363, -1.0, 0.0);s.store_scalar(2379, 0.0);s.store_scalar(2380, 0.0);s.store_scalar(2378, 0.0);s.b[2601] = ((s.v[268] > 0.0) || (s.v[269] > 0.0));s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });
        if s.b[2601] {s.store_scalar(2368, 1.0);s.copy_ad(2367, 1887);}
        s.b[2602] = (s.v[272] > 1e-10);s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });
        if (s.b[2601] && s.b[2602]) {s.store_add_scaled_inputs3_indices(2364, 1887, 1.0, 270, (-1.0), 808, 1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2027, 2364, 0.5, 808, 0.5, A::add(A::square(A::sub(s.ad_value(2364), s.ad_value(808))), s.ad_value(809)), 0.5);s.store_mul_add_scaled_inputs3_offset_rhs_indices(2028, 2027, 2027, 2.0, 808, (-1.0), 2364, -1.0, 0.0);s.store_div(2029, 808, 2027);s.store_mul(2365, 2364, 2029);s.store_sqrt_sub_from_scalar_ad(2366, 1.0, A::mul(s.ad_value(2365), s.ad_value(272)));s.store_add_scaled_inputs3_mixed_aii(2367, A::div(A::sub_from_scalar(1.0, s.ad_value(2366)), s.ad_value(272)), 1.0, 2364, 1.0, 2365, -1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_121(
        s: &mut Scratch,
    ) {
        if (s.b[2601] && s.b[2602]) {s.store_offset_ad(2368, A::div_scaled_product3(A::offset(A::div_from_scalar(0.5, s.ad_value(2366)), (-1.0)), A::add_scaled_product(s.ad_value(2028), 1.0, s.ad_value(2364), A::sub(s.ad_value(808), s.ad_value(2027)), 1.0), s.ad_value(2029), 1.0, s.ad_value(2028), 1.0), 1.0);}
        if s.b[2601] {s.store_scalar(2370, 1.0);s.store_scalar(2371, 0.0);}
        s.b[2603] = (s.v[271] > 0.0);s.store_scalar(2603, if s.b[2603] { 1.0 } else { 0.0 });
        if (s.b[2601] && s.b[2603]) {s.store_add_scaled_product_mixed_iia(2027, 745, 0.5, 1888, A::scale_offset(s.ad_value(1889), 0.7071067811865475, 1.0), 1.0);s.store_div(2369, 1887, 2027);}
        s.b[2604] = (((s.v[2369]) as f64).abs() < 230.25850929940458);s.store_scalar(2604, if s.b[2604] { 1.0 } else { 0.0 });
        if ((s.b[2601] && s.b[2603]) && s.b[2604]) {s.store_div_from_scalar_offset_ad(2370, 1.0, A::exp_scaled_input(s.ad_value(2369), -1.0), 1.0);}
        s.b[2605] = (s.v[2369] < 0.0);s.store_scalar(2605, if s.b[2605] { 1.0 } else { 0.0 });
        if (((s.b[2601] && s.b[2603]) && (!s.b[2604])) && s.b[2605]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2370, 1e-100, 2369, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[2606] = (s.v[2369] < 230.25850929940458);s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });
        if ((s.b[2601] && s.b[2603]) && s.b[2606]) {s.store_ln_one_plus_exp(2028, 2369);}
        if ((s.b[2601] && s.b[2603]) && (!s.b[2606])) {s.copy_ad(2028, 2369);}
        if (s.b[2601] && s.b[2603]) {s.store_mul(2371, 2027, 2028);}
        if s.b[2601] {s.store_add_scaled_product_right_sub(2372, 2368, 1.0, 271, 2370, 2368, 1.0);s.store_add_scaled_product_right_sub(2373, 2367, 1.0, 271, 2371, 2367, 1.0);s.store_add_scaled_inputs3_mixed_aii(2374, A::add_scaled_product(s.ad_value(1887), 1.0, s.ad_value(1888), s.ad_value(1891), (-1.0)), 1.0, 1903, (-1.0), 1893, (-0.5));s.store_add_scaled_inputs3_indices(2375, 1887, 1.0, 2374, (-1.0), 1892, -1.0);s.store_add_scaled_inputs3_indices(2376, 1893, 1.0, 2374, 1.0, 826, -1.0);s.store_add_scaled_inputs3_indices(2377, 1887, 1.0, 2376, (-1.0), 1894, -1.0);}
        s.b[2607] = (s.v[831] > 0.0);s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });
        if (s.b[2601] && s.b[2607]) {s.store_mul_mixed_ia(2378, 2372, A::add_scaled_products(s.ad_value(269), s.ad_value(2376), 1.0, s.ad_value(268), s.ad_value(2374), 1.0));s.store_mul_sub_rhs(2379, 268, 2375, 2373);s.store_mul_sub_rhs(2380, 269, 2377, 2373);}
        if (s.b[2601] && (!s.b[2607])) {s.store_mul_mixed_ia(2378, 2372, A::add_scaled_products(s.ad_value(268), s.ad_value(2376), 1.0, s.ad_value(269), s.ad_value(2374), 1.0));s.store_mul_sub_rhs(2379, 269, 2375, 2373);s.store_mul_sub_rhs(2380, 268, 2377, 2373);}
        if s.b[2601] {s.store_add(851, 851, 2378);s.store_add(853, 853, 2380);s.store_add_scaled_inputs4_indices(852, 852, 1.0, 2378, (-1.0), 2380, -1.0, 2379, -1.0);}
        s.store_mul(1910, 262, 1878);s.store_mul(1911, 263, 1879);s.store_scalar(2383, 0.0);s.store_scalar(2381, 0.0);s.b[2608] = ((s.v[262] > 0.0) && (s.v[264] > 0.0));s.store_scalar(2608, if s.b[2608] { 1.0 } else { 0.0 });
        if s.b[2608] {s.store_mul_add_scaled_inputs_rhs_indices(2027, 266, 1819, 0.5, 787, 1.0);}
        s.b[2609] = (s.v[2027] < 230.25850929940458);s.store_scalar(2609, if s.b[2609] { 1.0 } else { 0.0 });s.b[2610] = (s.v[2027] > (-230.25850929940458));s.store_scalar(2610, if s.b[2610] { 1.0 } else { 0.0 });
        if ((s.b[2608] && s.b[2609]) && s.b[2610]) {s.store_exp(2381, 2027);}
        if ((s.b[2608] && s.b[2609]) && (!s.b[2610])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2381, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2611] = (s.v[2381] > 1e-10);s.store_scalar(2611, if s.b[2611] { 1.0 } else { 0.0 });
        if ((s.b[2608] && s.b[2609]) && s.b[2611]) {s.store_ln_offset_input(2382, 2381, 1.0);s.store_mul_scale_offset_mixed_ia(2028, 2382, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0)), -1.0, 1.0);}
        if ((s.b[2608] && s.b[2609]) && (!s.b[2611])) {s.copy_ad(2382, 2381);s.store_div_scaled_value_offset_denominator(2028, s.ad_value(2382), 2.0, s.ad_value(2382), 2.0, 1.0);}
        if (s.b[2608] && (!s.b[2609])) {s.copy_ad(2382, 2027);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_122(
        s: &mut Scratch,
    ) {
        if (s.b[2608] && (!s.b[2609])) {s.store_mul_scale_offset_mixed_ia(2028, 2382, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0)), -1.0, 1.0);}
        if s.b[2608] {s.store_mul_ad_affine_product_lhs(2383, A::div_scaled_inputs(s.ad_value(264), (-2.0), s.ad_value(266), 1.0), s.ad_value(262), s.v[354], 0.0, 2028);}
        s.store_scalar(2386, 0.0);s.store_scalar(2384, 0.0);s.b[2612] = ((s.v[263] > 0.0) && (s.v[265] > 0.0));s.store_scalar(2612, if s.b[2612] { 1.0 } else { 0.0 });
        if s.b[2612] {s.store_mul_add_scaled_inputs_rhs_indices(2027, 266, 1819, 0.5, 788, 1.0);}
        s.b[2613] = (s.v[2027] < 230.25850929940458);s.store_scalar(2613, if s.b[2613] { 1.0 } else { 0.0 });s.b[2614] = (s.v[2027] > (-230.25850929940458));s.store_scalar(2614, if s.b[2614] { 1.0 } else { 0.0 });
        if ((s.b[2612] && s.b[2613]) && s.b[2614]) {s.store_exp(2384, 2027);}
        if ((s.b[2612] && s.b[2613]) && (!s.b[2614])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2384, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2615] = (s.v[2384] > 1e-10);s.store_scalar(2615, if s.b[2615] { 1.0 } else { 0.0 });
        if ((s.b[2612] && s.b[2613]) && s.b[2615]) {s.store_ln_offset_input(2385, 2384, 1.0);s.store_mul_scale_offset_mixed_ia(2028, 2385, A::div(A::ln(A::offset(s.ad_value(2385), 1.0)), A::offset(s.ad_value(2385), 2.0)), -1.0, 1.0);}
        if ((s.b[2612] && s.b[2613]) && (!s.b[2615])) {s.copy_ad(2385, 2384);s.store_div_scaled_value_offset_denominator(2028, s.ad_value(2385), 2.0, s.ad_value(2385), 2.0, 1.0);}
        if (s.b[2612] && (!s.b[2613])) {s.copy_ad(2385, 2027);s.store_mul_scale_offset_mixed_ia(2028, 2385, A::div(A::ln(A::offset(s.ad_value(2385), 1.0)), A::offset(s.ad_value(2385), 2.0)), -1.0, 1.0);}
        if s.b[2612] {s.store_mul_ad_affine_product_lhs(2386, A::div_scaled_inputs(s.ad_value(265), (-2.0), s.ad_value(266), 1.0), s.ad_value(263), s.v[354], 0.0, 2028);}
        s.store_add(2387, 2383, 2386);s.store_add_scaled_product_indices(856, 2387, 1.0, 267, 829, 1.0);s.store_mul(854, 274, 834);s.store_mul(855, 275, 837);s.store_scalar(1938, 0.0);s.store_scalar(1939, 0.0);s.store_scalar(1940, 0.0);s.store_scalar(1941, 0.0);s.b[2616] = (s.v[1] != 0.0);s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });s.b[2617] = (s.v[1890] <= 0.0);s.store_scalar(2617, if s.b[2617] { 1.0 } else { 0.0 });
        if (s.b[2616] && s.b[2617]) {s.store_scalar(1936, 0.5);s.store_scalar(1937, 1.0);s.copy_ad(1938, 1889);}
        if (s.b[2616] && (!s.b[2617])) {s.store_offset_scaled_div(1936, 1893, 1909, ((0.25) * (0.5)), 0.5);s.store_div_add_scaled_inputs_rhs_indices(1937, 1935, 1890, 1.0, 1934, -1.0);s.store_div(1938, 1889, 1937);}
        if s.b[2616] {s.store_square(1939, 1938);s.store_offset_scaled(1940, 1938, 0.7071067811865475, 1.0);}
        let (t4,) = {
    if s.b[2616] {
        let t3: f64 = (1e-5 * s.v[1940]);
        (t3,)
    } else {
        (s.v[1941],)
    }
};
        s.store_scalar(1941, t4);s.store_scalar(2618, 0.0);s.store_scalar(2619, 0.0);s.store_scalar(2620, 0.0);s.store_scalar(2621, 0.0);s.store_scalar(2622, 0.0);s.store_scalar(2623, 0.0);s.store_scalar(2624, 0.0);s.store_scalar(2625, 0.0);s.store_scalar(2626, 0.0);s.store_scalar(2627, 0.0);s.store_scalar(2628, 0.0);s.store_scalar(2629, 0.0);s.store_scalar(2630, 0.0);s.store_scalar(2631, 0.0);s.store_scalar(2632, 0.0);s.store_scalar(2633, 0.0);s.store_scalar(2634, 0.0);s.store_scalar(2635, 0.0);s.store_scalar(2636, 0.0);s.store_scalar(2637, 0.0);s.store_scalar(2638, 0.0);s.store_scalar(2639, 0.0);s.store_scalar(2640, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_123(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(2641, 0.0);s.store_scalar(2642, 0.0);s.store_scalar(2643, 0.0);s.store_scalar(2644, 0.0);s.store_scalar(2645, 0.0);s.store_scalar(2646, 0.0);s.store_scalar(2647, 0.0);s.store_scalar(2648, 0.0);s.store_scalar(2649, 0.0);s.store_scalar(2650, 0.0);s.store_scalar(2651, 0.0);s.store_scalar(2652, 0.0);s.store_scalar(2653, 0.0);s.store_scalar(2654, 0.0);s.store_scalar(2655, 0.0);s.store_scalar(2656, 0.0);s.store_scalar(2657, 0.0);s.store_scalar(2658, 0.0);s.store_scalar(2659, 0.0);s.store_scalar(2660, 0.0);s.store_scalar(2661, 0.0);s.store_scalar(2662, 0.0);s.store_scalar(2663, 0.0);s.store_scalar(2664, 0.0);s.store_scalar(848, 0.0);s.store_scalar(1912, 0.0);s.store_scalar(1913, 0.0);s.store_scalar(1914, 0.0);s.store_scalar(849, 0.0);s.store_scalar(1915, 0.0);s.store_scalar(1916, 0.0);s.store_scalar(1917, 0.0);s.store_scalar(857, 0.0);s.store_scalar(1918, 0.0);s.store_scalar(1919, 0.0);s.store_scalar(1920, 0.0);s.store_scalar(858, 0.0);s.store_scalar(1921, 0.0);s.store_scalar(1922, 0.0);s.store_scalar(1923, 0.0);s.b[2665] = (p.p43 > 0.0);s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });s.b[2666] = (s.v[474] == 1.0);s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });
        if (s.b[2665] && s.b[2666]) {s.store_scale(496, 832, (s.v[371] * s.v[668]));}
        if (s.b[2665] && s.b[2666]) {
            if (s.v[496] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(497, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0);
            } else {
                if (s.v[496] > s.v[660]) {
                    s.store_mul_scale_offset_mixed_ia(497, 661, A::sub(s.ad_value(496), s.ad_value(660)), 1.0, 1.0);
                } else {
                    s.store_exp(497, 496);
                }
            }
        }
        if (s.b[2665] && s.b[2666]) {s.store_mul_scale_offset_indices(502, 667, 497, 1.0, (-1.0));s.store_scaled_mul(496, 832, 670, s.v[371]);}
        if (s.b[2665] && s.b[2666]) {
            if (s.v[496] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(497, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0);
            } else {
                if (s.v[496] > s.v[662]) {
                    s.store_mul_scale_offset_mixed_ia(497, 663, A::sub(s.ad_value(496), s.ad_value(662)), 1.0, 1.0);
                } else {
                    s.store_exp(497, 496);
                }
            }
        }
        if (s.b[2665] && s.b[2666]) {s.store_mul_scale_offset_indices(503, 669, 497, 1.0, (-1.0));s.store_scalar(504, 0.0);}
        s.b[2667] = (s.v[666] > 0.0);s.store_scalar(2667, if s.b[2667] { 1.0 } else { 0.0 });
        if ((s.b[2665] && s.b[2666]) && s.b[2667]) {s.store_mul_add_scaled_product_rhs_indices(504, 832, 671, 1.0, 832, 672, 1.0);}
        if ((s.b[2665] && s.b[2666]) && (!s.b[2667])) {s.store_scaled_mul(496, 832, 672, (-s.v[371]));}
        if ((s.b[2665] && s.b[2666]) && (!s.b[2667])) {
            if (s.v[496] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(497, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0);
            } else {
                if (s.v[496] > s.v[664]) {
                    s.store_mul_scale_offset_mixed_ia(497, 665, A::sub(s.ad_value(496), s.ad_value(664)), 1.0, 1.0);
                } else {
                    s.store_exp(497, 496);
                }
            }
        }
        if ((s.b[2665] && s.b[2666]) && (!s.b[2667])) {s.store_mul_scaled_offset_rhs(504, 671, -1.0, 497, (-1.0));}
        if (s.b[2665] && s.b[2666]) {s.store_add_scaled_inputs3_indices(848, 502, 1.0, 503, 1.0, 504, 1.0);s.store_scale(496, 833, (s.v[371] * s.v[695]));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_124(
        s: &mut Scratch,
    ) {
        if (s.b[2665] && s.b[2666]) {
            if (s.v[496] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(497, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0);
            } else {
                if (s.v[496] > s.v[687]) {
                    s.store_mul_scale_offset_mixed_ia(497, 688, A::sub(s.ad_value(496), s.ad_value(687)), 1.0, 1.0);
                } else {
                    s.store_exp(497, 496);
                }
            }
        }
        if (s.b[2665] && s.b[2666]) {s.store_mul_scale_offset_indices(502, 694, 497, 1.0, (-1.0));s.store_scaled_mul(496, 833, 697, s.v[371]);}
        if (s.b[2665] && s.b[2666]) {
            if (s.v[496] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(497, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0);
            } else {
                if (s.v[496] > s.v[689]) {
                    s.store_mul_scale_offset_mixed_ia(497, 690, A::sub(s.ad_value(496), s.ad_value(689)), 1.0, 1.0);
                } else {
                    s.store_exp(497, 496);
                }
            }
        }
        if (s.b[2665] && s.b[2666]) {s.store_mul_scale_offset_indices(503, 696, 497, 1.0, (-1.0));s.store_scalar(504, 0.0);}
        s.b[2668] = (s.v[693] > 0.0);s.store_scalar(2668, if s.b[2668] { 1.0 } else { 0.0 });
        if ((s.b[2665] && s.b[2666]) && s.b[2668]) {s.store_mul_add_scaled_product_rhs_indices(504, 833, 698, 1.0, 833, 699, 1.0);}
        if ((s.b[2665] && s.b[2666]) && (!s.b[2668])) {s.store_scaled_mul(496, 833, 699, (-s.v[371]));}
        if ((s.b[2665] && s.b[2666]) && (!s.b[2668])) {
            if (s.v[496] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(497, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0);
            } else {
                if (s.v[496] > s.v[691]) {
                    s.store_mul_scale_offset_mixed_ia(497, 692, A::sub(s.ad_value(496), s.ad_value(691)), 1.0, 1.0);
                } else {
                    s.store_exp(497, 496);
                }
            }
        }
        if ((s.b[2665] && s.b[2666]) && (!s.b[2668])) {s.store_mul_scaled_offset_rhs(504, 698, -1.0, 497, (-1.0));}
        if (s.b[2665] && s.b[2666]) {s.store_add_scaled_inputs3_indices(849, 502, 1.0, 503, 1.0, 504, 1.0);s.store_scalar(2669, 0.0);s.store_scalar(2670, 0.0);s.store_primal_scaled_mul(2621, 657, 657, 4.0);s.store_primal_div(2622, 657, 658);s.store_add_scaled_product_indices(2623, 832, 1.0, 657, 2622, 1.0);s.store_add(2624, 658, 2623);s.store_sub(2625, 658, 2623);s.store_sqrt_square_add(2626, 2625, 2621);s.store_div_scaled_product_add_scaled_denominator_indices(2670, 832, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
        s.b[2671] = (s.v[651] > 0.5);s.store_scalar(2671, if s.b[2671] { 1.0 } else { 0.0 });s.b[2672] = (s.v[408] == 0.5);s.store_scalar(2672, if s.b[2672] { 1.0 } else { 0.0 });
        if (((s.b[2665] && s.b[2666]) && s.b[2671]) && s.b[2672]) {s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[405]));}
        if (((s.b[2665] && s.b[2666]) && s.b[2671]) && (!s.b[2672])) {s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[405])), s.v[408]);}
        if ((s.b[2665] && s.b[2666]) && s.b[2671]) {s.store_add_scaled_inputs3_offset_indices(1918, 2669, (-s.v[417]), 832, s.v[420], 2670, (-s.v[420]), s.v[417]);}
        s.b[2673] = (s.v[652] > 0.5);s.store_scalar(2673, if s.b[2673] { 1.0 } else { 0.0 });s.b[2674] = (s.v[409] == 0.5);s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });
        if (((s.b[2665] && s.b[2666]) && s.b[2673]) && s.b[2674]) {s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[406]));}
        if (((s.b[2665] && s.b[2666]) && s.b[2673]) && (!s.b[2674])) {s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[406])), s.v[409]);}
        if ((s.b[2665] && s.b[2666]) && s.b[2673]) {s.store_add_scaled_inputs3_offset_indices(1919, 2669, (-s.v[418]), 832, s.v[421], 2670, (-s.v[421]), s.v[418]);}
        s.b[2675] = (s.v[653] > 0.5);s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });s.b[2676] = (s.v[410] == 0.5);s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });
        if (((s.b[2665] && s.b[2666]) && s.b[2675]) && s.b[2676]) {s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[407]));}
        if (((s.b[2665] && s.b[2666]) && s.b[2675]) && (!s.b[2676])) {s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[407])), s.v[410]);}
        if ((s.b[2665] && s.b[2666]) && s.b[2675]) {s.store_add_scaled_inputs3_offset_indices(1920, 2669, (-s.v[419]), 832, s.v[422], 2670, (-s.v[422]), s.v[419]);}
        if (s.b[2665] && s.b[2666]) {s.store_scalar(2669, 0.0);s.store_scalar(2670, 0.0);s.store_primal_scaled_mul(2621, 684, 684, 4.0);s.store_primal_div(2622, 684, 685);s.store_add_scaled_product_indices(2623, 833, 1.0, 684, 2622, 1.0);s.store_add(2624, 685, 2623);s.store_sub(2625, 685, 2623);s.store_sqrt_square_add(2626, 2625, 2621);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_125(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2665] && s.b[2666]) {s.store_div_scaled_product_add_scaled_denominator_indices(2670, 833, 685, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
        s.b[2677] = (s.v[678] > 0.5);s.store_scalar(2677, if s.b[2677] { 1.0 } else { 0.0 });s.b[2678] = (s.v[575] == 0.5);s.store_scalar(2678, if s.b[2678] { 1.0 } else { 0.0 });
        if (((s.b[2665] && s.b[2666]) && s.b[2677]) && s.b[2678]) {s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::mul(s.ad_value(2670), s.ad_value(572)));}
        if (((s.b[2665] && s.b[2666]) && s.b[2677]) && (!s.b[2678])) {s.store_pow_sub_from_scalar_mul_base_indices(2669, 1.0, 2670, 572, 575);}
        if ((s.b[2665] && s.b[2666]) && s.b[2677]) {s.store_add_scaled_product_mixed_aia(1921, A::mul_sub_from_scalar_rhs(s.ad_value(584), 1.0, s.ad_value(2669)), 1.0, 587, A::sub(s.ad_value(833), s.ad_value(2670)), 1.0);}
        s.b[2679] = (s.v[679] > 0.5);s.store_scalar(2679, if s.b[2679] { 1.0 } else { 0.0 });s.b[2680] = (s.v[576] == 0.5);s.store_scalar(2680, if s.b[2680] { 1.0 } else { 0.0 });
        if (((s.b[2665] && s.b[2666]) && s.b[2679]) && s.b[2680]) {s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::mul(s.ad_value(2670), s.ad_value(573)));}
        if (((s.b[2665] && s.b[2666]) && s.b[2679]) && (!s.b[2680])) {s.store_pow_sub_from_scalar_mul_base_indices(2669, 1.0, 2670, 573, 576);}
        if ((s.b[2665] && s.b[2666]) && s.b[2679]) {s.store_add_scaled_product_mixed_aia(1922, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2669)), 1.0, 588, A::sub(s.ad_value(833), s.ad_value(2670)), 1.0);}
        s.b[2681] = (s.v[680] > 0.5);s.store_scalar(2681, if s.b[2681] { 1.0 } else { 0.0 });s.b[2682] = (s.v[577] == 0.5);s.store_scalar(2682, if s.b[2682] { 1.0 } else { 0.0 });
        if (((s.b[2665] && s.b[2666]) && s.b[2681]) && s.b[2682]) {s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::mul(s.ad_value(2670), s.ad_value(574)));}
        if (((s.b[2665] && s.b[2666]) && s.b[2681]) && (!s.b[2682])) {s.store_pow_sub_from_scalar_mul_base_indices(2669, 1.0, 2670, 574, 577);}
        if ((s.b[2665] && s.b[2666]) && s.b[2681]) {s.store_add_scaled_product_mixed_aia(1923, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2669)), 1.0, 589, A::sub(s.ad_value(833), s.ad_value(2670)), 1.0);}
        s.b[2683] = (p.p872 > 0.0);s.store_scalar(2683, if s.b[2683] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2683]) {s.store_scaled_offset_ad(642, A::powf(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt_square_offset(A::add(s.ad_value(825), s.ad_value(827)), (0.001 * 0.001)), 0.5), p.p873), (-(((0.5 * 0.001)) as f64).powf(p.p873)), p.p872);s.store_offset(640, 642, p.p862);s.store_div_from_scalar(450, 1.0, 640);s.store_div_from_scalar_offset_scaled_input(453, s.v[453], 642, 1.0 / (p.p862), 1.0);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2683])) {s.store_scalar(640, p.p862);}
        s.b[2684] = (p.p874 > 0.0);s.store_scalar(2684, if s.b[2684] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2684]) {s.store_scaled_offset_ad(644, A::powf(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt_square_offset(A::add(s.ad_value(825), s.ad_value(827)), (0.001 * 0.001)), 0.5), p.p875), (-(((0.5 * 0.001)) as f64).powf(p.p875)), p.p874);s.store_mul_scale_offset_indices(443, 443, 644, 1.0, 1.0);}
        if (s.b[2665] && (!s.b[2666])) {s.store_scalar(2634, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_126(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2665] && (!s.b[2666])) {s.store_scalar(2631, 0.0);}
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
        if ((s.b[2665] && (!s.b[2666])) && s.b[2690]) {s.store_scalar(1912, 0.0);s.store_scalar(1918, 0.0);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) {s.store_scale(2637, 2627, s.v[387]);}
        s.b[2691] = ((p.p840 == 0.0) && (p.p845 == 0.0));s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2691]) {s.store_scalar(2638, 0.0);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {s.store_sub_from_scalar(2639, s.v[393], 2633);s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));}
        s.b[2692] = (p.p831 == 0.5);s.store_scalar(2692, if s.b[2692] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && s.b[2692]) {s.store_scalar(2641, 0.0);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && (!s.b[2692])) {s.store_scaled_add_mixed_ai(2641, A::div_scaled_product(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2640)), 1.0), 2640, (1.0 - (2.0 * p.p831)));}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {s.store_add(2642, 2640, 2641);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_127(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.b[2693] = (p.p831 == 0.5);s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && s.b[2693]) {s.store_sqrt_scaled_input(2636, 2639, s.v[429]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && (!s.b[2693])) {s.store_powf_scaled_input(2636, 2639, s.v[429], p.p831);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {s.store_scale(2643, 2636, s.v[423]);s.store_mul_scale_offset_indices(2644, 2643, 2630, s.v[384], ((-1.0)) * (s.v[384]));s.store_scaled_mul(2638, 2644, 2642, p.p840);}
        s.b[2694] = (p.p845 == 0.0);s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2694]) {s.store_scalar(2645, 0.0);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) {s.store_div_scaled_inputs_indices(2646, 2643, (s.v[408] * s.v[438]), 2639, 1.0);s.store_div_from_scalar(2647, (0.666666666666667 * s.v[435]), 2646);s.store_square(2648, 2647);s.store_sqrt_div_scaled_square_offset_denominator(2649, 2648, 1.0, 1.0, 1.0);s.store_sqrt(2650, 2649);s.store_mul(2651, 2649, 2650);}
        s.b[2695] = (((-p.p831) * s.v[411]) == (-1.0));s.store_scalar(2695, if s.b[2695] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && s.b[2695]) {s.store_div_from_scalar_offset_product(2652, 1.0, 2646, 2651, 1.0);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2695])) {s.store_powf_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), ((-p.p831) * s.v[411]));}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) {s.store_div_scaled_product_add_scaled_denominator_indices(2653, 2642, 2652, 1.0, 2642, 1.0, 2652, 1.0, 1.0);s.store_sqrt_scaled_input_ad(2654, A::div(s.ad_value(2646), s.ad_value(2650)), 0.375);s.store_add_scaled_product_indices(2655, 2649, (-1.0), 2647, 2650, 2.0);s.store_add_scaled_value_products_indices(2656, 2649, (-s.v[435]), 2647, 2650, s.v[435], 2646, 2651, 0.5);s.store_mul_scale_offset_indices(2657, 2654, 2655, 1.0, (-1.0));s.store_square(2618, 2657);}
        s.b[2696] = (s.v[2657] > 0.0);s.store_scalar(2696, if s.b[2696] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && s.b[2696]) {s.store_div_from_scalar_offset_scaled_input(2619, 1.0, 2657, s.v[372], 1.0);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2696])) {s.store_div_from_scalar_sub_from_scalar_ad(2619, 1.0, 1.0, A::scale(s.ad_value(2657), s.v[372]));}
        s.b[2697] = (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458));s.store_scalar(2697, if s.b[2697] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && s.b[2697]) {s.store_exp_sub(2636, 2656, 2618);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2697])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2636, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) {s.store_mul_mixed_ai(2620, A::add_scaled_inputs_product(s.ad_value(2619), 0.29214664, A::square(s.ad_value(2619)), s.v[373], A::square(s.ad_value(2619)), s.ad_value(2619), s.v[374]), 2636);}
        s.b[2698] = (s.v[2657] > 0.0);s.store_scalar(2698, if s.b[2698] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && s.b[2698]) {s.copy_ad(2658, 2620);}
        s.b[2699] = (s.v[2656] > (-230.25850929940458));s.store_scalar(2699, if s.b[2699] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2698])) && s.b[2699]) {s.store_exp(2636, 2656);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2698])) && (!s.b[2699])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 2656, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) && (!s.b[2698])) {s.store_sub_scaled_inputs(2658, 2636, 2.0, 2620, 1.0);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2694])) {s.store_div_scaled_inputs_indices(2659, 2658, (s.v[435] * (1.772453850905516 * 0.5)), 2654, 1.0);s.store_mul3_affine_lhs(2645, 2644, 2659, p.p845, 0.0, 2653);}
        s.b[2700] = (p.p851 == 0.0);s.store_scalar(2700, if s.b[2700] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2700]) {s.store_scalar(2660, 0.0);}
        s.b[2701] = (p.p831 == 0.5);s.store_scalar(2701, if s.b[2701] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2701]) {s.store_sqrt_scaled_input_ad(2636, A::sub_from_scalar(p.p828, s.ad_value(2634)), s.v[429]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2701])) {s.store_powf_scale_offset_input(2636, 2634, (-s.v[429]), ((p.p828) * (s.v[429])), p.p831);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) {s.store_div_scaled_offset_numerator_indices(2661, 2634, ((-s.v[426]) * s.v[411]), (((p.p828) * (s.v[426])) * s.v[411]), 2636, 1.0);}
        s.b[2702] = (((((-s.v[441]) / s.v[2661])) as f64).abs() < 230.25850929940458);s.store_scalar(2702, if s.b[2702] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2702]) {s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(2661), 1.0));}
        s.b[2703] = (((-s.v[441]) / s.v[2661]) < 0.0);s.store_scalar(2703, if s.b[2703] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2702])) && s.b[2703]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 441, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2702])) && (!s.b[2703])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 441, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) {s.store_mul_scale_offset_mixed_ai(2660, A::mul3(s.ad_value(832), s.ad_value(2661), s.ad_value(2661)), 2636, p.p851, 0.0);}
        s.b[2704] = (p.p860 > 1000.0);s.store_scalar(2704, if s.b[2704] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2704]) {s.store_scalar(2662, 1.0);}
        s.b[2705] = (s.v[2635] > ((-s.v[444]) * p.p860));s.store_scalar(2705, if s.b[2705] { 1.0 } else { 0.0 });s.b[2706] = (p.p863 == 4.0);s.store_scalar(2706, if s.b[2706] { 1.0 } else { 0.0 });
    }
}
