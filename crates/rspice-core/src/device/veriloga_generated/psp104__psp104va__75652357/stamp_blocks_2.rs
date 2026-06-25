#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, ReactiveScratch, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_32(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2441] != 0.0)) {
            s.store_add_ad_lhs(1874, A::sub(s.ad_value(821), A::scale(A::sub(s.ad_value(2283), A::sqrt(A::add(A::mul(s.ad_value(2283), s.ad_value(2283)), s.ad_value(742)))), 0.5)), 744);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2441] != 0.0)) {
            s.copy_ad(2284, 1874);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2441] != 0.0)) {
            s.copy_ad(2280, 739);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2441] != 0.0)) {
            s.copy_ad(2281, 742);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2441] != 0.0)) {
            s.copy_ad(2282, 740);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_sub_ad_lhs(2287, A::sub(s.ad_value(823), s.ad_value(2288)), 694);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_add_ad_rhs(2289, 2284, A::scale(A::sub(s.ad_value(820), s.ad_value(824)), 0.5));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2301, 1.0);
        }

        s.v[2442] = if (s.v[185] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale(2292, 2280, s.v[355]);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale(2293, 2289, s.v[355]);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale(2294, 2287, s.v[355]);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_offset_ad(1920, A::div(A::scale(s.ad_value(2282), 0.5), A::sqrt(s.ad_value(2292))), 1.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_add_ad_rhs(1921, 2292, A::mul(s.ad_value(2282), A::sqrt(s.ad_value(2292))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_sub_ad(2295, A::add(A::div(A::sub(s.ad_value(2294), s.ad_value(1921)), s.ad_value(1920)), A::scale(s.ad_value(2292), 0.5)), A::mul(A::offset(s.ad_value(186), 1.0), s.ad_value(2293)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_offset_scaled(2296, 2292, 0.5, 2.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_add(2297, 2292, 2293);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_sub_ad(1920, A::sub(A::sub(s.ad_value(2294), s.ad_value(2297)), A::mul(s.ad_value(2282), A::sqrt(s.ad_value(2297)))), A::scale(A::ln(A::add(A::div(s.ad_value(2292), s.ad_value(2282)), A::sqrt(s.ad_value(2292)))), 2.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_add_ad_lhs(2298, A::scale(s.ad_value(1920), 2.0), 2296);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale_ad(1920, A::add(A::add(s.ad_value(2295), s.ad_value(2298)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2295), s.ad_value(2298)), A::sub(s.ad_value(2295), s.ad_value(2298))), 20.0))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_sub_ad_lhs(1921, A::scale(A::sub(s.ad_value(2294), s.ad_value(2293)), 2.0), 2296);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale_ad(2299, A::sub(A::add(s.ad_value(1920), s.ad_value(1921)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1920), s.ad_value(1921)), A::sub(s.ad_value(1920), s.ad_value(1921))), 20.0))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale_ad(1920, A::sub(A::add(s.ad_value(2299), s.ad_value(2296)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2299), s.ad_value(2296)), A::sub(s.ad_value(2299), s.ad_value(2296))), 5.0))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_scale_ad(2300, A::add(A::sub(s.ad_value(1920), s.ad_value(2296)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1920), A::neg(s.ad_value(2296))), A::sub(s.ad_value(1920), A::neg(s.ad_value(2296)))), 20.0))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) {
            s.store_mul_ad_rhs(1921, 696, A::offset(A::div(s.ad_value(2300), s.ad_value(2296)), 1.0));
        }

        s.v[2443] = if (s.v[1921] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) && (s.v[2443] != 0.0)) {
            s.store_exp(2301, 1921);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2442] != 0.0)) && (!(s.v[2443] != 0.0))) {
            s.store_div_from_scalar_ad(2301, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1921)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1921)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1921)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_offset_ad(2302, A::mul(s.ad_value(695), s.ad_value(2301)), 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scale(2303, 2302, s.v[709]);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul_ad(2304, A::mul(s.ad_value(194), A::offset(A::mul(s.ad_value(196), s.ad_value(824)), 1.0)), A::offset(A::mul(s.ad_value(195), s.ad_value(2289)), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul_ad_rhs(2305, 2303, A::offset(s.ad_value(2304), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_div_from_scalar(2306, 1.0, 2305);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul_ad_rhs(2290, 2282, A::sqrt(A::scale(s.ad_value(2306), s.v[709])));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_square(2291, 2290);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_div_from_scalar(2307, 1.0, 2291);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul(2308, 2284, 2306);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul(2309, 2287, 2306);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_div_ad(2310, A::scale(s.ad_value(824), 2.0), A::offset(A::sqrt(A::offset(A::mul(s.ad_value(192), s.ad_value(824)), 1.0)), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul_ad(2311, A::mul(s.ad_value(191), s.ad_value(2310)), A::offset(A::mul(s.ad_value(193), s.ad_value(2289)), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul(2312, 2280, 2306);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_sqrt_ad(1920, A::add(A::square(s.ad_value(2283)), s.ad_value(2281)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_sqrt_ad(1921, A::add(A::mul(A::sub(s.ad_value(2283), s.ad_value(2311)), A::sub(s.ad_value(2283), s.ad_value(2311))), s.ad_value(2281)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul_ad(2313, A::scale(s.ad_value(2306), 0.5), A::sub(A::add(s.ad_value(2311), s.ad_value(1920)), s.ad_value(1921)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_add(2314, 2312, 2308);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_sub(2315, 2314, 2313);
        }

        s.v[2444] = if (p.p45 > 0.0) { 1.0 } else { 0.0 };

        s.v[2445] = if (((s.v[2315]) as f64).abs() < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2444] != 0.0)) && (s.v[2445] != 0.0)) {
            s.store_offset_ad(2316, A::mul(s.ad_value(2290), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2315), 0.5), A::sub_from_scalar(1.0, A::scale(s.ad_value(2315), 0.3125))))), 1.0);
        }

        s.v[2446] = if (s.v[2315] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2444] != 0.0)) && (!(s.v[2445] != 0.0))) && (s.v[2446] != 0.0)) {
            s.store_exp_ad(2330, A::neg(s.ad_value(2315)));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2444] != 0.0)) && (!(s.v[2445] != 0.0))) && (!(s.v[2446] != 0.0))) {
            s.store_div_from_scalar_ad(2330, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2315), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2315), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2315), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2444] != 0.0)) && (!(s.v[2445] != 0.0))) {
            s.store_scalar(1919, (if (s.v[2315] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2444] != 0.0)) && (!(s.v[2445] != 0.0))) {
            s.store_offset_ad(2316, A::div(A::mul(A::mul(s.ad_value(1919), s.ad_value(2290)), A::sub_from_scalar(1.0, A::mul(s.ad_value(2330), A::sub_from_scalar(1.0, s.ad_value(2315))))), A::scale(A::sqrt(A::mul(s.ad_value(2315), A::sub_from_scalar(1.0, s.ad_value(2330)))), 2.0)), 1.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2444] != 0.0))) {
            s.store_offset_ad(2316, A::div(A::scale(s.ad_value(2290), 0.5), A::sqrt(s.ad_value(2315))), 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_sub_ad(2317, A::add(s.ad_value(2315), A::mul(s.ad_value(2290), A::sqrt(s.ad_value(2315)))), A::mul(s.ad_value(2316), A::ln(A::offset(s.ad_value(2316), (-1.0)))));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_div_ad_lhs(2318, A::sub(s.ad_value(2309), s.ad_value(2317)), 2316);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul_ad(2324, A::scale(s.ad_value(2291), 0.5), A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2291)), 1.0)), (-1.0)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2323, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2325, 1.0);
        }

        s.v[2447] = if (s.v[2318] > (-30.0)) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_offset_ad(2319, A::mul(s.ad_value(2316), s.ad_value(2318)), (-1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_scale_ad(1919, A::add(s.ad_value(2319), A::sqrt(A::offset(A::square(s.ad_value(2319)), 10.0))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_sub_ad_rhs(2320, 2318, A::ln(s.ad_value(1919)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_scale_ad(2321, A::add(s.ad_value(2320), A::sqrt(A::offset(A::square(s.ad_value(2320)), 2.0))), 0.5);
        }

        s.v[2448] = if ((s.v[2318] - s.v[2321]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) && (s.v[2448] != 0.0)) {
            s.store_exp_ad(1919, A::sub(s.ad_value(2318), s.ad_value(2321)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) && (!(s.v[2448] != 0.0))) {
            s.store_scale_ad(1919, A::offset(A::mul(A::offset(A::sub(s.ad_value(2318), s.ad_value(2321)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2318), s.ad_value(2321)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2318), s.ad_value(2321)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_div(2322, 1919, 2316);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_sub_ad_lhs(1919, A::scale(A::offset(s.ad_value(2321), 1.0), 2.0), 2322);
        }

        s.v[2449] = if (s.v[2322] > 1e-6) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) && (s.v[2449] != 0.0)) {
            s.store_mul_ad_rhs(2323, 2316, A::offset(A::sub(s.ad_value(2321), A::div(A::offset(A::sqrt(A::offset(A::mul(s.ad_value(2322), s.ad_value(1919)), 1.0)), (-1.0)), s.ad_value(2322))), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) && (!(s.v[2449] != 0.0))) {
            s.store_mul_ad(2323, A::mul(A::scale(s.ad_value(2316), 0.5), s.ad_value(2322)), A::offset(A::mul(A::scale(s.ad_value(1919), 0.25), s.ad_value(1919)), 1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_scale_ad(1919, A::add(A::offset(A::sub(s.ad_value(2309), s.ad_value(2323)), 2.0), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(2309), s.ad_value(2323)), (-2.0)), A::offset(A::sub(s.ad_value(2309), s.ad_value(2323)), (-2.0))), 1.0))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_mul_ad(2324, A::scale(s.ad_value(2291), 0.5), A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2291)), s.ad_value(1919)), 1.0)), (-1.0)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_div_ad_rhs(2325, 2324, A::add(s.ad_value(2324), s.ad_value(2323)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2447] != 0.0)) {
            s.store_sub_ad_rhs(2315, 2314, A::mul(s.ad_value(2325), s.ad_value(2313)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_offset_scaled(2326, 2290, 0.7071067811865475, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scale(2327, 2326, 1e-5);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_div_from_scalar(2328, 1.0, 2326);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2435, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2329, 0.0);
        }

        s.v[2450] = if (s.v[2315] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2450] != 0.0)) {
            s.store_exp_ad(2330, A::neg(s.ad_value(2315)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2450] != 0.0))) {
            s.store_div_from_scalar_ad(2330, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2315), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2315), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2315), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2451] = if (((s.v[2309]) as f64).abs() <= s.v[2327]) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2451] != 0.0)) {
            s.store_scale_ad(2415, A::square(s.ad_value(2328)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2451] != 0.0)) {
            s.store_mul_ad(2329, A::mul(s.ad_value(2309), s.ad_value(2328)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2309), A::sub_from_scalar(1.0, s.ad_value(2330))), s.ad_value(2290)), s.ad_value(2415)), 1.0));
        }

        s.v[2452] = if (s.v[2309] < (-s.v[2327])) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_neg(2417, 2309);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_scaled_mul(2418, 2417, 2328, 1.25);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_scale_ad(2419, A::sub(A::offset(s.ad_value(2418), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2418), (-6.0)), A::offset(s.ad_value(2418), (-6.0))), 64.0))), 0.5);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub(2414, 2417, 2419);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_add_ad(2420, A::square(s.ad_value(2414)), A::mul(s.ad_value(2291), A::offset(s.ad_value(2419), 1.0)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub_ad_lhs(2421, A::scale(s.ad_value(2414), 2.0), 2291);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub_ad_lhs(2422, A::ln(A::mul(s.ad_value(2420), s.ad_value(2307))), 2419);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_add(818, 2420, 2421);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_add_ad(817, A::square(s.ad_value(818)), A::mul(s.ad_value(2422), A::sub(A::scale(A::square(s.ad_value(2421)), 0.5), s.ad_value(2420))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_add_ad_rhs(2423, 2419, A::div(A::mul(A::mul(s.ad_value(2420), s.ad_value(818)), s.ad_value(2422)), A::add(s.ad_value(817), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422)), s.ad_value(2422)), s.ad_value(2421)), A::sub(A::scale(A::square(s.ad_value(2421)), 0.3333333333333333), s.ad_value(2420))))));
        }

        s.v[2453] = if (s.v[2423] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) && (s.v[2453] != 0.0)) {
            s.store_exp(2424, 2423);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) && (!(s.v[2453] != 0.0))) {
            s.store_scale_ad(2424, A::offset(A::mul(A::offset(s.ad_value(2423), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2423), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2423), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_div_from_scalar(2425, 1.0, 2424);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_div_from_scalar_ad(2414, 1.0, A::offset(A::square(s.ad_value(2423)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_mul_ad_lhs(2426, A::square(s.ad_value(2423)), 2414);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_scale_ad(2427, A::mul(A::mul(s.ad_value(2423), s.ad_value(2414)), s.ad_value(2414)), 4.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_mul_ad_lhs(2428, A::mul(A::sub(A::scale(s.ad_value(2414), 8.0), A::scale(s.ad_value(2426), 12.0)), s.ad_value(2414)), 2414);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub(2414, 2417, 2423);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_mul(2415, 2330, 2425);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_add_ad(2429, A::scale(s.ad_value(2414), 2.0), A::mul(s.ad_value(2291), A::add(A::sub(A::offset(s.ad_value(2424), (-1.0)), s.ad_value(2415)), A::mul(s.ad_value(2330), A::sub_from_scalar(1.0, s.ad_value(2427))))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub_ad(2430, A::square(s.ad_value(2414)), A::mul(s.ad_value(2291), A::add(A::add(A::offset(A::sub(s.ad_value(2424), s.ad_value(2423)), (-1.0)), s.ad_value(2415)), A::mul(s.ad_value(2330), A::sub(A::offset(s.ad_value(2423), (-1.0)), s.ad_value(2426))))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub_from_scalar_ad(2414, 2.0, A::mul(s.ad_value(2291), A::sub(A::add(s.ad_value(2424), s.ad_value(2415)), A::mul(s.ad_value(2330), s.ad_value(2428)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub_ad(2414, A::square(s.ad_value(2429)), A::scale(A::mul(s.ad_value(2430), s.ad_value(2414)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (s.v[2452] != 0.0)) {
            s.store_sub_ad(2329, A::neg(s.ad_value(2423)), A::scale(A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_div_from_scalar_ad(2431, 1.0, A::offset(A::scale(s.ad_value(2290), 0.7324648775608221), 1.25));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_mul_ad_lhs(2432, A::offset(A::mul(A::scale(s.ad_value(2326), 1.25), s.ad_value(2431)), (-1.0)), 2431);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_mul_ad(2433, A::mul(s.ad_value(2309), s.ad_value(2328)), A::offset(A::mul(s.ad_value(2432), s.ad_value(2309)), 1.0));
        }

        s.v[2454] = if ((-s.v[2433]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (s.v[2454] != 0.0)) {
            s.store_exp_ad(2414, A::neg(s.ad_value(2433)));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (!(s.v[2454] != 0.0))) {
            s.store_div_from_scalar_ad(2414, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2433))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2433))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2433))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_from_scalar(2434, 1.0, 2414);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_ad(2435, A::add(s.ad_value(2309), A::scale(s.ad_value(2291), 0.5)), A::mul(s.ad_value(2290), A::sqrt(A::sub(A::add(s.ad_value(2309), A::scale(s.ad_value(2291), 0.25)), s.ad_value(2434)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_offset(2436, 2315, 3.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_ad(2419, A::scale(A::sub(A::add(s.ad_value(2435), s.ad_value(2436)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2435), s.ad_value(2436)), A::sub(s.ad_value(2435), s.ad_value(2436))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2436), A::sqrt(A::offset(A::square(s.ad_value(2436)), 5.0))), 0.5));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub(2414, 2309, 2419);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_exp_ad(2415, A::neg(s.ad_value(2419)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_div_from_scalar_ad(2416, 1.0, A::offset(A::square(s.ad_value(2419)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_mul_ad_lhs(2426, A::square(s.ad_value(2419)), 2416);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_scale_ad(2427, A::mul(A::mul(s.ad_value(2419), s.ad_value(2416)), s.ad_value(2416)), 4.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_mul_ad_lhs(2428, A::mul(A::sub(A::scale(s.ad_value(2416), 8.0), A::scale(s.ad_value(2426), 12.0)), s.ad_value(2416)), 2416);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            let assign49580_ad_e63936: A = {
                if (1e-40 > ((s.v[2414] * s.v[2414]) - (s.v[2291] * (((s.v[2415] + s.v[2419]) - 1.0) - (s.v[2330] * ((s.v[2419] + 1.0) + s.v[2426])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2414)), A::mul(s.ad_value(2291), A::sub(A::offset(A::add(s.ad_value(2415), s.ad_value(2419)), (-1.0)), A::mul(s.ad_value(2330), A::add(A::offset(s.ad_value(2419), 1.0), s.ad_value(2426))))))
                }
            };
            s.store_ad(2420, &assign49580_ad_e63936);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_from_scalar_ad(2437, 1.0, A::scale(A::mul(s.ad_value(2291), A::sub(s.ad_value(2415), A::mul(s.ad_value(2330), s.ad_value(2428)))), 0.5));
        }

    }

    pub(super) fn stamp_transient_block_33(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_add_ad(2421, A::scale(s.ad_value(2414), 2.0), A::mul(s.ad_value(2291), A::sub(A::sub_from_scalar(1.0, s.ad_value(2415)), A::mul(s.ad_value(2330), A::offset(s.ad_value(2427), 1.0)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_add_ad(2422, A::sub(s.ad_value(2315), s.ad_value(2419)), A::ln(A::div(s.ad_value(2420), s.ad_value(2291))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_add(818, 2420, 2421);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_add_ad(817, A::square(s.ad_value(818)), A::mul(s.ad_value(2422), A::sub(A::scale(A::square(s.ad_value(2421)), 0.5), A::mul(s.ad_value(2420), s.ad_value(2437)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            let assign49640_ad_e64083: A = A::add(s.ad_value(2419), A::div(A::mul(A::mul(s.ad_value(2420), s.ad_value(818)), s.ad_value(2422)), A::add(s.ad_value(817), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422)), s.ad_value(2422)), s.ad_value(2421)), A::sub(A::scale(A::square(s.ad_value(2421)), 0.3333333333333333), A::mul(s.ad_value(2420), s.ad_value(2437)))))));
            s.store_ad(2438, &assign49640_ad_e64083);
        }

        s.v[2455] = if (s.v[2438] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (s.v[2455] != 0.0)) {
            s.store_exp(2424, 2438);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (s.v[2455] != 0.0)) {
            s.store_div_from_scalar(2425, 1.0, 2424);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (s.v[2455] != 0.0)) {
            s.store_mul(2424, 2330, 2424);
        }

        s.v[2456] = if (s.v[2438] > (s.v[2315] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (!(s.v[2455] != 0.0))) && (s.v[2456] != 0.0)) {
            s.store_exp_ad(2424, A::sub(s.ad_value(2438), s.ad_value(2315)));
        }

        if ((((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (!(s.v[2455] != 0.0))) && (s.v[2456] != 0.0)) {
            s.store_div(2425, 2330, 2424);
        }

        if ((((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (!(s.v[2455] != 0.0))) && (!(s.v[2456] != 0.0))) {
            s.store_div_from_scalar_ad(2424, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2315), s.ad_value(2438)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2315), s.ad_value(2438)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2315), s.ad_value(2438)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) && (!(s.v[2455] != 0.0))) && (!(s.v[2456] != 0.0))) {
            s.store_div_from_scalar_ad(2425, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2438), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2438), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2438), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_div_from_scalar_ad(2414, 1.0, A::offset(A::square(s.ad_value(2438)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_mul_ad_lhs(2426, A::square(s.ad_value(2438)), 2414);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_scale_ad(2427, A::mul(A::mul(s.ad_value(2438), s.ad_value(2414)), s.ad_value(2414)), 4.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_mul_ad_lhs(2428, A::mul(A::sub(A::scale(s.ad_value(2414), 8.0), A::scale(s.ad_value(2426), 12.0)), s.ad_value(2414)), 2414);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub(2414, 2309, 2438);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_add_ad(2429, A::scale(s.ad_value(2414), 2.0), A::mul(s.ad_value(2291), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2425)), s.ad_value(2424)), A::mul(s.ad_value(2330), A::offset(s.ad_value(2427), 1.0)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_ad(2430, A::square(s.ad_value(2414)), A::mul(s.ad_value(2291), A::sub(A::add(A::offset(A::add(s.ad_value(2425), s.ad_value(2438)), (-1.0)), s.ad_value(2424)), A::mul(s.ad_value(2330), A::add(A::offset(s.ad_value(2438), 1.0), s.ad_value(2426))))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_from_scalar_ad(2414, 2.0, A::mul(s.ad_value(2291), A::sub(A::add(s.ad_value(2425), s.ad_value(2424)), A::mul(s.ad_value(2330), s.ad_value(2428)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_sub_ad(2414, A::square(s.ad_value(2429)), A::scale(A::mul(s.ad_value(2430), s.ad_value(2414)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (!(s.v[2451] != 0.0))) && (!(s.v[2452] != 0.0))) {
            s.store_add_ad_rhs(2329, 2438, A::scale(A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2332, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2333, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2334, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2335, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2336, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2337, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2338, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2339, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2340, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_sub(2341, 2309, 2329);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2342, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_mul(2343, 2305, 2341);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2344, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2345, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2349, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2350, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) {
            s.store_scalar(2352, 1.0);
        }

        s.v[2457] = if (s.v[2309] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_div_from_scalar_ad(1919, 1.0, A::offset(A::square(s.ad_value(2329)), 2.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_mul_ad_lhs(2331, A::square(s.ad_value(2329)), 1919);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_scale_ad(2332, A::mul(A::mul(s.ad_value(2329), s.ad_value(1919)), s.ad_value(1919)), 4.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_mul_ad_lhs(2333, A::mul(A::sub(A::scale(s.ad_value(1919), 8.0), A::scale(s.ad_value(2331), 12.0)), s.ad_value(1919)), 1919);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_scalar(2334, 0.0);
        }

        s.v[2458] = if (s.v[2329] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2458] != 0.0)) {
            s.store_exp(2334, 2329);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2458] != 0.0)) {
            s.store_div_from_scalar(2335, 1.0, 2334);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2458] != 0.0)) {
            s.store_mul(2334, 2330, 2334);
        }

        s.v[2459] = if (s.v[2329] > (s.v[2315] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2458] != 0.0))) && (s.v[2459] != 0.0)) {
            s.store_exp_ad(2334, A::sub(s.ad_value(2329), s.ad_value(2315)));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2458] != 0.0))) && (s.v[2459] != 0.0)) {
            s.store_div(2335, 2330, 2334);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2458] != 0.0))) && (!(s.v[2459] != 0.0))) {
            s.store_div_from_scalar_ad(2334, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2315), s.ad_value(2329)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2315), s.ad_value(2329)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2315), s.ad_value(2329)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2458] != 0.0))) && (!(s.v[2459] != 0.0))) {
            s.store_div_from_scalar_ad(2335, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2329), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2329), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2329), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_sub_ad_rhs(2336, 2334, A::mul(s.ad_value(2330), A::add(A::offset(s.ad_value(2329), 1.0), s.ad_value(2331))));
        }

        s.v[2460] = if (s.v[2329] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2460] != 0.0)) {
            s.store_scale_ad(2337, A::mul(A::square(s.ad_value(2329)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2329), A::sub_from_scalar(1.0, A::scale(s.ad_value(2329), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2460] != 0.0)) {
            s.store_scale_ad(2336, A::mul(A::mul(A::mul(A::mul(s.ad_value(2330), s.ad_value(2329)), s.ad_value(2329)), s.ad_value(2329)), A::offset(A::scale(s.ad_value(2329), 1.75), 1.0)), 0.16666666666666666);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2460] != 0.0)) {
            s.store_sqrt_ad(1919, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2329), A::sub_from_scalar(1.0, A::scale(s.ad_value(2329), 0.25))), 0.3333333333333333)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2460] != 0.0)) {
            s.store_scaled_mul(2338, 2329, 1919, 0.7071067811865475);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2460] != 0.0)) {
            s.store_offset_ad(2339, A::scale(A::div(A::mul(s.ad_value(2290), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2329), 0.5)), A::scale(A::square(s.ad_value(2329)), 0.16666666666666666))), s.ad_value(1919)), 0.7071067811865475), 1.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2460] != 0.0))) {
            s.store_add_ad_lhs(2337, A::offset(s.ad_value(2329), (-1.0)), 2335);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2460] != 0.0))) {
            s.store_sqrt(2338, 2337);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (!(s.v[2460] != 0.0))) {
            s.store_offset_ad(2339, A::scale(A::div(A::mul(s.ad_value(2290), A::sub_from_scalar(1.0, s.ad_value(2335))), s.ad_value(2338)), 0.5), 1.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_div_ad(2340, A::offset(A::mul(A::scale(s.ad_value(702), 0.2), s.ad_value(2289)), 1.0), A::offset(A::mul(s.ad_value(702), s.ad_value(2289)), 1.0));
        }

        s.v[2461] = if (s.v[2336] > 1e-100) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul_ad_rhs(2341, 2290, A::sqrt(A::add(s.ad_value(2337), s.ad_value(2336))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_div_ad(2342, A::mul(A::mul(s.ad_value(2291), s.ad_value(2336)), s.ad_value(2305)), A::add(s.ad_value(2341), A::mul(s.ad_value(2290), s.ad_value(2338))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul_ad_lhs(2343, A::mul(s.ad_value(2338), s.ad_value(2290)), 2305);
        }

        s.v[2462] = if (s.v[212] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (s.v[2462] != 0.0)) {
            s.store_div_from_scalar_ad(2344, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(212), s.ad_value(2289))));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (!(s.v[2462] != 0.0))) {
            s.store_offset_ad(2344, A::mul(s.ad_value(212), s.ad_value(2289)), 1.0);
        }

        s.v[2463] = if (s.v[213] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (s.v[2463] != 0.0)) {
            s.store_sub_from_scalar_ad(2345, 1.0, A::mul(s.ad_value(213), s.ad_value(2342)));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (!(s.v[2463] != 0.0))) {
            s.store_div_from_scalar_ad(2345, 1.0, A::offset(A::mul(s.ad_value(213), s.ad_value(2342)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul_ad_lhs(2346, A::mul(A::mul(s.ad_value(751), s.ad_value(2344)), s.ad_value(2345)), 2342);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul_ad_rhs(2347, 768, A::add(s.ad_value(2343), A::mul(s.ad_value(769), s.ad_value(2342))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_ln_ad(1920, A::div(s.ad_value(2337), A::offset(A::add(s.ad_value(2337), s.ad_value(2336)), 1e-14)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_add_ad(2348, A::pow(A::mul(s.ad_value(2347), s.ad_value(698)), s.ad_value(699)), A::mul(s.ad_value(700), A::exp(A::mul(A::scale(s.ad_value(701), 0.5), s.ad_value(1920)))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul_ad_lhs(2349, A::add(A::offset(s.ad_value(2348), 1.0), s.ad_value(2346)), 2340);
        }

        s.v[2464] = if (s.v[216] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (s.v[2464] != 0.0)) {
            s.store_div_from_scalar_ad(2350, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(216), s.ad_value(2289))));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (!(s.v[2464] != 0.0))) {
            s.store_offset_ad(2350, A::mul(s.ad_value(216), s.ad_value(2289)), 1.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul(1921, 2342, 2350);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_div_ad_rhs(2351, 1921, A::add(s.ad_value(218), s.ad_value(1921)));
        }

        s.v[2465] = if (s.v[217] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (s.v[2465] != 0.0)) {
            s.store_div_from_scalar_ad(2352, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(217), s.ad_value(2351))));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2440] != 0.0)) && (s.v[2457] != 0.0)) && (s.v[2461] != 0.0)) && (!(s.v[2465] != 0.0))) {
            s.store_offset_ad(2352, A::mul(s.ad_value(217), s.ad_value(2351)), 1.0);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2287, 1810);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2289, 1811);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2305, 1812);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2306, 1813);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2290, 1814);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2291, 1815);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2307, 1816);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2309, 1817);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2314, 1818);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2315, 1819);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2326, 1820);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2327, 1821);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2328, 1822);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2435, 1823);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2330, 1824);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2329, 1825);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2332, 1826);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2333, 1827);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2334, 1828);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2335, 1829);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2337, 1830);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2336, 1831);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2338, 1832);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2339, 1833);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2340, 1834);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2341, 1835);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2342, 1836);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2343, 1837);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2344, 1838);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2345, 1839);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2349, 1840);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2350, 1841);
        }

        if ((s.v[2439] != 0.0) && (!(s.v[2440] != 0.0))) {
            s.copy_ad(2352, 1842);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2285, 714);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2286, 771);
        }

        s.v[2466] = if (p.p48 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2439] != 0.0) && (s.v[2466] != 0.0)) {
            s.copy_ad(2285, 715);
        }

        if ((s.v[2439] != 0.0) && (s.v[2466] != 0.0)) {
            s.copy_ad(2286, 772);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2354, 0.0);
        }

        if (s.v[2439] != 0.0) {
            s.store_scale(2353, 2305, 4.60517018598809);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2370, 2353);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2371, 820);
        }

        if (s.v[2439] != 0.0) {
            s.store_mul(2372, 820, 2306);
        }

    }

    pub(super) fn stamp_transient_block_34(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[2439] != 0.0) {
            s.copy_ad(2376, 2329);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2377, 0.0);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2380, 0.0);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2382, 2335);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2383, 2337);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2385, 2336);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2386, 2343);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2387, 2329);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2388, 2335);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2390, 2336);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2391, 2337);
        }

        if (s.v[2439] != 0.0) {
            s.store_sub(2392, 2309, 2329);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2393, 1.0);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2395, 1.0);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2394, 0.0);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2404, 2342);
        }

        if (s.v[2439] != 0.0) {
            s.store_mul(2408, 2392, 2305);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2405, 0.0);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2406, 2343);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2411, 0.0);
        }

        if (s.v[2439] != 0.0) {
            s.store_scalar(2410, 1.0);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2413, 2285);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(2412, 2408);
        }

        s.v[2467] = if (s.v[2309] > 0.0) { 1.0 } else { 0.0 };

        s.v[2468] = if (s.v[2336] > 1e-100) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_mul(2413, 2285, 2352);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_div(2354, 2413, 2349);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_add_ad_rhs(2355, 2341, A::scale(s.ad_value(2291), 0.5));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_div_ad_lhs(1919, A::div(A::mul(s.ad_value(2291), s.ad_value(2334)), s.ad_value(2355)), 2355);
        }

        s.v[2469] = if (s.v[1919] > 0.0001) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2469] != 0.0)) {
            s.store_sub_from_scalar(1920, 1.0, 1919);
        }

        s.v[2470] = if (s.v[1920] < 1e-10) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2469] != 0.0)) && (s.v[2470] != 0.0)) {
            s.store_scalar(1921, 1.0);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2469] != 0.0)) && (!(s.v[2470] != 0.0))) {
            s.store_sub_from_scalar_ad(1921, 1.0, A::sqrt(s.ad_value(1920)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (!(s.v[2469] != 0.0))) {
            s.store_scale(1921, 1919, 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_mul(2356, 1921, 2355);
        }

        s.v[2471] = if ((s.v[700] > 0.0) && (s.v[701] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_lhs(2357, A::scale(s.ad_value(2305), 0.475), 2356);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_sub_ad_rhs(1919, 2342, A::mul(s.ad_value(2339), s.ad_value(2357)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_scale_ad(2358, A::add(s.ad_value(1919), A::sqrt(A::offset(A::square(s.ad_value(1919)), 1e-12))), 0.5);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_add_ad(2359, A::sub(A::mul(s.ad_value(2305), s.ad_value(2341)), s.ad_value(2342)), A::mul(A::offset(s.ad_value(2339), (-1.0)), s.ad_value(2357)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_offset_ad(2360, A::div(A::mul(A::scale(s.ad_value(2291), 0.5), s.ad_value(2305)), s.ad_value(2359)), 1.0);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_add_ad_rhs(1919, 2359, A::mul(s.ad_value(769), s.ad_value(2358)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_ad(2361, &A::pow(A::mul(A::mul(s.ad_value(768), s.ad_value(1919)), s.ad_value(698)), s.ad_value(699)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_lhs(1920, A::div(A::mul(s.ad_value(699), A::offset(A::mul(s.ad_value(2360), A::sub_from_scalar(1.0, s.ad_value(769))), (-1.0))), s.ad_value(1919)), 2361);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_div(1919, 2358, 2359);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_rhs(2362, 700, A::pow(A::offset(s.ad_value(1919), 1.0), A::neg(s.ad_value(701))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_lhs(1921, A::div(A::mul(s.ad_value(701), A::add(A::offset(s.ad_value(2360), (-1.0)), A::div_from_scalar(1.0, A::offset(s.ad_value(1919), 1.0)))), s.ad_value(2359)), 2362);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_lhs(2363, A::mul(A::mul(s.ad_value(751), s.ad_value(2344)), s.ad_value(2345)), 2358);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_offset_ad(1919, A::div(A::sub(s.ad_value(1920), A::mul(A::mul(A::mul(s.ad_value(751), s.ad_value(2344)), s.ad_value(2345)), s.ad_value(2360))), s.ad_value(1921)), 1.0);
        }

        s.v[2472] = if (s.v[1919] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2472] != 0.0)) {
            s.store_scale_ad(1920, A::ln(A::offset(A::exp(A::scale(s.ad_value(1919), 2.0)), 1.0)), 0.5);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2472] != 0.0))) {
            s.copy_ad(1920, 1919);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_div_ad(2364, A::mul(A::mul(A::neg(s.ad_value(2357)), s.ad_value(1921)), s.ad_value(1920)), A::add(A::add(A::offset(s.ad_value(2361), 1.0), s.ad_value(2362)), s.ad_value(2363)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_rhs(2365, 2356, A::offset(A::div(s.ad_value(2364), A::offset(A::sqrt(A::offset(A::square(s.ad_value(2364)), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (!(s.v[2471] != 0.0))) {
            s.copy_ad(2365, 2356);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_scale_ad(2366, A::mul(A::mul(s.ad_value(2305), s.ad_value(2354)), s.ad_value(2365)), 0.7071067811865475);
        }

        s.v[2473] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) && (s.v[2473] != 0.0)) {
            s.store_div_ad_rhs(2366, 2366, A::sqrt(A::offset(s.ad_value(2366), 1.0)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_div_from_scalar_ad(2367, 2.0, A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2366), 4.0), 1.0)), 1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_mul(1919, 2367, 2366);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_mul_ad(2368, A::mul(s.ad_value(2365), s.ad_value(2367)), A::offset(A::div(A::mul(A::scale(s.ad_value(1919), 0.86), A::sub_from_scalar(1.0, A::mul(s.ad_value(1919), s.ad_value(2367)))), A::offset(A::mul(A::mul(A::scale(s.ad_value(1919), 4.0), s.ad_value(1919)), s.ad_value(2367)), 1.0)), 1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_scale(2369, 2368, 0.99);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_div_ad_lhs(1919, A::mul(A::mul(s.ad_value(2369), A::sub(s.ad_value(2369), A::scale(s.ad_value(2355), 2.0))), s.ad_value(2307)), 2336);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2468] != 0.0)) {
            s.store_mul_ad_rhs(2370, 2305, A::sub(s.ad_value(2369), A::ln(A::offset({
                if (s.v[1919] > (-0.99)) {
                    s.ad_value(1919)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2468] != 0.0))) {
            s.copy_ad(2370, 2353);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_offset(1919, 2286, 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_div_ad_lhs(1920, A::mul(A::sqrt(s.ad_value(1919)), s.ad_value(820)), 2370);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add_ad_lhs(1921, A::square(s.ad_value(1920)), 1919);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_scale(1919, 1920, 2.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_div_ad(2371, A::mul(s.ad_value(2370), s.ad_value(1919)), A::add(A::sqrt(A::sub(s.ad_value(1921), s.ad_value(1919))), A::sqrt(A::add(s.ad_value(1921), s.ad_value(1919)))));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(2372, 2371, 2306);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add(2373, 2315, 2372);
        }

        s.v[2474] = if (s.v[2372] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2474] != 0.0)) {
            s.store_exp_ad(2374, A::neg(s.ad_value(2372)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2474] != 0.0))) {
            s.store_div_from_scalar_ad(2374, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2372), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2372), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2372), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(2375, 2330, 2374);
        }

        s.v[2475] = if (((s.v[2309]) as f64).abs() <= s.v[2327]) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_scale_ad(2415, A::square(s.ad_value(2328)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul_ad(2376, A::mul(s.ad_value(2309), s.ad_value(2328)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2309), A::sub_from_scalar(1.0, s.ad_value(2375))), s.ad_value(2290)), s.ad_value(2415)), 1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_offset(2436, 2373, 3.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub_ad(2419, A::scale(A::sub(A::add(s.ad_value(2435), s.ad_value(2436)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2435), s.ad_value(2436)), A::sub(s.ad_value(2435), s.ad_value(2436))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2436), A::sqrt(A::offset(A::square(s.ad_value(2436)), 5.0))), 0.5));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub(2414, 2309, 2419);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_exp_ad(2415, A::neg(s.ad_value(2419)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_div_from_scalar_ad(2416, 1.0, A::offset(A::square(s.ad_value(2419)), 2.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_mul_ad_lhs(2426, A::square(s.ad_value(2419)), 2416);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_scale_ad(2427, A::mul(A::mul(s.ad_value(2419), s.ad_value(2416)), s.ad_value(2416)), 4.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_mul_ad_lhs(2428, A::mul(A::sub(A::scale(s.ad_value(2416), 8.0), A::scale(s.ad_value(2426), 12.0)), s.ad_value(2416)), 2416);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            let assign51910_ad_e66735: A = {
                if (1e-40 > ((s.v[2414] * s.v[2414]) - (s.v[2291] * (((s.v[2415] + s.v[2419]) - 1.0) - (s.v[2375] * ((s.v[2419] + 1.0) + s.v[2426])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2414)), A::mul(s.ad_value(2291), A::sub(A::offset(A::add(s.ad_value(2415), s.ad_value(2419)), (-1.0)), A::mul(s.ad_value(2375), A::add(A::offset(s.ad_value(2419), 1.0), s.ad_value(2426))))))
                }
            };
            s.store_ad(2420, &assign51910_ad_e66735);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub_from_scalar_ad(2437, 1.0, A::scale(A::mul(s.ad_value(2291), A::sub(s.ad_value(2415), A::mul(s.ad_value(2375), s.ad_value(2428)))), 0.5));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_add_ad(2421, A::scale(s.ad_value(2414), 2.0), A::mul(s.ad_value(2291), A::sub(A::sub_from_scalar(1.0, s.ad_value(2415)), A::mul(s.ad_value(2375), A::offset(s.ad_value(2427), 1.0)))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_add_ad(2422, A::sub(s.ad_value(2373), s.ad_value(2419)), A::ln(A::div(s.ad_value(2420), s.ad_value(2291))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_add(818, 2420, 2421);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_add_ad(817, A::square(s.ad_value(818)), A::mul(s.ad_value(2422), A::sub(A::scale(A::square(s.ad_value(2421)), 0.5), A::mul(s.ad_value(2420), s.ad_value(2437)))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            let assign51970_ad_e66864: A = A::add(s.ad_value(2419), A::div(A::mul(A::mul(s.ad_value(2420), s.ad_value(818)), s.ad_value(2422)), A::add(s.ad_value(817), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(818), s.ad_value(817)), s.ad_value(2422)), s.ad_value(2422)), s.ad_value(2421)), A::sub(A::scale(A::square(s.ad_value(2421)), 0.3333333333333333), A::mul(s.ad_value(2420), s.ad_value(2437)))))));
            s.store_ad(2438, &assign51970_ad_e66864);
        }

        s.v[2476] = if (s.v[2438] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (s.v[2476] != 0.0)) {
            s.store_exp(2424, 2438);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (s.v[2476] != 0.0)) {
            s.store_div_from_scalar(2425, 1.0, 2424);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (s.v[2476] != 0.0)) {
            s.store_mul(2424, 2375, 2424);
        }

        s.v[2477] = if (s.v[2438] > (s.v[2373] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (!(s.v[2476] != 0.0))) && (s.v[2477] != 0.0)) {
            s.store_exp_ad(2424, A::sub(s.ad_value(2438), s.ad_value(2373)));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (!(s.v[2476] != 0.0))) && (s.v[2477] != 0.0)) {
            s.store_div(2425, 2375, 2424);
        }

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (!(s.v[2476] != 0.0))) && (!(s.v[2477] != 0.0))) {
            s.store_div_from_scalar_ad(2424, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2373), s.ad_value(2438)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2373), s.ad_value(2438)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2373), s.ad_value(2438)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) && (!(s.v[2476] != 0.0))) && (!(s.v[2477] != 0.0))) {
            s.store_div_from_scalar_ad(2425, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2438), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2438), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2438), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_div_from_scalar_ad(2414, 1.0, A::offset(A::square(s.ad_value(2438)), 2.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_mul_ad_lhs(2426, A::square(s.ad_value(2438)), 2414);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_scale_ad(2427, A::mul(A::mul(s.ad_value(2438), s.ad_value(2414)), s.ad_value(2414)), 4.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_mul_ad_lhs(2428, A::mul(A::sub(A::scale(s.ad_value(2414), 8.0), A::scale(s.ad_value(2426), 12.0)), s.ad_value(2414)), 2414);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub(2414, 2309, 2438);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_add_ad(2429, A::scale(s.ad_value(2414), 2.0), A::mul(s.ad_value(2291), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2425)), s.ad_value(2424)), A::mul(s.ad_value(2375), A::offset(s.ad_value(2427), 1.0)))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub_ad(2430, A::square(s.ad_value(2414)), A::mul(s.ad_value(2291), A::sub(A::add(A::offset(A::add(s.ad_value(2425), s.ad_value(2438)), (-1.0)), s.ad_value(2424)), A::mul(s.ad_value(2375), A::add(A::offset(s.ad_value(2438), 1.0), s.ad_value(2426))))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub_from_scalar_ad(2414, 2.0, A::mul(s.ad_value(2291), A::sub(A::add(s.ad_value(2425), s.ad_value(2424)), A::mul(s.ad_value(2375), s.ad_value(2428)))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_sub_ad(2414, A::square(s.ad_value(2429)), A::scale(A::mul(s.ad_value(2430), s.ad_value(2414)), 2.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2475] != 0.0))) {
            s.store_add_ad_rhs(2376, 2438, A::scale(A::div(s.ad_value(2430), A::add(s.ad_value(2429), A::sqrt(s.ad_value(2414)))), 2.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_sub(2377, 2376, 2329);
        }

        s.v[2478] = if (s.v[2377] < 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_add_ad(2378, A::scale(A::sub(s.ad_value(2309), s.ad_value(2329)), 2.0), A::mul(s.ad_value(2291), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2335)), A::mul(s.ad_value(2334), s.ad_value(2374))), A::mul(s.ad_value(2375), A::offset(s.ad_value(2332), 1.0)))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_mul_ad_lhs(2379, A::mul(s.ad_value(2291), A::sub_from_scalar(1.0, s.ad_value(2374))), 2336);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_sub_from_scalar_ad(1919, 2.0, A::mul(s.ad_value(2291), A::sub(A::add(s.ad_value(2335), A::mul(s.ad_value(2334), s.ad_value(2374))), A::mul(s.ad_value(2375), s.ad_value(2333)))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_sub_ad(1919, A::square(s.ad_value(2378)), A::scale(A::mul(s.ad_value(1919), s.ad_value(2379)), 2.0));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_scale_ad(2377, A::div(s.ad_value(2379), A::add(s.ad_value(2378), A::sqrt(s.ad_value(1919)))), 2.0);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_add(2376, 2329, 2377);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(2380, 2377, 2305);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_div_ad(2381, A::square(s.ad_value(2376)), A::offset(A::square(s.ad_value(2376)), 2.0));
        }

        s.v[2479] = if (s.v[2376] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) {
            s.store_exp_ad(2382, A::neg(s.ad_value(2376)));
        }

        s.v[2480] = if (s.v[2376] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (s.v[2480] != 0.0)) {
            s.store_scale_ad(2383, A::mul(A::square(s.ad_value(2376)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2376), A::sub_from_scalar(1.0, A::scale(s.ad_value(2376), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (s.v[2480] != 0.0)) {
            s.store_sqrt_ad(1919, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2376), A::sub_from_scalar(1.0, A::scale(s.ad_value(2376), 0.25))), 0.3333333333333333)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (s.v[2480] != 0.0)) {
            s.store_scaled_mul(2384, 2376, 1919, 0.7071067811865475);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (s.v[2480] != 0.0)) {
            s.store_mul_ad(2385, A::mul(A::mul(A::mul(A::scale(s.ad_value(2375), 0.16666666666666666), s.ad_value(2376)), s.ad_value(2376)), s.ad_value(2376)), A::offset(A::scale(s.ad_value(2376), 1.75), 1.0));
        }

    }

    pub(super) fn stamp_transient_block_35(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (!(s.v[2480] != 0.0))) {
            s.store_add_ad_lhs(2383, A::offset(s.ad_value(2376), (-1.0)), 2382);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (!(s.v[2480] != 0.0))) {
            s.store_sqrt(2384, 2383);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2479] != 0.0)) && (!(s.v[2480] != 0.0))) {
            s.store_mul_ad_rhs(2385, 2375, A::sub(A::offset(A::sub(A::div_from_scalar(1.0, s.ad_value(2382)), s.ad_value(2376)), (-1.0)), s.ad_value(2381)));
        }

        s.v[2481] = if (s.v[2376] > (s.v[2373] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) && (s.v[2481] != 0.0)) {
            s.store_exp_ad(1919, A::sub(s.ad_value(2376), s.ad_value(2373)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) && (s.v[2481] != 0.0)) {
            s.store_div(2382, 2375, 1919);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) && (s.v[2481] != 0.0)) {
            s.store_sub_ad_rhs(2385, 1919, A::mul(s.ad_value(2375), A::add(A::offset(s.ad_value(2376), 1.0), s.ad_value(2381))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) && (!(s.v[2481] != 0.0))) {
            s.store_div_from_scalar_ad(2382, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2376), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2376), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2376), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) && (!(s.v[2481] != 0.0))) {
            s.store_div_from_scalar_ad(1919, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2373), s.ad_value(2376)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2373), s.ad_value(2376)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2373), s.ad_value(2376)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) && (!(s.v[2481] != 0.0))) {
            s.store_sub_ad_rhs(2385, 1919, A::mul(s.ad_value(2375), A::add(A::offset(s.ad_value(2376), 1.0), s.ad_value(2381))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) {
            s.store_add_ad_lhs(2383, A::offset(s.ad_value(2376), (-1.0)), 2382);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2479] != 0.0))) {
            s.store_sqrt(2384, 2383);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul_ad_lhs(2386, A::mul(s.ad_value(2384), s.ad_value(2290)), 2305);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_scaled_add(2387, 2329, 2376, 0.5);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_scalar(2388, 0.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(1919, 2382, 2335);
        }

        s.v[2482] = if (s.v[1919] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_sqrt(2388, 1919);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_scaled_add(2389, 2336, 2385, 0.5);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add_ad_rhs(2390, 2389, A::scale(A::mul(A::square(s.ad_value(2377)), A::sub(s.ad_value(2388), A::scale(s.ad_value(2307), 2.0))), 0.125));
        }

        s.v[2483] = if (s.v[2387] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2483] != 0.0)) {
            s.store_scale_ad(2391, A::mul(A::square(s.ad_value(2387)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2387), A::sub_from_scalar(1.0, A::scale(s.ad_value(2387), 0.25))), 0.3333333333333333))), 0.5);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2483] != 0.0)) {
            s.store_mul_ad_rhs(2392, 2290, A::sqrt(A::add(s.ad_value(2390), s.ad_value(2391))));
        }

        s.v[2484] = if (s.v[724] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2483] != 0.0)) && (s.v[2484] != 0.0)) {
            s.store_div_from_scalar_ad(2393, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(724), s.ad_value(2392)), 1.0)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2483] != 0.0)) {
            s.store_sqrt_ad(1919, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2387), A::sub_from_scalar(1.0, A::scale(s.ad_value(2387), 0.25))), 0.3333333333333333)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2483] != 0.0)) {
            s.store_scaled_mul(2394, 2387, 1919, 0.7071067811865475);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2483] != 0.0)) {
            s.store_add_ad_rhs(2395, 2393, A::scale(A::div(A::mul(s.ad_value(2290), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2387), 0.5)), A::scale(A::square(s.ad_value(2387)), 0.16666666666666666))), s.ad_value(1919)), 0.7071067811865475));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) {
            s.store_add_ad_lhs(2391, A::offset(s.ad_value(2387), (-1.0)), 2388);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) {
            s.store_mul_ad_rhs(2392, 2290, A::sqrt(A::add(s.ad_value(2390), s.ad_value(2391))));
        }

        s.v[2485] = if (s.v[724] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_add_ad(2396, A::sub_from_scalar(1.0, s.ad_value(2388)), A::scale(A::mul(s.ad_value(2392), s.ad_value(2307)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_div_from_scalar_ad(2393, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(724), s.ad_value(2392)), 1.0)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_div_ad_rhs(1919, 2393, A::offset(s.ad_value(2393), 1.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_rhs(2397, 724, A::mul(A::mul(A::square(s.ad_value(1919)), s.ad_value(2291)), s.ad_value(2390)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_add_ad(2398, A::scale(A::sub(s.ad_value(2392), s.ad_value(2397)), 2.0), A::mul(s.ad_value(2291), A::add(A::sub_from_scalar(1.0, s.ad_value(2388)), s.ad_value(2390))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_rhs(2399, 2397, A::sub(s.ad_value(2397), A::scale(s.ad_value(2392), 2.0)));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_sub_from_scalar_ad(2400, 1.0, A::scale(A::mul(s.ad_value(2291), A::add(s.ad_value(2388), s.ad_value(2390))), 0.5));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_div_ad(2401, A::mul(s.ad_value(2399), s.ad_value(2398)), A::sub(A::square(s.ad_value(2398)), A::mul(s.ad_value(2400), s.ad_value(2399))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_add(2387, 2387, 2401);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_exp(2402, 2401);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_div(2388, 2388, 2402);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_mul(2390, 2390, 2402);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_add_ad_lhs(2391, A::offset(s.ad_value(2387), (-1.0)), 2388);
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_rhs(2392, 2290, A::sqrt(A::add(s.ad_value(2390), s.ad_value(2391))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_add_ad(2403, A::sub_from_scalar(1.0, s.ad_value(2388)), A::scale(A::mul(A::mul(s.ad_value(2392), s.ad_value(2393)), s.ad_value(2307)), 2.0));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_div_ad(2377, A::mul(A::mul(s.ad_value(2377), s.ad_value(2402)), A::add(s.ad_value(2396), s.ad_value(2389))), A::add(s.ad_value(2403), A::mul(s.ad_value(2402), s.ad_value(2389))));
        }

        if ((((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) && (s.v[2485] != 0.0)) {
            s.store_mul(2380, 2377, 2305);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) {
            s.store_sqrt(2394, 2391);
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2483] != 0.0))) {
            s.store_add_ad_rhs(2395, 2393, A::scale(A::div(A::mul(s.ad_value(2290), A::sub_from_scalar(1.0, s.ad_value(2388))), s.ad_value(2394)), 0.5));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul_ad_rhs(2404, 2305, A::div(A::mul(s.ad_value(2291), s.ad_value(2390)), A::add(s.ad_value(2392), A::mul(s.ad_value(2290), s.ad_value(2394)))));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add_ad_rhs(2405, 2404, A::mul(s.ad_value(2305), s.ad_value(2395)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul_ad_lhs(2406, A::mul(s.ad_value(2394), s.ad_value(2290)), 2305);
        }

        s.v[2486] = if (s.v[213] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2486] != 0.0)) {
            s.store_sub_from_scalar_ad(2345, 1.0, A::mul(s.ad_value(213), s.ad_value(2404)));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2486] != 0.0))) {
            s.store_div_from_scalar_ad(2345, 1.0, A::offset(A::mul(s.ad_value(213), s.ad_value(2404)), 1.0));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul_ad_lhs(2346, A::mul(A::mul(s.ad_value(751), s.ad_value(2344)), s.ad_value(2345)), 2404);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add_ad_rhs(2407, 2406, A::mul(s.ad_value(769), s.ad_value(2404)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add_ad_rhs(2408, 2406, A::mul(s.ad_value(770), s.ad_value(2404)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(2409, 768, 2407);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_ln_ad(1920, A::div(s.ad_value(2391), A::offset(A::add(s.ad_value(2391), s.ad_value(2390)), 1e-14)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_add_ad(2348, A::pow(A::mul(s.ad_value(2409), s.ad_value(698)), s.ad_value(699)), A::mul(s.ad_value(700), A::exp(A::mul(A::scale(s.ad_value(701), 0.5), s.ad_value(1920)))));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul_ad_lhs(2410, A::add(A::offset(s.ad_value(2348), 1.0), s.ad_value(2346)), 2340);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_ln_ad(2411, A::div(A::offset(A::mul(A::sub(s.ad_value(820), s.ad_value(2380)), s.ad_value(773)), 1.0), A::offset(A::mul(A::sub(s.ad_value(2371), s.ad_value(2380)), s.ad_value(773)), 1.0)));
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(1921, 2404, 2350);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_div_ad_rhs(2351, 1921, A::add(s.ad_value(218), s.ad_value(1921)));
        }

        s.v[2487] = if (s.v[217] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (s.v[2487] != 0.0)) {
            s.store_div_from_scalar_ad(2352, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(217), s.ad_value(2351))));
        }

        if (((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) && (!(s.v[2487] != 0.0))) {
            s.store_offset_ad(2352, A::mul(s.ad_value(217), s.ad_value(2351)), 1.0);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(2413, 2285, 2352);
        }

        if ((s.v[2439] != 0.0) && (s.v[2467] != 0.0)) {
            s.store_mul(2412, 2392, 2305);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1875, 2287);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1876, 2305);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1877, 2290);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1878, 2309);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1879, 2314);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1880, 2343);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1881, 2380);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1882, 2386);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1883, 2393);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1884, 2395);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1885, 2404);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1886, 2405);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1887, 2408);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1888, 2410);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1889, 2411);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1890, 2413);
        }

        if (s.v[2439] != 0.0) {
            s.copy_ad(1891, 2412);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(739, 722);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1875, 1810);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1876, 1812);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1877, 1814);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1878, 1817);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1879, 1818);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1880, 1837);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1881, 1848);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1882, 1849);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1883, 1851);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1884, 1852);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1885, 1853);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1886, 1854);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1887, 1856);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1888, 1857);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1889, 1859);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1890, 1858);
        }

        if (!(s.v[2439] != 0.0)) {
            s.copy_ad(1891, 1860);
        }

        s.copy_ad(1892, 250);

        s.v[2488] = if (s.v[767] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2488] != 0.0) {
            s.store_div_ad_rhs(1892, 250, A::offset(A::mul(s.ad_value(767), A::powf(A::offset(A::square(s.ad_value(1887)), s.v[727]), ((-1.0) * 0.16666666666666666))), 1.0));
        }

        s.v[1893] = 1.0;

        s.v[1894] = 1.0;

        s.v[1895] = 0.0;

        s.v[1896] = 1.0;

        s.v[1897] = 1.0;

        s.copy_ad(2251, 1891);

        s.v[2254] = 0.0;

        s.v[2253] = 0.0;

        s.copy_ad(2255, 2251);

        s.v[2489] = if (s.v[1878] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2489] != 0.0) {
            s.store_mul_ad_lhs(2246, A::div(A::mul(A::add(s.ad_value(255), A::div(s.ad_value(256), s.ad_value(1886))), s.ad_value(1885)), s.ad_value(1886)), 1889);
        }

        s.v[2490] = if (s.v[2246] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2489] != 0.0) && (s.v[2490] != 0.0)) {
            s.store_div_from_scalar_ad(1893, 1.0, A::add(A::offset(s.ad_value(2246), 1.0), A::square(s.ad_value(2246))));
        }

        if ((s.v[2489] != 0.0) && (!(s.v[2490] != 0.0))) {
            s.store_sub_from_scalar(1893, 1.0, 2246);
        }

        if (s.v[2489] != 0.0) {
            s.store_mul(1894, 1888, 1893);
        }

        if (s.v[2489] != 0.0) {
            s.store_div(1895, 1890, 1894);
        }

        if (s.v[2489] != 0.0) {
            s.store_mul_ad_lhs(2247, A::mul(A::square(s.ad_value(1895)), s.ad_value(1881)), 1881);
        }

        s.v[2491] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2489] != 0.0) && (s.v[2491] != 0.0)) {
            s.store_div_ad_rhs(2247, 2247, A::offset(A::mul(s.ad_value(1895), s.ad_value(1881)), 1.0));
        }

        if (s.v[2489] != 0.0) {
            s.store_scale_ad(1896, A::mul(s.ad_value(1894), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2247), 2.0), 1.0)), 1.0)), 0.5);
        }

        if (s.v[2489] != 0.0) {
            s.store_div(1919, 1894, 1896);
        }

        if (s.v[2489] != 0.0) {
            s.store_mul_ad_rhs(2248, 1884, A::offset(A::scale(A::mul(A::mul(s.ad_value(2247), s.ad_value(1919)), s.ad_value(1919)), 0.5), 1.0));
        }

        if (s.v[2489] != 0.0) {
            s.store_div_ad_lhs(1897, A::mul(s.ad_value(1919), s.ad_value(1886)), 2248);
        }

        if (s.v[2489] != 0.0) {
            s.store_scaled_div(2249, 1881, 1897, 0.5);
        }

        if (s.v[2489] != 0.0) {
            s.store_square(2250, 2249);
        }

        if (s.v[2489] != 0.0) {
            s.store_add_ad_rhs(2251, 1891, A::scale(A::mul(A::mul(s.ad_value(1883), s.ad_value(1881)), A::add(A::offset(A::scale(A::mul(s.ad_value(2249), s.ad_value(1893)), 0.3333333333333333), (-1.0)), s.ad_value(1893))), 0.5));
        }

        if (s.v[2489] != 0.0) {
            s.store_scaled_mul(1919, 1884, 1881, 0.16666666666666666);
        }

        s.v[2492] = if (p.p49 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2489] != 0.0) && (s.v[2492] != 0.0)) {
            s.store_scalar(2252, 0.0);
        }

        if ((s.v[2489] != 0.0) && (s.v[2492] != 0.0)) {
            s.store_mul_ad(2253, A::mul(A::scale(s.ad_value(1893), 0.5), s.ad_value(1893)), A::sub(s.ad_value(1885), A::mul(A::scale(s.ad_value(1919), 3.0), A::sub_from_scalar(2.0, s.ad_value(2249)))));
        }

    }

    pub(super) fn stamp_transient_block_36(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[2489] != 0.0) && (!(s.v[2492] != 0.0))) {
            s.store_mul_ad(2252, A::sub_from_scalar(1.0, s.ad_value(1893)), A::sub(s.ad_value(1885), A::scale(A::mul(s.ad_value(1884), s.ad_value(1881)), 0.5)));
        }

        if ((s.v[2489] != 0.0) && (!(s.v[2492] != 0.0))) {
            s.store_scale_ad(2253, A::add(A::mul(A::square(s.ad_value(1893)), A::sub(s.ad_value(1885), A::mul(s.ad_value(1919), A::sub(A::sub_from_scalar(1.0, s.ad_value(2249)), A::scale(s.ad_value(2250), 0.2))))), A::mul(s.ad_value(2252), A::offset(s.ad_value(1893), 1.0))), 0.5);
        }

        if (s.v[2489] != 0.0) {
            s.store_add_ad_lhs(2254, A::mul(s.ad_value(1893), A::add(s.ad_value(1885), A::mul(s.ad_value(1919), s.ad_value(2249)))), 2252);
        }

        if (s.v[2489] != 0.0) {
            s.store_sub(2255, 2251, 2254);
        }

        s.store_mul(845, 2251, 1892);

        s.store_mul_ad_lhs(847, A::neg(s.ad_value(2253)), 1892);

        s.store_mul_ad_lhs(846, A::neg(s.ad_value(2255)), 1892);

        s.v[2271] = 0.0;

        s.v[2272] = 0.0;

        s.v[2270] = 0.0;

        s.v[2493] = if ((s.v[263] > 0.0) || (s.v[264] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2493] != 0.0) {
            s.store_scalar(2260, 1.0);
        }

        if (s.v[2493] != 0.0) {
            s.copy_ad(2259, 1875);
        }

        s.v[2494] = if (s.v[267] > 1e-10) { 1.0 } else { 0.0 };

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_add_ad_lhs(2256, A::sub(s.ad_value(1875), s.ad_value(265)), 802);
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_scale_ad(1919, A::add(A::add(s.ad_value(2256), s.ad_value(802)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(2256), s.ad_value(802)), A::sub(s.ad_value(2256), s.ad_value(802))), s.ad_value(803)))), 0.5);
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_mul_ad_rhs(1920, 1919, A::sub(A::sub(A::scale(s.ad_value(1919), 2.0), s.ad_value(802)), s.ad_value(2256)));
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_div(1921, 802, 1919);
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_mul(2257, 2256, 1921);
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_sqrt_ad(2258, A::sub_from_scalar(1.0, A::mul(s.ad_value(2257), s.ad_value(267))));
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_sub_ad_lhs(2259, A::add(A::div(A::sub_from_scalar(1.0, s.ad_value(2258)), s.ad_value(267)), s.ad_value(2256)), 2257);
        }

        if ((s.v[2493] != 0.0) && (s.v[2494] != 0.0)) {
            s.store_offset_ad(2260, A::div(A::mul(A::mul(A::offset(A::div_from_scalar(0.5, s.ad_value(2258)), (-1.0)), A::add(s.ad_value(1920), A::mul(s.ad_value(2256), A::sub(s.ad_value(802), s.ad_value(1919))))), s.ad_value(1921)), s.ad_value(1920)), 1.0);
        }

        if (s.v[2493] != 0.0) {
            s.store_scalar(2262, 1.0);
        }

        if (s.v[2493] != 0.0) {
            s.store_scalar(2263, 0.0);
        }

        s.v[2495] = if (s.v[266] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) {
            s.store_add_ad(1919, A::scale(s.ad_value(739), 0.5), A::mul(s.ad_value(1876), A::offset(A::scale(s.ad_value(1877), 0.7071067811865475), 1.0)));
        }

        if ((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) {
            s.store_div(2261, 1875, 1919);
        }

        s.v[2496] = if (((s.v[2261]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) && (s.v[2496] != 0.0)) {
            s.store_div_from_scalar_ad(2262, 1.0, A::offset(A::exp(A::neg(s.ad_value(2261))), 1.0));
        }

        s.v[2497] = if (s.v[2261] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) && (!(s.v[2496] != 0.0))) && (s.v[2497] != 0.0)) {
            s.store_div_from_scalar_ad(2262, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2261), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2261), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2261), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2498] = if (s.v[2261] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) && (s.v[2498] != 0.0)) {
            s.store_ln_ad(1920, A::offset(A::exp(s.ad_value(2261)), 1.0));
        }

        if (((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) && (!(s.v[2498] != 0.0))) {
            s.copy_ad(1920, 2261);
        }

        if ((s.v[2493] != 0.0) && (s.v[2495] != 0.0)) {
            s.store_mul(2263, 1919, 1920);
        }

        if (s.v[2493] != 0.0) {
            s.store_add_ad_lhs(2264, A::mul(s.ad_value(266), A::sub(s.ad_value(2262), s.ad_value(2260))), 2260);
        }

        if (s.v[2493] != 0.0) {
            s.store_add_ad_lhs(2265, A::mul(s.ad_value(266), A::sub(s.ad_value(2263), s.ad_value(2259))), 2259);
        }

        if (s.v[2493] != 0.0) {
            s.store_sub_ad(2266, A::sub(A::sub(s.ad_value(1875), A::mul(s.ad_value(1876), s.ad_value(1879))), s.ad_value(1891)), A::scale(s.ad_value(1881), 0.5));
        }

        if (s.v[2493] != 0.0) {
            s.store_sub_ad_lhs(2267, A::sub(s.ad_value(1875), s.ad_value(2266)), 1880);
        }

        if (s.v[2493] != 0.0) {
            s.store_sub_ad_lhs(2268, A::add(s.ad_value(1881), s.ad_value(2266)), 820);
        }

        if (s.v[2493] != 0.0) {
            s.store_sub_ad_lhs(2269, A::sub(s.ad_value(1875), s.ad_value(2268)), 1882);
        }

        s.v[2499] = if (s.v[825] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2493] != 0.0) && (s.v[2499] != 0.0)) {
            s.store_mul_ad_rhs(2270, 2264, A::add(A::mul(s.ad_value(264), s.ad_value(2268)), A::mul(s.ad_value(263), s.ad_value(2266))));
        }

        if ((s.v[2493] != 0.0) && (s.v[2499] != 0.0)) {
            s.store_mul_ad_rhs(2271, 263, A::sub(s.ad_value(2267), s.ad_value(2265)));
        }

        if ((s.v[2493] != 0.0) && (s.v[2499] != 0.0)) {
            s.store_mul_ad_rhs(2272, 264, A::sub(s.ad_value(2269), s.ad_value(2265)));
        }

        if ((s.v[2493] != 0.0) && (!(s.v[2499] != 0.0))) {
            s.store_mul_ad_rhs(2270, 2264, A::add(A::mul(s.ad_value(263), s.ad_value(2268)), A::mul(s.ad_value(264), s.ad_value(2266))));
        }

        if ((s.v[2493] != 0.0) && (!(s.v[2499] != 0.0))) {
            s.store_mul_ad_rhs(2271, 264, A::sub(s.ad_value(2267), s.ad_value(2265)));
        }

        if ((s.v[2493] != 0.0) && (!(s.v[2499] != 0.0))) {
            s.store_mul_ad_rhs(2272, 263, A::sub(s.ad_value(2269), s.ad_value(2265)));
        }

        if (s.v[2493] != 0.0) {
            s.store_add(845, 845, 2270);
        }

        if (s.v[2493] != 0.0) {
            s.store_add(847, 847, 2272);
        }

        if (s.v[2493] != 0.0) {
            s.store_sub_ad_lhs(846, A::sub(A::sub(s.ad_value(846), s.ad_value(2270)), s.ad_value(2272)), 2271);
        }

        s.store_mul(1898, 257, 1866);

        s.store_mul(1899, 258, 1867);

        s.v[2275] = 0.0;

        s.v[2273] = 0.0;

        s.v[2500] = if ((s.v[257] > 0.0) && (s.v[259] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2500] != 0.0) {
            s.store_mul_ad_rhs(1919, 261, A::add(A::scale(s.ad_value(1807), 0.5), s.ad_value(781)));
        }

        s.v[2501] = if (s.v[1919] < 230.25850929940458) { 1.0 } else { 0.0 };

        s.v[2502] = if (s.v[1919] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2500] != 0.0) && (s.v[2501] != 0.0)) && (s.v[2502] != 0.0)) {
            s.store_exp(2273, 1919);
        }

        if (((s.v[2500] != 0.0) && (s.v[2501] != 0.0)) && (!(s.v[2502] != 0.0))) {
            s.store_div_from_scalar_ad(2273, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2503] = if (s.v[2273] > 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2500] != 0.0) && (s.v[2501] != 0.0)) && (s.v[2503] != 0.0)) {
            s.store_ln_ad(2274, A::offset(s.ad_value(2273), 1.0));
        }

        if (((s.v[2500] != 0.0) && (s.v[2501] != 0.0)) && (s.v[2503] != 0.0)) {
            s.store_mul_ad_rhs(1920, 2274, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2274), 1.0)), A::offset(s.ad_value(2274), 2.0))));
        }

        if (((s.v[2500] != 0.0) && (s.v[2501] != 0.0)) && (!(s.v[2503] != 0.0))) {
            s.copy_ad(2274, 2273);
        }

        if (((s.v[2500] != 0.0) && (s.v[2501] != 0.0)) && (!(s.v[2503] != 0.0))) {
            s.store_div_ad(1920, A::scale(s.ad_value(2274), 2.0), A::offset(s.ad_value(2274), 2.0));
        }

        if ((s.v[2500] != 0.0) && (!(s.v[2501] != 0.0))) {
            s.copy_ad(2274, 1919);
        }

        if ((s.v[2500] != 0.0) && (!(s.v[2501] != 0.0))) {
            s.store_mul_ad_rhs(1920, 2274, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2274), 1.0)), A::offset(s.ad_value(2274), 2.0))));
        }

        if (s.v[2500] != 0.0) {
            s.store_mul_ad_lhs(2275, A::scale(A::mul(A::div(A::scale(s.ad_value(259), (-2.0)), s.ad_value(261)), s.ad_value(257)), s.v[348]), 1920);
        }

        s.v[2278] = 0.0;

        s.v[2276] = 0.0;

        s.v[2504] = if ((s.v[258] > 0.0) && (s.v[260] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2504] != 0.0) {
            s.store_mul_ad_rhs(1919, 261, A::add(A::scale(s.ad_value(1807), 0.5), s.ad_value(782)));
        }

        s.v[2505] = if (s.v[1919] < 230.25850929940458) { 1.0 } else { 0.0 };

        s.v[2506] = if (s.v[1919] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2504] != 0.0) && (s.v[2505] != 0.0)) && (s.v[2506] != 0.0)) {
            s.store_exp(2276, 1919);
        }

        if (((s.v[2504] != 0.0) && (s.v[2505] != 0.0)) && (!(s.v[2506] != 0.0))) {
            s.store_div_from_scalar_ad(2276, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1919)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2507] = if (s.v[2276] > 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2504] != 0.0) && (s.v[2505] != 0.0)) && (s.v[2507] != 0.0)) {
            s.store_ln_ad(2277, A::offset(s.ad_value(2276), 1.0));
        }

        if (((s.v[2504] != 0.0) && (s.v[2505] != 0.0)) && (s.v[2507] != 0.0)) {
            s.store_mul_ad_rhs(1920, 2277, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2277), 1.0)), A::offset(s.ad_value(2277), 2.0))));
        }

        if (((s.v[2504] != 0.0) && (s.v[2505] != 0.0)) && (!(s.v[2507] != 0.0))) {
            s.copy_ad(2277, 2276);
        }

        if (((s.v[2504] != 0.0) && (s.v[2505] != 0.0)) && (!(s.v[2507] != 0.0))) {
            s.store_div_ad(1920, A::scale(s.ad_value(2277), 2.0), A::offset(s.ad_value(2277), 2.0));
        }

        if ((s.v[2504] != 0.0) && (!(s.v[2505] != 0.0))) {
            s.copy_ad(2277, 1919);
        }

        if ((s.v[2504] != 0.0) && (!(s.v[2505] != 0.0))) {
            s.store_mul_ad_rhs(1920, 2277, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2277), 1.0)), A::offset(s.ad_value(2277), 2.0))));
        }

        if (s.v[2504] != 0.0) {
            s.store_mul_ad_lhs(2278, A::scale(A::mul(A::div(A::scale(s.ad_value(260), (-2.0)), s.ad_value(261)), s.ad_value(258)), s.v[348]), 1920);
        }

        s.store_add(2279, 2275, 2278);

        s.store_add_ad_lhs(850, A::mul(s.ad_value(262), s.ad_value(823)), 2279);

        s.store_mul(848, 269, 828);

        s.store_mul(849, 270, 831);

        s.v[2508] = 0.0;

        s.v[2509] = 0.0;

        s.v[2510] = 0.0;

        s.v[2511] = 0.0;

        s.v[2512] = 0.0;

        s.v[2513] = 0.0;

        s.v[2514] = 0.0;

        s.v[2515] = 0.0;

        s.v[2516] = 0.0;

        s.v[2517] = 0.0;

        s.v[2518] = 0.0;

        s.v[2519] = 0.0;

        s.v[2520] = 0.0;

        s.v[2521] = 0.0;

        s.v[2522] = 0.0;

        s.v[2523] = 0.0;

        s.v[2524] = 0.0;

        s.v[2525] = 0.0;

        s.v[2526] = 0.0;

        s.v[2527] = 0.0;

        s.v[2528] = 0.0;

        s.v[2529] = 0.0;

        s.v[2530] = 0.0;

        s.v[2531] = 0.0;

        s.v[2532] = 0.0;

        s.v[2533] = 0.0;

        s.v[2534] = 0.0;

        s.v[2535] = 0.0;

        s.v[2536] = 0.0;

        s.v[2537] = 0.0;

        s.v[2538] = 0.0;

        s.v[2539] = 0.0;

        s.v[2540] = 0.0;

        s.v[2541] = 0.0;

        s.v[2542] = 0.0;

        s.v[2543] = 0.0;

        s.v[2544] = 0.0;

        s.v[2545] = 0.0;

        s.v[2546] = 0.0;

        s.v[2547] = 0.0;

        s.v[2548] = 0.0;

        s.v[2549] = 0.0;

        s.v[2550] = 0.0;

        s.v[2551] = 0.0;

        s.v[2552] = 0.0;

        s.v[2553] = 0.0;

        s.v[2554] = 0.0;

        s.v[842] = 0.0;

        s.v[1900] = 0.0;

        s.v[1901] = 0.0;

        s.v[1902] = 0.0;

        s.v[843] = 0.0;

        s.v[1903] = 0.0;

        s.v[1904] = 0.0;

        s.v[1905] = 0.0;

        s.v[851] = 0.0;

        s.v[1906] = 0.0;

        s.v[1907] = 0.0;

        s.v[1908] = 0.0;

        s.v[852] = 0.0;

        s.v[1909] = 0.0;

        s.v[1910] = 0.0;

        s.v[1911] = 0.0;

        s.v[2555] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        s.v[2556] = if (s.v[468] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scale(490, 826, (s.v[365] * s.v[662]));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            let assign55290_ad_e69768: A = {
                if (s.v[490] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(490)), 1.0))
                } else {
                    {
                        if (s.v[490] > s.v[654]) {
                            A::mul(s.ad_value(655), A::offset(A::sub(s.ad_value(490), s.ad_value(654)), 1.0))
                        } else {
                            A::exp(s.ad_value(490))
                        }
                    }
                }
            };
            s.store_ad(491, &assign55290_ad_e69768);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_mul_ad_rhs(496, 661, A::offset(s.ad_value(491), (-1.0)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_mul_ad_lhs(490, A::scale(s.ad_value(826), s.v[365]), 664);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            let assign55320_ad_e69819: A = {
                if (s.v[490] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(490)), 1.0))
                } else {
                    {
                        if (s.v[490] > s.v[656]) {
                            A::mul(s.ad_value(657), A::offset(A::sub(s.ad_value(490), s.ad_value(656)), 1.0))
                        } else {
                            A::exp(s.ad_value(490))
                        }
                    }
                }
            };
            s.store_ad(491, &assign55320_ad_e69819);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_mul_ad_rhs(497, 663, A::offset(s.ad_value(491), (-1.0)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scalar(498, 0.0);
        }

        s.v[2557] = if (s.v[660] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2557] != 0.0)) {
            s.store_mul_ad_rhs(498, 826, A::add(s.ad_value(665), A::mul(s.ad_value(826), s.ad_value(666))));
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (!(s.v[2557] != 0.0))) {
            s.store_mul_ad_lhs(490, A::scale(A::neg(s.ad_value(826)), s.v[365]), 666);
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (!(s.v[2557] != 0.0))) {
            let assign55380_ad_e69900: A = {
                if (s.v[490] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(490)), 1.0))
                } else {
                    {
                        if (s.v[490] > s.v[658]) {
                            A::mul(s.ad_value(659), A::offset(A::sub(s.ad_value(490), s.ad_value(658)), 1.0))
                        } else {
                            A::exp(s.ad_value(490))
                        }
                    }
                }
            };
            s.store_ad(491, &assign55380_ad_e69900);
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (!(s.v[2557] != 0.0))) {
            s.store_mul_ad(498, A::neg(s.ad_value(665)), A::offset(s.ad_value(491), (-1.0)));
        }

    }

    pub(super) fn stamp_transient_block_37(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_add_ad_lhs(842, A::add(s.ad_value(496), s.ad_value(497)), 498);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scale(490, 827, (s.v[365] * s.v[689]));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            let assign55420_ad_e69965: A = {
                if (s.v[490] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(490)), 1.0))
                } else {
                    {
                        if (s.v[490] > s.v[681]) {
                            A::mul(s.ad_value(682), A::offset(A::sub(s.ad_value(490), s.ad_value(681)), 1.0))
                        } else {
                            A::exp(s.ad_value(490))
                        }
                    }
                }
            };
            s.store_ad(491, &assign55420_ad_e69965);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_mul_ad_rhs(496, 688, A::offset(s.ad_value(491), (-1.0)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_mul_ad_lhs(490, A::scale(s.ad_value(827), s.v[365]), 691);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            let assign55450_ad_e70016: A = {
                if (s.v[490] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(490)), 1.0))
                } else {
                    {
                        if (s.v[490] > s.v[683]) {
                            A::mul(s.ad_value(684), A::offset(A::sub(s.ad_value(490), s.ad_value(683)), 1.0))
                        } else {
                            A::exp(s.ad_value(490))
                        }
                    }
                }
            };
            s.store_ad(491, &assign55450_ad_e70016);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_mul_ad_rhs(497, 690, A::offset(s.ad_value(491), (-1.0)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scalar(498, 0.0);
        }

        s.v[2558] = if (s.v[687] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2558] != 0.0)) {
            s.store_mul_ad_rhs(498, 827, A::add(s.ad_value(692), A::mul(s.ad_value(827), s.ad_value(693))));
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (!(s.v[2558] != 0.0))) {
            s.store_mul_ad_lhs(490, A::scale(A::neg(s.ad_value(827)), s.v[365]), 693);
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (!(s.v[2558] != 0.0))) {
            let assign55510_ad_e70097: A = {
                if (s.v[490] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(490)), 1.0))
                } else {
                    {
                        if (s.v[490] > s.v[685]) {
                            A::mul(s.ad_value(686), A::offset(A::sub(s.ad_value(490), s.ad_value(685)), 1.0))
                        } else {
                            A::exp(s.ad_value(490))
                        }
                    }
                }
            };
            s.store_ad(491, &assign55510_ad_e70097);
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (!(s.v[2558] != 0.0))) {
            s.store_mul_ad(498, A::neg(s.ad_value(692)), A::offset(s.ad_value(491), (-1.0)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_add_ad_lhs(843, A::add(s.ad_value(496), s.ad_value(497)), 498);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scalar(2559, 0.0);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scalar(2560, 0.0);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(651), 4.0), 651);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_div(2512, 651, 652);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_add_ad_rhs(2513, 826, A::mul(s.ad_value(651), s.ad_value(2512)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_add(2514, 652, 2513);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_sub(2515, 652, 2513);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scale_ad(2560, A::div(A::mul(s.ad_value(826), s.ad_value(652)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2561] = if (s.v[645] > 0.5) { 1.0 } else { 0.0 };

        s.v[2562] = if (s.v[402] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2561] != 0.0)) && (s.v[2562] != 0.0)) {
            s.store_sqrt_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[399])));
        }

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2561] != 0.0)) && (!(s.v[2562] != 0.0))) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[399])), s.v[402]);
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2561] != 0.0)) {
            s.store_add_ad(1906, A::scale(A::sub_from_scalar(1.0, s.ad_value(2559)), s.v[411]), A::scale(A::sub(s.ad_value(826), s.ad_value(2560)), s.v[414]));
        }

        s.v[2563] = if (s.v[646] > 0.5) { 1.0 } else { 0.0 };

        s.v[2564] = if (s.v[403] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2563] != 0.0)) && (s.v[2564] != 0.0)) {
            s.store_sqrt_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[400])));
        }

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2563] != 0.0)) && (!(s.v[2564] != 0.0))) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[400])), s.v[403]);
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2563] != 0.0)) {
            s.store_add_ad(1907, A::scale(A::sub_from_scalar(1.0, s.ad_value(2559)), s.v[412]), A::scale(A::sub(s.ad_value(826), s.ad_value(2560)), s.v[415]));
        }

        s.v[2565] = if (s.v[647] > 0.5) { 1.0 } else { 0.0 };

        s.v[2566] = if (s.v[404] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2566] != 0.0)) {
            s.store_sqrt_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[401])));
        }

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2566] != 0.0))) {
            s.store_powf_ad(2559, A::sub_from_scalar(1.0, A::scale(s.ad_value(2560), s.v[401])), s.v[404]);
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_add_ad(1908, A::scale(A::sub_from_scalar(1.0, s.ad_value(2559)), s.v[413]), A::scale(A::sub(s.ad_value(826), s.ad_value(2560)), s.v[416]));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scalar(2559, 0.0);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scalar(2560, 0.0);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(678), 4.0), 678);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_div(2512, 678, 679);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_add_ad_rhs(2513, 827, A::mul(s.ad_value(678), s.ad_value(2512)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_add(2514, 679, 2513);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_sub(2515, 679, 2513);
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if ((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) {
            s.store_scale_ad(2560, A::div(A::mul(s.ad_value(827), s.ad_value(679)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2567] = if (s.v[672] > 0.5) { 1.0 } else { 0.0 };

        s.v[2568] = if (s.v[569] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2567] != 0.0)) && (s.v[2568] != 0.0)) {
            s.store_sqrt_ad(2559, A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(566))));
        }

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2567] != 0.0)) && (!(s.v[2568] != 0.0))) {
            s.store_ad(2559, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(566))), s.ad_value(569)));
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2567] != 0.0)) {
            s.store_add_ad(1909, A::mul(s.ad_value(578), A::sub_from_scalar(1.0, s.ad_value(2559))), A::mul(s.ad_value(581), A::sub(s.ad_value(827), s.ad_value(2560))));
        }

        s.v[2569] = if (s.v[673] > 0.5) { 1.0 } else { 0.0 };

        s.v[2570] = if (s.v[570] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2569] != 0.0)) && (s.v[2570] != 0.0)) {
            s.store_sqrt_ad(2559, A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(567))));
        }

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2569] != 0.0)) && (!(s.v[2570] != 0.0))) {
            s.store_ad(2559, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(567))), s.ad_value(570)));
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_add_ad(1910, A::mul(s.ad_value(579), A::sub_from_scalar(1.0, s.ad_value(2559))), A::mul(s.ad_value(582), A::sub(s.ad_value(827), s.ad_value(2560))));
        }

        s.v[2571] = if (s.v[674] > 0.5) { 1.0 } else { 0.0 };

        s.v[2572] = if (s.v[571] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2571] != 0.0)) && (s.v[2572] != 0.0)) {
            s.store_sqrt_ad(2559, A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(568))));
        }

        if ((((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2571] != 0.0)) && (!(s.v[2572] != 0.0))) {
            s.store_ad(2559, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2560), s.ad_value(568))), s.ad_value(571)));
        }

        if (((s.v[2555] != 0.0) && (s.v[2556] != 0.0)) && (s.v[2571] != 0.0)) {
            s.store_add_ad(1911, A::mul(s.ad_value(580), A::sub_from_scalar(1.0, s.ad_value(2559))), A::mul(s.ad_value(583), A::sub(s.ad_value(827), s.ad_value(2560))));
        }

        s.v[2573] = if (p.p865 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2573] != 0.0)) {
            s.store_scale_ad(636, A::offset(A::powf(A::scale(A::add(A::add(s.ad_value(819), s.ad_value(821)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001)))), 0.5), p.p866), (-(((0.5 * 0.001)) as f64).powf(p.p866))), p.p865);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2573] != 0.0)) {
            s.store_offset(634, 636, p.p855);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2573] != 0.0)) {
            s.store_div_from_scalar(444, 1.0, 634);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2573] != 0.0)) {
            s.store_div_from_scalar_ad(447, s.v[447], A::offset(A::scale(s.ad_value(636), 1.0 / (p.p855)), 1.0));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2573] != 0.0))) {
            s.store_scalar(634, p.p855);
        }

        s.v[2574] = if (p.p867 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2574] != 0.0)) {
            s.store_scale_ad(638, A::offset(A::powf(A::scale(A::add(A::add(s.ad_value(819), s.ad_value(821)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001)))), 0.5), p.p868), (-(((0.5 * 0.001)) as f64).powf(p.p868))), p.p867);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2574] != 0.0)) {
            s.store_mul_ad_rhs(437, 437, A::offset(s.ad_value(638), 1.0));
        }

        if ((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) {
            s.store_scalar(2524, 0.0);
        }

        if ((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) {
            s.store_scalar(2521, 0.0);
        }

        s.v[2575] = if !(((s.v[640] == 0.0) && (s.v[641] == 0.0)) && (s.v[642] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(651), 4.0), 651);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_div(2512, 651, 652);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_add_ad_rhs(2513, 826, A::mul(s.ad_value(651), s.ad_value(2512)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_add(2514, 652, 2513);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_sub(2515, 652, 2513);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_scale_ad(2518, A::div(A::mul(s.ad_value(826), s.ad_value(652)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2576] = if (s.v[826] < s.v[648]) { 1.0 } else { 0.0 };

        s.v[2577] = if (((((-0.5) * (s.v[826] * s.v[365]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2577] != 0.0)) {
            s.store_exp_ad(2519, A::scale(s.ad_value(826), (s.v[365] * (-0.5))));
        }

        s.v[2578] = if (((-0.5) * (s.v[826] * s.v[365])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (!(s.v[2577] != 0.0))) && (s.v[2578] != 0.0)) {
            let assign56250_ad_e70980: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(826), (s.v[365] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(826), (s.v[365] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(826), (s.v[365] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2519, &assign56250_ad_e70980);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (!(s.v[2577] != 0.0))) && (!(s.v[2578] != 0.0))) {
            s.store_scale_ad(2519, A::offset(A::mul(A::offset(A::scale(s.ad_value(826), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(826), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(826), (s.v[365] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_div_from_scalar(2520, 1.0, 2519);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_square(2517, 2520);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (!(s.v[2576] != 0.0))) {
            s.store_mul_ad_lhs(2517, A::offset(A::scale(A::sub(s.ad_value(826), s.ad_value(648)), s.v[365]), 1.0), 649);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (!(s.v[2576] != 0.0))) {
            s.store_sqrt(2520, 2517);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (!(s.v[2576] != 0.0))) {
            s.store_div_from_scalar(2519, 1.0, 2520);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_offset(2517, 2517, (-1.0));
        }

        s.v[2579] = if (s.v[826] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_scale_ad(2521, A::ln(A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2519), 1.0), A::offset(s.ad_value(2519), 3.0))))), (s.v[364] * 2.0));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) && (!(s.v[2579] != 0.0))) {
            s.store_sub_ad_lhs(2521, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2520), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2520), 1.0), A::offset(A::scale(s.ad_value(2520), 3.0), 1.0))))), (s.v[364] * 2.0)), 826);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_sub(2522, 650, 2521);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_scale_ad(2523, A::sub(A::add(s.ad_value(826), s.ad_value(2522)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(826), s.ad_value(2522)), A::sub(s.ad_value(826), s.ad_value(2522))), ((4.0 * s.v[364]) * s.v[364])))), 0.5);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_scale_ad(2524, A::sub(A::add(s.ad_value(826), s.ad_value(653)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(826), s.ad_value(653)), A::sub(s.ad_value(826), s.ad_value(653))), ((4.0 * s.v[362]) * s.v[362])))), 0.5);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2575] != 0.0)) {
            s.store_scale_ad(2525, A::sub(s.ad_value(826), A::sqrt(A::offset(A::mul(s.ad_value(826), s.ad_value(826)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[2580] = if (s.v[640] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2580] != 0.0)) {
            s.store_scalar(1900, 0.0);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2580] != 0.0)) {
            s.store_scalar(1906, 0.0);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) {
            s.store_scale(2527, 2517, s.v[381]);
        }

        s.v[2581] = if ((p.p833 == 0.0) && (p.p838 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (s.v[2581] != 0.0)) {
            s.store_scalar(2528, 0.0);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) {
            s.store_sub_from_scalar(2529, s.v[387], 2523);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) {
            s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));
        }

        s.v[2582] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) && (s.v[2582] != 0.0)) {
            s.store_scalar(2531, 0.0);
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) && (!(s.v[2582] != 0.0))) {
            s.store_scale_ad(2531, A::add(A::div(A::mul(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530))), A::sub_from_scalar(1.0, s.ad_value(2530))), s.ad_value(2530)), (1.0 - (2.0 * p.p824)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) {
            s.store_add(2532, 2530, 2531);
        }

        s.v[2583] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) && (s.v[2583] != 0.0)) {
            s.store_sqrt_ad(2526, A::scale(s.ad_value(2529), s.v[423]));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) && (!(s.v[2583] != 0.0))) {
            s.store_powf_ad(2526, A::scale(s.ad_value(2529), s.v[423]), p.p824);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) {
            s.store_scale(2533, 2526, s.v[417]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) {
            s.store_scale_ad(2534, A::mul(A::offset(s.ad_value(2520), (-1.0)), s.ad_value(2533)), s.v[378]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2581] != 0.0))) {
            s.store_scaled_mul(2528, 2534, 2532, p.p833);
        }

        s.v[2584] = if (p.p838 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (s.v[2584] != 0.0)) {
            s.store_scalar(2535, 0.0);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_scale_ad(2536, A::div(A::scale(s.ad_value(2533), s.v[402]), s.ad_value(2529)), s.v[432]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[429]), 2536);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_square(2538, 2537);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_sqrt_ad(2539, A::div(A::square(s.ad_value(2538)), A::offset(A::square(s.ad_value(2538)), 1.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_sqrt(2540, 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_mul(2541, 2539, 2540);
        }

        s.v[2585] = if (((-p.p824) * s.v[405]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (s.v[2585] != 0.0)) {
            s.store_div_from_scalar_ad(2542, 1.0, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2585] != 0.0))) {
            s.store_powf_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), ((-p.p824) * s.v[405]));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_div_ad(2543, A::mul(s.ad_value(2532), s.ad_value(2542)), A::add(s.ad_value(2532), s.ad_value(2542)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_sqrt_ad(2544, A::scale(A::div(s.ad_value(2536), s.ad_value(2540)), 0.375));
        }

    }

    pub(super) fn stamp_transient_block_38(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_sub_ad_lhs(2545, A::scale(A::mul(s.ad_value(2537), s.ad_value(2540)), 2.0), 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_add_ad(2546, A::sub(A::mul(A::scale(s.ad_value(2537), s.v[429]), s.ad_value(2540)), A::scale(s.ad_value(2539), s.v[429])), A::scale(A::mul(s.ad_value(2536), s.ad_value(2541)), 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_mul_ad_lhs(2547, A::offset(s.ad_value(2545), (-1.0)), 2544);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_square(2508, 2547);
        }

        s.v[2586] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (s.v[2586] != 0.0)) {
            s.store_div_from_scalar_ad(2509, 1.0, A::offset(A::scale(s.ad_value(2547), s.v[366]), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2586] != 0.0))) {
            s.store_div_from_scalar_ad(2509, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2547), s.v[366])));
        }

        s.v[2587] = if (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (s.v[2587] != 0.0)) {
            s.store_exp_ad(2526, A::sub(s.ad_value(2546), s.ad_value(2508)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2587] != 0.0))) {
            let assign56800_ad_e71930: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2526, &assign56800_ad_e71930);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_mul_ad_lhs(2510, A::add(A::add(A::scale(s.ad_value(2509), 0.29214664), A::scale(A::square(s.ad_value(2509)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(2509)), s.ad_value(2509)), s.v[368])), 2526);
        }

        s.v[2588] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (s.v[2588] != 0.0)) {
            s.copy_ad(2548, 2510);
        }

        s.v[2589] = if (s.v[2546] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2588] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_exp(2526, 2546);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2588] != 0.0))) && (!(s.v[2589] != 0.0))) {
            s.store_div_from_scalar_ad(2526, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2588] != 0.0))) {
            s.store_sub_ad_lhs(2548, A::scale(s.ad_value(2526), 2.0), 2510);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_scale_ad(2549, A::div(A::scale(s.ad_value(2548), s.v[429]), s.ad_value(2544)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2584] != 0.0))) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2534), s.ad_value(2549)), s.ad_value(2543)), p.p838);
        }

        s.v[2590] = if (p.p844 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (s.v[2590] != 0.0)) {
            s.store_scalar(2550, 0.0);
        }

        s.v[2591] = if (p.p824 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) && (s.v[2591] != 0.0)) {
            s.store_sqrt_ad(2526, A::scale(A::sub_from_scalar(p.p821, s.ad_value(2524)), s.v[423]));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) && (!(s.v[2591] != 0.0))) {
            s.store_powf_ad(2526, A::scale(A::sub_from_scalar(p.p821, s.ad_value(2524)), s.v[423]), p.p824);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) {
            s.store_scale_ad(2551, A::div(A::scale(A::sub_from_scalar(p.p821, s.ad_value(2524)), s.v[420]), s.ad_value(2526)), s.v[405]);
        }

        s.v[2592] = if (((((-s.v[435]) / s.v[2551])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) && (s.v[2592] != 0.0)) {
            s.store_exp_ad(2526, A::div(A::neg(s.ad_value(435)), s.ad_value(2551)));
        }

        s.v[2593] = if (((-s.v[435]) / s.v[2551]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) && (!(s.v[2592] != 0.0))) && (s.v[2593] != 0.0)) {
            let assign56990_ad_e72270: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(2551))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(2551))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(435)), s.ad_value(2551))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2526, 1e-100, assign56990_ad_e72270);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) && (!(s.v[2592] != 0.0))) && (!(s.v[2593] != 0.0))) {
            let assign57000_ad_e72321: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(435)), s.ad_value(2551)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2526, &assign57000_ad_e72321);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2590] != 0.0))) {
            s.store_scale_ad(2550, A::mul(A::mul(A::mul(s.ad_value(826), s.ad_value(2551)), s.ad_value(2551)), s.ad_value(2526)), p.p844);
        }

        s.v[2594] = if (p.p853 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (s.v[2594] != 0.0)) {
            s.store_scalar(2552, 1.0);
        }

        s.v[2595] = if (s.v[2525] > ((-s.v[438]) * p.p853)) { 1.0 } else { 0.0 };

        s.v[2596] = if (p.p856 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2594] != 0.0))) && (s.v[2595] != 0.0)) && (s.v[2596] != 0.0)) {
            s.store_mul_ad(2526, A::mul(A::mul(A::scale(s.ad_value(2525), s.v[442]), A::scale(s.ad_value(2525), s.v[442])), A::scale(s.ad_value(2525), s.v[442])), A::scale(s.ad_value(2525), s.v[442]));
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2594] != 0.0))) && (s.v[2595] != 0.0)) && (!(s.v[2596] != 0.0))) {
            s.store_powf_ad(2526, A::abs(A::scale(s.ad_value(2525), s.v[442])), p.p856);
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2594] != 0.0))) && (s.v[2595] != 0.0)) {
            s.store_div_from_scalar_ad(2552, 1.0, A::sub_from_scalar(1.0, s.ad_value(2526)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) {
            s.store_offset_ad(2552, A::scale(A::offset(s.ad_value(2525), (s.v[438] * p.p853)), s.v[445]), s.v[439]);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) {
            s.store_mul_ad_lhs(1900, A::scale(A::add(A::add(A::add(s.ad_value(2527), s.ad_value(2528)), s.ad_value(2535)), s.ad_value(2550)), p.p29), 2552);
        }

        s.v[2597] = if (s.v[402] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (s.v[2597] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[399])));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) && (!(s.v[2597] != 0.0))) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[399])), s.v[402]);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2580] != 0.0))) {
            s.store_scale_ad(1906, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2526)), s.v[411]), A::scale(A::sub(s.ad_value(826), s.ad_value(2518)), s.v[414])), p.p30);
        }

        s.v[2598] = if (s.v[641] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2598] != 0.0)) {
            s.store_scalar(1901, 0.0);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2598] != 0.0)) {
            s.store_scalar(1907, 0.0);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_scale(2527, 2517, s.v[382]);
        }

        s.v[2599] = if ((p.p834 == 0.0) && (p.p839 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (s.v[2599] != 0.0)) {
            s.store_scalar(2528, 0.0);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) {
            s.store_sub_from_scalar(2529, s.v[388], 2523);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) {
            s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));
        }

        s.v[2600] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) && (s.v[2600] != 0.0)) {
            s.store_scalar(2531, 0.0);
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) && (!(s.v[2600] != 0.0))) {
            s.store_scale_ad(2531, A::add(A::div(A::mul(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530))), A::sub_from_scalar(1.0, s.ad_value(2530))), s.ad_value(2530)), (1.0 - (2.0 * p.p825)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) {
            s.store_add(2532, 2530, 2531);
        }

        s.v[2601] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) && (s.v[2601] != 0.0)) {
            s.store_sqrt_ad(2526, A::scale(s.ad_value(2529), s.v[424]));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) && (!(s.v[2601] != 0.0))) {
            s.store_powf_ad(2526, A::scale(s.ad_value(2529), s.v[424]), p.p825);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) {
            s.store_scale(2533, 2526, s.v[418]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) {
            s.store_scale_ad(2534, A::mul(A::offset(s.ad_value(2520), (-1.0)), s.ad_value(2533)), s.v[379]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) {
            s.store_scaled_mul(2528, 2534, 2532, p.p834);
        }

        s.v[2602] = if (p.p839 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (s.v[2602] != 0.0)) {
            s.store_scalar(2535, 0.0);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_scale_ad(2536, A::div(A::scale(s.ad_value(2533), s.v[403]), s.ad_value(2529)), s.v[433]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[430]), 2536);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_square(2538, 2537);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_sqrt_ad(2539, A::div(A::square(s.ad_value(2538)), A::offset(A::square(s.ad_value(2538)), 1.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_sqrt(2540, 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_mul(2541, 2539, 2540);
        }

        s.v[2603] = if (((-p.p825) * s.v[406]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (s.v[2603] != 0.0)) {
            s.store_div_from_scalar_ad(2542, 1.0, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (!(s.v[2603] != 0.0))) {
            s.store_powf_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), ((-p.p825) * s.v[406]));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_div_ad(2543, A::mul(s.ad_value(2532), s.ad_value(2542)), A::add(s.ad_value(2532), s.ad_value(2542)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_sqrt_ad(2544, A::scale(A::div(s.ad_value(2536), s.ad_value(2540)), 0.375));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_sub_ad_lhs(2545, A::scale(A::mul(s.ad_value(2537), s.ad_value(2540)), 2.0), 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_add_ad(2546, A::sub(A::mul(A::scale(s.ad_value(2537), s.v[430]), s.ad_value(2540)), A::scale(s.ad_value(2539), s.v[430])), A::scale(A::mul(s.ad_value(2536), s.ad_value(2541)), 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_mul_ad_lhs(2547, A::offset(s.ad_value(2545), (-1.0)), 2544);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_square(2508, 2547);
        }

        s.v[2604] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (s.v[2604] != 0.0)) {
            s.store_div_from_scalar_ad(2509, 1.0, A::offset(A::scale(s.ad_value(2547), s.v[366]), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (!(s.v[2604] != 0.0))) {
            s.store_div_from_scalar_ad(2509, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2547), s.v[366])));
        }

        s.v[2605] = if (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (s.v[2605] != 0.0)) {
            s.store_exp_ad(2526, A::sub(s.ad_value(2546), s.ad_value(2508)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (!(s.v[2605] != 0.0))) {
            let assign57550_ad_e73196: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2526, &assign57550_ad_e73196);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_mul_ad_lhs(2510, A::add(A::add(A::scale(s.ad_value(2509), 0.29214664), A::scale(A::square(s.ad_value(2509)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(2509)), s.ad_value(2509)), s.v[368])), 2526);
        }

        s.v[2606] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (s.v[2606] != 0.0)) {
            s.copy_ad(2548, 2510);
        }

        s.v[2607] = if (s.v[2546] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (!(s.v[2606] != 0.0))) && (s.v[2607] != 0.0)) {
            s.store_exp(2526, 2546);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (!(s.v[2606] != 0.0))) && (!(s.v[2607] != 0.0))) {
            s.store_div_from_scalar_ad(2526, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (!(s.v[2606] != 0.0))) {
            s.store_sub_ad_lhs(2548, A::scale(s.ad_value(2526), 2.0), 2510);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_scale_ad(2549, A::div(A::scale(s.ad_value(2548), s.v[430]), s.ad_value(2544)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2534), s.ad_value(2549)), s.ad_value(2543)), p.p839);
        }

        s.v[2608] = if (p.p845 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (s.v[2608] != 0.0)) {
            s.store_scalar(2550, 0.0);
        }

        s.v[2609] = if (p.p825 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) && (s.v[2609] != 0.0)) {
            s.store_sqrt_ad(2526, A::scale(A::sub_from_scalar(p.p822, s.ad_value(2524)), s.v[424]));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) && (!(s.v[2609] != 0.0))) {
            s.store_powf_ad(2526, A::scale(A::sub_from_scalar(p.p822, s.ad_value(2524)), s.v[424]), p.p825);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) {
            s.store_scale_ad(2551, A::div(A::scale(A::sub_from_scalar(p.p822, s.ad_value(2524)), s.v[421]), s.ad_value(2526)), s.v[406]);
        }

        s.v[2610] = if (((((-s.v[436]) / s.v[2551])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) && (s.v[2610] != 0.0)) {
            s.store_exp_ad(2526, A::div(A::neg(s.ad_value(436)), s.ad_value(2551)));
        }

        s.v[2611] = if (((-s.v[436]) / s.v[2551]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) && (!(s.v[2610] != 0.0))) && (s.v[2611] != 0.0)) {
            let assign57740_ad_e73536: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(2551))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(2551))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(436)), s.ad_value(2551))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2526, 1e-100, assign57740_ad_e73536);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) && (!(s.v[2610] != 0.0))) && (!(s.v[2611] != 0.0))) {
            let assign57750_ad_e73587: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(436)), s.ad_value(2551)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2526, &assign57750_ad_e73587);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2608] != 0.0))) {
            s.store_scale_ad(2550, A::mul(A::mul(A::mul(s.ad_value(826), s.ad_value(2551)), s.ad_value(2551)), s.ad_value(2526)), p.p845);
        }

        s.v[2612] = if (p.p854 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (s.v[2612] != 0.0)) {
            s.store_scalar(2552, 1.0);
        }

        s.v[2613] = if (s.v[2525] > ((-s.v[438]) * p.p854)) { 1.0 } else { 0.0 };

        s.v[2614] = if (p.p857 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2612] != 0.0))) && (s.v[2613] != 0.0)) && (s.v[2614] != 0.0)) {
            s.store_mul_ad(2526, A::mul(A::mul(A::scale(s.ad_value(2525), s.v[443]), A::scale(s.ad_value(2525), s.v[443])), A::scale(s.ad_value(2525), s.v[443])), A::scale(s.ad_value(2525), s.v[443]));
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2612] != 0.0))) && (s.v[2613] != 0.0)) && (!(s.v[2614] != 0.0))) {
            s.store_powf_ad(2526, A::abs(A::scale(s.ad_value(2525), s.v[443])), p.p857);
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2612] != 0.0))) && (s.v[2613] != 0.0)) {
            s.store_div_from_scalar_ad(2552, 1.0, A::sub_from_scalar(1.0, s.ad_value(2526)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) {
            s.store_offset_ad(2552, A::scale(A::offset(s.ad_value(2525), (s.v[438] * p.p854)), s.v[446]), s.v[440]);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_mul_ad_lhs(1901, A::scale(A::add(A::add(A::add(s.ad_value(2527), s.ad_value(2528)), s.ad_value(2535)), s.ad_value(2550)), p.p29), 2552);
        }

        s.v[2615] = if (s.v[403] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (s.v[2615] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[400])));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2615] != 0.0))) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[400])), s.v[403]);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_scale_ad(1907, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2526)), s.v[412]), A::scale(A::sub(s.ad_value(826), s.ad_value(2518)), s.v[415])), p.p30);
        }

        s.v[2616] = if (s.v[642] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2616] != 0.0)) {
            s.store_scalar(1902, 0.0);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2616] != 0.0)) {
            s.store_scalar(1908, 0.0);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_scale(2527, 2517, s.v[383]);
        }

        s.v[2617] = if ((p.p835 == 0.0) && (p.p840 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2617] != 0.0)) {
            s.store_scalar(2528, 0.0);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) {
            s.store_sub_from_scalar(2529, s.v[389], 2523);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) {
            s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));
        }

        s.v[2618] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) && (s.v[2618] != 0.0)) {
            s.store_scalar(2531, 0.0);
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) && (!(s.v[2618] != 0.0))) {
            s.store_scale_ad(2531, A::add(A::div(A::mul(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530))), A::sub_from_scalar(1.0, s.ad_value(2530))), s.ad_value(2530)), (1.0 - (2.0 * p.p826)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) {
            s.store_add(2532, 2530, 2531);
        }

        s.v[2619] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) && (s.v[2619] != 0.0)) {
            s.store_sqrt_ad(2526, A::scale(s.ad_value(2529), s.v[425]));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) && (!(s.v[2619] != 0.0))) {
            s.store_powf_ad(2526, A::scale(s.ad_value(2529), s.v[425]), p.p826);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) {
            s.store_scale(2533, 2526, s.v[419]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) {
            s.store_scale_ad(2534, A::mul(A::offset(s.ad_value(2520), (-1.0)), s.ad_value(2533)), s.v[380]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) {
            s.store_scaled_mul(2528, 2534, 2532, p.p835);
        }

        s.v[2620] = if (p.p840 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2620] != 0.0)) {
            s.store_scalar(2535, 0.0);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_scale_ad(2536, A::div(A::scale(s.ad_value(2533), s.v[404]), s.ad_value(2529)), s.v[434]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_div_from_scalar(2537, (0.666666666666667 * s.v[431]), 2536);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_square(2538, 2537);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_sqrt_ad(2539, A::div(A::square(s.ad_value(2538)), A::offset(A::square(s.ad_value(2538)), 1.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_sqrt(2540, 2539);
        }

    }

    pub(super) fn stamp_transient_block_39(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_mul(2541, 2539, 2540);
        }

        s.v[2621] = if (((-p.p826) * s.v[407]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (s.v[2621] != 0.0)) {
            s.store_div_from_scalar_ad(2542, 1.0, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (!(s.v[2621] != 0.0))) {
            s.store_powf_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), ((-p.p826) * s.v[407]));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_div_ad(2543, A::mul(s.ad_value(2532), s.ad_value(2542)), A::add(s.ad_value(2532), s.ad_value(2542)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_sqrt_ad(2544, A::scale(A::div(s.ad_value(2536), s.ad_value(2540)), 0.375));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_sub_ad_lhs(2545, A::scale(A::mul(s.ad_value(2537), s.ad_value(2540)), 2.0), 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_add_ad(2546, A::sub(A::mul(A::scale(s.ad_value(2537), s.v[431]), s.ad_value(2540)), A::scale(s.ad_value(2539), s.v[431])), A::scale(A::mul(s.ad_value(2536), s.ad_value(2541)), 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_mul_ad_lhs(2547, A::offset(s.ad_value(2545), (-1.0)), 2544);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_square(2508, 2547);
        }

        s.v[2622] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (s.v[2622] != 0.0)) {
            s.store_div_from_scalar_ad(2509, 1.0, A::offset(A::scale(s.ad_value(2547), s.v[366]), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (!(s.v[2622] != 0.0))) {
            s.store_div_from_scalar_ad(2509, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2547), s.v[366])));
        }

        s.v[2623] = if (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (s.v[2623] != 0.0)) {
            s.store_exp_ad(2526, A::sub(s.ad_value(2546), s.ad_value(2508)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (!(s.v[2623] != 0.0))) {
            let assign58300_ad_e74462: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2526, &assign58300_ad_e74462);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_mul_ad_lhs(2510, A::add(A::add(A::scale(s.ad_value(2509), 0.29214664), A::scale(A::square(s.ad_value(2509)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(2509)), s.ad_value(2509)), s.v[368])), 2526);
        }

        s.v[2624] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (s.v[2624] != 0.0)) {
            s.copy_ad(2548, 2510);
        }

        s.v[2625] = if (s.v[2546] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (!(s.v[2624] != 0.0))) && (s.v[2625] != 0.0)) {
            s.store_exp(2526, 2546);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (!(s.v[2624] != 0.0))) && (!(s.v[2625] != 0.0))) {
            s.store_div_from_scalar_ad(2526, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (!(s.v[2624] != 0.0))) {
            s.store_sub_ad_lhs(2548, A::scale(s.ad_value(2526), 2.0), 2510);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_scale_ad(2549, A::div(A::scale(s.ad_value(2548), s.v[431]), s.ad_value(2544)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2534), s.ad_value(2549)), s.ad_value(2543)), p.p840);
        }

        s.v[2626] = if (p.p846 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2626] != 0.0)) {
            s.store_scalar(2550, 0.0);
        }

        s.v[2627] = if (p.p826 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) && (s.v[2627] != 0.0)) {
            s.store_sqrt_ad(2526, A::scale(A::sub_from_scalar(p.p823, s.ad_value(2524)), s.v[425]));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) && (!(s.v[2627] != 0.0))) {
            s.store_powf_ad(2526, A::scale(A::sub_from_scalar(p.p823, s.ad_value(2524)), s.v[425]), p.p826);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) {
            s.store_scale_ad(2551, A::div(A::scale(A::sub_from_scalar(p.p823, s.ad_value(2524)), s.v[422]), s.ad_value(2526)), s.v[407]);
        }

        s.v[2628] = if (((((-s.v[437]) / s.v[2551])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) && (s.v[2628] != 0.0)) {
            s.store_exp_ad(2526, A::div(A::neg(s.ad_value(437)), s.ad_value(2551)));
        }

        s.v[2629] = if (((-s.v[437]) / s.v[2551]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) && (!(s.v[2628] != 0.0))) && (s.v[2629] != 0.0)) {
            let assign58490_ad_e74802: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(2551))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(2551))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(437)), s.ad_value(2551))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2526, 1e-100, assign58490_ad_e74802);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) && (!(s.v[2628] != 0.0))) && (!(s.v[2629] != 0.0))) {
            let assign58500_ad_e74853: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(437)), s.ad_value(2551)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2526, &assign58500_ad_e74853);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2626] != 0.0))) {
            s.store_scale_ad(2550, A::mul(A::mul(A::mul(s.ad_value(826), s.ad_value(2551)), s.ad_value(2551)), s.ad_value(2526)), p.p846);
        }

        s.v[2630] = if (s.v[634] > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2630] != 0.0)) {
            s.store_scalar(2552, 1.0);
        }

        s.v[2631] = if (s.v[2525] > ((-s.v[438]) * s.v[634])) { 1.0 } else { 0.0 };

        s.v[2632] = if (p.p858 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2631] != 0.0)) && (s.v[2632] != 0.0)) {
            s.store_mul_ad(2526, A::mul(A::mul(A::mul(s.ad_value(2525), s.ad_value(444)), A::mul(s.ad_value(2525), s.ad_value(444))), A::mul(s.ad_value(2525), s.ad_value(444))), A::mul(s.ad_value(2525), s.ad_value(444)));
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2631] != 0.0)) && (!(s.v[2632] != 0.0))) {
            s.store_powf_ad(2526, A::abs(A::mul(s.ad_value(2525), s.ad_value(444))), p.p858);
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2631] != 0.0)) {
            s.store_div_from_scalar_ad(2552, 1.0, A::sub_from_scalar(1.0, s.ad_value(2526)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) {
            s.store_offset_ad(2552, A::mul(A::add(s.ad_value(2525), A::scale(s.ad_value(634), s.v[438])), s.ad_value(447)), s.v[441]);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_mul_ad_lhs(1902, A::scale(A::add(A::add(A::add(s.ad_value(2527), s.ad_value(2528)), s.ad_value(2535)), s.ad_value(2550)), p.p29), 2552);
        }

        s.v[2633] = if (s.v[467] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            let assign58620_ad_e75078: A = {
                if (s.v[826] < p.p863) {
                    {
                        if (((s.v[826] - p.p863) / p.p864) < (-37.0)) {
                            A::constant(p.p863)
                        } else {
                            A::offset(A::scale(A::ln(A::offset(A::exp(A::scale(A::offset(s.ad_value(826), (-p.p863)), 1.0 / (p.p864))), 1.0)), p.p864), p.p863)
                        }
                    }
                } else {
                    {
                        if (((s.v[826] - p.p863) / p.p864) > 37.0) {
                            s.ad_value(826)
                        } else {
                            A::add(s.ad_value(826), A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(p.p863, s.ad_value(826)), 1.0 / (p.p864))), 1.0)), p.p864))
                        }
                    }
                }
            };
            s.store_ad(2553, &assign58620_ad_e75078);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(651), 4.0), 651);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_div(2512, 651, 652);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_add_ad_rhs(2513, 2553, A::mul(s.ad_value(651), s.ad_value(2512)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_add(2514, 652, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_sub(2515, 652, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_scale_ad(2554, A::div(A::mul(s.ad_value(2553), s.ad_value(652)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2634] = if (s.v[404] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) && (s.v[2634] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2554), s.v[401])));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) && (!(s.v[2634] != 0.0))) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2554), s.v[401])), s.v[404]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_scale_ad(1908, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2526)), s.v[413]), A::scale(A::sub(s.ad_value(2553), s.ad_value(2554)), s.v[416])), p.p30);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_sub_ad_lhs(2553, A::offset(s.ad_value(826), p.p863), 2553);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(651), 4.0), 651);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_div(2512, 651, 652);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_add_ad_rhs(2513, 2553, A::mul(s.ad_value(651), s.ad_value(2512)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_add(2514, 652, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_sub(2515, 652, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_scale_ad(2554, A::div(A::mul(s.ad_value(2553), s.ad_value(652)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2635] = if (s.v[461] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) && (s.v[2635] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(460))));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) && (!(s.v[2635] != 0.0))) {
            s.store_ad(2526, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(460))), s.ad_value(461)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_scale_ad(466, A::add(A::mul(s.ad_value(464), A::sub_from_scalar(1.0, s.ad_value(2526))), A::mul(s.ad_value(465), A::sub(s.ad_value(2553), s.ad_value(2554)))), p.p30);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_add(1908, 1908, 466);
        }

        s.v[2636] = if (s.v[404] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2633] != 0.0))) && (s.v[2636] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[401])));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2633] != 0.0))) && (!(s.v[2636] != 0.0))) {
            s.store_powf_ad(2526, A::sub_from_scalar(1.0, A::scale(s.ad_value(2518), s.v[401])), s.v[404]);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2633] != 0.0))) {
            s.store_scale_ad(1908, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2526)), s.v[413]), A::scale(A::sub(s.ad_value(826), s.ad_value(2518)), s.v[416])), p.p30);
        }

        if ((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) {
            s.store_add_ad(842, A::add(A::mul(s.ad_value(640), s.ad_value(1900)), A::mul(s.ad_value(641), s.ad_value(1901))), A::mul(s.ad_value(642), s.ad_value(1902)));
        }

        s.v[2637] = if (s.v[630] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2637] != 0.0)) {
            s.store_mul_ad_rhs(637, 630, A::sub(A::pow(A::scale(A::add(A::add(s.ad_value(819), s.ad_value(821)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001)))), 0.5), s.ad_value(631)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(631))));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2637] != 0.0)) {
            s.store_add(635, 536, 637);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2637] != 0.0)) {
            s.store_div_from_scalar(610, 1.0, 635);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2637] != 0.0)) {
            s.store_div_ad_rhs(613, 613, A::offset(A::div(s.ad_value(637), s.ad_value(536)), 1.0));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2637] != 0.0))) {
            s.copy_ad(635, 536);
        }

        s.v[2638] = if (s.v[632] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2638] != 0.0)) {
            s.store_mul_ad_rhs(639, 632, A::sub(A::pow(A::scale(A::add(A::add(s.ad_value(819), s.ad_value(821)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(819), s.ad_value(821)), A::add(s.ad_value(819), s.ad_value(821))), (0.001 * 0.001)))), 0.5), s.ad_value(633)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(633))));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2638] != 0.0)) {
            s.store_mul_ad_rhs(604, 604, A::offset(s.ad_value(639), 1.0));
        }

        if ((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) {
            s.store_scalar(2524, 0.0);
        }

        if ((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) {
            s.store_scalar(2521, 0.0);
        }

        s.v[2639] = if !(((s.v[667] == 0.0) && (s.v[668] == 0.0)) && (s.v[669] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(678), 4.0), 678);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_div(2512, 678, 679);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_add_ad_rhs(2513, 827, A::mul(s.ad_value(678), s.ad_value(2512)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_add(2514, 679, 2513);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_sub(2515, 679, 2513);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_scale_ad(2518, A::div(A::mul(s.ad_value(827), s.ad_value(679)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2640] = if (s.v[827] < s.v[675]) { 1.0 } else { 0.0 };

        s.v[2641] = if (((((-0.5) * (s.v[827] * s.v[365]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (s.v[2640] != 0.0)) && (s.v[2641] != 0.0)) {
            s.store_exp_ad(2519, A::scale(s.ad_value(827), (s.v[365] * (-0.5))));
        }

        s.v[2642] = if (((-0.5) * (s.v[827] * s.v[365])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (s.v[2640] != 0.0)) && (!(s.v[2641] != 0.0))) && (s.v[2642] != 0.0)) {
            let assign59150_ad_e75912: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(827), (s.v[365] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(827), (s.v[365] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(827), (s.v[365] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2519, &assign59150_ad_e75912);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (s.v[2640] != 0.0)) && (!(s.v[2641] != 0.0))) && (!(s.v[2642] != 0.0))) {
            s.store_scale_ad(2519, A::offset(A::mul(A::offset(A::scale(s.ad_value(827), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(827), (s.v[365] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(827), (s.v[365] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (s.v[2640] != 0.0)) {
            s.store_div_from_scalar(2520, 1.0, 2519);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (s.v[2640] != 0.0)) {
            s.store_square(2517, 2520);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (!(s.v[2640] != 0.0))) {
            s.store_mul_ad_lhs(2517, A::offset(A::scale(A::sub(s.ad_value(827), s.ad_value(675)), s.v[365]), 1.0), 676);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (!(s.v[2640] != 0.0))) {
            s.store_sqrt(2520, 2517);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (!(s.v[2640] != 0.0))) {
            s.store_div_from_scalar(2519, 1.0, 2520);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_offset(2517, 2517, (-1.0));
        }

        s.v[2643] = if (s.v[827] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (s.v[2643] != 0.0)) {
            s.store_scale_ad(2521, A::ln(A::add(A::offset(s.ad_value(2519), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2519), 1.0), A::offset(s.ad_value(2519), 3.0))))), (s.v[364] * 2.0));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) && (!(s.v[2643] != 0.0))) {
            s.store_sub_ad_lhs(2521, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2520), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2520), 1.0), A::offset(A::scale(s.ad_value(2520), 3.0), 1.0))))), (s.v[364] * 2.0)), 827);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_sub(2522, 677, 2521);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_scale_ad(2523, A::sub(A::add(s.ad_value(827), s.ad_value(2522)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(827), s.ad_value(2522)), A::sub(s.ad_value(827), s.ad_value(2522))), ((4.0 * s.v[364]) * s.v[364])))), 0.5);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_scale_ad(2524, A::sub(A::add(s.ad_value(827), s.ad_value(680)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(827), s.ad_value(680)), A::sub(s.ad_value(827), s.ad_value(680))), ((4.0 * s.v[362]) * s.v[362])))), 0.5);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_scale_ad(2525, A::sub(s.ad_value(827), A::sqrt(A::offset(A::mul(s.ad_value(827), s.ad_value(827)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[2644] = if (s.v[667] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2644] != 0.0)) {
            s.store_scalar(1903, 0.0);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2644] != 0.0)) {
            s.store_scalar(1909, 0.0);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) {
            s.store_mul(2527, 557, 2517);
        }

        s.v[2645] = if ((s.v[516] == 0.0) && (s.v[519] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (s.v[2645] != 0.0)) {
            s.store_scalar(2528, 0.0);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) {
            s.store_sub(2529, 563, 2523);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) {
            s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));
        }

        s.v[2646] = if (s.v[505] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) && (s.v[2646] != 0.0)) {
            s.store_scalar(2531, 0.0);
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) && (!(s.v[2646] != 0.0))) {
            s.store_mul_ad(2531, A::add(A::div(A::mul(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530))), A::sub_from_scalar(1.0, s.ad_value(2530))), s.ad_value(2530)), A::sub_from_scalar(1.0, A::scale(s.ad_value(505), 2.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) {
            s.store_add(2532, 2530, 2531);
        }

        s.v[2647] = if (s.v[505] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_sqrt_ad(2526, A::mul(s.ad_value(2529), s.ad_value(590)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) && (!(s.v[2647] != 0.0))) {
            s.store_ad(2526, &A::pow(A::mul(s.ad_value(2529), s.ad_value(590)), s.ad_value(505)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) {
            s.store_mul(2533, 584, 2526);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) {
            s.store_mul_ad_rhs(2534, 554, A::mul(A::offset(s.ad_value(2520), (-1.0)), s.ad_value(2533)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) {
            s.store_mul_ad_rhs(2528, 516, A::mul(s.ad_value(2534), s.ad_value(2532)));
        }

        s.v[2648] = if (s.v[519] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (s.v[2648] != 0.0)) {
            s.store_scalar(2535, 0.0);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_mul_ad_rhs(2536, 599, A::div(A::mul(s.ad_value(2533), s.ad_value(569)), s.ad_value(2529)));
        }

    }

    pub(super) fn stamp_transient_block_40(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_div_ad_lhs(2537, A::scale(s.ad_value(596), 0.666666666666667), 2536);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_square(2538, 2537);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_sqrt_ad(2539, A::div(A::square(s.ad_value(2538)), A::offset(A::square(s.ad_value(2538)), 1.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_sqrt(2540, 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_mul(2541, 2539, 2540);
        }

        s.v[2649] = if (((-s.v[505]) * s.v[572]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (s.v[2649] != 0.0)) {
            s.store_div_from_scalar_ad(2542, 1.0, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (!(s.v[2649] != 0.0))) {
            s.store_pow_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), A::mul(A::neg(s.ad_value(505)), s.ad_value(572)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_div_ad(2543, A::mul(s.ad_value(2532), s.ad_value(2542)), A::add(s.ad_value(2532), s.ad_value(2542)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_sqrt_ad(2544, A::scale(A::div(s.ad_value(2536), s.ad_value(2540)), 0.375));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_sub_ad_lhs(2545, A::scale(A::mul(s.ad_value(2537), s.ad_value(2540)), 2.0), 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_add_ad(2546, A::sub(A::mul(A::mul(s.ad_value(596), s.ad_value(2537)), s.ad_value(2540)), A::mul(s.ad_value(596), s.ad_value(2539))), A::scale(A::mul(s.ad_value(2536), s.ad_value(2541)), 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_mul_ad_lhs(2547, A::offset(s.ad_value(2545), (-1.0)), 2544);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_square(2508, 2547);
        }

        s.v[2650] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (s.v[2650] != 0.0)) {
            s.store_div_from_scalar_ad(2509, 1.0, A::offset(A::scale(s.ad_value(2547), s.v[366]), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (!(s.v[2650] != 0.0))) {
            s.store_div_from_scalar_ad(2509, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2547), s.v[366])));
        }

        s.v[2651] = if (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (s.v[2651] != 0.0)) {
            s.store_exp_ad(2526, A::sub(s.ad_value(2546), s.ad_value(2508)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (!(s.v[2651] != 0.0))) {
            let assign59700_ad_e76862: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2526, &assign59700_ad_e76862);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_mul_ad_lhs(2510, A::add(A::add(A::scale(s.ad_value(2509), 0.29214664), A::scale(A::square(s.ad_value(2509)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(2509)), s.ad_value(2509)), s.v[368])), 2526);
        }

        s.v[2652] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (s.v[2652] != 0.0)) {
            s.copy_ad(2548, 2510);
        }

        s.v[2653] = if (s.v[2546] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (!(s.v[2652] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_exp(2526, 2546);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (!(s.v[2652] != 0.0))) && (!(s.v[2653] != 0.0))) {
            s.store_div_from_scalar_ad(2526, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) && (!(s.v[2652] != 0.0))) {
            s.store_sub_ad_lhs(2548, A::scale(s.ad_value(2526), 2.0), 2510);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_scale_ad(2549, A::div(A::mul(s.ad_value(596), s.ad_value(2548)), s.ad_value(2544)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2648] != 0.0))) {
            s.store_mul_ad_rhs(2535, 519, A::mul(A::mul(s.ad_value(2534), s.ad_value(2549)), s.ad_value(2543)));
        }

        s.v[2654] = if (s.v[525] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (s.v[2654] != 0.0)) {
            s.store_scalar(2550, 0.0);
        }

        s.v[2655] = if (s.v[505] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) && (s.v[2655] != 0.0)) {
            s.store_sqrt_ad(2526, A::mul(A::sub(s.ad_value(502), s.ad_value(2524)), s.ad_value(590)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) && (!(s.v[2655] != 0.0))) {
            s.store_ad(2526, &A::pow(A::mul(A::sub(s.ad_value(502), s.ad_value(2524)), s.ad_value(590)), s.ad_value(505)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) {
            s.store_mul_ad_rhs(2551, 572, A::div(A::mul(A::sub(s.ad_value(502), s.ad_value(2524)), s.ad_value(587)), s.ad_value(2526)));
        }

        s.v[2656] = if (((((-s.v[602]) / s.v[2551])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) && (s.v[2656] != 0.0)) {
            s.store_exp_ad(2526, A::div(A::neg(s.ad_value(602)), s.ad_value(2551)));
        }

        s.v[2657] = if (((-s.v[602]) / s.v[2551]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) && (!(s.v[2656] != 0.0))) && (s.v[2657] != 0.0)) {
            let assign59890_ad_e77202: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(602)), s.ad_value(2551))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(602)), s.ad_value(2551))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(602)), s.ad_value(2551))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2526, 1e-100, assign59890_ad_e77202);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) && (!(s.v[2656] != 0.0))) && (!(s.v[2657] != 0.0))) {
            let assign59900_ad_e77253: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(602)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(602)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(602)), s.ad_value(2551)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2526, &assign59900_ad_e77253);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2654] != 0.0))) {
            s.store_mul_ad_rhs(2550, 525, A::mul(A::mul(A::mul(s.ad_value(827), s.ad_value(2551)), s.ad_value(2551)), s.ad_value(2526)));
        }

        s.v[2658] = if (s.v[534] > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (s.v[2658] != 0.0)) {
            s.store_scalar(2552, 1.0);
        }

        s.v[2659] = if (s.v[2525] > ((-s.v[438]) * s.v[534])) { 1.0 } else { 0.0 };

        s.v[2660] = if (s.v[537] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2658] != 0.0))) && (s.v[2659] != 0.0)) && (s.v[2660] != 0.0)) {
            s.store_mul_ad(2526, A::mul(A::mul(A::mul(s.ad_value(2525), s.ad_value(608)), A::mul(s.ad_value(2525), s.ad_value(608))), A::mul(s.ad_value(2525), s.ad_value(608))), A::mul(s.ad_value(2525), s.ad_value(608)));
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2658] != 0.0))) && (s.v[2659] != 0.0)) && (!(s.v[2660] != 0.0))) {
            s.store_ad(2526, &A::pow(A::abs(A::mul(s.ad_value(2525), s.ad_value(608))), s.ad_value(537)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2658] != 0.0))) && (s.v[2659] != 0.0)) {
            s.store_div_from_scalar_ad(2552, 1.0, A::sub_from_scalar(1.0, s.ad_value(2526)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) {
            s.store_add_ad_rhs(2552, 605, A::mul(A::add(s.ad_value(2525), A::scale(s.ad_value(534), s.v[438])), s.ad_value(611)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) {
            s.store_mul_ad_lhs(1903, A::scale(A::add(A::add(A::add(s.ad_value(2527), s.ad_value(2528)), s.ad_value(2535)), s.ad_value(2550)), p.p29), 2552);
        }

        s.v[2661] = if (s.v[569] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (s.v[2661] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(566))));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2661] != 0.0))) {
            s.store_ad(2526, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(566))), s.ad_value(569)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2644] != 0.0))) {
            s.store_scale_ad(1909, A::add(A::mul(s.ad_value(578), A::sub_from_scalar(1.0, s.ad_value(2526))), A::mul(s.ad_value(581), A::sub(s.ad_value(827), s.ad_value(2518)))), p.p30);
        }

        s.v[2662] = if (s.v[668] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2662] != 0.0)) {
            s.store_scalar(1904, 0.0);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2662] != 0.0)) {
            s.store_scalar(1910, 0.0);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_mul(2527, 558, 2517);
        }

        s.v[2663] = if ((s.v[517] == 0.0) && (s.v[520] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (s.v[2663] != 0.0)) {
            s.store_scalar(2528, 0.0);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) {
            s.store_sub(2529, 564, 2523);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) {
            s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));
        }

        s.v[2664] = if (s.v[506] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) && (s.v[2664] != 0.0)) {
            s.store_scalar(2531, 0.0);
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) && (!(s.v[2664] != 0.0))) {
            s.store_mul_ad(2531, A::add(A::div(A::mul(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530))), A::sub_from_scalar(1.0, s.ad_value(2530))), s.ad_value(2530)), A::sub_from_scalar(1.0, A::scale(s.ad_value(506), 2.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) {
            s.store_add(2532, 2530, 2531);
        }

        s.v[2665] = if (s.v[506] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) && (s.v[2665] != 0.0)) {
            s.store_sqrt_ad(2526, A::mul(s.ad_value(2529), s.ad_value(591)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) && (!(s.v[2665] != 0.0))) {
            s.store_ad(2526, &A::pow(A::mul(s.ad_value(2529), s.ad_value(591)), s.ad_value(506)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) {
            s.store_mul(2533, 585, 2526);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) {
            s.store_mul_ad_rhs(2534, 555, A::mul(A::offset(s.ad_value(2520), (-1.0)), s.ad_value(2533)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) {
            s.store_mul_ad_rhs(2528, 517, A::mul(s.ad_value(2534), s.ad_value(2532)));
        }

        s.v[2666] = if (s.v[520] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (s.v[2666] != 0.0)) {
            s.store_scalar(2535, 0.0);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_mul_ad_rhs(2536, 600, A::div(A::mul(s.ad_value(2533), s.ad_value(570)), s.ad_value(2529)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_div_ad_lhs(2537, A::scale(s.ad_value(597), 0.666666666666667), 2536);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_square(2538, 2537);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_sqrt_ad(2539, A::div(A::square(s.ad_value(2538)), A::offset(A::square(s.ad_value(2538)), 1.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_sqrt(2540, 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_mul(2541, 2539, 2540);
        }

        s.v[2667] = if (((-s.v[506]) * s.v[573]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (s.v[2667] != 0.0)) {
            s.store_div_from_scalar_ad(2542, 1.0, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (!(s.v[2667] != 0.0))) {
            s.store_pow_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), A::mul(A::neg(s.ad_value(506)), s.ad_value(573)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_div_ad(2543, A::mul(s.ad_value(2532), s.ad_value(2542)), A::add(s.ad_value(2532), s.ad_value(2542)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_sqrt_ad(2544, A::scale(A::div(s.ad_value(2536), s.ad_value(2540)), 0.375));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_sub_ad_lhs(2545, A::scale(A::mul(s.ad_value(2537), s.ad_value(2540)), 2.0), 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_add_ad(2546, A::sub(A::mul(A::mul(s.ad_value(597), s.ad_value(2537)), s.ad_value(2540)), A::mul(s.ad_value(597), s.ad_value(2539))), A::scale(A::mul(s.ad_value(2536), s.ad_value(2541)), 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_mul_ad_lhs(2547, A::offset(s.ad_value(2545), (-1.0)), 2544);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_square(2508, 2547);
        }

        s.v[2668] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (s.v[2668] != 0.0)) {
            s.store_div_from_scalar_ad(2509, 1.0, A::offset(A::scale(s.ad_value(2547), s.v[366]), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (!(s.v[2668] != 0.0))) {
            s.store_div_from_scalar_ad(2509, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2547), s.v[366])));
        }

        s.v[2669] = if (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (s.v[2669] != 0.0)) {
            s.store_exp_ad(2526, A::sub(s.ad_value(2546), s.ad_value(2508)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (!(s.v[2669] != 0.0))) {
            let assign60450_ad_e78128: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2526, &assign60450_ad_e78128);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_mul_ad_lhs(2510, A::add(A::add(A::scale(s.ad_value(2509), 0.29214664), A::scale(A::square(s.ad_value(2509)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(2509)), s.ad_value(2509)), s.v[368])), 2526);
        }

        s.v[2670] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (s.v[2670] != 0.0)) {
            s.copy_ad(2548, 2510);
        }

        s.v[2671] = if (s.v[2546] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (!(s.v[2670] != 0.0))) && (s.v[2671] != 0.0)) {
            s.store_exp(2526, 2546);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (!(s.v[2670] != 0.0))) && (!(s.v[2671] != 0.0))) {
            s.store_div_from_scalar_ad(2526, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (!(s.v[2670] != 0.0))) {
            s.store_sub_ad_lhs(2548, A::scale(s.ad_value(2526), 2.0), 2510);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_scale_ad(2549, A::div(A::mul(s.ad_value(597), s.ad_value(2548)), s.ad_value(2544)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_mul_ad_rhs(2535, 520, A::mul(A::mul(s.ad_value(2534), s.ad_value(2549)), s.ad_value(2543)));
        }

        s.v[2672] = if (s.v[526] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (s.v[2672] != 0.0)) {
            s.store_scalar(2550, 0.0);
        }

        s.v[2673] = if (s.v[506] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) && (s.v[2673] != 0.0)) {
            s.store_sqrt_ad(2526, A::mul(A::sub(s.ad_value(503), s.ad_value(2524)), s.ad_value(591)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) && (!(s.v[2673] != 0.0))) {
            s.store_ad(2526, &A::pow(A::mul(A::sub(s.ad_value(503), s.ad_value(2524)), s.ad_value(591)), s.ad_value(506)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) {
            s.store_mul_ad_rhs(2551, 573, A::div(A::mul(A::sub(s.ad_value(503), s.ad_value(2524)), s.ad_value(588)), s.ad_value(2526)));
        }

        s.v[2674] = if (((((-s.v[603]) / s.v[2551])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) && (s.v[2674] != 0.0)) {
            s.store_exp_ad(2526, A::div(A::neg(s.ad_value(603)), s.ad_value(2551)));
        }

        s.v[2675] = if (((-s.v[603]) / s.v[2551]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) && (!(s.v[2674] != 0.0))) && (s.v[2675] != 0.0)) {
            let assign60640_ad_e78468: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(603)), s.ad_value(2551))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(603)), s.ad_value(2551))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(603)), s.ad_value(2551))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2526, 1e-100, assign60640_ad_e78468);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) && (!(s.v[2674] != 0.0))) && (!(s.v[2675] != 0.0))) {
            let assign60650_ad_e78519: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(603)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(603)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(603)), s.ad_value(2551)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2526, &assign60650_ad_e78519);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2672] != 0.0))) {
            s.store_mul_ad_rhs(2550, 526, A::mul(A::mul(A::mul(s.ad_value(827), s.ad_value(2551)), s.ad_value(2551)), s.ad_value(2526)));
        }

        s.v[2676] = if (s.v[535] > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (s.v[2676] != 0.0)) {
            s.store_scalar(2552, 1.0);
        }

        s.v[2677] = if (s.v[2525] > ((-s.v[438]) * s.v[535])) { 1.0 } else { 0.0 };

        s.v[2678] = if (s.v[538] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2676] != 0.0))) && (s.v[2677] != 0.0)) && (s.v[2678] != 0.0)) {
            s.store_mul_ad(2526, A::mul(A::mul(A::mul(s.ad_value(2525), s.ad_value(609)), A::mul(s.ad_value(2525), s.ad_value(609))), A::mul(s.ad_value(2525), s.ad_value(609))), A::mul(s.ad_value(2525), s.ad_value(609)));
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2676] != 0.0))) && (s.v[2677] != 0.0)) && (!(s.v[2678] != 0.0))) {
            s.store_ad(2526, &A::pow(A::abs(A::mul(s.ad_value(2525), s.ad_value(609))), s.ad_value(538)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2676] != 0.0))) && (s.v[2677] != 0.0)) {
            s.store_div_from_scalar_ad(2552, 1.0, A::sub_from_scalar(1.0, s.ad_value(2526)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) {
            s.store_add_ad_rhs(2552, 606, A::mul(A::add(s.ad_value(2525), A::scale(s.ad_value(535), s.v[438])), s.ad_value(612)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_mul_ad_lhs(1904, A::scale(A::add(A::add(A::add(s.ad_value(2527), s.ad_value(2528)), s.ad_value(2535)), s.ad_value(2550)), p.p29), 2552);
        }

        s.v[2679] = if (s.v[570] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (s.v[2679] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(567))));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2679] != 0.0))) {
            s.store_ad(2526, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(567))), s.ad_value(570)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_scale_ad(1910, A::add(A::mul(s.ad_value(579), A::sub_from_scalar(1.0, s.ad_value(2526))), A::mul(s.ad_value(582), A::sub(s.ad_value(827), s.ad_value(2518)))), p.p30);
        }

        s.v[2680] = if (s.v[669] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2680] != 0.0)) {
            s.store_scalar(1905, 0.0);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (s.v[2680] != 0.0)) {
            s.store_scalar(1911, 0.0);
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_mul(2527, 559, 2517);
        }

        s.v[2681] = if ((s.v[518] == 0.0) && (s.v[521] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2681] != 0.0)) {
            s.store_scalar(2528, 0.0);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) {
            s.store_sub(2529, 565, 2523);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) {
            s.store_sub_from_scalar_ad(2530, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2521), s.ad_value(2529)))));
        }

        s.v[2682] = if (s.v[507] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) && (s.v[2682] != 0.0)) {
            s.store_scalar(2531, 0.0);
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) && (!(s.v[2682] != 0.0))) {
            s.store_mul_ad(2531, A::add(A::div(A::mul(A::square(s.ad_value(2530)), A::ln(s.ad_value(2530))), A::sub_from_scalar(1.0, s.ad_value(2530))), s.ad_value(2530)), A::sub_from_scalar(1.0, A::scale(s.ad_value(507), 2.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) {
            s.store_add(2532, 2530, 2531);
        }

        s.v[2683] = if (s.v[507] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) && (s.v[2683] != 0.0)) {
            s.store_sqrt_ad(2526, A::mul(s.ad_value(2529), s.ad_value(592)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) && (!(s.v[2683] != 0.0))) {
            s.store_ad(2526, &A::pow(A::mul(s.ad_value(2529), s.ad_value(592)), s.ad_value(507)));
        }

    }

    pub(super) fn stamp_transient_block_41(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) {
            s.store_mul(2533, 586, 2526);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) {
            s.store_mul_ad_rhs(2534, 556, A::mul(A::offset(s.ad_value(2520), (-1.0)), s.ad_value(2533)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) {
            s.store_mul_ad_rhs(2528, 518, A::mul(s.ad_value(2534), s.ad_value(2532)));
        }

        s.v[2684] = if (s.v[521] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2684] != 0.0)) {
            s.store_scalar(2535, 0.0);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_mul_ad_rhs(2536, 601, A::div(A::mul(s.ad_value(2533), s.ad_value(571)), s.ad_value(2529)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_div_ad_lhs(2537, A::scale(s.ad_value(598), 0.666666666666667), 2536);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_square(2538, 2537);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_sqrt_ad(2539, A::div(A::square(s.ad_value(2538)), A::offset(A::square(s.ad_value(2538)), 1.0)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_sqrt(2540, 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_mul(2541, 2539, 2540);
        }

        s.v[2685] = if (((-s.v[507]) * s.v[574]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_div_from_scalar_ad(2542, 1.0, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (!(s.v[2685] != 0.0))) {
            s.store_pow_ad(2542, A::offset(A::mul(s.ad_value(2536), s.ad_value(2541)), 1.0), A::mul(A::neg(s.ad_value(507)), s.ad_value(574)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_div_ad(2543, A::mul(s.ad_value(2532), s.ad_value(2542)), A::add(s.ad_value(2532), s.ad_value(2542)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_sqrt_ad(2544, A::scale(A::div(s.ad_value(2536), s.ad_value(2540)), 0.375));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_sub_ad_lhs(2545, A::scale(A::mul(s.ad_value(2537), s.ad_value(2540)), 2.0), 2539);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_add_ad(2546, A::sub(A::mul(A::mul(s.ad_value(598), s.ad_value(2537)), s.ad_value(2540)), A::mul(s.ad_value(598), s.ad_value(2539))), A::scale(A::mul(s.ad_value(2536), s.ad_value(2541)), 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_mul_ad_lhs(2547, A::offset(s.ad_value(2545), (-1.0)), 2544);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_square(2508, 2547);
        }

        s.v[2686] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (s.v[2686] != 0.0)) {
            s.store_div_from_scalar_ad(2509, 1.0, A::offset(A::scale(s.ad_value(2547), s.v[366]), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (!(s.v[2686] != 0.0))) {
            s.store_div_from_scalar_ad(2509, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2547), s.v[366])));
        }

        s.v[2687] = if (((-s.v[2508]) + s.v[2546]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (s.v[2687] != 0.0)) {
            s.store_exp_ad(2526, A::sub(s.ad_value(2546), s.ad_value(2508)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (!(s.v[2687] != 0.0))) {
            let assign61200_ad_e79394: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2546), s.ad_value(2508))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2526, &assign61200_ad_e79394);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_mul_ad_lhs(2510, A::add(A::add(A::scale(s.ad_value(2509), 0.29214664), A::scale(A::square(s.ad_value(2509)), s.v[367])), A::scale(A::mul(A::square(s.ad_value(2509)), s.ad_value(2509)), s.v[368])), 2526);
        }

        s.v[2688] = if (s.v[2547] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (s.v[2688] != 0.0)) {
            s.copy_ad(2548, 2510);
        }

        s.v[2689] = if (s.v[2546] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (!(s.v[2688] != 0.0))) && (s.v[2689] != 0.0)) {
            s.store_exp(2526, 2546);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (!(s.v[2688] != 0.0))) && (!(s.v[2689] != 0.0))) {
            s.store_div_from_scalar_ad(2526, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2546)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (!(s.v[2688] != 0.0))) {
            s.store_sub_ad_lhs(2548, A::scale(s.ad_value(2526), 2.0), 2510);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_scale_ad(2549, A::div(A::mul(s.ad_value(598), s.ad_value(2548)), s.ad_value(2544)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_mul_ad_rhs(2535, 521, A::mul(A::mul(s.ad_value(2534), s.ad_value(2549)), s.ad_value(2543)));
        }

        s.v[2690] = if (s.v[527] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2690] != 0.0)) {
            s.store_scalar(2550, 0.0);
        }

        s.v[2691] = if (s.v[507] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2691] != 0.0)) {
            s.store_sqrt_ad(2526, A::mul(A::sub(s.ad_value(504), s.ad_value(2524)), s.ad_value(592)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) {
            s.store_ad(2526, &A::pow(A::mul(A::sub(s.ad_value(504), s.ad_value(2524)), s.ad_value(592)), s.ad_value(507)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) {
            s.store_mul_ad_rhs(2551, 574, A::div(A::mul(A::sub(s.ad_value(504), s.ad_value(2524)), s.ad_value(589)), s.ad_value(2526)));
        }

        s.v[2692] = if (((((-s.v[604]) / s.v[2551])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2692] != 0.0)) {
            s.store_exp_ad(2526, A::div(A::neg(s.ad_value(604)), s.ad_value(2551)));
        }

        s.v[2693] = if (((-s.v[604]) / s.v[2551]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2692] != 0.0))) && (s.v[2693] != 0.0)) {
            let assign61390_ad_e79734: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(604)), s.ad_value(2551))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(604)), s.ad_value(2551))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(604)), s.ad_value(2551))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2526, 1e-100, assign61390_ad_e79734);
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2692] != 0.0))) && (!(s.v[2693] != 0.0))) {
            let assign61400_ad_e79785: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(604)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(604)), s.ad_value(2551)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(604)), s.ad_value(2551)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2526, &assign61400_ad_e79785);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2690] != 0.0))) {
            s.store_mul_ad_rhs(2550, 527, A::mul(A::mul(A::mul(s.ad_value(827), s.ad_value(2551)), s.ad_value(2551)), s.ad_value(2526)));
        }

        s.v[2694] = if (s.v[635] > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2694] != 0.0)) {
            s.store_scalar(2552, 1.0);
        }

        s.v[2695] = if (s.v[2525] > ((-s.v[438]) * s.v[635])) { 1.0 } else { 0.0 };

        s.v[2696] = if (s.v[539] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2695] != 0.0)) && (s.v[2696] != 0.0)) {
            s.store_mul_ad(2526, A::mul(A::mul(A::mul(s.ad_value(2525), s.ad_value(610)), A::mul(s.ad_value(2525), s.ad_value(610))), A::mul(s.ad_value(2525), s.ad_value(610))), A::mul(s.ad_value(2525), s.ad_value(610)));
        }

        if ((((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2695] != 0.0)) && (!(s.v[2696] != 0.0))) {
            s.store_ad(2526, &A::pow(A::abs(A::mul(s.ad_value(2525), s.ad_value(610))), s.ad_value(539)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2695] != 0.0)) {
            s.store_div_from_scalar_ad(2552, 1.0, A::sub_from_scalar(1.0, s.ad_value(2526)));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) {
            s.store_add_ad_rhs(2552, 607, A::mul(A::add(s.ad_value(2525), A::scale(s.ad_value(635), s.v[438])), s.ad_value(613)));
        }

        if (((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_mul_ad_lhs(1905, A::scale(A::add(A::add(A::add(s.ad_value(2527), s.ad_value(2528)), s.ad_value(2535)), s.ad_value(2550)), p.p29), 2552);
        }

        s.v[2697] = if (s.v[629] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            let assign61520_ad_e80010: A = {
                if (s.v[827] < s.v[544]) {
                    {
                        if (((s.v[827] - s.v[544]) / s.v[545]) < (-37.0)) {
                            s.ad_value(544)
                        } else {
                            A::add(s.ad_value(544), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(827), s.ad_value(544)), s.ad_value(545))), 1.0)), s.ad_value(545)))
                        }
                    }
                } else {
                    {
                        if (((s.v[827] - s.v[544]) / s.v[545]) > 37.0) {
                            s.ad_value(827)
                        } else {
                            A::add(s.ad_value(827), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(544), s.ad_value(827)), s.ad_value(545))), 1.0)), s.ad_value(545)))
                        }
                    }
                }
            };
            s.store_ad(2553, &assign61520_ad_e80010);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(678), 4.0), 678);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_div(2512, 678, 679);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_add_ad_rhs(2513, 2553, A::mul(s.ad_value(678), s.ad_value(2512)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_add(2514, 679, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_sub(2515, 679, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_scale_ad(2554, A::div(A::mul(s.ad_value(2553), s.ad_value(679)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2698] = if (s.v[571] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) && (s.v[2698] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(568))));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) && (!(s.v[2698] != 0.0))) {
            s.store_ad(2526, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(568))), s.ad_value(571)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_scale_ad(1911, A::add(A::mul(s.ad_value(580), A::sub_from_scalar(1.0, s.ad_value(2526))), A::mul(s.ad_value(583), A::sub(s.ad_value(2553), s.ad_value(2554)))), p.p30);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_sub_ad_lhs(2553, A::add(s.ad_value(827), s.ad_value(544)), 2553);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_mul_ad_lhs(2511, A::scale(s.ad_value(678), 4.0), 678);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_div(2512, 678, 679);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_add_ad_rhs(2513, 2553, A::mul(s.ad_value(678), s.ad_value(2512)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_add(2514, 679, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_sub(2515, 679, 2513);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_sqrt_ad(2516, A::add(A::square(s.ad_value(2515)), s.ad_value(2511)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_scale_ad(2554, A::div(A::mul(s.ad_value(2553), s.ad_value(679)), A::add(s.ad_value(2514), s.ad_value(2516))), 2.0);
        }

        s.v[2699] = if (s.v[624] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) && (s.v[2699] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(623))));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) && (!(s.v[2699] != 0.0))) {
            s.store_ad(2526, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2554), s.ad_value(623))), s.ad_value(624)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_scale_ad(466, A::add(A::mul(s.ad_value(627), A::sub_from_scalar(1.0, s.ad_value(2526))), A::mul(s.ad_value(628), A::sub(s.ad_value(2553), s.ad_value(2554)))), p.p30);
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_add(1911, 1911, 466);
        }

        s.v[2700] = if (s.v[571] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2697] != 0.0))) && (s.v[2700] != 0.0)) {
            s.store_sqrt_ad(2526, A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(568))));
        }

        if (((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2697] != 0.0))) && (!(s.v[2700] != 0.0))) {
            s.store_ad(2526, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2518), s.ad_value(568))), s.ad_value(571)));
        }

        if ((((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2697] != 0.0))) {
            s.store_scale_ad(1911, A::add(A::mul(s.ad_value(580), A::sub_from_scalar(1.0, s.ad_value(2526))), A::mul(s.ad_value(583), A::sub(s.ad_value(827), s.ad_value(2518)))), p.p30);
        }

        if ((s.v[2555] != 0.0) && (!(s.v[2556] != 0.0))) {
            s.store_add_ad(843, A::add(A::mul(s.ad_value(667), s.ad_value(1903)), A::mul(s.ad_value(668), s.ad_value(1904))), A::mul(s.ad_value(669), s.ad_value(1905)));
        }

        s.store_scale(865, 805, s.v[712]);

        s.store_scale(866, 806, s.v[712]);

        s.store_scale(867, 807, s.v[712]);

        s.store_scale(868, 808, s.v[712]);

        s.store_scale(869, 809, s.v[712]);

        s.store_scale(870, 810, s.v[712]);

        s.store_scale(871, 811, s.v[712]);

        s.v[2701] = if (s.v[825] > 0.0) { 1.0 } else { 0.0 };

        s.v[2702] = if (s.v[295] > 0.0) { 1.0 } else { 0.0 };

        s.v[2703] = if (s.v[296] > 0.0) { 1.0 } else { 0.0 };

        s.v[2704] = if (s.v[297] > 0.0) { 1.0 } else { 0.0 };

        s.v[2705] = if (s.v[298] > 0.0) { 1.0 } else { 0.0 };

        s.v[2706] = if (s.v[299] > 0.0) { 1.0 } else { 0.0 };

        s.v[2707] = if (s.v[300] > 0.0) { 1.0 } else { 0.0 };

        s.v[2708] = if (s.v[301] > 0.0) { 1.0 } else { 0.0 };

        s.store_neg_ad(844, A::add(A::add(s.ad_value(845), s.ad_value(846)), s.ad_value(847)));

        s.store_add(848, 848, 1898);

        s.store_add(849, 849, 1899);

        s.store_add_ad(851, A::add(A::mul(s.ad_value(640), s.ad_value(1906)), A::mul(s.ad_value(641), s.ad_value(1907))), A::mul(s.ad_value(642), s.ad_value(1908)));

        s.store_add_ad(852, A::add(A::mul(s.ad_value(667), s.ad_value(1909)), A::mul(s.ad_value(668), s.ad_value(1910))), A::mul(s.ad_value(669), s.ad_value(1911)));

        s.v[2710] = if (s.v[825] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[2710] != 0.0) {
            s.copy_ad(2709, 847);
        }

        if (s.v[2710] != 0.0) {
            s.copy_ad(847, 844);
        }

        if (s.v[2710] != 0.0) {
            s.copy_ad(844, 2709);
        }

        s.v[858] = 0.0;

        s.v[2727] = 0.0;

        s.v[2722] = 0.0;

        s.v[853] = 1e-40;

        s.v[855] = 0.0;

        s.v[857] = 0.0;

        s.store_mul(854, 1892, 1883);

        s.v[856] = 0.0;

        s.v[2729] = 0.0;

        s.v[863] = 0.0;

        s.v[2742] = 0.0;

        s.v[864] = 0.0;

        s.v[2743] = if ((s.v[1817] > 0.0) && (s.v[710] > 0.0)) { 1.0 } else { 0.0 };

        s.v[2744] = if (p.p34 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2743] != 0.0) && (s.v[2744] != 0.0)) {
            s.store_scaled_mul(2711, 765, 1852, s.v[709]);
        }

        if ((s.v[2743] != 0.0) && (s.v[2744] != 0.0)) {
            s.store_mul(2712, 765, 1854);
        }

        if ((s.v[2743] != 0.0) && (s.v[2744] != 0.0)) {
            s.store_mul_ad_lhs(2713, A::mul(s.ad_value(765), s.ad_value(1852)), 1848);
        }

        if ((s.v[2743] != 0.0) && (s.v[2744] != 0.0)) {
            s.store_mul_ad(858, A::add(A::sub(s.ad_value(273), A::mul(s.ad_value(274), s.ad_value(2711))), A::mul(s.ad_value(275), A::square(s.ad_value(2711)))), A::ln(A::div(A::add(s.ad_value(2712), A::scale(s.ad_value(2713), 0.5)), A::sub(s.ad_value(2712), A::scale(s.ad_value(2713), 0.5)))));
        }

        if ((s.v[2743] != 0.0) && (s.v[2744] != 0.0)) {
            s.store_add_ad_rhs(858, 858, A::mul(A::add(s.ad_value(274), A::mul(s.ad_value(275), A::sub(s.ad_value(2712), A::scale(s.ad_value(2711), 2.0)))), s.ad_value(2713)));
        }

        if ((s.v[2743] != 0.0) && (s.v[2744] != 0.0)) {
            s.store_div_ad_lhs(858, A::mul(A::mul(A::mul(s.ad_value(716), s.ad_value(832)), s.ad_value(1864)), s.ad_value(858)), 2711);
        }

        if ((s.v[2743] != 0.0) && (s.v[2744] != 0.0)) {
            s.store_ad(858, &{
                if (s.v[858] > 0.0) {
                    s.ad_value(858)
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.v[2745] = if (p.p32 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) {
            s.store_div(2714, 1854, 1852);
        }

        if ((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) {
            s.store_div(2715, 1853, 1854);
        }

        if ((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) {
            s.store_scaled_div(2716, 1848, 2714, (0.5 * 0.16666666666666666));
        }

        if ((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) {
            s.store_square(2717, 2716);
        }

        if ((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) {
            s.store_offset_ad(2718, A::div(s.ad_value(2714), s.ad_value(1865)), (-1.0));
        }

        if ((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) {
            s.store_ad(2719, &{
                if ((1.0 - (12.0 * (s.v[2718] * s.v[2717]))) > 1e-20) {
                    A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2718), s.ad_value(2717)), 12.0))
                } else {
                    A::constant(1e-20)
                }
            });
        }

        if ((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) {
            s.store_div_from_scalar_ad(2720, 1.0, A::square(s.ad_value(2719)));
        }

        if ((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) {
            s.store_mul_ad_lhs(2721, A::mul(s.ad_value(710), s.ad_value(1854)), 1864);
        }

        if ((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) {
            s.store_sub_ad(2722, A::add(s.ad_value(2715), A::scale(s.ad_value(2717), 12.0)), A::scale(A::mul(A::mul(A::offset(s.ad_value(2715), 1.0), s.ad_value(2717)), s.ad_value(2718)), 24.0));
        }

        if ((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) {
            s.store_ad(2722, &{
                if (s.v[2722] > 1e-40) {
                    s.ad_value(2722)
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if ((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) {
            s.store_mul_ad_lhs(2722, A::mul(s.ad_value(2721), s.ad_value(2720)), 2722);
        }

        s.v[2746] = if (s.v[272] > 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_42(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) && (s.v[2746] != 0.0)) {
            s.store_div(2723, 1858, 1857);
        }

        if (((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) && (s.v[2746] != 0.0)) {
            s.store_mul_ad_lhs(2724, A::mul(A::square(s.ad_value(2723)), s.ad_value(1848)), 1848);
        }

        s.v[2747] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) && (s.v[2746] != 0.0)) && (s.v[2747] != 0.0)) {
            s.store_div_ad_rhs(2724, 2724, A::offset(A::mul(s.ad_value(2723), s.ad_value(1848)), 1.0));
        }

        if (((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) && (s.v[2746] != 0.0)) {
            s.store_scale_ad(2725, A::mul(s.ad_value(1857), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2724), 2.0), 1.0)), 1.0)), 0.5);
        }

        if (((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) && (s.v[2746] != 0.0)) {
            s.store_div_ad_rhs(2726, 1857, A::mul(s.ad_value(2725), s.ad_value(2719)));
        }

        if (((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) && (s.v[2746] != 0.0)) {
            s.store_mul_ad_lhs(2727, A::mul(A::mul(A::mul(s.ad_value(804), s.ad_value(832)), s.ad_value(1845)), s.ad_value(2726)), 2726);
        }

        if (((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) && (s.v[2746] != 0.0)) {
            s.store_add_ad_rhs(2722, 2722, A::scale(s.ad_value(2727), 1.0 / (s.v[712])));
        }

        if ((s.v[2743] != 0.0) && (s.v[2745] != 0.0)) {
            s.store_sqrt_ad(856, A::mul(s.ad_value(713), s.ad_value(2722)));
        }

        s.v[2748] = if ((((p.p50 == 1.0) && (s.v[713] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) {
            s.store_sub_ad(853, A::sub(A::scale(s.ad_value(2715), 0.08333333333333333), A::mul(s.ad_value(2717), A::sub(A::offset(s.ad_value(2715), 0.2), A::scale(s.ad_value(2717), 12.0)))), A::scale(A::mul(A::mul(s.ad_value(2717), A::sub(A::offset(s.ad_value(2715), 1.0), A::scale(s.ad_value(2717), 12.0))), s.ad_value(2718)), 1.6));
        }

        if ((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) {
            s.store_ad(853, &{
                if (s.v[853] > 1e-40) {
                    s.ad_value(853)
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if ((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) {
            s.store_mul_ad_lhs(853, A::div(s.ad_value(2720), s.ad_value(2721)), 853);
        }

        if ((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) {
            s.store_mul_ad(2728, A::mul(s.ad_value(2720), s.ad_value(2716)), A::sub(A::sub_from_scalar(1.0, A::scale(s.ad_value(2717), 12.0)), A::mul(A::sub(A::add(s.ad_value(2715), A::scale(s.ad_value(2717), 19.2)), A::scale(A::mul(s.ad_value(2715), s.ad_value(2717)), 12.0)), s.ad_value(2718))));
        }

        if ((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) {
            s.store_div_ad(854, A::mul(A::mul(A::square(s.ad_value(1896)), s.ad_value(1892)), s.ad_value(1883)), A::square(s.ad_value(1894)));
        }

        s.v[2749] = if (s.v[272] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) && (s.v[2749] != 0.0)) {
            s.store_add_ad_rhs(853, 853, A::div(A::mul(s.ad_value(2727), A::offset(A::scale(s.ad_value(2717), 12.0), 1.0)), A::scale(A::mul(A::scale(s.ad_value(2721), 12.0), s.ad_value(2721)), s.v[712])));
        }

        if (((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) && (s.v[2749] != 0.0)) {
            s.store_sub_ad_rhs(2728, 2728, A::div(A::mul(A::mul(s.ad_value(2727), s.ad_value(2716)), A::offset(s.ad_value(2718), 1.0)), A::scale(s.ad_value(2721), s.v[712])));
        }

        if ((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) {
            s.store_sqrt_ad(2729, A::div(s.ad_value(713), s.ad_value(853)));
        }

        s.v[2750] = if (s.v[856] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) && (s.v[2750] != 0.0)) {
            s.store_scalar(857, 0.0);
        }

        if (((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) && (!(s.v[2750] != 0.0))) {
            s.store_div_ad_lhs(857, A::mul(s.ad_value(2728), s.ad_value(2729)), 856);
        }

        if ((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) {
            s.store_ad(857, &{
                if (s.v[857] > 0.0) {
                    {
                        if (s.v[857] < 1.0) {
                            s.ad_value(857)
                        } else {
                            A::constant(1.0)
                        }
                    }
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((s.v[2743] != 0.0) && (s.v[2748] != 0.0)) {
            s.store_div_ad_lhs(855, A::mul(s.ad_value(857), s.ad_value(856)), 2729);
        }

        s.store_scale_ad(1912, A::abs(s.ad_value(835)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(1913, A::abs(s.ad_value(836)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(1914, A::abs(s.ad_value(833)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(1915, A::abs(s.ad_value(834)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(1916, A::mul(A::offset(s.ad_value(1873), 1.0), A::abs(s.ad_value(841))), (2.0 * 1.6021918e-19));

        s.store_scale_ad(1917, A::abs(s.ad_value(842)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(1918, A::abs(s.ad_value(843)), (2.0 * 1.6021918e-19));

        s.v[2751] = if (s.v[825] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2751] != 0.0) {
            s.store_add(859, 1912, 1914);
        }

        if (s.v[2751] != 0.0) {
            s.store_add(860, 1913, 1915);
        }

        if (s.v[2751] != 0.0) {
            s.copy_ad(861, 1917);
        }

        if (s.v[2751] != 0.0) {
            s.store_add(862, 1918, 1916);
        }

        if (!(s.v[2751] != 0.0)) {
            s.store_add(859, 1913, 1914);
        }

        if (!(s.v[2751] != 0.0)) {
            s.store_add(860, 1912, 1915);
        }

        if (!(s.v[2751] != 0.0)) {
            s.store_add(861, 1917, 1916);
        }

        if (!(s.v[2751] != 0.0)) {
            s.copy_ad(862, 1918);
        }

        s.v[2752] = if (((p.p46 != 0.0) && (s.v[282] > 0.0)) && (s.v[1868] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2752] != 0.0) {
            s.store_div_ad_lhs(1920, A::scale(s.ad_value(1871), 4.0), 718);
        }

        if (s.v[2752] != 0.0) {
            s.store_div_ad(2730, A::sqrt(A::offset(s.ad_value(1920), 1.0)), A::offset(A::sqrt(A::offset(s.ad_value(1920), 1.1)), (-1.0)));
        }

        if (s.v[2752] != 0.0) {
            s.store_scale(1920, 765, s.v[709]);
        }

        if (s.v[2752] != 0.0) {
            s.store_mul(2731, 1920, 2730);
        }

        if (s.v[2752] != 0.0) {
            s.store_mul_ad_rhs(2732, 1920, A::add(s.ad_value(1870), s.ad_value(2730)));
        }

        if (s.v[2752] != 0.0) {
            s.store_mul_ad_lhs(2733, A::mul(A::mul(A::neg(s.ad_value(1920)), s.ad_value(2730)), s.ad_value(1872)), 1869);
        }

        if (s.v[2752] != 0.0) {
            s.store_mul_ad(863, A::sub(s.ad_value(291), A::mul(A::sub(s.ad_value(292), A::mul(s.ad_value(293), s.ad_value(2731))), s.ad_value(2731))), A::ln(A::div(A::add(s.ad_value(2732), A::scale(s.ad_value(2733), 0.5)), A::sub(s.ad_value(2732), A::scale(s.ad_value(2733), 0.5)))));
        }

        if (s.v[2752] != 0.0) {
            s.store_add_ad_rhs(863, 863, A::mul(A::add(s.ad_value(292), A::mul(s.ad_value(293), A::sub(s.ad_value(2732), A::scale(s.ad_value(2731), 2.0)))), s.ad_value(2733)));
        }

        if (s.v[2752] != 0.0) {
            s.store_div_ad_lhs(863, A::mul(A::mul(A::mul(s.ad_value(720), s.ad_value(840)), s.ad_value(1864)), s.ad_value(863)), 2731);
        }

        if (s.v[2752] != 0.0) {
            s.store_ad(863, &{
                if (s.v[863] > 0.0) {
                    s.ad_value(863)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[2752] != 0.0) {
            s.store_div_ad_lhs(2734, A::scale(A::add(s.ad_value(1870), s.ad_value(2730)), s.v[709]), 2730);
        }

        if (s.v[2752] != 0.0) {
            s.store_div_ad(2735, A::mul(A::scale(s.ad_value(1812), 1.0 / (s.v[709])), s.ad_value(1870)), A::add(s.ad_value(1870), s.ad_value(2730)));
        }

        if (s.v[2752] != 0.0) {
            s.store_div_ad_lhs(2736, A::mul(A::scale(s.ad_value(1872), (((-0.5) * 0.16666666666666666) * s.v[709])), s.ad_value(1869)), 2734);
        }

        if (s.v[2752] != 0.0) {
            s.store_square(2737, 2736);
        }

        if (s.v[2752] != 0.0) {
            s.store_scalar(2738, 0.0);
        }

        if (s.v[2752] != 0.0) {
            s.store_mul(1920, 1852, 1865);
        }

        s.v[2753] = if (s.v[1920] > 1e-10) { 1.0 } else { 0.0 };

        if ((s.v[2752] != 0.0) && (s.v[2753] != 0.0)) {
            s.store_offset_ad(2738, A::div(A::mul(s.ad_value(2730), s.ad_value(2734)), s.ad_value(1920)), (-1.0));
        }

        if (s.v[2752] != 0.0) {
            s.store_ad(2739, &{
                if ((1.0 - (12.0 * (s.v[2738] * s.v[2737]))) > 1e-20) {
                    A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2738), s.ad_value(2737)), 12.0))
                } else {
                    A::constant(1e-20)
                }
            });
        }

        if (s.v[2752] != 0.0) {
            s.store_div_from_scalar_ad(2740, 1.0, A::square(s.ad_value(2739)));
        }

        if (s.v[2752] != 0.0) {
            s.store_mul_ad_lhs(2741, A::mul(A::scale(s.ad_value(711), s.v[709]), A::add(s.ad_value(1870), s.ad_value(2730))), 1864);
        }

        if (s.v[2752] != 0.0) {
            s.store_sub_ad(2742, A::add(s.ad_value(2735), A::scale(s.ad_value(2737), 12.0)), A::scale(A::mul(A::mul(A::offset(s.ad_value(2735), 1.0), s.ad_value(2737)), s.ad_value(2738)), 24.0));
        }

        if (s.v[2752] != 0.0) {
            s.store_ad(2742, &{
                if (s.v[2742] > 1e-40) {
                    s.ad_value(2742)
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if (s.v[2752] != 0.0) {
            s.store_mul_ad_lhs(2742, A::mul(s.ad_value(2741), s.ad_value(2740)), 2742);
        }

        if (s.v[2752] != 0.0) {
            s.store_sqrt_ad(864, A::mul(s.ad_value(721), s.ad_value(2742)));
        }

    }

    pub(super) fn stamp_reactive_block_0(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.v[984] = if (p.p37 >= 0.0) { 1.0 } else { 0.0 };

        if (s.v[984] != 0.0) {
            s.store_scalar(0, 1.0);
        }

        if (!(s.v[984] != 0.0)) {
            s.store_scalar(0, (-1.0));
        }

        s.v[761] = (8.8541878176e-12 * 11.8);

        s.v[344] = (273.15 + p.p38);

        s.v[468] = 0.0;

        s.v[985] = if (p.p920 > 0.5) { 1.0 } else { 0.0 };

        if (s.v[985] != 0.0) {
            s.store_scalar(468, 1.0);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(468, 0.0);
        }

        s.v[358] = (273.15 + p.p816);

        s.v[361] = (1.3806505e-23 / 1.6021918e-19);

        s.v[362] = (s.v[361] * s.v[358]);

        s.v[363] = (1.0 / s.v[362]);

        s.v[369] = ((-((0.000702 * s.v[358]) * s.v[358])) / (1108.0 + s.v[358]));

        s.v[372] = (p.p827 + s.v[369]);

        s.v[373] = (p.p828 + s.v[369]);

        s.v[374] = (p.p829 + s.v[369]);

        s.v[402] = (1.0 - p.p824);

        s.v[403] = (1.0 - p.p825);

        s.v[404] = (1.0 - p.p826);

        s.v[405] = (1.0 / s.v[402]);

        s.v[406] = (1.0 / s.v[403]);

        s.v[407] = (1.0 / s.v[404]);

        s.v[417] = (s.v[761] / p.p818);

        s.v[418] = ((p.p836 * s.v[761]) / p.p819);

        s.v[419] = ((p.p837 * s.v[761]) / p.p820);

        s.v[420] = (1.0 / s.v[417]);

        s.v[421] = (1.0 / s.v[418]);

        s.v[422] = (1.0 / s.v[419]);

        s.v[423] = (1.0 / p.p821);

        s.v[424] = (1.0 / p.p822);

        s.v[425] = (1.0 / p.p823);

        s.v[438] = (1.0 - (1.0 / p.p817));

        s.v[442] = (1.0 / p.p853);

        s.v[443] = (1.0 / p.p854);

        s.v[444] = (1.0 / p.p855);

        s.v[986] = if ((((p.p859 != 1.0) || (p.p860 != 1.0)) || (p.p861 != 1.0)) || (p.p862 != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[986] != 0.0) {
            s.store_scalar(467, 1.0);
        }

        if (!(s.v[986] != 0.0)) {
            s.store_scalar(467, 0.0);
        }

        s.v[987] = if (s.v[467] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[987] != 0.0) {
            s.store_scalar(451, (if ((p.p820 * p.p859) > 1e-18) { (p.p820 * p.p859) } else { 1e-18 }));
        }

        if (s.v[987] != 0.0) {
            s.store_scalar(452, (if ((p.p823 * p.p860) > 0.05) { (p.p823 * p.p860) } else { 0.05 }));
        }

        if (s.v[987] != 0.0) {
            s.store_scalar(453, (if ((if ((p.p826 * p.p861) > 0.05) { (p.p826 * p.p861) } else { 0.05 }) < 0.95) { (if ((p.p826 * p.p861) > 0.05) { (p.p826 * p.p861) } else { 0.05 }) } else { 0.95 }));
        }

        if (s.v[987] != 0.0) {
            s.store_scalar(454, (p.p829 * p.p862));
        }

        if (s.v[987] != 0.0) {
            s.store_offset(456, 454, s.v[369]);
        }

        if (s.v[987] != 0.0) {
            s.store_sub_from_scalar(461, 1.0, 453);
        }

        if (s.v[987] != 0.0) {
            s.store_div_from_scalar(462, 1.0, 461);
        }

        s.v[988] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[988] != 0.0) {
            s.store_scalar(499, p.p818);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(500, p.p819);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(501, p.p820);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(502, p.p821);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(503, p.p822);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(504, p.p823);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(505, p.p824);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(506, p.p825);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(507, p.p826);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(508, p.p827);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(509, p.p828);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(510, p.p829);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(511, p.p830);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(512, p.p831);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(513, p.p832);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(516, p.p833);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(517, p.p834);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(518, p.p835);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(514, p.p836);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(515, p.p837);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(519, p.p838);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(520, p.p839);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(521, p.p840);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(522, p.p841);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(523, p.p842);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(524, p.p843);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(525, p.p844);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(526, p.p845);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(527, p.p846);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(528, p.p847);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(529, p.p848);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(530, p.p849);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(531, p.p850);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(532, p.p851);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(533, p.p852);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(534, p.p853);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(535, p.p854);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(536, p.p855);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(537, p.p856);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(538, p.p857);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(539, p.p858);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(547, p.p922);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(630, p.p865);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(631, p.p866);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(632, p.p867);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(633, p.p868);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(540, p.p859);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(541, p.p860);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(542, p.p861);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(543, p.p862);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(544, p.p863);
        }

        if (s.v[988] != 0.0) {
            s.store_scalar(545, p.p864);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(499, p.p869);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(500, p.p870);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(501, p.p871);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(502, p.p872);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(503, p.p873);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(504, p.p874);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(505, p.p875);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(506, p.p876);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(507, p.p877);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(508, p.p878);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(509, p.p879);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(510, p.p880);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(511, p.p881);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(512, p.p882);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(513, p.p883);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(516, p.p884);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(517, p.p885);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(518, p.p886);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(514, p.p887);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(515, p.p888);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(519, p.p889);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(520, p.p890);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(521, p.p891);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(522, p.p892);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(523, p.p893);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(524, p.p894);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(525, p.p895);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(526, p.p896);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(527, p.p897);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(528, p.p898);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(529, p.p899);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(530, p.p900);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(531, p.p901);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(532, p.p902);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(533, p.p903);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(534, p.p904);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(535, p.p905);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(536, p.p906);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(537, p.p907);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(538, p.p908);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(539, p.p909);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(547, p.p924);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(630, p.p916);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(631, p.p917);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(632, p.p918);
        }

    }

    pub(super) fn stamp_reactive_block_1(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (!(s.v[988] != 0.0)) {
            s.store_scalar(633, p.p919);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(540, p.p910);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(541, p.p911);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(542, p.p912);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(543, p.p913);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(544, p.p914);
        }

        if (!(s.v[988] != 0.0)) {
            s.store_scalar(545, p.p915);
        }

        s.store_offset(548, 508, s.v[369]);

        s.store_offset(549, 509, s.v[369]);

        s.store_offset(550, 510, s.v[369]);

        s.store_sub_from_scalar(569, 1.0, 505);

        s.store_sub_from_scalar(570, 1.0, 506);

        s.store_sub_from_scalar(571, 1.0, 507);

        s.store_div_from_scalar(572, 1.0, 569);

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_div_from_scalar(584, s.v[761], 499);

        s.store_div_ad_lhs(585, A::scale(s.ad_value(514), s.v[761]), 500);

        s.store_div_ad_lhs(586, A::scale(s.ad_value(515), s.v[761]), 501);

        s.store_div_from_scalar(587, 1.0, 584);

        s.store_div_from_scalar(588, 1.0, 585);

        s.store_div_from_scalar(589, 1.0, 586);

        s.store_div_from_scalar(590, 1.0, 502);

        s.store_div_from_scalar(591, 1.0, 503);

        s.store_div_from_scalar(592, 1.0, 504);

        s.store_div_from_scalar(608, 1.0, 534);

        s.store_div_from_scalar(609, 1.0, 535);

        s.store_div_from_scalar(610, 1.0, 536);

        s.v[989] = if ((((s.v[540] != 1.0) || (s.v[541] != 1.0)) || (s.v[542] != 1.0)) || (s.v[543] != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[989] != 0.0) {
            s.store_scalar(629, 1.0);
        }

        if (!(s.v[989] != 0.0)) {
            s.store_scalar(629, 0.0);
        }

        s.v[990] = if (s.v[629] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[990] != 0.0) {
            s.store_ad(614, &{
                if ((s.v[501] * s.v[540]) > 1e-18) {
                    A::mul(s.ad_value(501), s.ad_value(540))
                } else {
                    A::constant(1e-18)
                }
            });
        }

        if (s.v[990] != 0.0) {
            s.store_ad(615, &{
                if ((s.v[504] * s.v[541]) > 0.05) {
                    A::mul(s.ad_value(504), s.ad_value(541))
                } else {
                    A::constant(0.05)
                }
            });
        }

        if (s.v[990] != 0.0) {
            s.store_ad(616, &{
                if ((if ((s.v[507] * s.v[542]) > 0.05) { (s.v[507] * s.v[542]) } else { 0.05 }) < 0.95) {
                    {
                        if ((s.v[507] * s.v[542]) > 0.05) {
                            A::mul(s.ad_value(507), s.ad_value(542))
                        } else {
                            A::constant(0.05)
                        }
                    }
                } else {
                    A::constant(0.95)
                }
            });
        }

        if (s.v[990] != 0.0) {
            s.store_mul(617, 510, 543);
        }

        if (s.v[990] != 0.0) {
            s.store_offset(619, 617, s.v[369]);
        }

        if (s.v[990] != 0.0) {
            s.store_sub_from_scalar(624, 1.0, 616);
        }

        if (s.v[990] != 0.0) {
            s.store_div_from_scalar(625, 1.0, 624);
        }

        s.v[345] = ((ctx.temperature() + p.p55) + p.p35);

        s.v[346] = (s.v[345] / s.v[344]);

        s.v[347] = (s.v[345] - s.v[344]);

        s.v[348] = ((s.v[345] * 1.3806505e-23) / 1.6021918e-19);

        s.v[349] = (1.0 / s.v[348]);

        s.v[350] = s.v[345];

        s.v[351] = (s.v[350] * s.v[350]);

        s.v[352] = (s.v[350] - s.v[344]);

        s.v[353] = (s.v[344] / s.v[350]);

        s.v[354] = ((s.v[353]) as f64).ln();

        s.v[709] = ((s.v[350] * 1.3806505e-23) / 1.6021918e-19);

        s.v[355] = (1.0 / s.v[709]);

        s.v[356] = ((1.179 - (9.025e-5 * s.v[350])) - (3.05e-7 * s.v[351]));

        s.v[357] = ((((1.045 + (0.00045 * s.v[350])) * ((0.523 + (0.0014 * s.v[350])) - (1.48e-6 * s.v[351]))) * s.v[351]) / 90000.0);

        if !(s.v[357] > 0.001) {
            s.store_scalar(357, 0.001);
        }

        s.v[359] = (((ctx.temperature() + p.p55) + p.p35)).max((273.15 + (-250.0)));

        s.v[360] = (s.v[359] / s.v[358]);

        s.v[364] = (s.v[361] * s.v[359]);

        s.v[365] = (1.0 / s.v[364]);

        s.v[370] = ((-((0.000702 * s.v[359]) * s.v[359])) / (1108.0 + s.v[359]));

        s.v[375] = (p.p827 + s.v[370]);

        s.v[376] = (p.p828 + s.v[370]);

        s.v[377] = (p.p829 + s.v[370]);

        s.v[378] = (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[372] * s.v[363]) - (s.v[375] * s.v[365])))) as f64).exp());

        s.v[379] = (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[373] * s.v[363]) - (s.v[376] * s.v[365])))) as f64).exp());

        s.v[380] = (((s.v[360]) as f64).powf(1.5) * (((0.5 * ((s.v[374] * s.v[363]) - (s.v[377] * s.v[365])))) as f64).exp());

        s.v[381] = ((p.p830 * s.v[378]) * s.v[378]);

        s.v[382] = ((p.p831 * s.v[379]) * s.v[379]);

        s.v[383] = ((p.p832 * s.v[380]) * s.v[380]);

        s.v[384] = ((p.p821 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[378]) as f64).ln()));

        s.v[385] = ((p.p822 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[379]) as f64).ln()));

        s.v[386] = ((p.p823 * s.v[360]) - ((2.0 * s.v[364]) * ((s.v[380]) as f64).ln()));

        s.v[387] = (s.v[384] + (s.v[364] * (((1.0 + ((((0.05 - s.v[384]) * s.v[365])) as f64).exp())) as f64).ln()));

        s.v[388] = (s.v[385] + (s.v[364] * (((1.0 + ((((0.05 - s.v[385]) * s.v[365])) as f64).exp())) as f64).ln()));

        s.v[389] = (s.v[386] + (s.v[364] * (((1.0 + ((((0.05 - s.v[386]) * s.v[365])) as f64).exp())) as f64).ln()));

        s.v[399] = (1.0 / s.v[387]);

        s.v[400] = (1.0 / s.v[388]);

        s.v[401] = (1.0 / s.v[389]);

        s.v[408] = (p.p818 * (((p.p821 * s.v[399])) as f64).powf(p.p824));

        s.v[409] = (p.p819 * (((p.p822 * s.v[400])) as f64).powf(p.p825));

        s.v[410] = (p.p820 * (((p.p823 * s.v[401])) as f64).powf(p.p826));

        s.v[411] = ((s.v[408] * s.v[387]) * s.v[405]);

        s.v[412] = ((s.v[409] * s.v[388]) * s.v[406]);

        s.v[413] = ((s.v[410] * s.v[389]) * s.v[407]);

        s.v[414] = (2.0 * s.v[408]);

        s.v[415] = (2.0 * s.v[409]);

        s.v[416] = (2.0 * s.v[410]);

        s.v[426] = ((0.5 * s.v[375])).max(s.v[364]);

        s.v[427] = ((0.5 * s.v[376])).max(s.v[364]);

        s.v[428] = ((0.5 * s.v[377])).max(s.v[364]);

        s.v[429] = (s.v[426] * s.v[365]);

        s.v[430] = (s.v[427] * s.v[365]);

        s.v[431] = (s.v[428] * s.v[365]);

        s.v[432] = (((((((32.0 * p.p841) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[426] * s.v[426]) * s.v[426]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[433] = (((((((32.0 * p.p842) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[427] * s.v[427]) * s.v[427]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[434] = (((((((32.0 * p.p843) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[428] * s.v[428]) * s.v[428]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[435] = (p.p847 * (1.0 + (p.p850 * (s.v[359] - s.v[358]))));

        s.v[436] = (p.p848 * (1.0 + (p.p851 * (s.v[359] - s.v[358]))));

        s.v[437] = (p.p849 * (1.0 + (p.p852 * (s.v[359] - s.v[358]))));

        if !(s.v[435] > 0.0) {
            s.store_scalar(435, 0.0);
        }

        if !(s.v[436] > 0.0) {
            s.store_scalar(436, 0.0);
        }

        if !(s.v[437] > 0.0) {
            s.store_scalar(437, 0.0);
        }

        s.v[1010] = if (s.v[467] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1010] != 0.0) {
            s.store_offset(455, 454, s.v[370]);
        }

        if (s.v[1010] != 0.0) {
            s.store_scale_ad(457, A::exp(A::scale(A::sub(A::scale(s.ad_value(456), s.v[363]), A::scale(s.ad_value(455), s.v[365])), 0.5)), ((s.v[360]) as f64).powf(1.5));
        }

        if (s.v[1010] != 0.0) {
            s.store_sub_ad(458, A::scale(s.ad_value(452), s.v[360]), A::scale(A::ln(s.ad_value(457)), (2.0 * s.v[364])));
        }

        if (s.v[1010] != 0.0) {
            s.store_add_ad_rhs(459, 458, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(458)), s.v[365])), 1.0)), s.v[364]));
        }

        if (s.v[1010] != 0.0) {
            s.store_div_from_scalar(460, 1.0, 459);
        }

        if (s.v[1010] != 0.0) {
            s.store_mul_ad_rhs(463, 451, A::pow(A::mul(s.ad_value(452), s.ad_value(460)), s.ad_value(453)));
        }

        if (s.v[1010] != 0.0) {
            s.store_mul_ad_lhs(464, A::mul(s.ad_value(463), s.ad_value(459)), 462);
        }

        if (s.v[1010] != 0.0) {
            s.store_scale(465, 463, 2.0);
        }

        s.store_offset(551, 508, s.v[370]);

        s.store_offset(552, 509, s.v[370]);

        s.store_offset(553, 510, s.v[370]);

        s.store_scale_ad(554, A::exp(A::scale(A::sub(A::scale(s.ad_value(548), s.v[363]), A::scale(s.ad_value(551), s.v[365])), 0.5)), ((s.v[360]) as f64).powf(1.5));

        s.store_scale_ad(555, A::exp(A::scale(A::sub(A::scale(s.ad_value(549), s.v[363]), A::scale(s.ad_value(552), s.v[365])), 0.5)), ((s.v[360]) as f64).powf(1.5));

        s.store_scale_ad(556, A::exp(A::scale(A::sub(A::scale(s.ad_value(550), s.v[363]), A::scale(s.ad_value(553), s.v[365])), 0.5)), ((s.v[360]) as f64).powf(1.5));

        s.store_mul_ad_lhs(557, A::mul(s.ad_value(511), s.ad_value(554)), 554);

        s.store_mul_ad_lhs(558, A::mul(s.ad_value(512), s.ad_value(555)), 555);

        s.store_mul_ad_lhs(559, A::mul(s.ad_value(513), s.ad_value(556)), 556);

        s.store_sub_ad(560, A::scale(s.ad_value(502), s.v[360]), A::scale(A::ln(s.ad_value(554)), (2.0 * s.v[364])));

        s.store_sub_ad(561, A::scale(s.ad_value(503), s.v[360]), A::scale(A::ln(s.ad_value(555)), (2.0 * s.v[364])));

        s.store_sub_ad(562, A::scale(s.ad_value(504), s.v[360]), A::scale(A::ln(s.ad_value(556)), (2.0 * s.v[364])));

        s.store_add_ad_rhs(563, 560, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(560)), s.v[365])), 1.0)), s.v[364]));

        s.store_add_ad_rhs(564, 561, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(561)), s.v[365])), 1.0)), s.v[364]));

        s.store_add_ad_rhs(565, 562, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(562)), s.v[365])), 1.0)), s.v[364]));

        s.store_div_from_scalar(566, 1.0, 563);

        s.store_div_from_scalar(567, 1.0, 564);

        s.store_div_from_scalar(568, 1.0, 565);

        s.store_mul_ad_rhs(575, 499, A::pow(A::mul(s.ad_value(502), s.ad_value(566)), s.ad_value(505)));

        s.store_mul_ad_rhs(576, 500, A::pow(A::mul(s.ad_value(503), s.ad_value(567)), s.ad_value(506)));

        s.store_mul_ad_rhs(577, 501, A::pow(A::mul(s.ad_value(504), s.ad_value(568)), s.ad_value(507)));

        s.store_mul_ad_lhs(578, A::mul(s.ad_value(575), s.ad_value(563)), 572);

        s.store_mul_ad_lhs(579, A::mul(s.ad_value(576), s.ad_value(564)), 573);

        s.store_mul_ad_lhs(580, A::mul(s.ad_value(577), s.ad_value(565)), 574);

        s.store_scale(581, 575, 2.0);

        s.store_scale(582, 576, 2.0);

        s.store_scale(583, 577, 2.0);

        s.store_max_with_scalar_ad(593, A::scale(s.ad_value(551), 0.5), s.v[364]);

        s.store_max_with_scalar_ad(594, A::scale(s.ad_value(552), 0.5), s.v[364]);

        s.store_max_with_scalar_ad(595, A::scale(s.ad_value(553), 0.5), s.v[364]);

        s.store_scale(596, 593, s.v[365]);

        s.store_scale(597, 594, s.v[365]);

        s.store_scale(598, 595, s.v[365]);

        s.store_scale_ad(599, A::sqrt(A::mul(A::scale(s.ad_value(522), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(593)), s.ad_value(593)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scale_ad(600, A::sqrt(A::mul(A::scale(s.ad_value(523), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(594)), s.ad_value(594)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scale_ad(601, A::sqrt(A::mul(A::scale(s.ad_value(524), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(595)), s.ad_value(595)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_ad_rhs(602, 528, A::offset(A::scale(s.ad_value(531), (s.v[359] - s.v[358])), 1.0));

        s.store_mul_ad_rhs(603, 529, A::offset(A::scale(s.ad_value(532), (s.v[359] - s.v[358])), 1.0));

        s.store_mul_ad_rhs(604, 530, A::offset(A::scale(s.ad_value(533), (s.v[359] - s.v[358])), 1.0));

        if !(s.v[602] > 0.0) {
            s.store_scalar(602, 0.0);
        }

        if !(s.v[603] > 0.0) {
            s.store_scalar(603, 0.0);
        }

        if !(s.v[604] > 0.0) {
            s.store_scalar(604, 0.0);
        }

        s.v[1011] = if (s.v[629] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1011] != 0.0) {
            s.store_offset(618, 617, s.v[370]);
        }

        if (s.v[1011] != 0.0) {
            s.store_scale_ad(620, A::exp(A::scale(A::sub(A::scale(s.ad_value(619), s.v[363]), A::scale(s.ad_value(618), s.v[365])), 0.5)), ((s.v[360]) as f64).powf(1.5));
        }

        if (s.v[1011] != 0.0) {
            s.store_sub_ad(621, A::scale(s.ad_value(615), s.v[360]), A::scale(A::ln(s.ad_value(620)), (2.0 * s.v[364])));
        }

        if (s.v[1011] != 0.0) {
            s.store_add_ad_rhs(622, 621, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(621)), s.v[365])), 1.0)), s.v[364]));
        }

        if (s.v[1011] != 0.0) {
            s.store_div_from_scalar(623, 1.0, 622);
        }

        if (s.v[1011] != 0.0) {
            s.store_mul_ad_rhs(626, 614, A::pow(A::mul(s.ad_value(615), s.ad_value(623)), s.ad_value(616)));
        }

        if (s.v[1011] != 0.0) {
            s.store_mul_ad_lhs(627, A::mul(s.ad_value(626), s.ad_value(622)), 625);
        }

        if (s.v[1011] != 0.0) {
            s.store_scale(628, 626, 2.0);
        }

        s.v[1] = 1.0;

        s.v[2] = 1.0;

        s.v[306] = 0.0;

        s.v[307] = 0.0;

        s.v[3] = p.p0;

        s.v[4] = p.p1;

        s.v[5] = p.p2;

        s.v[6] = p.p3;

        s.v[7] = p.p4;

        s.v[8] = p.p8;

        s.v[640] = p.p19;

        s.v[641] = p.p20;

        s.v[642] = p.p21;

        s.v[667] = p.p22;

        s.v[668] = p.p23;

        s.v[669] = p.p24;

        s.v[643] = p.p25;

        s.v[644] = p.p26;

        s.v[670] = p.p27;

        s.v[671] = p.p28;

        s.v[10] = p.p14;

        s.v[1012] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1012] != 0.0) {
            s.store_scalar(1, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if (s.v[1012] != 0.0) {
            s.store_floor_ad(1, A::offset(s.ad_value(1), 0.5));
        }

        if (s.v[1012] != 0.0) {
            s.store_div_from_scalar(2, 1.0, 1);
        }

        if ((s.v[4] * s.v[2]) > 1e-9) {
            s.store_scale(4, 2, s.v[4]);
        } else {
            s.store_scalar(4, 1e-9);
        }

        s.v[11] = p.p5;

        s.v[12] = p.p6;

        s.v[13] = p.p7;

        s.v[302] = (1e-6 / s.v[3]);

        s.store_div_from_scalar(303, 1e-6, 4);

        s.store_scale_ad(304, A::offset(A::scale(s.ad_value(303), p.p188), 1.0), (p.p186 * (1.0 + (p.p187 * s.v[302]))));

        s.store_scale_ad(305, A::offset(A::scale(s.ad_value(303), p.p192), 1.0), (p.p190 * (1.0 + (p.p191 * s.v[302]))));

        if (((s.v[3] + s.v[304]) - (2.0 * p.p189)) > 1e-9) {
            s.store_offset(306, 304, ((s.v[3]) + ((-(2.0 * p.p189)))));
        } else {
            s.store_scalar(306, 1e-9);
        }

        if (((s.v[4] + s.v[305]) - (2.0 * p.p193)) > 1e-9) {
            s.store_offset_ad(307, A::add(s.ad_value(4), s.ad_value(305)), (-(2.0 * p.p193)));
        } else {
            s.store_scalar(307, 1e-9);
        }

    }

    pub(super) fn stamp_reactive_block_2(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        s.store_div_from_scalar(308, 1e-6, 306);

        s.store_square(309, 308);

        s.store_div_from_scalar(310, 1e-6, 307);

        s.store_div_from_scalar(311, 1.0, 310);

        s.store_mul(312, 308, 310);

        s.store_div_from_scalar(313, 1.0, 312);

        if ((((s.v[3] + s.v[304]) - (2.0 * p.p189)) + p.p194) > 1e-9) {
            s.store_offset_ad(314, A::offset(A::offset(s.ad_value(304), s.v[3]), (-(2.0 * p.p189))), p.p194);
        } else {
            s.store_scalar(314, 1e-9);
        }

        if ((((s.v[4] + s.v[305]) - (2.0 * p.p193)) + p.p195) > 1e-9) {
            s.store_offset_ad(315, A::offset(A::add(s.ad_value(4), s.ad_value(305)), (-(2.0 * p.p193))), p.p195);
        } else {
            s.store_scalar(315, 1e-9);
        }

        s.store_scale(316, 315, 1000000.0);

        if (((s.v[3] + s.v[304]) + p.p194) > 1e-9) {
            s.store_offset(317, 304, ((s.v[3]) + (p.p194)));
        } else {
            s.store_scalar(317, 1e-9);
        }

        if (((s.v[4] + s.v[305]) + p.p195) > 1e-9) {
            s.store_offset_ad(318, A::add(s.ad_value(4), s.ad_value(305)), p.p195);
        } else {
            s.store_scalar(318, 1e-9);
        }

        s.store_scale(319, 317, 1000000.0);

        s.store_scale(320, 318, 1000000.0);

        s.v[40] = p.p56;

        s.v[41] = p.p57;

        s.v[42] = p.p58;

        s.v[43] = p.p59;

        s.v[44] = p.p60;

        s.v[45] = p.p61;

        s.v[46] = p.p62;

        s.v[47] = p.p63;

        s.v[48] = p.p64;

        s.v[49] = p.p65;

        s.v[50] = p.p66;

        s.v[55] = p.p67;

        s.v[56] = p.p68;

        s.v[57] = p.p69;

        s.v[58] = p.p70;

        s.v[51] = p.p71;

        s.v[52] = p.p73;

        s.v[53] = p.p72;

        s.v[54] = p.p74;

        s.v[59] = p.p78;

        s.v[60] = p.p80;

        s.v[61] = p.p79;

        s.v[62] = p.p75;

        s.v[63] = p.p77;

        s.v[64] = p.p76;

        s.v[65] = p.p81;

        s.v[66] = p.p82;

        s.v[67] = p.p83;

        s.v[68] = p.p84;

        s.v[69] = p.p85;

        s.v[70] = p.p86;

        s.v[71] = p.p87;

        s.v[72] = p.p88;

        s.v[73] = p.p89;

        s.v[74] = p.p90;

        s.v[75] = p.p91;

        s.v[76] = p.p92;

        s.v[77] = p.p93;

        s.v[78] = p.p94;

        s.v[79] = p.p95;

        s.v[80] = p.p96;

        s.v[81] = p.p97;

        s.v[82] = p.p98;

        s.v[83] = p.p99;

        s.v[84] = p.p100;

        s.v[85] = p.p101;

        s.v[86] = p.p102;

        s.v[87] = p.p103;

        s.v[88] = p.p104;

        s.v[89] = p.p105;

        s.v[90] = p.p106;

        s.v[91] = p.p107;

        s.v[92] = p.p108;

        s.v[93] = p.p109;

        s.v[94] = p.p110;

        s.v[95] = p.p111;

        s.v[96] = p.p112;

        s.v[97] = p.p113;

        s.v[98] = p.p114;

        s.v[99] = p.p115;

        s.v[100] = p.p116;

        s.v[101] = p.p117;

        s.v[102] = p.p118;

        s.v[103] = p.p119;

        s.v[104] = p.p120;

        s.v[105] = p.p119;

        s.v[1013] = if (if self.param_given[121] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1013] != 0.0) {
            s.store_scalar(105, p.p121);
        }

        s.v[106] = p.p120;

        s.v[1014] = if (if self.param_given[122] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1014] != 0.0) {
            s.store_scalar(106, p.p122);
        }

        s.copy_ad(107, 105);

        s.v[1015] = if (if self.param_given[123] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1015] != 0.0) {
            s.store_scalar(107, p.p123);
        }

        s.copy_ad(108, 106);

        s.v[1016] = if (if self.param_given[124] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1016] != 0.0) {
            s.store_scalar(108, p.p124);
        }

        s.v[109] = p.p125;

        s.v[110] = p.p126;

        s.v[111] = p.p127;

        s.v[112] = p.p128;

        s.v[113] = p.p129;

        s.v[114] = p.p130;

        s.v[115] = p.p131;

        s.v[116] = p.p132;

        s.v[117] = p.p133;

        s.v[118] = p.p134;

        s.v[119] = p.p135;

        s.v[120] = p.p136;

        s.v[121] = p.p98;

        s.v[1017] = if (if self.param_given[137] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1017] != 0.0) {
            s.store_scalar(121, p.p137);
        }

        s.v[122] = p.p103;

        s.v[1018] = if (if self.param_given[138] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1018] != 0.0) {
            s.store_scalar(122, p.p138);
        }

        s.v[123] = p.p139;

        s.v[124] = p.p140;

        s.v[125] = p.p141;

        s.v[126] = p.p142;

        s.v[127] = p.p143;

        s.v[128] = p.p144;

        s.v[129] = p.p145;

        s.v[130] = p.p146;

        s.v[131] = p.p147;

        s.v[132] = p.p148;

        s.v[133] = p.p149;

        s.v[134] = p.p150;

        s.v[135] = p.p151;

        s.v[136] = p.p152;

        s.v[137] = p.p153;

        s.v[138] = p.p154;

        s.v[139] = p.p155;

        s.v[145] = p.p161;

        s.v[146] = p.p162;

        s.v[147] = p.p163;

        s.v[148] = p.p164;

        s.v[149] = p.p165;

        s.v[150] = p.p166;

        s.v[151] = p.p167;

        s.v[152] = p.p168;

        s.v[153] = p.p169;

        s.v[154] = p.p170;

        s.v[155] = p.p171;

        s.v[156] = p.p173;

        s.v[157] = p.p172;

        s.v[1019] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1019] != 0.0) {
            s.store_add_ad(40, A::add(A::offset(A::scale(A::powf(s.ad_value(308), p.p198), p.p197), p.p196), A::scale(s.ad_value(310), p.p199)), A::scale(s.ad_value(312), p.p200));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(41, A::add(A::offset(A::scale(s.ad_value(308), p.p202), p.p201), A::scale(s.ad_value(310), p.p203)), A::scale(s.ad_value(312), p.p204));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(42, p.p205);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(43, p.p206);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(44, p.p207);
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(325, &A::scale({
                if ((1.0 + ((p.p209 * s.v[310]) * (((1.0 + (s.v[307] / p.p210))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(310), p.p209), A::ln(A::offset(A::scale(s.ad_value(307), 1.0 / (p.p210)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p208));
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(326, &A::scale({
                if ((1.0 + ((p.p212 * s.v[310]) * (((1.0 + (s.v[307] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(310), p.p212), A::ln(A::offset(A::scale(s.ad_value(307), 1.0 / (p.p213)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p211));
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(327, &A::scale({
                if ((1.0 + ((p.p215 * s.v[310]) * (((1.0 + (s.v[307] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(310), p.p215), A::ln(A::offset(A::scale(s.ad_value(307), 1.0 / (p.p213)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p214));
        }

        s.v[1020] = if (s.v[306] > (2.0 * s.v[327])) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1020] != 0.0)) {
            s.store_scalar(328, 75000000000.0);
        }

        if ((s.v[1019] != 0.0) && (s.v[1020] != 0.0)) {
            s.store_sub_ad(329, A::sqrt(A::add(s.ad_value(325), A::scale(s.ad_value(326), 0.5))), A::sqrt(s.ad_value(325)));
        }

        if ((s.v[1019] != 0.0) && (s.v[1020] != 0.0)) {
            s.store_add_ad(330, A::sqrt(s.ad_value(325)), A::mul(s.ad_value(328), A::ln(A::offset(A::mul(A::div(A::scale(s.ad_value(327), 2.0), s.ad_value(306)), A::offset(A::exp(A::div(s.ad_value(329), s.ad_value(328))), (-1.0))), 1.0))));
        }

        if ((s.v[1019] != 0.0) && (s.v[1020] != 0.0)) {
            s.store_square(330, 330);
        }

        s.v[1021] = if (s.v[306] >= s.v[327]) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (!(s.v[1020] != 0.0))) && (s.v[1021] != 0.0)) {
            s.store_add_ad_rhs(330, 325, A::div(A::mul(s.ad_value(326), s.ad_value(327)), s.ad_value(306)));
        }

        if (((s.v[1019] != 0.0) && (!(s.v[1020] != 0.0))) && (!(s.v[1021] != 0.0))) {
            s.store_add_ad_rhs(330, 325, A::mul(s.ad_value(326), A::sub_from_scalar(2.0, A::div(s.ad_value(306), s.ad_value(327)))));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad_rhs(45, 330, A::sub(A::sub_from_scalar(1.0, A::scale(s.ad_value(308), p.p216)), A::scale(s.ad_value(309), p.p217)));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(46, A::add(A::offset(A::scale(A::powf(s.ad_value(308), p.p220), p.p219), p.p218), A::scale(s.ad_value(310), p.p221)), A::scale(s.ad_value(312), p.p222));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(47, p.p223);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(48, p.p224);
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(49, A::add(A::offset(A::scale(A::powf(s.ad_value(308), p.p227), p.p226), p.p225), A::scale(s.ad_value(310), p.p228)), A::scale(s.ad_value(312), p.p229));
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(50, &A::scale({
                if (1e-6 > (1.0 + (p.p231 * s.v[308]))) {
                    A::constant(1e-6)
                } else {
                    A::offset(A::scale(s.ad_value(308), p.p231), 1.0)
                }
            }, p.p230));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(55, p.p232);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(56, p.p233);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(57, p.p236);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(58, p.p237);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(51, A::mul(A::offset(A::scale(A::powf(s.ad_value(308), p.p240), p.p239), p.p238), A::offset(A::scale(s.ad_value(310), p.p241), 1.0)), A::offset(A::scale(s.ad_value(312), p.p242), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(52, p.p244);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(53, p.p243);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(54, p.p245);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(62, A::scale(A::powf(s.ad_value(308), p.p247), p.p246), A::offset(A::scale(s.ad_value(310), p.p248), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(63, p.p250);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(64, p.p249);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(59, A::scale(A::powf(s.ad_value(308), p.p252), p.p251), A::offset(A::scale(s.ad_value(310), p.p253), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(60, p.p255);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(61, p.p254);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale_ad(331, A::offset(A::scale(s.ad_value(310), p.p258), 1.0), p.p257);
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(332, &A::scale({
                if ((1.0 + (p.p260 * s.v[310])) > 0.001) {
                    A::offset(A::scale(s.ad_value(310), p.p260), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p259));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(333, A::offset(A::mul(A::div(A::mul(s.ad_value(331), s.ad_value(332)), s.ad_value(306)), A::sub_from_scalar(1.0, A::exp(A::div(A::neg(s.ad_value(306)), s.ad_value(332))))), 1.0), A::mul(A::div_from_scalar((p.p261 * p.p262), s.ad_value(306)), A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(s.ad_value(306)), 1.0 / (p.p262))))));
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(333, &{
                if (s.v[333] > 1e-15) {
                    s.ad_value(333)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(334, A::offset(A::scale(s.ad_value(310), p.p263), 1.0), A::mul(A::scale(s.ad_value(310), p.p264), A::ln(A::offset(A::scale(s.ad_value(307), 1.0 / (p.p265)), 1.0))));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad_lhs(65, A::div(A::scale(s.ad_value(307), p.p256), A::mul(s.ad_value(333), s.ad_value(306))), 334);
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(66, A::add(A::offset(A::scale(s.ad_value(308), p.p267), p.p266), A::scale(s.ad_value(310), p.p268)), A::scale(s.ad_value(312), p.p269));
        }

    }

    pub(super) fn stamp_reactive_block_3(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[1019] != 0.0) {
            s.store_scale_ad(67, A::offset(A::scale(s.ad_value(310), p.p271), 1.0), p.p270);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(68, p.p272);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(69, p.p273);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(70, p.p274);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(71, A::mul(A::offset(A::scale(A::powf(s.ad_value(308), p.p277), p.p276), p.p275), A::offset(A::scale(s.ad_value(310), p.p278), 1.0)), A::offset(A::scale(s.ad_value(312), p.p279), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(72, p.p280);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(73, p.p281);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(74, p.p282);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(75, A::mul(A::scale(A::offset(A::scale(s.ad_value(308), p.p284), 1.0), p.p283), A::offset(A::scale(s.ad_value(310), p.p285), 1.0)), A::offset(A::scale(s.ad_value(312), p.p286), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(76, p.p287);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(77, p.p288);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(78, A::scale(s.ad_value(310), p.p289), A::offset(A::scale(s.ad_value(310), p.p290), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(79, p.p291);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(80, p.p292);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(81, p.p293);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(82, A::mul(A::offset(A::mul(A::div(A::scale(s.ad_value(334), p.p295), s.ad_value(333)), A::powf(s.ad_value(308), p.p296)), p.p294), A::offset(A::scale(s.ad_value(310), p.p297), 1.0)), A::offset(A::scale(s.ad_value(312), p.p298), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(83, A::add(A::offset(A::scale(s.ad_value(308), p.p300), p.p299), A::scale(s.ad_value(310), p.p301)), A::scale(s.ad_value(312), p.p302));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(84, p.p303);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(85, p.p304);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(86, p.p305);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar_ad(87, p.p306, A::offset(A::scale(s.ad_value(308), p.p307), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(88, A::scale(A::powf(s.ad_value(308), p.p309), p.p308), A::offset(A::scale(s.ad_value(310), p.p310), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_powf(335, 308, p.p312);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_ad(89, A::mul(A::scale(s.ad_value(335), p.p311), A::offset(A::scale(s.ad_value(310), p.p314), 1.0)), A::offset(A::mul(A::scale(s.ad_value(308), p.p313), s.ad_value(335)), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_powf(335, 308, p.p316);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_ad(90, A::mul(A::scale(s.ad_value(335), p.p315), A::offset(A::scale(s.ad_value(310), p.p318), 1.0)), A::offset(A::mul(A::scale(s.ad_value(308), p.p317), s.ad_value(335)), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(91, p.p319);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(92, A::scale(A::offset(A::scale(s.ad_value(308), p.p321), 1.0), p.p320), A::offset(A::scale(s.ad_value(310), p.p322), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(93, p.p323);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(94, p.p324);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(95, A::scale(A::offset(A::scale(s.ad_value(308), p.p326), 1.0), p.p325), A::offset(A::scale(s.ad_value(310), p.p327), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(96, A::scale(A::offset(A::scale(s.ad_value(308), p.p329), 1.0), p.p328), A::offset(A::scale(s.ad_value(310), p.p330), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(97, p.p331);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(98, p.p332);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar(99, p.p333, 312);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar_ad(100, (p.p334 * p.p234), A::scale(s.ad_value(310), 1e-6));
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar_ad(101, (p.p335 * p.p235), A::scale(s.ad_value(310), 1e-6));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(102, p.p336);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(103, p.p337);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(104, p.p338);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(105, p.p337);
        }

        s.v[1022] = if (if self.param_given[339] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1022] != 0.0)) {
            s.store_scalar(105, p.p339);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(106, p.p338);
        }

        s.v[1023] = if (if self.param_given[340] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1023] != 0.0)) {
            s.store_scalar(106, p.p340);
        }

        if (s.v[1019] != 0.0) {
            s.copy_ad(107, 105);
        }

        s.v[1024] = if (if self.param_given[341] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1024] != 0.0)) {
            s.store_scalar(107, p.p341);
        }

        if (s.v[1019] != 0.0) {
            s.copy_ad(108, 106);
        }

        s.v[1025] = if (if self.param_given[342] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1025] != 0.0)) {
            s.store_scalar(108, p.p342);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(109, p.p343);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar_ad(110, (p.p344 * p.p234), A::scale(s.ad_value(310), 1e-6));
        }

        if (s.v[1019] != 0.0) {
            s.store_div_from_scalar_ad(111, (p.p345 * p.p235), A::scale(s.ad_value(310), 1e-6));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(112, p.p346);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(113, p.p347);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(114, p.p348);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(115, p.p349);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(116, p.p350);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(117, p.p351);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale_ad(118, A::mul(A::scale(s.ad_value(315), (8.8541878176e-12 * p.p207)), s.ad_value(314)), 1.0 / (p.p206));
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(125, 315, ((8.8541878176e-12 * p.p207) * (p.p234 * 1.0 / (p.p232))));
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(126, 315, ((8.8541878176e-12 * p.p207) * (p.p235 * 1.0 / (p.p233))));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(119, A::add(A::offset(A::scale(A::powf(s.ad_value(308), p.p354), p.p353), p.p352), A::scale(s.ad_value(310), p.p355)), A::scale(s.ad_value(312), p.p356));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(120, A::add(A::offset(A::scale(s.ad_value(308), p.p358), p.p357), A::scale(s.ad_value(310), p.p359)), A::scale(s.ad_value(312), p.p360));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(32, p.p294);
        }

        s.v[1026] = if (if self.param_given[361] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1026] != 0.0)) {
            s.store_scalar(32, p.p361);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(33, p.p295);
        }

        s.v[1027] = if (if self.param_given[362] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1027] != 0.0)) {
            s.store_scalar(33, p.p362);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(34, p.p296);
        }

        s.v[1028] = if (if self.param_given[363] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1028] != 0.0)) {
            s.store_scalar(34, p.p363);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(35, p.p297);
        }

        s.v[1029] = if (if self.param_given[364] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1029] != 0.0)) {
            s.store_scalar(35, p.p364);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(36, p.p298);
        }

        s.v[1030] = if (if self.param_given[365] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1030] != 0.0)) {
            s.store_scalar(36, p.p365);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(121, A::mul(A::add(s.ad_value(32), A::mul(A::div(A::mul(s.ad_value(33), s.ad_value(334)), s.ad_value(333)), A::pow(s.ad_value(308), s.ad_value(34)))), A::offset(A::mul(s.ad_value(35), s.ad_value(310)), 1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(312)), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(37, p.p306);
        }

        s.v[1031] = if (if self.param_given[366] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1031] != 0.0)) {
            s.store_scalar(37, p.p366);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(38, p.p307);
        }

        s.v[1032] = if (if self.param_given[367] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1032] != 0.0)) {
            s.store_scalar(38, p.p367);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_ad_rhs(122, 37, A::offset(A::mul(s.ad_value(38), s.ad_value(308)), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(123, A::scale(A::powf(s.ad_value(308), p.p369), p.p368), A::offset(A::scale(s.ad_value(310), p.p370), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_powf(335, 308, p.p372);
        }

        if (s.v[1019] != 0.0) {
            s.store_div_ad(124, A::mul(A::scale(s.ad_value(335), p.p371), A::offset(A::scale(s.ad_value(310), p.p374), 1.0)), A::offset(A::mul(A::scale(s.ad_value(308), p.p373), s.ad_value(335)), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(127, p.p375);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(128, p.p376);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(129, p.p377);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(130, 319, p.p378);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(131, 316, p.p379);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(132, 316, p.p380);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(133, p.p381);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(134, p.p382);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(135, p.p383);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(136, p.p384);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(137, 320, p.p385);
        }

        if (s.v[1019] != 0.0) {
            s.store_scale(138, 320, p.p386);
        }

        if (s.v[1019] != 0.0) {
            s.store_sub_from_scalar_ad(1001, 1.0, A::div_from_scalar((2.0 * p.p393), s.ad_value(306)));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(139, p.p387);
        }

        if (s.v[1019] != 0.0) {
            s.store_offset_scaled(338, 307, p.p396, (2.0 * p.p395));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(145, p.p397);
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(146, A::add(A::offset(A::scale(s.ad_value(308), p.p399), p.p398), A::scale(s.ad_value(310), p.p400)), A::scale(s.ad_value(312), p.p401));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(147, A::add(A::offset(A::scale(A::powf(s.ad_value(308), p.p404), p.p403), p.p402), A::scale(s.ad_value(310), p.p405)), A::scale(s.ad_value(312), p.p406));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(148, A::mul(A::scale(A::offset(A::scale(A::powf(s.ad_value(308), p.p409), p.p408), 1.0), p.p407), A::offset(A::scale(s.ad_value(310), p.p410), 1.0)), A::offset(A::scale(s.ad_value(312), p.p411), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_offset_ad(149, A::scale(A::powf(s.ad_value(308), p.p414), p.p413), p.p412);
        }

        if (s.v[1019] != 0.0) {
            s.store_offset_ad(341, A::mul(A::div_from_scalar((p.p415 * p.p416), s.ad_value(306)), A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(s.ad_value(306)), 1.0 / (p.p416))))), 1.0);
        }

        if (s.v[1019] != 0.0) {
            s.store_ad(341, &{
                if (s.v[341] > 1e-15) {
                    s.ad_value(341)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(150, A::div(A::scale(s.ad_value(338), p.p256), A::mul(s.ad_value(341), s.ad_value(306))), A::offset(A::scale(s.ad_value(310), p.p417), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(151, A::add(A::offset(A::scale(s.ad_value(308), p.p419), p.p418), A::scale(s.ad_value(310), p.p420)), A::scale(s.ad_value(312), p.p421));
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(152, A::scale(A::powf(s.ad_value(308), p.p423), p.p422), A::offset(A::scale(s.ad_value(310), p.p424), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(153, p.p425);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(154, p.p426);
        }

        if (s.v[1019] != 0.0) {
            s.store_mul_ad(155, A::scale(A::powf(s.ad_value(308), p.p428), p.p427), A::offset(A::scale(s.ad_value(310), p.p429), 1.0));
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(156, p.p431);
        }

        if (s.v[1019] != 0.0) {
            s.store_scalar(157, p.p430);
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(342, A::add(A::offset(A::scale(s.ad_value(308), p.p808), p.p807), A::scale(s.ad_value(310), p.p809)), A::scale(s.ad_value(312), p.p810));
        }

        if (s.v[1019] != 0.0) {
            s.store_add_ad(343, A::add(A::offset(A::scale(s.ad_value(308), p.p812), p.p811), A::scale(s.ad_value(310), p.p813)), A::scale(s.ad_value(312), p.p814));
        }

        s.v[1034] = if ((((if self.param_given[448] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[449] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[450] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[451] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1034] != 0.0)) {
            s.store_add_ad(40, A::add(A::offset(A::scale(s.ad_value(308), p.p449), p.p448), A::scale(s.ad_value(310), p.p450)), A::scale(s.ad_value(312), p.p451));
        }

        s.v[1035] = if ((((if self.param_given[452] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[453] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[454] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[455] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1035] != 0.0)) {
            s.store_add_ad(41, A::add(A::offset(A::scale(s.ad_value(308), p.p453), p.p452), A::scale(s.ad_value(310), p.p454)), A::scale(s.ad_value(312), p.p455));
        }

        s.v[1036] = if ((((if self.param_given[456] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[457] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[458] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[459] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1036] != 0.0)) {
            s.store_add_ad(45, A::add(A::offset(A::scale(s.ad_value(308), p.p457), p.p456), A::scale(s.ad_value(310), p.p458)), A::scale(s.ad_value(312), p.p459));
        }

        s.v[1037] = if ((((if self.param_given[460] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[461] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[462] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[463] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1037] != 0.0)) {
            s.store_add_ad(46, A::add(A::offset(A::scale(s.ad_value(308), p.p461), p.p460), A::scale(s.ad_value(310), p.p462)), A::scale(s.ad_value(312), p.p463));
        }

        s.v[1038] = if ((((if self.param_given[464] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[465] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[466] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[467] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1038] != 0.0)) {
            s.store_add_ad(47, A::add(A::offset(A::scale(s.ad_value(308), p.p465), p.p464), A::scale(s.ad_value(310), p.p466)), A::scale(s.ad_value(312), p.p467));
        }

        s.v[1039] = if ((((if self.param_given[468] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[469] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[470] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[471] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_reactive_block_4(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedReactiveStamper<'_>,
        s: &mut ReactiveScratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[1019] != 0.0) && (s.v[1039] != 0.0)) {
            s.store_add_ad(49, A::add(A::offset(A::scale(s.ad_value(308), p.p469), p.p468), A::scale(s.ad_value(310), p.p470)), A::scale(s.ad_value(312), p.p471));
        }

        s.v[1040] = if ((((if self.param_given[472] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[473] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[474] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[475] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1040] != 0.0)) {
            s.store_add_ad(50, A::add(A::offset(A::scale(s.ad_value(308), p.p473), p.p472), A::scale(s.ad_value(310), p.p474)), A::scale(s.ad_value(312), p.p475));
        }

        s.v[1041] = if ((((if self.param_given[476] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[477] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[478] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[479] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1041] != 0.0)) {
            s.store_add_ad(57, A::add(A::offset(A::scale(s.ad_value(308), p.p477), p.p476), A::scale(s.ad_value(310), p.p478)), A::scale(s.ad_value(312), p.p479));
        }

        s.v[1042] = if ((((if self.param_given[480] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[481] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[482] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[483] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1042] != 0.0)) {
            s.store_add_ad(58, A::add(A::offset(A::scale(s.ad_value(308), p.p481), p.p480), A::scale(s.ad_value(310), p.p482)), A::scale(s.ad_value(312), p.p483));
        }

        s.v[1043] = if ((((if self.param_given[484] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[485] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[486] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[487] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1043] != 0.0)) {
            s.store_add_ad(51, A::add(A::offset(A::scale(s.ad_value(308), p.p485), p.p484), A::scale(s.ad_value(310), p.p486)), A::scale(s.ad_value(312), p.p487));
        }

        s.v[1044] = if ((((if self.param_given[492] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[493] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[494] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[495] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1044] != 0.0)) {
            s.store_add_ad(52, A::add(A::offset(A::scale(s.ad_value(308), p.p493), p.p492), A::scale(s.ad_value(310), p.p494)), A::scale(s.ad_value(312), p.p495));
        }

        s.v[1045] = if ((((if self.param_given[488] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[489] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[490] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[491] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1045] != 0.0)) {
            s.store_add_ad(53, A::add(A::offset(A::scale(s.ad_value(308), p.p489), p.p488), A::scale(s.ad_value(310), p.p490)), A::scale(s.ad_value(312), p.p491));
        }

        s.v[1046] = if ((((if self.param_given[496] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[497] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[498] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[499] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1046] != 0.0)) {
            s.store_add_ad(54, A::add(A::offset(A::scale(s.ad_value(308), p.p497), p.p496), A::scale(s.ad_value(310), p.p498)), A::scale(s.ad_value(312), p.p499));
        }

        s.v[1047] = if ((((if self.param_given[500] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[501] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[502] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[503] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1047] != 0.0)) {
            s.store_mul_ad_rhs(62, 309, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p501), p.p500), A::scale(s.ad_value(310), p.p502)), A::scale(s.ad_value(312), p.p503)));
        }

        s.v[1048] = if ((((if self.param_given[508] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[509] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[510] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[511] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1048] != 0.0)) {
            s.store_add_ad(63, A::add(A::offset(A::scale(s.ad_value(308), p.p509), p.p508), A::scale(s.ad_value(310), p.p510)), A::scale(s.ad_value(312), p.p511));
        }

        s.v[1049] = if ((((if self.param_given[504] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[505] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[506] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[507] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1049] != 0.0)) {
            s.store_add_ad(64, A::add(A::offset(A::scale(s.ad_value(308), p.p505), p.p504), A::scale(s.ad_value(310), p.p506)), A::scale(s.ad_value(312), p.p507));
        }

        s.v[1050] = if ((((if self.param_given[512] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[513] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[514] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[515] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1050] != 0.0)) {
            s.store_mul_ad_rhs(59, 309, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p513), p.p512), A::scale(s.ad_value(310), p.p514)), A::scale(s.ad_value(312), p.p515)));
        }

        s.v[1051] = if ((((if self.param_given[520] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[521] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[522] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[523] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1051] != 0.0)) {
            s.store_add_ad(60, A::add(A::offset(A::scale(s.ad_value(308), p.p521), p.p520), A::scale(s.ad_value(310), p.p522)), A::scale(s.ad_value(312), p.p523));
        }

        s.v[1052] = if ((((if self.param_given[516] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[517] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[518] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[519] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1052] != 0.0)) {
            s.store_add_ad(61, A::add(A::offset(A::scale(s.ad_value(308), p.p517), p.p516), A::scale(s.ad_value(310), p.p518)), A::scale(s.ad_value(312), p.p519));
        }

        s.v[1053] = if ((((if self.param_given[524] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[525] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[526] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[527] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1053] != 0.0)) {
            s.store_mul_ad(65, A::div(s.ad_value(307), s.ad_value(306)), A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p525), p.p524), A::scale(s.ad_value(310), p.p526)), A::scale(s.ad_value(312), p.p527)));
        }

        s.v[1054] = if ((((if self.param_given[528] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[529] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[530] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[531] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1054] != 0.0)) {
            s.store_add_ad(66, A::add(A::offset(A::scale(s.ad_value(308), p.p529), p.p528), A::scale(s.ad_value(310), p.p530)), A::scale(s.ad_value(312), p.p531));
        }

        s.v[1055] = if ((((if self.param_given[532] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[533] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[534] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[535] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1055] != 0.0)) {
            s.store_add_ad(67, A::add(A::offset(A::scale(s.ad_value(308), p.p533), p.p532), A::scale(s.ad_value(310), p.p534)), A::scale(s.ad_value(312), p.p535));
        }

        s.v[1056] = if ((((if self.param_given[536] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[537] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[538] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[539] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1056] != 0.0)) {
            s.store_add_ad(69, A::add(A::offset(A::scale(s.ad_value(308), p.p537), p.p536), A::scale(s.ad_value(310), p.p538)), A::scale(s.ad_value(312), p.p539));
        }

        s.v[1057] = if ((((if self.param_given[540] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[541] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[542] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[543] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1057] != 0.0)) {
            s.store_add_ad(71, A::add(A::offset(A::scale(s.ad_value(308), p.p541), p.p540), A::scale(s.ad_value(310), p.p542)), A::scale(s.ad_value(312), p.p543));
        }

        s.v[1058] = if ((((if self.param_given[544] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[545] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[546] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[547] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1058] != 0.0)) {
            s.store_add_ad(73, A::add(A::offset(A::scale(s.ad_value(308), p.p545), p.p544), A::scale(s.ad_value(310), p.p546)), A::scale(s.ad_value(312), p.p547));
        }

        s.v[1059] = if ((((if self.param_given[548] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[549] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[550] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[551] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1059] != 0.0)) {
            s.store_add_ad(75, A::add(A::offset(A::scale(s.ad_value(308), p.p549), p.p548), A::scale(s.ad_value(310), p.p550)), A::scale(s.ad_value(312), p.p551));
        }

        s.v[1060] = if ((((if self.param_given[552] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[553] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[554] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[555] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1060] != 0.0)) {
            s.store_mul_ad_rhs(78, 310, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p553), p.p552), A::scale(s.ad_value(310), p.p554)), A::scale(s.ad_value(312), p.p555)));
        }

        s.v[1061] = if ((((if self.param_given[556] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[557] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[558] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[559] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1061] != 0.0)) {
            s.store_add_ad(79, A::add(A::offset(A::scale(s.ad_value(308), p.p557), p.p556), A::scale(s.ad_value(310), p.p558)), A::scale(s.ad_value(312), p.p559));
        }

        s.v[1062] = if ((((if self.param_given[560] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[561] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[562] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[563] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1062] != 0.0)) {
            s.store_add_ad(80, A::add(A::offset(A::scale(s.ad_value(308), p.p561), p.p560), A::scale(s.ad_value(310), p.p562)), A::scale(s.ad_value(312), p.p563));
        }

        s.v[1063] = if ((((if self.param_given[564] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[565] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[566] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[567] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1063] != 0.0)) {
            s.store_add_ad(81, A::add(A::offset(A::scale(s.ad_value(308), p.p565), p.p564), A::scale(s.ad_value(310), p.p566)), A::scale(s.ad_value(312), p.p567));
        }

        s.v[1064] = if ((((if self.param_given[568] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[569] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[570] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[571] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1064] != 0.0)) {
            s.store_mul_ad_rhs(82, 308, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p569), p.p568), A::scale(s.ad_value(310), p.p570)), A::scale(s.ad_value(312), p.p571)));
        }

        s.v[1065] = if ((((if self.param_given[572] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[573] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[574] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[575] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1065] != 0.0)) {
            s.store_add_ad(83, A::add(A::offset(A::scale(s.ad_value(308), p.p573), p.p572), A::scale(s.ad_value(310), p.p574)), A::scale(s.ad_value(312), p.p575));
        }

        s.v[1066] = if ((((if self.param_given[576] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[577] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[578] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[579] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1066] != 0.0)) {
            s.store_add_ad(84, A::add(A::offset(A::scale(s.ad_value(308), p.p577), p.p576), A::scale(s.ad_value(310), p.p578)), A::scale(s.ad_value(312), p.p579));
        }

        s.v[1067] = if ((((if self.param_given[580] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[581] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[582] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[583] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1067] != 0.0)) {
            s.store_add_ad(85, A::add(A::offset(A::scale(s.ad_value(308), p.p581), p.p580), A::scale(s.ad_value(310), p.p582)), A::scale(s.ad_value(312), p.p583));
        }

        s.v[1068] = if ((((if self.param_given[584] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[585] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[586] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[587] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1068] != 0.0)) {
            s.store_add_ad(87, A::add(A::offset(A::scale(s.ad_value(308), p.p585), p.p584), A::scale(s.ad_value(310), p.p586)), A::scale(s.ad_value(312), p.p587));
        }

        s.v[1069] = if ((((if self.param_given[588] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[589] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[590] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[591] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1069] != 0.0)) {
            s.store_mul_ad_rhs(88, 308, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p589), p.p588), A::scale(s.ad_value(310), p.p590)), A::scale(s.ad_value(312), p.p591)));
        }

        s.v[1070] = if ((((if self.param_given[592] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[593] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[594] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[595] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1070] != 0.0)) {
            s.store_add_ad(89, A::add(A::offset(A::scale(s.ad_value(308), p.p593), p.p592), A::scale(s.ad_value(310), p.p594)), A::scale(s.ad_value(312), p.p595));
        }

        s.v[1071] = if ((((if self.param_given[596] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[597] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[598] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[599] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1071] != 0.0)) {
            s.store_add_ad(90, A::add(A::offset(A::scale(s.ad_value(308), p.p597), p.p596), A::scale(s.ad_value(310), p.p598)), A::scale(s.ad_value(312), p.p599));
        }

        s.v[1072] = if ((((if self.param_given[600] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[601] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[602] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[603] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1072] != 0.0)) {
            s.store_add_ad(92, A::add(A::offset(A::scale(s.ad_value(308), p.p601), p.p600), A::scale(s.ad_value(310), p.p602)), A::scale(s.ad_value(312), p.p603));
        }

        s.v[1073] = if ((((if self.param_given[604] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[605] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[606] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[607] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1073] != 0.0)) {
            s.store_add_ad(94, A::add(A::offset(A::scale(s.ad_value(308), p.p605), p.p604), A::scale(s.ad_value(310), p.p606)), A::scale(s.ad_value(312), p.p607));
        }

        s.v[1074] = if ((((if self.param_given[608] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[609] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[610] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[611] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1074] != 0.0)) {
            s.store_add_ad(95, A::add(A::offset(A::scale(s.ad_value(308), p.p609), p.p608), A::scale(s.ad_value(310), p.p610)), A::scale(s.ad_value(312), p.p611));
        }

        s.v[1075] = if ((((if self.param_given[612] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[613] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[614] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[615] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1075] != 0.0)) {
            s.store_add_ad(96, A::add(A::offset(A::scale(s.ad_value(308), p.p613), p.p612), A::scale(s.ad_value(310), p.p614)), A::scale(s.ad_value(312), p.p615));
        }

        s.v[1076] = if ((((if self.param_given[616] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[617] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[618] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[619] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1076] != 0.0)) {
            s.store_mul_ad_rhs(99, 313, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p617), p.p616), A::scale(s.ad_value(310), p.p618)), A::scale(s.ad_value(312), p.p619)));
        }

        s.v[1077] = if ((((if self.param_given[620] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[621] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[622] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[623] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1077] != 0.0)) {
            s.store_mul_ad_rhs(100, 311, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p621), p.p620), A::scale(s.ad_value(310), p.p622)), A::scale(s.ad_value(312), p.p623)));
        }

        s.v[1078] = if ((((if self.param_given[624] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[625] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[626] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[627] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1078] != 0.0)) {
            s.store_mul_ad_rhs(101, 311, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p625), p.p624), A::scale(s.ad_value(310), p.p626)), A::scale(s.ad_value(312), p.p627)));
        }

        s.v[1079] = if ((((if self.param_given[628] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[629] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[630] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[631] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1079] != 0.0)) {
            s.store_add_ad(102, A::add(A::offset(A::scale(s.ad_value(308), p.p629), p.p628), A::scale(s.ad_value(310), p.p630)), A::scale(s.ad_value(312), p.p631));
        }

        s.v[1080] = if ((((if self.param_given[632] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[633] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[634] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[635] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1080] != 0.0)) {
            s.store_mul_ad_rhs(110, 311, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p633), p.p632), A::scale(s.ad_value(310), p.p634)), A::scale(s.ad_value(312), p.p635)));
        }

        s.v[1081] = if ((((if self.param_given[636] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[637] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[638] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[639] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1081] != 0.0)) {
            s.store_mul_ad_rhs(111, 311, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p637), p.p636), A::scale(s.ad_value(310), p.p638)), A::scale(s.ad_value(312), p.p639)));
        }

        s.v[1082] = if ((((if self.param_given[640] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[641] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[642] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[643] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1082] != 0.0)) {
            s.store_add_ad(114, A::add(A::offset(A::scale(s.ad_value(308), p.p641), p.p640), A::scale(s.ad_value(310), p.p642)), A::scale(s.ad_value(312), p.p643));
        }

        s.v[1083] = if ((((if self.param_given[644] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[645] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[646] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[647] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1083] != 0.0)) {
            s.store_add_ad(115, A::add(A::offset(A::scale(s.ad_value(308), p.p645), p.p644), A::scale(s.ad_value(310), p.p646)), A::scale(s.ad_value(312), p.p647));
        }

        s.v[1084] = if ((((if self.param_given[648] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[649] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[650] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[651] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_mul_ad(118, A::scale(A::mul(s.ad_value(316), s.ad_value(314)), 1000000.0), A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p649), p.p648), A::scale(s.ad_value(310), p.p650)), A::scale(s.ad_value(312), p.p651)));
        }

        s.v[1085] = if ((((if self.param_given[652] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[653] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[654] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[655] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1085] != 0.0)) {
            s.store_add_ad(119, A::add(A::offset(A::scale(s.ad_value(308), p.p653), p.p652), A::scale(s.ad_value(310), p.p654)), A::scale(s.ad_value(312), p.p655));
        }

        s.v[1086] = if ((((if self.param_given[656] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[657] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[658] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[659] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1086] != 0.0)) {
            s.store_add_ad(120, A::add(A::offset(A::scale(s.ad_value(308), p.p657), p.p656), A::scale(s.ad_value(310), p.p658)), A::scale(s.ad_value(312), p.p659));
        }

        s.v[1087] = if ((((((((if self.param_given[660] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[661] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[662] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[663] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[568] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[569] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[570] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[571] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) {
            s.store_scalar(28, p.p568);
        }

        s.v[1088] = if (if self.param_given[660] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) && (s.v[1088] != 0.0)) {
            s.store_scalar(28, p.p660);
        }

        if ((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) {
            s.store_scalar(29, p.p569);
        }

        s.v[1089] = if (if self.param_given[661] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) && (s.v[1089] != 0.0)) {
            s.store_scalar(29, p.p661);
        }

        if ((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) {
            s.store_scalar(30, p.p570);
        }

        s.v[1090] = if (if self.param_given[662] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) && (s.v[1090] != 0.0)) {
            s.store_scalar(30, p.p662);
        }

        if ((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) {
            s.store_scalar(31, p.p571);
        }

        s.v[1091] = if (if self.param_given[663] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) && (s.v[1091] != 0.0)) {
            s.store_scalar(31, p.p663);
        }

        if ((s.v[1019] != 0.0) && (s.v[1087] != 0.0)) {
            s.store_mul_ad_rhs(121, 308, A::add(A::add(A::add(s.ad_value(28), A::mul(s.ad_value(29), s.ad_value(308))), A::mul(s.ad_value(30), s.ad_value(310))), A::mul(s.ad_value(31), s.ad_value(312))));
        }

        s.v[1092] = if ((((((((if self.param_given[664] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[665] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[666] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[667] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[584] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[585] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[586] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[587] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) {
            s.store_scalar(28, p.p584);
        }

        s.v[1093] = if (if self.param_given[664] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) && (s.v[1093] != 0.0)) {
            s.store_scalar(28, p.p664);
        }

        if ((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) {
            s.store_scalar(29, p.p585);
        }

        s.v[1094] = if (if self.param_given[665] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) && (s.v[1094] != 0.0)) {
            s.store_scalar(29, p.p665);
        }

        if ((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) {
            s.store_scalar(30, p.p586);
        }

        s.v[1095] = if (if self.param_given[666] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) && (s.v[1095] != 0.0)) {
            s.store_scalar(30, p.p666);
        }

        if ((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) {
            s.store_scalar(31, p.p587);
        }

        s.v[1096] = if (if self.param_given[667] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) && (s.v[1096] != 0.0)) {
            s.store_scalar(31, p.p667);
        }

        if ((s.v[1019] != 0.0) && (s.v[1092] != 0.0)) {
            s.store_add_ad(122, A::add(A::add(s.ad_value(28), A::mul(s.ad_value(29), s.ad_value(308))), A::mul(s.ad_value(30), s.ad_value(310))), A::mul(s.ad_value(31), s.ad_value(312)));
        }

        s.v[1097] = if ((((if self.param_given[668] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[669] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[670] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[671] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_mul_ad_rhs(123, 308, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p669), p.p668), A::scale(s.ad_value(310), p.p670)), A::scale(s.ad_value(312), p.p671)));
        }

        s.v[1098] = if ((((if self.param_given[672] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[673] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[674] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[675] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_mul_ad_rhs(124, 308, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p673), p.p672), A::scale(s.ad_value(310), p.p674)), A::scale(s.ad_value(312), p.p675)));
        }

        s.v[1099] = if ((((if self.param_given[676] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[677] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[678] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[679] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1099] != 0.0)) {
            s.store_mul_ad_rhs(125, 316, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p677), p.p676), A::scale(s.ad_value(310), p.p678)), A::scale(s.ad_value(312), p.p679)));
        }

        s.v[1100] = if ((((if self.param_given[680] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[681] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[682] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[683] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1100] != 0.0)) {
            s.store_mul_ad_rhs(126, 316, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p681), p.p680), A::scale(s.ad_value(310), p.p682)), A::scale(s.ad_value(312), p.p683)));
        }

        s.v[1101] = if ((((if self.param_given[684] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[685] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[686] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[687] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1101] != 0.0)) {
            s.store_mul_ad_rhs(130, 319, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p685), p.p684), A::scale(s.ad_value(310), p.p686)), A::scale(s.ad_value(312), p.p687)));
        }

        s.v[1102] = if ((((if self.param_given[688] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[689] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[690] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[691] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1102] != 0.0)) {
            s.store_mul_ad_rhs(131, 316, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p689), p.p688), A::scale(s.ad_value(310), p.p690)), A::scale(s.ad_value(312), p.p691)));
        }

        s.v[1103] = if ((((if self.param_given[692] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[693] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[694] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[695] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1103] != 0.0)) {
            s.store_mul_ad_rhs(132, 316, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p693), p.p692), A::scale(s.ad_value(310), p.p694)), A::scale(s.ad_value(312), p.p695)));
        }

        s.v[1104] = if ((((if self.param_given[696] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[697] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[698] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[699] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1104] != 0.0)) {
            s.store_mul_ad_rhs(137, 320, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p697), p.p696), A::scale(s.ad_value(310), p.p698)), A::scale(s.ad_value(312), p.p699)));
        }

        s.v[1105] = if ((((if self.param_given[700] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[701] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[702] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[703] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1105] != 0.0)) {
            s.store_mul_ad_rhs(138, 320, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p701), p.p700), A::scale(s.ad_value(310), p.p702)), A::scale(s.ad_value(312), p.p703)));
        }

        s.v[1110] = if ((((if self.param_given[720] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[721] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[722] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[723] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1110] != 0.0)) {
            s.store_add_ad(145, A::add(A::offset(A::scale(s.ad_value(308), p.p721), p.p720), A::scale(s.ad_value(310), p.p722)), A::scale(s.ad_value(312), p.p723));
        }

        s.v[1111] = if ((((if self.param_given[724] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[725] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[726] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[727] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1111] != 0.0)) {
            s.store_add_ad(146, A::add(A::offset(A::scale(s.ad_value(308), p.p725), p.p724), A::scale(s.ad_value(310), p.p726)), A::scale(s.ad_value(312), p.p727));
        }

        s.v[1112] = if ((((if self.param_given[728] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[729] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[730] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[731] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1112] != 0.0)) {
            s.store_add_ad(147, A::add(A::offset(A::scale(s.ad_value(308), p.p729), p.p728), A::scale(s.ad_value(310), p.p730)), A::scale(s.ad_value(312), p.p731));
        }

        s.v[1113] = if ((((if self.param_given[732] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[733] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[734] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[735] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1113] != 0.0)) {
            s.store_add_ad(148, A::add(A::offset(A::scale(s.ad_value(308), p.p733), p.p732), A::scale(s.ad_value(310), p.p734)), A::scale(s.ad_value(312), p.p735));
        }

        s.v[1114] = if ((((if self.param_given[736] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[737] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[738] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[739] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1114] != 0.0)) {
            s.store_add_ad(149, A::add(A::offset(A::scale(s.ad_value(308), p.p737), p.p736), A::scale(s.ad_value(310), p.p738)), A::scale(s.ad_value(312), p.p739));
        }

        s.v[1115] = if ((((if self.param_given[740] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[741] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[742] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[743] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1115] != 0.0)) {
            s.store_mul_ad(150, A::div(s.ad_value(338), s.ad_value(306)), A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p741), p.p740), A::scale(s.ad_value(310), p.p742)), A::scale(s.ad_value(312), p.p743)));
        }

        s.v[1116] = if ((((if self.param_given[744] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[745] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[746] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[747] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1116] != 0.0)) {
            s.store_add_ad(151, A::add(A::offset(A::scale(s.ad_value(308), p.p745), p.p744), A::scale(s.ad_value(310), p.p746)), A::scale(s.ad_value(312), p.p747));
        }

        s.v[1117] = if ((((if self.param_given[748] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[749] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[750] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[751] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1117] != 0.0)) {
            s.store_mul_ad_rhs(152, 309, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p749), p.p748), A::scale(s.ad_value(310), p.p750)), A::scale(s.ad_value(312), p.p751)));
        }

        s.v[1118] = if ((((if self.param_given[752] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[753] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[754] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[755] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1118] != 0.0)) {
            s.store_add_ad(153, A::add(A::offset(A::scale(s.ad_value(308), p.p753), p.p752), A::scale(s.ad_value(310), p.p754)), A::scale(s.ad_value(312), p.p755));
        }

        s.v[1119] = if ((((if self.param_given[756] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[757] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[758] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[759] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1119] != 0.0)) {
            s.store_add_ad(154, A::add(A::offset(A::scale(s.ad_value(308), p.p757), p.p756), A::scale(s.ad_value(310), p.p758)), A::scale(s.ad_value(312), p.p759));
        }

        s.v[1120] = if ((((if self.param_given[760] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[761] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[762] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[763] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1120] != 0.0)) {
            s.store_mul_ad_rhs(155, 309, A::add(A::add(A::offset(A::scale(s.ad_value(308), p.p761), p.p760), A::scale(s.ad_value(310), p.p762)), A::scale(s.ad_value(312), p.p763)));
        }

        s.v[1121] = if ((((if self.param_given[768] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[769] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[770] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[771] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1121] != 0.0)) {
            s.store_add_ad(156, A::add(A::offset(A::scale(s.ad_value(308), p.p769), p.p768), A::scale(s.ad_value(310), p.p770)), A::scale(s.ad_value(312), p.p771));
        }

        s.v[1122] = if ((((if self.param_given[764] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[765] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[766] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[767] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1019] != 0.0) && (s.v[1122] != 0.0)) {
            s.store_add_ad(157, A::add(A::offset(A::scale(s.ad_value(308), p.p765), p.p764), A::scale(s.ad_value(310), p.p766)), A::scale(s.ad_value(312), p.p767));
        }

    }
}
