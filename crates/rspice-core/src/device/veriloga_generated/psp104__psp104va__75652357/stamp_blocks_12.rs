#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_51(
        s: &mut ReactiveScratch,
    ) {
        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) {s.store_pow_ad(2361, A::mul3(s.ad_value(768), s.ad_value(1919), s.ad_value(698)), s.ad_value(699));s.store_mul_mixed_ai(1920, A::div_scaled_product_offset_rhs(s.ad_value(699), A::mul_sub_from_scalar_rhs(s.ad_value(2360), 1.0, s.ad_value(769)), (-1.0), 1.0, s.ad_value(1919), 1.0), 2361);s.store_div(1919, 2358, 2359);s.store_mul_pow_mixed_iaa(2362, 700, A::offset(s.ad_value(1919), 1.0), A::neg(s.ad_value(701)));s.store_mul_div_scaled_product_mixed_iiai(1921, 2362, 701, A::add(A::offset(s.ad_value(2360), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(1919), 1.0, 1.0)), 1.0, 2359, 1.0);s.store_mul_product3_indices(2363, 2358, 751, 2344, 2345, 1.0);s.store_offset_ad(1919, A::div_scaled_add_product(s.ad_value(1920), 1.0, A::mul3(s.ad_value(751), s.ad_value(2344), s.ad_value(2345)), s.ad_value(2360), (-1.0), s.ad_value(1921), 1.0), 1.0);}
        s.b[2472] = (s.v[1919] < 230.25850929940458);s.store_scalar(2472, if s.b[2472] { 1.0 } else { 0.0 });
        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) && s.b[2472]) {s.store_scaled_ln_one_plus_exp_scaled_input(1920, 1919, 2.0, 0.5);}
        if ((((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) && (!s.b[2472])) {s.copy_ad(1920, 1919);}
        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2471]) {s.store_div_scaled_product3_mixed_iiia(2364, 2357, 1921, 1920, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2361), 1.0, s.ad_value(2362), 1.0, s.ad_value(2363), 1.0, 1.0), 1.0);s.store_mul_scale_offset_mixed_ia(2365, 2356, A::div_scaled_value_offset_denominator(s.ad_value(2364), 1.0, A::sqrt_square_offset(s.ad_value(2364), 1.0), 1.0, 1.0), 1.0, 1.0);}
        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && (!s.b[2471])) {s.copy_ad(2365, 2356);}
        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {s.store_mul3_affine_lhs(2366, 2305, 2354, 0.7071067811865475, 0.0, 2365);}
        s.b[2473] = (s.v[0] == (-1.0));s.store_scalar(2473, if s.b[2473] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2467]) && s.b[2468]) && s.b[2473]) {s.store_div_mixed_ia(2366, 2366, A::sqrt(A::offset(s.ad_value(2366), 1.0)));}
        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {s.store_div_from_scalar_offset_ad(2367, 2.0, A::sqrt(A::scale_offset(s.ad_value(2366), 4.0, 1.0)), 1.0);s.store_mul(1919, 2367, 2366);s.store_mul_ad_product_rhs_mixed_ia(2368, 2365, 2367, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1919), 1.0, A::mul(s.ad_value(1919), s.ad_value(2367)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(1919), s.ad_value(1919), s.ad_value(2367), 4.0), 1.0)), 1.0));s.store_scale(2369, 2368, 0.99);s.store_div_scaled_product3_mixed_iaii(1919, 2369, A::sub_scaled_inputs(s.ad_value(2369), 1.0, s.ad_value(2355), 2.0), 2307, 1.0, 2336, 1.0);}
        if ((s.b[2439] && s.b[2467]) && s.b[2468]) {
            s.store_mul_sub_mixed_iia(2370, 2305, 2369, A::ln(A::offset({
                if (s.v[1919] > (-0.99)) {
                    s.ad_value(1919)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }
        if ((s.b[2439] && s.b[2467]) && (!s.b[2468])) {s.copy_ad(2370, 2353);}
        if (s.b[2439] && s.b[2467]) {s.store_offset(1919, 2286, 1.0);s.store_div_scaled_product_mixed_aii(1920, A::sqrt(s.ad_value(1919)), 820, 1.0, 2370, 1.0);s.store_add_mixed_ai(1921, A::square(s.ad_value(1920)), 1919);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_52(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2439] && s.b[2467]) {s.store_scale(1919, 1920, 2.0);s.store_div_scaled_product_add_scaled_denominator(2371, 2370, 1919, 1.0, A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), 1.0, A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919))), 1.0, 1.0);s.store_mul(2372, 2371, 2306);s.store_add(2373, 2315, 2372);}
        s.b[2474] = (s.v[2372] < 460.51701859880916);s.store_scalar(2474, if s.b[2474] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2467]) && s.b[2474]) {s.store_exp_neg_input(2374, 2372);}
        if ((s.b[2439] && s.b[2467]) && (!s.b[2474])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2374, 1e-200, 2372, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2439] && s.b[2467]) {s.store_mul(2375, 2330, 2374);}
        s.b[2475] = (((s.v[2309]) as f64).abs() <= s.v[2327]);s.store_scalar(2475, if s.b[2475] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2467]) && s.b[2475]) {s.store_scaled_square(2415, 2328, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2376, 2309, 2328, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2309), 1.0, s.ad_value(2375)), s.ad_value(2290), s.ad_value(2415)), 1.0));}
        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {s.store_offset(2436, 2373, 3.0);s.store_sub_ad(2419, A::add_scaled_inputs3(s.ad_value(2435), 0.5, s.ad_value(2436), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2435), s.ad_value(2436)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2436), 0.5, A::sqrt_square_offset(s.ad_value(2436), 5.0), 0.5));s.store_sub(2414, 2309, 2419);s.store_exp_neg_input(2415, 2419);s.store_div_from_scalar_offset_square(2416, 1.0, 2419, 2.0);s.store_mul_square_lhs(2426, 2419, 2416);s.store_mul3_affine_lhs(2427, 2419, 2416, 4.0, 0.0, 2416);s.store_mul_ad_product_lhs_mixed_ai(2428, A::sub_scaled_inputs(s.ad_value(2416), 8.0, s.ad_value(2426), 12.0), 2416, 2416);}
        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {
            if (1e-40 > ((s.v[2414] * s.v[2414]) - (s.v[2291] * (((s.v[2415] + s.v[2419]) - 1.0) - (s.v[2375] * ((s.v[2419] + 1.0) + s.v[2426])))))) {
                s.store_scalar(2420, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2420, 2414, 1.0, 2291, A::add_scaled_product(A::offset(A::add(s.ad_value(2415), s.ad_value(2419)), (-1.0)), 1.0, s.ad_value(2375), A::add(A::offset(s.ad_value(2419), 1.0), s.ad_value(2426)), (-1.0)), (-1.0));
            }
        }
        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2437, 1.0, 2291, A::add_scaled_product(s.ad_value(2415), 1.0, s.ad_value(2375), s.ad_value(2428), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2421, 2414, 2.0, 2291, A::add_scaled_sub_value_product(1.0, s.ad_value(2415), 1.0, s.ad_value(2375), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2422, 2373, 1.0, 2419, (-1.0), A::ln(A::div(s.ad_value(2420), s.ad_value(2291))), 1.0);s.store_add(818, 2420, 2421);s.store_add_scaled_square_product_mixed_iia(817, 818, 1.0, 2422, A::add_scaled_square_product(s.ad_value(2421), 0.5, s.ad_value(2420), s.ad_value(2437), (-1.0)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_53(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {s.store_add_mixed_ia(2438, 2419, A::div_scaled_product3(s.ad_value(2420), s.ad_value(818), s.ad_value(2422), 1.0, A::add(s.ad_value(817), A::mul3(A::mul3(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422), s.ad_value(2422)), s.ad_value(2421), A::add_scaled_square_product(s.ad_value(2421), 0.3333333333333333, s.ad_value(2420), s.ad_value(2437), (-1.0)))), 1.0));}
        s.b[2476] = (s.v[2438] < 230.25850929940458);s.store_scalar(2476, if s.b[2476] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2467]) && (!s.b[2475])) && s.b[2476]) {s.store_exp(2424, 2438);s.store_div_from_scalar(2425, 1.0, 2424);s.store_mul(2424, 2375, 2424);}
        s.b[2477] = (s.v[2438] > (s.v[2373] - 230.25850929940458));s.store_scalar(2477, if s.b[2477] { 1.0 } else { 0.0 });
        if ((((s.b[2439] && s.b[2467]) && (!s.b[2475])) && (!s.b[2476])) && s.b[2477]) {s.store_exp_sub(2424, 2438, 2373);s.store_div(2425, 2375, 2424);}
        if ((((s.b[2439] && s.b[2467]) && (!s.b[2475])) && (!s.b[2476])) && (!s.b[2477])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2424, 1e-100, A::sub(s.ad_value(2373), s.ad_value(2438)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2425, 1e-100, 2438, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[2439] && s.b[2467]) && (!s.b[2475])) {s.store_div_from_scalar_offset_square(2414, 1.0, 2438, 2.0);s.store_mul_square_lhs(2426, 2438, 2414);s.store_mul3_affine_lhs(2427, 2438, 2414, 4.0, 0.0, 2414);s.store_mul_ad_product_lhs_mixed_ai(2428, A::sub_scaled_inputs(s.ad_value(2414), 8.0, s.ad_value(2426), 12.0), 2414, 2414);s.store_sub(2414, 2309, 2438);s.store_add_scaled_product_mixed_iia(2429, 2414, 2.0, 2291, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2425)), 1.0, s.ad_value(2424), 1.0, s.ad_value(2375), A::offset(s.ad_value(2427), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2430, 2414, 1.0, 2291, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2425), 1.0, s.ad_value(2438), 1.0, s.ad_value(2424), 1.0, (-1.0)), 1.0, s.ad_value(2375), A::add(A::offset(s.ad_value(2438), 1.0), s.ad_value(2426)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2414, 2.0, 2291, A::add_scaled_inputs_product(s.ad_value(2425), 1.0, s.ad_value(2424), 1.0, s.ad_value(2375), s.ad_value(2428), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2414, 2429, 1.0, 2430, 2414, (-2.0));s.store_add_scaled_inputs_mixed_ia(2376, 2438, 1.0, A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0);}
        if (s.b[2439] && s.b[2467]) {s.store_sub(2377, 2376, 2329);}
        s.b[2478] = (s.v[2377] < 1e-10);s.store_scalar(2478, if s.b[2478] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2467]) && s.b[2478]) {s.store_add_scaled_inputs_product_mixed_iiia(2378, 2309, 2.0, 2329, (-2.0), 2291, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2335), 1.0, s.ad_value(2334), s.ad_value(2374), 1.0), 1.0, s.ad_value(2375), s.ad_value(2332), 1.0, (-1.0)), 1.0);s.store_mul_mixed_ai(2379, A::mul_sub_from_scalar_rhs(s.ad_value(2291), 1.0, s.ad_value(2374)), 2336);s.store_sub_from_scalar_scaled_mul_mixed_ia(1919, 2.0, 2291, A::add_scaled_value_products(s.ad_value(2335), 1.0, s.ad_value(2334), s.ad_value(2374), 1.0, s.ad_value(2375), s.ad_value(2333), (-1.0)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_54(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2439] && s.b[2467]) && s.b[2478]) {s.store_add_scaled_square_product_indices(1919, 2378, 1.0, 1919, 2379, (-2.0));s.store_scaled_div_mixed_ia(2377, 2379, A::add(s.ad_value(2378), A::sqrt(s.ad_value(1919))), 2.0);s.store_add(2376, 2329, 2377);}
        if (s.b[2439] && s.b[2467]) {s.store_mul(2380, 2377, 2305);s.store_div_scaled_product_offset_denominator_mixed_iia(2381, 2376, 2376, 1.0, A::square(s.ad_value(2376)), 2.0, 1.0);}
        s.b[2479] = (s.v[2376] < 230.25850929940458);s.store_scalar(2479, if s.b[2479] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2467]) && s.b[2479]) {s.store_exp_neg_input(2382, 2376);}
        s.b[2480] = (s.v[2376] < 1e-5);s.store_scalar(2480, if s.b[2480] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2467]) && s.b[2479]) && s.b[2480]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2383, 2376, 1.0, 2376, 1.0, 2376, 0.25, 0.3333333333333333, 0.5);s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2376), 1.0, A::scale(s.ad_value(2376), 0.25), 0.3333333333333333));s.store_scaled_mul(2384, 2376, 1919, 0.7071067811865475);s.store_mul3_ad_middle(2385, A::mul3_scaled_output(s.ad_value(2375), s.ad_value(2376), s.ad_value(2376), 0.16666666666666666), 2376, A::scale_offset(s.ad_value(2376), 1.75, 1.0));}
        if (((s.b[2439] && s.b[2467]) && s.b[2479]) && (!s.b[2480])) {s.store_add_offset_lhs(2383, 2376, (-1.0), 2382);s.store_sqrt(2384, 2383);s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(2385, 2375, A::div_from_scalar(1.0, s.ad_value(2382)), 1.0, 2376, (-1.0), 2381, -1.0, (-1.0));}
        s.b[2481] = (s.v[2376] > (s.v[2373] - 230.25850929940458));s.store_scalar(2481, if s.b[2481] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2467]) && (!s.b[2479])) && s.b[2481]) {s.store_exp_sub(1919, 2376, 2373);s.store_div(2382, 2375, 1919);s.store_add_scaled_product_mixed_iia(2385, 1919, 1.0, 2375, A::add(A::offset(s.ad_value(2376), 1.0), s.ad_value(2381)), (-1.0));}
        if (((s.b[2439] && s.b[2467]) && (!s.b[2479])) && (!s.b[2481])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2382, 1e-100, 2376, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(1919, 1e-100, A::sub(s.ad_value(2373), s.ad_value(2376)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(2385, 1919, 1.0, 2375, A::add(A::offset(s.ad_value(2376), 1.0), s.ad_value(2381)), (-1.0));}
        if ((s.b[2439] && s.b[2467]) && (!s.b[2479])) {s.store_add_offset_lhs(2383, 2376, (-1.0), 2382);s.store_sqrt(2384, 2383);}
        if (s.b[2439] && s.b[2467]) {s.store_mul3_lhs(2386, 2384, 2290, 2305);s.store_scaled_add(2387, 2329, 2376, 0.5);s.store_scalar(2388, 0.0);s.store_mul(1919, 2382, 2335);}
        s.b[2482] = (s.v[1919] > 0.0);s.store_scalar(2482, if s.b[2482] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2467]) && s.b[2482]) {s.store_sqrt(2388, 1919);}
        if (s.b[2439] && s.b[2467]) {s.store_scaled_add(2389, 2336, 2385, 0.5);s.store_add_scaled_product_mixed_iaa(2390, 2389, 1.0, A::square(s.ad_value(2377)), A::sub_scaled_inputs(s.ad_value(2388), 1.0, s.ad_value(2307), 2.0), 0.125);}
        s.b[2483] = (s.v[2387] < 1e-5);s.store_scalar(2483, if s.b[2483] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2467]) && s.b[2483]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2391, 2387, 1.0, 2387, 1.0, 2387, 0.25, 0.3333333333333333, 0.5);s.store_mul_sqrt_mixed_ia(2392, 2290, A::add(s.ad_value(2390), s.ad_value(2391)));}
        s.b[2484] = (s.v[724] > 0.0);s.store_scalar(2484, if s.b[2484] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2467]) && s.b[2483]) && s.b[2484]) {s.store_div_from_scalar_sqrt_ad(2393, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2392)), 1.0));}
        if ((s.b[2439] && s.b[2467]) && s.b[2483]) {s.store_sqrt_sub_from_scalar_ad(1919, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2387), 1.0, A::scale(s.ad_value(2387), 0.25), 0.3333333333333333));s.store_scaled_mul(2394, 2387, 1919, 0.7071067811865475);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_55(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2439] && s.b[2467]) && s.b[2483]) {s.store_add_mixed_ia(2395, 2393, A::div_scaled_product(s.ad_value(2290), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2387), 0.5)), 1.0, A::square(s.ad_value(2387)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1919), 1.0));}
        if ((s.b[2439] && s.b[2467]) && (!s.b[2483])) {s.store_add_offset_lhs(2391, 2387, (-1.0), 2388);s.store_mul_sqrt_mixed_ia(2392, 2290, A::add(s.ad_value(2390), s.ad_value(2391)));}
        s.b[2485] = (s.v[724] > 0.0);s.store_scalar(2485, if s.b[2485] { 1.0 } else { 0.0 });
        if (((s.b[2439] && s.b[2467]) && (!s.b[2483])) && s.b[2485]) {s.store_add_scaled_sub_value_product_indices(2396, 1.0, 2388, 1.0, 2392, 2307, 2.0);s.store_div_from_scalar_sqrt_ad(2393, 1.0, A::offset(A::mul(s.ad_value(724), s.ad_value(2392)), 1.0));s.store_div_scaled_value_offset_denominator(1919, s.ad_value(2393), 1.0, s.ad_value(2393), 1.0, 1.0);s.store_mul_product3_mixed_iaii(2397, 724, A::square(s.ad_value(1919)), 2291, 2390, 1.0);s.store_add_scaled_inputs_product_mixed_iiia(2398, 2392, 2.0, 2397, (-2.0), 2291, A::add(A::sub_from_scalar(1.0, s.ad_value(2388)), s.ad_value(2390)), 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(2399, 2397, 2397, 1.0, 2392, 2.0);s.store_sub_from_scalar_scaled_mul_mixed_ia(2400, 1.0, 2291, A::add(s.ad_value(2388), s.ad_value(2390)), 0.5);s.store_div_scaled_product_mixed_iia(2401, 2399, 2398, 1.0, A::add_scaled_square_product(s.ad_value(2398), 1.0, s.ad_value(2400), s.ad_value(2399), (-1.0)), 1.0);s.store_add(2387, 2387, 2401);s.store_exp(2402, 2401);s.store_div(2388, 2388, 2402);s.store_mul(2390, 2390, 2402);s.store_add_offset_lhs(2391, 2387, (-1.0), 2388);s.store_mul_sqrt_mixed_ia(2392, 2290, A::add(s.ad_value(2390), s.ad_value(2391)));s.store_add_ad(2403, A::sub_from_scalar(1.0, s.ad_value(2388)), A::mul3_scaled_output(s.ad_value(2392), s.ad_value(2393), s.ad_value(2307), 2.0));s.store_div_scaled_product3_mixed_iiaa(2377, 2377, 2402, A::add(s.ad_value(2396), s.ad_value(2389)), 1.0, A::add_scaled_product(s.ad_value(2403), 1.0, s.ad_value(2402), s.ad_value(2389), 1.0), 1.0);s.store_mul(2380, 2377, 2305);}
        if ((s.b[2439] && s.b[2467]) && (!s.b[2483])) {s.store_sqrt(2394, 2391);s.store_add_scaled_inputs_mixed_ia(2395, 2393, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2290), 1.0, s.ad_value(2388)), s.ad_value(2394)), 0.5);}
        if (s.b[2439] && s.b[2467]) {s.store_mul_div_scaled_product_mixed_iiia(2404, 2305, 2291, 2390, 1.0, A::add_scaled_product(s.ad_value(2392), 1.0, s.ad_value(2290), s.ad_value(2394), 1.0), 1.0);s.store_add_scaled_product_indices(2405, 2404, 1.0, 2305, 2395, 1.0);s.store_mul3_lhs(2406, 2394, 2290, 2305);}
        s.b[2486] = (s.v[213] < 0.0);s.store_scalar(2486, if s.b[2486] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2467]) && s.b[2486]) {s.store_sub_from_scalar_scaled_mul(2345, 1.0, 213, 2404, 1.0);}
        if ((s.b[2439] && s.b[2467]) && (!s.b[2486])) {s.store_div_from_scalar_offset_product(2345, 1.0, 213, 2404, 1.0);}
        if (s.b[2439] && s.b[2467]) {s.store_mul_product3_indices(2346, 2404, 751, 2344, 2345, 1.0);s.store_add_scaled_product_indices(2407, 2406, 1.0, 769, 2404, 1.0);s.store_add_scaled_product_indices(2408, 2406, 1.0, 770, 2404, 1.0);s.store_mul(2409, 768, 2407);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_56(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2439] && s.b[2467]) {s.store_ln_ad(1920, A::div_scaled_value_offset_denominator(s.ad_value(2391), 1.0, A::add(s.ad_value(2391), s.ad_value(2390)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2348, A::pow(A::mul(s.ad_value(2409), s.ad_value(698)), s.ad_value(699)), 1.0, 700, A::exp(A::mul_scaled_lhs(s.ad_value(701), 0.5, s.ad_value(1920))), 1.0);s.store_mul_add_mixed_iai(2410, 2340, A::offset(s.ad_value(2348), 1.0), 2346);s.store_ln_ad(2411, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(820), s.ad_value(2380)), s.ad_value(773)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2371), s.ad_value(2380)), s.ad_value(773)), 1.0), 1.0));s.store_mul(1921, 2404, 2350);s.store_div_add_scaled_inputs_rhs_indices(2351, 1921, 218, 1.0, 1921, 1.0);}
        s.b[2487] = (s.v[217] < 0.0);s.store_scalar(2487, if s.b[2487] { 1.0 } else { 0.0 });
        if ((s.b[2439] && s.b[2467]) && s.b[2487]) {s.store_div_from_scalar_sub_from_scalar_ad(2352, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2351)));}
        if ((s.b[2439] && s.b[2467]) && (!s.b[2487])) {s.store_offset_mul(2352, 217, 2351, 1.0);}
        if (s.b[2439] && s.b[2467]) {s.store_mul(2413, 2285, 2352);s.store_mul(2412, 2392, 2305);}
        if s.b[2439] {s.copy_ad(1875, 2287);s.copy_ad(1876, 2305);s.copy_ad(1877, 2290);s.copy_ad(1878, 2309);s.copy_ad(1879, 2314);s.copy_ad(1880, 2343);s.copy_ad(1881, 2380);s.copy_ad(1882, 2386);s.copy_ad(1883, 2393);s.copy_ad(1884, 2395);s.copy_ad(1885, 2404);s.copy_ad(1886, 2405);s.copy_ad(1887, 2408);s.copy_ad(1888, 2410);s.copy_ad(1889, 2411);s.copy_ad(1890, 2413);s.copy_ad(1891, 2412);}
        if (!s.b[2439]) {s.copy_ad(739, 722);s.copy_ad(1875, 1810);s.copy_ad(1876, 1812);s.copy_ad(1877, 1814);s.copy_ad(1878, 1817);s.copy_ad(1879, 1818);s.copy_ad(1880, 1837);s.copy_ad(1881, 1848);s.copy_ad(1882, 1849);s.copy_ad(1883, 1851);s.copy_ad(1884, 1852);s.copy_ad(1885, 1853);s.copy_ad(1886, 1854);s.copy_ad(1887, 1856);s.copy_ad(1888, 1857);s.copy_ad(1889, 1859);s.copy_ad(1890, 1858);s.copy_ad(1891, 1860);}
        s.copy_ad(1892, 250);s.b[2488] = (s.v[767] > 0.0);s.store_scalar(2488, if s.b[2488] { 1.0 } else { 0.0 });
        if s.b[2488] {s.store_div_scaled_value_offset_denominator(1892, s.ad_value(250), 1.0, A::mul(s.ad_value(767), A::powf(A::offset(A::square(s.ad_value(1887)), s.v[727]), ((-1.0) * 0.16666666666666666))), 1.0, 1.0);}
        s.store_scalar(1893, 1.0);s.store_scalar(1894, 1.0);s.store_scalar(1895, 0.0);s.store_scalar(1896, 1.0);s.store_scalar(1897, 1.0);s.copy_ad(2251, 1891);s.store_scalar(2254, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_57(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(2253, 0.0);s.copy_ad(2255, 2251);s.b[2489] = (s.v[1878] > 0.0);s.store_scalar(2489, if s.b[2489] { 1.0 } else { 0.0 });
        if s.b[2489] {s.store_mul_div_scaled_product_mixed_iaii(2246, 1889, A::add(s.ad_value(255), A::div(s.ad_value(256), s.ad_value(1886))), 1885, 1.0, 1886, 1.0);}
        s.b[2490] = (s.v[2246] > 0.0);s.store_scalar(2490, if s.b[2490] { 1.0 } else { 0.0 });
        if (s.b[2489] && s.b[2490]) {s.store_div_from_scalar_add_ad(1893, 1.0, A::offset(s.ad_value(2246), 1.0), A::square(s.ad_value(2246)));}
        if (s.b[2489] && (!s.b[2490])) {s.store_sub_from_scalar(1893, 1.0, 2246);}
        if s.b[2489] {s.store_mul(1894, 1888, 1893);s.store_div(1895, 1890, 1894);s.store_mul_ad_product_lhs_mixed_ai(2247, A::square(s.ad_value(1895)), 1881, 1881);}
        s.b[2491] = (s.v[0] == (-1.0));s.store_scalar(2491, if s.b[2491] { 1.0 } else { 0.0 });
        if (s.b[2489] && s.b[2491]) {s.store_div_scaled_value_offset_denominator(2247, s.ad_value(2247), 1.0, A::mul(s.ad_value(1895), s.ad_value(1881)), 1.0, 1.0);}
        if s.b[2489] {s.store_mul_scale_offset_mixed_ia(1896, 1894, A::sqrt(A::scale_offset(s.ad_value(2247), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div(1919, 1894, 1896);s.store_mul_scale_offset_mixed_ia(2248, 1884, A::mul3_scaled_output(s.ad_value(2247), s.ad_value(1919), s.ad_value(1919), 0.5), 1.0, 1.0);s.store_div_scaled_product_indices(1897, 1919, 1886, 1.0, 2248, 1.0);s.store_scaled_div(2249, 1881, 1897, 0.5);s.store_square(2250, 2249);s.store_add_product3_rhs_mixed_iia(2251, 1891, 1883, 1881, A::add(A::offset(A::mul_scaled_output(s.ad_value(2249), s.ad_value(1893), 0.3333333333333333), (-1.0)), s.ad_value(1893)), 0.5);s.store_scaled_mul(1919, 1884, 1881, 0.16666666666666666);}
        s.b[2492] = (p.p49 == 1.0);s.store_scalar(2492, if s.b[2492] { 1.0 } else { 0.0 });
        if (s.b[2489] && s.b[2492]) {s.store_scalar(2252, 0.0);s.store_mul_ad_affine_product_rhs(2253, 1893, s.ad_value(1893), A::sub(s.ad_value(1885), A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(1919), 2.0, s.ad_value(2249), 3.0)), 0.5, 0.0);}
        if (s.b[2489] && (!s.b[2492])) {s.store_mul_scale_offset_mixed_ai(2252, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1884), s.ad_value(1881), (-0.5)), 1893, -1.0, 1.0);s.store_add_scaled_products_mixed_aaia(2253, A::square(s.ad_value(1893)), A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1919), A::sub_scaled_inputs(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2250), 0.2), (-1.0)), 0.5, 2252, A::offset(s.ad_value(1893), 1.0), 0.5);}
        if s.b[2489] {s.store_add_scaled_product_mixed_iia(2254, 2252, 1.0, 1893, A::add_scaled_product(s.ad_value(1885), 1.0, s.ad_value(1919), s.ad_value(2249), 1.0), 1.0);s.store_sub(2255, 2251, 2254);}
        s.store_mul(845, 2251, 1892);s.store_mul_scale_offset_indices(847, 1892, 2253, -1.0, 0.0);s.store_mul_scale_offset_indices(846, 1892, 2255, -1.0, 0.0);s.store_scalar(2271, 0.0);s.store_scalar(2272, 0.0);s.store_scalar(2270, 0.0);s.b[2493] = ((s.v[263] > 0.0) || (s.v[264] > 0.0));s.store_scalar(2493, if s.b[2493] { 1.0 } else { 0.0 });
        if s.b[2493] {s.store_scalar(2260, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_58(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2493] {s.copy_ad(2259, 1875);}
        s.b[2494] = (s.v[267] > 1e-10);s.store_scalar(2494, if s.b[2494] { 1.0 } else { 0.0 });
        if (s.b[2493] && s.b[2494]) {s.store_add_scaled_inputs3_indices(2256, 1875, 1.0, 265, (-1.0), 802, 1.0);s.store_add_scaled_inputs3_sqrt_third_mixed_iia(1919, 2256, 0.5, 802, 0.5, A::add(A::square(A::sub(s.ad_value(2256), s.ad_value(802))), s.ad_value(803)), 0.5);s.store_mul_add_scaled_inputs3_offset_rhs_indices(1920, 1919, 1919, 2.0, 802, (-1.0), 2256, -1.0, 0.0);s.store_div(1921, 802, 1919);s.store_mul(2257, 2256, 1921);s.store_sqrt_sub_from_scalar_ad(2258, 1.0, A::mul(s.ad_value(2257), s.ad_value(267)));s.store_add_scaled_inputs3_mixed_aii(2259, A::div(A::sub_from_scalar(1.0, s.ad_value(2258)), s.ad_value(267)), 1.0, 2256, 1.0, 2257, -1.0);s.store_offset_ad(2260, A::div_scaled_product3(A::offset(A::div_from_scalar(0.5, s.ad_value(2258)), (-1.0)), A::add_scaled_product(s.ad_value(1920), 1.0, s.ad_value(2256), A::sub(s.ad_value(802), s.ad_value(1919)), 1.0), s.ad_value(1921), 1.0, s.ad_value(1920), 1.0), 1.0);}
        if s.b[2493] {s.store_scalar(2262, 1.0);s.store_scalar(2263, 0.0);}
        s.b[2495] = (s.v[266] > 0.0);s.store_scalar(2495, if s.b[2495] { 1.0 } else { 0.0 });
        if (s.b[2493] && s.b[2495]) {s.store_add_scaled_product_mixed_iia(1919, 739, 0.5, 1876, A::scale_offset(s.ad_value(1877), 0.7071067811865475, 1.0), 1.0);s.store_div(2261, 1875, 1919);}
        s.b[2496] = (((s.v[2261]) as f64).abs() < 230.25850929940458);s.store_scalar(2496, if s.b[2496] { 1.0 } else { 0.0 });
        if ((s.b[2493] && s.b[2495]) && s.b[2496]) {s.store_div_from_scalar_offset_ad(2262, 1.0, A::exp_scaled_input(s.ad_value(2261), -1.0), 1.0);}
        s.b[2497] = (s.v[2261] < 0.0);s.store_scalar(2497, if s.b[2497] { 1.0 } else { 0.0 });
        if (((s.b[2493] && s.b[2495]) && (!s.b[2496])) && s.b[2497]) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2262, 1e-100, 2261, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[2498] = (s.v[2261] < 230.25850929940458);s.store_scalar(2498, if s.b[2498] { 1.0 } else { 0.0 });
        if ((s.b[2493] && s.b[2495]) && s.b[2498]) {s.store_ln_one_plus_exp(1920, 2261);}
        if ((s.b[2493] && s.b[2495]) && (!s.b[2498])) {s.copy_ad(1920, 2261);}
        if (s.b[2493] && s.b[2495]) {s.store_mul(2263, 1919, 1920);}
        if s.b[2493] {s.store_add_scaled_product_right_sub(2264, 2260, 1.0, 266, 2262, 2260, 1.0);s.store_add_scaled_product_right_sub(2265, 2259, 1.0, 266, 2263, 2259, 1.0);s.store_add_scaled_inputs3_mixed_aii(2266, A::add_scaled_product(s.ad_value(1875), 1.0, s.ad_value(1876), s.ad_value(1879), (-1.0)), 1.0, 1891, (-1.0), 1881, (-0.5));s.store_add_scaled_inputs3_indices(2267, 1875, 1.0, 2266, (-1.0), 1880, -1.0);s.store_add_scaled_inputs3_indices(2268, 1881, 1.0, 2266, 1.0, 820, -1.0);s.store_add_scaled_inputs3_indices(2269, 1875, 1.0, 2268, (-1.0), 1882, -1.0);}
        s.b[2499] = (s.v[825] > 0.0);s.store_scalar(2499, if s.b[2499] { 1.0 } else { 0.0 });
        if (s.b[2493] && s.b[2499]) {s.store_mul_mixed_ia(2270, 2264, A::add_scaled_products(s.ad_value(264), s.ad_value(2268), 1.0, s.ad_value(263), s.ad_value(2266), 1.0));s.store_mul_sub_rhs(2271, 263, 2267, 2265);s.store_mul_sub_rhs(2272, 264, 2269, 2265);}
        if (s.b[2493] && (!s.b[2499])) {s.store_mul_mixed_ia(2270, 2264, A::add_scaled_products(s.ad_value(263), s.ad_value(2268), 1.0, s.ad_value(264), s.ad_value(2266), 1.0));s.store_mul_sub_rhs(2271, 264, 2267, 2265);s.store_mul_sub_rhs(2272, 263, 2269, 2265);}
        if s.b[2493] {s.store_add(845, 845, 2270);s.store_add(847, 847, 2272);s.store_add_scaled_inputs4_indices(846, 846, 1.0, 2270, (-1.0), 2272, -1.0, 2271, -1.0);}
        s.store_mul(1898, 257, 1866);s.store_mul(1899, 258, 1867);s.store_scalar(2275, 0.0);s.store_scalar(2273, 0.0);s.b[2500] = ((s.v[257] > 0.0) && (s.v[259] > 0.0));s.store_scalar(2500, if s.b[2500] { 1.0 } else { 0.0 });
        if s.b[2500] {s.store_mul_add_scaled_inputs_rhs_indices(1919, 261, 1807, 0.5, 781, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_59(
        s: &mut ReactiveScratch,
    ) {
        s.b[2501] = (s.v[1919] < 230.25850929940458);s.store_scalar(2501, if s.b[2501] { 1.0 } else { 0.0 });s.b[2502] = (s.v[1919] > (-230.25850929940458));s.store_scalar(2502, if s.b[2502] { 1.0 } else { 0.0 });
        if ((s.b[2500] && s.b[2501]) && s.b[2502]) {s.store_exp(2273, 1919);}
        if ((s.b[2500] && s.b[2501]) && (!s.b[2502])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2273, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2503] = (s.v[2273] > 1e-10);s.store_scalar(2503, if s.b[2503] { 1.0 } else { 0.0 });
        if ((s.b[2500] && s.b[2501]) && s.b[2503]) {s.store_ln_offset_input(2274, 2273, 1.0);s.store_mul_scale_offset_mixed_ia(1920, 2274, A::div(A::ln(A::offset(s.ad_value(2274), 1.0)), A::offset(s.ad_value(2274), 2.0)), -1.0, 1.0);}
        if ((s.b[2500] && s.b[2501]) && (!s.b[2503])) {s.copy_ad(2274, 2273);s.store_div_scaled_value_offset_denominator(1920, s.ad_value(2274), 2.0, s.ad_value(2274), 2.0, 1.0);}
        if (s.b[2500] && (!s.b[2501])) {s.copy_ad(2274, 1919);s.store_mul_scale_offset_mixed_ia(1920, 2274, A::div(A::ln(A::offset(s.ad_value(2274), 1.0)), A::offset(s.ad_value(2274), 2.0)), -1.0, 1.0);}
        if s.b[2500] {s.store_mul_ad_affine_product_lhs(2275, A::div_scaled_inputs(s.ad_value(259), (-2.0), s.ad_value(261), 1.0), s.ad_value(257), s.v[348], 0.0, 1920);}
        s.store_scalar(2278, 0.0);s.store_scalar(2276, 0.0);s.b[2504] = ((s.v[258] > 0.0) && (s.v[260] > 0.0));s.store_scalar(2504, if s.b[2504] { 1.0 } else { 0.0 });
        if s.b[2504] {s.store_mul_add_scaled_inputs_rhs_indices(1919, 261, 1807, 0.5, 782, 1.0);}
        s.b[2505] = (s.v[1919] < 230.25850929940458);s.store_scalar(2505, if s.b[2505] { 1.0 } else { 0.0 });s.b[2506] = (s.v[1919] > (-230.25850929940458));s.store_scalar(2506, if s.b[2506] { 1.0 } else { 0.0 });
        if ((s.b[2504] && s.b[2505]) && s.b[2506]) {s.store_exp(2276, 1919);}
        if ((s.b[2504] && s.b[2505]) && (!s.b[2506])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2276, 1e-100, (-230.25850929940458), 1919, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2507] = (s.v[2276] > 1e-10);s.store_scalar(2507, if s.b[2507] { 1.0 } else { 0.0 });
        if ((s.b[2504] && s.b[2505]) && s.b[2507]) {s.store_ln_offset_input(2277, 2276, 1.0);s.store_mul_scale_offset_mixed_ia(1920, 2277, A::div(A::ln(A::offset(s.ad_value(2277), 1.0)), A::offset(s.ad_value(2277), 2.0)), -1.0, 1.0);}
        if ((s.b[2504] && s.b[2505]) && (!s.b[2507])) {s.copy_ad(2277, 2276);s.store_div_scaled_value_offset_denominator(1920, s.ad_value(2277), 2.0, s.ad_value(2277), 2.0, 1.0);}
        if (s.b[2504] && (!s.b[2505])) {s.copy_ad(2277, 1919);s.store_mul_scale_offset_mixed_ia(1920, 2277, A::div(A::ln(A::offset(s.ad_value(2277), 1.0)), A::offset(s.ad_value(2277), 2.0)), -1.0, 1.0);}
        if s.b[2504] {s.store_mul_ad_affine_product_lhs(2278, A::div_scaled_inputs(s.ad_value(260), (-2.0), s.ad_value(261), 1.0), s.ad_value(258), s.v[348], 0.0, 1920);}
        s.store_add(2279, 2275, 2278);s.store_add_scaled_product_indices(850, 2279, 1.0, 262, 823, 1.0);s.store_mul(848, 269, 828);s.store_mul(849, 270, 831);s.store_scalar(2508, 0.0);s.store_scalar(2511, 0.0);s.store_scalar(2512, 0.0);s.store_scalar(2513, 0.0);s.store_scalar(2514, 0.0);s.store_scalar(2515, 0.0);s.store_scalar(2516, 0.0);s.store_scalar(2517, 0.0);s.store_scalar(2518, 0.0);s.store_scalar(2519, 0.0);s.store_scalar(2520, 0.0);s.store_scalar(2521, 0.0);s.store_scalar(2522, 0.0);s.store_scalar(2523, 0.0);s.store_scalar(2524, 0.0);s.store_scalar(2525, 0.0);s.store_scalar(2526, 0.0);s.store_scalar(2529, 0.0);s.store_scalar(2533, 0.0);s.store_scalar(2536, 0.0);s.store_scalar(2537, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_60(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.store_scalar(2538, 0.0);s.store_scalar(2539, 0.0);s.store_scalar(2540, 0.0);s.store_scalar(2541, 0.0);s.store_scalar(2544, 0.0);s.store_scalar(2545, 0.0);s.store_scalar(2546, 0.0);s.store_scalar(2547, 0.0);s.store_scalar(2551, 0.0);s.store_scalar(2553, 0.0);s.store_scalar(2554, 0.0);s.store_scalar(851, 0.0);s.store_scalar(1906, 0.0);s.store_scalar(1907, 0.0);s.store_scalar(1908, 0.0);s.store_scalar(852, 0.0);s.store_scalar(1909, 0.0);s.store_scalar(1910, 0.0);s.store_scalar(1911, 0.0);s.b[2555] = (p.p43 > 0.0);s.store_scalar(2555, if s.b[2555] { 1.0 } else { 0.0 });s.b[2556] = (s.v[468] == 1.0);s.store_scalar(2556, if s.b[2556] { 1.0 } else { 0.0 });
        if (s.b[2555] && s.b[2556]) {s.store_scalar(2559, 0.0);s.store_scalar(2560, 0.0);s.store_primal_scaled_mul(2511, 651, 651, 4.0);s.store_primal_div(2512, 651, 652);s.store_add_scaled_product_indices(2513, 826, 1.0, 651, 2512, 1.0);s.store_add(2514, 652, 2513);s.store_sub(2515, 652, 2513);s.store_sqrt_square_add(2516, 2515, 2511);s.store_div_scaled_product_add_scaled_denominator_indices(2560, 826, 652, 2.0, 2514, 1.0, 2516, 1.0, 1.0);}
        s.b[2561] = (s.v[645] > 0.5);s.store_scalar(2561, if s.b[2561] { 1.0 } else { 0.0 });s.b[2562] = (s.v[402] == 0.5);s.store_scalar(2562, if s.b[2562] { 1.0 } else { 0.0 });
        if (((s.b[2555] && s.b[2556]) && s.b[2561]) && s.b[2562]) {s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::scale(s.ad_value(2560), s.v[399]));}
        if (((s.b[2555] && s.b[2556]) && s.b[2561]) && (!s.b[2562])) {s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[399])), s.v[402]);}
        if ((s.b[2555] && s.b[2556]) && s.b[2561]) {s.store_add_scaled_inputs3_offset_indices(1906, 2559, (-s.v[411]), 826, s.v[414], 2560, (-s.v[414]), s.v[411]);}
        s.b[2563] = (s.v[646] > 0.5);s.store_scalar(2563, if s.b[2563] { 1.0 } else { 0.0 });s.b[2564] = (s.v[403] == 0.5);s.store_scalar(2564, if s.b[2564] { 1.0 } else { 0.0 });
        if (((s.b[2555] && s.b[2556]) && s.b[2563]) && s.b[2564]) {s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::scale(s.ad_value(2560), s.v[400]));}
        if (((s.b[2555] && s.b[2556]) && s.b[2563]) && (!s.b[2564])) {s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[400])), s.v[403]);}
        if ((s.b[2555] && s.b[2556]) && s.b[2563]) {s.store_add_scaled_inputs3_offset_indices(1907, 2559, (-s.v[412]), 826, s.v[415], 2560, (-s.v[415]), s.v[412]);}
        s.b[2565] = (s.v[647] > 0.5);s.store_scalar(2565, if s.b[2565] { 1.0 } else { 0.0 });s.b[2566] = (s.v[404] == 0.5);s.store_scalar(2566, if s.b[2566] { 1.0 } else { 0.0 });
        if (((s.b[2555] && s.b[2556]) && s.b[2565]) && s.b[2566]) {s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::scale(s.ad_value(2560), s.v[401]));}
        if (((s.b[2555] && s.b[2556]) && s.b[2565]) && (!s.b[2566])) {s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[401])), s.v[404]);}
        if ((s.b[2555] && s.b[2556]) && s.b[2565]) {s.store_add_scaled_inputs3_offset_indices(1908, 2559, (-s.v[413]), 826, s.v[416], 2560, (-s.v[416]), s.v[413]);}
        if (s.b[2555] && s.b[2556]) {s.store_scalar(2559, 0.0);s.store_scalar(2560, 0.0);s.store_primal_scaled_mul(2511, 678, 678, 4.0);s.store_primal_div(2512, 678, 679);s.store_add_scaled_product_indices(2513, 827, 1.0, 678, 2512, 1.0);s.store_add(2514, 679, 2513);s.store_sub(2515, 679, 2513);s.store_sqrt_square_add(2516, 2515, 2511);s.store_div_scaled_product_add_scaled_denominator_indices(2560, 827, 679, 2.0, 2514, 1.0, 2516, 1.0, 1.0);}
        s.b[2567] = (s.v[672] > 0.5);s.store_scalar(2567, if s.b[2567] { 1.0 } else { 0.0 });s.b[2568] = (s.v[569] == 0.5);s.store_scalar(2568, if s.b[2568] { 1.0 } else { 0.0 });
        if (((s.b[2555] && s.b[2556]) && s.b[2567]) && s.b[2568]) {s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::mul(s.ad_value(2560), s.ad_value(566)));}
        if (((s.b[2555] && s.b[2556]) && s.b[2567]) && (!s.b[2568])) {s.store_pow_sub_from_scalar_mul_base_indices(2559, 1.0, 2560, 566, 569);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_61(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[2555] && s.b[2556]) && s.b[2567]) {s.store_add_scaled_product_mixed_aia(1909, A::mul_sub_from_scalar_rhs(s.ad_value(578), 1.0, s.ad_value(2559)), 1.0, 581, A::sub(s.ad_value(827), s.ad_value(2560)), 1.0);}
        s.b[2569] = (s.v[673] > 0.5);s.store_scalar(2569, if s.b[2569] { 1.0 } else { 0.0 });s.b[2570] = (s.v[570] == 0.5);s.store_scalar(2570, if s.b[2570] { 1.0 } else { 0.0 });
        if (((s.b[2555] && s.b[2556]) && s.b[2569]) && s.b[2570]) {s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::mul(s.ad_value(2560), s.ad_value(567)));}
        if (((s.b[2555] && s.b[2556]) && s.b[2569]) && (!s.b[2570])) {s.store_pow_sub_from_scalar_mul_base_indices(2559, 1.0, 2560, 567, 570);}
        if ((s.b[2555] && s.b[2556]) && s.b[2569]) {s.store_add_scaled_product_mixed_aia(1910, A::mul_sub_from_scalar_rhs(s.ad_value(579), 1.0, s.ad_value(2559)), 1.0, 582, A::sub(s.ad_value(827), s.ad_value(2560)), 1.0);}
        s.b[2571] = (s.v[674] > 0.5);s.store_scalar(2571, if s.b[2571] { 1.0 } else { 0.0 });s.b[2572] = (s.v[571] == 0.5);s.store_scalar(2572, if s.b[2572] { 1.0 } else { 0.0 });
        if (((s.b[2555] && s.b[2556]) && s.b[2571]) && s.b[2572]) {s.store_sqrt_sub_from_scalar_ad(2559, 1.0, A::mul(s.ad_value(2560), s.ad_value(568)));}
        if (((s.b[2555] && s.b[2556]) && s.b[2571]) && (!s.b[2572])) {s.store_pow_sub_from_scalar_mul_base_indices(2559, 1.0, 2560, 568, 571);}
        if ((s.b[2555] && s.b[2556]) && s.b[2571]) {s.store_add_scaled_product_mixed_aia(1911, A::mul_sub_from_scalar_rhs(s.ad_value(580), 1.0, s.ad_value(2559)), 1.0, 583, A::sub(s.ad_value(827), s.ad_value(2560)), 1.0);}
        s.b[2573] = (p.p865 > 0.0);s.store_scalar(2573, if s.b[2573] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2573]) {s.store_scaled_offset_ad(636, A::powf(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt_square_offset(A::add(s.ad_value(819), s.ad_value(821)), (0.001 * 0.001)), 0.5), p.p866), (-(((0.5 * 0.001)) as f64).powf(p.p866)), p.p865);s.store_offset(634, 636, p.p855);s.store_div_from_scalar(444, 1.0, 634);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2573])) {s.store_scalar(634, p.p855);}
        s.b[2574] = (p.p867 > 0.0);s.store_scalar(2574, if s.b[2574] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2574]) {s.store_scaled_offset_ad(638, A::powf(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt_square_offset(A::add(s.ad_value(819), s.ad_value(821)), (0.001 * 0.001)), 0.5), p.p868), (-(((0.5 * 0.001)) as f64).powf(p.p868)), p.p867);s.store_mul_scale_offset_indices(437, 437, 638, 1.0, 1.0);}
        if (s.b[2555] && (!s.b[2556])) {s.store_scalar(2524, 0.0);s.store_scalar(2521, 0.0);}
        s.b[2575] = (!(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)));s.store_scalar(2575, if s.b[2575] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2575]) {s.store_primal_scaled_mul(2511, 651, 651, 4.0);s.store_primal_div(2512, 651, 652);s.store_add_scaled_product_indices(2513, 826, 1.0, 651, 2512, 1.0);s.store_add(2514, 652, 2513);s.store_sub(2515, 652, 2513);s.store_sqrt_square_add(2516, 2515, 2511);s.store_div_scaled_product_add_scaled_denominator_indices(2518, 826, 652, 2.0, 2514, 1.0, 2516, 1.0, 1.0);}
        s.b[2576] = (s.v[826] < s.v[648]);s.store_scalar(2576, if s.b[2576] { 1.0 } else { 0.0 });s.b[2577] = (((((-0.5) * (s.v[826] * s.v[365]))) as f64).abs() < 230.25850929940458);s.store_scalar(2577, if s.b[2577] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) && s.b[2577]) {s.store_exp_scaled_input(2519, 826, (s.v[365] * (-0.5)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_62(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2578] = (((-0.5) * (s.v[826] * s.v[365])) < 0.0);s.store_scalar(2578, if s.b[2578] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) && (!s.b[2577])) && s.b[2578]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2519, 1e-100, (-230.25850929940458), A::scale(s.ad_value(826), (s.v[365] * (-0.5))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) && (!s.b[2577])) && (!s.b[2578])) {s.store_scaled_offset_ad(2519, A::mul_offset_rhs(A::scale_offset(s.ad_value(826), (s.v[365] * (-0.5)), (-230.25850929940458)), A::mul_scaled_output(A::scale_offset(s.ad_value(826), (s.v[365] * (-0.5)), (-230.25850929940458)), A::scale_offset(s.ad_value(826), (((s.v[365] * (-0.5))) * (0.3333333333333333)), (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0))), 0.5), 1.0), 1.0, 1e100);}
        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2576]) {s.store_div_from_scalar(2520, 1.0, 2519);s.store_square(2517, 2520);}
        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && (!s.b[2576])) {s.store_mul_scale_offset_mixed_ia(2517, 649, A::sub_scaled_inputs(s.ad_value(826), s.v[365], s.ad_value(648), s.v[365]), 1.0, 1.0);s.store_sqrt(2520, 2517);s.store_div_from_scalar(2519, 1.0, 2520);}
        if ((s.b[2555] && (!s.b[2556])) && s.b[2575]) {s.store_offset(2517, 2517, (-1.0));}
        s.b[2579] = (s.v[826] > 0.0);s.store_scalar(2579, if s.b[2579] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && s.b[2579]) {s.store_scaled_ln_ad(2521, A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2519), 1.0, A::offset(s.ad_value(2519), 3.0)))), (s.v[364] * 2.0));}
        if (((s.b[2555] && (!s.b[2556])) && s.b[2575]) && (!s.b[2579])) {s.store_sub_mixed_ai(2521, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2520), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2520), 1.0, A::scale_offset(s.ad_value(2520), 3.0, 1.0))))), (s.v[364] * 2.0)), 826);}
        if ((s.b[2555] && (!s.b[2556])) && s.b[2575]) {s.store_sub(2522, 650, 2521);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2523, 826, 0.5, 2522, 0.5, 826, 2522, ((4.0 * s.v[364]) * s.v[364]), (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2524, 826, 0.5, 653, 0.5, 826, 653, ((4.0 * s.v[362]) * s.v[362]), (-0.5));s.store_scaled_sub_mixed_ia(2525, 826, A::sqrt_square_offset(s.ad_value(826), ((4.0 * 1e-6) * 1e-6)), 0.5);}
        s.b[2580] = (s.v[640] == 0.0);s.store_scalar(2580, if s.b[2580] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2580]) {s.store_scalar(1906, 0.0);}
        s.b[2581] = ((p.p833 == 0.0) && (p.p838 == 0.0));s.store_scalar(2581, if s.b[2581] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) {s.store_sub_from_scalar(2529, s.v[387], 2523);}
        s.b[2583] = (p.p824 == 0.5);s.store_scalar(2583, if s.b[2583] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) && s.b[2583]) {s.store_sqrt_scaled_input(2526, 2529, s.v[423]);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) && (!s.b[2583])) {s.store_powf_scaled_input(2526, 2529, s.v[423], p.p824);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2581])) {s.store_scale(2533, 2526, s.v[417]);}
        s.b[2584] = (p.p838 == 0.0);s.store_scalar(2584, if s.b[2584] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) {s.store_div_scaled_inputs_indices(2536, 2533, (s.v[402] * s.v[432]), 2529, 1.0);s.store_div_from_scalar(2537, (0.666666666666667 * s.v[429]), 2536);s.store_square(2538, 2537);s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);s.store_sqrt(2540, 2539);s.store_mul(2541, 2539, 2540);s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);s.store_add_scaled_value_products_indices(2546, 2539, (-s.v[429]), 2537, 2540, s.v[429], 2536, 2541, 0.5);s.store_mul_scale_offset_indices(2547, 2544, 2545, 1.0, (-1.0));s.store_square(2508, 2547);}
        s.b[2587] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));s.store_scalar(2587, if s.b[2587] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && s.b[2587]) {s.store_exp_sub(2526, 2546, 2508);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2587])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2588] = (s.v[2547] > 0.0);s.store_scalar(2588, if s.b[2588] { 1.0 } else { 0.0 });s.b[2589] = (s.v[2546] > (-230.25850929940458));s.store_scalar(2589, if s.b[2589] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2588])) && s.b[2589]) {s.store_exp(2526, 2546);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2584])) && (!s.b[2588])) && (!s.b[2589])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2590] = (p.p844 == 0.0);s.store_scalar(2590, if s.b[2590] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_63(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2591] = (p.p824 == 0.5);s.store_scalar(2591, if s.b[2591] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && s.b[2591]) {s.store_sqrt_scaled_input_ad(2526, A::sub_from_scalar(p.p821, s.ad_value(2524)), s.v[423]);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && (!s.b[2591])) {s.store_powf_scale_offset_input(2526, 2524, (-s.v[423]), ((p.p821) * (s.v[423])), p.p824);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) {s.store_div_scaled_offset_numerator_indices(2551, 2524, ((-s.v[420]) * s.v[405]), (((p.p821) * (s.v[420])) * s.v[405]), 2526, 1.0);}
        s.b[2592] = (((((-s.v[435]) / s.v[2551])) as f64).abs() < 230.25850929940458);s.store_scalar(2592, if s.b[2592] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && s.b[2592]) {s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(435), -1.0, s.ad_value(2551), 1.0));}
        s.b[2593] = (((-s.v[435]) / s.v[2551]) < 0.0);s.store_scalar(2593, if s.b[2593] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && (!s.b[2592])) && s.b[2593]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 435, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2590])) && (!s.b[2592])) && (!s.b[2593])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 435, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2594] = (p.p853 > 1000.0);s.store_scalar(2594, if s.b[2594] { 1.0 } else { 0.0 });s.b[2595] = (s.v[2525] > ((-s.v[438]) * p.p853));s.store_scalar(2595, if s.b[2595] { 1.0 } else { 0.0 });s.b[2596] = (p.p856 == 4.0);s.store_scalar(2596, if s.b[2596] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2594])) && s.b[2595]) && s.b[2596]) {s.store_mul_scale_offset_mixed_ai(2526, A::mul3_scaled_output(s.ad_value(2525), s.ad_value(2525), s.ad_value(2525), ((s.v[442] * s.v[442]) * s.v[442])), 2525, s.v[442], 0.0);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2594])) && s.b[2595]) && (!s.b[2596])) {s.store_powf_ad(2526, A::abs_scaled_input(s.ad_value(2525), s.v[442]), p.p856);}
        s.b[2597] = (s.v[402] == 0.5);s.store_scalar(2597, if s.b[2597] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && s.b[2597]) {s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[399]));}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) && (!s.b[2597])) {s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[399])), s.v[402]);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2580])) {s.store_add_scaled_inputs3_offset_indices(1906, 2526, ((-s.v[411]) * p.p30), 826, (s.v[414] * p.p30), 2518, ((-s.v[414]) * p.p30), (s.v[411] * p.p30));}
        s.b[2598] = (s.v[641] == 0.0);s.store_scalar(2598, if s.b[2598] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2598]) {s.store_scalar(1907, 0.0);}
        s.b[2599] = ((p.p834 == 0.0) && (p.p839 == 0.0));s.store_scalar(2599, if s.b[2599] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) {s.store_sub_from_scalar(2529, s.v[388], 2523);}
        s.b[2601] = (p.p825 == 0.5);s.store_scalar(2601, if s.b[2601] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) && s.b[2601]) {s.store_sqrt_scaled_input(2526, 2529, s.v[424]);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) && (!s.b[2601])) {s.store_powf_scaled_input(2526, 2529, s.v[424], p.p825);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2599])) {s.store_scale(2533, 2526, s.v[418]);}
        s.b[2602] = (p.p839 == 0.0);s.store_scalar(2602, if s.b[2602] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) {s.store_div_scaled_inputs_indices(2536, 2533, (s.v[403] * s.v[433]), 2529, 1.0);s.store_div_from_scalar(2537, (0.666666666666667 * s.v[430]), 2536);s.store_square(2538, 2537);s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);s.store_sqrt(2540, 2539);s.store_mul(2541, 2539, 2540);s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);s.store_add_scaled_value_products_indices(2546, 2539, (-s.v[430]), 2537, 2540, s.v[430], 2536, 2541, 0.5);s.store_mul_scale_offset_indices(2547, 2544, 2545, 1.0, (-1.0));s.store_square(2508, 2547);}
        s.b[2605] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));s.store_scalar(2605, if s.b[2605] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && s.b[2605]) {s.store_exp_sub(2526, 2546, 2508);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2605])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2606] = (s.v[2547] > 0.0);s.store_scalar(2606, if s.b[2606] { 1.0 } else { 0.0 });s.b[2607] = (s.v[2546] > (-230.25850929940458));s.store_scalar(2607, if s.b[2607] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2606])) && s.b[2607]) {s.store_exp(2526, 2546);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2602])) && (!s.b[2606])) && (!s.b[2607])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2608] = (p.p845 == 0.0);s.store_scalar(2608, if s.b[2608] { 1.0 } else { 0.0 });s.b[2609] = (p.p825 == 0.5);s.store_scalar(2609, if s.b[2609] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && s.b[2609]) {s.store_sqrt_scaled_input_ad(2526, A::sub_from_scalar(p.p822, s.ad_value(2524)), s.v[424]);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2609])) {s.store_powf_scale_offset_input(2526, 2524, (-s.v[424]), ((p.p822) * (s.v[424])), p.p825);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) {s.store_div_scaled_offset_numerator_indices(2551, 2524, ((-s.v[421]) * s.v[406]), (((p.p822) * (s.v[421])) * s.v[406]), 2526, 1.0);}
        s.b[2610] = (((((-s.v[436]) / s.v[2551])) as f64).abs() < 230.25850929940458);s.store_scalar(2610, if s.b[2610] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && s.b[2610]) {s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(436), -1.0, s.ad_value(2551), 1.0));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_64(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2611] = (((-s.v[436]) / s.v[2551]) < 0.0);s.store_scalar(2611, if s.b[2611] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2610])) && s.b[2611]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 436, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2608])) && (!s.b[2610])) && (!s.b[2611])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 436, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2612] = (p.p854 > 1000.0);s.store_scalar(2612, if s.b[2612] { 1.0 } else { 0.0 });s.b[2613] = (s.v[2525] > ((-s.v[438]) * p.p854));s.store_scalar(2613, if s.b[2613] { 1.0 } else { 0.0 });s.b[2614] = (p.p857 == 4.0);s.store_scalar(2614, if s.b[2614] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && s.b[2613]) && s.b[2614]) {s.store_mul_scale_offset_mixed_ai(2526, A::mul3_scaled_output(s.ad_value(2525), s.ad_value(2525), s.ad_value(2525), ((s.v[443] * s.v[443]) * s.v[443])), 2525, s.v[443], 0.0);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2612])) && s.b[2613]) && (!s.b[2614])) {s.store_powf_ad(2526, A::abs_scaled_input(s.ad_value(2525), s.v[443]), p.p857);}
        s.b[2615] = (s.v[403] == 0.5);s.store_scalar(2615, if s.b[2615] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && s.b[2615]) {s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[400]));}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) && (!s.b[2615])) {s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[400])), s.v[403]);}
        if ((s.b[2555] && (!s.b[2556])) && (!s.b[2598])) {s.store_add_scaled_inputs3_offset_indices(1907, 2526, ((-s.v[412]) * p.p30), 826, (s.v[415] * p.p30), 2518, ((-s.v[415]) * p.p30), (s.v[412] * p.p30));}
        s.b[2616] = (s.v[642] == 0.0);s.store_scalar(2616, if s.b[2616] { 1.0 } else { 0.0 });
        if ((s.b[2555] && (!s.b[2556])) && s.b[2616]) {s.store_scalar(1908, 0.0);}
        s.b[2617] = ((p.p835 == 0.0) && (p.p840 == 0.0));s.store_scalar(2617, if s.b[2617] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) {s.store_sub_from_scalar(2529, s.v[389], 2523);}
        s.b[2619] = (p.p826 == 0.5);s.store_scalar(2619, if s.b[2619] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && s.b[2619]) {s.store_sqrt_scaled_input(2526, 2529, s.v[425]);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) && (!s.b[2619])) {s.store_powf_scaled_input(2526, 2529, s.v[425], p.p826);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2617])) {s.store_scale(2533, 2526, s.v[419]);}
        s.b[2620] = (p.p840 == 0.0);s.store_scalar(2620, if s.b[2620] { 1.0 } else { 0.0 });
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) {s.store_div_scaled_inputs_indices(2536, 2533, (s.v[404] * s.v[434]), 2529, 1.0);s.store_div_from_scalar(2537, (0.666666666666667 * s.v[431]), 2536);s.store_square(2538, 2537);s.store_sqrt_div_scaled_square_offset_denominator(2539, 2538, 1.0, 1.0, 1.0);s.store_sqrt(2540, 2539);s.store_mul(2541, 2539, 2540);s.store_sqrt_scaled_input_ad(2544, A::div(s.ad_value(2536), s.ad_value(2540)), 0.375);s.store_add_scaled_product_indices(2545, 2539, (-1.0), 2537, 2540, 2.0);s.store_add_scaled_value_products_indices(2546, 2539, (-s.v[431]), 2537, 2540, s.v[431], 2536, 2541, 0.5);s.store_mul_scale_offset_indices(2547, 2544, 2545, 1.0, (-1.0));s.store_square(2508, 2547);}
        s.b[2623] = (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458));s.store_scalar(2623, if s.b[2623] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && s.b[2623]) {s.store_exp_sub(2526, 2546, 2508);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2623])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2526, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2624] = (s.v[2547] > 0.0);s.store_scalar(2624, if s.b[2624] { 1.0 } else { 0.0 });s.b[2625] = (s.v[2546] > (-230.25850929940458));s.store_scalar(2625, if s.b[2625] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2624])) && s.b[2625]) {s.store_exp(2526, 2546);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2620])) && (!s.b[2624])) && (!s.b[2625])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 2546, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2626] = (p.p846 == 0.0);s.store_scalar(2626, if s.b[2626] { 1.0 } else { 0.0 });s.b[2627] = (p.p826 == 0.5);s.store_scalar(2627, if s.b[2627] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && s.b[2627]) {s.store_sqrt_scaled_input_ad(2526, A::sub_from_scalar(p.p823, s.ad_value(2524)), s.v[425]);}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2627])) {s.store_powf_scale_offset_input(2526, 2524, (-s.v[425]), ((p.p823) * (s.v[425])), p.p826);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) {s.store_div_scaled_offset_numerator_indices(2551, 2524, ((-s.v[422]) * s.v[407]), (((p.p823) * (s.v[422])) * s.v[407]), 2526, 1.0);}
        s.b[2628] = (((((-s.v[437]) / s.v[2551])) as f64).abs() < 230.25850929940458);s.store_scalar(2628, if s.b[2628] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && s.b[2628]) {s.store_ad_value(2526, A::exp_div_scaled_inputs(s.ad_value(437), -1.0, s.ad_value(2551), 1.0));}
        s.b[2629] = (((-s.v[437]) / s.v[2551]) < 0.0);s.store_scalar(2629, if s.b[2629] { 1.0 } else { 0.0 });
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2628])) && s.b[2629]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_div_scaled_inputs_self_offset_rhs(2526, 1e-100, (-230.25850929940458), 437, -1.0, 2551, 1.0, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2626])) && (!s.b[2628])) && (!s.b[2629])) {s.store_scaled_softlimit_poly_offset_lhs_div_scaled_inputs(2526, 437, -1.0, 2551, 1.0, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        s.b[2630] = (s.v[634] > 1000.0);s.store_scalar(2630, if s.b[2630] { 1.0 } else { 0.0 });s.b[2631] = (s.v[2525] > ((-s.v[438]) * s.v[634]));s.store_scalar(2631, if s.b[2631] { 1.0 } else { 0.0 });s.b[2632] = (p.p858 == 4.0);s.store_scalar(2632, if s.b[2632] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_65(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && s.b[2631]) && s.b[2632]) {s.store_mul_ad_product_lhs_mixed_ai(2526, A::mul3(A::square(A::mul(s.ad_value(2525), s.ad_value(444))), s.ad_value(2525), s.ad_value(444)), 2525, 444);}
        if (((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2630])) && s.b[2631]) && (!s.b[2632])) {s.store_powf_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(444))), p.p858);}
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
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && s.b[2633]) {s.store_add_scaled_product_mixed_aia(466, A::mul_sub_from_scalar_rhs(s.ad_value(464), 1.0, s.ad_value(2526)), p.p30, 465, A::sub(s.ad_value(2553), s.ad_value(2554)), p.p30);s.store_add(1908, 1908, 466);}
        s.b[2636] = (s.v[404] == 0.5);s.store_scalar(2636, if s.b[2636] { 1.0 } else { 0.0 });
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) && s.b[2636]) {s.store_sqrt_sub_from_scalar_ad(2526, 1.0, A::scale(s.ad_value(2518), s.v[401]));}
        if ((((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) && (!s.b[2636])) {s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[401])), s.v[404]);}
        if (((s.b[2555] && (!s.b[2556])) && (!s.b[2616])) && (!s.b[2633])) {s.store_add_scaled_inputs3_offset_indices(1908, 2526, ((-s.v[413]) * p.p30), 826, (s.v[416] * p.p30), 2518, ((-s.v[416]) * p.p30), (s.v[413] * p.p30));}
        s.b[2637] = (s.v[630] > 0.0);s.store_scalar(2637, if s.b[2637] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_66(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2555] && (!s.b[2556])) && s.b[2637]) {s.store_mul_sub_mixed_iaa(637, 630, A::pow(A::add_scaled_inputs3(s.ad_value(819), 0.5, s.ad_value(821), 0.5, A::sqrt_square_offset(A::add(s.ad_value(819), s.ad_value(821)), (0.001 * 0.001)), 0.5), s.ad_value(631)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(631)));s.store_add(635, 536, 637);s.store_div_from_scalar(610, 1.0, 635);}
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
        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && s.b[2643]) {s.store_scaled_ln_ad(2521, A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2519), 1.0, A::offset(s.ad_value(2519), 3.0)))), (s.v[364] * 2.0));}
        if (((s.b[2555] && (!s.b[2556])) && s.b[2639]) && (!s.b[2643])) {s.store_sub_mixed_ai(2521, A::scale(A::ln(A::add(A::scale_offset(s.ad_value(2520), 2.0, 1.0), A::sqrt(A::mul_offset_lhs(s.ad_value(2520), 1.0, A::scale_offset(s.ad_value(2520), 3.0, 1.0))))), (s.v[364] * 2.0)), 827);}
        if ((s.b[2555] && (!s.b[2556])) && s.b[2639]) {s.store_sub(2522, 677, 2521);}
    }
}
