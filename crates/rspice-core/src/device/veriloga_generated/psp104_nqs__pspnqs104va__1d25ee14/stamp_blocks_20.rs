#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2530, A::add_scaled_square_product(s.ad_value(2529), 0.5, s.ad_value(2528), s.ad_value(2545), (-1.0)), 1.0);s.store_add_mixed_ia(2546, 2527, A::div_scaled_product3(s.ad_value(2528), s.ad_value(824), s.ad_value(2530), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530), s.ad_value(2530)), s.ad_value(2529), A::add_scaled_square_product(s.ad_value(2529), 0.3333333333333333, s.ad_value(2528), s.ad_value(2545), (-1.0)))), 1.0));}
        s.b[2584] = (s.v[2546] < 230.25850929940458);s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && (!s.b[2583])) && s.b[2584]) {s.store_exp(2532, 2546);s.store_div_from_scalar(2533, 1.0, 2532);s.store_mul(2532, 2483, 2532);}
        s.b[2585] = (s.v[2546] > (s.v[2481] - 230.25850929940458));s.store_scalar(2585, if s.b[2585] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2575]) && (!s.b[2583])) && (!s.b[2584])) && s.b[2585]) {s.store_exp_sub(2532, 2546, 2481);s.store_div(2533, 2483, 2532);}
        if ((((s.b[2547] && s.b[2575]) && (!s.b[2583])) && (!s.b[2584])) && (!s.b[2585])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2532, 1e-100, A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2533, 1e-100, 2546, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2583])) {s.store_div_from_scalar_offset_square(2522, 1.0, 2546, 2.0);s.store_mul_square_lhs(2534, 2546, 2522);s.store_mul3_affine_lhs(2535, 2546, 2522, 4.0, 0.0, 2522);s.store_mul_ad_product_lhs_mixed_ai(2536, A::sub_scaled_inputs(s.ad_value(2522), 8.0, s.ad_value(2534), 12.0), 2522, 2522);s.store_sub(2522, 2417, 2546);s.store_add_scaled_product_mixed_iia(2537, 2522, 2.0, 2399, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2533)), 1.0, s.ad_value(2532), 1.0, s.ad_value(2483), A::offset(s.ad_value(2535), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2538, 2522, 1.0, 2399, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2533), 1.0, s.ad_value(2546), 1.0, s.ad_value(2532), 1.0, (-1.0)), 1.0, s.ad_value(2483), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2522, 2.0, 2399, A::add_scaled_inputs_product(s.ad_value(2533), 1.0, s.ad_value(2532), 1.0, s.ad_value(2483), s.ad_value(2536), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2522, 2537, 1.0, 2538, 2522, (-2.0));s.store_add_scaled_inputs_mixed_ia(2484, 2546, 1.0, A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0);}
        if (s.b[2547] && s.b[2575]) {s.store_sub(2485, 2484, 2437);}
        s.b[2586] = (s.v[2485] < 1e-10);s.store_scalar(2586, if s.b[2586] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2586]) {s.store_add_scaled_inputs_product_mixed_iiia(2486, 2417, 2.0, 2437, (-2.0), 2399, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2443), 1.0, s.ad_value(2442), s.ad_value(2482), 1.0), 1.0, s.ad_value(2483), s.ad_value(2440), 1.0, (-1.0)), 1.0);s.store_mul_mixed_ai(2487, A::mul_sub_from_scalar_rhs(s.ad_value(2399), 1.0, s.ad_value(2482)), 2444);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2547] && s.b[2575]) && s.b[2586]) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2027, 2.0, 2399, A::add_scaled_value_products(s.ad_value(2443), 1.0, s.ad_value(2442), s.ad_value(2482), 1.0, s.ad_value(2483), s.ad_value(2441), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2027, 2486, 1.0, 2027, 2487, (-2.0));s.store_scaled_div_mixed_ia(2485, 2487, A::add(s.ad_value(2486), A::sqrt(s.ad_value(2027))), 2.0);s.store_add(2484, 2437, 2485);}
        if (s.b[2547] && s.b[2575]) {s.store_mul(2488, 2485, 2413);s.store_div_scaled_product_offset_denominator_mixed_iia(2489, 2484, 2484, 1.0, A::square(s.ad_value(2484)), 2.0, 1.0);}
        s.b[2587] = (s.v[2484] < 230.25850929940458);s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2587]) {s.store_exp_neg_input(2490, 2484);}
        s.b[2588] = (s.v[2484] < 1e-5);s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && s.b[2588]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2491, 2484, 1.0, 2484, 1.0, 2484, 0.25, 0.3333333333333333, 0.5);s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2484), 1.0, A::scale(s.ad_value(2484), 0.25), 0.3333333333333333));s.store_scaled_mul(2492, 2484, 2027, 0.7071067811865475);s.store_mul3_ad_middle(2493, A::mul3_scaled_output(s.ad_value(2483), s.ad_value(2484), s.ad_value(2484), 0.16666666666666666), 2484, A::scale_offset(s.ad_value(2484), 1.75, 1.0));}
        if (((s.b[2547] && s.b[2575]) && s.b[2587]) && (!s.b[2588])) {s.store_add_offset_lhs(2491, 2484, (-1.0), 2490);s.store_sqrt(2492, 2491);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(2493, 2483, A::div_from_scalar(1.0, s.ad_value(2490)), 1.0, 2484, (-1.0), 2489, -1.0, (-1.0));}
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
        if ((s.b[2547] && s.b[2575]) && s.b[2591]) {s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2495), 1.0, A::scale(s.ad_value(2495), 0.25), 0.3333333333333333));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2547] && s.b[2575]) && s.b[2591]) {s.store_scaled_mul(2502, 2495, 2027, 0.7071067811865475);s.store_add_mixed_ia(2503, 2501, A::div_scaled_product(s.ad_value(2398), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2495), 0.5)), 1.0, A::square(s.ad_value(2495)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0));}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2591])) {s.store_add_offset_lhs(2499, 2495, (-1.0), 2496);s.store_mul_sqrt_mixed_ia(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));}
        s.b[2593] = (s.v[730] > 0.0);s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2575]) && (!s.b[2591])) && s.b[2593]) {s.store_add_scaled_sub_value_product_indices(2504, 1.0, 2496, 1.0, 2500, 2415, 2.0);s.store_div_from_scalar_sqrt_ad(2501, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0));s.store_div_scaled_value_offset_denominator(2027, s.ad_value(2501), 1.0, s.ad_value(2501), 1.0, 1.0);s.store_mul_product3_mixed_iaii(2505, 730, A::square(s.ad_value(2027)), 2399, 2498, 1.0);s.store_add_scaled_inputs_product_mixed_iiia(2506, 2500, 2.0, 2505, (-2.0), 2399, A::add(A::sub_from_scalar(1.0, s.ad_value(2496)), s.ad_value(2498)), 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(2507, 2505, 2505, 1.0, 2500, 2.0);s.store_sub_from_scalar_scaled_mul_mixed_ia(2508, 1.0, 2399, A::add(s.ad_value(2496), s.ad_value(2498)), 0.5);s.store_div_scaled_product_mixed_iia(2509, 2507, 2506, 1.0, A::add_scaled_square_product(s.ad_value(2506), 1.0, s.ad_value(2508), s.ad_value(2507), (-1.0)), 1.0);s.store_add(2495, 2495, 2509);s.store_exp(2510, 2509);s.store_div(2496, 2496, 2510);s.store_mul(2498, 2498, 2510);s.store_add_offset_lhs(2499, 2495, (-1.0), 2496);s.store_mul_sqrt_mixed_ia(2500, 2398, A::add(s.ad_value(2498), s.ad_value(2499)));s.store_add_ad(2511, A::sub_from_scalar(1.0, s.ad_value(2496)), A::mul3_scaled_output(s.ad_value(2500), s.ad_value(2501), s.ad_value(2415), 2.0));s.store_div_scaled_product3_mixed_iiaa(2485, 2485, 2510, A::add(s.ad_value(2504), s.ad_value(2497)), 1.0, A::add_scaled_product(s.ad_value(2511), 1.0, s.ad_value(2510), s.ad_value(2497), 1.0), 1.0);s.store_mul(2488, 2485, 2413);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2591])) {s.store_sqrt(2502, 2499);s.store_add_scaled_inputs_mixed_ia(2503, 2501, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, s.ad_value(2496)), s.ad_value(2502)), 0.5);}
        if (s.b[2547] && s.b[2575]) {s.store_mul_div_scaled_product_mixed_iiia(2512, 2413, 2399, 2498, 1.0, A::add_scaled_product(s.ad_value(2500), 1.0, s.ad_value(2398), s.ad_value(2502), 1.0), 1.0);s.store_add_scaled_product_indices(2513, 2512, 1.0, 2413, 2503, 1.0);s.store_mul3_lhs(2514, 2502, 2398, 2413);}
        s.b[2594] = (s.v[218] < 0.0);s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2594]) {s.store_sub_from_scalar_scaled_mul(2453, 1.0, 218, 2512, 1.0);}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2594])) {s.store_div_from_scalar_offset_product(2453, 1.0, 218, 2512, 1.0);}
        if (s.b[2547] && s.b[2575]) {s.store_mul_product3_indices(2454, 2512, 757, 2452, 2453, 1.0);s.store_add_scaled_product_indices(2515, 2514, 1.0, 775, 2512, 1.0);s.store_add_scaled_product_indices(2516, 2514, 1.0, 776, 2512, 1.0);s.store_mul(2517, 774, 2515);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_56(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2547] && s.b[2575]) {s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2499), 1.0, A::add(s.ad_value(2499), s.ad_value(2498)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2456, A::pow(A::mul(s.ad_value(2517), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);s.store_mul_add_mixed_iai(2518, 2448, A::offset(s.ad_value(2456), 1.0), 2454);s.store_ln_ad(2519, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(826), s.ad_value(2488)), s.ad_value(779)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2479), s.ad_value(2488)), s.ad_value(779)), 1.0), 1.0));s.store_mul(2029, 2512, 2458);s.store_div_add_scaled_inputs_rhs_indices(2459, 2029, 223, 1.0, 2029, 1.0);}
        s.b[2595] = (s.v[222] < 0.0);s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2575]) && s.b[2595]) {s.store_div_from_scalar_sub_from_scalar_ad(2460, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2459)));}
        if ((s.b[2547] && s.b[2575]) && (!s.b[2595])) {s.store_offset_mul(2460, 222, 2459, 1.0);}
        if (s.b[2547] && s.b[2575]) {s.store_mul(2521, 2393, 2460);s.store_mul(2520, 2500, 2413);}
        if s.b[2547] {s.copy_ad(1887, 2395);s.copy_ad(1888, 2413);s.copy_ad(1889, 2398);s.copy_ad(1890, 2417);s.copy_ad(1891, 2422);s.copy_ad(1892, 2451);s.copy_ad(1893, 2488);s.copy_ad(1894, 2494);s.copy_ad(1895, 2501);s.copy_ad(1896, 2503);s.copy_ad(1897, 2512);s.copy_ad(1898, 2513);s.copy_ad(1899, 2516);s.copy_ad(1900, 2518);s.copy_ad(1901, 2519);s.copy_ad(1902, 2521);s.copy_ad(1903, 2520);s.copy_ad(1932, 2414);s.copy_ad(1933, 2435);s.copy_ad(1934, 2495);s.copy_ad(1935, 2500);}
        if (!s.b[2547]) {s.copy_ad(745, 728);s.copy_ad(1887, 1822);s.copy_ad(1888, 1824);s.copy_ad(1889, 1826);s.copy_ad(1890, 1829);s.copy_ad(1891, 1830);s.copy_ad(1892, 1849);s.copy_ad(1893, 1860);s.copy_ad(1894, 1861);s.copy_ad(1895, 1863);s.copy_ad(1896, 1864);s.copy_ad(1897, 1865);s.copy_ad(1898, 1866);s.copy_ad(1899, 1868);s.copy_ad(1900, 1869);s.copy_ad(1901, 1871);s.copy_ad(1902, 1870);s.copy_ad(1903, 1872);s.copy_ad(1932, 1825);s.copy_ad(1933, 1833);s.copy_ad(1934, 1862);s.copy_ad(1935, 1931);}
        s.copy_ad(1904, 255);s.b[2596] = (s.v[773] > 0.0);s.store_scalar(2596, if s.b[2596] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[2596] {s.store_div_scaled_value_offset_denominator(1904, s.ad_value(255), 1.0, A::mul(s.ad_value(773), A::powf(A::offset(A::square(s.ad_value(1899)), s.v[733]), ((-1.0) * 0.16666666666666666))), 1.0, 1.0);}
        s.store_scalar(1905, 1.0);s.store_scalar(1906, 1.0);s.store_scalar(1907, 0.0);s.store_scalar(1908, 1.0);s.store_scalar(1909, 1.0);s.copy_ad(2359, 1903);s.store_scalar(2362, 0.0);s.store_scalar(2361, 0.0);s.copy_ad(2363, 2359);s.b[2597] = (s.v[1890] > 0.0);s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });
        if s.b[2597] {s.store_mul_div_scaled_product_mixed_iaii(2354, 1901, A::add(s.ad_value(260), A::div(s.ad_value(261), s.ad_value(1898))), 1897, 1.0, 1898, 1.0);}
        s.b[2598] = (s.v[2354] > 0.0);s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });
        if (s.b[2597] && s.b[2598]) {s.store_div_from_scalar_add_ad(1905, 1.0, A::offset(s.ad_value(2354), 1.0), A::square(s.ad_value(2354)));}
        if (s.b[2597] && (!s.b[2598])) {s.store_sub_from_scalar(1905, 1.0, 2354);}
        if s.b[2597] {s.store_mul(1906, 1900, 1905);s.store_div(1907, 1902, 1906);s.store_mul_ad_product_lhs_mixed_ai(2355, A::square(s.ad_value(1907)), 1893, 1893);}
        s.b[2599] = (s.v[0] == (-1.0));s.store_scalar(2599, if s.b[2599] { 1.0 } else { 0.0 });
        if (s.b[2597] && s.b[2599]) {s.store_div_scaled_value_offset_denominator(2355, s.ad_value(2355), 1.0, A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0, 1.0);}
        if s.b[2597] {s.store_mul_scale_offset_mixed_ia(1908, 1906, A::sqrt(A::scale_offset(s.ad_value(2355), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div(2027, 1906, 1908);s.store_mul_scale_offset_mixed_ia(2356, 1896, A::mul3_scaled_output(s.ad_value(2355), s.ad_value(2027), s.ad_value(2027), 0.5), 1.0, 1.0);s.store_div_scaled_product_indices(1909, 2027, 1898, 1.0, 2356, 1.0);s.store_scaled_div(2357, 1893, 1909, 0.5);s.store_square(2358, 2357);s.store_add_product3_rhs_mixed_iia(2359, 1903, 1895, 1893, A::add(A::offset(A::mul_scaled_output(s.ad_value(2357), s.ad_value(1905), 0.3333333333333333), (-1.0)), s.ad_value(1905)), 0.5);s.store_scaled_mul(2027, 1896, 1893, 0.16666666666666666);}
        s.b[2600] = (p[49] == 1.0);s.store_scalar(2600, if s.b[2600] { 1.0 } else { 0.0 });
        if (s.b[2597] && s.b[2600]) {s.store_scalar(2360, 0.0);s.store_mul_ad_affine_product_rhs(2361, 1905, s.ad_value(1905), A::sub(s.ad_value(1897), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 2.0, s.ad_value(2357), 3.0)), 0.5, 0.0);}
        if (s.b[2597] && (!s.b[2600])) {s.store_mul_scale_offset_mixed_ai(2360, A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(1896), s.ad_value(1893), (-0.5)), 1905, -1.0, 1.0);s.store_add_scaled_products_mixed_aaia(2361, A::square(s.ad_value(1905)), A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(2027), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2357)), 1.0, s.ad_value(2358), 0.2), (-1.0)), 0.5, 2360, A::offset(s.ad_value(1905), 1.0), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2597] {s.store_add_scaled_product_mixed_iia(2362, 2360, 1.0, 1905, A::add_scaled_product(s.ad_value(1897), 1.0, s.ad_value(2027), s.ad_value(2357), 1.0), 1.0);s.store_sub(2363, 2359, 2362);}
        s.store_mul(851, 2359, 1904);s.store_mul_scale_offset_indices(853, 1904, 2361, -1.0, 0.0);s.store_mul_scale_offset_indices(852, 1904, 2363, -1.0, 0.0);s.store_scalar(2379, 0.0);s.store_scalar(2380, 0.0);s.store_scalar(2378, 0.0);s.b[2601] = ((s.v[268] > 0.0) || (s.v[269] > 0.0));s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });
        if s.b[2601] {s.store_scalar(2368, 1.0);s.copy_ad(2367, 1887);}
        s.b[2602] = (s.v[272] > 1e-10);s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });
        if (s.b[2601] && s.b[2602]) {s.store_add_scaled_inputs3_indices(2364, 1887, 1.0, 270, (-1.0), 808, 1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2027, 2364, 0.5, 808, 0.5, A::add(A::square(A::sub(s.ad_value(2364), s.ad_value(808))), s.ad_value(809)), 0.5);s.store_mul_add_scaled_inputs3_offset_rhs_indices(2028, 2027, 2027, 2.0, 808, (-1.0), 2364, -1.0, 0.0);s.store_div(2029, 808, 2027);s.store_mul(2365, 2364, 2029);s.store_sqrt_sub_from_scalar_ad(2366, 1.0, A::mul(s.ad_value(2365), s.ad_value(272)));s.store_add_scaled_inputs3_mixed_aii(2367, A::div(A::sub_from_scalar(1.0, s.ad_value(2366)), s.ad_value(272)), 1.0, 2364, 1.0, 2365, -1.0);s.store_offset_ad(2368, A::div_scaled_product3(A::offset(A::div_from_scalar(0.5, s.ad_value(2366)), (-1.0)), A::add_scaled_product(s.ad_value(2028), 1.0, s.ad_value(2364), A::sub(s.ad_value(808), s.ad_value(2027)), 1.0), s.ad_value(2029), 1.0, s.ad_value(2028), 1.0), 1.0);}
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
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        s: &mut ReactiveScratch,
    ) {
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
        if (s.b[2608] && (!s.b[2609])) {s.copy_ad(2382, 2027);s.store_mul_scale_offset_mixed_ia(2028, 2382, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0)), -1.0, 1.0);}
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
        s.store_add(2387, 2383, 2386);s.store_add_scaled_product_indices(856, 2387, 1.0, 267, 829, 1.0);s.store_mul(854, 274, 834);s.store_mul(855, 275, 837);s.store_scalar(1938, 0.0);s.store_scalar(1939, 0.0);s.store_scalar(1940, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(1941, 0.0);s.b[2616] = (s.v[1] != 0.0);s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });s.b[2617] = (s.v[1890] <= 0.0);s.store_scalar(2617, if s.b[2617] { 1.0 } else { 0.0 });
        if (s.b[2616] && s.b[2617]) {s.store_scalar(1936, 0.5);s.store_scalar(1937, 1.0);s.copy_ad(1938, 1889);}
        if (s.b[2616] && (!s.b[2617])) {s.store_offset_scaled_div(1936, 1893, 1909, ((0.25) * (0.5)), 0.5);s.store_div_add_scaled_inputs_rhs_indices(1937, 1935, 1890, 1.0, 1934, -1.0);s.store_div(1938, 1889, 1937);}
        if s.b[2616] {s.store_square(1939, 1938);s.store_offset_scaled(1940, 1938, 0.7071067811865475, 1.0);s.store_scale(1941, 1940, 1e-5);}
        s.store_scalar(2618, 0.0);s.store_scalar(2621, 0.0);s.store_scalar(2622, 0.0);s.store_scalar(2623, 0.0);s.store_scalar(2624, 0.0);s.store_scalar(2625, 0.0);s.store_scalar(2626, 0.0);s.store_scalar(2627, 0.0);s.store_scalar(2628, 0.0);s.store_scalar(2629, 0.0);s.store_scalar(2630, 0.0);s.store_scalar(2631, 0.0);s.store_scalar(2632, 0.0);s.store_scalar(2633, 0.0);s.store_scalar(2634, 0.0);s.store_scalar(2635, 0.0);s.store_scalar(2636, 0.0);s.store_scalar(2639, 0.0);s.store_scalar(2643, 0.0);s.store_scalar(2646, 0.0);s.store_scalar(2647, 0.0);s.store_scalar(2648, 0.0);s.store_scalar(2649, 0.0);s.store_scalar(2650, 0.0);s.store_scalar(2651, 0.0);s.store_scalar(2654, 0.0);s.store_scalar(2655, 0.0);s.store_scalar(2656, 0.0);s.store_scalar(2657, 0.0);s.store_scalar(2661, 0.0);s.store_scalar(2663, 0.0);s.store_scalar(2664, 0.0);s.store_scalar(857, 0.0);s.store_scalar(1918, 0.0);s.store_scalar(1919, 0.0);s.store_scalar(1920, 0.0);s.store_scalar(858, 0.0);s.store_scalar(1921, 0.0);s.store_scalar(1922, 0.0);s.store_scalar(1923, 0.0);s.b[2665] = (p[43] > 0.0);s.store_scalar(2665, if s.b[2665] { 1.0 } else { 0.0 });s.b[2666] = (s.v[474] == 1.0);s.store_scalar(2666, if s.b[2666] { 1.0 } else { 0.0 });
        if (s.b[2665] && s.b[2666]) {s.store_scalar(2669, 0.0);s.store_scalar(2670, 0.0);s.store_primal_scaled_mul(2621, 657, 657, 4.0);s.store_primal_div(2622, 657, 658);s.store_add_scaled_product_indices(2623, 832, 1.0, 657, 2622, 1.0);s.store_add(2624, 658, 2623);s.store_sub(2625, 658, 2623);s.store_sqrt_square_add(2626, 2625, 2621);s.store_div_scaled_product_add_scaled_denominator_indices(2670, 832, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
        s.b[2671] = (s.v[651] > 0.5);s.store_scalar(2671, if s.b[2671] { 1.0 } else { 0.0 });s.b[2672] = (s.v[408] == 0.5);s.store_scalar(2672, if s.b[2672] { 1.0 } else { 0.0 });
        if (((s.b[2665] && s.b[2666]) && s.b[2671]) && s.b[2672]) {s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[405]));}
        if (((s.b[2665] && s.b[2666]) && s.b[2671]) && (!s.b[2672])) {s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[405])), s.v[408]);}
        if ((s.b[2665] && s.b[2666]) && s.b[2671]) {s.store_add_scaled_inputs3_offset_indices(1918, 2669, (-s.v[417]), 832, s.v[420], 2670, (-s.v[420]), s.v[417]);}
        s.b[2673] = (s.v[652] > 0.5);s.store_scalar(2673, if s.b[2673] { 1.0 } else { 0.0 });s.b[2674] = (s.v[409] == 0.5);s.store_scalar(2674, if s.b[2674] { 1.0 } else { 0.0 });
        if (((s.b[2665] && s.b[2666]) && s.b[2673]) && s.b[2674]) {s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[406]));}
        if (((s.b[2665] && s.b[2666]) && s.b[2673]) && (!s.b[2674])) {s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[406])), s.v[409]);}
        if ((s.b[2665] && s.b[2666]) && s.b[2673]) {s.store_add_scaled_inputs3_offset_indices(1919, 2669, (-s.v[418]), 832, s.v[421], 2670, (-s.v[421]), s.v[418]);}
        s.b[2675] = (s.v[653] > 0.5);s.store_scalar(2675, if s.b[2675] { 1.0 } else { 0.0 });s.b[2676] = (s.v[410] == 0.5);s.store_scalar(2676, if s.b[2676] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2665] && s.b[2666]) && s.b[2675]) && s.b[2676]) {s.store_sqrt_sub_from_scalar_ad(2669, 1.0, A::scale(s.ad_value(2670), s.v[407]));}
        if (((s.b[2665] && s.b[2666]) && s.b[2675]) && (!s.b[2676])) {s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[407])), s.v[410]);}
        if ((s.b[2665] && s.b[2666]) && s.b[2675]) {s.store_add_scaled_inputs3_offset_indices(1920, 2669, (-s.v[419]), 832, s.v[422], 2670, (-s.v[422]), s.v[419]);}
        if (s.b[2665] && s.b[2666]) {s.store_scalar(2669, 0.0);s.store_scalar(2670, 0.0);s.store_primal_scaled_mul(2621, 684, 684, 4.0);s.store_primal_div(2622, 684, 685);s.store_add_scaled_product_indices(2623, 833, 1.0, 684, 2622, 1.0);s.store_add(2624, 685, 2623);s.store_sub(2625, 685, 2623);s.store_sqrt_square_add(2626, 2625, 2621);s.store_div_scaled_product_add_scaled_denominator_indices(2670, 833, 685, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
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
        s.b[2683] = (p[872] > 0.0);s.store_scalar(2683, if s.b[2683] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2683]) {s.store_scaled_offset_ad(642, A::powf(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt_square_offset(A::add(s.ad_value(825), s.ad_value(827)), (0.001 * 0.001)), 0.5), p[873]), (-(((0.5 * 0.001)) as f64).powf(p[873])), p[872]);s.store_offset(640, 642, p[862]);s.store_div_from_scalar(450, 1.0, 640);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2683])) {s.store_scalar(640, p[862]);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2684] = (p[874] > 0.0);s.store_scalar(2684, if s.b[2684] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2684]) {s.store_scaled_offset_ad(644, A::powf(A::add_scaled_inputs3(s.ad_value(825), 0.5, s.ad_value(827), 0.5, A::sqrt_square_offset(A::add(s.ad_value(825), s.ad_value(827)), (0.001 * 0.001)), 0.5), p[875]), (-(((0.5 * 0.001)) as f64).powf(p[875])), p[874]);s.store_mul_scale_offset_indices(443, 443, 644, 1.0, 1.0);}
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
        s.b[2691] = ((p[840] == 0.0) && (p[845] == 0.0));s.store_scalar(2691, if s.b[2691] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {s.store_sub_from_scalar(2639, s.v[393], 2633);}
        s.b[2693] = (p[831] == 0.5);s.store_scalar(2693, if s.b[2693] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && s.b[2693]) {s.store_sqrt_scaled_input(2636, 2639, s.v[429]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) && (!s.b[2693])) {s.store_powf_scaled_input(2636, 2639, s.v[429], p[831]);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2691])) {s.store_scale(2643, 2636, s.v[423]);}
        s.b[2694] = (p[845] == 0.0);s.store_scalar(2694, if s.b[2694] { 1.0 } else { 0.0 });
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
        s.b[2700] = (p[851] == 0.0);s.store_scalar(2700, if s.b[2700] { 1.0 } else { 0.0 });s.b[2701] = (p[831] == 0.5);s.store_scalar(2701, if s.b[2701] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2701]) {s.store_sqrt_scaled_input_ad(2636, A::sub_from_scalar(p[828], s.ad_value(2634)), s.v[429]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2701])) {s.store_powf_scale_offset_input(2636, 2634, (-s.v[429]), ((p[828]) * (s.v[429])), p[831]);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) {s.store_div_scaled_offset_numerator_indices(2661, 2634, ((-s.v[426]) * s.v[411]), (((p[828]) * (s.v[426])) * s.v[411]), 2636, 1.0);}
        s.b[2702] = (((((-s.v[441]) / s.v[2661])) as f64).abs() < 230.25850929940458);s.store_scalar(2702, if s.b[2702] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && s.b[2702]) {s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(441), -1.0, s.ad_value(2661), 1.0));}
        s.b[2703] = (((-s.v[441]) / s.v[2661]) < 0.0);s.store_scalar(2703, if s.b[2703] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2702])) && s.b[2703]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 441, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2700])) && (!s.b[2702])) && (!s.b[2703])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 441, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2704] = (p[860] > 1000.0);s.store_scalar(2704, if s.b[2704] { 1.0 } else { 0.0 });s.b[2705] = (s.v[2635] > ((-s.v[444]) * p[860]));s.store_scalar(2705, if s.b[2705] { 1.0 } else { 0.0 });s.b[2706] = (p[863] == 4.0);s.store_scalar(2706, if s.b[2706] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2704])) && s.b[2705]) && s.b[2706]) {s.store_mul_scale_offset_mixed_ai(2636, A::mul3_scaled_output(s.ad_value(2635), s.ad_value(2635), s.ad_value(2635), ((s.v[448] * s.v[448]) * s.v[448])), 2635, s.v[448], 0.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2704])) && s.b[2705]) && (!s.b[2706])) {s.store_powf_ad(2636, A::abs_scaled_input(s.ad_value(2635), s.v[448]), p[863]);}
        s.b[2707] = (s.v[408] == 0.5);s.store_scalar(2707, if s.b[2707] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && s.b[2707]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[405]));}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) && (!s.b[2707])) {s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[405])), s.v[408]);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2690])) {s.store_add_scaled_inputs3_offset_indices(1918, 2636, ((-s.v[417]) * p[30]), 832, (s.v[420] * p[30]), 2628, ((-s.v[420]) * p[30]), (s.v[417] * p[30]));}
        s.b[2708] = (s.v[647] == 0.0);s.store_scalar(2708, if s.b[2708] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2708]) {s.store_scalar(1919, 0.0);}
        s.b[2709] = ((p[841] == 0.0) && (p[846] == 0.0));s.store_scalar(2709, if s.b[2709] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) {s.store_sub_from_scalar(2639, s.v[394], 2633);}
        s.b[2711] = (p[832] == 0.5);s.store_scalar(2711, if s.b[2711] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) && s.b[2711]) {s.store_sqrt_scaled_input(2636, 2639, s.v[430]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) && (!s.b[2711])) {s.store_powf_scaled_input(2636, 2639, s.v[430], p[832]);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2709])) {s.store_scale(2643, 2636, s.v[424]);}
        s.b[2712] = (p[846] == 0.0);s.store_scalar(2712, if s.b[2712] { 1.0 } else { 0.0 });
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
        s.b[2718] = (p[852] == 0.0);s.store_scalar(2718, if s.b[2718] { 1.0 } else { 0.0 });s.b[2719] = (p[832] == 0.5);s.store_scalar(2719, if s.b[2719] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && s.b[2719]) {s.store_sqrt_scaled_input_ad(2636, A::sub_from_scalar(p[829], s.ad_value(2634)), s.v[430]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2719])) {s.store_powf_scale_offset_input(2636, 2634, (-s.v[430]), ((p[829]) * (s.v[430])), p[832]);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) {s.store_div_scaled_offset_numerator_indices(2661, 2634, ((-s.v[427]) * s.v[412]), (((p[829]) * (s.v[427])) * s.v[412]), 2636, 1.0);}
        s.b[2720] = (((((-s.v[442]) / s.v[2661])) as f64).abs() < 230.25850929940458);s.store_scalar(2720, if s.b[2720] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && s.b[2720]) {s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2661), 1.0));}
        s.b[2721] = (((-s.v[442]) / s.v[2661]) < 0.0);s.store_scalar(2721, if s.b[2721] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2720])) && s.b[2721]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 442, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2718])) && (!s.b[2720])) && (!s.b[2721])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 442, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2722] = (p[861] > 1000.0);s.store_scalar(2722, if s.b[2722] { 1.0 } else { 0.0 });s.b[2723] = (s.v[2635] > ((-s.v[444]) * p[861]));s.store_scalar(2723, if s.b[2723] { 1.0 } else { 0.0 });s.b[2724] = (p[864] == 4.0);s.store_scalar(2724, if s.b[2724] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2722])) && s.b[2723]) && s.b[2724]) {s.store_mul_scale_offset_mixed_ai(2636, A::mul3_scaled_output(s.ad_value(2635), s.ad_value(2635), s.ad_value(2635), ((s.v[449] * s.v[449]) * s.v[449])), 2635, s.v[449], 0.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2722])) && s.b[2723]) && (!s.b[2724])) {s.store_powf_ad(2636, A::abs_scaled_input(s.ad_value(2635), s.v[449]), p[864]);}
        s.b[2725] = (s.v[409] == 0.5);s.store_scalar(2725, if s.b[2725] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && s.b[2725]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[406]));}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) && (!s.b[2725])) {s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[406])), s.v[409]);}
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2708])) {s.store_add_scaled_inputs3_offset_indices(1919, 2636, ((-s.v[418]) * p[30]), 832, (s.v[421] * p[30]), 2628, ((-s.v[421]) * p[30]), (s.v[418] * p[30]));}
        s.b[2726] = (s.v[648] == 0.0);s.store_scalar(2726, if s.b[2726] { 1.0 } else { 0.0 });
        if ((s.b[2665] && (!s.b[2666])) && s.b[2726]) {s.store_scalar(1920, 0.0);}
        s.b[2727] = ((p[842] == 0.0) && (p[847] == 0.0));s.store_scalar(2727, if s.b[2727] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) {s.store_sub_from_scalar(2639, s.v[395], 2633);}
        s.b[2729] = (p[833] == 0.5);s.store_scalar(2729, if s.b[2729] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) && s.b[2729]) {s.store_sqrt_scaled_input(2636, 2639, s.v[431]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) && (!s.b[2729])) {s.store_powf_scaled_input(2636, 2639, s.v[431], p[833]);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2727])) {s.store_scale(2643, 2636, s.v[425]);}
        s.b[2730] = (p[847] == 0.0);s.store_scalar(2730, if s.b[2730] { 1.0 } else { 0.0 });
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
        s.b[2736] = (p[853] == 0.0);s.store_scalar(2736, if s.b[2736] { 1.0 } else { 0.0 });s.b[2737] = (p[833] == 0.5);s.store_scalar(2737, if s.b[2737] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && s.b[2737]) {s.store_sqrt_scaled_input_ad(2636, A::sub_from_scalar(p[830], s.ad_value(2634)), s.v[431]);}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2737])) {s.store_powf_scale_offset_input(2636, 2634, (-s.v[431]), ((p[830]) * (s.v[431])), p[833]);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) {s.store_div_scaled_offset_numerator_indices(2661, 2634, ((-s.v[428]) * s.v[413]), (((p[830]) * (s.v[428])) * s.v[413]), 2636, 1.0);}
        s.b[2738] = (((((-s.v[443]) / s.v[2661])) as f64).abs() < 230.25850929940458);s.store_scalar(2738, if s.b[2738] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && s.b[2738]) {s.store_ad_value(2636, A::exp_div_scaled_inputs(s.ad_value(443), -1.0, s.ad_value(2661), 1.0));}
        s.b[2739] = (((-s.v[443]) / s.v[2661]) < 0.0);s.store_scalar(2739, if s.b[2739] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2738])) && s.b[2739]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2636, 1e-100, (-230.25850929940458), 443, -1.0, 2661, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2736])) && (!s.b[2738])) && (!s.b[2739])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2636, 443, -1.0, 2661, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2740] = (s.v[640] > 1000.0);s.store_scalar(2740, if s.b[2740] { 1.0 } else { 0.0 });s.b[2741] = (s.v[2635] > ((-s.v[444]) * s.v[640]));s.store_scalar(2741, if s.b[2741] { 1.0 } else { 0.0 });s.b[2742] = (p[865] == 4.0);s.store_scalar(2742, if s.b[2742] { 1.0 } else { 0.0 });
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2740])) && s.b[2741]) && s.b[2742]) {s.store_mul_ad_product_lhs_mixed_ai(2636, A::mul3(A::square(A::mul(s.ad_value(2635), s.ad_value(450))), s.ad_value(2635), s.ad_value(450)), 2635, 450);}
        if (((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2740])) && s.b[2741]) && (!s.b[2742])) {s.store_powf_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(450))), p[865]);}
        s.b[2743] = (s.v[473] == 1.0);s.store_scalar(2743, if s.b[2743] { 1.0 } else { 0.0 });
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {
            if (s.v[832] < p[870]) {
                if (((s.v[832] - p[870]) / p[871]) < (-37.0)) {
                    s.store_scalar(2663, p[870]);
                } else {
                    s.store_offset_scaled_ad(2663, A::ln_one_plus_exp(A::scaled_offset(s.ad_value(832), (-p[870]), 1.0 / (p[871]))), p[871], p[870]);
                }
            } else {
                if (((s.v[832] - p[870]) / p[871]) > 37.0) {
                    s.copy_ad(2663, 832);
                } else {
                    s.store_add_scaled_inputs_mixed_ia(2663, 832, 1.0, A::ln_one_plus_exp(A::scale_offset(s.ad_value(832), (-1.0 / (p[871])), ((p[870]) * (1.0 / (p[871]))))), p[871]);
                }
            }
        }
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {s.store_primal_scaled_mul(2621, 657, 657, 4.0);s.store_primal_div(2622, 657, 658);s.store_add_scaled_product_indices(2623, 2663, 1.0, 657, 2622, 1.0);s.store_add(2624, 658, 2623);s.store_sub(2625, 658, 2623);s.store_sqrt_square_add(2626, 2625, 2621);s.store_div_scaled_product_add_scaled_denominator_indices(2664, 2663, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
        s.b[2744] = (s.v[410] == 0.5);s.store_scalar(2744, if s.b[2744] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && s.b[2744]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2664), s.v[407]));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && (!s.b[2744])) {s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2664), s.v[407])), s.v[410]);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {s.store_add_scaled_inputs3_offset_indices(1920, 2636, ((-s.v[419]) * p[30]), 2663, (s.v[422] * p[30]), 2664, ((-s.v[422]) * p[30]), (s.v[419] * p[30]));s.store_sub_offset_lhs(2663, 832, p[870], 2663);s.store_primal_scaled_mul(2621, 657, 657, 4.0);s.store_primal_div(2622, 657, 658);s.store_add_scaled_product_indices(2623, 2663, 1.0, 657, 2622, 1.0);s.store_add(2624, 658, 2623);s.store_sub(2625, 658, 2623);s.store_sqrt_square_add(2626, 2625, 2621);s.store_div_scaled_product_add_scaled_denominator_indices(2664, 2663, 658, 2.0, 2624, 1.0, 2626, 1.0, 1.0);}
        s.b[2745] = (s.v[467] == 0.5);s.store_scalar(2745, if s.b[2745] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && s.b[2745]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::mul(s.ad_value(2664), s.ad_value(466)));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) && (!s.b[2745])) {s.store_pow_sub_from_scalar_mul_base_indices(2636, 1.0, 2664, 466, 467);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && s.b[2743]) {s.store_add_scaled_product_mixed_aia(472, A::mul_sub_from_scalar_rhs(s.ad_value(470), 1.0, s.ad_value(2636)), p[30], 471, A::sub(s.ad_value(2663), s.ad_value(2664)), p[30]);s.store_add(1920, 1920, 472);}
        s.b[2746] = (s.v[410] == 0.5);s.store_scalar(2746, if s.b[2746] { 1.0 } else { 0.0 });
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) && s.b[2746]) {s.store_sqrt_sub_from_scalar_ad(2636, 1.0, A::scale(s.ad_value(2628), s.v[407]));}
        if ((((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) && (!s.b[2746])) {s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[407])), s.v[410]);}
        if (((s.b[2665] && (!s.b[2666])) && (!s.b[2726])) && (!s.b[2743])) {s.store_add_scaled_inputs3_offset_indices(1920, 2636, ((-s.v[419]) * p[30]), 832, (s.v[422] * p[30]), 2628, ((-s.v[422]) * p[30]), (s.v[419] * p[30]));}
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
        if ((s.b[2665] && (!s.b[2666])) && (!s.b[2754])) {s.store_add_scaled_product_mixed_aia(1921, A::mul_sub_from_scalar_rhs(s.ad_value(584), 1.0, s.ad_value(2636)), p[30], 587, A::sub(s.ad_value(833), s.ad_value(2628)), p[30]);}
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
}
