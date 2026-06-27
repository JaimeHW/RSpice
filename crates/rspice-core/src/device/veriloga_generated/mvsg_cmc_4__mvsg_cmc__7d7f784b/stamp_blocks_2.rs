#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};

impl Instance {

    pub(super) fn stamp_transient_block_32(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            s.store_scalar(2344, 0.0);
            s.store_scalar(2345, 0.0);
            s.store_scalar(2346, 0.0);
            s.store_scalar(2347, 0.0);
            s.store_scalar(2348, 0.0);
            s.store_scalar(2349, 0.0);
            s.store_scalar(2350, 0.0);
            s.store_scalar(2351, 0.0);
            s.store_scalar(2352, 0.0);
            s.store_scalar(2353, 0.0);
            s.store_mul_scaled_ad_lhs(2333, A::div(s.ad_value(2318), s.ad_value(2302)), 2319, -1.0);
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            if ((!(s.v[2333] > 50.0)) && (!(s.v[2333] < (-50.0)))) {
                s.store_exp(2323, 2333);
            } else {
                if ((!(s.v[2333] > 50.0)) && (s.v[2333] < (-50.0))) {
                    s.store_scalar(2323, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2333] > 50.0) {
                        s.store_scaled_offset(2323, 2333, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2323, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            s.store_add_scaled_product_right_ad(2329, 2333, 1.0, 2307, A::sub_scaled_inputs(s.ad_value(2301), -1.0, s.ad_value(2308), 1.0), 1.0);
            s.store_add_scaled_product_indices(2330, 2333, 1.0, 2307, 2308, -1.0);
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            if ((!(s.v[2329] > 50.0)) && (!(s.v[2329] < (-50.0)))) {
                s.store_exp(2331, 2329);
            } else {
                if ((!(s.v[2329] > 50.0)) && (s.v[2329] < (-50.0))) {
                    s.store_scalar(2331, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2329] > 50.0) {
                        s.store_scaled_offset(2331, 2329, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2331, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            if ((!(s.v[2330] > 50.0)) && (!(s.v[2330] < (-50.0)))) {
                s.store_exp(2332, 2330);
            } else {
                if ((!(s.v[2330] > 50.0)) && (s.v[2330] < (-50.0))) {
                    s.store_scalar(2332, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2330] > 50.0) {
                        s.store_scaled_offset(2332, 2330, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2332, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            s.store_sub(2325, 2331, 2332);
            s.store_mul_ad_product_lhs(2299, A::mul3(s.ad_value(2320), s.ad_value(2310), s.ad_value(2311)), s.ad_value(2312), 2309);
            s.store_add_scaled_product_left_ad(2335, 2333, 1.0, A::div(s.ad_value(2306), s.ad_value(2302)), 2301, 1.0);
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            if ((!(s.v[2335] > 50.0)) && (!(s.v[2335] < (-50.0)))) {
                s.store_exp(2336, 2335);
            } else {
                if ((!(s.v[2335] > 50.0)) && (s.v[2335] < (-50.0))) {
                    s.store_scalar(2336, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2335] > 50.0) {
                        s.store_scaled_offset(2336, 2335, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2336, 0.0);
                    }
                }
            }
        }

        s.b[2354] = (s.v[2305] == 1.0);
        s.v[2354] = if s.b[2354] { 1.0 } else { 0.0 };

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && s.b[2354]) {
            s.store_mul_sub_ad_rhs(2326, 2299, A::add_scaled_product(s.ad_value(2336), 1.0, s.ad_value(2313), s.ad_value(2325), (-1.0)), s.ad_value(2323));
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) {
            s.store_add_scaled_product_right_ad(2340, 2333, 1.0, 2307, A::sub_scaled_inputs(s.ad_value(2303), -1.0, s.ad_value(2308), 1.0), 1.0);
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) {
            if ((!(s.v[2340] > 50.0)) && (!(s.v[2340] < (-50.0)))) {
                s.store_exp(2341, 2340);
            } else {
                if ((!(s.v[2340] > 50.0)) && (s.v[2340] < (-50.0))) {
                    s.store_scalar(2341, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2340] > 50.0) {
                        s.store_scaled_offset(2341, 2340, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2341, 0.0);
                    }
                }
            }
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) {
            s.store_sub(2342, 2341, 2332);
            s.store_add_scaled_product_left_ad(2343, 2333, 1.0, A::div(s.ad_value(2306), s.ad_value(2302)), 2303, 1.0);
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) {
            if ((!(s.v[2343] > 50.0)) && (!(s.v[2343] < (-50.0)))) {
                s.store_exp(2344, 2343);
            } else {
                if ((!(s.v[2343] > 50.0)) && (s.v[2343] < (-50.0))) {
                    s.store_scalar(2344, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2343] > 50.0) {
                        s.store_scaled_offset(2344, 2343, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2344, 0.0);
                    }
                }
            }
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) {
            s.store_sub_ad_lhs(2345, A::add_scaled_product(s.ad_value(2344), 1.0, s.ad_value(2313), s.ad_value(2342), (-1.0)), 2323);
            s.store_mul_sub_ad_rhs(2346, 2299, A::add_scaled_product(s.ad_value(2336), 1.0, s.ad_value(2313), s.ad_value(2325), (-1.0)), s.ad_value(2323));
        }

        s.b[2355] = (s.v[2305] > 0.0);
        s.v[2355] = if s.b[2355] { 1.0 } else { 0.0 };

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) && s.b[2355]) {
            s.store_mul(2339, 2305, 2306);
            s.store_add_scaled_product_left_ad(2347, 2333, 1.0, A::div(s.ad_value(2339), s.ad_value(2302)), 2303, 1.0);
        }

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) && s.b[2355]) {
            if ((!(s.v[2347] > 50.0)) && (!(s.v[2347] < (-50.0)))) {
                s.store_exp(2348, 2347);
            } else {
                if ((!(s.v[2347] > 50.0)) && (s.v[2347] < (-50.0))) {
                    s.store_scalar(2348, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2347] > 50.0) {
                        s.store_scaled_offset(2348, 2347, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2348, 0.0);
                    }
                }
            }
        }

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) && s.b[2355]) {
            s.store_sub_ad_lhs(2349, A::add_scaled_product(s.ad_value(2348), 1.0, s.ad_value(2313), s.ad_value(2342), (-1.0)), 2323);
            s.store_add_scaled_product_left_ad(2350, 2333, 1.0, A::div(s.ad_value(2339), s.ad_value(2302)), 2301, 1.0);
        }

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) && s.b[2355]) {
            if ((!(s.v[2350] > 50.0)) && (!(s.v[2350] < (-50.0)))) {
                s.store_exp(2351, 2350);
            } else {
                if ((!(s.v[2350] > 50.0)) && (s.v[2350] < (-50.0))) {
                    s.store_scalar(2351, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2350] > 50.0) {
                        s.store_scaled_offset(2351, 2350, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2351, 0.0);
                    }
                }
            }
        }

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) && s.b[2355]) {
            s.store_div_scaled_product_indices(2352, 2299, 2345, 1.0, 2349, 1.0);
            s.store_mul_sub_ad_rhs(2353, 2352, A::add_scaled_product(s.ad_value(2351), 1.0, s.ad_value(2313), s.ad_value(2325), (-1.0)), s.ad_value(2323));
        }

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) && (!s.b[2355])) {
            s.store_mul(2353, 2299, 2345);
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) {
            s.store_mul_square_lhs(2322, 2304, 2302);
            s.store_div_scaled_inputs3(2334, s.ad_value(2301), 1.0, s.ad_value(2303), -1.0, s.ad_value(2322), (-(-0.5)), s.ad_value(2322), 1.0);
        }

        s.b[2356] = (s.v[2334] > 50.0);
        s.v[2356] = if s.b[2356] { 1.0 } else { 0.0 };

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) && s.b[2356]) {
            s.store_scalar(2324, 0.0);
        }

        s.b[2357] = (s.v[2334] < (-50.0));
        s.v[2357] = if s.b[2357] { 1.0 } else { 0.0 };

        if (((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) && (!s.b[2356])) && s.b[2357]) {
            s.store_scalar(2324, 1.0);
        }

        if (((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) && (!s.b[2356])) && (!s.b[2357])) {
            s.store_div_from_scalar_offset_ad(2324, 1.0, A::exp(s.ad_value(2334)), 1.0);
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2354])) {
            s.store_add_scaled_product_value_ad(2326, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2324), s.ad_value(2353)), 1.0, 2324, 2346, 1.0);
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            let assign40200_ad_e38295: A = {
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2301), s.ad_value(2314)), A::tanh_scaled_input(A::div(s.ad_value(2301), s.ad_value(2314)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(2301), s.ad_value(2314)), A::div(s.ad_value(2301), s.ad_value(2314))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs(2327, s.ad_value(2301), -1.0, A::pow(A::offset(A::pow(assign40200_ad_e38295, s.ad_value(2315)), 1.0), A::div_from_scalar(1.0, s.ad_value(2315))), 1.0);
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            s.store_mul_ad_product_lhs(2300, A::mul3_scaled_output(s.ad_value(2320), s.ad_value(2310), s.ad_value(2311), -1.0), s.ad_value(2316), 2309);
            s.store_mul_div_lhs(2337, 2317, 2302, 2327);
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            if ((!(s.v[2337] > 50.0)) && (!(s.v[2337] < (-50.0)))) {
                s.store_exp(2338, 2337);
            } else {
                if ((!(s.v[2337] > 50.0)) && (s.v[2337] < (-50.0))) {
                    s.store_scalar(2338, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2337] > 50.0) {
                        s.store_scaled_offset(2338, 2337, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2338, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            s.store_mul_offset_rhs(2328, 2300, 2338, (-1.0));
            s.store_add(2321, 2326, 2328);
            s.copy_ad(2298, 2321);
            s.copy_ad(140, 2298);
            s.store_scalar(2358, 0.0);
            s.store_scalar(2359, 0.0);
            s.store_scalar(2360, 0.0);
            s.store_scaled_voltage(2361, ctx, nodes, Some(8), Some(5), p.p6);
            s.copy_ad(2362, 113);
            s.store_scalar(2363, p.p265);
            s.store_scalar(2364, p.p267);
            s.store_scalar(2365, 1.0);
            s.store_scalar(2366, p.p263);
            s.store_scalar(2367, p.p281);
            s.store_scalar(2368, p.p280);
            s.copy_ad(2369, 112);
            s.store_scalar(2370, p.p0);
            s.store_scalar(2371, p.p2);
            s.store_scalar(2372, 0.0);
            s.store_scalar(2373, 0.0);
            s.store_scalar(2374, p.p289);
            s.store_scalar(2375, p.p290);
            s.store_scalar(2376, (p.p255 * p.p288));
            s.store_scalar(2377, p.p287);
            s.store_scalar(2378, p.p257);
            s.store_scalar(2379, p.p256);
            s.store_scalar(2380, p.p6);
            s.store_scalar(2381, 0.0);
            s.store_scalar(2382, 0.0);
            s.store_scalar(2383, 0.0);
            s.store_scalar(2384, 0.0);
            s.store_scalar(2385, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_33(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            s.store_scalar(2386, 0.0);
            s.store_scalar(2387, 0.0);
            s.store_scalar(2388, 0.0);
            s.store_scalar(2389, 0.0);
            s.store_scalar(2390, 0.0);
            s.store_scalar(2391, 0.0);
            s.store_scalar(2392, 0.0);
            s.store_scalar(2393, 0.0);
            s.store_scalar(2394, 0.0);
            s.store_scalar(2395, 0.0);
            s.store_scalar(2396, 0.0);
            s.store_scalar(2397, 0.0);
            s.store_scalar(2398, 0.0);
            s.store_scalar(2399, 0.0);
            s.store_scalar(2400, 0.0);
            s.store_scalar(2401, 0.0);
            s.store_scalar(2402, 0.0);
            s.store_scalar(2403, 0.0);
            s.store_scalar(2404, 0.0);
            s.store_scalar(2405, 0.0);
            s.store_scalar(2406, 0.0);
            s.store_scalar(2407, 0.0);
            s.store_scalar(2408, 0.0);
            s.store_scalar(2409, 0.0);
            s.store_scalar(2410, 0.0);
            s.store_scalar(2411, 0.0);
            s.store_scalar(2412, 0.0);
            s.store_scalar(2413, 0.0);
            s.store_mul_scaled_ad_lhs(2393, A::div(s.ad_value(2378), s.ad_value(2362)), 2379, -1.0);
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            if ((!(s.v[2393] > 50.0)) && (!(s.v[2393] < (-50.0)))) {
                s.store_exp(2383, 2393);
            } else {
                if ((!(s.v[2393] > 50.0)) && (s.v[2393] < (-50.0))) {
                    s.store_scalar(2383, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2393] > 50.0) {
                        s.store_scaled_offset(2383, 2393, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2383, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            s.store_add_scaled_product_right_ad(2389, 2393, 1.0, 2367, A::sub_scaled_inputs(s.ad_value(2361), -1.0, s.ad_value(2368), 1.0), 1.0);
            s.store_add_scaled_product_indices(2390, 2393, 1.0, 2367, 2368, -1.0);
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            if ((!(s.v[2389] > 50.0)) && (!(s.v[2389] < (-50.0)))) {
                s.store_exp(2391, 2389);
            } else {
                if ((!(s.v[2389] > 50.0)) && (s.v[2389] < (-50.0))) {
                    s.store_scalar(2391, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2389] > 50.0) {
                        s.store_scaled_offset(2391, 2389, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2391, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            if ((!(s.v[2390] > 50.0)) && (!(s.v[2390] < (-50.0)))) {
                s.store_exp(2392, 2390);
            } else {
                if ((!(s.v[2390] > 50.0)) && (s.v[2390] < (-50.0))) {
                    s.store_scalar(2392, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2390] > 50.0) {
                        s.store_scaled_offset(2392, 2390, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2392, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            s.store_sub(2385, 2391, 2392);
            s.store_mul_ad_product_lhs(2359, A::mul3(s.ad_value(2380), s.ad_value(2370), s.ad_value(2371)), s.ad_value(2372), 2369);
            s.store_add_scaled_product_left_ad(2395, 2393, 1.0, A::div(s.ad_value(2366), s.ad_value(2362)), 2361, 1.0);
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            if ((!(s.v[2395] > 50.0)) && (!(s.v[2395] < (-50.0)))) {
                s.store_exp(2396, 2395);
            } else {
                if ((!(s.v[2395] > 50.0)) && (s.v[2395] < (-50.0))) {
                    s.store_scalar(2396, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2395] > 50.0) {
                        s.store_scaled_offset(2396, 2395, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2396, 0.0);
                    }
                }
            }
        }

        s.b[2414] = (s.v[2365] == 1.0);
        s.v[2414] = if s.b[2414] { 1.0 } else { 0.0 };

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && s.b[2414]) {
            s.store_mul_sub_ad_rhs(2386, 2359, A::add_scaled_product(s.ad_value(2396), 1.0, s.ad_value(2373), s.ad_value(2385), (-1.0)), s.ad_value(2383));
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) {
            s.store_add_scaled_product_right_ad(2400, 2393, 1.0, 2367, A::sub_scaled_inputs(s.ad_value(2363), -1.0, s.ad_value(2368), 1.0), 1.0);
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) {
            if ((!(s.v[2400] > 50.0)) && (!(s.v[2400] < (-50.0)))) {
                s.store_exp(2401, 2400);
            } else {
                if ((!(s.v[2400] > 50.0)) && (s.v[2400] < (-50.0))) {
                    s.store_scalar(2401, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2400] > 50.0) {
                        s.store_scaled_offset(2401, 2400, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2401, 0.0);
                    }
                }
            }
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) {
            s.store_sub(2402, 2401, 2392);
            s.store_add_scaled_product_left_ad(2403, 2393, 1.0, A::div(s.ad_value(2366), s.ad_value(2362)), 2363, 1.0);
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) {
            if ((!(s.v[2403] > 50.0)) && (!(s.v[2403] < (-50.0)))) {
                s.store_exp(2404, 2403);
            } else {
                if ((!(s.v[2403] > 50.0)) && (s.v[2403] < (-50.0))) {
                    s.store_scalar(2404, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2403] > 50.0) {
                        s.store_scaled_offset(2404, 2403, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2404, 0.0);
                    }
                }
            }
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) {
            s.store_sub_ad_lhs(2405, A::add_scaled_product(s.ad_value(2404), 1.0, s.ad_value(2373), s.ad_value(2402), (-1.0)), 2383);
            s.store_mul_sub_ad_rhs(2406, 2359, A::add_scaled_product(s.ad_value(2396), 1.0, s.ad_value(2373), s.ad_value(2385), (-1.0)), s.ad_value(2383));
        }

        s.b[2415] = (s.v[2365] > 0.0);
        s.v[2415] = if s.b[2415] { 1.0 } else { 0.0 };

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) && s.b[2415]) {
            s.store_mul(2399, 2365, 2366);
            s.store_add_scaled_product_left_ad(2407, 2393, 1.0, A::div(s.ad_value(2399), s.ad_value(2362)), 2363, 1.0);
        }

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) && s.b[2415]) {
            if ((!(s.v[2407] > 50.0)) && (!(s.v[2407] < (-50.0)))) {
                s.store_exp(2408, 2407);
            } else {
                if ((!(s.v[2407] > 50.0)) && (s.v[2407] < (-50.0))) {
                    s.store_scalar(2408, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2407] > 50.0) {
                        s.store_scaled_offset(2408, 2407, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2408, 0.0);
                    }
                }
            }
        }

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) && s.b[2415]) {
            s.store_sub_ad_lhs(2409, A::add_scaled_product(s.ad_value(2408), 1.0, s.ad_value(2373), s.ad_value(2402), (-1.0)), 2383);
            s.store_add_scaled_product_left_ad(2410, 2393, 1.0, A::div(s.ad_value(2399), s.ad_value(2362)), 2361, 1.0);
        }

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) && s.b[2415]) {
            if ((!(s.v[2410] > 50.0)) && (!(s.v[2410] < (-50.0)))) {
                s.store_exp(2411, 2410);
            } else {
                if ((!(s.v[2410] > 50.0)) && (s.v[2410] < (-50.0))) {
                    s.store_scalar(2411, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2410] > 50.0) {
                        s.store_scaled_offset(2411, 2410, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2411, 0.0);
                    }
                }
            }
        }

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) && s.b[2415]) {
            s.store_div_scaled_product_indices(2412, 2359, 2405, 1.0, 2409, 1.0);
            s.store_mul_sub_ad_rhs(2413, 2412, A::add_scaled_product(s.ad_value(2411), 1.0, s.ad_value(2373), s.ad_value(2385), (-1.0)), s.ad_value(2383));
        }

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) && (!s.b[2415])) {
            s.store_mul(2413, 2359, 2405);
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) {
            s.store_mul_square_lhs(2382, 2364, 2362);
            s.store_div_scaled_inputs3(2394, s.ad_value(2361), 1.0, s.ad_value(2363), -1.0, s.ad_value(2382), (-(-0.5)), s.ad_value(2382), 1.0);
        }

        s.b[2416] = (s.v[2394] > 50.0);
        s.v[2416] = if s.b[2416] { 1.0 } else { 0.0 };

        if ((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) && s.b[2416]) {
            s.store_scalar(2384, 0.0);
        }

        s.b[2417] = (s.v[2394] < (-50.0));
        s.v[2417] = if s.b[2417] { 1.0 } else { 0.0 };

        if (((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) && (!s.b[2416])) && s.b[2417]) {
            s.store_scalar(2384, 1.0);
        }

        if (((((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) && (!s.b[2416])) && (!s.b[2417])) {
            s.store_div_from_scalar_offset_ad(2384, 1.0, A::exp(s.ad_value(2394)), 1.0);
        }

        if (((s.b[1934] && s.b[2176]) && s.b[2297]) && (!s.b[2414])) {
            s.store_add_scaled_product_value_ad(2386, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2384), s.ad_value(2413)), 1.0, 2384, 2406, 1.0);
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            let assign41230_ad_e39738: A = {
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2361), s.ad_value(2374)), A::tanh_scaled_input(A::div(s.ad_value(2361), s.ad_value(2374)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(2361), s.ad_value(2374)), A::div(s.ad_value(2361), s.ad_value(2374))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs(2387, s.ad_value(2361), -1.0, A::pow(A::offset(A::pow(assign41230_ad_e39738, s.ad_value(2375)), 1.0), A::div_from_scalar(1.0, s.ad_value(2375))), 1.0);
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            s.store_mul_ad_product_lhs(2360, A::mul3_scaled_output(s.ad_value(2380), s.ad_value(2370), s.ad_value(2371), -1.0), s.ad_value(2376), 2369);
            s.store_mul_div_lhs(2397, 2377, 2362, 2387);
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            if ((!(s.v[2397] > 50.0)) && (!(s.v[2397] < (-50.0)))) {
                s.store_exp(2398, 2397);
            } else {
                if ((!(s.v[2397] > 50.0)) && (s.v[2397] < (-50.0))) {
                    s.store_scalar(2398, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2397] > 50.0) {
                        s.store_scaled_offset(2398, 2397, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2398, 0.0);
                    }
                }
            }
        }

        if ((s.b[1934] && s.b[2176]) && s.b[2297]) {
            s.store_mul_offset_rhs(2388, 2360, 2398, (-1.0));
            s.store_add(2381, 2386, 2388);
            s.copy_ad(2358, 2381);
            s.copy_ad(141, 2358);
        }

        s.v[234] = 0.0;

        s.v[235] = 0.0;

        s.v[238] = 0.0;

        s.v[242] = 0.0;

        s.v[243] = 0.0;

        s.v[244] = 0.0;

        s.v[245] = 0.0;

        s.v[246] = 0.0;

        s.v[247] = 0.0;

        s.v[248] = 0.0;

        s.v[254] = 0.0;

        s.v[255] = 0.0;

        s.v[256] = 0.0;

        s.v[257] = 0.0;

        s.v[258] = 0.0;

        s.b[2418] = (p.p291 == 1.0);
        s.v[2418] = if s.b[2418] { 1.0 } else { 0.0 };

        if s.b[2418] {
            s.store_scaled_voltage(234, ctx, nodes, Some(8), Some(7), p.p6);
        }

    }

    pub(super) fn stamp_transient_block_34(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[2418] {
            s.store_scalar(2419, 0.0);
            s.store_scalar(2420, 0.0);
            s.store_scalar(2421, 0.0);
            s.copy_ad(2422, 234);
            s.copy_ad(2423, 113);
            s.store_scalar(2424, p.p294);
            s.store_scalar(2425, p.p296);
            s.store_scalar(2426, p.p295);
            s.store_scalar(2427, p.p292);
            s.store_scalar(2428, 4.0);
            s.store_scalar(2429, 600.0);
            s.copy_ad(2430, 112);
            s.store_scalar(2431, (p.p0 * (1.0 - p.p311)));
            s.store_scalar(2432, p.p2);
            s.store_scalar(2433, p.p293);
            s.store_scalar(2434, 0.0);
            s.store_scalar(2435, p.p299);
            s.store_scalar(2436, p.p300);
            s.store_scalar(2437, p.p298);
            s.store_scalar(2438, p.p297);
            s.store_scalar(2439, 0.0);
            s.store_scalar(2440, 0.0);
            s.store_scalar(2441, p.p6);
            s.store_scalar(2442, 0.0);
            s.store_scalar(2443, 0.0);
            s.store_scalar(2444, 0.0);
            s.store_scalar(2445, 0.0);
            s.store_scalar(2446, 0.0);
            s.store_scalar(2447, 0.0);
            s.store_scalar(2448, 0.0);
            s.store_scalar(2449, 0.0);
            s.store_scalar(2450, 0.0);
            s.store_scalar(2451, 0.0);
            s.store_scalar(2452, 0.0);
            s.store_scalar(2453, 0.0);
            s.store_scalar(2454, 0.0);
            s.store_scalar(2455, 0.0);
            s.store_scalar(2456, 0.0);
            s.store_scalar(2457, 0.0);
            s.store_scalar(2458, 0.0);
            s.store_scalar(2459, 0.0);
            s.store_scalar(2460, 0.0);
            s.store_scalar(2461, 0.0);
            s.store_scalar(2462, 0.0);
            s.store_scalar(2463, 0.0);
            s.store_scalar(2464, 0.0);
            s.store_scalar(2465, 0.0);
            s.store_scalar(2466, 0.0);
            s.store_scalar(2467, 0.0);
            s.store_scalar(2468, 0.0);
            s.store_scalar(2469, 0.0);
            s.store_scalar(2470, 0.0);
            s.store_scalar(2471, 0.0);
            s.store_scalar(2472, 0.0);
            s.store_scalar(2473, 0.0);
            s.store_scalar(2474, 0.0);
            s.store_mul_scaled_ad_lhs(2454, A::div(s.ad_value(2439), s.ad_value(2423)), 2440, -1.0);
        }

        if s.b[2418] {
            if ((!(s.v[2454] > 50.0)) && (!(s.v[2454] < (-50.0)))) {
                s.store_exp(2444, 2454);
            } else {
                if ((!(s.v[2454] > 50.0)) && (s.v[2454] < (-50.0))) {
                    s.store_scalar(2444, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2454] > 50.0) {
                        s.store_scaled_offset(2444, 2454, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2444, 0.0);
                    }
                }
            }
        }

        if s.b[2418] {
            s.store_add_scaled_product_right_ad(2450, 2454, 1.0, 2428, A::sub_scaled_inputs(s.ad_value(2422), -1.0, s.ad_value(2429), 1.0), 1.0);
            s.store_add_scaled_product_indices(2451, 2454, 1.0, 2428, 2429, -1.0);
        }

        if s.b[2418] {
            if ((!(s.v[2450] > 50.0)) && (!(s.v[2450] < (-50.0)))) {
                s.store_exp(2452, 2450);
            } else {
                if ((!(s.v[2450] > 50.0)) && (s.v[2450] < (-50.0))) {
                    s.store_scalar(2452, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2450] > 50.0) {
                        s.store_scaled_offset(2452, 2450, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2452, 0.0);
                    }
                }
            }
        }

        if s.b[2418] {
            if ((!(s.v[2451] > 50.0)) && (!(s.v[2451] < (-50.0)))) {
                s.store_exp(2453, 2451);
            } else {
                if ((!(s.v[2451] > 50.0)) && (s.v[2451] < (-50.0))) {
                    s.store_scalar(2453, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2451] > 50.0) {
                        s.store_scaled_offset(2453, 2451, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2453, 0.0);
                    }
                }
            }
        }

        if s.b[2418] {
            s.store_sub(2446, 2452, 2453);
            s.store_mul_ad_product_lhs(2420, A::mul3(s.ad_value(2441), s.ad_value(2431), s.ad_value(2432)), s.ad_value(2433), 2430);
            s.store_add_scaled_product_left_ad(2456, 2454, 1.0, A::div(s.ad_value(2427), s.ad_value(2423)), 2422, 1.0);
        }

        if s.b[2418] {
            if ((!(s.v[2456] > 50.0)) && (!(s.v[2456] < (-50.0)))) {
                s.store_exp(2457, 2456);
            } else {
                if ((!(s.v[2456] > 50.0)) && (s.v[2456] < (-50.0))) {
                    s.store_scalar(2457, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2456] > 50.0) {
                        s.store_scaled_offset(2457, 2456, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2457, 0.0);
                    }
                }
            }
        }

        s.b[2475] = (s.v[2426] == 1.0);
        s.v[2475] = if s.b[2475] { 1.0 } else { 0.0 };

        if (s.b[2418] && s.b[2475]) {
            s.store_mul_sub_ad_rhs(2447, 2420, A::add_scaled_product(s.ad_value(2457), 1.0, s.ad_value(2434), s.ad_value(2446), (-1.0)), s.ad_value(2444));
        }

        if (s.b[2418] && (!s.b[2475])) {
            s.store_add_scaled_product_right_ad(2461, 2454, 1.0, 2428, A::sub_scaled_inputs(s.ad_value(2424), -1.0, s.ad_value(2429), 1.0), 1.0);
        }

        if (s.b[2418] && (!s.b[2475])) {
            if ((!(s.v[2461] > 50.0)) && (!(s.v[2461] < (-50.0)))) {
                s.store_exp(2462, 2461);
            } else {
                if ((!(s.v[2461] > 50.0)) && (s.v[2461] < (-50.0))) {
                    s.store_scalar(2462, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2461] > 50.0) {
                        s.store_scaled_offset(2462, 2461, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2462, 0.0);
                    }
                }
            }
        }

        if (s.b[2418] && (!s.b[2475])) {
            s.store_sub(2463, 2462, 2453);
            s.store_add_scaled_product_left_ad(2464, 2454, 1.0, A::div(s.ad_value(2427), s.ad_value(2423)), 2424, 1.0);
        }

        if (s.b[2418] && (!s.b[2475])) {
            if ((!(s.v[2464] > 50.0)) && (!(s.v[2464] < (-50.0)))) {
                s.store_exp(2465, 2464);
            } else {
                if ((!(s.v[2464] > 50.0)) && (s.v[2464] < (-50.0))) {
                    s.store_scalar(2465, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2464] > 50.0) {
                        s.store_scaled_offset(2465, 2464, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2465, 0.0);
                    }
                }
            }
        }

        if (s.b[2418] && (!s.b[2475])) {
            s.store_sub_ad_lhs(2466, A::add_scaled_product(s.ad_value(2465), 1.0, s.ad_value(2434), s.ad_value(2463), (-1.0)), 2444);
            s.store_mul_sub_ad_rhs(2467, 2420, A::add_scaled_product(s.ad_value(2457), 1.0, s.ad_value(2434), s.ad_value(2446), (-1.0)), s.ad_value(2444));
        }

        s.b[2476] = (s.v[2426] > 0.0);
        s.v[2476] = if s.b[2476] { 1.0 } else { 0.0 };

        if ((s.b[2418] && (!s.b[2475])) && s.b[2476]) {
            s.store_mul(2460, 2426, 2427);
            s.store_add_scaled_product_left_ad(2468, 2454, 1.0, A::div(s.ad_value(2460), s.ad_value(2423)), 2424, 1.0);
        }

        if ((s.b[2418] && (!s.b[2475])) && s.b[2476]) {
            if ((!(s.v[2468] > 50.0)) && (!(s.v[2468] < (-50.0)))) {
                s.store_exp(2469, 2468);
            } else {
                if ((!(s.v[2468] > 50.0)) && (s.v[2468] < (-50.0))) {
                    s.store_scalar(2469, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2468] > 50.0) {
                        s.store_scaled_offset(2469, 2468, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2469, 0.0);
                    }
                }
            }
        }

        if ((s.b[2418] && (!s.b[2475])) && s.b[2476]) {
            s.store_sub_ad_lhs(2470, A::add_scaled_product(s.ad_value(2469), 1.0, s.ad_value(2434), s.ad_value(2463), (-1.0)), 2444);
            s.store_add_scaled_product_left_ad(2471, 2454, 1.0, A::div(s.ad_value(2460), s.ad_value(2423)), 2422, 1.0);
        }

        if ((s.b[2418] && (!s.b[2475])) && s.b[2476]) {
            if ((!(s.v[2471] > 50.0)) && (!(s.v[2471] < (-50.0)))) {
                s.store_exp(2472, 2471);
            } else {
                if ((!(s.v[2471] > 50.0)) && (s.v[2471] < (-50.0))) {
                    s.store_scalar(2472, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2471] > 50.0) {
                        s.store_scaled_offset(2472, 2471, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2472, 0.0);
                    }
                }
            }
        }

        if ((s.b[2418] && (!s.b[2475])) && s.b[2476]) {
            s.store_div_scaled_product_indices(2473, 2420, 2466, 1.0, 2470, 1.0);
            s.store_mul_sub_ad_rhs(2474, 2473, A::add_scaled_product(s.ad_value(2472), 1.0, s.ad_value(2434), s.ad_value(2446), (-1.0)), s.ad_value(2444));
        }

        if ((s.b[2418] && (!s.b[2475])) && (!s.b[2476])) {
            s.store_mul(2474, 2420, 2466);
        }

        if (s.b[2418] && (!s.b[2475])) {
            s.store_mul_square_lhs(2443, 2425, 2423);
            s.store_div_scaled_inputs3(2455, s.ad_value(2422), 1.0, s.ad_value(2424), -1.0, s.ad_value(2443), (-(-0.5)), s.ad_value(2443), 1.0);
        }

        s.b[2477] = (s.v[2455] > 50.0);
        s.v[2477] = if s.b[2477] { 1.0 } else { 0.0 };

        if ((s.b[2418] && (!s.b[2475])) && s.b[2477]) {
            s.store_scalar(2445, 0.0);
        }

        s.b[2478] = (s.v[2455] < (-50.0));
        s.v[2478] = if s.b[2478] { 1.0 } else { 0.0 };

        if (((s.b[2418] && (!s.b[2475])) && (!s.b[2477])) && s.b[2478]) {
            s.store_scalar(2445, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_35(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2418] && (!s.b[2475])) && (!s.b[2477])) && (!s.b[2478])) {
            s.store_div_from_scalar_offset_ad(2445, 1.0, A::exp(s.ad_value(2455)), 1.0);
        }

        if (s.b[2418] && (!s.b[2475])) {
            s.store_add_scaled_product_value_ad(2447, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2445), s.ad_value(2474)), 1.0, 2445, 2467, 1.0);
        }

        if s.b[2418] {
            let assign42480_ad_e40850: A = {
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2422), s.ad_value(2435)), A::tanh_scaled_input(A::div(s.ad_value(2422), s.ad_value(2435)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(2422), s.ad_value(2435)), A::div(s.ad_value(2422), s.ad_value(2435))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs(2448, s.ad_value(2422), -1.0, A::pow(A::offset(A::pow(assign42480_ad_e40850, s.ad_value(2436)), 1.0), A::div_from_scalar(1.0, s.ad_value(2436))), 1.0);
        }

        if s.b[2418] {
            s.store_mul_ad_product_lhs(2421, A::mul3_scaled_output(s.ad_value(2441), s.ad_value(2431), s.ad_value(2432), -1.0), s.ad_value(2437), 2430);
            s.store_mul_div_lhs(2458, 2438, 2423, 2448);
        }

        if s.b[2418] {
            if ((!(s.v[2458] > 50.0)) && (!(s.v[2458] < (-50.0)))) {
                s.store_exp(2459, 2458);
            } else {
                if ((!(s.v[2458] > 50.0)) && (s.v[2458] < (-50.0))) {
                    s.store_scalar(2459, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2458] > 50.0) {
                        s.store_scaled_offset(2459, 2458, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2459, 0.0);
                    }
                }
            }
        }

        if s.b[2418] {
            s.store_mul_offset_rhs(2449, 2421, 2459, (-1.0));
            s.store_add(2442, 2447, 2449);
            s.copy_ad(2419, 2442);
            s.copy_ad(235, 2419);
        }

        s.b[2479] = (p.p301 == 1.0);
        s.v[2479] = if s.b[2479] { 1.0 } else { 0.0 };

        if (s.b[2418] && s.b[2479]) {
            s.store_scalar(2480, 0.0);
            s.store_scalar(2481, 0.0);
            s.store_scalar(2482, 0.0);
            s.copy_ad(2483, 234);
            s.copy_ad(2484, 113);
            s.store_scalar(2485, 1.0);
            s.store_scalar(2486, 10.0);
            s.store_scalar(2487, 1.0);
            s.store_scalar(2488, 0.0);
            s.store_scalar(2489, 4.0);
            s.store_scalar(2490, 600.0);
            s.copy_ad(2491, 112);
            s.store_scalar(2492, (p.p0 * (1.0 - p.p311)));
            s.store_scalar(2493, p.p2);
            s.store_scalar(2494, 0.0);
            s.store_scalar(2495, 0.0);
            s.store_scalar(2496, p.p304);
            s.store_scalar(2497, p.p305);
            s.store_scalar(2498, p.p303);
            s.store_scalar(2499, p.p302);
            s.store_scalar(2500, 0.0);
            s.store_scalar(2501, 0.0);
            s.store_scalar(2502, p.p6);
            s.store_scalar(2503, 0.0);
            s.store_scalar(2504, 0.0);
            s.store_scalar(2505, 0.0);
            s.store_scalar(2506, 0.0);
            s.store_scalar(2507, 0.0);
            s.store_scalar(2508, 0.0);
            s.store_scalar(2509, 0.0);
            s.store_scalar(2510, 0.0);
            s.store_scalar(2511, 0.0);
            s.store_scalar(2512, 0.0);
            s.store_scalar(2513, 0.0);
            s.store_scalar(2514, 0.0);
            s.store_scalar(2515, 0.0);
            s.store_scalar(2516, 0.0);
            s.store_scalar(2517, 0.0);
            s.store_scalar(2518, 0.0);
            s.store_scalar(2519, 0.0);
            s.store_scalar(2520, 0.0);
            s.store_scalar(2521, 0.0);
            s.store_scalar(2522, 0.0);
            s.store_scalar(2523, 0.0);
            s.store_scalar(2524, 0.0);
            s.store_scalar(2525, 0.0);
            s.store_scalar(2526, 0.0);
            s.store_scalar(2527, 0.0);
            s.store_scalar(2528, 0.0);
            s.store_scalar(2529, 0.0);
            s.store_scalar(2530, 0.0);
            s.store_scalar(2531, 0.0);
            s.store_scalar(2532, 0.0);
            s.store_scalar(2533, 0.0);
            s.store_scalar(2534, 0.0);
            s.store_scalar(2535, 0.0);
            s.store_mul_scaled_ad_lhs(2515, A::div(s.ad_value(2500), s.ad_value(2484)), 2501, -1.0);
        }

        if (s.b[2418] && s.b[2479]) {
            if ((!(s.v[2515] > 50.0)) && (!(s.v[2515] < (-50.0)))) {
                s.store_exp(2505, 2515);
            } else {
                if ((!(s.v[2515] > 50.0)) && (s.v[2515] < (-50.0))) {
                    s.store_scalar(2505, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2515] > 50.0) {
                        s.store_scaled_offset(2505, 2515, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2505, 0.0);
                    }
                }
            }
        }

        if (s.b[2418] && s.b[2479]) {
            s.store_add_scaled_product_right_ad(2511, 2515, 1.0, 2489, A::sub_scaled_inputs(s.ad_value(2483), -1.0, s.ad_value(2490), 1.0), 1.0);
            s.store_add_scaled_product_indices(2512, 2515, 1.0, 2489, 2490, -1.0);
        }

        if (s.b[2418] && s.b[2479]) {
            if ((!(s.v[2511] > 50.0)) && (!(s.v[2511] < (-50.0)))) {
                s.store_exp(2513, 2511);
            } else {
                if ((!(s.v[2511] > 50.0)) && (s.v[2511] < (-50.0))) {
                    s.store_scalar(2513, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2511] > 50.0) {
                        s.store_scaled_offset(2513, 2511, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2513, 0.0);
                    }
                }
            }
        }

        if (s.b[2418] && s.b[2479]) {
            if ((!(s.v[2512] > 50.0)) && (!(s.v[2512] < (-50.0)))) {
                s.store_exp(2514, 2512);
            } else {
                if ((!(s.v[2512] > 50.0)) && (s.v[2512] < (-50.0))) {
                    s.store_scalar(2514, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2512] > 50.0) {
                        s.store_scaled_offset(2514, 2512, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2514, 0.0);
                    }
                }
            }
        }

        if (s.b[2418] && s.b[2479]) {
            s.store_sub(2507, 2513, 2514);
            s.store_mul_ad_product_lhs(2481, A::mul3(s.ad_value(2502), s.ad_value(2492), s.ad_value(2493)), s.ad_value(2494), 2491);
            s.store_add_scaled_product_left_ad(2517, 2515, 1.0, A::div(s.ad_value(2488), s.ad_value(2484)), 2483, 1.0);
        }

        if (s.b[2418] && s.b[2479]) {
            if ((!(s.v[2517] > 50.0)) && (!(s.v[2517] < (-50.0)))) {
                s.store_exp(2518, 2517);
            } else {
                if ((!(s.v[2517] > 50.0)) && (s.v[2517] < (-50.0))) {
                    s.store_scalar(2518, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2517] > 50.0) {
                        s.store_scaled_offset(2518, 2517, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2518, 0.0);
                    }
                }
            }
        }

        s.b[2536] = (s.v[2487] == 1.0);
        s.v[2536] = if s.b[2536] { 1.0 } else { 0.0 };

        if ((s.b[2418] && s.b[2479]) && s.b[2536]) {
            s.store_mul_sub_ad_rhs(2508, 2481, A::add_scaled_product(s.ad_value(2518), 1.0, s.ad_value(2495), s.ad_value(2507), (-1.0)), s.ad_value(2505));
        }

        if ((s.b[2418] && s.b[2479]) && (!s.b[2536])) {
            s.store_add_scaled_product_right_ad(2522, 2515, 1.0, 2489, A::sub_scaled_inputs(s.ad_value(2485), -1.0, s.ad_value(2490), 1.0), 1.0);
        }

        if ((s.b[2418] && s.b[2479]) && (!s.b[2536])) {
            if ((!(s.v[2522] > 50.0)) && (!(s.v[2522] < (-50.0)))) {
                s.store_exp(2523, 2522);
            } else {
                if ((!(s.v[2522] > 50.0)) && (s.v[2522] < (-50.0))) {
                    s.store_scalar(2523, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2522] > 50.0) {
                        s.store_scaled_offset(2523, 2522, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2523, 0.0);
                    }
                }
            }
        }

        if ((s.b[2418] && s.b[2479]) && (!s.b[2536])) {
            s.store_sub(2524, 2523, 2514);
            s.store_add_scaled_product_left_ad(2525, 2515, 1.0, A::div(s.ad_value(2488), s.ad_value(2484)), 2485, 1.0);
        }

        if ((s.b[2418] && s.b[2479]) && (!s.b[2536])) {
            if ((!(s.v[2525] > 50.0)) && (!(s.v[2525] < (-50.0)))) {
                s.store_exp(2526, 2525);
            } else {
                if ((!(s.v[2525] > 50.0)) && (s.v[2525] < (-50.0))) {
                    s.store_scalar(2526, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2525] > 50.0) {
                        s.store_scaled_offset(2526, 2525, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2526, 0.0);
                    }
                }
            }
        }

        if ((s.b[2418] && s.b[2479]) && (!s.b[2536])) {
            s.store_sub_ad_lhs(2527, A::add_scaled_product(s.ad_value(2526), 1.0, s.ad_value(2495), s.ad_value(2524), (-1.0)), 2505);
            s.store_mul_sub_ad_rhs(2528, 2481, A::add_scaled_product(s.ad_value(2518), 1.0, s.ad_value(2495), s.ad_value(2507), (-1.0)), s.ad_value(2505));
        }

        s.b[2537] = (s.v[2487] > 0.0);
        s.v[2537] = if s.b[2537] { 1.0 } else { 0.0 };

        if (((s.b[2418] && s.b[2479]) && (!s.b[2536])) && s.b[2537]) {
            s.store_mul(2521, 2487, 2488);
            s.store_add_scaled_product_left_ad(2529, 2515, 1.0, A::div(s.ad_value(2521), s.ad_value(2484)), 2485, 1.0);
        }

    }

    pub(super) fn stamp_transient_block_36(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        if (((s.b[2418] && s.b[2479]) && (!s.b[2536])) && s.b[2537]) {
            if ((!(s.v[2529] > 50.0)) && (!(s.v[2529] < (-50.0)))) {
                s.store_exp(2530, 2529);
            } else {
                if ((!(s.v[2529] > 50.0)) && (s.v[2529] < (-50.0))) {
                    s.store_scalar(2530, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2529] > 50.0) {
                        s.store_scaled_offset(2530, 2529, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2530, 0.0);
                    }
                }
            }
        }

        if (((s.b[2418] && s.b[2479]) && (!s.b[2536])) && s.b[2537]) {
            s.store_sub_ad_lhs(2531, A::add_scaled_product(s.ad_value(2530), 1.0, s.ad_value(2495), s.ad_value(2524), (-1.0)), 2505);
            s.store_add_scaled_product_left_ad(2532, 2515, 1.0, A::div(s.ad_value(2521), s.ad_value(2484)), 2483, 1.0);
        }

        if (((s.b[2418] && s.b[2479]) && (!s.b[2536])) && s.b[2537]) {
            if ((!(s.v[2532] > 50.0)) && (!(s.v[2532] < (-50.0)))) {
                s.store_exp(2533, 2532);
            } else {
                if ((!(s.v[2532] > 50.0)) && (s.v[2532] < (-50.0))) {
                    s.store_scalar(2533, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2532] > 50.0) {
                        s.store_scaled_offset(2533, 2532, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2533, 0.0);
                    }
                }
            }
        }

        if (((s.b[2418] && s.b[2479]) && (!s.b[2536])) && s.b[2537]) {
            s.store_div_scaled_product_indices(2534, 2481, 2527, 1.0, 2531, 1.0);
            s.store_mul_sub_ad_rhs(2535, 2534, A::add_scaled_product(s.ad_value(2533), 1.0, s.ad_value(2495), s.ad_value(2507), (-1.0)), s.ad_value(2505));
        }

        if (((s.b[2418] && s.b[2479]) && (!s.b[2536])) && (!s.b[2537])) {
            s.store_mul(2535, 2481, 2527);
        }

        if ((s.b[2418] && s.b[2479]) && (!s.b[2536])) {
            s.store_mul_square_lhs(2504, 2486, 2484);
            s.store_div_scaled_inputs3(2516, s.ad_value(2483), 1.0, s.ad_value(2485), -1.0, s.ad_value(2504), (-(-0.5)), s.ad_value(2504), 1.0);
        }

        s.b[2538] = (s.v[2516] > 50.0);
        s.v[2538] = if s.b[2538] { 1.0 } else { 0.0 };

        if (((s.b[2418] && s.b[2479]) && (!s.b[2536])) && s.b[2538]) {
            s.store_scalar(2506, 0.0);
        }

        s.b[2539] = (s.v[2516] < (-50.0));
        s.v[2539] = if s.b[2539] { 1.0 } else { 0.0 };

        if ((((s.b[2418] && s.b[2479]) && (!s.b[2536])) && (!s.b[2538])) && s.b[2539]) {
            s.store_scalar(2506, 1.0);
        }

        if ((((s.b[2418] && s.b[2479]) && (!s.b[2536])) && (!s.b[2538])) && (!s.b[2539])) {
            s.store_div_from_scalar_offset_ad(2506, 1.0, A::exp(s.ad_value(2516)), 1.0);
        }

        if ((s.b[2418] && s.b[2479]) && (!s.b[2536])) {
            s.store_add_scaled_product_value_ad(2508, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2506), s.ad_value(2535)), 1.0, 2506, 2528, 1.0);
        }

        if (s.b[2418] && s.b[2479]) {
            let assign43520_ad_e42080: A = {
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2483), s.ad_value(2496)), A::tanh_scaled_input(A::div(s.ad_value(2483), s.ad_value(2496)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(2483), s.ad_value(2496)), A::div(s.ad_value(2483), s.ad_value(2496))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs(2509, s.ad_value(2483), -1.0, A::pow(A::offset(A::pow(assign43520_ad_e42080, s.ad_value(2497)), 1.0), A::div_from_scalar(1.0, s.ad_value(2497))), 1.0);
        }

        if (s.b[2418] && s.b[2479]) {
            s.store_mul_ad_product_lhs(2482, A::mul3_scaled_output(s.ad_value(2502), s.ad_value(2492), s.ad_value(2493), -1.0), s.ad_value(2498), 2491);
            s.store_mul_div_lhs(2519, 2499, 2484, 2509);
        }

        if (s.b[2418] && s.b[2479]) {
            if ((!(s.v[2519] > 50.0)) && (!(s.v[2519] < (-50.0)))) {
                s.store_exp(2520, 2519);
            } else {
                if ((!(s.v[2519] > 50.0)) && (s.v[2519] < (-50.0))) {
                    s.store_scalar(2520, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2519] > 50.0) {
                        s.store_scaled_offset(2520, 2519, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2520, 0.0);
                    }
                }
            }
        }

        if (s.b[2418] && s.b[2479]) {
            s.store_mul_offset_rhs(2510, 2482, 2520, (-1.0));
            s.store_add(2503, 2508, 2510);
            s.copy_ad(2480, 2503);
            s.copy_ad(238, 2480);
        }

        s.b[2540] = (s.v[234] <= (p.p308 * p.p306));
        s.v[2540] = if s.b[2540] { 1.0 } else { 0.0 };

        if (s.b[2418] && s.b[2540]) {
            s.store_offset_scaled_ad(242, A::sqrt(A::sub_from_scalar(1.0, A::scale(s.ad_value(234), 1.0 / (p.p306)))), (-((((((p.p6 * 2.0) * p.p307) * p.p0) * (1.0 - p.p311)) * p.p2) * p.p306)), ((((((p.p6 * 2.0) * p.p307) * p.p0) * (1.0 - p.p311)) * p.p2) * p.p306));
        }

        if (s.b[2418] && (!s.b[2540])) {
            s.store_scalar(243, (1.0 - (((1.0 - p.p308)) as f64).sqrt()));
        }

        s.b[2541] = (p.p309 >= 1.0);
        s.v[2541] = if s.b[2541] { 1.0 } else { 0.0 };

        if ((s.b[2418] && (!s.b[2540])) && s.b[2541]) {
            s.store_scalar(249, (1.0 / ((2.0 * p.p306) * (((1.0 - p.p308)) as f64).sqrt())));
            s.store_offset(254, 234, (-(p.p308 * p.p306)));
            s.store_mul(244, 249, 254);
        }

        s.b[2542] = (p.p309 >= 2.0);
        s.v[2542] = if s.b[2542] { 1.0 } else { 0.0 };

        if (((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) {
            s.store_scale(250, 249, 1.0 / (((4.0 * p.p306) * (1.0 - p.p308))));
            s.store_square(255, 254);
            s.store_mul(245, 250, 255);
        }

        s.b[2543] = (p.p309 >= 3.0);
        s.v[2543] = if s.b[2543] { 1.0 } else { 0.0 };

        if ((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) {
            s.store_scale(251, 250, 1.0 / (((2.0 * p.p306) * (1.0 - p.p308))));
            s.store_mul(256, 255, 254);
            s.store_mul(246, 251, 256);
        }

        s.b[2544] = (p.p309 >= 4.0);
        s.v[2544] = if s.b[2544] { 1.0 } else { 0.0 };

        if (((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) && s.b[2544]) {
            s.store_scale(252, 251, (5.0 * 1.0 / (((8.0 * p.p306) * (1.0 - p.p308)))));
            s.store_mul(257, 256, 254);
            s.store_mul(247, 252, 257);
        }

        s.b[2545] = (p.p309 >= 5.0);
        s.v[2545] = if s.b[2545] { 1.0 } else { 0.0 };

        if ((((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) && s.b[2544]) && s.b[2545]) {
            s.store_scale(253, 252, (7.0 * 1.0 / (((10.0 * p.p306) * (1.0 - p.p308)))));
            s.store_mul(258, 257, 254);
            s.store_mul(248, 253, 258);
        }

        if ((((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) && s.b[2544]) && (!s.b[2545])) {
            s.store_scalar(253, 0.0);
        }

        if (((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && s.b[2543]) && (!s.b[2544])) {
            s.store_scalar(252, 0.0);
        }

        if ((((s.b[2418] && (!s.b[2540])) && s.b[2541]) && s.b[2542]) && (!s.b[2543])) {
            s.store_scalar(251, 0.0);
        }

        if (((s.b[2418] && (!s.b[2540])) && s.b[2541]) && (!s.b[2542])) {
            s.store_scalar(250, 0.0);
        }

        if ((s.b[2418] && (!s.b[2540])) && (!s.b[2541])) {
            s.store_scalar(249, 0.0);
        }

        if (s.b[2418] && (!s.b[2540])) {
            s.store_scaled_add_ad_lhs(242, A::add(A::add_scaled_inputs4(s.ad_value(243), 1.0, s.ad_value(244), 1.0, s.ad_value(245), 1.0, s.ad_value(246), 1.0), s.ad_value(247)), 248, ((((((p.p6 * 2.0) * p.p307) * p.p0) * (1.0 - p.p311)) * p.p2) * p.p306));
        }

        s.v[148] = 0.0;

        s.v[149] = 0.0;

        s.store_add_scaled_voltages(146, ctx, nodes, Some(19), Some(18), p.p6, Some(19), Some(8), p.p6);

        s.store_add_scaled_voltages(147, ctx, nodes, Some(18), Some(19), p.p6, Some(18), Some(8), p.p6);

        s.b[2547] = (p.p312 == 1.0);
        s.v[2547] = if s.b[2547] { 1.0 } else { 0.0 };

        s.b[2548] = (p.p313 == 0.0);
        s.v[2548] = if s.b[2548] { 1.0 } else { 0.0 };

        if (s.b[2547] && s.b[2548]) {
            s.store_add_scaled_voltages(146, ctx, nodes, Some(2), Some(0), p.p6, Some(2), Some(8), p.p6);
            s.store_add_scaled_voltages(147, ctx, nodes, Some(0), Some(2), p.p6, Some(0), Some(8), p.p6);
        }

        if s.b[2547] {
            s.store_scalar(2549, 0.0);
            s.store_scalar(2550, 0.0);
            s.store_scalar(2551, 0.0);
            s.copy_ad(2552, 146);
            s.copy_ad(2553, 113);
            s.store_scalar(2554, p.p260);
            s.store_scalar(2555, p.p262);
            s.store_scalar(2556, p.p261);
            s.store_scalar(2557, 0.0);
            s.store_scalar(2558, p.p317);
            s.store_scalar(2559, p.p316);
            s.copy_ad(2560, 112);
            s.store_scalar(2561, p.p0);
            s.store_scalar(2562, p.p2);
            s.store_scalar(2563, p.p314);
            s.store_scalar(2564, 1.0);
            s.store_scalar(2565, p.p270);
            s.store_scalar(2566, p.p271);
            s.store_scalar(2567, 0.0);
            s.store_scalar(2568, p.p268);
            s.store_scalar(2569, 0.0);
            s.store_scalar(2570, p.p256);
            s.store_scalar(2571, p.p6);
            s.store_scalar(2572, 0.0);
            s.store_scalar(2573, 0.0);
            s.store_scalar(2574, 0.0);
            s.store_scalar(2575, 0.0);
            s.store_scalar(2576, 0.0);
            s.store_scalar(2577, 0.0);
            s.store_scalar(2578, 0.0);
            s.store_scalar(2579, 0.0);
            s.store_scalar(2580, 0.0);
            s.store_scalar(2581, 0.0);
            s.store_scalar(2582, 0.0);
            s.store_scalar(2583, 0.0);
            s.store_scalar(2584, 0.0);
            s.store_scalar(2585, 0.0);
            s.store_scalar(2586, 0.0);
            s.store_scalar(2587, 0.0);
            s.store_scalar(2588, 0.0);
            s.store_scalar(2589, 0.0);
            s.store_scalar(2590, 0.0);
            s.store_scalar(2591, 0.0);
            s.store_scalar(2592, 0.0);
            s.store_scalar(2593, 0.0);
            s.store_scalar(2594, 0.0);
            s.store_scalar(2595, 0.0);
            s.store_scalar(2596, 0.0);
            s.store_scalar(2597, 0.0);
            s.store_scalar(2598, 0.0);
            s.store_scalar(2599, 0.0);
            s.store_scalar(2600, 0.0);
            s.store_scalar(2601, 0.0);
            s.store_scalar(2602, 0.0);
            s.store_scalar(2603, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_37(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[2547] {
            s.store_scalar(2604, 0.0);
            s.store_mul_scaled_ad_lhs(2584, A::div(s.ad_value(2569), s.ad_value(2553)), 2570, -1.0);
        }

        if s.b[2547] {
            if ((!(s.v[2584] > 50.0)) && (!(s.v[2584] < (-50.0)))) {
                s.store_exp(2574, 2584);
            } else {
                if ((!(s.v[2584] > 50.0)) && (s.v[2584] < (-50.0))) {
                    s.store_scalar(2574, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2584] > 50.0) {
                        s.store_scaled_offset(2574, 2584, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2574, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            s.store_add_scaled_product_right_ad(2580, 2584, 1.0, 2558, A::sub_scaled_inputs(s.ad_value(2552), -1.0, s.ad_value(2559), 1.0), 1.0);
            s.store_add_scaled_product_indices(2581, 2584, 1.0, 2558, 2559, -1.0);
        }

        if s.b[2547] {
            if ((!(s.v[2580] > 50.0)) && (!(s.v[2580] < (-50.0)))) {
                s.store_exp(2582, 2580);
            } else {
                if ((!(s.v[2580] > 50.0)) && (s.v[2580] < (-50.0))) {
                    s.store_scalar(2582, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2580] > 50.0) {
                        s.store_scaled_offset(2582, 2580, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2582, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            if ((!(s.v[2581] > 50.0)) && (!(s.v[2581] < (-50.0)))) {
                s.store_exp(2583, 2581);
            } else {
                if ((!(s.v[2581] > 50.0)) && (s.v[2581] < (-50.0))) {
                    s.store_scalar(2583, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2581] > 50.0) {
                        s.store_scaled_offset(2583, 2581, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2583, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            s.store_sub(2576, 2582, 2583);
            s.store_mul_ad_product_lhs(2550, A::mul3(s.ad_value(2571), s.ad_value(2561), s.ad_value(2562)), s.ad_value(2563), 2560);
            s.store_add_scaled_product_left_ad(2586, 2584, 1.0, A::div(s.ad_value(2557), s.ad_value(2553)), 2552, 1.0);
        }

        if s.b[2547] {
            if ((!(s.v[2586] > 50.0)) && (!(s.v[2586] < (-50.0)))) {
                s.store_exp(2587, 2586);
            } else {
                if ((!(s.v[2586] > 50.0)) && (s.v[2586] < (-50.0))) {
                    s.store_scalar(2587, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2586] > 50.0) {
                        s.store_scaled_offset(2587, 2586, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2587, 0.0);
                    }
                }
            }
        }

        s.b[2605] = (s.v[2556] == 1.0);
        s.v[2605] = if s.b[2605] { 1.0 } else { 0.0 };

        if (s.b[2547] && s.b[2605]) {
            s.store_mul_sub_ad_rhs(2577, 2550, A::add_scaled_product(s.ad_value(2587), 1.0, s.ad_value(2564), s.ad_value(2576), (-1.0)), s.ad_value(2574));
        }

        if (s.b[2547] && (!s.b[2605])) {
            s.store_add_scaled_product_right_ad(2591, 2584, 1.0, 2558, A::sub_scaled_inputs(s.ad_value(2554), -1.0, s.ad_value(2559), 1.0), 1.0);
        }

        if (s.b[2547] && (!s.b[2605])) {
            if ((!(s.v[2591] > 50.0)) && (!(s.v[2591] < (-50.0)))) {
                s.store_exp(2592, 2591);
            } else {
                if ((!(s.v[2591] > 50.0)) && (s.v[2591] < (-50.0))) {
                    s.store_scalar(2592, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2591] > 50.0) {
                        s.store_scaled_offset(2592, 2591, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2592, 0.0);
                    }
                }
            }
        }

        if (s.b[2547] && (!s.b[2605])) {
            s.store_sub(2593, 2592, 2583);
            s.store_add_scaled_product_left_ad(2594, 2584, 1.0, A::div(s.ad_value(2557), s.ad_value(2553)), 2554, 1.0);
        }

        if (s.b[2547] && (!s.b[2605])) {
            if ((!(s.v[2594] > 50.0)) && (!(s.v[2594] < (-50.0)))) {
                s.store_exp(2595, 2594);
            } else {
                if ((!(s.v[2594] > 50.0)) && (s.v[2594] < (-50.0))) {
                    s.store_scalar(2595, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2594] > 50.0) {
                        s.store_scaled_offset(2595, 2594, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2595, 0.0);
                    }
                }
            }
        }

        if (s.b[2547] && (!s.b[2605])) {
            s.store_sub_ad_lhs(2596, A::add_scaled_product(s.ad_value(2595), 1.0, s.ad_value(2564), s.ad_value(2593), (-1.0)), 2574);
            s.store_mul_sub_ad_rhs(2597, 2550, A::add_scaled_product(s.ad_value(2587), 1.0, s.ad_value(2564), s.ad_value(2576), (-1.0)), s.ad_value(2574));
        }

        s.b[2606] = (s.v[2556] > 0.0);
        s.v[2606] = if s.b[2606] { 1.0 } else { 0.0 };

        if ((s.b[2547] && (!s.b[2605])) && s.b[2606]) {
            s.store_mul(2590, 2556, 2557);
            s.store_add_scaled_product_left_ad(2598, 2584, 1.0, A::div(s.ad_value(2590), s.ad_value(2553)), 2554, 1.0);
        }

        if ((s.b[2547] && (!s.b[2605])) && s.b[2606]) {
            if ((!(s.v[2598] > 50.0)) && (!(s.v[2598] < (-50.0)))) {
                s.store_exp(2599, 2598);
            } else {
                if ((!(s.v[2598] > 50.0)) && (s.v[2598] < (-50.0))) {
                    s.store_scalar(2599, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2598] > 50.0) {
                        s.store_scaled_offset(2599, 2598, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2599, 0.0);
                    }
                }
            }
        }

        if ((s.b[2547] && (!s.b[2605])) && s.b[2606]) {
            s.store_sub_ad_lhs(2600, A::add_scaled_product(s.ad_value(2599), 1.0, s.ad_value(2564), s.ad_value(2593), (-1.0)), 2574);
            s.store_add_scaled_product_left_ad(2601, 2584, 1.0, A::div(s.ad_value(2590), s.ad_value(2553)), 2552, 1.0);
        }

        if ((s.b[2547] && (!s.b[2605])) && s.b[2606]) {
            if ((!(s.v[2601] > 50.0)) && (!(s.v[2601] < (-50.0)))) {
                s.store_exp(2602, 2601);
            } else {
                if ((!(s.v[2601] > 50.0)) && (s.v[2601] < (-50.0))) {
                    s.store_scalar(2602, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2601] > 50.0) {
                        s.store_scaled_offset(2602, 2601, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2602, 0.0);
                    }
                }
            }
        }

        if ((s.b[2547] && (!s.b[2605])) && s.b[2606]) {
            s.store_div_scaled_product_indices(2603, 2550, 2596, 1.0, 2600, 1.0);
            s.store_mul_sub_ad_rhs(2604, 2603, A::add_scaled_product(s.ad_value(2602), 1.0, s.ad_value(2564), s.ad_value(2576), (-1.0)), s.ad_value(2574));
        }

        if ((s.b[2547] && (!s.b[2605])) && (!s.b[2606])) {
            s.store_mul(2604, 2550, 2596);
        }

        if (s.b[2547] && (!s.b[2605])) {
            s.store_mul_square_lhs(2573, 2555, 2553);
            s.store_div_scaled_inputs3(2585, s.ad_value(2552), 1.0, s.ad_value(2554), -1.0, s.ad_value(2573), (-(-0.5)), s.ad_value(2573), 1.0);
        }

        s.b[2607] = (s.v[2585] > 50.0);
        s.v[2607] = if s.b[2607] { 1.0 } else { 0.0 };

        if ((s.b[2547] && (!s.b[2605])) && s.b[2607]) {
            s.store_scalar(2575, 0.0);
        }

        s.b[2608] = (s.v[2585] < (-50.0));
        s.v[2608] = if s.b[2608] { 1.0 } else { 0.0 };

        if (((s.b[2547] && (!s.b[2605])) && (!s.b[2607])) && s.b[2608]) {
            s.store_scalar(2575, 1.0);
        }

        if (((s.b[2547] && (!s.b[2605])) && (!s.b[2607])) && (!s.b[2608])) {
            s.store_div_from_scalar_offset_ad(2575, 1.0, A::exp(s.ad_value(2585)), 1.0);
        }

        if (s.b[2547] && (!s.b[2605])) {
            s.store_add_scaled_product_value_ad(2577, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2575), s.ad_value(2604)), 1.0, 2575, 2597, 1.0);
        }

        if s.b[2547] {
            let assign44980_ad_e43628: A = {
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2552), s.ad_value(2565)), A::tanh_scaled_input(A::div(s.ad_value(2552), s.ad_value(2565)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(2552), s.ad_value(2565)), A::div(s.ad_value(2552), s.ad_value(2565))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs(2578, s.ad_value(2552), -1.0, A::pow(A::offset(A::pow(assign44980_ad_e43628, s.ad_value(2566)), 1.0), A::div_from_scalar(1.0, s.ad_value(2566))), 1.0);
        }

        if s.b[2547] {
            s.store_mul_ad_product_lhs(2551, A::mul3_scaled_output(s.ad_value(2571), s.ad_value(2561), s.ad_value(2562), -1.0), s.ad_value(2567), 2560);
            s.store_mul_div_lhs(2588, 2568, 2553, 2578);
        }

        if s.b[2547] {
            if ((!(s.v[2588] > 50.0)) && (!(s.v[2588] < (-50.0)))) {
                s.store_exp(2589, 2588);
            } else {
                if ((!(s.v[2588] > 50.0)) && (s.v[2588] < (-50.0))) {
                    s.store_scalar(2589, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2588] > 50.0) {
                        s.store_scaled_offset(2589, 2588, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2589, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            s.store_mul_offset_rhs(2579, 2551, 2589, (-1.0));
            s.store_add(2572, 2577, 2579);
            s.copy_ad(2549, 2572);
            s.copy_ad(148, 2549);
            s.store_scalar(2609, 0.0);
            s.store_scalar(2610, 0.0);
            s.store_scalar(2611, 0.0);
            s.copy_ad(2612, 147);
            s.copy_ad(2613, 113);
            s.store_scalar(2614, p.p265);
            s.store_scalar(2615, p.p267);
            s.store_scalar(2616, p.p266);
            s.store_scalar(2617, 0.0);
            s.store_scalar(2618, p.p319);
            s.store_scalar(2619, p.p318);
            s.copy_ad(2620, 112);
            s.store_scalar(2621, p.p0);
            s.store_scalar(2622, p.p2);
            s.store_scalar(2623, p.p315);
            s.store_scalar(2624, 1.0);
            s.store_scalar(2625, p.p274);
            s.store_scalar(2626, p.p275);
            s.store_scalar(2627, 0.0);
            s.store_scalar(2628, p.p272);
            s.store_scalar(2629, 0.0);
            s.store_scalar(2630, p.p256);
            s.store_scalar(2631, p.p6);
            s.store_scalar(2632, 0.0);
            s.store_scalar(2633, 0.0);
            s.store_scalar(2634, 0.0);
            s.store_scalar(2635, 0.0);
            s.store_scalar(2636, 0.0);
            s.store_scalar(2637, 0.0);
            s.store_scalar(2638, 0.0);
            s.store_scalar(2639, 0.0);
            s.store_scalar(2640, 0.0);
            s.store_scalar(2641, 0.0);
            s.store_scalar(2642, 0.0);
            s.store_scalar(2643, 0.0);
            s.store_scalar(2644, 0.0);
            s.store_scalar(2645, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_38(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv2 = ctx.node_voltage(nodes[2]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        if s.b[2547] {
            s.store_scalar(2646, 0.0);
            s.store_scalar(2647, 0.0);
            s.store_scalar(2648, 0.0);
            s.store_scalar(2649, 0.0);
            s.store_scalar(2650, 0.0);
            s.store_scalar(2651, 0.0);
            s.store_scalar(2652, 0.0);
            s.store_scalar(2653, 0.0);
            s.store_scalar(2654, 0.0);
            s.store_scalar(2655, 0.0);
            s.store_scalar(2656, 0.0);
            s.store_scalar(2657, 0.0);
            s.store_scalar(2658, 0.0);
            s.store_scalar(2659, 0.0);
            s.store_scalar(2660, 0.0);
            s.store_scalar(2661, 0.0);
            s.store_scalar(2662, 0.0);
            s.store_scalar(2663, 0.0);
            s.store_scalar(2664, 0.0);
            s.store_mul_scaled_ad_lhs(2644, A::div(s.ad_value(2629), s.ad_value(2613)), 2630, -1.0);
        }

        if s.b[2547] {
            if ((!(s.v[2644] > 50.0)) && (!(s.v[2644] < (-50.0)))) {
                s.store_exp(2634, 2644);
            } else {
                if ((!(s.v[2644] > 50.0)) && (s.v[2644] < (-50.0))) {
                    s.store_scalar(2634, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2644] > 50.0) {
                        s.store_scaled_offset(2634, 2644, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2634, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            s.store_add_scaled_product_right_ad(2640, 2644, 1.0, 2618, A::sub_scaled_inputs(s.ad_value(2612), -1.0, s.ad_value(2619), 1.0), 1.0);
            s.store_add_scaled_product_indices(2641, 2644, 1.0, 2618, 2619, -1.0);
        }

        if s.b[2547] {
            if ((!(s.v[2640] > 50.0)) && (!(s.v[2640] < (-50.0)))) {
                s.store_exp(2642, 2640);
            } else {
                if ((!(s.v[2640] > 50.0)) && (s.v[2640] < (-50.0))) {
                    s.store_scalar(2642, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2640] > 50.0) {
                        s.store_scaled_offset(2642, 2640, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2642, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            if ((!(s.v[2641] > 50.0)) && (!(s.v[2641] < (-50.0)))) {
                s.store_exp(2643, 2641);
            } else {
                if ((!(s.v[2641] > 50.0)) && (s.v[2641] < (-50.0))) {
                    s.store_scalar(2643, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2641] > 50.0) {
                        s.store_scaled_offset(2643, 2641, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2643, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            s.store_sub(2636, 2642, 2643);
            s.store_mul_ad_product_lhs(2610, A::mul3(s.ad_value(2631), s.ad_value(2621), s.ad_value(2622)), s.ad_value(2623), 2620);
            s.store_add_scaled_product_left_ad(2646, 2644, 1.0, A::div(s.ad_value(2617), s.ad_value(2613)), 2612, 1.0);
        }

        if s.b[2547] {
            if ((!(s.v[2646] > 50.0)) && (!(s.v[2646] < (-50.0)))) {
                s.store_exp(2647, 2646);
            } else {
                if ((!(s.v[2646] > 50.0)) && (s.v[2646] < (-50.0))) {
                    s.store_scalar(2647, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2646] > 50.0) {
                        s.store_scaled_offset(2647, 2646, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2647, 0.0);
                    }
                }
            }
        }

        s.b[2665] = (s.v[2616] == 1.0);
        s.v[2665] = if s.b[2665] { 1.0 } else { 0.0 };

        if (s.b[2547] && s.b[2665]) {
            s.store_mul_sub_ad_rhs(2637, 2610, A::add_scaled_product(s.ad_value(2647), 1.0, s.ad_value(2624), s.ad_value(2636), (-1.0)), s.ad_value(2634));
        }

        if (s.b[2547] && (!s.b[2665])) {
            s.store_add_scaled_product_right_ad(2651, 2644, 1.0, 2618, A::sub_scaled_inputs(s.ad_value(2614), -1.0, s.ad_value(2619), 1.0), 1.0);
        }

        if (s.b[2547] && (!s.b[2665])) {
            if ((!(s.v[2651] > 50.0)) && (!(s.v[2651] < (-50.0)))) {
                s.store_exp(2652, 2651);
            } else {
                if ((!(s.v[2651] > 50.0)) && (s.v[2651] < (-50.0))) {
                    s.store_scalar(2652, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2651] > 50.0) {
                        s.store_scaled_offset(2652, 2651, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2652, 0.0);
                    }
                }
            }
        }

        if (s.b[2547] && (!s.b[2665])) {
            s.store_sub(2653, 2652, 2643);
            s.store_add_scaled_product_left_ad(2654, 2644, 1.0, A::div(s.ad_value(2617), s.ad_value(2613)), 2614, 1.0);
        }

        if (s.b[2547] && (!s.b[2665])) {
            if ((!(s.v[2654] > 50.0)) && (!(s.v[2654] < (-50.0)))) {
                s.store_exp(2655, 2654);
            } else {
                if ((!(s.v[2654] > 50.0)) && (s.v[2654] < (-50.0))) {
                    s.store_scalar(2655, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2654] > 50.0) {
                        s.store_scaled_offset(2655, 2654, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2655, 0.0);
                    }
                }
            }
        }

        if (s.b[2547] && (!s.b[2665])) {
            s.store_sub_ad_lhs(2656, A::add_scaled_product(s.ad_value(2655), 1.0, s.ad_value(2624), s.ad_value(2653), (-1.0)), 2634);
            s.store_mul_sub_ad_rhs(2657, 2610, A::add_scaled_product(s.ad_value(2647), 1.0, s.ad_value(2624), s.ad_value(2636), (-1.0)), s.ad_value(2634));
        }

        s.b[2666] = (s.v[2616] > 0.0);
        s.v[2666] = if s.b[2666] { 1.0 } else { 0.0 };

        if ((s.b[2547] && (!s.b[2665])) && s.b[2666]) {
            s.store_mul(2650, 2616, 2617);
            s.store_add_scaled_product_left_ad(2658, 2644, 1.0, A::div(s.ad_value(2650), s.ad_value(2613)), 2614, 1.0);
        }

        if ((s.b[2547] && (!s.b[2665])) && s.b[2666]) {
            if ((!(s.v[2658] > 50.0)) && (!(s.v[2658] < (-50.0)))) {
                s.store_exp(2659, 2658);
            } else {
                if ((!(s.v[2658] > 50.0)) && (s.v[2658] < (-50.0))) {
                    s.store_scalar(2659, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2658] > 50.0) {
                        s.store_scaled_offset(2659, 2658, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2659, 0.0);
                    }
                }
            }
        }

        if ((s.b[2547] && (!s.b[2665])) && s.b[2666]) {
            s.store_sub_ad_lhs(2660, A::add_scaled_product(s.ad_value(2659), 1.0, s.ad_value(2624), s.ad_value(2653), (-1.0)), 2634);
            s.store_add_scaled_product_left_ad(2661, 2644, 1.0, A::div(s.ad_value(2650), s.ad_value(2613)), 2612, 1.0);
        }

        if ((s.b[2547] && (!s.b[2665])) && s.b[2666]) {
            if ((!(s.v[2661] > 50.0)) && (!(s.v[2661] < (-50.0)))) {
                s.store_exp(2662, 2661);
            } else {
                if ((!(s.v[2661] > 50.0)) && (s.v[2661] < (-50.0))) {
                    s.store_scalar(2662, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2661] > 50.0) {
                        s.store_scaled_offset(2662, 2661, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2662, 0.0);
                    }
                }
            }
        }

        if ((s.b[2547] && (!s.b[2665])) && s.b[2666]) {
            s.store_div_scaled_product_indices(2663, 2610, 2656, 1.0, 2660, 1.0);
            s.store_mul_sub_ad_rhs(2664, 2663, A::add_scaled_product(s.ad_value(2662), 1.0, s.ad_value(2624), s.ad_value(2636), (-1.0)), s.ad_value(2634));
        }

        if ((s.b[2547] && (!s.b[2665])) && (!s.b[2666])) {
            s.store_mul(2664, 2610, 2656);
        }

        if (s.b[2547] && (!s.b[2665])) {
            s.store_mul_square_lhs(2633, 2615, 2613);
            s.store_div_scaled_inputs3(2645, s.ad_value(2612), 1.0, s.ad_value(2614), -1.0, s.ad_value(2633), (-(-0.5)), s.ad_value(2633), 1.0);
        }

        s.b[2667] = (s.v[2645] > 50.0);
        s.v[2667] = if s.b[2667] { 1.0 } else { 0.0 };

        if ((s.b[2547] && (!s.b[2665])) && s.b[2667]) {
            s.store_scalar(2635, 0.0);
        }

        s.b[2668] = (s.v[2645] < (-50.0));
        s.v[2668] = if s.b[2668] { 1.0 } else { 0.0 };

        if (((s.b[2547] && (!s.b[2665])) && (!s.b[2667])) && s.b[2668]) {
            s.store_scalar(2635, 1.0);
        }

        if (((s.b[2547] && (!s.b[2665])) && (!s.b[2667])) && (!s.b[2668])) {
            s.store_div_from_scalar_offset_ad(2635, 1.0, A::exp(s.ad_value(2645)), 1.0);
        }

        if (s.b[2547] && (!s.b[2665])) {
            s.store_add_scaled_product_value_ad(2637, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(2635), s.ad_value(2664)), 1.0, 2635, 2657, 1.0);
        }

        if s.b[2547] {
            let assign46010_ad_e44671: A = {
                if (p.p52 != 0.0) {
                    A::mul(A::div(s.ad_value(2612), s.ad_value(2625)), A::tanh_scaled_input(A::div(s.ad_value(2612), s.ad_value(2625)), (0.001 / p.p53)))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::sqrt(A::offset(A::mul(A::div(s.ad_value(2612), s.ad_value(2625)), A::div(s.ad_value(2612), s.ad_value(2625))), p.p53))
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs(2638, s.ad_value(2612), -1.0, A::pow(A::offset(A::pow(assign46010_ad_e44671, s.ad_value(2626)), 1.0), A::div_from_scalar(1.0, s.ad_value(2626))), 1.0);
        }

        if s.b[2547] {
            s.store_mul_ad_product_lhs(2611, A::mul3_scaled_output(s.ad_value(2631), s.ad_value(2621), s.ad_value(2622), -1.0), s.ad_value(2627), 2620);
            s.store_mul_div_lhs(2648, 2628, 2613, 2638);
        }

        if s.b[2547] {
            if ((!(s.v[2648] > 50.0)) && (!(s.v[2648] < (-50.0)))) {
                s.store_exp(2649, 2648);
            } else {
                if ((!(s.v[2648] > 50.0)) && (s.v[2648] < (-50.0))) {
                    s.store_scalar(2649, (50.0 * (-1.0 as f64)).exp());
                } else {
                    if (s.v[2648] > 50.0) {
                        s.store_scaled_offset(2649, 2648, (((-50.0)) + (1.0)), ((50.0) as f64).exp());
                    } else {
                        s.store_scalar(2649, 0.0);
                    }
                }
            }
        }

        if s.b[2547] {
            s.store_mul_offset_rhs(2639, 2611, 2649, (-1.0));
            s.store_add(2632, 2637, 2639);
            s.copy_ad(2609, 2632);
            s.copy_ad(149, 2609);
        }

        s.b[2669] = (p.p313 == 0.0);
        s.v[2669] = if s.b[2669] { 1.0 } else { 0.0 };

        s.b[2670] = ((s.v[4] >= p.p353) && (s.v[4] > 0.0));
        s.v[2670] = if s.b[2670] { 1.0 } else { 0.0 };

        s.b[2671] = ((s.v[3] >= p.p353) && (s.v[3] > 0.0));
        s.v[2671] = if s.b[2671] { 1.0 } else { 0.0 };

        s.b[2674] = ((((nv6 - nv2) - p.p27) / p.p28) > 50.0);
        s.v[2674] = if s.b[2674] { 1.0 } else { 0.0 };

        if s.b[2674] {
            s.store_add_scaled_products_mixed_iaia(214, 13, A::voltage(ctx, nodes, Some(6), Some(2)), (p.p0 * p.p2), 7, A::offset(A::voltage(ctx, nodes, Some(6), Some(2)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2675] = ((((nv6 - nv2) - p.p27) / p.p28) < (-50.0));
        s.v[2675] = if s.b[2675] { 1.0 } else { 0.0 };

        if ((!s.b[2674]) && s.b[2675]) {
            s.store_add_scaled_products_mixed_iaia(214, 13, A::voltage(ctx, nodes, Some(6), Some(2)), (p.p0 * p.p2), 7, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(6), Some(2)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2674]) && (!s.b[2675])) {
            s.store_add_scaled_products_mixed_iaia(214, 13, A::voltage(ctx, nodes, Some(6), Some(2)), (p.p0 * p.p2), 7, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(6), Some(2)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2676] = ((((nv6 - nv0) - p.p27) / p.p28) > 50.0);
        s.v[2676] = if s.b[2676] { 1.0 } else { 0.0 };

        if s.b[2676] {
            s.store_add_scaled_products_mixed_iaia(215, 14, A::voltage(ctx, nodes, Some(6), Some(0)), (p.p0 * p.p2), 8, A::offset(A::voltage(ctx, nodes, Some(6), Some(0)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2677] = ((((nv6 - nv0) - p.p27) / p.p28) < (-50.0));
        s.v[2677] = if s.b[2677] { 1.0 } else { 0.0 };

        if ((!s.b[2676]) && s.b[2677]) {
            s.store_add_scaled_products_mixed_iaia(215, 14, A::voltage(ctx, nodes, Some(6), Some(0)), (p.p0 * p.p2), 8, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(6), Some(0)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2676]) && (!s.b[2677])) {
            s.store_add_scaled_products_mixed_iaia(215, 14, A::voltage(ctx, nodes, Some(6), Some(0)), (p.p0 * p.p2), 8, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(6), Some(0)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2678] = ((((nv2 - nv0) - p.p27) / p.p28) > 50.0);
        s.v[2678] = if s.b[2678] { 1.0 } else { 0.0 };

        if s.b[2678] {
            s.store_add_scaled_products_mixed_iaia(216, 15, A::voltage(ctx, nodes, Some(2), Some(0)), (p.p0 * p.p2), 9, A::offset(A::voltage(ctx, nodes, Some(2), Some(0)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2679] = ((((nv2 - nv0) - p.p27) / p.p28) < (-50.0));
        s.v[2679] = if s.b[2679] { 1.0 } else { 0.0 };

        if ((!s.b[2678]) && s.b[2679]) {
            s.store_add_scaled_products_mixed_iaia(216, 15, A::voltage(ctx, nodes, Some(2), Some(0)), (p.p0 * p.p2), 9, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(2), Some(0)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2678]) && (!s.b[2679])) {
            s.store_add_scaled_products_mixed_iaia(216, 15, A::voltage(ctx, nodes, Some(2), Some(0)), (p.p0 * p.p2), 9, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(2), Some(0)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2680] = ((((nv3 - nv2) - p.p27) / p.p28) > 50.0);
        s.v[2680] = if s.b[2680] { 1.0 } else { 0.0 };

        if s.b[2680] {
            s.store_add_scaled_products_mixed_iaia(218, 16, A::voltage(ctx, nodes, Some(3), Some(2)), (p.p0 * p.p2), 10, A::offset(A::voltage(ctx, nodes, Some(3), Some(2)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2681] = ((((nv3 - nv2) - p.p27) / p.p28) < (-50.0));
        s.v[2681] = if s.b[2681] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_39(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut Scratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let nv0 = ctx.node_voltage(nodes[0]);
        let nv3 = ctx.node_voltage(nodes[3]);
        let nv6 = ctx.node_voltage(nodes[6]);
        if ((!s.b[2680]) && s.b[2681]) {
            s.store_add_scaled_products_mixed_iaia(218, 16, A::voltage(ctx, nodes, Some(3), Some(2)), (p.p0 * p.p2), 10, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(3), Some(2)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2680]) && (!s.b[2681])) {
            s.store_add_scaled_products_mixed_iaia(218, 16, A::voltage(ctx, nodes, Some(3), Some(2)), (p.p0 * p.p2), 10, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(3), Some(2)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2682] = ((((nv3 - nv0) - p.p27) / p.p28) > 50.0);
        s.v[2682] = if s.b[2682] { 1.0 } else { 0.0 };

        if s.b[2682] {
            s.store_add_scaled_products_mixed_iaia(217, 17, A::voltage(ctx, nodes, Some(3), Some(0)), (p.p0 * p.p2), 11, A::offset(A::voltage(ctx, nodes, Some(3), Some(0)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2683] = ((((nv3 - nv0) - p.p27) / p.p28) < (-50.0));
        s.v[2683] = if s.b[2683] { 1.0 } else { 0.0 };

        if ((!s.b[2682]) && s.b[2683]) {
            s.store_add_scaled_products_mixed_iaia(217, 17, A::voltage(ctx, nodes, Some(3), Some(0)), (p.p0 * p.p2), 11, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(3), Some(0)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2682]) && (!s.b[2683])) {
            s.store_add_scaled_products_mixed_iaia(217, 17, A::voltage(ctx, nodes, Some(3), Some(0)), (p.p0 * p.p2), 11, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(3), Some(0)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        s.b[2684] = ((((nv6 - nv3) - p.p27) / p.p28) > 50.0);
        s.v[2684] = if s.b[2684] { 1.0 } else { 0.0 };

        if s.b[2684] {
            s.store_add_scaled_products_mixed_iaia(219, 18, A::voltage(ctx, nodes, Some(6), Some(3)), (p.p0 * p.p2), 12, A::offset(A::voltage(ctx, nodes, Some(6), Some(3)), (-p.p27)), (p.p0 * p.p2));
        }

        s.b[2685] = ((((nv6 - nv3) - p.p27) / p.p28) < (-50.0));
        s.v[2685] = if s.b[2685] { 1.0 } else { 0.0 };

        if ((!s.b[2684]) && s.b[2685]) {
            s.store_add_scaled_products_mixed_iaia(219, 18, A::voltage(ctx, nodes, Some(6), Some(3)), (p.p0 * p.p2), 12, A::exp_scaled_input(A::offset(A::voltage(ctx, nodes, Some(6), Some(3)), (-p.p27)), 1.0 / (p.p28)), (p.p28 * (p.p0 * p.p2)));
        }

        if ((!s.b[2684]) && (!s.b[2685])) {
            s.store_add_scaled_products_mixed_iaia(219, 18, A::voltage(ctx, nodes, Some(6), Some(3)), (p.p0 * p.p2), 12, A::ln_one_plus_exp(A::scaled_offset(A::voltage(ctx, nodes, Some(6), Some(3)), (-p.p27), 1.0 / (p.p28))), (p.p28 * (p.p0 * p.p2)));
        }

        let assign46640_ad_e45462: A = A::add_scaled_value_products3(A::add_scaled_products3(s.ad_value(115), A::voltage(ctx, nodes, Some(5), Some(9)), 1.0, s.ad_value(160), A::voltage(ctx, nodes, Some(18), Some(17)), 1.0, s.ad_value(154), A::voltage(ctx, nodes, Some(13), Some(19)), 1.0), 1.0, s.ad_value(184), A::voltage(ctx, nodes, Some(12), Some(13)), 1.0, s.ad_value(178), A::voltage(ctx, nodes, Some(11), Some(12)), 1.0, s.ad_value(172), A::voltage(ctx, nodes, Some(10), Some(11)), 1.0);
        s.store_add_scaled_value_products(114, A::add_scaled_value_products3(assign46640_ad_e45462, 1.0, s.ad_value(166), A::voltage(ctx, nodes, Some(9), Some(10)), 1.0, s.ad_value(190), A::voltage(ctx, nodes, Some(14), Some(5)), 1.0, s.ad_value(196), A::voltage(ctx, nodes, Some(15), Some(14)), 1.0), 1.0, s.ad_value(202), A::voltage(ctx, nodes, Some(16), Some(15)), 1.0, s.ad_value(208), A::voltage(ctx, nodes, Some(17), Some(16)), 1.0);

        s.b[2698] = ((s.v[4] >= p.p353) && (s.v[4] > 0.0));
        s.v[2698] = if s.b[2698] { 1.0 } else { 0.0 };

        if s.b[2698] {
            s.store_add_ad_rhs(114, 114, A::div_scaled_product(A::voltage(ctx, nodes, Some(18), Some(0)), A::voltage(ctx, nodes, Some(18), Some(0)), 1.0, s.ad_value(1), 1.0));
        }

        s.b[2699] = ((s.v[3] >= p.p353) && (s.v[3] > 0.0));
        s.v[2699] = if s.b[2699] { 1.0 } else { 0.0 };

        if s.b[2699] {
            s.store_add_ad_rhs(114, 114, A::div_scaled_product(A::voltage(ctx, nodes, Some(19), Some(2)), A::voltage(ctx, nodes, Some(19), Some(2)), 1.0, s.ad_value(2), 1.0));
        }

        s.b[2700] = (p.p320 > 0.0);
        s.v[2700] = if s.b[2700] { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_0(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        let ctx_temp = ctx.temperature();
        s.v[109] = (p.p5 + 273.15);

        s.v[108] = ctx_temp;

        s.store_voltage(110, ctx, nodes, Some(4), None);

        s.store_offset(111, 110, (s.v[108] + p.p3));

        s.b[298] = (s.v[111] < ((-270.0) + 273.15));
        s.v[298] = if s.b[298] { 1.0 } else { 0.0 };

        if s.b[298] {
            s.store_scalar(111, ((-270.0) + 273.15));
        }

        s.b[299] = (s.v[111] > (1500.0 + 273.15));
        s.v[299] = if s.b[299] { 1.0 } else { 0.0 };

        if ((!s.b[298]) && s.b[299]) {
            s.store_scalar(111, (1500.0 + 273.15));
        }

        s.store_scale(113, 111, (1.38062e-23 * 6.241457005723417e18));

        s.store_scale_ad(7, {
            if ((1.0 + (p.p21 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p21, (((((-s.v[109])) * (p.p21))) + (1.0)))
            }
        }, p.p9);

        s.store_scale_ad(8, {
            if ((1.0 + (p.p22 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p22, (((((-s.v[109])) * (p.p22))) + (1.0)))
            }
        }, p.p10);

        s.store_scale_ad(9, {
            if ((1.0 + (p.p23 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p23, (((((-s.v[109])) * (p.p23))) + (1.0)))
            }
        }, p.p11);

        s.store_scale_ad(10, {
            if ((1.0 + (p.p24 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p24, (((((-s.v[109])) * (p.p24))) + (1.0)))
            }
        }, p.p13);

        s.store_scale_ad(11, {
            if ((1.0 + (p.p25 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p25, (((((-s.v[109])) * (p.p25))) + (1.0)))
            }
        }, p.p12);

        s.store_scale_ad(12, {
            if ((1.0 + (p.p26 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p26, (((((-s.v[109])) * (p.p26))) + (1.0)))
            }
        }, p.p14);

        s.store_scale_ad(13, {
            if ((1.0 + (p.p21 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p21, (((((-s.v[109])) * (p.p21))) + (1.0)))
            }
        }, p.p15);

        s.store_scale_ad(14, {
            if ((1.0 + (p.p22 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p22, (((((-s.v[109])) * (p.p22))) + (1.0)))
            }
        }, p.p16);

        s.store_scale_ad(15, {
            if ((1.0 + (p.p23 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p23, (((((-s.v[109])) * (p.p23))) + (1.0)))
            }
        }, p.p17);

        s.store_scale_ad(16, {
            if ((1.0 + (p.p24 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p24, (((((-s.v[109])) * (p.p24))) + (1.0)))
            }
        }, p.p19);

        s.store_scale_ad(17, {
            if ((1.0 + (p.p25 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p25, (((((-s.v[109])) * (p.p25))) + (1.0)))
            }
        }, p.p18);

        s.store_scale_ad(18, {
            if ((1.0 + (p.p26 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p26, (((((-s.v[109])) * (p.p26))) + (1.0)))
            }
        }, p.p20);

        s.store_scale_ad(19, {
            if ((1.0 + (p.p8 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p8, (((((-s.v[109])) * (p.p8))) + (1.0)))
            }
        }, p.p7);

        s.store_scale_ad(20, {
            if ((1.0 + (p.p82 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p82, (((((-s.v[109])) * (p.p82))) + (1.0)))
            }
        }, p.p81);

        s.store_scale_ad(23, {
            if ((1.0 + (p.p104 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p104, (((((-s.v[109])) * (p.p104))) + (1.0)))
            }
        }, p.p103);

        s.store_scale_ad(26, {
            if ((1.0 + (p.p126 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p126, (((((-s.v[109])) * (p.p126))) + (1.0)))
            }
        }, p.p125);

        s.store_scale_ad(29, {
            if ((1.0 + (p.p148 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p148, (((((-s.v[109])) * (p.p148))) + (1.0)))
            }
        }, p.p147);

        s.store_scale_ad(21, {
            if ((1.0 + (p.p87 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p87, (((((-s.v[109])) * (p.p87))) + (1.0)))
            }
        }, p.p86);

        s.store_scale_ad(24, {
            if ((1.0 + (p.p109 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p109, (((((-s.v[109])) * (p.p109))) + (1.0)))
            }
        }, p.p108);

        s.store_scale_ad(27, {
            if ((1.0 + (p.p131 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p131, (((((-s.v[109])) * (p.p131))) + (1.0)))
            }
        }, p.p130);

        s.store_scale_ad(30, {
            if ((1.0 + (p.p153 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p153, (((((-s.v[109])) * (p.p153))) + (1.0)))
            }
        }, p.p152);

        s.store_scale_ad(22, {
            if ((1.0 + (p.p89 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p89, (((((-s.v[109])) * (p.p89))) + (1.0)))
            }
        }, p.p88);

        s.store_scale_ad(25, {
            if ((1.0 + (p.p111 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p111, (((((-s.v[109])) * (p.p111))) + (1.0)))
            }
        }, p.p110);

        s.store_scale_ad(28, {
            if ((1.0 + (p.p133 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p133, (((((-s.v[109])) * (p.p133))) + (1.0)))
            }
        }, p.p132);

        s.store_scale_ad(31, {
            if ((1.0 + (p.p155 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p155, (((((-s.v[109])) * (p.p155))) + (1.0)))
            }
        }, p.p154);

        s.store_scale_ad(32, {
            if ((1.0 + (p.p170 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p170, (((((-s.v[109])) * (p.p170))) + (1.0)))
            }
        }, p.p169);

        s.store_scale_ad(35, {
            if ((1.0 + (p.p192 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p192, (((((-s.v[109])) * (p.p192))) + (1.0)))
            }
        }, p.p191);

        s.store_scale_ad(38, {
            if ((1.0 + (p.p214 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p214, (((((-s.v[109])) * (p.p214))) + (1.0)))
            }
        }, p.p213);

        s.store_scale_ad(41, {
            if ((1.0 + (p.p236 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p236, (((((-s.v[109])) * (p.p236))) + (1.0)))
            }
        }, p.p235);

        s.store_scale_ad(33, {
            if ((1.0 + (p.p175 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p175, (((((-s.v[109])) * (p.p175))) + (1.0)))
            }
        }, p.p174);

        s.store_scale_ad(36, {
            if ((1.0 + (p.p197 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p197, (((((-s.v[109])) * (p.p197))) + (1.0)))
            }
        }, p.p196);

        s.store_scale_ad(39, {
            if ((1.0 + (p.p219 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p219, (((((-s.v[109])) * (p.p219))) + (1.0)))
            }
        }, p.p218);

        s.store_scale_ad(42, {
            if ((1.0 + (p.p241 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p241, (((((-s.v[109])) * (p.p241))) + (1.0)))
            }
        }, p.p240);

        s.store_scale_ad(34, {
            if ((1.0 + (p.p177 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p177, (((((-s.v[109])) * (p.p177))) + (1.0)))
            }
        }, p.p176);

        s.store_scale_ad(37, {
            if ((1.0 + (p.p199 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p199, (((((-s.v[109])) * (p.p199))) + (1.0)))
            }
        }, p.p198);

        s.store_scale_ad(40, {
            if ((1.0 + (p.p221 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p221, (((((-s.v[109])) * (p.p221))) + (1.0)))
            }
        }, p.p220);

        s.store_scale_ad(43, {
            if ((1.0 + (p.p243 * (s.v[111] - s.v[109]))) < 0.01) {
                A::constant(0.01)
            } else {
                A::scale_offset(s.ad_value(111), p.p243, (((((-s.v[109])) * (p.p243))) + (1.0)))
            }
        }, p.p242);

        s.store_scaled_voltage(44, ctx, nodes, Some(5), Some(9), p.p6);

        s.store_scaled_voltage(45, ctx, nodes, Some(8), Some(9), p.p6);

        s.v[224] = 0.0;

        s.v[226] = 0.0;

        s.v[225] = 0.0;

        s.v[227] = 0.0;

        s.v[228] = 0.0;

        s.v[229] = 0.0;

        s.v[230] = 1.0;

        s.b[308] = (p.p328 == 1.0);
        s.v[308] = if s.b[308] { 1.0 } else { 0.0 };

        s.b[309] = (p.p328 == 2.0);
        s.v[309] = if s.b[309] { 1.0 } else { 0.0 };

        if ((!s.b[308]) && s.b[309]) {
            s.store_voltage(224, ctx, nodes, Some(22), None);
            s.store_voltage(225, ctx, nodes, Some(23), None);
            s.store_scaled_abs_ad(228, A::sub(s.ad_value(225), s.ad_value(224)), 1.0 / (p.p338));
            s.store_voltage(226, ctx, nodes, Some(25), None);
            s.store_voltage(227, ctx, nodes, Some(26), None);
            s.store_scaled_abs_ad(229, A::sub(s.ad_value(227), s.ad_value(226)), 1.0 / (p.p337));
            s.store_div_from_scalar_add_ad(230, 1.0, A::offset(s.ad_value(228), 1.0), s.ad_value(229));
        }

        s.b[312] = (p.p78 == 1.0);
        s.v[312] = if s.b[312] { 1.0 } else { 0.0 };

        if s.b[312] {
            s.store_scaled_voltage(60, ctx, nodes, Some(7), Some(10), p.p6);
            s.store_scaled_voltage(62, ctx, nodes, Some(2), Some(10), p.p6);
        }

        if (!s.b[312]) {
            s.store_scaled_voltage(60, ctx, nodes, Some(2), Some(10), p.p6);
            s.store_scaled_voltage(62, ctx, nodes, Some(7), Some(10), p.p6);
        }

        s.store_scaled_voltage(61, ctx, nodes, Some(9), Some(10), p.p6);

        s.store_scaled_voltage(63, ctx, nodes, Some(3), Some(10), p.p6);

        s.b[313] = (p.p100 == 1.0);
        s.v[313] = if s.b[313] { 1.0 } else { 0.0 };

        if s.b[313] {
            s.store_scaled_voltage(66, ctx, nodes, Some(7), Some(11), p.p6);
            s.store_scaled_voltage(68, ctx, nodes, Some(2), Some(11), p.p6);
        }

        if (!s.b[313]) {
            s.store_scaled_voltage(66, ctx, nodes, Some(2), Some(11), p.p6);
            s.store_scaled_voltage(68, ctx, nodes, Some(7), Some(11), p.p6);
        }

        s.store_scaled_voltage(67, ctx, nodes, Some(10), Some(11), p.p6);

        s.store_scaled_voltage(69, ctx, nodes, Some(3), Some(11), p.p6);

        s.b[314] = (p.p122 == 1.0);
        s.v[314] = if s.b[314] { 1.0 } else { 0.0 };

        if s.b[314] {
            s.store_scaled_voltage(72, ctx, nodes, Some(7), Some(12), p.p6);
            s.store_scaled_voltage(74, ctx, nodes, Some(2), Some(12), p.p6);
        }

        if (!s.b[314]) {
            s.store_scaled_voltage(72, ctx, nodes, Some(2), Some(12), p.p6);
            s.store_scaled_voltage(74, ctx, nodes, Some(7), Some(12), p.p6);
        }

        s.store_scaled_voltage(73, ctx, nodes, Some(11), Some(12), p.p6);

        s.store_scaled_voltage(75, ctx, nodes, Some(3), Some(12), p.p6);

        s.b[315] = (p.p144 == 1.0);
        s.v[315] = if s.b[315] { 1.0 } else { 0.0 };

        if s.b[315] {
            s.store_scaled_voltage(78, ctx, nodes, Some(7), Some(13), p.p6);
            s.store_scaled_voltage(80, ctx, nodes, Some(2), Some(13), p.p6);
        }

        if (!s.b[315]) {
            s.store_scaled_voltage(78, ctx, nodes, Some(2), Some(13), p.p6);
            s.store_scaled_voltage(80, ctx, nodes, Some(7), Some(13), p.p6);
        }

        s.store_scaled_voltage(79, ctx, nodes, Some(12), Some(13), p.p6);

        s.store_scaled_voltage(81, ctx, nodes, Some(3), Some(13), p.p6);

        s.b[316] = (p.p166 == 1.0);
        s.v[316] = if s.b[316] { 1.0 } else { 0.0 };

        if s.b[316] {
            s.store_scaled_voltage(84, ctx, nodes, Some(7), Some(5), p.p6);
            s.store_scaled_voltage(86, ctx, nodes, Some(2), Some(5), p.p6);
        }

        if (!s.b[316]) {
            s.store_scaled_voltage(84, ctx, nodes, Some(2), Some(5), p.p6);
            s.store_scaled_voltage(86, ctx, nodes, Some(7), Some(5), p.p6);
        }

        s.store_scaled_voltage(85, ctx, nodes, Some(14), Some(5), p.p6);

        s.store_scaled_voltage(87, ctx, nodes, Some(3), Some(5), p.p6);

        s.b[317] = (p.p188 == 1.0);
        s.v[317] = if s.b[317] { 1.0 } else { 0.0 };

        if s.b[317] {
            s.store_scaled_voltage(90, ctx, nodes, Some(7), Some(14), p.p6);
            s.store_scaled_voltage(92, ctx, nodes, Some(2), Some(14), p.p6);
        }

        if (!s.b[317]) {
            s.store_scaled_voltage(90, ctx, nodes, Some(2), Some(14), p.p6);
            s.store_scaled_voltage(92, ctx, nodes, Some(7), Some(14), p.p6);
        }

        s.store_scaled_voltage(91, ctx, nodes, Some(15), Some(14), p.p6);

        s.store_scaled_voltage(93, ctx, nodes, Some(3), Some(14), p.p6);

    }

    pub(super) fn stamp_reactive_block_1(
        ctx: &GeneratedEvalContext<'_>,
        s: &mut ReactiveScratch,
        p: &Parameters,
        nodes: &[usize; Instance::NODE_COUNT],
    ) {
        s.b[318] = (p.p210 == 1.0);
        s.v[318] = if s.b[318] { 1.0 } else { 0.0 };

        if s.b[318] {
            s.store_scaled_voltage(96, ctx, nodes, Some(7), Some(15), p.p6);
            s.store_scaled_voltage(98, ctx, nodes, Some(2), Some(15), p.p6);
        }

        if (!s.b[318]) {
            s.store_scaled_voltage(96, ctx, nodes, Some(2), Some(15), p.p6);
            s.store_scaled_voltage(98, ctx, nodes, Some(7), Some(15), p.p6);
        }

        s.store_scaled_voltage(97, ctx, nodes, Some(16), Some(15), p.p6);

        s.store_scaled_voltage(99, ctx, nodes, Some(3), Some(15), p.p6);

        s.b[319] = (p.p232 == 1.0);
        s.v[319] = if s.b[319] { 1.0 } else { 0.0 };

        if s.b[319] {
            s.store_scaled_voltage(102, ctx, nodes, Some(7), Some(16), p.p6);
            s.store_scaled_voltage(104, ctx, nodes, Some(2), Some(16), p.p6);
        }

        if (!s.b[319]) {
            s.store_scaled_voltage(102, ctx, nodes, Some(2), Some(16), p.p6);
            s.store_scaled_voltage(104, ctx, nodes, Some(7), Some(16), p.p6);
        }

        s.store_scaled_voltage(103, ctx, nodes, Some(17), Some(16), p.p6);

        s.store_scaled_voltage(105, ctx, nodes, Some(3), Some(16), p.p6);

        s.v[209] = 0.0;

        s.v[210] = 0.0;

        s.v[211] = 0.0;

        s.v[212] = 0.0;

        s.v[213] = 0.0;

        s.b[320] = (p.p233 > p.p354);
        s.v[320] = if s.b[320] { 1.0 } else { 0.0 };

        if s.b[320] {
            s.store_scalar(323, 0.0);
            s.store_scalar(324, 0.0);
            s.store_scalar(325, 0.0);
            s.store_scalar(326, 0.0);
            s.store_scalar(327, 0.0);
            s.store_scalar(328, 0.0);
            s.store_scalar(329, 0.0);
            s.copy_ad(330, 102);
            s.copy_ad(331, 103);
            s.store_scalar(332, p.p239);
            s.copy_ad(333, 104);
            s.copy_ad(334, 105);
            s.store_scalar(335, p.p237);
            s.copy_ad(336, 111);
            s.store_scalar(337, s.v[109]);
            s.copy_ad(338, 113);
            s.store_scalar(339, p.p0);
            s.store_scalar(340, p.p233);
            s.copy_ad(341, 41);
            s.store_scalar(342, p.p238);
            s.copy_ad(343, 42);
            s.copy_ad(344, 43);
            s.store_scalar(345, p.p234);
            s.store_scalar(346, p.p248);
            s.store_scalar(347, p.p247);
            s.store_scalar(348, 0.0);
            s.store_scalar(349, p.p249);
            s.store_scalar(350, p.p253);
            s.store_scalar(351, p.p244);
            s.store_scalar(352, p.p245);
            s.store_scalar(353, p.p246);
            s.store_scalar(354, p.p252);
            s.store_scalar(355, p.p251);
            s.store_scalar(356, p.p250);
            s.store_scalar(357, p.p39);
            s.store_scalar(358, p.p47);
            s.store_scalar(359, p.p45);
            s.store_scalar(360, p.p42);
            s.store_scalar(361, p.p2);
            s.store_scalar(362, p.p6);
            s.store_scalar(363, 1.0);
            s.store_scalar(364, 0.0);
            s.store_scalar(365, 0.0);
            s.store_scalar(366, 0.0);
            s.store_scalar(367, 0.0);
            s.store_scalar(368, 0.0);
            s.store_scalar(369, 0.0);
            s.store_scalar(370, 0.0);
            s.store_scalar(371, 0.0);
            s.store_scalar(372, 0.0);
            s.store_scalar(373, 0.0);
            s.store_scalar(374, 0.0);
            s.store_scalar(375, 0.0);
            s.store_scalar(377, 0.0);
            s.store_scalar(378, 0.0);
            s.store_scalar(379, 0.0);
            s.store_scalar(380, 0.0);
            s.store_scalar(381, 0.0);
            s.store_scalar(382, 0.0);
            s.store_scalar(383, 0.0);
            s.store_scalar(384, 0.0);
            s.store_scalar(385, 0.0);
            s.store_scalar(386, 0.0);
            s.store_scalar(387, 0.0);
            s.store_scalar(388, 0.0);
            s.store_scalar(389, 0.0);
            s.store_scalar(390, 0.0);
            s.store_scalar(391, 0.0);
            s.store_scalar(392, 0.0);
            s.store_scalar(393, 0.0);
            s.store_scalar(394, 0.0);
            s.store_scalar(395, 0.0);
            s.store_scalar(396, 0.0);
            s.store_scalar(397, 0.0);
            s.store_scalar(398, 0.0);
            s.store_scalar(399, 0.0);
            s.store_scalar(400, 0.0);
            s.store_scalar(401, 0.0);
            s.store_scalar(402, 0.0);
            s.store_scalar(405, 0.0);
            s.store_scalar(406, 0.0);
            s.store_scalar(407, 0.0);
            s.store_scalar(408, 0.0);
            s.store_scalar(409, 0.0);
            s.store_scalar(410, 0.0);
            s.store_scalar(411, 0.0);
            s.store_scalar(412, 0.0);
            s.store_scalar(413, 0.0);
            s.store_scalar(414, 0.0);
            s.store_scalar(415, 0.0);
            s.store_scalar(416, 0.0);
            s.store_scalar(417, 0.0);
            s.store_scalar(418, 0.0);
            s.store_scalar(419, 0.0);
            s.store_scalar(420, 0.0);
            s.store_scalar(421, 0.0);
            s.store_scalar(422, 0.0);
            s.store_scalar(423, 0.0);
            s.store_scalar(424, 0.0);
            s.store_scalar(425, 0.0);
            s.store_scalar(426, 0.0);
            s.store_scalar(427, 0.0);
            s.store_scalar(428, 0.0);
            s.store_scalar(429, 0.0);
            s.store_scalar(430, 0.0);
            s.store_scalar(431, 0.0);
            s.store_scalar(432, 0.0);
        }

        if s.b[320] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(429, 331, A::tanh_scaled_input(s.ad_value(331), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(429, 331, p.p53);
                } else {
                    s.store_scalar(429, 0.0);
                }
            }
        }

        if s.b[320] {
            s.store_sub(430, 330, 331);
            s.store_mul(364, 350, 338);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[320] {
            s.store_add_scaled_product_value_ad(366, A::div_scaled_inputs(s.ad_value(346), 1.0, s.ad_value(338), 2.302585092994046), 1.0, 349, 429, 1.0);
            s.store_add_scaled_product_right_ad(367, 345, 1.0, 356, A::sub(s.ad_value(336), s.ad_value(337)), 1.0);
            s.store_pow_ad(385, A::div(s.ad_value(336), s.ad_value(337)), s.ad_value(358));
        }

        s.b[433] = (s.v[357] != 0.0);
        s.v[433] = if s.b[433] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[433]) {
            s.store_div_ad_rhs(368, 429, A::pow(A::offset(A::pow(A::div(s.ad_value(429), s.ad_value(357)), s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353))));
        }

        if (s.b[320] && (!s.b[433])) {
            s.store_scalar(368, 0.0);
        }

        if s.b[320] {
            s.store_mul_add_scaled_product_rhs(365, 429, s.ad_value(347), 1.0, s.ad_value(368), s.ad_value(348), (-1.0));
            s.store_sub(328, 367, 365);
            s.store_scaled_mul(370, 366, 338, 2.0);
            s.store_mul(371, 341, 370);
            s.store_sub_scaled_inputs(428, 328, 1.0, 364, (p.p51 * 0.5));
        }

        if s.b[320] {
            let assign3020_ad_e4515: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::sub(s.ad_value(330), s.ad_value(430))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(427, assign3020_ad_e4515, 1.0, s.ad_value(428), (-1.0), s.ad_value(364), 1.0);
        }

        s.b[434] = (s.v[427] > 50.0);
        s.v[434] = if s.b[434] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[434]) {
            s.store_scalar(386, 0.0);
        }

        s.b[435] = (s.v[427] < (-50.0));
        s.v[435] = if s.b[435] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[434])) && s.b[435]) {
            s.store_scalar(386, 1.0);
        }

        if ((s.b[320] && (!s.b[434])) && (!s.b[435])) {
            s.store_div_from_scalar_offset_ad(386, 1.0, A::exp(s.ad_value(427)), 1.0);
        }

        if s.b[320] {
            let assign3080_ad_e4603: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::sub(s.ad_value(330), s.ad_value(430))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(387, assign3080_ad_e4603, 1.0, A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(364), s.ad_value(386), (-(p.p51 * 0.1))), (-1.0), s.ad_value(370), 1.0);
        }

        s.b[436] = (s.v[387] > 50.0);
        s.v[436] = if s.b[436] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[436]) {
            s.store_mul(388, 371, 387);
        }

        s.b[437] = (s.v[387] < (-50.0));
        s.v[437] = if s.b[437] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[436])) && s.b[437]) {
            s.store_mul_exp_rhs(388, 371, 387);
        }

        if ((s.b[320] && (!s.b[436])) && (!s.b[437])) {
            s.store_mul_ln_one_plus_exp_rhs(388, 371, 387);
        }

        if s.b[320] {
            s.store_div_ad_rhs(374, 352, A::mul_offset_rhs(s.ad_value(385), A::div_scaled_product(s.ad_value(354), s.ad_value(388), 1.0, s.ad_value(341), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(375, 351, A::div_scaled_offset_numerator(A::mul(s.ad_value(359), s.ad_value(337)), 1.0, 1.0, A::offset(A::mul(s.ad_value(359), s.ad_value(336)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(360), s.ad_value(429), 1.0, s.ad_value(340), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(355), s.ad_value(388), 1.0, s.ad_value(341), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(392, 375, 340, 1.0, 374, 1.0);
            s.store_add_scaled_product_right_ad(393, 392, (-1.0), 392, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(388), 2.0, s.ad_value(341), s.ad_value(392), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(394, A::mul_sub_from_scalar_rhs(s.ad_value(392), 1.0, s.ad_value(386)), 1.0, 370, 386, 1.0);
            s.store_add_scaled_product_value_ad(329, A::mul_sub_from_scalar_rhs(s.ad_value(393), 1.0, s.ad_value(386)), 1.0, 370, 386, 1.0);
        }

        if s.b[320] {
            let assign3210_ad_e4832: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(331), s.ad_value(329)), 0.5, A::div(s.ad_value(331), s.ad_value(329)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(331), s.ad_value(329))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(331), s.ad_value(329)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(331), s.ad_value(329)), A::div(s.ad_value(331), s.ad_value(329)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(395, 1.0, A::offset(A::pow(assign3210_ad_e4832, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul(396, 331, 395);
        }

        if s.b[320] {
            let assign3230_ad_e4913: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(329), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(397, 1.0, A::offset(A::pow(assign3230_ad_e4913, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul_neg_lhs(398, 331, 397);
            s.store_div_scaled_inputs2(427, s.ad_value(330), 1.0, s.ad_value(428), (-1.0), s.ad_value(364), 1.0);
        }

        s.b[438] = (s.v[427] > 50.0);
        s.v[438] = if s.b[438] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[438]) {
            s.store_scalar(369, 0.0);
        }

        s.b[439] = (s.v[427] < (-50.0));
        s.v[439] = if s.b[439] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[438])) && s.b[439]) {
            s.store_scalar(369, 1.0);
        }

        if ((s.b[320] && (!s.b[438])) && (!s.b[439])) {
            s.store_div_from_scalar_offset_ad(369, 1.0, A::exp(s.ad_value(427)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs3(372, s.ad_value(430), 1.0, s.ad_value(398), (-1.0), A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(364), s.ad_value(369), (-(p.p51 * 0.1))), -1.0, s.ad_value(370), 1.0);
        }

        s.b[440] = (s.v[372] > 50.0);
        s.v[440] = if s.b[440] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[440]) {
            s.store_mul(373, 371, 372);
        }

        s.b[441] = (s.v[372] < (-50.0));
        s.v[441] = if s.b[441] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[440])) && s.b[441]) {
            s.store_mul_exp_rhs(373, 371, 372);
        }

        if ((s.b[320] && (!s.b[440])) && (!s.b[441])) {
            s.store_mul_ln_one_plus_exp_rhs(373, 371, 372);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2(427, s.ad_value(430), 1.0, s.ad_value(428), (-1.0), s.ad_value(364), 1.0);
        }

        s.b[442] = (s.v[427] > 50.0);
        s.v[442] = if s.b[442] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[442]) {
            s.store_scalar(399, 0.0);
        }

        s.b[443] = (s.v[427] < (-50.0));
        s.v[443] = if s.b[443] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[442])) && s.b[443]) {
            s.store_scalar(399, 1.0);
        }

        if ((s.b[320] && (!s.b[442])) && (!s.b[443])) {
            s.store_div_from_scalar_offset_ad(399, 1.0, A::exp(s.ad_value(427)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs3(400, s.ad_value(330), 1.0, s.ad_value(396), (-1.0), A::add_scaled_product(s.ad_value(328), 1.0, s.ad_value(364), s.ad_value(399), (-(p.p51 * 0.1))), -1.0, s.ad_value(370), 1.0);
        }

        s.b[444] = (s.v[400] > 50.0);
        s.v[444] = if s.b[444] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[444]) {
            s.store_mul(401, 371, 400);
        }

        s.b[445] = (s.v[400] < (-50.0));
        s.v[445] = if s.b[445] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[444])) && s.b[445]) {
            s.store_mul_exp_rhs(401, 371, 400);
        }

        if ((s.b[320] && (!s.b[444])) && (!s.b[445])) {
            s.store_mul_ln_one_plus_exp_rhs(401, 371, 400);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2(402, s.ad_value(373), 1.0, s.ad_value(401), (-1.0), s.ad_value(341), 1.0);
            s.store_div(428, 402, 394);
            s.store_div_scaled_inputs(377, s.ad_value(346), 1.0, s.ad_value(338), 2.302585092994046);
            s.store_scaled_mul(379, 377, 338, 2.0);
            s.store_mul(380, 341, 379);
            s.store_sub_scaled_inputs(432, 367, 1.0, 364, (p.p51 * 0.5));
        }

        if s.b[320] {
            let assign3580_ad_e5294: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::sub(s.ad_value(330), s.ad_value(430))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(431, assign3580_ad_e5294, 1.0, s.ad_value(432), (-1.0), s.ad_value(364), 1.0);
        }

        s.b[446] = (s.v[431] > 50.0);
        s.v[446] = if s.b[446] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[446]) {
            s.store_scalar(389, 0.0);
        }

        s.b[447] = (s.v[431] < (-50.0));
        s.v[447] = if s.b[447] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[446])) && s.b[447]) {
            s.store_scalar(389, 1.0);
        }

        if ((s.b[320] && (!s.b[446])) && (!s.b[447])) {
            s.store_div_from_scalar_offset_ad(389, 1.0, A::exp(s.ad_value(431)), 1.0);
        }

        if s.b[320] {
            let assign3640_ad_e5382: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sub(s.ad_value(330), s.ad_value(430)), A::tanh_scaled_input(A::sub(s.ad_value(330), s.ad_value(430)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(330), 0.5, s.ad_value(430), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(330), s.ad_value(430)), A::sub(s.ad_value(330), s.ad_value(430))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(390, assign3640_ad_e5382, 1.0, A::add_scaled_product(s.ad_value(367), 1.0, s.ad_value(364), s.ad_value(389), (-(p.p51 * 0.1))), (-1.0), s.ad_value(379), 1.0);
        }

        s.b[448] = (s.v[390] > 50.0);
        s.v[448] = if s.b[448] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[448]) {
            s.store_mul(391, 380, 390);
        }

        s.b[449] = (s.v[390] < (-50.0));
        s.v[449] = if s.b[449] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[448])) && s.b[449]) {
            s.store_mul_exp_rhs(391, 380, 390);
        }

        if ((s.b[320] && (!s.b[448])) && (!s.b[449])) {
            s.store_mul_ln_one_plus_exp_rhs(391, 380, 390);
        }

        if s.b[320] {
            s.store_div(383, 352, 385);
            s.store_mul_div_scaled_offset_numerator_rhs(384, 351, A::mul(s.ad_value(359), s.ad_value(337)), 1.0, 1.0, A::offset(A::mul(s.ad_value(359), s.ad_value(336)), 1.0), 1.0);
            s.store_div_scaled_product_indices(405, 384, 340, 1.0, 383, 1.0);
            s.store_add_scaled_product_right_ad(406, 405, (-1.0), 405, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(391), 2.0, s.ad_value(341), s.ad_value(405), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(407, A::mul_sub_from_scalar_rhs(s.ad_value(406), 1.0, s.ad_value(389)), 1.0, 379, 389, 1.0);
        }

        if s.b[320] {
            let assign3750_ad_e5557: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(331), s.ad_value(407)), 0.5, A::div(s.ad_value(331), s.ad_value(407)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(331), s.ad_value(407))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(331), s.ad_value(407)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(331), s.ad_value(407)), A::div(s.ad_value(331), s.ad_value(407)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(408, 1.0, A::offset(A::pow(assign3750_ad_e5557, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul(409, 331, 408);
        }

        if s.b[320] {
            let assign3770_ad_e5638: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), A::div_scaled_inputs(s.ad_value(331), -1.0, s.ad_value(407), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(410, 1.0, A::offset(A::pow(assign3770_ad_e5638, s.ad_value(353)), 1.0), A::div_from_scalar(1.0, s.ad_value(353)));
        }

        if s.b[320] {
            s.store_mul_neg_lhs(411, 331, 410);
            s.store_div_scaled_inputs2(431, s.ad_value(330), 1.0, s.ad_value(432), (-1.0), s.ad_value(364), 1.0);
        }

        s.b[450] = (s.v[431] > 50.0);
        s.v[450] = if s.b[450] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[450]) {
            s.store_scalar(378, 0.0);
        }

        s.b[451] = (s.v[431] < (-50.0));
        s.v[451] = if s.b[451] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[450])) && s.b[451]) {
            s.store_scalar(378, 1.0);
        }

        if ((s.b[320] && (!s.b[450])) && (!s.b[451])) {
            s.store_div_from_scalar_offset_ad(378, 1.0, A::exp(s.ad_value(431)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs3(381, s.ad_value(430), 1.0, s.ad_value(411), (-1.0), A::add_scaled_product(s.ad_value(367), 1.0, s.ad_value(364), s.ad_value(378), (-(p.p51 * 0.1))), -1.0, s.ad_value(379), 1.0);
        }

        s.b[452] = (s.v[381] > 50.0);
        s.v[452] = if s.b[452] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[452]) {
            s.store_mul(382, 380, 381);
        }

        s.b[453] = (s.v[381] < (-50.0));
        s.v[453] = if s.b[453] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[452])) && s.b[453]) {
            s.store_mul_exp_rhs(382, 380, 381);
        }

        if ((s.b[320] && (!s.b[452])) && (!s.b[453])) {
            s.store_mul_ln_one_plus_exp_rhs(382, 380, 381);
        }

        if s.b[320] {
            s.store_div_scaled_inputs2(431, s.ad_value(430), 1.0, s.ad_value(432), (-1.0), s.ad_value(364), 1.0);
        }

        s.b[454] = (s.v[431] > 50.0);
        s.v[454] = if s.b[454] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[454]) {
            s.store_scalar(412, 0.0);
        }

        s.b[455] = (s.v[431] < (-50.0));
        s.v[455] = if s.b[455] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[454])) && s.b[455]) {
            s.store_scalar(412, 1.0);
        }

        if ((s.b[320] && (!s.b[454])) && (!s.b[455])) {
            s.store_div_from_scalar_offset_ad(412, 1.0, A::exp(s.ad_value(431)), 1.0);
        }

        if s.b[320] {
            s.store_div_scaled_inputs3(413, s.ad_value(330), 1.0, s.ad_value(409), (-1.0), A::add_scaled_product(s.ad_value(367), 1.0, s.ad_value(364), s.ad_value(412), (-(p.p51 * 0.1))), -1.0, s.ad_value(379), 1.0);
        }

        s.b[456] = (s.v[413] > 50.0);
        s.v[456] = if s.b[456] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[456]) {
            s.store_mul(414, 380, 413);
        }

        s.b[457] = (s.v[413] < (-50.0));
        s.v[457] = if s.b[457] { 1.0 } else { 0.0 };

        if ((s.b[320] && (!s.b[456])) && s.b[457]) {
            s.store_mul_exp_rhs(414, 380, 413);
        }

        if ((s.b[320] && (!s.b[456])) && (!s.b[457])) {
            s.store_mul_ln_one_plus_exp_rhs(414, 380, 413);
        }

    }

    pub(super) fn stamp_reactive_block_3(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[320] {
            s.store_offset_square(415, 382, 1e-38);
            s.store_offset_mul(416, 415, 382, 1e-57);
            s.store_offset_square(417, 414, 1e-38);
            s.store_offset_mul(418, 417, 414, 1e-57);
            s.store_offset_mul(419, 382, 414, 1e-38);
            s.store_div_scaled_inputs3(420, s.ad_value(415), (2.0 / 3.0), s.ad_value(417), (2.0 / 3.0), s.ad_value(419), (2.0 / 3.0), A::offset(A::add(s.ad_value(382), s.ad_value(414)), 2e-19), 1.0);
            s.store_div_ad(421, A::add_scaled_inputs_products(s.ad_value(416), (2.0 * 2.0), s.ad_value(418), (3.0 * 2.0), s.ad_value(415), s.ad_value(414), (4.0 * 2.0), s.ad_value(417), s.ad_value(382), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(415), 15.0, s.ad_value(417), 15.0, s.ad_value(419), (2.0 * 15.0)));
            s.store_sub(422, 420, 421);
            s.copy_ad(423, 421);
            s.store_mul_product3_rhs(323, 363, A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(340)), s.ad_value(362), s.ad_value(422), 1.0);
            s.store_mul_product3_rhs(324, 363, A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(340)), s.ad_value(362), s.ad_value(423), 1.0);
        }

        s.b[458] = (s.v[332] == 1.0);
        s.v[458] = if s.b[458] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[458]) {
            s.store_div_scaled_inputs3(424, s.ad_value(333), 1.0, s.ad_value(367), -1.0, s.ad_value(364), (-(-(p.p51 * 0.5))), s.ad_value(379), 1.0);
        }

        s.b[459] = (s.v[424] > 50.0);
        s.v[459] = if s.b[459] { 1.0 } else { 0.0 };

        if ((s.b[320] && s.b[458]) && s.b[459]) {
            s.copy_ad(427, 424);
        }

        s.b[460] = (s.v[424] < (-50.0));
        s.v[460] = if s.b[460] { 1.0 } else { 0.0 };

        if (((s.b[320] && s.b[458]) && (!s.b[459])) && s.b[460]) {
            s.store_exp(427, 424);
        }

        if (((s.b[320] && s.b[458]) && (!s.b[459])) && (!s.b[460])) {
            s.store_ln_one_plus_exp(427, 424);
        }

        if (s.b[320] && s.b[458]) {
            s.store_mul_ad_product_lhs(325, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(362)), s.ad_value(343), s.ad_value(379)), s.ad_value(427), 363);
            s.store_div_scaled_inputs3(425, s.ad_value(334), 1.0, s.ad_value(367), -1.0, s.ad_value(364), (-(-(p.p51 * 0.5))), s.ad_value(379), 1.0);
        }

        s.b[461] = (s.v[425] > 50.0);
        s.v[461] = if s.b[461] { 1.0 } else { 0.0 };

        if ((s.b[320] && s.b[458]) && s.b[461]) {
            s.copy_ad(427, 425);
        }

        s.b[462] = (s.v[425] < (-50.0));
        s.v[462] = if s.b[462] { 1.0 } else { 0.0 };

        if (((s.b[320] && s.b[458]) && (!s.b[461])) && s.b[462]) {
            s.store_exp(427, 425);
        }

        if (((s.b[320] && s.b[458]) && (!s.b[461])) && (!s.b[462])) {
            s.store_ln_one_plus_exp(427, 425);
        }

        if (s.b[320] && s.b[458]) {
            s.store_mul_ad_product_lhs(326, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(362)), s.ad_value(344), s.ad_value(379)), s.ad_value(427), 363);
        }

        if (s.b[320] && (!s.b[458])) {
            s.store_scalar(325, 0.0);
            s.store_scalar(326, 0.0);
        }

        s.b[463] = (s.v[335] == 1.0);
        s.v[463] = if s.b[463] { 1.0 } else { 0.0 };

        if (s.b[320] && s.b[463]) {
            s.store_div_scaled_inputs3(426, s.ad_value(330), 1.0, s.ad_value(367), -1.0, s.ad_value(364), (-(-(p.p51 * 0.5))), s.ad_value(379), 1.0);
        }

        s.b[464] = (s.v[426] > 50.0);
        s.v[464] = if s.b[464] { 1.0 } else { 0.0 };

        if ((s.b[320] && s.b[463]) && s.b[464]) {
            s.copy_ad(427, 426);
        }

        s.b[465] = (s.v[426] < (-50.0));
        s.v[465] = if s.b[465] { 1.0 } else { 0.0 };

        if (((s.b[320] && s.b[463]) && (!s.b[464])) && s.b[465]) {
            s.store_exp(427, 426);
        }

        if (((s.b[320] && s.b[463]) && (!s.b[464])) && (!s.b[465])) {
            s.store_ln_one_plus_exp(427, 426);
        }

        if (s.b[320] && s.b[463]) {
            s.store_mul_ad_product_lhs(327, A::mul3(A::mul3(s.ad_value(339), s.ad_value(361), s.ad_value(362)), s.ad_value(342), s.ad_value(379)), s.ad_value(427), 363);
        }

        if (s.b[320] && (!s.b[463])) {
            s.store_scalar(327, 0.0);
        }

        if s.b[320] {
            s.copy_ad(209, 323);
            s.copy_ad(210, 324);
            s.copy_ad(211, 325);
            s.copy_ad(212, 326);
            s.copy_ad(213, 327);
        }

        s.b[466] = (p.p232 == 1.0);
        s.v[466] = if s.b[466] { 1.0 } else { 0.0 };

        s.v[203] = 0.0;

        s.v[204] = 0.0;

        s.v[205] = 0.0;

        s.v[206] = 0.0;

        s.v[207] = 0.0;

        s.b[467] = (p.p211 > p.p354);
        s.v[467] = if s.b[467] { 1.0 } else { 0.0 };

        if s.b[467] {
            s.store_scalar(470, 0.0);
            s.store_scalar(471, 0.0);
            s.store_scalar(472, 0.0);
            s.store_scalar(473, 0.0);
            s.store_scalar(474, 0.0);
            s.store_scalar(475, 0.0);
            s.store_scalar(476, 0.0);
            s.copy_ad(477, 96);
            s.copy_ad(478, 97);
            s.store_scalar(479, p.p217);
            s.copy_ad(480, 98);
            s.copy_ad(481, 99);
            s.store_scalar(482, p.p215);
            s.copy_ad(483, 111);
            s.store_scalar(484, s.v[109]);
            s.copy_ad(485, 113);
            s.store_scalar(486, p.p0);
            s.store_scalar(487, p.p211);
            s.copy_ad(488, 38);
            s.store_scalar(489, p.p216);
            s.copy_ad(490, 39);
            s.copy_ad(491, 40);
            s.store_scalar(492, p.p212);
            s.store_scalar(493, p.p226);
            s.store_scalar(494, p.p225);
            s.store_scalar(495, 0.0);
            s.store_scalar(496, p.p227);
            s.store_scalar(497, p.p231);
            s.store_scalar(498, p.p222);
            s.store_scalar(499, p.p223);
            s.store_scalar(500, p.p224);
            s.store_scalar(501, p.p230);
            s.store_scalar(502, p.p229);
            s.store_scalar(503, p.p228);
            s.store_scalar(504, p.p39);
            s.store_scalar(505, p.p47);
            s.store_scalar(506, p.p45);
            s.store_scalar(507, p.p42);
            s.store_scalar(508, p.p2);
            s.store_scalar(509, p.p6);
            s.store_scalar(510, 1.0);
            s.store_scalar(511, 0.0);
            s.store_scalar(512, 0.0);
            s.store_scalar(513, 0.0);
            s.store_scalar(514, 0.0);
            s.store_scalar(515, 0.0);
            s.store_scalar(516, 0.0);
            s.store_scalar(517, 0.0);
            s.store_scalar(518, 0.0);
            s.store_scalar(519, 0.0);
            s.store_scalar(520, 0.0);
            s.store_scalar(521, 0.0);
            s.store_scalar(522, 0.0);
            s.store_scalar(524, 0.0);
            s.store_scalar(525, 0.0);
            s.store_scalar(526, 0.0);
            s.store_scalar(527, 0.0);
            s.store_scalar(528, 0.0);
            s.store_scalar(529, 0.0);
            s.store_scalar(530, 0.0);
            s.store_scalar(531, 0.0);
            s.store_scalar(532, 0.0);
            s.store_scalar(533, 0.0);
            s.store_scalar(534, 0.0);
            s.store_scalar(535, 0.0);
            s.store_scalar(536, 0.0);
            s.store_scalar(537, 0.0);
            s.store_scalar(538, 0.0);
            s.store_scalar(539, 0.0);
            s.store_scalar(540, 0.0);
            s.store_scalar(541, 0.0);
            s.store_scalar(542, 0.0);
            s.store_scalar(543, 0.0);
            s.store_scalar(544, 0.0);
            s.store_scalar(545, 0.0);
            s.store_scalar(546, 0.0);
            s.store_scalar(547, 0.0);
            s.store_scalar(548, 0.0);
            s.store_scalar(549, 0.0);
            s.store_scalar(552, 0.0);
            s.store_scalar(553, 0.0);
            s.store_scalar(554, 0.0);
            s.store_scalar(555, 0.0);
            s.store_scalar(556, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_4(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[467] {
            s.store_scalar(557, 0.0);
            s.store_scalar(558, 0.0);
            s.store_scalar(559, 0.0);
            s.store_scalar(560, 0.0);
            s.store_scalar(561, 0.0);
            s.store_scalar(562, 0.0);
            s.store_scalar(563, 0.0);
            s.store_scalar(564, 0.0);
            s.store_scalar(565, 0.0);
            s.store_scalar(566, 0.0);
            s.store_scalar(567, 0.0);
            s.store_scalar(568, 0.0);
            s.store_scalar(569, 0.0);
            s.store_scalar(570, 0.0);
            s.store_scalar(571, 0.0);
            s.store_scalar(572, 0.0);
            s.store_scalar(573, 0.0);
            s.store_scalar(574, 0.0);
            s.store_scalar(575, 0.0);
            s.store_scalar(576, 0.0);
            s.store_scalar(577, 0.0);
            s.store_scalar(578, 0.0);
            s.store_scalar(579, 0.0);
        }

        if s.b[467] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(576, 478, A::tanh_scaled_input(s.ad_value(478), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(576, 478, p.p53);
                } else {
                    s.store_scalar(576, 0.0);
                }
            }
        }

        if s.b[467] {
            s.store_sub(577, 477, 478);
            s.store_mul(511, 497, 485);
            s.store_add_scaled_product_value_ad(513, A::div_scaled_inputs(s.ad_value(493), 1.0, s.ad_value(485), 2.302585092994046), 1.0, 496, 576, 1.0);
            s.store_add_scaled_product_right_ad(514, 492, 1.0, 503, A::sub(s.ad_value(483), s.ad_value(484)), 1.0);
            s.store_pow_ad(532, A::div(s.ad_value(483), s.ad_value(484)), s.ad_value(505));
        }

        s.b[580] = (s.v[504] != 0.0);
        s.v[580] = if s.b[580] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[580]) {
            s.store_div_ad_rhs(515, 576, A::pow(A::offset(A::pow(A::div(s.ad_value(576), s.ad_value(504)), s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500))));
        }

        if (s.b[467] && (!s.b[580])) {
            s.store_scalar(515, 0.0);
        }

        if s.b[467] {
            s.store_mul_add_scaled_product_rhs(512, 576, s.ad_value(494), 1.0, s.ad_value(515), s.ad_value(495), (-1.0));
            s.store_sub(475, 514, 512);
            s.store_scaled_mul(517, 513, 485, 2.0);
            s.store_mul(518, 488, 517);
            s.store_sub_scaled_inputs(575, 475, 1.0, 511, (p.p51 * 0.5));
        }

        if s.b[467] {
            let assign5860_ad_e6939: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::sub(s.ad_value(477), s.ad_value(577))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(574, assign5860_ad_e6939, 1.0, s.ad_value(575), (-1.0), s.ad_value(511), 1.0);
        }

        s.b[581] = (s.v[574] > 50.0);
        s.v[581] = if s.b[581] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[581]) {
            s.store_scalar(533, 0.0);
        }

        s.b[582] = (s.v[574] < (-50.0));
        s.v[582] = if s.b[582] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[581])) && s.b[582]) {
            s.store_scalar(533, 1.0);
        }

        if ((s.b[467] && (!s.b[581])) && (!s.b[582])) {
            s.store_div_from_scalar_offset_ad(533, 1.0, A::exp(s.ad_value(574)), 1.0);
        }

        if s.b[467] {
            let assign5920_ad_e7027: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::sub(s.ad_value(477), s.ad_value(577))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(534, assign5920_ad_e7027, 1.0, A::add_scaled_product(s.ad_value(475), 1.0, s.ad_value(511), s.ad_value(533), (-(p.p51 * 0.1))), (-1.0), s.ad_value(517), 1.0);
        }

        s.b[583] = (s.v[534] > 50.0);
        s.v[583] = if s.b[583] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[583]) {
            s.store_mul(535, 518, 534);
        }

        s.b[584] = (s.v[534] < (-50.0));
        s.v[584] = if s.b[584] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[583])) && s.b[584]) {
            s.store_mul_exp_rhs(535, 518, 534);
        }

        if ((s.b[467] && (!s.b[583])) && (!s.b[584])) {
            s.store_mul_ln_one_plus_exp_rhs(535, 518, 534);
        }

        if s.b[467] {
            s.store_div_ad_rhs(521, 499, A::mul_offset_rhs(s.ad_value(532), A::div_scaled_product(s.ad_value(501), s.ad_value(535), 1.0, s.ad_value(488), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(522, 498, A::div_scaled_offset_numerator(A::mul(s.ad_value(506), s.ad_value(484)), 1.0, 1.0, A::offset(A::mul(s.ad_value(506), s.ad_value(483)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(507), s.ad_value(576), 1.0, s.ad_value(487), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(502), s.ad_value(535), 1.0, s.ad_value(488), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(539, 522, 487, 1.0, 521, 1.0);
            s.store_add_scaled_product_right_ad(540, 539, (-1.0), 539, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(535), 2.0, s.ad_value(488), s.ad_value(539), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(541, A::mul_sub_from_scalar_rhs(s.ad_value(539), 1.0, s.ad_value(533)), 1.0, 517, 533, 1.0);
            s.store_add_scaled_product_value_ad(476, A::mul_sub_from_scalar_rhs(s.ad_value(540), 1.0, s.ad_value(533)), 1.0, 517, 533, 1.0);
        }

        if s.b[467] {
            let assign6050_ad_e7256: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(478), s.ad_value(476)), 0.5, A::div(s.ad_value(478), s.ad_value(476)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(478), s.ad_value(476))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(478), s.ad_value(476)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(478), s.ad_value(476)), A::div(s.ad_value(478), s.ad_value(476)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(542, 1.0, A::offset(A::pow(assign6050_ad_e7256, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul(543, 478, 542);
        }

        if s.b[467] {
            let assign6070_ad_e7337: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(476), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(544, 1.0, A::offset(A::pow(assign6070_ad_e7337, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul_neg_lhs(545, 478, 544);
            s.store_div_scaled_inputs2(574, s.ad_value(477), 1.0, s.ad_value(575), (-1.0), s.ad_value(511), 1.0);
        }

        s.b[585] = (s.v[574] > 50.0);
        s.v[585] = if s.b[585] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[585]) {
            s.store_scalar(516, 0.0);
        }

        s.b[586] = (s.v[574] < (-50.0));
        s.v[586] = if s.b[586] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[585])) && s.b[586]) {
            s.store_scalar(516, 1.0);
        }

        if ((s.b[467] && (!s.b[585])) && (!s.b[586])) {
            s.store_div_from_scalar_offset_ad(516, 1.0, A::exp(s.ad_value(574)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs3(519, s.ad_value(577), 1.0, s.ad_value(545), (-1.0), A::add_scaled_product(s.ad_value(475), 1.0, s.ad_value(511), s.ad_value(516), (-(p.p51 * 0.1))), -1.0, s.ad_value(517), 1.0);
        }

        s.b[587] = (s.v[519] > 50.0);
        s.v[587] = if s.b[587] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[587]) {
            s.store_mul(520, 518, 519);
        }

        s.b[588] = (s.v[519] < (-50.0));
        s.v[588] = if s.b[588] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[587])) && s.b[588]) {
            s.store_mul_exp_rhs(520, 518, 519);
        }

        if ((s.b[467] && (!s.b[587])) && (!s.b[588])) {
            s.store_mul_ln_one_plus_exp_rhs(520, 518, 519);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2(574, s.ad_value(577), 1.0, s.ad_value(575), (-1.0), s.ad_value(511), 1.0);
        }

        s.b[589] = (s.v[574] > 50.0);
        s.v[589] = if s.b[589] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[589]) {
            s.store_scalar(546, 0.0);
        }

        s.b[590] = (s.v[574] < (-50.0));
        s.v[590] = if s.b[590] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[589])) && s.b[590]) {
            s.store_scalar(546, 1.0);
        }

        if ((s.b[467] && (!s.b[589])) && (!s.b[590])) {
            s.store_div_from_scalar_offset_ad(546, 1.0, A::exp(s.ad_value(574)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs3(547, s.ad_value(477), 1.0, s.ad_value(543), (-1.0), A::add_scaled_product(s.ad_value(475), 1.0, s.ad_value(511), s.ad_value(546), (-(p.p51 * 0.1))), -1.0, s.ad_value(517), 1.0);
        }

        s.b[591] = (s.v[547] > 50.0);
        s.v[591] = if s.b[591] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[591]) {
            s.store_mul(548, 518, 547);
        }

        s.b[592] = (s.v[547] < (-50.0));
        s.v[592] = if s.b[592] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[591])) && s.b[592]) {
            s.store_mul_exp_rhs(548, 518, 547);
        }

        if ((s.b[467] && (!s.b[591])) && (!s.b[592])) {
            s.store_mul_ln_one_plus_exp_rhs(548, 518, 547);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2(549, s.ad_value(520), 1.0, s.ad_value(548), (-1.0), s.ad_value(488), 1.0);
            s.store_div(575, 549, 541);
            s.store_div_scaled_inputs(524, s.ad_value(493), 1.0, s.ad_value(485), 2.302585092994046);
            s.store_scaled_mul(526, 524, 485, 2.0);
            s.store_mul(527, 488, 526);
            s.store_sub_scaled_inputs(579, 514, 1.0, 511, (p.p51 * 0.5));
        }

        if s.b[467] {
            let assign6420_ad_e7718: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::sub(s.ad_value(477), s.ad_value(577))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(578, assign6420_ad_e7718, 1.0, s.ad_value(579), (-1.0), s.ad_value(511), 1.0);
        }

        s.b[593] = (s.v[578] > 50.0);
        s.v[593] = if s.b[593] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[593]) {
            s.store_scalar(536, 0.0);
        }

        s.b[594] = (s.v[578] < (-50.0));
        s.v[594] = if s.b[594] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[593])) && s.b[594]) {
            s.store_scalar(536, 1.0);
        }

        if ((s.b[467] && (!s.b[593])) && (!s.b[594])) {
            s.store_div_from_scalar_offset_ad(536, 1.0, A::exp(s.ad_value(578)), 1.0);
        }

        if s.b[467] {
            let assign6480_ad_e7806: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sub(s.ad_value(477), s.ad_value(577)), A::tanh_scaled_input(A::sub(s.ad_value(477), s.ad_value(577)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(477), 0.5, s.ad_value(577), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(477), s.ad_value(577)), A::sub(s.ad_value(477), s.ad_value(577))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(537, assign6480_ad_e7806, 1.0, A::add_scaled_product(s.ad_value(514), 1.0, s.ad_value(511), s.ad_value(536), (-(p.p51 * 0.1))), (-1.0), s.ad_value(526), 1.0);
        }

        s.b[595] = (s.v[537] > 50.0);
        s.v[595] = if s.b[595] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[595]) {
            s.store_mul(538, 527, 537);
        }

        s.b[596] = (s.v[537] < (-50.0));
        s.v[596] = if s.b[596] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[595])) && s.b[596]) {
            s.store_mul_exp_rhs(538, 527, 537);
        }

        if ((s.b[467] && (!s.b[595])) && (!s.b[596])) {
            s.store_mul_ln_one_plus_exp_rhs(538, 527, 537);
        }

        if s.b[467] {
            s.store_div(530, 499, 532);
            s.store_mul_div_scaled_offset_numerator_rhs(531, 498, A::mul(s.ad_value(506), s.ad_value(484)), 1.0, 1.0, A::offset(A::mul(s.ad_value(506), s.ad_value(483)), 1.0), 1.0);
            s.store_div_scaled_product_indices(552, 531, 487, 1.0, 530, 1.0);
            s.store_add_scaled_product_right_ad(553, 552, (-1.0), 552, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(538), 2.0, s.ad_value(488), s.ad_value(552), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(554, A::mul_sub_from_scalar_rhs(s.ad_value(553), 1.0, s.ad_value(536)), 1.0, 526, 536, 1.0);
        }

    }

    pub(super) fn stamp_reactive_block_5(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[467] {
            let assign6590_ad_e7981: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(478), s.ad_value(554)), 0.5, A::div(s.ad_value(478), s.ad_value(554)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(478), s.ad_value(554))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(478), s.ad_value(554)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(478), s.ad_value(554)), A::div(s.ad_value(478), s.ad_value(554)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(555, 1.0, A::offset(A::pow(assign6590_ad_e7981, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul(556, 478, 555);
        }

        if s.b[467] {
            let assign6610_ad_e8062: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), A::div_scaled_inputs(s.ad_value(478), -1.0, s.ad_value(554), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(557, 1.0, A::offset(A::pow(assign6610_ad_e8062, s.ad_value(500)), 1.0), A::div_from_scalar(1.0, s.ad_value(500)));
        }

        if s.b[467] {
            s.store_mul_neg_lhs(558, 478, 557);
            s.store_div_scaled_inputs2(578, s.ad_value(477), 1.0, s.ad_value(579), (-1.0), s.ad_value(511), 1.0);
        }

        s.b[597] = (s.v[578] > 50.0);
        s.v[597] = if s.b[597] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[597]) {
            s.store_scalar(525, 0.0);
        }

        s.b[598] = (s.v[578] < (-50.0));
        s.v[598] = if s.b[598] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[597])) && s.b[598]) {
            s.store_scalar(525, 1.0);
        }

        if ((s.b[467] && (!s.b[597])) && (!s.b[598])) {
            s.store_div_from_scalar_offset_ad(525, 1.0, A::exp(s.ad_value(578)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs3(528, s.ad_value(577), 1.0, s.ad_value(558), (-1.0), A::add_scaled_product(s.ad_value(514), 1.0, s.ad_value(511), s.ad_value(525), (-(p.p51 * 0.1))), -1.0, s.ad_value(526), 1.0);
        }

        s.b[599] = (s.v[528] > 50.0);
        s.v[599] = if s.b[599] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[599]) {
            s.store_mul(529, 527, 528);
        }

        s.b[600] = (s.v[528] < (-50.0));
        s.v[600] = if s.b[600] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[599])) && s.b[600]) {
            s.store_mul_exp_rhs(529, 527, 528);
        }

        if ((s.b[467] && (!s.b[599])) && (!s.b[600])) {
            s.store_mul_ln_one_plus_exp_rhs(529, 527, 528);
        }

        if s.b[467] {
            s.store_div_scaled_inputs2(578, s.ad_value(577), 1.0, s.ad_value(579), (-1.0), s.ad_value(511), 1.0);
        }

        s.b[601] = (s.v[578] > 50.0);
        s.v[601] = if s.b[601] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[601]) {
            s.store_scalar(559, 0.0);
        }

        s.b[602] = (s.v[578] < (-50.0));
        s.v[602] = if s.b[602] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[601])) && s.b[602]) {
            s.store_scalar(559, 1.0);
        }

        if ((s.b[467] && (!s.b[601])) && (!s.b[602])) {
            s.store_div_from_scalar_offset_ad(559, 1.0, A::exp(s.ad_value(578)), 1.0);
        }

        if s.b[467] {
            s.store_div_scaled_inputs3(560, s.ad_value(477), 1.0, s.ad_value(556), (-1.0), A::add_scaled_product(s.ad_value(514), 1.0, s.ad_value(511), s.ad_value(559), (-(p.p51 * 0.1))), -1.0, s.ad_value(526), 1.0);
        }

        s.b[603] = (s.v[560] > 50.0);
        s.v[603] = if s.b[603] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[603]) {
            s.store_mul(561, 527, 560);
        }

        s.b[604] = (s.v[560] < (-50.0));
        s.v[604] = if s.b[604] { 1.0 } else { 0.0 };

        if ((s.b[467] && (!s.b[603])) && s.b[604]) {
            s.store_mul_exp_rhs(561, 527, 560);
        }

        if ((s.b[467] && (!s.b[603])) && (!s.b[604])) {
            s.store_mul_ln_one_plus_exp_rhs(561, 527, 560);
        }

        if s.b[467] {
            s.store_offset_square(562, 529, 1e-38);
            s.store_offset_mul(563, 562, 529, 1e-57);
            s.store_offset_square(564, 561, 1e-38);
            s.store_offset_mul(565, 564, 561, 1e-57);
            s.store_offset_mul(566, 529, 561, 1e-38);
            s.store_div_scaled_inputs3(567, s.ad_value(562), (2.0 / 3.0), s.ad_value(564), (2.0 / 3.0), s.ad_value(566), (2.0 / 3.0), A::offset(A::add(s.ad_value(529), s.ad_value(561)), 2e-19), 1.0);
            s.store_div_ad(568, A::add_scaled_inputs_products(s.ad_value(563), (2.0 * 2.0), s.ad_value(565), (3.0 * 2.0), s.ad_value(562), s.ad_value(561), (4.0 * 2.0), s.ad_value(564), s.ad_value(529), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(562), 15.0, s.ad_value(564), 15.0, s.ad_value(566), (2.0 * 15.0)));
            s.store_sub(569, 567, 568);
            s.copy_ad(570, 568);
            s.store_mul_product3_rhs(470, 510, A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(487)), s.ad_value(509), s.ad_value(569), 1.0);
            s.store_mul_product3_rhs(471, 510, A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(487)), s.ad_value(509), s.ad_value(570), 1.0);
        }

        s.b[605] = (s.v[479] == 1.0);
        s.v[605] = if s.b[605] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[605]) {
            s.store_div_scaled_inputs3(571, s.ad_value(480), 1.0, s.ad_value(514), -1.0, s.ad_value(511), (-(-(p.p51 * 0.5))), s.ad_value(526), 1.0);
        }

        s.b[606] = (s.v[571] > 50.0);
        s.v[606] = if s.b[606] { 1.0 } else { 0.0 };

        if ((s.b[467] && s.b[605]) && s.b[606]) {
            s.copy_ad(574, 571);
        }

        s.b[607] = (s.v[571] < (-50.0));
        s.v[607] = if s.b[607] { 1.0 } else { 0.0 };

        if (((s.b[467] && s.b[605]) && (!s.b[606])) && s.b[607]) {
            s.store_exp(574, 571);
        }

        if (((s.b[467] && s.b[605]) && (!s.b[606])) && (!s.b[607])) {
            s.store_ln_one_plus_exp(574, 571);
        }

        if (s.b[467] && s.b[605]) {
            s.store_mul_ad_product_lhs(472, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(509)), s.ad_value(490), s.ad_value(526)), s.ad_value(574), 510);
            s.store_div_scaled_inputs3(572, s.ad_value(481), 1.0, s.ad_value(514), -1.0, s.ad_value(511), (-(-(p.p51 * 0.5))), s.ad_value(526), 1.0);
        }

        s.b[608] = (s.v[572] > 50.0);
        s.v[608] = if s.b[608] { 1.0 } else { 0.0 };

        if ((s.b[467] && s.b[605]) && s.b[608]) {
            s.copy_ad(574, 572);
        }

        s.b[609] = (s.v[572] < (-50.0));
        s.v[609] = if s.b[609] { 1.0 } else { 0.0 };

        if (((s.b[467] && s.b[605]) && (!s.b[608])) && s.b[609]) {
            s.store_exp(574, 572);
        }

        if (((s.b[467] && s.b[605]) && (!s.b[608])) && (!s.b[609])) {
            s.store_ln_one_plus_exp(574, 572);
        }

        if (s.b[467] && s.b[605]) {
            s.store_mul_ad_product_lhs(473, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(509)), s.ad_value(491), s.ad_value(526)), s.ad_value(574), 510);
        }

        if (s.b[467] && (!s.b[605])) {
            s.store_scalar(472, 0.0);
            s.store_scalar(473, 0.0);
        }

        s.b[610] = (s.v[482] == 1.0);
        s.v[610] = if s.b[610] { 1.0 } else { 0.0 };

        if (s.b[467] && s.b[610]) {
            s.store_div_scaled_inputs3(573, s.ad_value(477), 1.0, s.ad_value(514), -1.0, s.ad_value(511), (-(-(p.p51 * 0.5))), s.ad_value(526), 1.0);
        }

        s.b[611] = (s.v[573] > 50.0);
        s.v[611] = if s.b[611] { 1.0 } else { 0.0 };

        if ((s.b[467] && s.b[610]) && s.b[611]) {
            s.copy_ad(574, 573);
        }

        s.b[612] = (s.v[573] < (-50.0));
        s.v[612] = if s.b[612] { 1.0 } else { 0.0 };

        if (((s.b[467] && s.b[610]) && (!s.b[611])) && s.b[612]) {
            s.store_exp(574, 573);
        }

        if (((s.b[467] && s.b[610]) && (!s.b[611])) && (!s.b[612])) {
            s.store_ln_one_plus_exp(574, 573);
        }

        if (s.b[467] && s.b[610]) {
            s.store_mul_ad_product_lhs(474, A::mul3(A::mul3(s.ad_value(486), s.ad_value(508), s.ad_value(509)), s.ad_value(489), s.ad_value(526)), s.ad_value(574), 510);
        }

        if (s.b[467] && (!s.b[610])) {
            s.store_scalar(474, 0.0);
        }

        if s.b[467] {
            s.copy_ad(203, 470);
            s.copy_ad(204, 471);
            s.copy_ad(205, 472);
            s.copy_ad(206, 473);
            s.copy_ad(207, 474);
        }

        s.b[613] = (p.p210 == 1.0);
        s.v[613] = if s.b[613] { 1.0 } else { 0.0 };

        s.v[197] = 0.0;

        s.v[198] = 0.0;

        s.v[199] = 0.0;

        s.v[200] = 0.0;

        s.v[201] = 0.0;

        s.b[614] = (p.p189 > p.p354);
        s.v[614] = if s.b[614] { 1.0 } else { 0.0 };

        if s.b[614] {
            s.store_scalar(617, 0.0);
            s.store_scalar(618, 0.0);
            s.store_scalar(619, 0.0);
            s.store_scalar(620, 0.0);
            s.store_scalar(621, 0.0);
            s.store_scalar(622, 0.0);
            s.store_scalar(623, 0.0);
            s.copy_ad(624, 90);
            s.copy_ad(625, 91);
            s.store_scalar(626, p.p195);
            s.copy_ad(627, 92);
            s.copy_ad(628, 93);
            s.store_scalar(629, p.p193);
            s.copy_ad(630, 111);
            s.store_scalar(631, s.v[109]);
            s.copy_ad(632, 113);
            s.store_scalar(633, p.p0);
            s.store_scalar(634, p.p189);
            s.copy_ad(635, 35);
            s.store_scalar(636, p.p194);
            s.copy_ad(637, 36);
            s.copy_ad(638, 37);
            s.store_scalar(639, p.p190);
            s.store_scalar(640, p.p204);
            s.store_scalar(641, p.p203);
            s.store_scalar(642, 0.0);
            s.store_scalar(643, p.p205);
            s.store_scalar(644, p.p209);
            s.store_scalar(645, p.p200);
            s.store_scalar(646, p.p201);
            s.store_scalar(647, p.p202);
            s.store_scalar(648, p.p208);
            s.store_scalar(649, p.p207);
            s.store_scalar(650, p.p206);
            s.store_scalar(651, p.p39);
            s.store_scalar(652, p.p47);
            s.store_scalar(653, p.p45);
            s.store_scalar(654, p.p42);
            s.store_scalar(655, p.p2);
            s.store_scalar(656, p.p6);
            s.store_scalar(657, 1.0);
            s.store_scalar(658, 0.0);
            s.store_scalar(659, 0.0);
            s.store_scalar(660, 0.0);
            s.store_scalar(661, 0.0);
            s.store_scalar(662, 0.0);
            s.store_scalar(663, 0.0);
            s.store_scalar(664, 0.0);
            s.store_scalar(665, 0.0);
            s.store_scalar(666, 0.0);
            s.store_scalar(667, 0.0);
        }

    }

    pub(super) fn stamp_reactive_block_6(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[614] {
            s.store_scalar(668, 0.0);
            s.store_scalar(669, 0.0);
            s.store_scalar(671, 0.0);
            s.store_scalar(672, 0.0);
            s.store_scalar(673, 0.0);
            s.store_scalar(674, 0.0);
            s.store_scalar(675, 0.0);
            s.store_scalar(676, 0.0);
            s.store_scalar(677, 0.0);
            s.store_scalar(678, 0.0);
            s.store_scalar(679, 0.0);
            s.store_scalar(680, 0.0);
            s.store_scalar(681, 0.0);
            s.store_scalar(682, 0.0);
            s.store_scalar(683, 0.0);
            s.store_scalar(684, 0.0);
            s.store_scalar(685, 0.0);
            s.store_scalar(686, 0.0);
            s.store_scalar(687, 0.0);
            s.store_scalar(688, 0.0);
            s.store_scalar(689, 0.0);
            s.store_scalar(690, 0.0);
            s.store_scalar(691, 0.0);
            s.store_scalar(692, 0.0);
            s.store_scalar(693, 0.0);
            s.store_scalar(694, 0.0);
            s.store_scalar(695, 0.0);
            s.store_scalar(696, 0.0);
            s.store_scalar(699, 0.0);
            s.store_scalar(700, 0.0);
            s.store_scalar(701, 0.0);
            s.store_scalar(702, 0.0);
            s.store_scalar(703, 0.0);
            s.store_scalar(704, 0.0);
            s.store_scalar(705, 0.0);
            s.store_scalar(706, 0.0);
            s.store_scalar(707, 0.0);
            s.store_scalar(708, 0.0);
            s.store_scalar(709, 0.0);
            s.store_scalar(710, 0.0);
            s.store_scalar(711, 0.0);
            s.store_scalar(712, 0.0);
            s.store_scalar(713, 0.0);
            s.store_scalar(714, 0.0);
            s.store_scalar(715, 0.0);
            s.store_scalar(716, 0.0);
            s.store_scalar(717, 0.0);
            s.store_scalar(718, 0.0);
            s.store_scalar(719, 0.0);
            s.store_scalar(720, 0.0);
            s.store_scalar(721, 0.0);
            s.store_scalar(722, 0.0);
            s.store_scalar(723, 0.0);
            s.store_scalar(724, 0.0);
            s.store_scalar(725, 0.0);
            s.store_scalar(726, 0.0);
        }

        if s.b[614] {
            if (p.p52 != 0.0) {
                s.store_mul_ad_rhs(723, 625, A::tanh_scaled_input(s.ad_value(625), (0.001 / p.p53)));
            } else {
                if (p.p52 == 0.0) {
                    s.store_sqrt_square_offset(723, 625, p.p53);
                } else {
                    s.store_scalar(723, 0.0);
                }
            }
        }

        if s.b[614] {
            s.store_sub(724, 624, 625);
            s.store_mul(658, 644, 632);
            s.store_add_scaled_product_value_ad(660, A::div_scaled_inputs(s.ad_value(640), 1.0, s.ad_value(632), 2.302585092994046), 1.0, 643, 723, 1.0);
            s.store_add_scaled_product_right_ad(661, 639, 1.0, 650, A::sub(s.ad_value(630), s.ad_value(631)), 1.0);
            s.store_pow_ad(679, A::div(s.ad_value(630), s.ad_value(631)), s.ad_value(652));
        }

        s.b[727] = (s.v[651] != 0.0);
        s.v[727] = if s.b[727] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[727]) {
            s.store_div_ad_rhs(662, 723, A::pow(A::offset(A::pow(A::div(s.ad_value(723), s.ad_value(651)), s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647))));
        }

        if (s.b[614] && (!s.b[727])) {
            s.store_scalar(662, 0.0);
        }

        if s.b[614] {
            s.store_mul_add_scaled_product_rhs(659, 723, s.ad_value(641), 1.0, s.ad_value(662), s.ad_value(642), (-1.0));
            s.store_sub(622, 661, 659);
            s.store_scaled_mul(664, 660, 632, 2.0);
            s.store_mul(665, 635, 664);
            s.store_sub_scaled_inputs(722, 622, 1.0, 658, (p.p51 * 0.5));
        }

        if s.b[614] {
            let assign8700_ad_e9363: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::sub(s.ad_value(624), s.ad_value(724))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(721, assign8700_ad_e9363, 1.0, s.ad_value(722), (-1.0), s.ad_value(658), 1.0);
        }

        s.b[728] = (s.v[721] > 50.0);
        s.v[728] = if s.b[728] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[728]) {
            s.store_scalar(680, 0.0);
        }

        s.b[729] = (s.v[721] < (-50.0));
        s.v[729] = if s.b[729] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[728])) && s.b[729]) {
            s.store_scalar(680, 1.0);
        }

        if ((s.b[614] && (!s.b[728])) && (!s.b[729])) {
            s.store_div_from_scalar_offset_ad(680, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            let assign8760_ad_e9451: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::sub(s.ad_value(624), s.ad_value(724))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(681, assign8760_ad_e9451, 1.0, A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(680), (-(p.p51 * 0.1))), (-1.0), s.ad_value(664), 1.0);
        }

        s.b[730] = (s.v[681] > 50.0);
        s.v[730] = if s.b[730] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[730]) {
            s.store_mul(682, 665, 681);
        }

        s.b[731] = (s.v[681] < (-50.0));
        s.v[731] = if s.b[731] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[730])) && s.b[731]) {
            s.store_mul_exp_rhs(682, 665, 681);
        }

        if ((s.b[614] && (!s.b[730])) && (!s.b[731])) {
            s.store_mul_ln_one_plus_exp_rhs(682, 665, 681);
        }

        if s.b[614] {
            s.store_div_ad_rhs(668, 646, A::mul_offset_rhs(s.ad_value(679), A::div_scaled_product(s.ad_value(648), s.ad_value(682), 1.0, s.ad_value(635), 1.0), 1.0));
            s.store_div_scaled_product3_mixed_iaaa(669, 645, A::div_scaled_offset_numerator(A::mul(s.ad_value(653), s.ad_value(631)), 1.0, 1.0, A::offset(A::mul(s.ad_value(653), s.ad_value(630)), 1.0), 1.0), A::offset(A::div_scaled_product(s.ad_value(654), s.ad_value(723), 1.0, s.ad_value(634), 1.0), 1.0), 1.0, A::offset(A::div_scaled_product(s.ad_value(649), s.ad_value(682), 1.0, s.ad_value(635), 1.0), 1.0), 1.0);
            s.store_div_scaled_product_indices(686, 669, 634, 1.0, 668, 1.0);
            s.store_add_scaled_product_right_ad(687, 686, (-1.0), 686, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(682), 2.0, s.ad_value(635), s.ad_value(686), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(688, A::mul_sub_from_scalar_rhs(s.ad_value(686), 1.0, s.ad_value(680)), 1.0, 664, 680, 1.0);
            s.store_add_scaled_product_value_ad(623, A::mul_sub_from_scalar_rhs(s.ad_value(687), 1.0, s.ad_value(680)), 1.0, 664, 680, 1.0);
        }

        if s.b[614] {
            let assign8890_ad_e9680: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(625), s.ad_value(623)), 0.5, A::div(s.ad_value(625), s.ad_value(623)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(625), s.ad_value(623))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(625), s.ad_value(623)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(625), s.ad_value(623)), A::div(s.ad_value(625), s.ad_value(623)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(689, 1.0, A::offset(A::pow(assign8890_ad_e9680, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul(690, 625, 689);
        }

        if s.b[614] {
            let assign8910_ad_e9761: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(623), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(691, 1.0, A::offset(A::pow(assign8910_ad_e9761, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul_neg_lhs(692, 625, 691);
            s.store_div_scaled_inputs2(721, s.ad_value(624), 1.0, s.ad_value(722), (-1.0), s.ad_value(658), 1.0);
        }

        s.b[732] = (s.v[721] > 50.0);
        s.v[732] = if s.b[732] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[732]) {
            s.store_scalar(663, 0.0);
        }

        s.b[733] = (s.v[721] < (-50.0));
        s.v[733] = if s.b[733] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[732])) && s.b[733]) {
            s.store_scalar(663, 1.0);
        }

        if ((s.b[614] && (!s.b[732])) && (!s.b[733])) {
            s.store_div_from_scalar_offset_ad(663, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3(666, s.ad_value(724), 1.0, s.ad_value(692), (-1.0), A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(663), (-(p.p51 * 0.1))), -1.0, s.ad_value(664), 1.0);
        }

        s.b[734] = (s.v[666] > 50.0);
        s.v[734] = if s.b[734] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[734]) {
            s.store_mul(667, 665, 666);
        }

        s.b[735] = (s.v[666] < (-50.0));
        s.v[735] = if s.b[735] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[734])) && s.b[735]) {
            s.store_mul_exp_rhs(667, 665, 666);
        }

        if ((s.b[614] && (!s.b[734])) && (!s.b[735])) {
            s.store_mul_ln_one_plus_exp_rhs(667, 665, 666);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2(721, s.ad_value(724), 1.0, s.ad_value(722), (-1.0), s.ad_value(658), 1.0);
        }

        s.b[736] = (s.v[721] > 50.0);
        s.v[736] = if s.b[736] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[736]) {
            s.store_scalar(693, 0.0);
        }

        s.b[737] = (s.v[721] < (-50.0));
        s.v[737] = if s.b[737] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[736])) && s.b[737]) {
            s.store_scalar(693, 1.0);
        }

        if ((s.b[614] && (!s.b[736])) && (!s.b[737])) {
            s.store_div_from_scalar_offset_ad(693, 1.0, A::exp(s.ad_value(721)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3(694, s.ad_value(624), 1.0, s.ad_value(690), (-1.0), A::add_scaled_product(s.ad_value(622), 1.0, s.ad_value(658), s.ad_value(693), (-(p.p51 * 0.1))), -1.0, s.ad_value(664), 1.0);
        }

        s.b[738] = (s.v[694] > 50.0);
        s.v[738] = if s.b[738] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[738]) {
            s.store_mul(695, 665, 694);
        }

        s.b[739] = (s.v[694] < (-50.0));
        s.v[739] = if s.b[739] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[738])) && s.b[739]) {
            s.store_mul_exp_rhs(695, 665, 694);
        }

    }

    pub(super) fn stamp_reactive_block_7(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[614] && (!s.b[738])) && (!s.b[739])) {
            s.store_mul_ln_one_plus_exp_rhs(695, 665, 694);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2(696, s.ad_value(667), 1.0, s.ad_value(695), (-1.0), s.ad_value(635), 1.0);
            s.store_div(722, 696, 688);
            s.store_div_scaled_inputs(671, s.ad_value(640), 1.0, s.ad_value(632), 2.302585092994046);
            s.store_scaled_mul(673, 671, 632, 2.0);
            s.store_mul(674, 635, 673);
            s.store_sub_scaled_inputs(726, 661, 1.0, 658, (p.p51 * 0.5));
        }

        if s.b[614] {
            let assign9260_ad_e10142: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::sub(s.ad_value(624), s.ad_value(724))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(725, assign9260_ad_e10142, 1.0, s.ad_value(726), (-1.0), s.ad_value(658), 1.0);
        }

        s.b[740] = (s.v[725] > 50.0);
        s.v[740] = if s.b[740] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[740]) {
            s.store_scalar(683, 0.0);
        }

        s.b[741] = (s.v[725] < (-50.0));
        s.v[741] = if s.b[741] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[740])) && s.b[741]) {
            s.store_scalar(683, 1.0);
        }

        if ((s.b[614] && (!s.b[740])) && (!s.b[741])) {
            s.store_div_from_scalar_offset_ad(683, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            let assign9320_ad_e10230: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_inputs_product(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sub(s.ad_value(624), s.ad_value(724)), A::tanh_scaled_input(A::sub(s.ad_value(624), s.ad_value(724)), (0.001 / p.p53)), 0.5)
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs3(s.ad_value(624), 0.5, s.ad_value(724), 0.5, A::sqrt(A::offset(A::mul(A::sub(s.ad_value(624), s.ad_value(724)), A::sub(s.ad_value(624), s.ad_value(724))), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_scaled_inputs2(684, assign9320_ad_e10230, 1.0, A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(683), (-(p.p51 * 0.1))), (-1.0), s.ad_value(673), 1.0);
        }

        s.b[742] = (s.v[684] > 50.0);
        s.v[742] = if s.b[742] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[742]) {
            s.store_mul(685, 674, 684);
        }

        s.b[743] = (s.v[684] < (-50.0));
        s.v[743] = if s.b[743] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[742])) && s.b[743]) {
            s.store_mul_exp_rhs(685, 674, 684);
        }

        if ((s.b[614] && (!s.b[742])) && (!s.b[743])) {
            s.store_mul_ln_one_plus_exp_rhs(685, 674, 684);
        }

        if s.b[614] {
            s.store_div(677, 646, 679);
            s.store_mul_div_scaled_offset_numerator_rhs(678, 645, A::mul(s.ad_value(653), s.ad_value(631)), 1.0, 1.0, A::offset(A::mul(s.ad_value(653), s.ad_value(630)), 1.0), 1.0);
            s.store_div_scaled_product_indices(699, 678, 634, 1.0, 677, 1.0);
            s.store_add_scaled_product_right_ad(700, 699, (-1.0), 699, A::sqrt(A::offset(A::div_scaled_value_by_product(s.ad_value(685), 2.0, s.ad_value(635), s.ad_value(699), 1.0), 1.0)), 1.0);
            s.store_add_scaled_product_value_ad(701, A::mul_sub_from_scalar_rhs(s.ad_value(700), 1.0, s.ad_value(683)), 1.0, 673, 683, 1.0);
        }

        if s.b[614] {
            let assign9430_ad_e10405: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div(s.ad_value(625), s.ad_value(701)), 0.5, A::div(s.ad_value(625), s.ad_value(701)), A::tanh_scaled_input(A::neg(A::div(s.ad_value(625), s.ad_value(701))), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div(s.ad_value(625), s.ad_value(701)), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div(s.ad_value(625), s.ad_value(701)), A::div(s.ad_value(625), s.ad_value(701)), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(702, 1.0, A::offset(A::pow(assign9430_ad_e10405, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul(703, 625, 702);
        }

        if s.b[614] {
            let assign9450_ad_e10486: A = {
                if (p.p52 != 0.0) {
                    A::add_scaled_product(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), 0.5, A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), A::tanh_scaled_input(A::neg(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0)), (0.001 / p.p53)), (-0.5))
                } else {
                    {
                        if (p.p52 == 0.0) {
                            A::add_scaled_inputs(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), 0.5, A::sqrt(A::offset(A::mul_scaled_output(A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), A::div_scaled_inputs(s.ad_value(625), -1.0, s.ad_value(701), 1.0), 1.0), p.p53)), 0.5)
                        } else {
                            A::constant(0.0)
                        }
                    }
                }
            };
            s.store_div_from_scalar_pow_ad(704, 1.0, A::offset(A::pow(assign9450_ad_e10486, s.ad_value(647)), 1.0), A::div_from_scalar(1.0, s.ad_value(647)));
        }

        if s.b[614] {
            s.store_mul_neg_lhs(705, 625, 704);
            s.store_div_scaled_inputs2(725, s.ad_value(624), 1.0, s.ad_value(726), (-1.0), s.ad_value(658), 1.0);
        }

        s.b[744] = (s.v[725] > 50.0);
        s.v[744] = if s.b[744] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[744]) {
            s.store_scalar(672, 0.0);
        }

        s.b[745] = (s.v[725] < (-50.0));
        s.v[745] = if s.b[745] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[744])) && s.b[745]) {
            s.store_scalar(672, 1.0);
        }

        if ((s.b[614] && (!s.b[744])) && (!s.b[745])) {
            s.store_div_from_scalar_offset_ad(672, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3(675, s.ad_value(724), 1.0, s.ad_value(705), (-1.0), A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(672), (-(p.p51 * 0.1))), -1.0, s.ad_value(673), 1.0);
        }

        s.b[746] = (s.v[675] > 50.0);
        s.v[746] = if s.b[746] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[746]) {
            s.store_mul(676, 674, 675);
        }

        s.b[747] = (s.v[675] < (-50.0));
        s.v[747] = if s.b[747] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[746])) && s.b[747]) {
            s.store_mul_exp_rhs(676, 674, 675);
        }

        if ((s.b[614] && (!s.b[746])) && (!s.b[747])) {
            s.store_mul_ln_one_plus_exp_rhs(676, 674, 675);
        }

        if s.b[614] {
            s.store_div_scaled_inputs2(725, s.ad_value(724), 1.0, s.ad_value(726), (-1.0), s.ad_value(658), 1.0);
        }

        s.b[748] = (s.v[725] > 50.0);
        s.v[748] = if s.b[748] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[748]) {
            s.store_scalar(706, 0.0);
        }

        s.b[749] = (s.v[725] < (-50.0));
        s.v[749] = if s.b[749] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[748])) && s.b[749]) {
            s.store_scalar(706, 1.0);
        }

        if ((s.b[614] && (!s.b[748])) && (!s.b[749])) {
            s.store_div_from_scalar_offset_ad(706, 1.0, A::exp(s.ad_value(725)), 1.0);
        }

        if s.b[614] {
            s.store_div_scaled_inputs3(707, s.ad_value(624), 1.0, s.ad_value(703), (-1.0), A::add_scaled_product(s.ad_value(661), 1.0, s.ad_value(658), s.ad_value(706), (-(p.p51 * 0.1))), -1.0, s.ad_value(673), 1.0);
        }

        s.b[750] = (s.v[707] > 50.0);
        s.v[750] = if s.b[750] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[750]) {
            s.store_mul(708, 674, 707);
        }

        s.b[751] = (s.v[707] < (-50.0));
        s.v[751] = if s.b[751] { 1.0 } else { 0.0 };

        if ((s.b[614] && (!s.b[750])) && s.b[751]) {
            s.store_mul_exp_rhs(708, 674, 707);
        }

        if ((s.b[614] && (!s.b[750])) && (!s.b[751])) {
            s.store_mul_ln_one_plus_exp_rhs(708, 674, 707);
        }

        if s.b[614] {
            s.store_offset_square(709, 676, 1e-38);
            s.store_offset_mul(710, 709, 676, 1e-57);
            s.store_offset_square(711, 708, 1e-38);
            s.store_offset_mul(712, 711, 708, 1e-57);
            s.store_offset_mul(713, 676, 708, 1e-38);
            s.store_div_scaled_inputs3(714, s.ad_value(709), (2.0 / 3.0), s.ad_value(711), (2.0 / 3.0), s.ad_value(713), (2.0 / 3.0), A::offset(A::add(s.ad_value(676), s.ad_value(708)), 2e-19), 1.0);
            s.store_div_ad(715, A::add_scaled_inputs_products(s.ad_value(710), (2.0 * 2.0), s.ad_value(712), (3.0 * 2.0), s.ad_value(709), s.ad_value(708), (4.0 * 2.0), s.ad_value(711), s.ad_value(676), (6.0 * 2.0)), A::add_scaled_inputs3(s.ad_value(709), 15.0, s.ad_value(711), 15.0, s.ad_value(713), (2.0 * 15.0)));
            s.store_sub(716, 714, 715);
            s.copy_ad(717, 715);
            s.store_mul_product3_rhs(617, 657, A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(634)), s.ad_value(656), s.ad_value(716), 1.0);
            s.store_mul_product3_rhs(618, 657, A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(634)), s.ad_value(656), s.ad_value(717), 1.0);
        }

        s.b[752] = (s.v[626] == 1.0);
        s.v[752] = if s.b[752] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[752]) {
            s.store_div_scaled_inputs3(718, s.ad_value(627), 1.0, s.ad_value(661), -1.0, s.ad_value(658), (-(-(p.p51 * 0.5))), s.ad_value(673), 1.0);
        }

        s.b[753] = (s.v[718] > 50.0);
        s.v[753] = if s.b[753] { 1.0 } else { 0.0 };

        if ((s.b[614] && s.b[752]) && s.b[753]) {
            s.copy_ad(721, 718);
        }

        s.b[754] = (s.v[718] < (-50.0));
        s.v[754] = if s.b[754] { 1.0 } else { 0.0 };

        if (((s.b[614] && s.b[752]) && (!s.b[753])) && s.b[754]) {
            s.store_exp(721, 718);
        }

        if (((s.b[614] && s.b[752]) && (!s.b[753])) && (!s.b[754])) {
            s.store_ln_one_plus_exp(721, 718);
        }

        if (s.b[614] && s.b[752]) {
            s.store_mul_ad_product_lhs(619, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(637), s.ad_value(673)), s.ad_value(721), 657);
            s.store_div_scaled_inputs3(719, s.ad_value(628), 1.0, s.ad_value(661), -1.0, s.ad_value(658), (-(-(p.p51 * 0.5))), s.ad_value(673), 1.0);
        }

        s.b[755] = (s.v[719] > 50.0);
        s.v[755] = if s.b[755] { 1.0 } else { 0.0 };

        if ((s.b[614] && s.b[752]) && s.b[755]) {
            s.copy_ad(721, 719);
        }

        s.b[756] = (s.v[719] < (-50.0));
        s.v[756] = if s.b[756] { 1.0 } else { 0.0 };

        if (((s.b[614] && s.b[752]) && (!s.b[755])) && s.b[756]) {
            s.store_exp(721, 719);
        }

        if (((s.b[614] && s.b[752]) && (!s.b[755])) && (!s.b[756])) {
            s.store_ln_one_plus_exp(721, 719);
        }

        if (s.b[614] && s.b[752]) {
            s.store_mul_ad_product_lhs(620, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(638), s.ad_value(673)), s.ad_value(721), 657);
        }

        if (s.b[614] && (!s.b[752])) {
            s.store_scalar(619, 0.0);
            s.store_scalar(620, 0.0);
        }

        s.b[757] = (s.v[629] == 1.0);
        s.v[757] = if s.b[757] { 1.0 } else { 0.0 };

        if (s.b[614] && s.b[757]) {
            s.store_div_scaled_inputs3(720, s.ad_value(624), 1.0, s.ad_value(661), -1.0, s.ad_value(658), (-(-(p.p51 * 0.5))), s.ad_value(673), 1.0);
        }

        s.b[758] = (s.v[720] > 50.0);
        s.v[758] = if s.b[758] { 1.0 } else { 0.0 };

        if ((s.b[614] && s.b[757]) && s.b[758]) {
            s.copy_ad(721, 720);
        }

        s.b[759] = (s.v[720] < (-50.0));
        s.v[759] = if s.b[759] { 1.0 } else { 0.0 };

        if (((s.b[614] && s.b[757]) && (!s.b[758])) && s.b[759]) {
            s.store_exp(721, 720);
        }

        if (((s.b[614] && s.b[757]) && (!s.b[758])) && (!s.b[759])) {
            s.store_ln_one_plus_exp(721, 720);
        }

        if (s.b[614] && s.b[757]) {
            s.store_mul_ad_product_lhs(621, A::mul3(A::mul3(s.ad_value(633), s.ad_value(655), s.ad_value(656)), s.ad_value(636), s.ad_value(673)), s.ad_value(721), 657);
        }

        if (s.b[614] && (!s.b[757])) {
            s.store_scalar(621, 0.0);
        }

        if s.b[614] {
            s.copy_ad(197, 617);
            s.copy_ad(198, 618);
            s.copy_ad(199, 619);
            s.copy_ad(200, 620);
            s.copy_ad(201, 621);
        }

        s.b[760] = (p.p188 == 1.0);
        s.v[760] = if s.b[760] { 1.0 } else { 0.0 };

        s.v[191] = 0.0;

        s.v[192] = 0.0;

        s.v[193] = 0.0;

        s.v[194] = 0.0;

        s.v[195] = 0.0;

        s.b[761] = (p.p167 > p.p354);
        s.v[761] = if s.b[761] { 1.0 } else { 0.0 };

        if s.b[761] {
            s.store_scalar(764, 0.0);
            s.store_scalar(765, 0.0);
            s.store_scalar(766, 0.0);
            s.store_scalar(767, 0.0);
            s.store_scalar(768, 0.0);
            s.store_scalar(769, 0.0);
            s.store_scalar(770, 0.0);
            s.copy_ad(771, 84);
            s.copy_ad(772, 85);
            s.store_scalar(773, p.p173);
            s.copy_ad(774, 86);
            s.copy_ad(775, 87);
            s.store_scalar(776, p.p171);
            s.copy_ad(777, 111);
            s.store_scalar(778, s.v[109]);
            s.copy_ad(779, 113);
            s.store_scalar(780, p.p0);
            s.store_scalar(781, p.p167);
            s.copy_ad(782, 32);
            s.store_scalar(783, p.p172);
            s.copy_ad(784, 33);
            s.copy_ad(785, 34);
        }

    }
}
