#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_112(
        s: &mut Scratch,
    ) {
        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2474])) {s.store_offset_scaled_ad(2353, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2304), 1.0, s.ad_value(2349)), s.ad_value(2352)), 0.5, 1.0);}
        if ((s.b[2453] && s.b[2454]) && s.b[2471]) {s.store_div_scaled_offset_numerator(2354, A::mul_scaled_lhs(s.ad_value(709), 0.2, s.ad_value(2303)), 1.0, 1.0, A::offset(A::mul(s.ad_value(709), s.ad_value(2303)), 1.0), 1.0);}
        s.b[2475] = (s.v[2350] > 1e-100);s.store_scalar(2475, if s.b[2475] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) {s.store_mul_sqrt_mixed_ia(2355, 2304, A::add(s.ad_value(2351), s.ad_value(2350)));s.store_div_scaled_product3_mixed_iiia(2356, 2305, 2350, 2319, 1.0, A::add_scaled_product(s.ad_value(2355), 1.0, s.ad_value(2304), s.ad_value(2352), 1.0), 1.0);s.store_mul3_lhs(2357, 2352, 2304, 2319);}
        s.b[2476] = (s.v[215] < 0.0);s.store_scalar(2476, if s.b[2476] { 1.0 } else { 0.0 });
        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2476]) {s.store_div_from_scalar_sub_from_scalar_ad(2358, 1.0, 1.0, A::mul(s.ad_value(215), s.ad_value(2303)));}
        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2476])) {s.store_offset_mul(2358, 215, 2303, 1.0);}
        s.b[2477] = (s.v[216] < 0.0);s.store_scalar(2477, if s.b[2477] { 1.0 } else { 0.0 });
        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2477]) {s.store_sub_from_scalar_scaled_mul(2359, 1.0, 216, 2356, 1.0);}
        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2477])) {s.store_div_from_scalar_offset_product(2359, 1.0, 216, 2356, 1.0);}
        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) {s.store_mul_product3_indices(2360, 2356, 746, 2358, 2359, 1.0);s.store_mul_add_scaled_product_rhs_indices(2361, 763, 2357, 1.0, 764, 2356, 1.0);s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2351), 1.0, A::add(s.ad_value(2351), s.ad_value(2350)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2362, A::pow(A::mul(s.ad_value(2361), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);s.store_mul_add_mixed_iai(2363, 2354, A::offset(s.ad_value(2362), 1.0), 2360);}
        s.b[2478] = (s.v[219] < 0.0);s.store_scalar(2478, if s.b[2478] { 1.0 } else { 0.0 });
        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2478]) {s.store_div_from_scalar_sub_from_scalar_ad(2364, 1.0, 1.0, A::mul(s.ad_value(219), s.ad_value(2303)));}
        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2478])) {s.store_offset_mul(2364, 219, 2303, 1.0);}
        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) {s.store_mul(1931, 2356, 2364);s.store_div_add_scaled_inputs_rhs_indices(2365, 1931, 221, 1.0, 1931, 1.0);}
        s.b[2479] = (s.v[220] < 0.0);s.store_scalar(2479, if s.b[2479] { 1.0 } else { 0.0 });
        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && s.b[2479]) {s.store_div_from_scalar_sub_from_scalar_ad(2366, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2365)));}
        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2475]) && (!s.b[2479])) {s.store_offset_mul(2366, 220, 2365, 1.0);}
        if (s.b[2453] && (!s.b[2454])) {s.copy_ad(2301, 1806);s.copy_ad(2303, 1807);s.copy_ad(2319, 1808);s.copy_ad(2320, 1809);s.copy_ad(2304, 1810);s.copy_ad(2305, 1811);s.copy_ad(2321, 1812);s.copy_ad(2323, 1813);s.copy_ad(2328, 1814);s.copy_ad(2329, 1815);s.copy_ad(2340, 1816);}
        let (t0,) = {
    if (s.b[2453] && (!s.b[2454])) {
        (s.v[1817],)
    } else {
        (s.v[2341],)
    }
};
        s.store_scalar(2341, t0);
        if (s.b[2453] && (!s.b[2454])) {s.copy_ad(2342, 1818);s.copy_ad(2449, 1819);s.copy_ad(2344, 1820);s.copy_ad(2343, 1821);s.copy_ad(2346, 1822);s.copy_ad(2347, 1823);s.copy_ad(2348, 1824);s.copy_ad(2349, 1825);s.copy_ad(2351, 1826);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_113(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2453] && (!s.b[2454])) {s.copy_ad(2350, 1827);s.copy_ad(2352, 1828);s.copy_ad(2353, 1829);s.copy_ad(2354, 1830);s.copy_ad(2355, 1831);s.copy_ad(2356, 1832);s.copy_ad(2357, 1833);s.copy_ad(2358, 1834);s.copy_ad(2359, 1835);s.copy_ad(2363, 1836);s.copy_ad(2364, 1837);s.copy_ad(2366, 1838);}
        if s.b[2453] {s.copy_ad(2299, 1921);s.copy_ad(2300, 766);}
        s.b[2480] = (p.p48 != 0.0);s.store_scalar(2480, if s.b[2480] { 1.0 } else { 0.0 });
        if (s.b[2453] && s.b[2480]) {s.copy_ad(2299, 1922);s.copy_ad(2300, 767);}
        if s.b[2453] {s.store_scalar(2368, 0.0);s.store_scale(2367, 2319, 4.60517018598809);s.copy_ad(2384, 2367);s.copy_ad(2385, 815);s.store_mul(2386, 815, 2320);s.copy_ad(2390, 2343);s.store_scalar(2391, 0.0);s.store_scalar(2394, 0.0);s.copy_ad(2396, 2349);s.copy_ad(2397, 2351);s.copy_ad(2399, 2350);s.copy_ad(2400, 2357);s.copy_ad(2401, 2343);s.copy_ad(2402, 2349);s.copy_ad(2404, 2350);s.copy_ad(2405, 2351);s.store_sub(2406, 2323, 2343);s.store_scalar(2407, 1.0);s.store_scalar(2409, 1.0);s.store_scalar(2408, 0.0);s.copy_ad(2418, 2356);s.store_mul(2422, 2406, 2319);s.store_scalar(2419, 0.0);s.copy_ad(2420, 2357);s.store_scalar(2425, 0.0);s.store_scalar(2424, 1.0);s.copy_ad(2427, 2299);s.copy_ad(2426, 2422);}
        s.b[2481] = (s.v[2323] > 0.0);s.store_scalar(2481, if s.b[2481] { 1.0 } else { 0.0 });s.b[2482] = (s.v[2350] > 1e-100);s.store_scalar(2482, if s.b[2482] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {s.store_mul(2427, 2299, 2366);s.store_div(2368, 2427, 2363);s.store_add_scaled_inputs(2369, 2355, 1.0, 2305, 0.5);s.store_div_scaled_product_by_product_indices(1929, 2305, 2348, 1.0, 2369, 2369, 1.0);}
        s.b[2483] = (s.v[1929] > 0.0001);s.store_scalar(2483, if s.b[2483] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2483]) {s.store_sub_from_scalar(1930, 1.0, 1929);}
        s.b[2484] = (s.v[1930] < 1e-10);s.store_scalar(2484, if s.b[2484] { 1.0 } else { 0.0 });
        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2483]) && s.b[2484]) {s.store_scalar(1931, 1.0);}
        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2483]) && (!s.b[2484])) {s.store_sub_from_scalar_ad(1931, 1.0, A::sqrt(s.ad_value(1930)));}
        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && (!s.b[2483])) {s.store_scale(1931, 1929, 0.5);}
        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {s.store_mul(2370, 1931, 2369);}
        s.b[2485] = ((s.v[707] > 0.0) && (s.v[708] > 0.0));s.store_scalar(2485, if s.b[2485] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) {s.store_scaled_mul(2371, 2319, 2370, 0.475);s.store_add_scaled_product_indices(1929, 2356, 1.0, 2353, 2371, (-1.0));s.store_scaled_add_mixed_ia(2372, 1929, A::sqrt_square_offset(s.ad_value(1929), 1e-12), 0.5);s.store_add_scaled_value_products_mixed_iiiai(2373, 2356, (-1.0), 2319, 2355, 1.0, A::offset(s.ad_value(2353), (-1.0)), 2371, 1.0);s.store_offset_div_scaled_product_indices(2374, 2305, 2319, 0.5, 2373, 1.0, 1.0);s.store_add_scaled_product_indices(1929, 2373, 1.0, 764, 2372, 1.0);s.store_pow_ad(2375, A::mul3(s.ad_value(763), s.ad_value(1929), s.ad_value(705)), s.ad_value(706));s.store_mul_mixed_ai(1930, A::div_scaled_product_offset_rhs(s.ad_value(706), A::mul_sub_from_scalar_rhs(s.ad_value(2374), 1.0, s.ad_value(764)), (-1.0), 1.0, s.ad_value(1929), 1.0), 2375);s.store_div(1929, 2372, 2373);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_114(
        s: &mut Scratch,
    ) {
        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) {s.store_mul_pow_mixed_iaa(2376, 707, A::offset(s.ad_value(1929), 1.0), A::neg(s.ad_value(708)));s.store_mul_div_scaled_product_mixed_iiai(1931, 2376, 708, A::add(A::offset(s.ad_value(2374), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1929), 1.0, 1.0)), 1.0, 2373, 1.0);s.store_mul_product3_indices(2377, 2372, 746, 2358, 2359, 1.0);s.store_offset_ad(1929, A::div_scaled_add_product(s.ad_value(1930), 1.0, A::mul3(s.ad_value(746), s.ad_value(2358), s.ad_value(2359)), s.ad_value(2374), (-1.0), s.ad_value(1931), 1.0), 1.0);}
        s.b[2486] = (s.v[1929] < 230.25850929940458);s.store_scalar(2486, if s.b[2486] { 1.0 } else { 0.0 });
        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) && s.b[2486]) {s.store_scaled_ln_one_plus_exp_scaled_input(1930, 1929, 2.0, 0.5);}
        if ((((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) && (!s.b[2486])) {s.copy_ad(1930, 1929);}
        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2485]) {s.store_div_scaled_product3_mixed_iiia(2378, 2371, 1931, 1930, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2375), 1.0, s.ad_value(2376), 1.0, s.ad_value(2377), 1.0, 1.0), 1.0);s.store_mul_scale_offset_mixed_ia(2379, 2370, A::div_scaled_value_offset_denominator(s.ad_value(2378), 1.0, A::sqrt_square_offset(s.ad_value(2378), 1.0), 1.0, 1.0), 1.0, 1.0);}
        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && (!s.b[2485])) {s.copy_ad(2379, 2370);}
        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {s.store_mul3_affine_lhs(2380, 2319, 2368, 0.7071067811865475, 0.0, 2379);}
        s.b[2487] = (s.v[0] == (-1.0));s.store_scalar(2487, if s.b[2487] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2481]) && s.b[2482]) && s.b[2487]) {s.store_div_mixed_ia(2380, 2380, A::sqrt(A::offset(s.ad_value(2380), 1.0)));}
        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {s.store_div_from_scalar_offset_ad(2381, 2.0, A::sqrt(A::scale_offset(s.ad_value(2380), 4.0, 1.0)), 1.0);s.store_mul(1929, 2381, 2380);s.store_mul_ad_product_rhs_mixed_ia(2382, 2379, 2381, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1929), 1.0, A::mul(s.ad_value(1929), s.ad_value(2381)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1929), s.ad_value(1929), s.ad_value(2381), 4.0), 1.0)), 1.0));s.store_scale(2383, 2382, 0.99);s.store_div_scaled_product3_mixed_iaii(1929, 2383, A::sub_scaled_inputs(s.ad_value(2383), 1.0, s.ad_value(2369), 2.0), 2321, 1.0, 2350, 1.0);}
        if ((s.b[2453] && s.b[2481]) && s.b[2482]) {
            s.store_mul_sub_mixed_iia(2384, 2319, 2383, A::ln(A::offset({
                if (s.v[1929] > (-0.99)) {
                    s.ad_value(1929)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }
        if ((s.b[2453] && s.b[2481]) && (!s.b[2482])) {s.copy_ad(2384, 2367);}
        if (s.b[2453] && s.b[2481]) {s.store_offset(1929, 2300, 1.0);s.store_div_scaled_product_mixed_aii(1930, A::sqrt(s.ad_value(1929)), 815, 1.0, 2384, 1.0);s.store_add_mixed_ai(1931, A::square(s.ad_value(1930)), 1929);s.store_scale(1929, 1930, 2.0);s.store_div_scaled_product_add_scaled_denominator(2385, 2384, 1929, 1.0, A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), 1.0, A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929))), 1.0, 1.0);s.store_mul(2386, 2385, 2320);s.store_add(2387, 2329, 2386);}
        s.b[2488] = (s.v[2386] < 460.51701859880916);s.store_scalar(2488, if s.b[2488] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_115(
        s: &mut Scratch,
    ) {
        if ((s.b[2453] && s.b[2481]) && s.b[2488]) {s.store_exp_neg_input(2388, 2386);}
        if ((s.b[2453] && s.b[2481]) && (!s.b[2488])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2388, 1e-200, 2386, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2453] && s.b[2481]) {s.store_mul(2389, 2344, 2388);}
        s.b[2489] = (((s.v[2323]) as f64).abs() <= s.v[2341]);s.store_scalar(2489, if s.b[2489] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2481]) && s.b[2489]) {s.store_scaled_square(2429, 2342, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2390, 2323, 2342, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2323), 1.0, s.ad_value(2389)), s.ad_value(2304), s.ad_value(2429)), 1.0));}
        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {s.store_offset(2450, 2387, 3.0);s.store_sub_ad(2433, A::add_scaled_inputs3(s.ad_value(2449), 0.5, s.ad_value(2450), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2449), s.ad_value(2450)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2450), 0.5, A::sqrt_square_offset(s.ad_value(2450), 5.0), 0.5));s.store_sub(2428, 2323, 2433);s.store_exp_neg_input(2429, 2433);s.store_div_from_scalar_offset_square(2430, 1.0, 2433, 2.0);s.store_mul_square_lhs(2440, 2433, 2430);s.store_mul3_affine_lhs(2441, 2433, 2430, 4.0, 0.0, 2430);s.store_mul_ad_product_lhs_mixed_ai(2442, A::sub_scaled_inputs(s.ad_value(2430), 8.0, s.ad_value(2440), 12.0), 2430, 2430);}
        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {
            if (1e-40 > ((s.v[2428] * s.v[2428]) - (s.v[2305] * (((s.v[2429] + s.v[2433]) - 1.0) - (s.v[2389] * ((s.v[2433] + 1.0) + s.v[2440])))))) {
                s.store_scalar(2434, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2434, 2428, 1.0, 2305, A::add_scaled_product(A::offset(A::add(s.ad_value(2429), s.ad_value(2433)), (-1.0)), 1.0, s.ad_value(2389), A::add(A::offset(s.ad_value(2433), 1.0), s.ad_value(2440)), (-1.0)), (-1.0));
            }
        }
        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2451, 1.0, 2305, A::add_scaled_product(s.ad_value(2429), 1.0, s.ad_value(2389), s.ad_value(2442), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2435, 2428, 2.0, 2305, A::add_scaled_sub_value_product(1.0, s.ad_value(2429), 1.0, s.ad_value(2389), A::offset(s.ad_value(2441), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2436, 2387, 1.0, 2433, (-1.0), A::ln(A::div(s.ad_value(2434), s.ad_value(2305))), 1.0);s.store_add(813, 2434, 2435);s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2436, A::add_scaled_square_product(s.ad_value(2435), 0.5, s.ad_value(2434), s.ad_value(2451), (-1.0)), 1.0);s.store_add_mixed_ia(2452, 2433, A::div_scaled_product3(s.ad_value(2434), s.ad_value(813), s.ad_value(2436), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436), s.ad_value(2436)), s.ad_value(2435), A::add_scaled_square_product(s.ad_value(2435), 0.3333333333333333, s.ad_value(2434), s.ad_value(2451), (-1.0)))), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_116(
        s: &mut Scratch,
    ) {
        s.b[2490] = (s.v[2452] < 230.25850929940458);s.store_scalar(2490, if s.b[2490] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2481]) && (!s.b[2489])) && s.b[2490]) {s.store_exp(2438, 2452);s.store_div_from_scalar(2439, 1.0, 2438);s.store_mul(2438, 2389, 2438);}
        s.b[2491] = (s.v[2452] > (s.v[2387] - 230.25850929940458));s.store_scalar(2491, if s.b[2491] { 1.0 } else { 0.0 });
        if ((((s.b[2453] && s.b[2481]) && (!s.b[2489])) && (!s.b[2490])) && s.b[2491]) {s.store_exp_sub(2438, 2452, 2387);s.store_div(2439, 2389, 2438);}
        if ((((s.b[2453] && s.b[2481]) && (!s.b[2489])) && (!s.b[2490])) && (!s.b[2491])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2438, 1e-100, A::sub(s.ad_value(2387), s.ad_value(2452)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2439, 1e-100, 2452, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[2453] && s.b[2481]) && (!s.b[2489])) {s.store_div_from_scalar_offset_square(2428, 1.0, 2452, 2.0);s.store_mul_square_lhs(2440, 2452, 2428);s.store_mul3_affine_lhs(2441, 2452, 2428, 4.0, 0.0, 2428);s.store_mul_ad_product_lhs_mixed_ai(2442, A::sub_scaled_inputs(s.ad_value(2428), 8.0, s.ad_value(2440), 12.0), 2428, 2428);s.store_sub(2428, 2323, 2452);s.store_add_scaled_product_mixed_iia(2443, 2428, 2.0, 2305, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2439)), 1.0, s.ad_value(2438), 1.0, s.ad_value(2389), A::offset(s.ad_value(2441), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2444, 2428, 1.0, 2305, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2439), 1.0, s.ad_value(2452), 1.0, s.ad_value(2438), 1.0, (-1.0)), 1.0, s.ad_value(2389), A::add(A::offset(s.ad_value(2452), 1.0), s.ad_value(2440)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2428, 2.0, 2305, A::add_scaled_inputs_product(s.ad_value(2439), 1.0, s.ad_value(2438), 1.0, s.ad_value(2389), s.ad_value(2442), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2428, 2443, 1.0, 2444, 2428, (-2.0));s.store_add_scaled_inputs_mixed_ia(2390, 2452, 1.0, A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0);}
        if (s.b[2453] && s.b[2481]) {s.store_sub(2391, 2390, 2343);}
        s.b[2492] = (s.v[2391] < 1e-10);s.store_scalar(2492, if s.b[2492] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2481]) && s.b[2492]) {s.store_add_scaled_inputs_product_mixed_iiia(2392, 2323, 2.0, 2343, (-2.0), 2305, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2349), 1.0, s.ad_value(2348), s.ad_value(2388), 1.0), 1.0, s.ad_value(2389), s.ad_value(2346), 1.0, (-1.0)), 1.0);s.store_mul_mixed_ai(2393, A::mul_sub_from_scalar_rhs(s.ad_value(2305), 1.0, s.ad_value(2388)), 2350);s.store_sub_from_scalar_scaled_mul_mixed_ia(1929, 2.0, 2305, A::add_scaled_value_products(s.ad_value(2349), 1.0, s.ad_value(2348), s.ad_value(2388), 1.0, s.ad_value(2389), s.ad_value(2347), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(1929, 2392, 1.0, 1929, 2393, (-2.0));s.store_scaled_div_mixed_ia(2391, 2393, A::add(s.ad_value(2392), A::sqrt(s.ad_value(1929))), 2.0);s.store_add(2390, 2343, 2391);}
        if (s.b[2453] && s.b[2481]) {s.store_mul(2394, 2391, 2319);s.store_div_scaled_product_offset_denominator_mixed_iia(2395, 2390, 2390, 1.0, A::square(s.ad_value(2390)), 2.0, 1.0);}
        s.b[2493] = (s.v[2390] < 230.25850929940458);s.store_scalar(2493, if s.b[2493] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2481]) && s.b[2493]) {s.store_exp_neg_input(2396, 2390);}
        s.b[2494] = (s.v[2390] < 1e-5);s.store_scalar(2494, if s.b[2494] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2481]) && s.b[2493]) && s.b[2494]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2397, 2390, 1.0, 2390, 1.0, 2390, 0.25, 0.3333333333333333, 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_117(
        s: &mut Scratch,
    ) {
        if (((s.b[2453] && s.b[2481]) && s.b[2493]) && s.b[2494]) {s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2390), 1.0, A::scale(s.ad_value(2390), 0.25), 0.3333333333333333));s.store_scaled_mul(2398, 2390, 1929, 0.7071067811865475);s.store_mul3_ad_middle(2399, A::mul3_scaled_output(s.ad_value(2389), s.ad_value(2390), s.ad_value(2390), 0.16666666666666666), 2390, A::scale_offset(s.ad_value(2390), 1.75, 1.0));}
        if (((s.b[2453] && s.b[2481]) && s.b[2493]) && (!s.b[2494])) {s.store_add_offset_lhs(2397, 2390, (-1.0), 2396);s.store_sqrt(2398, 2397);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(2399, 2389, A::div_from_scalar(1.0, s.ad_value(2396)), 1.0, 2390, (-1.0), 2395, -1.0, (-1.0));}
        s.b[2495] = (s.v[2390] > (s.v[2387] - 230.25850929940458));s.store_scalar(2495, if s.b[2495] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2481]) && (!s.b[2493])) && s.b[2495]) {s.store_exp_sub(1929, 2390, 2387);s.store_div(2396, 2389, 1929);s.store_add_scaled_product_mixed_iia(2399, 1929, 1.0, 2389, A::add(A::offset(s.ad_value(2390), 1.0), s.ad_value(2395)), (-1.0));}
        if (((s.b[2453] && s.b[2481]) && (!s.b[2493])) && (!s.b[2495])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2396, 1e-100, 2390, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(1929, 1e-100, A::sub(s.ad_value(2387), s.ad_value(2390)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(2399, 1929, 1.0, 2389, A::add(A::offset(s.ad_value(2390), 1.0), s.ad_value(2395)), (-1.0));}
        if ((s.b[2453] && s.b[2481]) && (!s.b[2493])) {s.store_add_offset_lhs(2397, 2390, (-1.0), 2396);s.store_sqrt(2398, 2397);}
        if (s.b[2453] && s.b[2481]) {s.store_mul3_lhs(2400, 2398, 2304, 2319);s.store_scaled_add(2401, 2343, 2390, 0.5);s.store_scalar(2402, 0.0);s.store_mul(1929, 2396, 2349);}
        s.b[2496] = (s.v[1929] > 0.0);s.store_scalar(2496, if s.b[2496] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2481]) && s.b[2496]) {s.store_sqrt(2402, 1929);}
        if (s.b[2453] && s.b[2481]) {s.store_scaled_add(2403, 2350, 2399, 0.5);s.store_add_scaled_product_mixed_iaa(2404, 2403, 1.0, A::square(s.ad_value(2391)), A::sub_scaled_inputs(s.ad_value(2402), 1.0, s.ad_value(2321), 2.0), 0.125);}
        s.b[2497] = (s.v[2401] < 1e-5);s.store_scalar(2497, if s.b[2497] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2481]) && s.b[2497]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2405, 2401, 1.0, 2401, 1.0, 2401, 0.25, 0.3333333333333333, 0.5);s.store_mul_sqrt_mixed_ia(2406, 2304, A::add(s.ad_value(2404), s.ad_value(2405)));}
        s.b[2498] = (s.v[719] > 0.0);s.store_scalar(2498, if s.b[2498] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2481]) && s.b[2497]) && s.b[2498]) {s.store_div_from_scalar_sqrt_ad(2407, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2406)), 1.0));}
        if ((s.b[2453] && s.b[2481]) && s.b[2497]) {s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2401), 1.0, A::scale(s.ad_value(2401), 0.25), 0.3333333333333333));s.store_scaled_mul(2408, 2401, 1929, 0.7071067811865475);s.store_add_mixed_ia(2409, 2407, A::div_scaled_product(s.ad_value(2304), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2401), 0.5)), 1.0, A::square(s.ad_value(2401)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1929), 1.0));}
        if ((s.b[2453] && s.b[2481]) && (!s.b[2497])) {s.store_add_offset_lhs(2405, 2401, (-1.0), 2402);s.store_mul_sqrt_mixed_ia(2406, 2304, A::add(s.ad_value(2404), s.ad_value(2405)));}
        s.b[2499] = (s.v[719] > 0.0);s.store_scalar(2499, if s.b[2499] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2481]) && (!s.b[2497])) && s.b[2499]) {s.store_add_scaled_sub_value_product_indices(2410, 1.0, 2402, 1.0, 2406, 2321, 2.0);s.store_div_from_scalar_sqrt_ad(2407, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2406)), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_118(
        s: &mut Scratch,
    ) {
        if (((s.b[2453] && s.b[2481]) && (!s.b[2497])) && s.b[2499]) {s.store_div_scaled_value_offset_denominator(1929, s.ad_value(2407), 1.0, s.ad_value(2407), 1.0, 1.0);s.store_mul_product3_mixed_iaii(2411, 719, A::square(s.ad_value(1929)), 2305, 2404, 1.0);s.store_add_scaled_inputs_product_mixed_iiia(2412, 2406, 2.0, 2411, (-2.0), 2305, A::add(A::sub_from_scalar(1.0, s.ad_value(2402)), s.ad_value(2404)), 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(2413, 2411, 2411, 1.0, 2406, 2.0);s.store_sub_from_scalar_scaled_mul_mixed_ia(2414, 1.0, 2305, A::add(s.ad_value(2402), s.ad_value(2404)), 0.5);s.store_div_scaled_product_mixed_iia(2415, 2413, 2412, 1.0, A::add_scaled_square_product(s.ad_value(2412), 1.0, s.ad_value(2414), s.ad_value(2413), (-1.0)), 1.0);s.store_add(2401, 2401, 2415);s.store_exp(2416, 2415);s.store_div(2402, 2402, 2416);s.store_mul(2404, 2404, 2416);s.store_add_offset_lhs(2405, 2401, (-1.0), 2402);s.store_mul_sqrt_mixed_ia(2406, 2304, A::add(s.ad_value(2404), s.ad_value(2405)));s.store_add_ad(2417, A::sub_from_scalar(1.0, s.ad_value(2402)), A::mul3_scaled_output(s.ad_value(2406), s.ad_value(2407), s.ad_value(2321), 2.0));s.store_div_scaled_product3_mixed_iiaa(2391, 2391, 2416, A::add(s.ad_value(2410), s.ad_value(2403)), 1.0, A::add_scaled_product(s.ad_value(2417), 1.0, s.ad_value(2416), s.ad_value(2403), 1.0), 1.0);s.store_mul(2394, 2391, 2319);}
        if ((s.b[2453] && s.b[2481]) && (!s.b[2497])) {s.store_sqrt(2408, 2405);s.store_add_scaled_inputs_mixed_ia(2409, 2407, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2304), 1.0, s.ad_value(2402)), s.ad_value(2408)), 0.5);}
        if (s.b[2453] && s.b[2481]) {s.store_mul_div_scaled_product_mixed_iiia(2418, 2319, 2305, 2404, 1.0, A::add_scaled_product(s.ad_value(2406), 1.0, s.ad_value(2304), s.ad_value(2408), 1.0), 1.0);s.store_add_scaled_product_indices(2419, 2418, 1.0, 2319, 2409, 1.0);s.store_mul3_lhs(2420, 2408, 2304, 2319);}
        s.b[2500] = (s.v[216] < 0.0);s.store_scalar(2500, if s.b[2500] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2481]) && s.b[2500]) {s.store_sub_from_scalar_scaled_mul(2359, 1.0, 216, 2418, 1.0);}
        if ((s.b[2453] && s.b[2481]) && (!s.b[2500])) {s.store_div_from_scalar_offset_product(2359, 1.0, 216, 2418, 1.0);}
        if (s.b[2453] && s.b[2481]) {s.store_mul_product3_indices(2360, 2418, 746, 2358, 2359, 1.0);s.store_add_scaled_product_indices(2421, 2420, 1.0, 764, 2418, 1.0);s.store_add_scaled_product_indices(2422, 2420, 1.0, 765, 2418, 1.0);s.store_mul(2423, 763, 2421);s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2405), 1.0, A::add(s.ad_value(2405), s.ad_value(2404)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2362, A::pow(A::mul(s.ad_value(2423), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);s.store_mul_add_mixed_iai(2424, 2354, A::offset(s.ad_value(2362), 1.0), 2360);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_119(
        s: &mut Scratch,
    ) {
        if (s.b[2453] && s.b[2481]) {s.store_ln_ad(2425, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(815), s.ad_value(2394)), s.ad_value(768)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2385), s.ad_value(2394)), s.ad_value(768)), 1.0), 1.0));s.store_mul(1931, 2418, 2364);s.store_div_add_scaled_inputs_rhs_indices(2365, 1931, 221, 1.0, 1931, 1.0);}
        s.b[2501] = (s.v[220] < 0.0);s.store_scalar(2501, if s.b[2501] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2481]) && s.b[2501]) {s.store_div_from_scalar_sub_from_scalar_ad(2366, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2365)));}
        if ((s.b[2453] && s.b[2481]) && (!s.b[2501])) {s.store_offset_mul(2366, 220, 2365, 1.0);}
        if (s.b[2453] && s.b[2481]) {s.store_mul(2427, 2299, 2366);s.store_mul(2426, 2406, 2319);}
        if s.b[2453] {s.copy_ad(1871, 2301);s.copy_ad(1872, 2319);s.copy_ad(1873, 2304);}
        let (t1,) = {
    if s.b[2453] {
        (s.v[2323],)
    } else {
        (s.v[1874],)
    }
};
        s.store_scalar(1874, t1);
        if s.b[2453] {s.copy_ad(1875, 2328);s.copy_ad(1876, 2357);s.copy_ad(1877, 2394);s.copy_ad(1878, 2400);s.copy_ad(1879, 2407);s.copy_ad(1880, 2409);s.copy_ad(1881, 2418);s.copy_ad(1882, 2419);s.copy_ad(1883, 2422);s.copy_ad(1884, 2424);s.copy_ad(1885, 2425);s.copy_ad(1886, 2427);s.copy_ad(1887, 2426);}
        if (!s.b[2453]) {s.copy_ad(734, 717);s.copy_ad(1871, 1806);s.copy_ad(1872, 1808);s.copy_ad(1873, 1810);}
        let (t2,) = {
    if (!s.b[2453]) {
        (s.v[1813],)
    } else {
        (s.v[1874],)
    }
};
        s.store_scalar(1874, t2);
        if (!s.b[2453]) {s.copy_ad(1875, 1814);s.copy_ad(1876, 1833);s.copy_ad(1877, 1844);s.copy_ad(1878, 1845);s.copy_ad(1879, 1847);s.copy_ad(1880, 1848);s.copy_ad(1881, 1849);s.copy_ad(1882, 1850);s.copy_ad(1883, 1852);s.copy_ad(1884, 1853);s.copy_ad(1885, 1855);s.copy_ad(1886, 1854);s.copy_ad(1887, 1856);}
        s.copy_ad(1888, 253);s.b[2502] = (s.v[762] > 0.0);s.store_scalar(2502, if s.b[2502] { 1.0 } else { 0.0 });
        if s.b[2502] {s.store_div_scaled_value_offset_denominator(1888, s.ad_value(253), 1.0, A::mul(s.ad_value(762), A::powf(A::add(A::square(s.ad_value(1883)), s.ad_value(722)), ((-1.0) * 0.16666666666666666))), 1.0, 1.0);}
        s.store_scalar(1889, 1.0);s.store_scalar(1890, 1.0);s.store_scalar(1891, 0.0);s.store_scalar(1892, 1.0);s.store_scalar(1893, 1.0);s.copy_ad(2265, 1887);s.store_scalar(2268, 0.0);s.store_scalar(2267, 0.0);s.copy_ad(2269, 2265);s.b[2503] = (s.v[1874] > 0.0);s.store_scalar(2503, if s.b[2503] { 1.0 } else { 0.0 });
        if s.b[2503] {s.store_mul_div_scaled_product_mixed_iaii(2260, 1885, A::add(s.ad_value(258), A::div(s.ad_value(259), s.ad_value(1882))), 1881, 1.0, 1882, 1.0);}
        s.b[2504] = (s.v[2260] > 0.0);s.store_scalar(2504, if s.b[2504] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_120(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2503] && s.b[2504]) {s.store_div_from_scalar_add_ad(1889, 1.0, A::offset(s.ad_value(2260), 1.0), A::square(s.ad_value(2260)));}
        if (s.b[2503] && (!s.b[2504])) {s.store_sub_from_scalar(1889, 1.0, 2260);}
        if s.b[2503] {s.store_mul(1890, 1884, 1889);s.store_div(1891, 1886, 1890);s.store_mul_ad_product_lhs_mixed_ai(2261, A::square(s.ad_value(1891)), 1877, 1877);}
        s.b[2505] = (s.v[0] == (-1.0));s.store_scalar(2505, if s.b[2505] { 1.0 } else { 0.0 });
        if (s.b[2503] && s.b[2505]) {s.store_div_scaled_value_offset_denominator(2261, s.ad_value(2261), 1.0, A::mul(s.ad_value(1891), s.ad_value(1877)), 1.0, 1.0);}
        if s.b[2503] {s.store_mul_scale_offset_mixed_ia(1892, 1890, A::sqrt(A::scale_offset(s.ad_value(2261), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div(1929, 1890, 1892);s.store_mul_scale_offset_mixed_ia(2262, 1880, A::mul3_scaled_output(s.ad_value(2261), s.ad_value(1929), s.ad_value(1929), 0.5), 1.0, 1.0);s.store_div_scaled_product_indices(1893, 1929, 1882, 1.0, 2262, 1.0);s.store_scaled_div(2263, 1877, 1893, 0.5);s.store_square(2264, 2263);s.store_add_product3_rhs_mixed_iia(2265, 1887, 1879, 1877, A::add(A::offset(A::mul_scaled_output(s.ad_value(2263), s.ad_value(1889), 0.3333333333333333), (-1.0)), s.ad_value(1889)), 0.5);s.store_scaled_mul(1929, 1880, 1877, 0.16666666666666666);}
        s.b[2506] = (p.p49 == 1.0);s.store_scalar(2506, if s.b[2506] { 1.0 } else { 0.0 });
        if (s.b[2503] && s.b[2506]) {s.store_scalar(2266, 0.0);s.store_mul_ad_affine_product_rhs(2267, 1889, s.ad_value(1889), A::sub(s.ad_value(1881), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1929), 2.0, s.ad_value(2263), 3.0)), 0.5, 0.0);}
        if (s.b[2503] && (!s.b[2506])) {s.store_mul_scale_offset_mixed_ai(2266, A::add_scaled_product(s.ad_value(1881), 1.0, s.ad_value(1880), s.ad_value(1877), (-0.5)), 1889, -1.0, 1.0);s.store_add_scaled_products_mixed_aaia(2267, A::square(s.ad_value(1889)), A::add_scaled_product(s.ad_value(1881), 1.0, s.ad_value(1929), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2263)), 1.0, s.ad_value(2264), 0.2), (-1.0)), 0.5, 2266, A::offset(s.ad_value(1889), 1.0), 0.5);}
        if s.b[2503] {s.store_add_scaled_product_mixed_iia(2268, 2266, 1.0, 1889, A::add_scaled_product(s.ad_value(1881), 1.0, s.ad_value(1929), s.ad_value(2263), 1.0), 1.0);s.store_sub(2269, 2265, 2268);}
        s.store_mul(840, 2265, 1888);s.store_mul_scale_offset_indices(842, 1888, 2267, -1.0, 0.0);s.store_mul_scale_offset_indices(841, 1888, 2269, -1.0, 0.0);s.store_scalar(2285, 0.0);s.store_scalar(2286, 0.0);s.store_scalar(2284, 0.0);s.b[2507] = ((s.v[266] > 0.0) || (s.v[267] > 0.0));s.store_scalar(2507, if s.b[2507] { 1.0 } else { 0.0 });
        if s.b[2507] {s.store_scalar(2274, 1.0);s.copy_ad(2273, 1871);}
        s.b[2508] = (s.v[270] > 1e-10);s.store_scalar(2508, if s.b[2508] { 1.0 } else { 0.0 });
        if (s.b[2507] && s.b[2508]) {s.store_add_scaled_inputs3_indices(2270, 1871, 1.0, 268, (-1.0), 797, 1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1929, 2270, 0.5, 797, 0.5, A::add(A::square(A::sub(s.ad_value(2270), s.ad_value(797))), s.ad_value(798)), 0.5);s.store_mul_add_scaled_inputs3_offset_rhs_indices(1930, 1929, 1929, 2.0, 797, (-1.0), 2270, -1.0, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_121(
        s: &mut Scratch,
    ) {
        if (s.b[2507] && s.b[2508]) {s.store_div(1931, 797, 1929);s.store_mul(2271, 2270, 1931);s.store_sqrt_sub_from_scalar_ad(2272, 1.0, A::mul(s.ad_value(2271), s.ad_value(270)));s.store_add_scaled_inputs3_mixed_aii(2273, A::div(A::sub_from_scalar(1.0, s.ad_value(2272)), s.ad_value(270)), 1.0, 2270, 1.0, 2271, -1.0);s.store_offset_ad(2274, A::div_scaled_product3(A::offset(A::div_from_scalar(0.5, s.ad_value(2272)), (-1.0)), A::add_scaled_product(s.ad_value(1930), 1.0, s.ad_value(2270), A::sub(s.ad_value(797), s.ad_value(1929)), 1.0), s.ad_value(1931), 1.0, s.ad_value(1930), 1.0), 1.0);}
        if s.b[2507] {s.store_scalar(2276, 1.0);s.store_scalar(2277, 0.0);}
        s.b[2509] = (s.v[269] > 0.0);s.store_scalar(2509, if s.b[2509] { 1.0 } else { 0.0 });
        if (s.b[2507] && s.b[2509]) {s.store_add_scaled_product_mixed_iia(1929, 734, 0.5, 1872, A::scale_offset(s.ad_value(1873), 0.7071067811865475, 1.0), 1.0);s.store_div(2275, 1871, 1929);}
        s.b[2510] = (((s.v[2275]) as f64).abs() < 230.25850929940458);s.store_scalar(2510, if s.b[2510] { 1.0 } else { 0.0 });
        if ((s.b[2507] && s.b[2509]) && s.b[2510]) {s.store_div_from_scalar_offset_ad(2276, 1.0, A::exp_scaled_input(s.ad_value(2275), -1.0), 1.0);}
        s.b[2511] = (s.v[2275] < 0.0);s.store_scalar(2511, if s.b[2511] { 1.0 } else { 0.0 });
        if (((s.b[2507] && s.b[2509]) && (!s.b[2510])) && s.b[2511]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2276, 1e-100, 2275, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[2512] = (s.v[2275] < 230.25850929940458);s.store_scalar(2512, if s.b[2512] { 1.0 } else { 0.0 });
        if ((s.b[2507] && s.b[2509]) && s.b[2512]) {s.store_ln_one_plus_exp(1930, 2275);}
        if ((s.b[2507] && s.b[2509]) && (!s.b[2512])) {s.copy_ad(1930, 2275);}
        if (s.b[2507] && s.b[2509]) {s.store_mul(2277, 1929, 1930);}
        if s.b[2507] {s.store_add_scaled_product_right_sub(2278, 2274, 1.0, 269, 2276, 2274, 1.0);s.store_add_scaled_product_right_sub(2279, 2273, 1.0, 269, 2277, 2273, 1.0);s.store_add_scaled_inputs3_mixed_aii(2280, A::add_scaled_product(s.ad_value(1871), 1.0, s.ad_value(1872), s.ad_value(1875), (-1.0)), 1.0, 1887, (-1.0), 1877, (-0.5));s.store_add_scaled_inputs3_indices(2281, 1871, 1.0, 2280, (-1.0), 1876, -1.0);s.store_add_scaled_inputs3_indices(2282, 1877, 1.0, 2280, 1.0, 815, -1.0);s.store_add_scaled_inputs3_indices(2283, 1871, 1.0, 2282, (-1.0), 1878, -1.0);}
        s.b[2513] = (s.v[820] > 0.0);s.store_scalar(2513, if s.b[2513] { 1.0 } else { 0.0 });
        if (s.b[2507] && s.b[2513]) {s.store_mul_mixed_ia(2284, 2278, A::add_scaled_products(s.ad_value(267), s.ad_value(2282), 1.0, s.ad_value(266), s.ad_value(2280), 1.0));s.store_mul_sub_rhs(2285, 266, 2281, 2279);s.store_mul_sub_rhs(2286, 267, 2283, 2279);}
        if (s.b[2507] && (!s.b[2513])) {s.store_mul_mixed_ia(2284, 2278, A::add_scaled_products(s.ad_value(266), s.ad_value(2282), 1.0, s.ad_value(267), s.ad_value(2280), 1.0));s.store_mul_sub_rhs(2285, 267, 2281, 2279);s.store_mul_sub_rhs(2286, 266, 2283, 2279);}
        if s.b[2507] {s.store_add(840, 840, 2284);s.store_add(842, 842, 2286);s.store_add_scaled_inputs4_indices(841, 841, 1.0, 2284, (-1.0), 2286, -1.0, 2285, -1.0);}
        s.store_mul(1894, 260, 1862);s.store_mul(1895, 261, 1863);s.store_scalar(2289, 0.0);s.store_scalar(2287, 0.0);s.b[2514] = ((s.v[260] > 0.0) && (s.v[262] > 0.0));s.store_scalar(2514, if s.b[2514] { 1.0 } else { 0.0 });
        if s.b[2514] {s.store_mul_add_scaled_inputs_rhs_indices(1929, 264, 1803, 0.5, 776, 1.0);}
        s.b[2515] = (s.v[1929] < 230.25850929940458);s.store_scalar(2515, if s.b[2515] { 1.0 } else { 0.0 });s.b[2516] = (s.v[1929] > (-230.25850929940458));s.store_scalar(2516, if s.b[2516] { 1.0 } else { 0.0 });
        if ((s.b[2514] && s.b[2515]) && s.b[2516]) {s.store_exp(2287, 1929);}
        if ((s.b[2514] && s.b[2515]) && (!s.b[2516])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2287, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2517] = (s.v[2287] > 1e-10);s.store_scalar(2517, if s.b[2517] { 1.0 } else { 0.0 });
        if ((s.b[2514] && s.b[2515]) && s.b[2517]) {s.store_ln_offset_input(2288, 2287, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_122(
        s: &mut Scratch,
    ) {
        if ((s.b[2514] && s.b[2515]) && s.b[2517]) {s.store_mul_scale_offset_mixed_ia(1930, 2288, A::div(A::ln(A::offset(s.ad_value(2288), 1.0)), A::offset(s.ad_value(2288), 2.0)), -1.0, 1.0);}
        if ((s.b[2514] && s.b[2515]) && (!s.b[2517])) {s.copy_ad(2288, 2287);s.store_div_scaled_value_offset_denominator(1930, s.ad_value(2288), 2.0, s.ad_value(2288), 2.0, 1.0);}
        if (s.b[2514] && (!s.b[2515])) {s.copy_ad(2288, 1929);s.store_mul_scale_offset_mixed_ia(1930, 2288, A::div(A::ln(A::offset(s.ad_value(2288), 1.0)), A::offset(s.ad_value(2288), 2.0)), -1.0, 1.0);}
        if s.b[2514] {s.store_mul_ad_affine_product_lhs(2289, A::div_scaled_inputs(s.ad_value(262), (-2.0), s.ad_value(264), 1.0), s.ad_value(260), s.v[355], 0.0, 1930);}
        s.store_scalar(2292, 0.0);s.store_scalar(2290, 0.0);s.b[2518] = ((s.v[261] > 0.0) && (s.v[263] > 0.0));s.store_scalar(2518, if s.b[2518] { 1.0 } else { 0.0 });
        if s.b[2518] {s.store_mul_add_scaled_inputs_rhs_indices(1929, 264, 1803, 0.5, 777, 1.0);}
        s.b[2519] = (s.v[1929] < 230.25850929940458);s.store_scalar(2519, if s.b[2519] { 1.0 } else { 0.0 });s.b[2520] = (s.v[1929] > (-230.25850929940458));s.store_scalar(2520, if s.b[2520] { 1.0 } else { 0.0 });
        if ((s.b[2518] && s.b[2519]) && s.b[2520]) {s.store_exp(2290, 1929);}
        if ((s.b[2518] && s.b[2519]) && (!s.b[2520])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2290, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2521] = (s.v[2290] > 1e-10);s.store_scalar(2521, if s.b[2521] { 1.0 } else { 0.0 });
        if ((s.b[2518] && s.b[2519]) && s.b[2521]) {s.store_ln_offset_input(2291, 2290, 1.0);s.store_mul_scale_offset_mixed_ia(1930, 2291, A::div(A::ln(A::offset(s.ad_value(2291), 1.0)), A::offset(s.ad_value(2291), 2.0)), -1.0, 1.0);}
        if ((s.b[2518] && s.b[2519]) && (!s.b[2521])) {s.copy_ad(2291, 2290);s.store_div_scaled_value_offset_denominator(1930, s.ad_value(2291), 2.0, s.ad_value(2291), 2.0, 1.0);}
        if (s.b[2518] && (!s.b[2519])) {s.copy_ad(2291, 1929);s.store_mul_scale_offset_mixed_ia(1930, 2291, A::div(A::ln(A::offset(s.ad_value(2291), 1.0)), A::offset(s.ad_value(2291), 2.0)), -1.0, 1.0);}
        if s.b[2518] {s.store_mul_ad_affine_product_lhs(2292, A::div_scaled_inputs(s.ad_value(263), (-2.0), s.ad_value(264), 1.0), s.ad_value(261), s.v[355], 0.0, 1930);}
        s.store_add(2293, 2289, 2292);s.store_add_scaled_product_indices(845, 2293, 1.0, 265, 818, 1.0);s.store_mul(843, 272, 823);s.store_mul(844, 273, 826);s.store_scalar(2522, 0.0);s.store_scalar(2523, 0.0);s.store_scalar(2524, 0.0);s.store_scalar(2525, 0.0);s.store_scalar(2526, 0.0);s.store_scalar(2527, 0.0);s.store_scalar(2528, 0.0);s.store_scalar(2529, 0.0);s.store_scalar(2530, 0.0);s.store_scalar(2531, 0.0);s.store_scalar(2532, 0.0);s.store_scalar(2533, 0.0);s.store_scalar(2534, 0.0);s.store_scalar(2535, 0.0);s.store_scalar(2536, 0.0);s.store_scalar(2537, 0.0);s.store_scalar(2538, 0.0);s.store_scalar(2539, 0.0);s.store_scalar(2540, 0.0);s.store_scalar(2541, 0.0);s.store_scalar(2542, 0.0);s.store_scalar(2543, 0.0);s.store_scalar(2544, 0.0);s.store_scalar(2545, 0.0);s.store_scalar(2546, 0.0);s.store_scalar(2547, 0.0);s.store_scalar(2548, 0.0);s.store_scalar(2549, 0.0);s.store_scalar(2550, 0.0);s.store_scalar(2551, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_123(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(2552, 0.0);s.store_scalar(2553, 0.0);s.store_scalar(2554, 0.0);s.store_scalar(2555, 0.0);s.store_scalar(2556, 0.0);s.store_scalar(2557, 0.0);s.store_scalar(2558, 0.0);s.store_scalar(2559, 0.0);s.store_scalar(2560, 0.0);s.store_scalar(2561, 0.0);s.store_scalar(2562, 0.0);s.store_scalar(2563, 0.0);s.store_scalar(2564, 0.0);s.store_scalar(2565, 0.0);s.store_scalar(2566, 0.0);s.store_scalar(2567, 0.0);s.store_scalar(2568, 0.0);s.store_scalar(837, 0.0);s.store_scalar(1896, 0.0);s.store_scalar(1897, 0.0);s.store_scalar(1898, 0.0);s.store_scalar(838, 0.0);s.store_scalar(1899, 0.0);s.store_scalar(1900, 0.0);s.store_scalar(1901, 0.0);s.store_scalar(846, 0.0);s.store_scalar(1902, 0.0);s.store_scalar(1903, 0.0);s.store_scalar(1904, 0.0);s.store_scalar(847, 0.0);s.store_scalar(1905, 0.0);s.store_scalar(1906, 0.0);s.store_scalar(1907, 0.0);s.b[2569] = (p.p43 > 0.0);s.store_scalar(2569, if s.b[2569] { 1.0 } else { 0.0 });s.b[2570] = (s.v[475] == 1.0);s.store_scalar(2570, if s.b[2570] { 1.0 } else { 0.0 });
        if (s.b[2569] && s.b[2570]) {s.store_scale(497, 821, (s.v[372] * s.v[669]));}
        if (s.b[2569] && s.b[2570]) {
            if (s.v[497] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(498, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0);
            } else {
                if (s.v[497] > s.v[661]) {
                    s.store_mul_scale_offset_mixed_ia(498, 662, A::sub(s.ad_value(497), s.ad_value(661)), 1.0, 1.0);
                } else {
                    s.store_exp(498, 497);
                }
            }
        }
        if (s.b[2569] && s.b[2570]) {s.store_mul_scale_offset_indices(503, 668, 498, 1.0, (-1.0));s.store_scaled_mul(497, 821, 671, s.v[372]);}
        if (s.b[2569] && s.b[2570]) {
            if (s.v[497] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(498, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0);
            } else {
                if (s.v[497] > s.v[663]) {
                    s.store_mul_scale_offset_mixed_ia(498, 664, A::sub(s.ad_value(497), s.ad_value(663)), 1.0, 1.0);
                } else {
                    s.store_exp(498, 497);
                }
            }
        }
        if (s.b[2569] && s.b[2570]) {s.store_mul_scale_offset_indices(504, 670, 498, 1.0, (-1.0));s.store_scalar(505, 0.0);}
        s.b[2571] = (s.v[667] > 0.0);s.store_scalar(2571, if s.b[2571] { 1.0 } else { 0.0 });
        if ((s.b[2569] && s.b[2570]) && s.b[2571]) {s.store_mul_add_scaled_product_rhs_indices(505, 821, 672, 1.0, 821, 673, 1.0);}
        if ((s.b[2569] && s.b[2570]) && (!s.b[2571])) {s.store_scaled_mul(497, 821, 673, (-s.v[372]));}
        if ((s.b[2569] && s.b[2570]) && (!s.b[2571])) {
            if (s.v[497] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(498, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0);
            } else {
                if (s.v[497] > s.v[665]) {
                    s.store_mul_scale_offset_mixed_ia(498, 666, A::sub(s.ad_value(497), s.ad_value(665)), 1.0, 1.0);
                } else {
                    s.store_exp(498, 497);
                }
            }
        }
        if ((s.b[2569] && s.b[2570]) && (!s.b[2571])) {s.store_mul_scaled_offset_rhs(505, 672, -1.0, 498, (-1.0));}
        if (s.b[2569] && s.b[2570]) {s.store_add_scaled_inputs3_indices(837, 503, 1.0, 504, 1.0, 505, 1.0);s.store_scale(497, 822, (s.v[372] * s.v[696]));}
        if (s.b[2569] && s.b[2570]) {
            if (s.v[497] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(498, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0);
            } else {
                if (s.v[497] > s.v[688]) {
                    s.store_mul_scale_offset_mixed_ia(498, 689, A::sub(s.ad_value(497), s.ad_value(688)), 1.0, 1.0);
                } else {
                    s.store_exp(498, 497);
                }
            }
        }
        if (s.b[2569] && s.b[2570]) {s.store_mul_scale_offset_indices(503, 695, 498, 1.0, (-1.0));s.store_scaled_mul(497, 822, 698, s.v[372]);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_124(
        s: &mut Scratch,
    ) {
        if (s.b[2569] && s.b[2570]) {
            if (s.v[497] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(498, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0);
            } else {
                if (s.v[497] > s.v[690]) {
                    s.store_mul_scale_offset_mixed_ia(498, 691, A::sub(s.ad_value(497), s.ad_value(690)), 1.0, 1.0);
                } else {
                    s.store_exp(498, 497);
                }
            }
        }
        if (s.b[2569] && s.b[2570]) {s.store_mul_scale_offset_indices(504, 697, 498, 1.0, (-1.0));s.store_scalar(505, 0.0);}
        s.b[2572] = (s.v[694] > 0.0);s.store_scalar(2572, if s.b[2572] { 1.0 } else { 0.0 });
        if ((s.b[2569] && s.b[2570]) && s.b[2572]) {s.store_mul_add_scaled_product_rhs_indices(505, 822, 699, 1.0, 822, 700, 1.0);}
        if ((s.b[2569] && s.b[2570]) && (!s.b[2572])) {s.store_scaled_mul(497, 822, 700, (-s.v[372]));}
        if ((s.b[2569] && s.b[2570]) && (!s.b[2572])) {
            if (s.v[497] < (-230.25850929940458)) {
                s.store_div_from_scalar_offset_ad(498, 1e-100, A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0);
            } else {
                if (s.v[497] > s.v[692]) {
                    s.store_mul_scale_offset_mixed_ia(498, 693, A::sub(s.ad_value(497), s.ad_value(692)), 1.0, 1.0);
                } else {
                    s.store_exp(498, 497);
                }
            }
        }
        if ((s.b[2569] && s.b[2570]) && (!s.b[2572])) {s.store_mul_scaled_offset_rhs(505, 699, -1.0, 498, (-1.0));}
        if (s.b[2569] && s.b[2570]) {s.store_add_scaled_inputs3_indices(838, 503, 1.0, 504, 1.0, 505, 1.0);s.store_scalar(2573, 0.0);s.store_scalar(2574, 0.0);s.store_primal_scaled_mul(2525, 658, 658, 4.0);s.store_primal_div(2526, 658, 659);s.store_add_scaled_product_indices(2527, 821, 1.0, 658, 2526, 1.0);s.store_add(2528, 659, 2527);s.store_sub(2529, 659, 2527);s.store_sqrt_square_add(2530, 2529, 2525);s.store_div_scaled_product_add_scaled_denominator_indices(2574, 821, 659, 2.0, 2528, 1.0, 2530, 1.0, 1.0);}
        s.b[2575] = (s.v[652] > 0.5);s.store_scalar(2575, if s.b[2575] { 1.0 } else { 0.0 });s.b[2576] = (s.v[409] == 0.5);s.store_scalar(2576, if s.b[2576] { 1.0 } else { 0.0 });
        if (((s.b[2569] && s.b[2570]) && s.b[2575]) && s.b[2576]) {s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::scale(s.ad_value(2574), s.v[406]));}
        if (((s.b[2569] && s.b[2570]) && s.b[2575]) && (!s.b[2576])) {s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[406])), s.v[409]);}
        if ((s.b[2569] && s.b[2570]) && s.b[2575]) {s.store_add_scaled_inputs3_offset_indices(1902, 2573, (-s.v[418]), 821, s.v[421], 2574, (-s.v[421]), s.v[418]);}
        s.b[2577] = (s.v[653] > 0.5);s.store_scalar(2577, if s.b[2577] { 1.0 } else { 0.0 });s.b[2578] = (s.v[410] == 0.5);s.store_scalar(2578, if s.b[2578] { 1.0 } else { 0.0 });
        if (((s.b[2569] && s.b[2570]) && s.b[2577]) && s.b[2578]) {s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::scale(s.ad_value(2574), s.v[407]));}
        if (((s.b[2569] && s.b[2570]) && s.b[2577]) && (!s.b[2578])) {s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[407])), s.v[410]);}
        if ((s.b[2569] && s.b[2570]) && s.b[2577]) {s.store_add_scaled_inputs3_offset_indices(1903, 2573, (-s.v[419]), 821, s.v[422], 2574, (-s.v[422]), s.v[419]);}
        s.b[2579] = (s.v[654] > 0.5);s.store_scalar(2579, if s.b[2579] { 1.0 } else { 0.0 });s.b[2580] = (s.v[411] == 0.5);s.store_scalar(2580, if s.b[2580] { 1.0 } else { 0.0 });
        if (((s.b[2569] && s.b[2570]) && s.b[2579]) && s.b[2580]) {s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::scale(s.ad_value(2574), s.v[408]));}
        if (((s.b[2569] && s.b[2570]) && s.b[2579]) && (!s.b[2580])) {s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[408])), s.v[411]);}
        if ((s.b[2569] && s.b[2570]) && s.b[2579]) {s.store_add_scaled_inputs3_offset_indices(1904, 2573, (-s.v[420]), 821, s.v[423], 2574, (-s.v[423]), s.v[420]);}
        if (s.b[2569] && s.b[2570]) {s.store_scalar(2573, 0.0);s.store_scalar(2574, 0.0);s.store_primal_scaled_mul(2525, 685, 685, 4.0);s.store_primal_div(2526, 685, 686);s.store_add_scaled_product_indices(2527, 822, 1.0, 685, 2526, 1.0);s.store_add(2528, 686, 2527);s.store_sub(2529, 686, 2527);s.store_sqrt_square_add(2530, 2529, 2525);s.store_div_scaled_product_add_scaled_denominator_indices(2574, 822, 686, 2.0, 2528, 1.0, 2530, 1.0, 1.0);}
        s.b[2581] = (s.v[679] > 0.5);s.store_scalar(2581, if s.b[2581] { 1.0 } else { 0.0 });s.b[2582] = (s.v[576] == 0.5);s.store_scalar(2582, if s.b[2582] { 1.0 } else { 0.0 });
        if (((s.b[2569] && s.b[2570]) && s.b[2581]) && s.b[2582]) {s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::mul(s.ad_value(2574), s.ad_value(573)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_125(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2569] && s.b[2570]) && s.b[2581]) && (!s.b[2582])) {s.store_pow_sub_from_scalar_mul_base_indices(2573, 1.0, 2574, 573, 576);}
        if ((s.b[2569] && s.b[2570]) && s.b[2581]) {s.store_add_scaled_product_mixed_aia(1905, A::mul_sub_from_scalar_rhs(s.ad_value(585), 1.0, s.ad_value(2573)), 1.0, 588, A::sub(s.ad_value(822), s.ad_value(2574)), 1.0);}
        s.b[2583] = (s.v[680] > 0.5);s.store_scalar(2583, if s.b[2583] { 1.0 } else { 0.0 });s.b[2584] = (s.v[577] == 0.5);s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });
        if (((s.b[2569] && s.b[2570]) && s.b[2583]) && s.b[2584]) {s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::mul(s.ad_value(2574), s.ad_value(574)));}
        if (((s.b[2569] && s.b[2570]) && s.b[2583]) && (!s.b[2584])) {s.store_pow_sub_from_scalar_mul_base_indices(2573, 1.0, 2574, 574, 577);}
        if ((s.b[2569] && s.b[2570]) && s.b[2583]) {s.store_add_scaled_product_mixed_aia(1906, A::mul_sub_from_scalar_rhs(s.ad_value(586), 1.0, s.ad_value(2573)), 1.0, 589, A::sub(s.ad_value(822), s.ad_value(2574)), 1.0);}
        s.b[2585] = (s.v[681] > 0.5);s.store_scalar(2585, if s.b[2585] { 1.0 } else { 0.0 });s.b[2586] = (s.v[578] == 0.5);s.store_scalar(2586, if s.b[2586] { 1.0 } else { 0.0 });
        if (((s.b[2569] && s.b[2570]) && s.b[2585]) && s.b[2586]) {s.store_sqrt_sub_from_scalar_ad(2573, 1.0, A::mul(s.ad_value(2574), s.ad_value(575)));}
        if (((s.b[2569] && s.b[2570]) && s.b[2585]) && (!s.b[2586])) {s.store_pow_sub_from_scalar_mul_base_indices(2573, 1.0, 2574, 575, 578);}
        if ((s.b[2569] && s.b[2570]) && s.b[2585]) {s.store_add_scaled_product_mixed_aia(1907, A::mul_sub_from_scalar_rhs(s.ad_value(587), 1.0, s.ad_value(2573)), 1.0, 590, A::sub(s.ad_value(822), s.ad_value(2574)), 1.0);}
        s.b[2587] = (p.p889 > 0.0);s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2587]) {s.store_scaled_offset_ad(643, A::powf(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), p.p890), (-(((0.5 * 0.001)) as f64).powf(p.p890)), p.p889);s.store_offset(641, 643, p.p879);s.store_div_from_scalar(451, 1.0, 641);s.store_div_from_scalar_offset_scaled_input(454, s.v[454], 643, 1.0 / (p.p879), 1.0);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2587])) {s.store_scalar(641, p.p879);}
        s.b[2588] = (p.p891 > 0.0);s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2588]) {s.store_scaled_offset_ad(645, A::powf(A::add_scaled_inputs3(s.ad_value(814), 0.5, s.ad_value(816), 0.5, A::sqrt_square_offset(A::add(s.ad_value(814), s.ad_value(816)), (0.001 * 0.001)), 0.5), p.p892), (-(((0.5 * 0.001)) as f64).powf(p.p892)), p.p891);s.store_mul_scale_offset_indices(444, 444, 645, 1.0, 1.0);}
        if (s.b[2569] && (!s.b[2570])) {s.store_scalar(2538, 0.0);s.store_scalar(2535, 0.0);}
        s.b[2589] = (!(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)));s.store_scalar(2589, if s.b[2589] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2589]) {s.store_primal_scaled_mul(2525, 658, 658, 4.0);s.store_primal_div(2526, 658, 659);s.store_add_scaled_product_indices(2527, 821, 1.0, 658, 2526, 1.0);s.store_add(2528, 659, 2527);s.store_sub(2529, 659, 2527);s.store_sqrt_square_add(2530, 2529, 2525);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_126(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if ((s.b[2569] && (!s.b[2570])) && s.b[2589]) {s.store_div_scaled_product_add_scaled_denominator_indices(2532, 821, 659, 2.0, 2528, 1.0, 2530, 1.0, 1.0);}
        s.b[2590] = (s.v[821] < s.v[655]);s.store_scalar(2590, if s.b[2590] { 1.0 } else { 0.0 });s.b[2591] = (((((-0.5) * (s.v[821] * s.v[372]))) as f64).abs() < 230.25850929940458);s.store_scalar(2591, if s.b[2591] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) && s.b[2591]) {s.store_exp_scaled_input(2533, 821, (s.v[372] * (-0.5)));}
        s.b[2592] = (((-0.5) * (s.v[821] * s.v[372])) < 0.0);s.store_scalar(2592, if s.b[2592] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) && (!s.b[2591])) && s.b[2592]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2533, 1e-100, (-230.25850929940458), A::scale(s.ad_value(821), (s.v[372] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) && (!s.b[2591])) && (!s.b[2592])) {s.store_scaled_offset_ad(2533, A::mul_offset_rhs(A::scale_offset(s.ad_value(821), (s.v[372] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(821), (s.v[372] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(821), (((s.v[372] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2590]) {s.store_div_from_scalar(2534, 1.0, 2533);s.store_square(2531, 2534);}
        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && (!s.b[2590])) {s.store_mul_scale_offset_mixed_ia(2531, 656, A::sub_scaled_inputs(s.ad_value(821), s.v[372], s.ad_value(655), s.v[372]), 1.0, 1.0);s.store_sqrt(2534, 2531);s.store_div_from_scalar(2533, 1.0, 2534);}
        if ((s.b[2569] && (!s.b[2570])) && s.b[2589]) {s.store_offset(2531, 2531, (-1.0));}
        s.b[2593] = (s.v[821] > 0.0);s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && s.b[2593]) {s.store_scaled_ln_ad(2535, A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2533), 1.0, A::offset(s.ad_value(2533), 3.0)))), (s.v[371] * 2.0));}
        if (((s.b[2569] && (!s.b[2570])) && s.b[2589]) && (!s.b[2593])) {s.store_sub_mixed_ai(2535, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2534), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2534), 1.0, A::scale_offset(s.ad_value(2534), 3.0, 1.0))))), (s.v[371] * 2.0)), 821);}
        if ((s.b[2569] && (!s.b[2570])) && s.b[2589]) {s.store_sub(2536, 657, 2535);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2537, 821, 0.5, 2536, 0.5, 821, 2536, ((4.0 * s.v[371]) * s.v[371]), (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2538, 821, 0.5, 660, 0.5, 821, 660, ((4.0 * s.v[369]) * s.v[369]), (-0.5));s.store_scaled_sub_mixed_ia(2539, 821, A::sqrt_square_offset(s.ad_value(821), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[2594] = (s.v[647] == 0.0);s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });
        if ((s.b[2569] && (!s.b[2570])) && s.b[2594]) {s.store_scalar(1896, 0.0);s.store_scalar(1902, 0.0);}
        if ((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) {s.store_scale(2541, 2531, s.v[388]);}
        s.b[2595] = ((p.p857 == 0.0) && (p.p862 == 0.0));s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && s.b[2595]) {s.store_scalar(2542, 0.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) {s.store_sub_from_scalar(2543, s.v[394], 2537);s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));}
        s.b[2596] = (p.p848 == 0.5);s.store_scalar(2596, if s.b[2596] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) && s.b[2596]) {s.store_scalar(2545, 0.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) && (!s.b[2596])) {s.store_scaled_add_mixed_ai(2545, A::div_scaled_product(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544)), 1.0, A::sub_from_scalar(1.0, s.ad_value(2544)), 1.0), 2544, (1.0 - (2.0 * p.p848)));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) {s.store_add(2546, 2544, 2545);}
        s.b[2597] = (p.p848 == 0.5);s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) && s.b[2597]) {s.store_sqrt_scaled_input(2540, 2543, s.v[430]);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) && (!s.b[2597])) {s.store_powf_scaled_input(2540, 2543, s.v[430], p.p848);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2595])) {s.store_scale(2547, 2540, s.v[424]);s.store_mul_scale_offset_indices(2548, 2547, 2534, s.v[385], ((-1.0)) * (s.v[385]));s.store_scaled_mul(2542, 2548, 2546, p.p857);}
        s.b[2598] = (p.p862 == 0.0);s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && s.b[2598]) {s.store_scalar(2549, 0.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_127(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) {s.store_div_scaled_inputs_indices(2550, 2547, (s.v[409] * s.v[439]), 2543, 1.0);s.store_div_from_scalar(2551, (0.666666666666667 * s.v[436]), 2550);s.store_square(2552, 2551);s.store_sqrt_div_scaled_square_offset_denominator(2553, 2552, 1.0, 1.0, 1.0);s.store_sqrt(2554, 2553);s.store_mul(2555, 2553, 2554);}
        s.b[2599] = (((-p.p848) * s.v[412]) == (-1.0));s.store_scalar(2599, if s.b[2599] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && s.b[2599]) {s.store_div_from_scalar_offset_product(2556, 1.0, 2550, 2555, 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2599])) {s.store_powf_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), ((-p.p848) * s.v[412]));}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) {s.store_div_scaled_product_add_scaled_denominator_indices(2557, 2546, 2556, 1.0, 2546, 1.0, 2556, 1.0, 1.0);s.store_sqrt_scaled_input_ad(2558, A::div(s.ad_value(2550), s.ad_value(2554)), 0.375);s.store_add_scaled_product_indices(2559, 2553, (-1.0), 2551, 2554, 2.0);s.store_add_scaled_value_products_indices(2560, 2553, (-s.v[436]), 2551, 2554, s.v[436], 2550, 2555, 0.5);s.store_mul_scale_offset_indices(2561, 2558, 2559, 1.0, (-1.0));s.store_square(2522, 2561);}
        s.b[2600] = (s.v[2561] > 0.0);s.store_scalar(2600, if s.b[2600] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && s.b[2600]) {s.store_div_from_scalar_offset_scaled_input(2523, 1.0, 2561, s.v[373], 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2600])) {s.store_div_from_scalar_sub_from_scalar_ad(2523, 1.0, 1.0, A::scale(s.ad_value(2561), s.v[373]));}
        s.b[2601] = (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458));s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && s.b[2601]) {s.store_exp_sub(2540, 2560, 2522);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2601])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2540, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) {s.store_mul_mixed_ai(2524, A::add_scaled_inputs_product(s.ad_value(2523), 0.29214664, A::square(s.ad_value(2523)), s.v[374], A::square(s.ad_value(2523)), s.ad_value(2523), s.v[375]), 2540);}
        s.b[2602] = (s.v[2561] > 0.0);s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && s.b[2602]) {s.copy_ad(2562, 2524);}
        s.b[2603] = (s.v[2560] > (-230.25850929940458));s.store_scalar(2603, if s.b[2603] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2602])) && s.b[2603]) {s.store_exp(2540, 2560);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2603])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 2560, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) && (!s.b[2602])) {s.store_sub_scaled_inputs(2562, 2540, 2.0, 2524, 1.0);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2598])) {s.store_div_scaled_inputs_indices(2563, 2562, (s.v[436] * (1.772453850905516 * 0.5)), 2558, 1.0);s.store_mul3_affine_lhs(2549, 2548, 2563, p.p862, 0.0, 2557);}
        s.b[2604] = (p.p868 == 0.0);s.store_scalar(2604, if s.b[2604] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && s.b[2604]) {s.store_scalar(2564, 0.0);}
        s.b[2605] = (p.p848 == 0.5);s.store_scalar(2605, if s.b[2605] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && s.b[2605]) {s.store_sqrt_scaled_input_ad(2540, A::sub_from_scalar(p.p845, s.ad_value(2538)), s.v[430]);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && (!s.b[2605])) {s.store_powf_scale_offset_input(2540, 2538, (-s.v[430]), ((p.p845) * (s.v[430])), p.p848);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) {s.store_div_scaled_offset_numerator_indices(2565, 2538, ((-s.v[427]) * s.v[412]), (((p.p845) * (s.v[427])) * s.v[412]), 2540, 1.0);}
        s.b[2606] = (((((-s.v[442]) / s.v[2565])) as f64).abs() < 230.25850929940458);s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && s.b[2606]) {s.store_ad_value(2540, A::exp_div_scaled_inputs(s.ad_value(442), -1.0, s.ad_value(2565), 1.0));}
        s.b[2607] = (((-s.v[442]) / s.v[2565]) < 0.0);s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && (!s.b[2606])) && s.b[2607]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2540, 1e-100, (-230.25850929940458), 442, -1.0, 2565, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) && (!s.b[2606])) && (!s.b[2607])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2540, 442, -1.0, 2565, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2604])) {s.store_mul_scale_offset_mixed_ai(2564, A::mul3(s.ad_value(821), s.ad_value(2565), s.ad_value(2565)), 2540, p.p868, 0.0);}
        s.b[2608] = (p.p877 > 1000.0);s.store_scalar(2608, if s.b[2608] { 1.0 } else { 0.0 });
        if (((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && s.b[2608]) {s.store_scalar(2566, 1.0);}
        s.b[2609] = (s.v[2539] > ((-s.v[445]) * p.p877));s.store_scalar(2609, if s.b[2609] { 1.0 } else { 0.0 });s.b[2610] = (p.p880 == 4.0);s.store_scalar(2610, if s.b[2610] { 1.0 } else { 0.0 });
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2608])) && s.b[2609]) && s.b[2610]) {s.store_mul_scale_offset_mixed_ai(2540, A::mul3_scaled_output(s.ad_value(2539), s.ad_value(2539), s.ad_value(2539), ((s.v[449] * s.v[449]) * s.v[449])), 2539, s.v[449], 0.0);}
        if (((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2608])) && s.b[2609]) && (!s.b[2610])) {s.store_powf_ad(2540, A::abs_scaled_input(s.ad_value(2539), s.v[449]), p.p880);}
        if ((((s.b[2569] && (!s.b[2570])) && (!s.b[2594])) && (!s.b[2608])) && s.b[2609]) {s.store_div_from_scalar_sub_from_scalar_ad(2566, 1.0, 1.0, s.ad_value(2540));}
    }
}
