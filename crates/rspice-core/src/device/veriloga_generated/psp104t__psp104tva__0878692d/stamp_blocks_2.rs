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
        if (((s.v[2255] != 0.0) && (s.v[2256] != 0.0)) && (s.v[2259] != 0.0)) {
            s.store_offset_ad(1929, A::div(A::scale(s.ad_value(836), 2.0), s.ad_value(232)), (-1.0));
        }

        if (((s.v[2255] != 0.0) && (s.v[2256] != 0.0)) && (s.v[2259] != 0.0)) {
            s.store_mul_ad(836, A::scale(s.ad_value(232), 0.5), A::offset(A::div(s.ad_value(1929), A::sqrt(A::offset(A::square(s.ad_value(1929)), 1.0))), 1.0));
        }

        s.v[2453] = if (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };

        s.v[2454] = if ((p.p45 > 0.0) || (p.p47 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.copy_ad(2294, 717);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.copy_ad(2295, 727);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.copy_ad(2296, 718);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.copy_ad(2297, 1804);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.copy_ad(2298, 1805);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2302, 0.0);
        }

        s.v[2455] = if (p.p47 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2455] != 0.0)) {
            s.store_add_ad_lhs(2297, A::scale(A::sub(A::add(s.ad_value(817), s.ad_value(816)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(817), s.ad_value(816)), A::sub(s.ad_value(817), s.ad_value(816))), s.ad_value(738)))), 0.5), 736);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2455] != 0.0)) {
            s.store_add_ad_lhs(1870, A::sub(s.ad_value(816), A::scale(A::sub(s.ad_value(2297), A::sqrt(A::add(A::mul(s.ad_value(2297), s.ad_value(2297)), s.ad_value(737)))), 0.5)), 739);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2455] != 0.0)) {
            s.copy_ad(2298, 1870);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2455] != 0.0)) {
            s.copy_ad(2294, 734);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2455] != 0.0)) {
            s.copy_ad(2295, 737);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2455] != 0.0)) {
            s.copy_ad(2296, 735);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_sub_ad_lhs(2301, A::sub(s.ad_value(818), s.ad_value(2302)), 701);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_add_ad_rhs(2303, 2298, A::scale(A::sub(s.ad_value(815), s.ad_value(819)), 0.5));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2315, 1.0);
        }

        s.v[2456] = if (s.v[188] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_mul(2306, 2294, 362);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_mul(2307, 2303, 362);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_mul(2308, 2301, 362);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_offset_ad(1930, A::div(A::scale(s.ad_value(2296), 0.5), A::sqrt(s.ad_value(2306))), 1.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_add_ad_rhs(1931, 2306, A::mul(s.ad_value(2296), A::sqrt(s.ad_value(2306))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_sub_ad(2309, A::add(A::div(A::sub(s.ad_value(2308), s.ad_value(1931)), s.ad_value(1930)), A::scale(s.ad_value(2306), 0.5)), A::mul(A::offset(s.ad_value(189), 1.0), s.ad_value(2307)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_offset_scaled(2310, 2306, 0.5, 2.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_add(2311, 2306, 2307);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_sub_ad(1930, A::sub(A::sub(s.ad_value(2308), s.ad_value(2311)), A::mul(s.ad_value(2296), A::sqrt(s.ad_value(2311)))), A::scale(A::ln(A::add(A::div(s.ad_value(2306), s.ad_value(2296)), A::sqrt(s.ad_value(2306)))), 2.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_add_ad_lhs(2312, A::scale(s.ad_value(1930), 2.0), 2310);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_scale_ad(1930, A::add(A::add(s.ad_value(2309), s.ad_value(2312)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2309), s.ad_value(2312)), A::sub(s.ad_value(2309), s.ad_value(2312))), 20.0))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_sub_ad_lhs(1931, A::scale(A::sub(s.ad_value(2308), s.ad_value(2307)), 2.0), 2310);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_scale_ad(2313, A::sub(A::add(s.ad_value(1930), s.ad_value(1931)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1930), s.ad_value(1931)), A::sub(s.ad_value(1930), s.ad_value(1931))), 20.0))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_scale_ad(1930, A::sub(A::add(s.ad_value(2313), s.ad_value(2310)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2313), s.ad_value(2310)), A::sub(s.ad_value(2313), s.ad_value(2310))), 5.0))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_scale_ad(2314, A::add(A::sub(s.ad_value(1930), s.ad_value(2310)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(1930), A::neg(s.ad_value(2310))), A::sub(s.ad_value(1930), A::neg(s.ad_value(2310)))), 20.0))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) {
            s.store_mul_ad_rhs(1931, 703, A::offset(A::div(s.ad_value(2314), s.ad_value(2310)), 1.0));
        }

        s.v[2457] = if (s.v[1931] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) && (s.v[2457] != 0.0)) {
            s.store_exp(2315, 1931);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2456] != 0.0)) && (!(s.v[2457] != 0.0))) {
            s.store_div_from_scalar_ad(2315, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1931)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1931)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1931)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_offset_ad(2316, A::mul(s.ad_value(702), s.ad_value(2315)), 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul(2317, 1916, 2316);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul_ad(2318, A::mul(s.ad_value(197), A::offset(A::mul(s.ad_value(199), s.ad_value(819)), 1.0)), A::offset(A::mul(s.ad_value(198), s.ad_value(2303)), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul_ad_rhs(2319, 2317, A::offset(s.ad_value(2318), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_div_from_scalar(2320, 1.0, 2319);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul_ad_rhs(2304, 2296, A::sqrt(A::mul(s.ad_value(1916), s.ad_value(2320))));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_square(2305, 2304);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_div_from_scalar(2321, 1.0, 2305);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul(2322, 2298, 2320);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul(2323, 2301, 2320);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_div_ad(2324, A::scale(s.ad_value(819), 2.0), A::offset(A::sqrt(A::offset(A::mul(s.ad_value(195), s.ad_value(819)), 1.0)), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul_ad(2325, A::mul(s.ad_value(194), s.ad_value(2324)), A::offset(A::mul(s.ad_value(196), s.ad_value(2303)), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul(2326, 2294, 2320);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_sqrt_ad(1930, A::add(A::square(s.ad_value(2297)), s.ad_value(2295)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_sqrt_ad(1931, A::add(A::mul(A::sub(s.ad_value(2297), s.ad_value(2325)), A::sub(s.ad_value(2297), s.ad_value(2325))), s.ad_value(2295)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul_ad(2327, A::scale(s.ad_value(2320), 0.5), A::sub(A::add(s.ad_value(2325), s.ad_value(1930)), s.ad_value(1931)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_add(2328, 2326, 2322);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_sub(2329, 2328, 2327);
        }

        s.v[2458] = if (p.p45 > 0.0) { 1.0 } else { 0.0 };

        s.v[2459] = if (((s.v[2329]) as f64).abs() < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2458] != 0.0)) && (s.v[2459] != 0.0)) {
            s.store_offset_ad(2330, A::mul(s.ad_value(2304), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2329), 0.5), A::sub_from_scalar(1.0, A::scale(s.ad_value(2329), 0.3125))))), 1.0);
        }

        s.v[2460] = if (s.v[2329] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2458] != 0.0)) && (!(s.v[2459] != 0.0))) && (s.v[2460] != 0.0)) {
            s.store_exp_ad(2344, A::neg(s.ad_value(2329)));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2458] != 0.0)) && (!(s.v[2459] != 0.0))) && (!(s.v[2460] != 0.0))) {
            s.store_div_from_scalar_ad(2344, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2329), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2329), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2329), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2458] != 0.0)) && (!(s.v[2459] != 0.0))) {
            s.store_scalar(1929, (if (s.v[2329] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2458] != 0.0)) && (!(s.v[2459] != 0.0))) {
            s.store_offset_ad(2330, A::div(A::mul(A::mul(s.ad_value(1929), s.ad_value(2304)), A::sub_from_scalar(1.0, A::mul(s.ad_value(2344), A::sub_from_scalar(1.0, s.ad_value(2329))))), A::scale(A::sqrt(A::mul(s.ad_value(2329), A::sub_from_scalar(1.0, s.ad_value(2344)))), 2.0)), 1.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2458] != 0.0))) {
            s.store_offset_ad(2330, A::div(A::scale(s.ad_value(2304), 0.5), A::sqrt(s.ad_value(2329))), 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_sub_ad(2331, A::add(s.ad_value(2329), A::mul(s.ad_value(2304), A::sqrt(s.ad_value(2329)))), A::mul(s.ad_value(2330), A::ln(A::offset(s.ad_value(2330), (-1.0)))));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_div_ad_lhs(2332, A::sub(s.ad_value(2323), s.ad_value(2331)), 2330);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul_ad(2338, A::scale(s.ad_value(2305), 0.5), A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2305)), 1.0)), (-1.0)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2337, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2339, 1.0);
        }

        s.v[2461] = if (s.v[2332] > (-30.0)) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_offset_ad(2333, A::mul(s.ad_value(2330), s.ad_value(2332)), (-1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_scale_ad(1929, A::add(s.ad_value(2333), A::sqrt(A::offset(A::square(s.ad_value(2333)), 10.0))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_sub_ad_rhs(2334, 2332, A::ln(s.ad_value(1929)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_scale_ad(2335, A::add(s.ad_value(2334), A::sqrt(A::offset(A::square(s.ad_value(2334)), 2.0))), 0.5);
        }

        s.v[2462] = if ((s.v[2332] - s.v[2335]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) && (s.v[2462] != 0.0)) {
            s.store_exp_ad(1929, A::sub(s.ad_value(2332), s.ad_value(2335)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) && (!(s.v[2462] != 0.0))) {
            s.store_scale_ad(1929, A::offset(A::mul(A::offset(A::sub(s.ad_value(2332), s.ad_value(2335)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2332), s.ad_value(2335)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2332), s.ad_value(2335)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_div(2336, 1929, 2330);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_sub_ad_lhs(1929, A::scale(A::offset(s.ad_value(2335), 1.0), 2.0), 2336);
        }

        s.v[2463] = if (s.v[2336] > 1e-6) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) && (s.v[2463] != 0.0)) {
            s.store_mul_ad_rhs(2337, 2330, A::offset(A::sub(s.ad_value(2335), A::div(A::offset(A::sqrt(A::offset(A::mul(s.ad_value(2336), s.ad_value(1929)), 1.0)), (-1.0)), s.ad_value(2336))), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) && (!(s.v[2463] != 0.0))) {
            s.store_mul_ad(2337, A::mul(A::scale(s.ad_value(2330), 0.5), s.ad_value(2336)), A::offset(A::mul(A::scale(s.ad_value(1929), 0.25), s.ad_value(1929)), 1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_scale_ad(1929, A::add(A::offset(A::sub(s.ad_value(2323), s.ad_value(2337)), 2.0), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(2323), s.ad_value(2337)), (-2.0)), A::offset(A::sub(s.ad_value(2323), s.ad_value(2337)), (-2.0))), 1.0))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_mul_ad(2338, A::scale(s.ad_value(2305), 0.5), A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2305)), s.ad_value(1929)), 1.0)), (-1.0)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_div_ad_rhs(2339, 2338, A::add(s.ad_value(2338), s.ad_value(2337)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2461] != 0.0)) {
            s.store_sub_ad_rhs(2329, 2328, A::mul(s.ad_value(2339), s.ad_value(2327)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_offset_scaled(2340, 2304, 0.7071067811865475, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scale(2341, 2340, 1e-5);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_div_from_scalar(2342, 1.0, 2340);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2449, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2343, 0.0);
        }

        s.v[2464] = if (s.v[2329] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2464] != 0.0)) {
            s.store_exp_ad(2344, A::neg(s.ad_value(2329)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2464] != 0.0))) {
            s.store_div_from_scalar_ad(2344, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2329), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2329), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2329), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2465] = if (((s.v[2323]) as f64).abs() <= s.v[2341]) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2465] != 0.0)) {
            s.store_scale_ad(2429, A::square(s.ad_value(2342)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2465] != 0.0)) {
            s.store_mul_ad(2343, A::mul(s.ad_value(2323), s.ad_value(2342)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2323), A::sub_from_scalar(1.0, s.ad_value(2344))), s.ad_value(2304)), s.ad_value(2429)), 1.0));
        }

        s.v[2466] = if (s.v[2323] < (-s.v[2341])) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_neg(2431, 2323);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_scaled_mul(2432, 2431, 2342, 1.25);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_scale_ad(2433, A::sub(A::offset(s.ad_value(2432), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2432), (-6.0)), A::offset(s.ad_value(2432), (-6.0))), 64.0))), 0.5);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub(2428, 2431, 2433);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_add_ad(2434, A::square(s.ad_value(2428)), A::mul(s.ad_value(2305), A::offset(s.ad_value(2433), 1.0)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub_ad_lhs(2435, A::scale(s.ad_value(2428), 2.0), 2305);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub_ad_lhs(2436, A::ln(A::mul(s.ad_value(2434), s.ad_value(2321))), 2433);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_add(813, 2434, 2435);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_add_ad(812, A::square(s.ad_value(813)), A::mul(s.ad_value(2436), A::sub(A::scale(A::square(s.ad_value(2435)), 0.5), s.ad_value(2434))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_add_ad_rhs(2437, 2433, A::div(A::mul(A::mul(s.ad_value(2434), s.ad_value(813)), s.ad_value(2436)), A::add(s.ad_value(812), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436)), s.ad_value(2436)), s.ad_value(2435)), A::sub(A::scale(A::square(s.ad_value(2435)), 0.3333333333333333), s.ad_value(2434))))));
        }

        s.v[2467] = if (s.v[2437] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) && (s.v[2467] != 0.0)) {
            s.store_exp(2438, 2437);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) && (!(s.v[2467] != 0.0))) {
            s.store_scale_ad(2438, A::offset(A::mul(A::offset(s.ad_value(2437), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2437), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2437), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_div_from_scalar(2439, 1.0, 2438);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_div_from_scalar_ad(2428, 1.0, A::offset(A::square(s.ad_value(2437)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_mul_ad_lhs(2440, A::square(s.ad_value(2437)), 2428);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_scale_ad(2441, A::mul(A::mul(s.ad_value(2437), s.ad_value(2428)), s.ad_value(2428)), 4.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_mul_ad_lhs(2442, A::mul(A::sub(A::scale(s.ad_value(2428), 8.0), A::scale(s.ad_value(2440), 12.0)), s.ad_value(2428)), 2428);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub(2428, 2431, 2437);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_mul(2429, 2344, 2439);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_add_ad(2443, A::scale(s.ad_value(2428), 2.0), A::mul(s.ad_value(2305), A::add(A::sub(A::offset(s.ad_value(2438), (-1.0)), s.ad_value(2429)), A::mul(s.ad_value(2344), A::sub_from_scalar(1.0, s.ad_value(2441))))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub_ad(2444, A::square(s.ad_value(2428)), A::mul(s.ad_value(2305), A::add(A::add(A::offset(A::sub(s.ad_value(2438), s.ad_value(2437)), (-1.0)), s.ad_value(2429)), A::mul(s.ad_value(2344), A::sub(A::offset(s.ad_value(2437), (-1.0)), s.ad_value(2440))))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub_from_scalar_ad(2428, 2.0, A::mul(s.ad_value(2305), A::sub(A::add(s.ad_value(2438), s.ad_value(2429)), A::mul(s.ad_value(2344), s.ad_value(2442)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub_ad(2428, A::square(s.ad_value(2443)), A::scale(A::mul(s.ad_value(2444), s.ad_value(2428)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (s.v[2466] != 0.0)) {
            s.store_sub_ad(2343, A::neg(s.ad_value(2437)), A::scale(A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_div_from_scalar_ad(2445, 1.0, A::offset(A::scale(s.ad_value(2304), 0.7324648775608221), 1.25));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_mul_ad_lhs(2446, A::offset(A::mul(A::scale(s.ad_value(2340), 1.25), s.ad_value(2445)), (-1.0)), 2445);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_mul_ad(2447, A::mul(s.ad_value(2323), s.ad_value(2342)), A::offset(A::mul(s.ad_value(2446), s.ad_value(2323)), 1.0));
        }

        s.v[2468] = if ((-s.v[2447]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (s.v[2468] != 0.0)) {
            s.store_exp_ad(2428, A::neg(s.ad_value(2447)));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (!(s.v[2468] != 0.0))) {
            s.store_div_from_scalar_ad(2428, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2447))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2447))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2447))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_from_scalar(2448, 1.0, 2428);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_ad(2449, A::add(s.ad_value(2323), A::scale(s.ad_value(2305), 0.5)), A::mul(s.ad_value(2304), A::sqrt(A::sub(A::add(s.ad_value(2323), A::scale(s.ad_value(2305), 0.25)), s.ad_value(2448)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_offset(2450, 2329, 3.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_ad(2433, A::scale(A::sub(A::add(s.ad_value(2449), s.ad_value(2450)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2449), s.ad_value(2450)), A::sub(s.ad_value(2449), s.ad_value(2450))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2450), A::sqrt(A::offset(A::square(s.ad_value(2450)), 5.0))), 0.5));
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
        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub(2428, 2323, 2433);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_exp_ad(2429, A::neg(s.ad_value(2433)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_div_from_scalar_ad(2430, 1.0, A::offset(A::square(s.ad_value(2433)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_mul_ad_lhs(2440, A::square(s.ad_value(2433)), 2430);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_scale_ad(2441, A::mul(A::mul(s.ad_value(2433), s.ad_value(2430)), s.ad_value(2430)), 4.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_mul_ad_lhs(2442, A::mul(A::sub(A::scale(s.ad_value(2430), 8.0), A::scale(s.ad_value(2440), 12.0)), s.ad_value(2430)), 2430);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            let assign49760_ad_e64162: A = {
                if (1e-40 > ((s.v[2428] * s.v[2428]) - (s.v[2305] * (((s.v[2429] + s.v[2433]) - 1.0) - (s.v[2344] * ((s.v[2433] + 1.0) + s.v[2440])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2428)), A::mul(s.ad_value(2305), A::sub(A::offset(A::add(s.ad_value(2429), s.ad_value(2433)), (-1.0)), A::mul(s.ad_value(2344), A::add(A::offset(s.ad_value(2433), 1.0), s.ad_value(2440))))))
                }
            };
            s.store_ad(2434, &assign49760_ad_e64162);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_from_scalar_ad(2451, 1.0, A::scale(A::mul(s.ad_value(2305), A::sub(s.ad_value(2429), A::mul(s.ad_value(2344), s.ad_value(2442)))), 0.5));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_add_ad(2435, A::scale(s.ad_value(2428), 2.0), A::mul(s.ad_value(2305), A::sub(A::sub_from_scalar(1.0, s.ad_value(2429)), A::mul(s.ad_value(2344), A::offset(s.ad_value(2441), 1.0)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_add_ad(2436, A::sub(s.ad_value(2329), s.ad_value(2433)), A::ln(A::div(s.ad_value(2434), s.ad_value(2305))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_add(813, 2434, 2435);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_add_ad(812, A::square(s.ad_value(813)), A::mul(s.ad_value(2436), A::sub(A::scale(A::square(s.ad_value(2435)), 0.5), A::mul(s.ad_value(2434), s.ad_value(2451)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            let assign49820_ad_e64309: A = A::add(s.ad_value(2433), A::div(A::mul(A::mul(s.ad_value(2434), s.ad_value(813)), s.ad_value(2436)), A::add(s.ad_value(812), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436)), s.ad_value(2436)), s.ad_value(2435)), A::sub(A::scale(A::square(s.ad_value(2435)), 0.3333333333333333), A::mul(s.ad_value(2434), s.ad_value(2451)))))));
            s.store_ad(2452, &assign49820_ad_e64309);
        }

        s.v[2469] = if (s.v[2452] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (s.v[2469] != 0.0)) {
            s.store_exp(2438, 2452);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (s.v[2469] != 0.0)) {
            s.store_div_from_scalar(2439, 1.0, 2438);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (s.v[2469] != 0.0)) {
            s.store_mul(2438, 2344, 2438);
        }

        s.v[2470] = if (s.v[2452] > (s.v[2329] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (!(s.v[2469] != 0.0))) && (s.v[2470] != 0.0)) {
            s.store_exp_ad(2438, A::sub(s.ad_value(2452), s.ad_value(2329)));
        }

        if ((((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (!(s.v[2469] != 0.0))) && (s.v[2470] != 0.0)) {
            s.store_div(2439, 2344, 2438);
        }

        if ((((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (!(s.v[2469] != 0.0))) && (!(s.v[2470] != 0.0))) {
            s.store_div_from_scalar_ad(2438, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2329), s.ad_value(2452)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2329), s.ad_value(2452)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2329), s.ad_value(2452)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) && (!(s.v[2469] != 0.0))) && (!(s.v[2470] != 0.0))) {
            s.store_div_from_scalar_ad(2439, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2452), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2452), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2452), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_div_from_scalar_ad(2428, 1.0, A::offset(A::square(s.ad_value(2452)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_mul_ad_lhs(2440, A::square(s.ad_value(2452)), 2428);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_scale_ad(2441, A::mul(A::mul(s.ad_value(2452), s.ad_value(2428)), s.ad_value(2428)), 4.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_mul_ad_lhs(2442, A::mul(A::sub(A::scale(s.ad_value(2428), 8.0), A::scale(s.ad_value(2440), 12.0)), s.ad_value(2428)), 2428);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub(2428, 2323, 2452);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_add_ad(2443, A::scale(s.ad_value(2428), 2.0), A::mul(s.ad_value(2305), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2439)), s.ad_value(2438)), A::mul(s.ad_value(2344), A::offset(s.ad_value(2441), 1.0)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_ad(2444, A::square(s.ad_value(2428)), A::mul(s.ad_value(2305), A::sub(A::add(A::offset(A::add(s.ad_value(2439), s.ad_value(2452)), (-1.0)), s.ad_value(2438)), A::mul(s.ad_value(2344), A::add(A::offset(s.ad_value(2452), 1.0), s.ad_value(2440))))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_from_scalar_ad(2428, 2.0, A::mul(s.ad_value(2305), A::sub(A::add(s.ad_value(2439), s.ad_value(2438)), A::mul(s.ad_value(2344), s.ad_value(2442)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_sub_ad(2428, A::square(s.ad_value(2443)), A::scale(A::mul(s.ad_value(2444), s.ad_value(2428)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (!(s.v[2465] != 0.0))) && (!(s.v[2466] != 0.0))) {
            s.store_add_ad_rhs(2343, 2452, A::scale(A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2346, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2347, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2348, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2349, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2350, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2351, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2352, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2353, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2354, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_sub(2355, 2323, 2343);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2356, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_mul(2357, 2319, 2355);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2358, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2359, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2363, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2364, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) {
            s.store_scalar(2366, 1.0);
        }

        s.v[2471] = if (s.v[2323] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_div_from_scalar_ad(1929, 1.0, A::offset(A::square(s.ad_value(2343)), 2.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_lhs(2345, A::square(s.ad_value(2343)), 1929);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_scale_ad(2346, A::mul(A::mul(s.ad_value(2343), s.ad_value(1929)), s.ad_value(1929)), 4.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_mul_ad_lhs(2347, A::mul(A::sub(A::scale(s.ad_value(1929), 8.0), A::scale(s.ad_value(2345), 12.0)), s.ad_value(1929)), 1929);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_scalar(2348, 0.0);
        }

        s.v[2472] = if (s.v[2343] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2472] != 0.0)) {
            s.store_exp(2348, 2343);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2472] != 0.0)) {
            s.store_div_from_scalar(2349, 1.0, 2348);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2472] != 0.0)) {
            s.store_mul(2348, 2344, 2348);
        }

        s.v[2473] = if (s.v[2343] > (s.v[2329] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2472] != 0.0))) && (s.v[2473] != 0.0)) {
            s.store_exp_ad(2348, A::sub(s.ad_value(2343), s.ad_value(2329)));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2472] != 0.0))) && (s.v[2473] != 0.0)) {
            s.store_div(2349, 2344, 2348);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2472] != 0.0))) && (!(s.v[2473] != 0.0))) {
            s.store_div_from_scalar_ad(2348, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2329), s.ad_value(2343)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2329), s.ad_value(2343)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2329), s.ad_value(2343)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2472] != 0.0))) && (!(s.v[2473] != 0.0))) {
            s.store_div_from_scalar_ad(2349, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2343), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2343), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2343), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_sub_ad_rhs(2350, 2348, A::mul(s.ad_value(2344), A::add(A::offset(s.ad_value(2343), 1.0), s.ad_value(2345))));
        }

        s.v[2474] = if (s.v[2343] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2474] != 0.0)) {
            s.store_scale_ad(2351, A::mul(A::square(s.ad_value(2343)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2343), A::sub_from_scalar(1.0, A::scale(s.ad_value(2343), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2474] != 0.0)) {
            s.store_scale_ad(2350, A::mul(A::mul(A::mul(A::mul(s.ad_value(2344), s.ad_value(2343)), s.ad_value(2343)), s.ad_value(2343)), A::offset(A::scale(s.ad_value(2343), 1.75), 1.0)), 0.16666666666666666);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2474] != 0.0)) {
            s.store_sqrt_ad(1929, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2343), A::sub_from_scalar(1.0, A::scale(s.ad_value(2343), 0.25))), 0.3333333333333333)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2474] != 0.0)) {
            s.store_scaled_mul(2352, 2343, 1929, 0.7071067811865475);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2474] != 0.0)) {
            s.store_offset_ad(2353, A::scale(A::div(A::mul(s.ad_value(2304), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2343), 0.5)), A::scale(A::square(s.ad_value(2343)), 0.16666666666666666))), s.ad_value(1929)), 0.7071067811865475), 1.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2474] != 0.0))) {
            s.store_add_ad_lhs(2351, A::offset(s.ad_value(2343), (-1.0)), 2349);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2474] != 0.0))) {
            s.store_sqrt(2352, 2351);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (!(s.v[2474] != 0.0))) {
            s.store_offset_ad(2353, A::scale(A::div(A::mul(s.ad_value(2304), A::sub_from_scalar(1.0, s.ad_value(2349))), s.ad_value(2352)), 0.5), 1.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) {
            s.store_div_ad(2354, A::offset(A::mul(A::scale(s.ad_value(709), 0.2), s.ad_value(2303)), 1.0), A::offset(A::mul(s.ad_value(709), s.ad_value(2303)), 1.0));
        }

        s.v[2475] = if (s.v[2350] > 1e-100) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul_ad_rhs(2355, 2304, A::sqrt(A::add(s.ad_value(2351), s.ad_value(2350))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_div_ad(2356, A::mul(A::mul(s.ad_value(2305), s.ad_value(2350)), s.ad_value(2319)), A::add(s.ad_value(2355), A::mul(s.ad_value(2304), s.ad_value(2352))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul_ad_lhs(2357, A::mul(s.ad_value(2352), s.ad_value(2304)), 2319);
        }

        s.v[2476] = if (s.v[215] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (s.v[2476] != 0.0)) {
            s.store_div_from_scalar_ad(2358, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(215), s.ad_value(2303))));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (!(s.v[2476] != 0.0))) {
            s.store_offset_ad(2358, A::mul(s.ad_value(215), s.ad_value(2303)), 1.0);
        }

        s.v[2477] = if (s.v[216] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (s.v[2477] != 0.0)) {
            s.store_sub_from_scalar_ad(2359, 1.0, A::mul(s.ad_value(216), s.ad_value(2356)));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (!(s.v[2477] != 0.0))) {
            s.store_div_from_scalar_ad(2359, 1.0, A::offset(A::mul(s.ad_value(216), s.ad_value(2356)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul_ad_lhs(2360, A::mul(A::mul(s.ad_value(746), s.ad_value(2358)), s.ad_value(2359)), 2356);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul_ad_rhs(2361, 763, A::add(s.ad_value(2357), A::mul(s.ad_value(764), s.ad_value(2356))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_ln_ad(1930, A::div(s.ad_value(2351), A::offset(A::add(s.ad_value(2351), s.ad_value(2350)), 1e-14)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_add_ad(2362, A::pow(A::mul(s.ad_value(2361), s.ad_value(705)), s.ad_value(706)), A::mul(s.ad_value(707), A::exp(A::mul(A::scale(s.ad_value(708), 0.5), s.ad_value(1930)))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul_ad_lhs(2363, A::add(A::offset(s.ad_value(2362), 1.0), s.ad_value(2360)), 2354);
        }

        s.v[2478] = if (s.v[219] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (s.v[2478] != 0.0)) {
            s.store_div_from_scalar_ad(2364, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(219), s.ad_value(2303))));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (!(s.v[2478] != 0.0))) {
            s.store_offset_ad(2364, A::mul(s.ad_value(219), s.ad_value(2303)), 1.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_mul(1931, 2356, 2364);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) {
            s.store_div_ad_rhs(2365, 1931, A::add(s.ad_value(221), s.ad_value(1931)));
        }

        s.v[2479] = if (s.v[220] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (s.v[2479] != 0.0)) {
            s.store_div_from_scalar_ad(2366, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(220), s.ad_value(2365))));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2454] != 0.0)) && (s.v[2471] != 0.0)) && (s.v[2475] != 0.0)) && (!(s.v[2479] != 0.0))) {
            s.store_offset_ad(2366, A::mul(s.ad_value(220), s.ad_value(2365)), 1.0);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2301, 1806);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2303, 1807);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2319, 1808);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2320, 1809);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2304, 1810);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2305, 1811);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2321, 1812);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2323, 1813);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2328, 1814);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2329, 1815);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2340, 1816);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2341, 1817);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2342, 1818);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2449, 1819);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2344, 1820);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2343, 1821);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2346, 1822);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2347, 1823);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2348, 1824);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2349, 1825);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2351, 1826);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2350, 1827);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2352, 1828);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2353, 1829);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2354, 1830);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2355, 1831);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2356, 1832);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2357, 1833);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2358, 1834);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2359, 1835);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2363, 1836);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2364, 1837);
        }

        if ((s.v[2453] != 0.0) && (!(s.v[2454] != 0.0))) {
            s.copy_ad(2366, 1838);
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
        if (s.v[2453] != 0.0) {
            s.copy_ad(2299, 1921);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2300, 766);
        }

        s.v[2480] = if (p.p48 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2453] != 0.0) && (s.v[2480] != 0.0)) {
            s.copy_ad(2299, 1922);
        }

        if ((s.v[2453] != 0.0) && (s.v[2480] != 0.0)) {
            s.copy_ad(2300, 767);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2368, 0.0);
        }

        if (s.v[2453] != 0.0) {
            s.store_scale(2367, 2319, 4.60517018598809);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2384, 2367);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2385, 815);
        }

        if (s.v[2453] != 0.0) {
            s.store_mul(2386, 815, 2320);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2390, 2343);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2391, 0.0);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2394, 0.0);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2396, 2349);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2397, 2351);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2399, 2350);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2400, 2357);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2401, 2343);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2402, 2349);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2404, 2350);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2405, 2351);
        }

        if (s.v[2453] != 0.0) {
            s.store_sub(2406, 2323, 2343);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2407, 1.0);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2409, 1.0);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2408, 0.0);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2418, 2356);
        }

        if (s.v[2453] != 0.0) {
            s.store_mul(2422, 2406, 2319);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2419, 0.0);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2420, 2357);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2425, 0.0);
        }

        if (s.v[2453] != 0.0) {
            s.store_scalar(2424, 1.0);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2427, 2299);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(2426, 2422);
        }

        s.v[2481] = if (s.v[2323] > 0.0) { 1.0 } else { 0.0 };

        s.v[2482] = if (s.v[2350] > 1e-100) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_mul(2427, 2299, 2366);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_div(2368, 2427, 2363);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_add_ad_rhs(2369, 2355, A::scale(s.ad_value(2305), 0.5));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_div_ad_lhs(1929, A::div(A::mul(s.ad_value(2305), s.ad_value(2348)), s.ad_value(2369)), 2369);
        }

        s.v[2483] = if (s.v[1929] > 0.0001) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2483] != 0.0)) {
            s.store_sub_from_scalar(1930, 1.0, 1929);
        }

        s.v[2484] = if (s.v[1930] < 1e-10) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2483] != 0.0)) && (s.v[2484] != 0.0)) {
            s.store_scalar(1931, 1.0);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2483] != 0.0)) && (!(s.v[2484] != 0.0))) {
            s.store_sub_from_scalar_ad(1931, 1.0, A::sqrt(s.ad_value(1930)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (!(s.v[2483] != 0.0))) {
            s.store_scale(1931, 1929, 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_mul(2370, 1931, 2369);
        }

        s.v[2485] = if ((s.v[707] > 0.0) && (s.v[708] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_lhs(2371, A::scale(s.ad_value(2319), 0.475), 2370);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_sub_ad_rhs(1929, 2356, A::mul(s.ad_value(2353), s.ad_value(2371)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_scale_ad(2372, A::add(s.ad_value(1929), A::sqrt(A::offset(A::square(s.ad_value(1929)), 1e-12))), 0.5);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_add_ad(2373, A::sub(A::mul(s.ad_value(2319), s.ad_value(2355)), s.ad_value(2356)), A::mul(A::offset(s.ad_value(2353), (-1.0)), s.ad_value(2371)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_offset_ad(2374, A::div(A::mul(A::scale(s.ad_value(2305), 0.5), s.ad_value(2319)), s.ad_value(2373)), 1.0);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_add_ad_rhs(1929, 2373, A::mul(s.ad_value(764), s.ad_value(2372)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_ad(2375, &A::pow(A::mul(A::mul(s.ad_value(763), s.ad_value(1929)), s.ad_value(705)), s.ad_value(706)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_lhs(1930, A::div(A::mul(s.ad_value(706), A::offset(A::mul(s.ad_value(2374), A::sub_from_scalar(1.0, s.ad_value(764))), (-1.0))), s.ad_value(1929)), 2375);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_div(1929, 2372, 2373);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_rhs(2376, 707, A::pow(A::offset(s.ad_value(1929), 1.0), A::neg(s.ad_value(708))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_lhs(1931, A::div(A::mul(s.ad_value(708), A::add(A::offset(s.ad_value(2374), (-1.0)), A::div_from_scalar(1.0, A::offset(s.ad_value(1929), 1.0)))), s.ad_value(2373)), 2376);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_lhs(2377, A::mul(A::mul(s.ad_value(746), s.ad_value(2358)), s.ad_value(2359)), 2372);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_offset_ad(1929, A::div(A::sub(s.ad_value(1930), A::mul(A::mul(A::mul(s.ad_value(746), s.ad_value(2358)), s.ad_value(2359)), s.ad_value(2374))), s.ad_value(1931)), 1.0);
        }

        s.v[2486] = if (s.v[1929] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) && (s.v[2486] != 0.0)) {
            s.store_scale_ad(1930, A::ln(A::offset(A::exp(A::scale(s.ad_value(1929), 2.0)), 1.0)), 0.5);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) && (!(s.v[2486] != 0.0))) {
            s.copy_ad(1930, 1929);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_div_ad(2378, A::mul(A::mul(A::neg(s.ad_value(2371)), s.ad_value(1931)), s.ad_value(1930)), A::add(A::add(A::offset(s.ad_value(2375), 1.0), s.ad_value(2376)), s.ad_value(2377)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2485] != 0.0)) {
            s.store_mul_ad_rhs(2379, 2370, A::offset(A::div(s.ad_value(2378), A::offset(A::sqrt(A::offset(A::square(s.ad_value(2378)), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (!(s.v[2485] != 0.0))) {
            s.copy_ad(2379, 2370);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_scale_ad(2380, A::mul(A::mul(s.ad_value(2319), s.ad_value(2368)), s.ad_value(2379)), 0.7071067811865475);
        }

        s.v[2487] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) && (s.v[2487] != 0.0)) {
            s.store_div_ad_rhs(2380, 2380, A::sqrt(A::offset(s.ad_value(2380), 1.0)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_div_from_scalar_ad(2381, 2.0, A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2380), 4.0), 1.0)), 1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_mul(1929, 2381, 2380);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_mul_ad(2382, A::mul(s.ad_value(2379), s.ad_value(2381)), A::offset(A::div(A::mul(A::scale(s.ad_value(1929), 0.86), A::sub_from_scalar(1.0, A::mul(s.ad_value(1929), s.ad_value(2381)))), A::offset(A::mul(A::mul(A::scale(s.ad_value(1929), 4.0), s.ad_value(1929)), s.ad_value(2381)), 1.0)), 1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_scale(2383, 2382, 0.99);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_div_ad_lhs(1929, A::mul(A::mul(s.ad_value(2383), A::sub(s.ad_value(2383), A::scale(s.ad_value(2369), 2.0))), s.ad_value(2321)), 2350);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2482] != 0.0)) {
            s.store_mul_ad_rhs(2384, 2319, A::sub(s.ad_value(2383), A::ln(A::offset({
                if (s.v[1929] > (-0.99)) {
                    s.ad_value(1929)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2482] != 0.0))) {
            s.copy_ad(2384, 2367);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_offset(1929, 2300, 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_div_ad_lhs(1930, A::mul(A::sqrt(s.ad_value(1929)), s.ad_value(815)), 2384);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add_ad_lhs(1931, A::square(s.ad_value(1930)), 1929);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_scale(1929, 1930, 2.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_div_ad(2385, A::mul(s.ad_value(2384), s.ad_value(1929)), A::add(A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929)))));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(2386, 2385, 2320);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add(2387, 2329, 2386);
        }

        s.v[2488] = if (s.v[2386] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2488] != 0.0)) {
            s.store_exp_ad(2388, A::neg(s.ad_value(2386)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2488] != 0.0))) {
            s.store_div_from_scalar_ad(2388, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2386), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2386), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2386), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(2389, 2344, 2388);
        }

        s.v[2489] = if (((s.v[2323]) as f64).abs() <= s.v[2341]) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2489] != 0.0)) {
            s.store_scale_ad(2429, A::square(s.ad_value(2342)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2489] != 0.0)) {
            s.store_mul_ad(2390, A::mul(s.ad_value(2323), s.ad_value(2342)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2323), A::sub_from_scalar(1.0, s.ad_value(2389))), s.ad_value(2304)), s.ad_value(2429)), 1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_offset(2450, 2387, 3.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub_ad(2433, A::scale(A::sub(A::add(s.ad_value(2449), s.ad_value(2450)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2449), s.ad_value(2450)), A::sub(s.ad_value(2449), s.ad_value(2450))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2450), A::sqrt(A::offset(A::square(s.ad_value(2450)), 5.0))), 0.5));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub(2428, 2323, 2433);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_exp_ad(2429, A::neg(s.ad_value(2433)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_div_from_scalar_ad(2430, 1.0, A::offset(A::square(s.ad_value(2433)), 2.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_mul_ad_lhs(2440, A::square(s.ad_value(2433)), 2430);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_scale_ad(2441, A::mul(A::mul(s.ad_value(2433), s.ad_value(2430)), s.ad_value(2430)), 4.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_mul_ad_lhs(2442, A::mul(A::sub(A::scale(s.ad_value(2430), 8.0), A::scale(s.ad_value(2440), 12.0)), s.ad_value(2430)), 2430);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            let assign52090_ad_e66961: A = {
                if (1e-40 > ((s.v[2428] * s.v[2428]) - (s.v[2305] * (((s.v[2429] + s.v[2433]) - 1.0) - (s.v[2389] * ((s.v[2433] + 1.0) + s.v[2440])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2428)), A::mul(s.ad_value(2305), A::sub(A::offset(A::add(s.ad_value(2429), s.ad_value(2433)), (-1.0)), A::mul(s.ad_value(2389), A::add(A::offset(s.ad_value(2433), 1.0), s.ad_value(2440))))))
                }
            };
            s.store_ad(2434, &assign52090_ad_e66961);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub_from_scalar_ad(2451, 1.0, A::scale(A::mul(s.ad_value(2305), A::sub(s.ad_value(2429), A::mul(s.ad_value(2389), s.ad_value(2442)))), 0.5));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_add_ad(2435, A::scale(s.ad_value(2428), 2.0), A::mul(s.ad_value(2305), A::sub(A::sub_from_scalar(1.0, s.ad_value(2429)), A::mul(s.ad_value(2389), A::offset(s.ad_value(2441), 1.0)))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_add_ad(2436, A::sub(s.ad_value(2387), s.ad_value(2433)), A::ln(A::div(s.ad_value(2434), s.ad_value(2305))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_add(813, 2434, 2435);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_add_ad(812, A::square(s.ad_value(813)), A::mul(s.ad_value(2436), A::sub(A::scale(A::square(s.ad_value(2435)), 0.5), A::mul(s.ad_value(2434), s.ad_value(2451)))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            let assign52150_ad_e67090: A = A::add(s.ad_value(2433), A::div(A::mul(A::mul(s.ad_value(2434), s.ad_value(813)), s.ad_value(2436)), A::add(s.ad_value(812), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436)), s.ad_value(2436)), s.ad_value(2435)), A::sub(A::scale(A::square(s.ad_value(2435)), 0.3333333333333333), A::mul(s.ad_value(2434), s.ad_value(2451)))))));
            s.store_ad(2452, &assign52150_ad_e67090);
        }

        s.v[2490] = if (s.v[2452] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (s.v[2490] != 0.0)) {
            s.store_exp(2438, 2452);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (s.v[2490] != 0.0)) {
            s.store_div_from_scalar(2439, 1.0, 2438);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (s.v[2490] != 0.0)) {
            s.store_mul(2438, 2389, 2438);
        }

        s.v[2491] = if (s.v[2452] > (s.v[2387] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (!(s.v[2490] != 0.0))) && (s.v[2491] != 0.0)) {
            s.store_exp_ad(2438, A::sub(s.ad_value(2452), s.ad_value(2387)));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (!(s.v[2490] != 0.0))) && (s.v[2491] != 0.0)) {
            s.store_div(2439, 2389, 2438);
        }

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (!(s.v[2490] != 0.0))) && (!(s.v[2491] != 0.0))) {
            s.store_div_from_scalar_ad(2438, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2387), s.ad_value(2452)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2387), s.ad_value(2452)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2387), s.ad_value(2452)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) && (!(s.v[2490] != 0.0))) && (!(s.v[2491] != 0.0))) {
            s.store_div_from_scalar_ad(2439, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2452), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2452), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2452), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_div_from_scalar_ad(2428, 1.0, A::offset(A::square(s.ad_value(2452)), 2.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_mul_ad_lhs(2440, A::square(s.ad_value(2452)), 2428);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_scale_ad(2441, A::mul(A::mul(s.ad_value(2452), s.ad_value(2428)), s.ad_value(2428)), 4.0);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_mul_ad_lhs(2442, A::mul(A::sub(A::scale(s.ad_value(2428), 8.0), A::scale(s.ad_value(2440), 12.0)), s.ad_value(2428)), 2428);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub(2428, 2323, 2452);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_add_ad(2443, A::scale(s.ad_value(2428), 2.0), A::mul(s.ad_value(2305), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2439)), s.ad_value(2438)), A::mul(s.ad_value(2389), A::offset(s.ad_value(2441), 1.0)))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub_ad(2444, A::square(s.ad_value(2428)), A::mul(s.ad_value(2305), A::sub(A::add(A::offset(A::add(s.ad_value(2439), s.ad_value(2452)), (-1.0)), s.ad_value(2438)), A::mul(s.ad_value(2389), A::add(A::offset(s.ad_value(2452), 1.0), s.ad_value(2440))))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub_from_scalar_ad(2428, 2.0, A::mul(s.ad_value(2305), A::sub(A::add(s.ad_value(2439), s.ad_value(2438)), A::mul(s.ad_value(2389), s.ad_value(2442)))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_sub_ad(2428, A::square(s.ad_value(2443)), A::scale(A::mul(s.ad_value(2444), s.ad_value(2428)), 2.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2489] != 0.0))) {
            s.store_add_ad_rhs(2390, 2452, A::scale(A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_sub(2391, 2390, 2343);
        }

        s.v[2492] = if (s.v[2391] < 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2492] != 0.0)) {
            s.store_add_ad(2392, A::scale(A::sub(s.ad_value(2323), s.ad_value(2343)), 2.0), A::mul(s.ad_value(2305), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2349)), A::mul(s.ad_value(2348), s.ad_value(2388))), A::mul(s.ad_value(2389), A::offset(s.ad_value(2346), 1.0)))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2492] != 0.0)) {
            s.store_mul_ad_lhs(2393, A::mul(s.ad_value(2305), A::sub_from_scalar(1.0, s.ad_value(2388))), 2350);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2492] != 0.0)) {
            s.store_sub_from_scalar_ad(1929, 2.0, A::mul(s.ad_value(2305), A::sub(A::add(s.ad_value(2349), A::mul(s.ad_value(2348), s.ad_value(2388))), A::mul(s.ad_value(2389), s.ad_value(2347)))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2492] != 0.0)) {
            s.store_sub_ad(1929, A::square(s.ad_value(2392)), A::scale(A::mul(s.ad_value(1929), s.ad_value(2393)), 2.0));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2492] != 0.0)) {
            s.store_scale_ad(2391, A::div(s.ad_value(2393), A::add(s.ad_value(2392), A::sqrt(s.ad_value(1929)))), 2.0);
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
        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2492] != 0.0)) {
            s.store_add(2390, 2343, 2391);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(2394, 2391, 2319);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_div_ad(2395, A::square(s.ad_value(2390)), A::offset(A::square(s.ad_value(2390)), 2.0));
        }

        s.v[2493] = if (s.v[2390] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) {
            s.store_exp_ad(2396, A::neg(s.ad_value(2390)));
        }

        s.v[2494] = if (s.v[2390] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (s.v[2494] != 0.0)) {
            s.store_scale_ad(2397, A::mul(A::square(s.ad_value(2390)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2390), A::sub_from_scalar(1.0, A::scale(s.ad_value(2390), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (s.v[2494] != 0.0)) {
            s.store_sqrt_ad(1929, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2390), A::sub_from_scalar(1.0, A::scale(s.ad_value(2390), 0.25))), 0.3333333333333333)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (s.v[2494] != 0.0)) {
            s.store_scaled_mul(2398, 2390, 1929, 0.7071067811865475);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (s.v[2494] != 0.0)) {
            s.store_mul_ad(2399, A::mul(A::mul(A::mul(A::scale(s.ad_value(2389), 0.16666666666666666), s.ad_value(2390)), s.ad_value(2390)), s.ad_value(2390)), A::offset(A::scale(s.ad_value(2390), 1.75), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (!(s.v[2494] != 0.0))) {
            s.store_add_ad_lhs(2397, A::offset(s.ad_value(2390), (-1.0)), 2396);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (!(s.v[2494] != 0.0))) {
            s.store_sqrt(2398, 2397);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2493] != 0.0)) && (!(s.v[2494] != 0.0))) {
            s.store_mul_ad_rhs(2399, 2389, A::sub(A::offset(A::sub(A::div_from_scalar(1.0, s.ad_value(2396)), s.ad_value(2390)), (-1.0)), s.ad_value(2395)));
        }

        s.v[2495] = if (s.v[2390] > (s.v[2387] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) && (s.v[2495] != 0.0)) {
            s.store_exp_ad(1929, A::sub(s.ad_value(2390), s.ad_value(2387)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) && (s.v[2495] != 0.0)) {
            s.store_div(2396, 2389, 1929);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) && (s.v[2495] != 0.0)) {
            s.store_sub_ad_rhs(2399, 1929, A::mul(s.ad_value(2389), A::add(A::offset(s.ad_value(2390), 1.0), s.ad_value(2395))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) && (!(s.v[2495] != 0.0))) {
            s.store_div_from_scalar_ad(2396, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2390), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2390), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2390), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) && (!(s.v[2495] != 0.0))) {
            s.store_div_from_scalar_ad(1929, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2387), s.ad_value(2390)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2387), s.ad_value(2390)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2387), s.ad_value(2390)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) && (!(s.v[2495] != 0.0))) {
            s.store_sub_ad_rhs(2399, 1929, A::mul(s.ad_value(2389), A::add(A::offset(s.ad_value(2390), 1.0), s.ad_value(2395))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) {
            s.store_add_ad_lhs(2397, A::offset(s.ad_value(2390), (-1.0)), 2396);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2493] != 0.0))) {
            s.store_sqrt(2398, 2397);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul_ad_lhs(2400, A::mul(s.ad_value(2398), s.ad_value(2304)), 2319);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_scaled_add(2401, 2343, 2390, 0.5);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_scalar(2402, 0.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(1929, 2396, 2349);
        }

        s.v[2496] = if (s.v[1929] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2496] != 0.0)) {
            s.store_sqrt(2402, 1929);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_scaled_add(2403, 2350, 2399, 0.5);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add_ad_rhs(2404, 2403, A::scale(A::mul(A::square(s.ad_value(2391)), A::sub(s.ad_value(2402), A::scale(s.ad_value(2321), 2.0))), 0.125));
        }

        s.v[2497] = if (s.v[2401] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2497] != 0.0)) {
            s.store_scale_ad(2405, A::mul(A::square(s.ad_value(2401)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2401), A::sub_from_scalar(1.0, A::scale(s.ad_value(2401), 0.25))), 0.3333333333333333))), 0.5);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2497] != 0.0)) {
            s.store_mul_ad_rhs(2406, 2304, A::sqrt(A::add(s.ad_value(2404), s.ad_value(2405))));
        }

        s.v[2498] = if (s.v[719] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2497] != 0.0)) && (s.v[2498] != 0.0)) {
            s.store_div_from_scalar_ad(2407, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(719), s.ad_value(2406)), 1.0)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2497] != 0.0)) {
            s.store_sqrt_ad(1929, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2401), A::sub_from_scalar(1.0, A::scale(s.ad_value(2401), 0.25))), 0.3333333333333333)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2497] != 0.0)) {
            s.store_scaled_mul(2408, 2401, 1929, 0.7071067811865475);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2497] != 0.0)) {
            s.store_add_ad_rhs(2409, 2407, A::scale(A::div(A::mul(s.ad_value(2304), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2401), 0.5)), A::scale(A::square(s.ad_value(2401)), 0.16666666666666666))), s.ad_value(1929)), 0.7071067811865475));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) {
            s.store_add_ad_lhs(2405, A::offset(s.ad_value(2401), (-1.0)), 2402);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) {
            s.store_mul_ad_rhs(2406, 2304, A::sqrt(A::add(s.ad_value(2404), s.ad_value(2405))));
        }

        s.v[2499] = if (s.v[719] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_add_ad(2410, A::sub_from_scalar(1.0, s.ad_value(2402)), A::scale(A::mul(s.ad_value(2406), s.ad_value(2321)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_div_from_scalar_ad(2407, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(719), s.ad_value(2406)), 1.0)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_div_ad_rhs(1929, 2407, A::offset(s.ad_value(2407), 1.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_mul_ad_rhs(2411, 719, A::mul(A::mul(A::square(s.ad_value(1929)), s.ad_value(2305)), s.ad_value(2404)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_add_ad(2412, A::scale(A::sub(s.ad_value(2406), s.ad_value(2411)), 2.0), A::mul(s.ad_value(2305), A::add(A::sub_from_scalar(1.0, s.ad_value(2402)), s.ad_value(2404))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_mul_ad_rhs(2413, 2411, A::sub(s.ad_value(2411), A::scale(s.ad_value(2406), 2.0)));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_sub_from_scalar_ad(2414, 1.0, A::scale(A::mul(s.ad_value(2305), A::add(s.ad_value(2402), s.ad_value(2404))), 0.5));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_div_ad(2415, A::mul(s.ad_value(2413), s.ad_value(2412)), A::sub(A::square(s.ad_value(2412)), A::mul(s.ad_value(2414), s.ad_value(2413))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_add(2401, 2401, 2415);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_exp(2416, 2415);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_div(2402, 2402, 2416);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_mul(2404, 2404, 2416);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_add_ad_lhs(2405, A::offset(s.ad_value(2401), (-1.0)), 2402);
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_mul_ad_rhs(2406, 2304, A::sqrt(A::add(s.ad_value(2404), s.ad_value(2405))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_add_ad(2417, A::sub_from_scalar(1.0, s.ad_value(2402)), A::scale(A::mul(A::mul(s.ad_value(2406), s.ad_value(2407)), s.ad_value(2321)), 2.0));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_div_ad(2391, A::mul(A::mul(s.ad_value(2391), s.ad_value(2416)), A::add(s.ad_value(2410), s.ad_value(2403))), A::add(s.ad_value(2417), A::mul(s.ad_value(2416), s.ad_value(2403))));
        }

        if ((((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) && (s.v[2499] != 0.0)) {
            s.store_mul(2394, 2391, 2319);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) {
            s.store_sqrt(2408, 2405);
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2497] != 0.0))) {
            s.store_add_ad_rhs(2409, 2407, A::scale(A::div(A::mul(s.ad_value(2304), A::sub_from_scalar(1.0, s.ad_value(2402))), s.ad_value(2408)), 0.5));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul_ad_rhs(2418, 2319, A::div(A::mul(s.ad_value(2305), s.ad_value(2404)), A::add(s.ad_value(2406), A::mul(s.ad_value(2304), s.ad_value(2408)))));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add_ad_rhs(2419, 2418, A::mul(s.ad_value(2319), s.ad_value(2409)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul_ad_lhs(2420, A::mul(s.ad_value(2408), s.ad_value(2304)), 2319);
        }

        s.v[2500] = if (s.v[216] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2500] != 0.0)) {
            s.store_sub_from_scalar_ad(2359, 1.0, A::mul(s.ad_value(216), s.ad_value(2418)));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2500] != 0.0))) {
            s.store_div_from_scalar_ad(2359, 1.0, A::offset(A::mul(s.ad_value(216), s.ad_value(2418)), 1.0));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul_ad_lhs(2360, A::mul(A::mul(s.ad_value(746), s.ad_value(2358)), s.ad_value(2359)), 2418);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add_ad_rhs(2421, 2420, A::mul(s.ad_value(764), s.ad_value(2418)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add_ad_rhs(2422, 2420, A::mul(s.ad_value(765), s.ad_value(2418)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(2423, 763, 2421);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_ln_ad(1930, A::div(s.ad_value(2405), A::offset(A::add(s.ad_value(2405), s.ad_value(2404)), 1e-14)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_add_ad(2362, A::pow(A::mul(s.ad_value(2423), s.ad_value(705)), s.ad_value(706)), A::mul(s.ad_value(707), A::exp(A::mul(A::scale(s.ad_value(708), 0.5), s.ad_value(1930)))));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul_ad_lhs(2424, A::add(A::offset(s.ad_value(2362), 1.0), s.ad_value(2360)), 2354);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_ln_ad(2425, A::div(A::offset(A::mul(A::sub(s.ad_value(815), s.ad_value(2394)), s.ad_value(768)), 1.0), A::offset(A::mul(A::sub(s.ad_value(2385), s.ad_value(2394)), s.ad_value(768)), 1.0)));
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(1931, 2418, 2364);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_div_ad_rhs(2365, 1931, A::add(s.ad_value(221), s.ad_value(1931)));
        }

        s.v[2501] = if (s.v[220] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (s.v[2501] != 0.0)) {
            s.store_div_from_scalar_ad(2366, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(220), s.ad_value(2365))));
        }

        if (((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) && (!(s.v[2501] != 0.0))) {
            s.store_offset_ad(2366, A::mul(s.ad_value(220), s.ad_value(2365)), 1.0);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(2427, 2299, 2366);
        }

        if ((s.v[2453] != 0.0) && (s.v[2481] != 0.0)) {
            s.store_mul(2426, 2406, 2319);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1871, 2301);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1872, 2319);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1873, 2304);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1874, 2323);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1875, 2328);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1876, 2357);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1877, 2394);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1878, 2400);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1879, 2407);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1880, 2409);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1881, 2418);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1882, 2419);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1883, 2422);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1884, 2424);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1885, 2425);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1886, 2427);
        }

        if (s.v[2453] != 0.0) {
            s.copy_ad(1887, 2426);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(734, 717);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1871, 1806);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1872, 1808);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1873, 1810);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1874, 1813);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1875, 1814);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1876, 1833);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1877, 1844);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1878, 1845);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1879, 1847);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1880, 1848);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1881, 1849);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1882, 1850);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1883, 1852);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1884, 1853);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1885, 1855);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1886, 1854);
        }

        if (!(s.v[2453] != 0.0)) {
            s.copy_ad(1887, 1856);
        }

        s.copy_ad(1888, 253);

        s.v[2502] = if (s.v[762] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2502] != 0.0) {
            s.store_div_ad_rhs(1888, 253, A::offset(A::mul(s.ad_value(762), A::powf(A::add(A::square(s.ad_value(1883)), s.ad_value(722)), ((-1.0) * 0.16666666666666666))), 1.0));
        }

        s.v[1889] = 1.0;

        s.v[1890] = 1.0;

        s.v[1891] = 0.0;

        s.v[1892] = 1.0;

        s.v[1893] = 1.0;

        s.copy_ad(2265, 1887);

        s.v[2268] = 0.0;

        s.v[2267] = 0.0;

        s.copy_ad(2269, 2265);

        s.v[2503] = if (s.v[1874] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2503] != 0.0) {
            s.store_mul_ad_lhs(2260, A::div(A::mul(A::add(s.ad_value(258), A::div(s.ad_value(259), s.ad_value(1882))), s.ad_value(1881)), s.ad_value(1882)), 1885);
        }

        s.v[2504] = if (s.v[2260] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2503] != 0.0) && (s.v[2504] != 0.0)) {
            s.store_div_from_scalar_ad(1889, 1.0, A::add(A::offset(s.ad_value(2260), 1.0), A::square(s.ad_value(2260))));
        }

        if ((s.v[2503] != 0.0) && (!(s.v[2504] != 0.0))) {
            s.store_sub_from_scalar(1889, 1.0, 2260);
        }

        if (s.v[2503] != 0.0) {
            s.store_mul(1890, 1884, 1889);
        }

        if (s.v[2503] != 0.0) {
            s.store_div(1891, 1886, 1890);
        }

        if (s.v[2503] != 0.0) {
            s.store_mul_ad_lhs(2261, A::mul(A::square(s.ad_value(1891)), s.ad_value(1877)), 1877);
        }

        s.v[2505] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2503] != 0.0) && (s.v[2505] != 0.0)) {
            s.store_div_ad_rhs(2261, 2261, A::offset(A::mul(s.ad_value(1891), s.ad_value(1877)), 1.0));
        }

        if (s.v[2503] != 0.0) {
            s.store_scale_ad(1892, A::mul(s.ad_value(1890), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2261), 2.0), 1.0)), 1.0)), 0.5);
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
        if (s.v[2503] != 0.0) {
            s.store_div(1929, 1890, 1892);
        }

        if (s.v[2503] != 0.0) {
            s.store_mul_ad_rhs(2262, 1880, A::offset(A::scale(A::mul(A::mul(s.ad_value(2261), s.ad_value(1929)), s.ad_value(1929)), 0.5), 1.0));
        }

        if (s.v[2503] != 0.0) {
            s.store_div_ad_lhs(1893, A::mul(s.ad_value(1929), s.ad_value(1882)), 2262);
        }

        if (s.v[2503] != 0.0) {
            s.store_scaled_div(2263, 1877, 1893, 0.5);
        }

        if (s.v[2503] != 0.0) {
            s.store_square(2264, 2263);
        }

        if (s.v[2503] != 0.0) {
            s.store_add_ad_rhs(2265, 1887, A::scale(A::mul(A::mul(s.ad_value(1879), s.ad_value(1877)), A::add(A::offset(A::scale(A::mul(s.ad_value(2263), s.ad_value(1889)), 0.3333333333333333), (-1.0)), s.ad_value(1889))), 0.5));
        }

        if (s.v[2503] != 0.0) {
            s.store_scaled_mul(1929, 1880, 1877, 0.16666666666666666);
        }

        s.v[2506] = if (p.p49 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2503] != 0.0) && (s.v[2506] != 0.0)) {
            s.store_scalar(2266, 0.0);
        }

        if ((s.v[2503] != 0.0) && (s.v[2506] != 0.0)) {
            s.store_mul_ad(2267, A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub(s.ad_value(1881), A::mul(A::scale(s.ad_value(1929), 3.0), A::sub_from_scalar(2.0, s.ad_value(2263)))));
        }

        if ((s.v[2503] != 0.0) && (!(s.v[2506] != 0.0))) {
            s.store_mul_ad(2266, A::sub_from_scalar(1.0, s.ad_value(1889)), A::sub(s.ad_value(1881), A::scale(A::mul(s.ad_value(1880), s.ad_value(1877)), 0.5)));
        }

        if ((s.v[2503] != 0.0) && (!(s.v[2506] != 0.0))) {
            s.store_scale_ad(2267, A::add(A::mul(A::square(s.ad_value(1889)), A::sub(s.ad_value(1881), A::mul(s.ad_value(1929), A::sub(A::sub_from_scalar(1.0, s.ad_value(2263)), A::scale(s.ad_value(2264), 0.2))))), A::mul(s.ad_value(2266), A::offset(s.ad_value(1889), 1.0))), 0.5);
        }

        if (s.v[2503] != 0.0) {
            s.store_add_ad_lhs(2268, A::mul(s.ad_value(1889), A::add(s.ad_value(1881), A::mul(s.ad_value(1929), s.ad_value(2263)))), 2266);
        }

        if (s.v[2503] != 0.0) {
            s.store_sub(2269, 2265, 2268);
        }

        s.store_mul(840, 2265, 1888);

        s.store_mul_ad_lhs(842, A::neg(s.ad_value(2267)), 1888);

        s.store_mul_ad_lhs(841, A::neg(s.ad_value(2269)), 1888);

        s.v[2285] = 0.0;

        s.v[2286] = 0.0;

        s.v[2284] = 0.0;

        s.v[2507] = if ((s.v[266] > 0.0) || (s.v[267] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2507] != 0.0) {
            s.store_scalar(2274, 1.0);
        }

        if (s.v[2507] != 0.0) {
            s.copy_ad(2273, 1871);
        }

        s.v[2508] = if (s.v[270] > 1e-10) { 1.0 } else { 0.0 };

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_add_ad_lhs(2270, A::sub(s.ad_value(1871), s.ad_value(268)), 797);
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_scale_ad(1929, A::add(A::add(s.ad_value(2270), s.ad_value(797)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(2270), s.ad_value(797)), A::sub(s.ad_value(2270), s.ad_value(797))), s.ad_value(798)))), 0.5);
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_mul_ad_rhs(1930, 1929, A::sub(A::sub(A::scale(s.ad_value(1929), 2.0), s.ad_value(797)), s.ad_value(2270)));
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_div(1931, 797, 1929);
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_mul(2271, 2270, 1931);
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_sqrt_ad(2272, A::sub_from_scalar(1.0, A::mul(s.ad_value(2271), s.ad_value(270))));
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_sub_ad_lhs(2273, A::add(A::div(A::sub_from_scalar(1.0, s.ad_value(2272)), s.ad_value(270)), s.ad_value(2270)), 2271);
        }

        if ((s.v[2507] != 0.0) && (s.v[2508] != 0.0)) {
            s.store_offset_ad(2274, A::div(A::mul(A::mul(A::offset(A::div_from_scalar(0.5, s.ad_value(2272)), (-1.0)), A::add(s.ad_value(1930), A::mul(s.ad_value(2270), A::sub(s.ad_value(797), s.ad_value(1929))))), s.ad_value(1931)), s.ad_value(1930)), 1.0);
        }

        if (s.v[2507] != 0.0) {
            s.store_scalar(2276, 1.0);
        }

        if (s.v[2507] != 0.0) {
            s.store_scalar(2277, 0.0);
        }

        s.v[2509] = if (s.v[269] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) {
            s.store_add_ad(1929, A::scale(s.ad_value(734), 0.5), A::mul(s.ad_value(1872), A::offset(A::scale(s.ad_value(1873), 0.7071067811865475), 1.0)));
        }

        if ((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) {
            s.store_div(2275, 1871, 1929);
        }

        s.v[2510] = if (((s.v[2275]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) && (s.v[2510] != 0.0)) {
            s.store_div_from_scalar_ad(2276, 1.0, A::offset(A::exp(A::neg(s.ad_value(2275))), 1.0));
        }

        s.v[2511] = if (s.v[2275] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) && (!(s.v[2510] != 0.0))) && (s.v[2511] != 0.0)) {
            s.store_div_from_scalar_ad(2276, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2275), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2275), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2275), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2512] = if (s.v[2275] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) && (s.v[2512] != 0.0)) {
            s.store_ln_ad(1930, A::offset(A::exp(s.ad_value(2275)), 1.0));
        }

        if (((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) && (!(s.v[2512] != 0.0))) {
            s.copy_ad(1930, 2275);
        }

        if ((s.v[2507] != 0.0) && (s.v[2509] != 0.0)) {
            s.store_mul(2277, 1929, 1930);
        }

        if (s.v[2507] != 0.0) {
            s.store_add_ad_lhs(2278, A::mul(s.ad_value(269), A::sub(s.ad_value(2276), s.ad_value(2274))), 2274);
        }

        if (s.v[2507] != 0.0) {
            s.store_add_ad_lhs(2279, A::mul(s.ad_value(269), A::sub(s.ad_value(2277), s.ad_value(2273))), 2273);
        }

        if (s.v[2507] != 0.0) {
            s.store_sub_ad(2280, A::sub(A::sub(s.ad_value(1871), A::mul(s.ad_value(1872), s.ad_value(1875))), s.ad_value(1887)), A::scale(s.ad_value(1877), 0.5));
        }

        if (s.v[2507] != 0.0) {
            s.store_sub_ad_lhs(2281, A::sub(s.ad_value(1871), s.ad_value(2280)), 1876);
        }

        if (s.v[2507] != 0.0) {
            s.store_sub_ad_lhs(2282, A::add(s.ad_value(1877), s.ad_value(2280)), 815);
        }

        if (s.v[2507] != 0.0) {
            s.store_sub_ad_lhs(2283, A::sub(s.ad_value(1871), s.ad_value(2282)), 1878);
        }

        s.v[2513] = if (s.v[820] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2507] != 0.0) && (s.v[2513] != 0.0)) {
            s.store_mul_ad_rhs(2284, 2278, A::add(A::mul(s.ad_value(267), s.ad_value(2282)), A::mul(s.ad_value(266), s.ad_value(2280))));
        }

        if ((s.v[2507] != 0.0) && (s.v[2513] != 0.0)) {
            s.store_mul_ad_rhs(2285, 266, A::sub(s.ad_value(2281), s.ad_value(2279)));
        }

        if ((s.v[2507] != 0.0) && (s.v[2513] != 0.0)) {
            s.store_mul_ad_rhs(2286, 267, A::sub(s.ad_value(2283), s.ad_value(2279)));
        }

        if ((s.v[2507] != 0.0) && (!(s.v[2513] != 0.0))) {
            s.store_mul_ad_rhs(2284, 2278, A::add(A::mul(s.ad_value(266), s.ad_value(2282)), A::mul(s.ad_value(267), s.ad_value(2280))));
        }

        if ((s.v[2507] != 0.0) && (!(s.v[2513] != 0.0))) {
            s.store_mul_ad_rhs(2285, 267, A::sub(s.ad_value(2281), s.ad_value(2279)));
        }

        if ((s.v[2507] != 0.0) && (!(s.v[2513] != 0.0))) {
            s.store_mul_ad_rhs(2286, 266, A::sub(s.ad_value(2283), s.ad_value(2279)));
        }

        if (s.v[2507] != 0.0) {
            s.store_add(840, 840, 2284);
        }

        if (s.v[2507] != 0.0) {
            s.store_add(842, 842, 2286);
        }

        if (s.v[2507] != 0.0) {
            s.store_sub_ad_lhs(841, A::sub(A::sub(s.ad_value(841), s.ad_value(2284)), s.ad_value(2286)), 2285);
        }

        s.store_mul(1894, 260, 1862);

        s.store_mul(1895, 261, 1863);

        s.v[2289] = 0.0;

        s.v[2287] = 0.0;

        s.v[2514] = if ((s.v[260] > 0.0) && (s.v[262] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2514] != 0.0) {
            s.store_mul_ad_rhs(1929, 264, A::add(A::scale(s.ad_value(1803), 0.5), s.ad_value(776)));
        }

        s.v[2515] = if (s.v[1929] < 230.25850929940458) { 1.0 } else { 0.0 };

        s.v[2516] = if (s.v[1929] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2514] != 0.0) && (s.v[2515] != 0.0)) && (s.v[2516] != 0.0)) {
            s.store_exp(2287, 1929);
        }

        if (((s.v[2514] != 0.0) && (s.v[2515] != 0.0)) && (!(s.v[2516] != 0.0))) {
            s.store_div_from_scalar_ad(2287, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2517] = if (s.v[2287] > 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2514] != 0.0) && (s.v[2515] != 0.0)) && (s.v[2517] != 0.0)) {
            s.store_ln_ad(2288, A::offset(s.ad_value(2287), 1.0));
        }

        if (((s.v[2514] != 0.0) && (s.v[2515] != 0.0)) && (s.v[2517] != 0.0)) {
            s.store_mul_ad_rhs(1930, 2288, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2288), 1.0)), A::offset(s.ad_value(2288), 2.0))));
        }

        if (((s.v[2514] != 0.0) && (s.v[2515] != 0.0)) && (!(s.v[2517] != 0.0))) {
            s.copy_ad(2288, 2287);
        }

        if (((s.v[2514] != 0.0) && (s.v[2515] != 0.0)) && (!(s.v[2517] != 0.0))) {
            s.store_div_ad(1930, A::scale(s.ad_value(2288), 2.0), A::offset(s.ad_value(2288), 2.0));
        }

        if ((s.v[2514] != 0.0) && (!(s.v[2515] != 0.0))) {
            s.copy_ad(2288, 1929);
        }

        if ((s.v[2514] != 0.0) && (!(s.v[2515] != 0.0))) {
            s.store_mul_ad_rhs(1930, 2288, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2288), 1.0)), A::offset(s.ad_value(2288), 2.0))));
        }

        if (s.v[2514] != 0.0) {
            s.store_mul_ad_lhs(2289, A::scale(A::mul(A::div(A::scale(s.ad_value(262), (-2.0)), s.ad_value(264)), s.ad_value(260)), s.v[355]), 1930);
        }

        s.v[2292] = 0.0;

        s.v[2290] = 0.0;

        s.v[2518] = if ((s.v[261] > 0.0) && (s.v[263] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2518] != 0.0) {
            s.store_mul_ad_rhs(1929, 264, A::add(A::scale(s.ad_value(1803), 0.5), s.ad_value(777)));
        }

        s.v[2519] = if (s.v[1929] < 230.25850929940458) { 1.0 } else { 0.0 };

        s.v[2520] = if (s.v[1929] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2518] != 0.0) && (s.v[2519] != 0.0)) && (s.v[2520] != 0.0)) {
            s.store_exp(2290, 1929);
        }

        if (((s.v[2518] != 0.0) && (s.v[2519] != 0.0)) && (!(s.v[2520] != 0.0))) {
            s.store_div_from_scalar_ad(2290, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(1929)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2521] = if (s.v[2290] > 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2518] != 0.0) && (s.v[2519] != 0.0)) && (s.v[2521] != 0.0)) {
            s.store_ln_ad(2291, A::offset(s.ad_value(2290), 1.0));
        }

        if (((s.v[2518] != 0.0) && (s.v[2519] != 0.0)) && (s.v[2521] != 0.0)) {
            s.store_mul_ad_rhs(1930, 2291, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2291), 1.0)), A::offset(s.ad_value(2291), 2.0))));
        }

        if (((s.v[2518] != 0.0) && (s.v[2519] != 0.0)) && (!(s.v[2521] != 0.0))) {
            s.copy_ad(2291, 2290);
        }

        if (((s.v[2518] != 0.0) && (s.v[2519] != 0.0)) && (!(s.v[2521] != 0.0))) {
            s.store_div_ad(1930, A::scale(s.ad_value(2291), 2.0), A::offset(s.ad_value(2291), 2.0));
        }

        if ((s.v[2518] != 0.0) && (!(s.v[2519] != 0.0))) {
            s.copy_ad(2291, 1929);
        }

        if ((s.v[2518] != 0.0) && (!(s.v[2519] != 0.0))) {
            s.store_mul_ad_rhs(1930, 2291, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2291), 1.0)), A::offset(s.ad_value(2291), 2.0))));
        }

        if (s.v[2518] != 0.0) {
            s.store_mul_ad_lhs(2292, A::scale(A::mul(A::div(A::scale(s.ad_value(263), (-2.0)), s.ad_value(264)), s.ad_value(261)), s.v[355]), 1930);
        }

        s.store_add(2293, 2289, 2292);

        s.store_add_ad_lhs(845, A::mul(s.ad_value(265), s.ad_value(818)), 2293);

        s.store_mul(843, 272, 823);

        s.store_mul(844, 273, 826);

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

        s.v[2555] = 0.0;

        s.v[2556] = 0.0;

        s.v[2557] = 0.0;

        s.v[2558] = 0.0;

        s.v[2559] = 0.0;

        s.v[2560] = 0.0;

        s.v[2561] = 0.0;

        s.v[2562] = 0.0;

        s.v[2563] = 0.0;

        s.v[2564] = 0.0;

        s.v[2565] = 0.0;

        s.v[2566] = 0.0;

        s.v[2567] = 0.0;

        s.v[2568] = 0.0;

        s.v[837] = 0.0;

        s.v[1896] = 0.0;

        s.v[1897] = 0.0;

        s.v[1898] = 0.0;

        s.v[838] = 0.0;

        s.v[1899] = 0.0;

        s.v[1900] = 0.0;

        s.v[1901] = 0.0;

        s.v[846] = 0.0;

        s.v[1902] = 0.0;

        s.v[1903] = 0.0;

        s.v[1904] = 0.0;

        s.v[847] = 0.0;

        s.v[1905] = 0.0;

        s.v[1906] = 0.0;

        s.v[1907] = 0.0;

        s.v[2569] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        s.v[2570] = if (s.v[475] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scale(497, 821, (s.v[372] * s.v[669]));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            let assign55470_ad_e69994: A = {
                if (s.v[497] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0))
                } else {
                    {
                        if (s.v[497] > s.v[661]) {
                            A::mul(s.ad_value(662), A::offset(A::sub(s.ad_value(497), s.ad_value(661)), 1.0))
                        } else {
                            A::exp(s.ad_value(497))
                        }
                    }
                }
            };
            s.store_ad(498, &assign55470_ad_e69994);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_mul_ad_rhs(503, 668, A::offset(s.ad_value(498), (-1.0)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_mul_ad_lhs(497, A::scale(s.ad_value(821), s.v[372]), 671);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            let assign55500_ad_e70045: A = {
                if (s.v[497] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0))
                } else {
                    {
                        if (s.v[497] > s.v[663]) {
                            A::mul(s.ad_value(664), A::offset(A::sub(s.ad_value(497), s.ad_value(663)), 1.0))
                        } else {
                            A::exp(s.ad_value(497))
                        }
                    }
                }
            };
            s.store_ad(498, &assign55500_ad_e70045);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_mul_ad_rhs(504, 670, A::offset(s.ad_value(498), (-1.0)));
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
        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scalar(505, 0.0);
        }

        s.v[2571] = if (s.v[667] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2571] != 0.0)) {
            s.store_mul_ad_rhs(505, 821, A::add(s.ad_value(672), A::mul(s.ad_value(821), s.ad_value(673))));
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (!(s.v[2571] != 0.0))) {
            s.store_mul_ad_lhs(497, A::scale(A::neg(s.ad_value(821)), s.v[372]), 673);
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (!(s.v[2571] != 0.0))) {
            let assign55560_ad_e70126: A = {
                if (s.v[497] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0))
                } else {
                    {
                        if (s.v[497] > s.v[665]) {
                            A::mul(s.ad_value(666), A::offset(A::sub(s.ad_value(497), s.ad_value(665)), 1.0))
                        } else {
                            A::exp(s.ad_value(497))
                        }
                    }
                }
            };
            s.store_ad(498, &assign55560_ad_e70126);
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (!(s.v[2571] != 0.0))) {
            s.store_mul_ad(505, A::neg(s.ad_value(672)), A::offset(s.ad_value(498), (-1.0)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_add_ad_lhs(837, A::add(s.ad_value(503), s.ad_value(504)), 505);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scale(497, 822, (s.v[372] * s.v[696]));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            let assign55600_ad_e70191: A = {
                if (s.v[497] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0))
                } else {
                    {
                        if (s.v[497] > s.v[688]) {
                            A::mul(s.ad_value(689), A::offset(A::sub(s.ad_value(497), s.ad_value(688)), 1.0))
                        } else {
                            A::exp(s.ad_value(497))
                        }
                    }
                }
            };
            s.store_ad(498, &assign55600_ad_e70191);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_mul_ad_rhs(503, 695, A::offset(s.ad_value(498), (-1.0)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_mul_ad_lhs(497, A::scale(s.ad_value(822), s.v[372]), 698);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            let assign55630_ad_e70242: A = {
                if (s.v[497] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0))
                } else {
                    {
                        if (s.v[497] > s.v[690]) {
                            A::mul(s.ad_value(691), A::offset(A::sub(s.ad_value(497), s.ad_value(690)), 1.0))
                        } else {
                            A::exp(s.ad_value(497))
                        }
                    }
                }
            };
            s.store_ad(498, &assign55630_ad_e70242);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_mul_ad_rhs(504, 697, A::offset(s.ad_value(498), (-1.0)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scalar(505, 0.0);
        }

        s.v[2572] = if (s.v[694] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2572] != 0.0)) {
            s.store_mul_ad_rhs(505, 822, A::add(s.ad_value(699), A::mul(s.ad_value(822), s.ad_value(700))));
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (!(s.v[2572] != 0.0))) {
            s.store_mul_ad_lhs(497, A::scale(A::neg(s.ad_value(822)), s.v[372]), 700);
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (!(s.v[2572] != 0.0))) {
            let assign55690_ad_e70323: A = {
                if (s.v[497] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(497)), 1.0))
                } else {
                    {
                        if (s.v[497] > s.v[692]) {
                            A::mul(s.ad_value(693), A::offset(A::sub(s.ad_value(497), s.ad_value(692)), 1.0))
                        } else {
                            A::exp(s.ad_value(497))
                        }
                    }
                }
            };
            s.store_ad(498, &assign55690_ad_e70323);
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (!(s.v[2572] != 0.0))) {
            s.store_mul_ad(505, A::neg(s.ad_value(699)), A::offset(s.ad_value(498), (-1.0)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_add_ad_lhs(838, A::add(s.ad_value(503), s.ad_value(504)), 505);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scalar(2573, 0.0);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scalar(2574, 0.0);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(658), 4.0), 658);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_div(2526, 658, 659);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_add_ad_rhs(2527, 821, A::mul(s.ad_value(658), s.ad_value(2526)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_add(2528, 659, 2527);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_sub(2529, 659, 2527);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scale_ad(2574, A::div(A::mul(s.ad_value(821), s.ad_value(659)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2575] = if (s.v[652] > 0.5) { 1.0 } else { 0.0 };

        s.v[2576] = if (s.v[409] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_sqrt_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[406])));
        }

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2575] != 0.0)) && (!(s.v[2576] != 0.0))) {
            s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[406])), s.v[409]);
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2575] != 0.0)) {
            s.store_add_ad(1902, A::scale(A::sub_from_scalar(1.0, s.ad_value(2573)), s.v[418]), A::scale(A::sub(s.ad_value(821), s.ad_value(2574)), s.v[421]));
        }

        s.v[2577] = if (s.v[653] > 0.5) { 1.0 } else { 0.0 };

        s.v[2578] = if (s.v[410] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2577] != 0.0)) && (s.v[2578] != 0.0)) {
            s.store_sqrt_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[407])));
        }

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2577] != 0.0)) && (!(s.v[2578] != 0.0))) {
            s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[407])), s.v[410]);
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2577] != 0.0)) {
            s.store_add_ad(1903, A::scale(A::sub_from_scalar(1.0, s.ad_value(2573)), s.v[419]), A::scale(A::sub(s.ad_value(821), s.ad_value(2574)), s.v[422]));
        }

        s.v[2579] = if (s.v[654] > 0.5) { 1.0 } else { 0.0 };

        s.v[2580] = if (s.v[411] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2579] != 0.0)) && (s.v[2580] != 0.0)) {
            s.store_sqrt_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[408])));
        }

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2579] != 0.0)) && (!(s.v[2580] != 0.0))) {
            s.store_powf_ad(2573, A::sub_from_scalar(1.0, A::scale(s.ad_value(2574), s.v[408])), s.v[411]);
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_add_ad(1904, A::scale(A::sub_from_scalar(1.0, s.ad_value(2573)), s.v[420]), A::scale(A::sub(s.ad_value(821), s.ad_value(2574)), s.v[423]));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scalar(2573, 0.0);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scalar(2574, 0.0);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(685), 4.0), 685);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_div(2526, 685, 686);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_add_ad_rhs(2527, 822, A::mul(s.ad_value(685), s.ad_value(2526)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_add(2528, 686, 2527);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_sub(2529, 686, 2527);
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if ((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) {
            s.store_scale_ad(2574, A::div(A::mul(s.ad_value(822), s.ad_value(686)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2581] = if (s.v[679] > 0.5) { 1.0 } else { 0.0 };

        s.v[2582] = if (s.v[576] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2581] != 0.0)) && (s.v[2582] != 0.0)) {
            s.store_sqrt_ad(2573, A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(573))));
        }

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2581] != 0.0)) && (!(s.v[2582] != 0.0))) {
            s.store_ad(2573, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(573))), s.ad_value(576)));
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2581] != 0.0)) {
            s.store_add_ad(1905, A::mul(s.ad_value(585), A::sub_from_scalar(1.0, s.ad_value(2573))), A::mul(s.ad_value(588), A::sub(s.ad_value(822), s.ad_value(2574))));
        }

        s.v[2583] = if (s.v[680] > 0.5) { 1.0 } else { 0.0 };

        s.v[2584] = if (s.v[577] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2583] != 0.0)) && (s.v[2584] != 0.0)) {
            s.store_sqrt_ad(2573, A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(574))));
        }

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2583] != 0.0)) && (!(s.v[2584] != 0.0))) {
            s.store_ad(2573, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(574))), s.ad_value(577)));
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2583] != 0.0)) {
            s.store_add_ad(1906, A::mul(s.ad_value(586), A::sub_from_scalar(1.0, s.ad_value(2573))), A::mul(s.ad_value(589), A::sub(s.ad_value(822), s.ad_value(2574))));
        }

        s.v[2585] = if (s.v[681] > 0.5) { 1.0 } else { 0.0 };

        s.v[2586] = if (s.v[578] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2585] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_sqrt_ad(2573, A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(575))));
        }

        if ((((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2585] != 0.0)) && (!(s.v[2586] != 0.0))) {
            s.store_ad(2573, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2574), s.ad_value(575))), s.ad_value(578)));
        }

        if (((s.v[2569] != 0.0) && (s.v[2570] != 0.0)) && (s.v[2585] != 0.0)) {
            s.store_add_ad(1907, A::mul(s.ad_value(587), A::sub_from_scalar(1.0, s.ad_value(2573))), A::mul(s.ad_value(590), A::sub(s.ad_value(822), s.ad_value(2574))));
        }

        s.v[2587] = if (p.p889 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2587] != 0.0)) {
            s.store_scale_ad(643, A::offset(A::powf(A::scale(A::add(A::add(s.ad_value(814), s.ad_value(816)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(814), s.ad_value(816)), A::add(s.ad_value(814), s.ad_value(816))), (0.001 * 0.001)))), 0.5), p.p890), (-(((0.5 * 0.001)) as f64).powf(p.p890))), p.p889);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2587] != 0.0)) {
            s.store_offset(641, 643, p.p879);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2587] != 0.0)) {
            s.store_div_from_scalar(451, 1.0, 641);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2587] != 0.0)) {
            s.store_div_from_scalar_ad(454, s.v[454], A::offset(A::scale(s.ad_value(643), 1.0 / (p.p879)), 1.0));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2587] != 0.0))) {
            s.store_scalar(641, p.p879);
        }

        s.v[2588] = if (p.p891 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2588] != 0.0)) {
            s.store_scale_ad(645, A::offset(A::powf(A::scale(A::add(A::add(s.ad_value(814), s.ad_value(816)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(814), s.ad_value(816)), A::add(s.ad_value(814), s.ad_value(816))), (0.001 * 0.001)))), 0.5), p.p892), (-(((0.5 * 0.001)) as f64).powf(p.p892))), p.p891);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2588] != 0.0)) {
            s.store_mul_ad_rhs(444, 444, A::offset(s.ad_value(645), 1.0));
        }

        if ((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) {
            s.store_scalar(2538, 0.0);
        }

        if ((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) {
            s.store_scalar(2535, 0.0);
        }

        s.v[2589] = if !(((s.v[647] == 0.0) && (s.v[648] == 0.0)) && (s.v[649] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(658), 4.0), 658);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_div(2526, 658, 659);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_add_ad_rhs(2527, 821, A::mul(s.ad_value(658), s.ad_value(2526)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_add(2528, 659, 2527);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_sub(2529, 659, 2527);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_scale_ad(2532, A::div(A::mul(s.ad_value(821), s.ad_value(659)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2590] = if (s.v[821] < s.v[655]) { 1.0 } else { 0.0 };

        s.v[2591] = if (((((-0.5) * (s.v[821] * s.v[372]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (s.v[2590] != 0.0)) && (s.v[2591] != 0.0)) {
            s.store_exp_ad(2533, A::scale(s.ad_value(821), (s.v[372] * (-0.5))));
        }

        s.v[2592] = if (((-0.5) * (s.v[821] * s.v[372])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (s.v[2590] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2592] != 0.0)) {
            let assign56430_ad_e71206: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(821), (s.v[372] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(821), (s.v[372] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(821), (s.v[372] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2533, &assign56430_ad_e71206);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (s.v[2590] != 0.0)) && (!(s.v[2591] != 0.0))) && (!(s.v[2592] != 0.0))) {
            s.store_scale_ad(2533, A::offset(A::mul(A::offset(A::scale(s.ad_value(821), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(821), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(821), (s.v[372] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (s.v[2590] != 0.0)) {
            s.store_div_from_scalar(2534, 1.0, 2533);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (s.v[2590] != 0.0)) {
            s.store_square(2531, 2534);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (!(s.v[2590] != 0.0))) {
            s.store_mul_ad_lhs(2531, A::offset(A::scale(A::sub(s.ad_value(821), s.ad_value(655)), s.v[372]), 1.0), 656);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (!(s.v[2590] != 0.0))) {
            s.store_sqrt(2534, 2531);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (!(s.v[2590] != 0.0))) {
            s.store_div_from_scalar(2533, 1.0, 2534);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_offset(2531, 2531, (-1.0));
        }

        s.v[2593] = if (s.v[821] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (s.v[2593] != 0.0)) {
            s.store_scale_ad(2535, A::ln(A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2533), 1.0), A::offset(s.ad_value(2533), 3.0))))), (s.v[371] * 2.0));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) && (!(s.v[2593] != 0.0))) {
            s.store_sub_ad_lhs(2535, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2534), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2534), 1.0), A::offset(A::scale(s.ad_value(2534), 3.0), 1.0))))), (s.v[371] * 2.0)), 821);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_sub(2536, 657, 2535);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_scale_ad(2537, A::sub(A::add(s.ad_value(821), s.ad_value(2536)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(821), s.ad_value(2536)), A::sub(s.ad_value(821), s.ad_value(2536))), ((4.0 * s.v[371]) * s.v[371])))), 0.5);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_scale_ad(2538, A::sub(A::add(s.ad_value(821), s.ad_value(660)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(821), s.ad_value(660)), A::sub(s.ad_value(821), s.ad_value(660))), ((4.0 * s.v[369]) * s.v[369])))), 0.5);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_scale_ad(2539, A::sub(s.ad_value(821), A::sqrt(A::offset(A::mul(s.ad_value(821), s.ad_value(821)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[2594] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2594] != 0.0)) {
            s.store_scalar(1896, 0.0);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2594] != 0.0)) {
            s.store_scalar(1902, 0.0);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) {
            s.store_scale(2541, 2531, s.v[388]);
        }

        s.v[2595] = if ((p.p857 == 0.0) && (p.p862 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (s.v[2595] != 0.0)) {
            s.store_scalar(2542, 0.0);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) {
            s.store_sub_from_scalar(2543, s.v[394], 2537);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) {
            s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));
        }

        s.v[2596] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) && (s.v[2596] != 0.0)) {
            s.store_scalar(2545, 0.0);
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) && (!(s.v[2596] != 0.0))) {
            s.store_scale_ad(2545, A::add(A::div(A::mul(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544))), A::sub_from_scalar(1.0, s.ad_value(2544))), s.ad_value(2544)), (1.0 - (2.0 * p.p848)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) {
            s.store_add(2546, 2544, 2545);
        }

        s.v[2597] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) && (s.v[2597] != 0.0)) {
            s.store_sqrt_ad(2540, A::scale(s.ad_value(2543), s.v[430]));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) && (!(s.v[2597] != 0.0))) {
            s.store_powf_ad(2540, A::scale(s.ad_value(2543), s.v[430]), p.p848);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) {
            s.store_scale(2547, 2540, s.v[424]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) {
            s.store_scale_ad(2548, A::mul(A::offset(s.ad_value(2534), (-1.0)), s.ad_value(2547)), s.v[385]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2595] != 0.0))) {
            s.store_scaled_mul(2542, 2548, 2546, p.p857);
        }

        s.v[2598] = if (p.p862 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (s.v[2598] != 0.0)) {
            s.store_scalar(2549, 0.0);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_scale_ad(2550, A::div(A::scale(s.ad_value(2547), s.v[409]), s.ad_value(2543)), s.v[439]);
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
        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[436]), 2550);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_square(2552, 2551);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_sqrt_ad(2553, A::div(A::square(s.ad_value(2552)), A::offset(A::square(s.ad_value(2552)), 1.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_sqrt(2554, 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_mul(2555, 2553, 2554);
        }

        s.v[2599] = if (((-p.p848) * s.v[412]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (s.v[2599] != 0.0)) {
            s.store_div_from_scalar_ad(2556, 1.0, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2599] != 0.0))) {
            s.store_powf_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), ((-p.p848) * s.v[412]));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_div_ad(2557, A::mul(s.ad_value(2546), s.ad_value(2556)), A::add(s.ad_value(2546), s.ad_value(2556)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_sqrt_ad(2558, A::scale(A::div(s.ad_value(2550), s.ad_value(2554)), 0.375));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_sub_ad_lhs(2559, A::scale(A::mul(s.ad_value(2551), s.ad_value(2554)), 2.0), 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_add_ad(2560, A::sub(A::mul(A::scale(s.ad_value(2551), s.v[436]), s.ad_value(2554)), A::scale(s.ad_value(2553), s.v[436])), A::scale(A::mul(s.ad_value(2550), s.ad_value(2555)), 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_mul_ad_lhs(2561, A::offset(s.ad_value(2559), (-1.0)), 2558);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_square(2522, 2561);
        }

        s.v[2600] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (s.v[2600] != 0.0)) {
            s.store_div_from_scalar_ad(2523, 1.0, A::offset(A::scale(s.ad_value(2561), s.v[373]), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2600] != 0.0))) {
            s.store_div_from_scalar_ad(2523, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2561), s.v[373])));
        }

        s.v[2601] = if (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (s.v[2601] != 0.0)) {
            s.store_exp_ad(2540, A::sub(s.ad_value(2560), s.ad_value(2522)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2601] != 0.0))) {
            let assign56980_ad_e72156: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2540, &assign56980_ad_e72156);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_mul_ad_lhs(2524, A::add(A::add(A::scale(s.ad_value(2523), 0.29214664), A::scale(A::square(s.ad_value(2523)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(2523)), s.ad_value(2523)), s.v[375])), 2540);
        }

        s.v[2602] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (s.v[2602] != 0.0)) {
            s.copy_ad(2562, 2524);
        }

        s.v[2603] = if (s.v[2560] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (s.v[2603] != 0.0)) {
            s.store_exp(2540, 2560);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) && (!(s.v[2603] != 0.0))) {
            s.store_div_from_scalar_ad(2540, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) && (!(s.v[2602] != 0.0))) {
            s.store_sub_ad_lhs(2562, A::scale(s.ad_value(2540), 2.0), 2524);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_scale_ad(2563, A::div(A::scale(s.ad_value(2562), s.v[436]), s.ad_value(2558)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2598] != 0.0))) {
            s.store_scale_ad(2549, A::mul(A::mul(s.ad_value(2548), s.ad_value(2563)), s.ad_value(2557)), p.p862);
        }

        s.v[2604] = if (p.p868 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (s.v[2604] != 0.0)) {
            s.store_scalar(2564, 0.0);
        }

        s.v[2605] = if (p.p848 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) && (s.v[2605] != 0.0)) {
            s.store_sqrt_ad(2540, A::scale(A::sub_from_scalar(p.p845, s.ad_value(2538)), s.v[430]));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) && (!(s.v[2605] != 0.0))) {
            s.store_powf_ad(2540, A::scale(A::sub_from_scalar(p.p845, s.ad_value(2538)), s.v[430]), p.p848);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) {
            s.store_scale_ad(2565, A::div(A::scale(A::sub_from_scalar(p.p845, s.ad_value(2538)), s.v[427]), s.ad_value(2540)), s.v[412]);
        }

        s.v[2606] = if (((((-s.v[442]) / s.v[2565])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) && (s.v[2606] != 0.0)) {
            s.store_exp_ad(2540, A::div(A::neg(s.ad_value(442)), s.ad_value(2565)));
        }

        s.v[2607] = if (((-s.v[442]) / s.v[2565]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) && (!(s.v[2606] != 0.0))) && (s.v[2607] != 0.0)) {
            let assign57170_ad_e72496: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2565))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2565))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2565))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2540, 1e-100, assign57170_ad_e72496);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) && (!(s.v[2606] != 0.0))) && (!(s.v[2607] != 0.0))) {
            let assign57180_ad_e72547: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2565)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2540, &assign57180_ad_e72547);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2604] != 0.0))) {
            s.store_scale_ad(2564, A::mul(A::mul(A::mul(s.ad_value(821), s.ad_value(2565)), s.ad_value(2565)), s.ad_value(2540)), p.p868);
        }

        s.v[2608] = if (p.p877 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (s.v[2608] != 0.0)) {
            s.store_scalar(2566, 1.0);
        }

        s.v[2609] = if (s.v[2539] > ((-s.v[445]) * p.p877)) { 1.0 } else { 0.0 };

        s.v[2610] = if (p.p880 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2608] != 0.0))) && (s.v[2609] != 0.0)) && (s.v[2610] != 0.0)) {
            s.store_mul_ad(2540, A::mul(A::mul(A::scale(s.ad_value(2539), s.v[449]), A::scale(s.ad_value(2539), s.v[449])), A::scale(s.ad_value(2539), s.v[449])), A::scale(s.ad_value(2539), s.v[449]));
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2608] != 0.0))) && (s.v[2609] != 0.0)) && (!(s.v[2610] != 0.0))) {
            s.store_powf_ad(2540, A::abs(A::scale(s.ad_value(2539), s.v[449])), p.p880);
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2608] != 0.0))) && (s.v[2609] != 0.0)) {
            s.store_div_from_scalar_ad(2566, 1.0, A::sub_from_scalar(1.0, s.ad_value(2540)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2608] != 0.0))) && (!(s.v[2609] != 0.0))) {
            s.store_offset_ad(2566, A::scale(A::offset(s.ad_value(2539), (s.v[445] * p.p877)), s.v[452]), s.v[446]);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) {
            s.store_mul_ad_lhs(1896, A::scale(A::add(A::add(A::add(s.ad_value(2541), s.ad_value(2542)), s.ad_value(2549)), s.ad_value(2564)), p.p29), 2566);
        }

        s.v[2611] = if (s.v[409] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (s.v[2611] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[406])));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) && (!(s.v[2611] != 0.0))) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[406])), s.v[409]);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2594] != 0.0))) {
            s.store_scale_ad(1902, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2540)), s.v[418]), A::scale(A::sub(s.ad_value(821), s.ad_value(2532)), s.v[421])), p.p30);
        }

        s.v[2612] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2612] != 0.0)) {
            s.store_scalar(1897, 0.0);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2612] != 0.0)) {
            s.store_scalar(1903, 0.0);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) {
            s.store_scale(2541, 2531, s.v[389]);
        }

        s.v[2613] = if ((p.p858 == 0.0) && (p.p863 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (s.v[2613] != 0.0)) {
            s.store_scalar(2542, 0.0);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) {
            s.store_sub_from_scalar(2543, s.v[395], 2537);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) {
            s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));
        }

        s.v[2614] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) && (s.v[2614] != 0.0)) {
            s.store_scalar(2545, 0.0);
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) && (!(s.v[2614] != 0.0))) {
            s.store_scale_ad(2545, A::add(A::div(A::mul(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544))), A::sub_from_scalar(1.0, s.ad_value(2544))), s.ad_value(2544)), (1.0 - (2.0 * p.p849)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) {
            s.store_add(2546, 2544, 2545);
        }

        s.v[2615] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) && (s.v[2615] != 0.0)) {
            s.store_sqrt_ad(2540, A::scale(s.ad_value(2543), s.v[431]));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) && (!(s.v[2615] != 0.0))) {
            s.store_powf_ad(2540, A::scale(s.ad_value(2543), s.v[431]), p.p849);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) {
            s.store_scale(2547, 2540, s.v[425]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) {
            s.store_scale_ad(2548, A::mul(A::offset(s.ad_value(2534), (-1.0)), s.ad_value(2547)), s.v[386]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2613] != 0.0))) {
            s.store_scaled_mul(2542, 2548, 2546, p.p858);
        }

        s.v[2616] = if (p.p863 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (s.v[2616] != 0.0)) {
            s.store_scalar(2549, 0.0);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_scale_ad(2550, A::div(A::scale(s.ad_value(2547), s.v[410]), s.ad_value(2543)), s.v[440]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[437]), 2550);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_square(2552, 2551);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_sqrt_ad(2553, A::div(A::square(s.ad_value(2552)), A::offset(A::square(s.ad_value(2552)), 1.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_sqrt(2554, 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_mul(2555, 2553, 2554);
        }

        s.v[2617] = if (((-p.p849) * s.v[413]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2617] != 0.0)) {
            s.store_div_from_scalar_ad(2556, 1.0, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2617] != 0.0))) {
            s.store_powf_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), ((-p.p849) * s.v[413]));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_div_ad(2557, A::mul(s.ad_value(2546), s.ad_value(2556)), A::add(s.ad_value(2546), s.ad_value(2556)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_sqrt_ad(2558, A::scale(A::div(s.ad_value(2550), s.ad_value(2554)), 0.375));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_sub_ad_lhs(2559, A::scale(A::mul(s.ad_value(2551), s.ad_value(2554)), 2.0), 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_add_ad(2560, A::sub(A::mul(A::scale(s.ad_value(2551), s.v[437]), s.ad_value(2554)), A::scale(s.ad_value(2553), s.v[437])), A::scale(A::mul(s.ad_value(2550), s.ad_value(2555)), 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_mul_ad_lhs(2561, A::offset(s.ad_value(2559), (-1.0)), 2558);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_square(2522, 2561);
        }

        s.v[2618] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2618] != 0.0)) {
            s.store_div_from_scalar_ad(2523, 1.0, A::offset(A::scale(s.ad_value(2561), s.v[373]), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2618] != 0.0))) {
            s.store_div_from_scalar_ad(2523, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2561), s.v[373])));
        }

        s.v[2619] = if (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2619] != 0.0)) {
            s.store_exp_ad(2540, A::sub(s.ad_value(2560), s.ad_value(2522)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2619] != 0.0))) {
            let assign57730_ad_e73422: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2540, &assign57730_ad_e73422);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_mul_ad_lhs(2524, A::add(A::add(A::scale(s.ad_value(2523), 0.29214664), A::scale(A::square(s.ad_value(2523)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(2523)), s.ad_value(2523)), s.v[375])), 2540);
        }

        s.v[2620] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (s.v[2620] != 0.0)) {
            s.copy_ad(2562, 2524);
        }

        s.v[2621] = if (s.v[2560] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (s.v[2621] != 0.0)) {
            s.store_exp(2540, 2560);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) && (!(s.v[2621] != 0.0))) {
            s.store_div_from_scalar_ad(2540, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) && (!(s.v[2620] != 0.0))) {
            s.store_sub_ad_lhs(2562, A::scale(s.ad_value(2540), 2.0), 2524);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_scale_ad(2563, A::div(A::scale(s.ad_value(2562), s.v[437]), s.ad_value(2558)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2616] != 0.0))) {
            s.store_scale_ad(2549, A::mul(A::mul(s.ad_value(2548), s.ad_value(2563)), s.ad_value(2557)), p.p863);
        }

        s.v[2622] = if (p.p869 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (s.v[2622] != 0.0)) {
            s.store_scalar(2564, 0.0);
        }

        s.v[2623] = if (p.p849 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) && (s.v[2623] != 0.0)) {
            s.store_sqrt_ad(2540, A::scale(A::sub_from_scalar(p.p846, s.ad_value(2538)), s.v[431]));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) && (!(s.v[2623] != 0.0))) {
            s.store_powf_ad(2540, A::scale(A::sub_from_scalar(p.p846, s.ad_value(2538)), s.v[431]), p.p849);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) {
            s.store_scale_ad(2565, A::div(A::scale(A::sub_from_scalar(p.p846, s.ad_value(2538)), s.v[428]), s.ad_value(2540)), s.v[413]);
        }

        s.v[2624] = if (((((-s.v[443]) / s.v[2565])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) && (s.v[2624] != 0.0)) {
            s.store_exp_ad(2540, A::div(A::neg(s.ad_value(443)), s.ad_value(2565)));
        }

        s.v[2625] = if (((-s.v[443]) / s.v[2565]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) && (!(s.v[2624] != 0.0))) && (s.v[2625] != 0.0)) {
            let assign57920_ad_e73762: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2565))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2565))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2565))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2540, 1e-100, assign57920_ad_e73762);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) && (!(s.v[2624] != 0.0))) && (!(s.v[2625] != 0.0))) {
            let assign57930_ad_e73813: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2565)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2540, &assign57930_ad_e73813);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2622] != 0.0))) {
            s.store_scale_ad(2564, A::mul(A::mul(A::mul(s.ad_value(821), s.ad_value(2565)), s.ad_value(2565)), s.ad_value(2540)), p.p869);
        }

        s.v[2626] = if (p.p878 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (s.v[2626] != 0.0)) {
            s.store_scalar(2566, 1.0);
        }

        s.v[2627] = if (s.v[2539] > ((-s.v[445]) * p.p878)) { 1.0 } else { 0.0 };

        s.v[2628] = if (p.p881 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2626] != 0.0))) && (s.v[2627] != 0.0)) && (s.v[2628] != 0.0)) {
            s.store_mul_ad(2540, A::mul(A::mul(A::scale(s.ad_value(2539), s.v[450]), A::scale(s.ad_value(2539), s.v[450])), A::scale(s.ad_value(2539), s.v[450])), A::scale(s.ad_value(2539), s.v[450]));
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2626] != 0.0))) && (s.v[2627] != 0.0)) && (!(s.v[2628] != 0.0))) {
            s.store_powf_ad(2540, A::abs(A::scale(s.ad_value(2539), s.v[450])), p.p881);
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2626] != 0.0))) && (s.v[2627] != 0.0)) {
            s.store_div_from_scalar_ad(2566, 1.0, A::sub_from_scalar(1.0, s.ad_value(2540)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2626] != 0.0))) && (!(s.v[2627] != 0.0))) {
            s.store_offset_ad(2566, A::scale(A::offset(s.ad_value(2539), (s.v[445] * p.p878)), s.v[453]), s.v[447]);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) {
            s.store_mul_ad_lhs(1897, A::scale(A::add(A::add(A::add(s.ad_value(2541), s.ad_value(2542)), s.ad_value(2549)), s.ad_value(2564)), p.p29), 2566);
        }

        s.v[2629] = if (s.v[410] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (s.v[2629] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[407])));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) && (!(s.v[2629] != 0.0))) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[407])), s.v[410]);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2612] != 0.0))) {
            s.store_scale_ad(1903, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2540)), s.v[419]), A::scale(A::sub(s.ad_value(821), s.ad_value(2532)), s.v[422])), p.p30);
        }

        s.v[2630] = if (s.v[649] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2630] != 0.0)) {
            s.store_scalar(1898, 0.0);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2630] != 0.0)) {
            s.store_scalar(1904, 0.0);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) {
            s.store_scale(2541, 2531, s.v[390]);
        }

        s.v[2631] = if ((p.p859 == 0.0) && (p.p864 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2631] != 0.0)) {
            s.store_scalar(2542, 0.0);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) {
            s.store_sub_from_scalar(2543, s.v[396], 2537);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) {
            s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));
        }

        s.v[2632] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) && (s.v[2632] != 0.0)) {
            s.store_scalar(2545, 0.0);
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) && (!(s.v[2632] != 0.0))) {
            s.store_scale_ad(2545, A::add(A::div(A::mul(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544))), A::sub_from_scalar(1.0, s.ad_value(2544))), s.ad_value(2544)), (1.0 - (2.0 * p.p850)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) {
            s.store_add(2546, 2544, 2545);
        }

        s.v[2633] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) && (s.v[2633] != 0.0)) {
            s.store_sqrt_ad(2540, A::scale(s.ad_value(2543), s.v[432]));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) && (!(s.v[2633] != 0.0))) {
            s.store_powf_ad(2540, A::scale(s.ad_value(2543), s.v[432]), p.p850);
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
        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) {
            s.store_scale(2547, 2540, s.v[426]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) {
            s.store_scale_ad(2548, A::mul(A::offset(s.ad_value(2534), (-1.0)), s.ad_value(2547)), s.v[387]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2631] != 0.0))) {
            s.store_scaled_mul(2542, 2548, 2546, p.p859);
        }

        s.v[2634] = if (p.p864 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2634] != 0.0)) {
            s.store_scalar(2549, 0.0);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_scale_ad(2550, A::div(A::scale(s.ad_value(2547), s.v[411]), s.ad_value(2543)), s.v[441]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_div_from_scalar(2551, (0.666666666666667 * s.v[438]), 2550);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_square(2552, 2551);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_sqrt_ad(2553, A::div(A::square(s.ad_value(2552)), A::offset(A::square(s.ad_value(2552)), 1.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_sqrt(2554, 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_mul(2555, 2553, 2554);
        }

        s.v[2635] = if (((-p.p850) * s.v[414]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (s.v[2635] != 0.0)) {
            s.store_div_from_scalar_ad(2556, 1.0, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (!(s.v[2635] != 0.0))) {
            s.store_powf_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), ((-p.p850) * s.v[414]));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_div_ad(2557, A::mul(s.ad_value(2546), s.ad_value(2556)), A::add(s.ad_value(2546), s.ad_value(2556)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_sqrt_ad(2558, A::scale(A::div(s.ad_value(2550), s.ad_value(2554)), 0.375));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_sub_ad_lhs(2559, A::scale(A::mul(s.ad_value(2551), s.ad_value(2554)), 2.0), 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_add_ad(2560, A::sub(A::mul(A::scale(s.ad_value(2551), s.v[438]), s.ad_value(2554)), A::scale(s.ad_value(2553), s.v[438])), A::scale(A::mul(s.ad_value(2550), s.ad_value(2555)), 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_mul_ad_lhs(2561, A::offset(s.ad_value(2559), (-1.0)), 2558);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_square(2522, 2561);
        }

        s.v[2636] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (s.v[2636] != 0.0)) {
            s.store_div_from_scalar_ad(2523, 1.0, A::offset(A::scale(s.ad_value(2561), s.v[373]), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (!(s.v[2636] != 0.0))) {
            s.store_div_from_scalar_ad(2523, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2561), s.v[373])));
        }

        s.v[2637] = if (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (s.v[2637] != 0.0)) {
            s.store_exp_ad(2540, A::sub(s.ad_value(2560), s.ad_value(2522)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (!(s.v[2637] != 0.0))) {
            let assign58480_ad_e74688: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2540, &assign58480_ad_e74688);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_mul_ad_lhs(2524, A::add(A::add(A::scale(s.ad_value(2523), 0.29214664), A::scale(A::square(s.ad_value(2523)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(2523)), s.ad_value(2523)), s.v[375])), 2540);
        }

        s.v[2638] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (s.v[2638] != 0.0)) {
            s.copy_ad(2562, 2524);
        }

        s.v[2639] = if (s.v[2560] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (!(s.v[2638] != 0.0))) && (s.v[2639] != 0.0)) {
            s.store_exp(2540, 2560);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (!(s.v[2638] != 0.0))) && (!(s.v[2639] != 0.0))) {
            s.store_div_from_scalar_ad(2540, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) && (!(s.v[2638] != 0.0))) {
            s.store_sub_ad_lhs(2562, A::scale(s.ad_value(2540), 2.0), 2524);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_scale_ad(2563, A::div(A::scale(s.ad_value(2562), s.v[438]), s.ad_value(2558)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2634] != 0.0))) {
            s.store_scale_ad(2549, A::mul(A::mul(s.ad_value(2548), s.ad_value(2563)), s.ad_value(2557)), p.p864);
        }

        s.v[2640] = if (p.p870 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2640] != 0.0)) {
            s.store_scalar(2564, 0.0);
        }

        s.v[2641] = if (p.p850 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) && (s.v[2641] != 0.0)) {
            s.store_sqrt_ad(2540, A::scale(A::sub_from_scalar(p.p847, s.ad_value(2538)), s.v[432]));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) && (!(s.v[2641] != 0.0))) {
            s.store_powf_ad(2540, A::scale(A::sub_from_scalar(p.p847, s.ad_value(2538)), s.v[432]), p.p850);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) {
            s.store_scale_ad(2565, A::div(A::scale(A::sub_from_scalar(p.p847, s.ad_value(2538)), s.v[429]), s.ad_value(2540)), s.v[414]);
        }

        s.v[2642] = if (((((-s.v[444]) / s.v[2565])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) && (s.v[2642] != 0.0)) {
            s.store_exp_ad(2540, A::div(A::neg(s.ad_value(444)), s.ad_value(2565)));
        }

        s.v[2643] = if (((-s.v[444]) / s.v[2565]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) && (!(s.v[2642] != 0.0))) && (s.v[2643] != 0.0)) {
            let assign58670_ad_e75028: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(2565))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(2565))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(444)), s.ad_value(2565))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2540, 1e-100, assign58670_ad_e75028);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) && (!(s.v[2642] != 0.0))) && (!(s.v[2643] != 0.0))) {
            let assign58680_ad_e75079: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(444)), s.ad_value(2565)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2540, &assign58680_ad_e75079);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2640] != 0.0))) {
            s.store_scale_ad(2564, A::mul(A::mul(A::mul(s.ad_value(821), s.ad_value(2565)), s.ad_value(2565)), s.ad_value(2540)), p.p870);
        }

        s.v[2644] = if (s.v[641] > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2644] != 0.0)) {
            s.store_scalar(2566, 1.0);
        }

        s.v[2645] = if (s.v[2539] > ((-s.v[445]) * s.v[641])) { 1.0 } else { 0.0 };

        s.v[2646] = if (p.p882 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2644] != 0.0))) && (s.v[2645] != 0.0)) && (s.v[2646] != 0.0)) {
            s.store_mul_ad(2540, A::mul(A::mul(A::mul(s.ad_value(2539), s.ad_value(451)), A::mul(s.ad_value(2539), s.ad_value(451))), A::mul(s.ad_value(2539), s.ad_value(451))), A::mul(s.ad_value(2539), s.ad_value(451)));
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2644] != 0.0))) && (s.v[2645] != 0.0)) && (!(s.v[2646] != 0.0))) {
            s.store_powf_ad(2540, A::abs(A::mul(s.ad_value(2539), s.ad_value(451))), p.p882);
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2644] != 0.0))) && (s.v[2645] != 0.0)) {
            s.store_div_from_scalar_ad(2566, 1.0, A::sub_from_scalar(1.0, s.ad_value(2540)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2644] != 0.0))) && (!(s.v[2645] != 0.0))) {
            s.store_offset_ad(2566, A::mul(A::add(s.ad_value(2539), A::scale(s.ad_value(641), s.v[445])), s.ad_value(454)), s.v[448]);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) {
            s.store_mul_ad_lhs(1898, A::scale(A::add(A::add(A::add(s.ad_value(2541), s.ad_value(2542)), s.ad_value(2549)), s.ad_value(2564)), p.p29), 2566);
        }

        s.v[2647] = if (s.v[474] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            let assign58800_ad_e75304: A = {
                if (s.v[821] < p.p887) {
                    {
                        if (((s.v[821] - p.p887) / p.p888) < (-37.0)) {
                            A::constant(p.p887)
                        } else {
                            A::offset(A::scale(A::ln(A::offset(A::exp(A::scale(A::offset(s.ad_value(821), (-p.p887)), 1.0 / (p.p888))), 1.0)), p.p888), p.p887)
                        }
                    }
                } else {
                    {
                        if (((s.v[821] - p.p887) / p.p888) > 37.0) {
                            s.ad_value(821)
                        } else {
                            A::add(s.ad_value(821), A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(p.p887, s.ad_value(821)), 1.0 / (p.p888))), 1.0)), p.p888))
                        }
                    }
                }
            };
            s.store_ad(2567, &assign58800_ad_e75304);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(658), 4.0), 658);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_div(2526, 658, 659);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_add_ad_rhs(2527, 2567, A::mul(s.ad_value(658), s.ad_value(2526)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_add(2528, 659, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_sub(2529, 659, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_scale_ad(2568, A::div(A::mul(s.ad_value(2567), s.ad_value(659)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2648] = if (s.v[411] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) && (s.v[2648] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2568), s.v[408])));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) && (!(s.v[2648] != 0.0))) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2568), s.v[408])), s.v[411]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_scale_ad(1904, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2540)), s.v[420]), A::scale(A::sub(s.ad_value(2567), s.ad_value(2568)), s.v[423])), p.p30);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_sub_ad_lhs(2567, A::offset(s.ad_value(821), p.p887), 2567);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(658), 4.0), 658);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_div(2526, 658, 659);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_add_ad_rhs(2527, 2567, A::mul(s.ad_value(658), s.ad_value(2526)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_add(2528, 659, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_sub(2529, 659, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_scale_ad(2568, A::div(A::mul(s.ad_value(2567), s.ad_value(659)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2649] = if (s.v[468] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) && (s.v[2649] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(467))));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) && (!(s.v[2649] != 0.0))) {
            s.store_ad(2540, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(467))), s.ad_value(468)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_scale_ad(473, A::add(A::mul(s.ad_value(471), A::sub_from_scalar(1.0, s.ad_value(2540))), A::mul(s.ad_value(472), A::sub(s.ad_value(2567), s.ad_value(2568)))), p.p30);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (s.v[2647] != 0.0)) {
            s.store_add(1904, 1904, 473);
        }

        s.v[2650] = if (s.v[411] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2647] != 0.0))) && (s.v[2650] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[408])));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2647] != 0.0))) && (!(s.v[2650] != 0.0))) {
            s.store_powf_ad(2540, A::sub_from_scalar(1.0, A::scale(s.ad_value(2532), s.v[408])), s.v[411]);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2630] != 0.0))) && (!(s.v[2647] != 0.0))) {
            s.store_scale_ad(1904, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2540)), s.v[420]), A::scale(A::sub(s.ad_value(821), s.ad_value(2532)), s.v[423])), p.p30);
        }

        if ((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) {
            s.store_add_ad(837, A::add(A::mul(s.ad_value(647), s.ad_value(1896)), A::mul(s.ad_value(648), s.ad_value(1897))), A::mul(s.ad_value(649), s.ad_value(1898)));
        }

        s.v[2651] = if (s.v[637] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2651] != 0.0)) {
            s.store_mul_ad_rhs(644, 637, A::sub(A::pow(A::scale(A::add(A::add(s.ad_value(814), s.ad_value(816)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(814), s.ad_value(816)), A::add(s.ad_value(814), s.ad_value(816))), (0.001 * 0.001)))), 0.5), s.ad_value(638)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(638))));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2651] != 0.0)) {
            s.store_add(642, 543, 644);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2651] != 0.0)) {
            s.store_div_from_scalar(617, 1.0, 642);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2651] != 0.0)) {
            s.store_div_ad_rhs(620, 620, A::offset(A::div(s.ad_value(644), s.ad_value(543)), 1.0));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2651] != 0.0))) {
            s.copy_ad(642, 543);
        }

        s.v[2652] = if (s.v[639] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2652] != 0.0)) {
            s.store_mul_ad_rhs(646, 639, A::sub(A::pow(A::scale(A::add(A::add(s.ad_value(814), s.ad_value(816)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(814), s.ad_value(816)), A::add(s.ad_value(814), s.ad_value(816))), (0.001 * 0.001)))), 0.5), s.ad_value(640)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(640))));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2652] != 0.0)) {
            s.store_mul_ad_rhs(611, 611, A::offset(s.ad_value(646), 1.0));
        }

        if ((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) {
            s.store_scalar(2538, 0.0);
        }

        if ((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) {
            s.store_scalar(2535, 0.0);
        }

        s.v[2653] = if !(((s.v[674] == 0.0) && (s.v[675] == 0.0)) && (s.v[676] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(685), 4.0), 685);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_div(2526, 685, 686);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_add_ad_rhs(2527, 822, A::mul(s.ad_value(685), s.ad_value(2526)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_add(2528, 686, 2527);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_sub(2529, 686, 2527);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_scale_ad(2532, A::div(A::mul(s.ad_value(822), s.ad_value(686)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2654] = if (s.v[822] < s.v[682]) { 1.0 } else { 0.0 };

        s.v[2655] = if (((((-0.5) * (s.v[822] * s.v[372]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (s.v[2654] != 0.0)) && (s.v[2655] != 0.0)) {
            s.store_exp_ad(2533, A::scale(s.ad_value(822), (s.v[372] * (-0.5))));
        }

        s.v[2656] = if (((-0.5) * (s.v[822] * s.v[372])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (s.v[2654] != 0.0)) && (!(s.v[2655] != 0.0))) && (s.v[2656] != 0.0)) {
            let assign59330_ad_e76138: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(822), (s.v[372] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(822), (s.v[372] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(822), (s.v[372] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2533, &assign59330_ad_e76138);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (s.v[2654] != 0.0)) && (!(s.v[2655] != 0.0))) && (!(s.v[2656] != 0.0))) {
            s.store_scale_ad(2533, A::offset(A::mul(A::offset(A::scale(s.ad_value(822), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(822), (s.v[372] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(822), (s.v[372] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (s.v[2654] != 0.0)) {
            s.store_div_from_scalar(2534, 1.0, 2533);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (s.v[2654] != 0.0)) {
            s.store_square(2531, 2534);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (!(s.v[2654] != 0.0))) {
            s.store_mul_ad_lhs(2531, A::offset(A::scale(A::sub(s.ad_value(822), s.ad_value(682)), s.v[372]), 1.0), 683);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (!(s.v[2654] != 0.0))) {
            s.store_sqrt(2534, 2531);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (!(s.v[2654] != 0.0))) {
            s.store_div_from_scalar(2533, 1.0, 2534);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_offset(2531, 2531, (-1.0));
        }

        s.v[2657] = if (s.v[822] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (s.v[2657] != 0.0)) {
            s.store_scale_ad(2535, A::ln(A::add(A::offset(s.ad_value(2533), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2533), 1.0), A::offset(s.ad_value(2533), 3.0))))), (s.v[371] * 2.0));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) && (!(s.v[2657] != 0.0))) {
            s.store_sub_ad_lhs(2535, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2534), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2534), 1.0), A::offset(A::scale(s.ad_value(2534), 3.0), 1.0))))), (s.v[371] * 2.0)), 822);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_sub(2536, 684, 2535);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_scale_ad(2537, A::sub(A::add(s.ad_value(822), s.ad_value(2536)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(822), s.ad_value(2536)), A::sub(s.ad_value(822), s.ad_value(2536))), ((4.0 * s.v[371]) * s.v[371])))), 0.5);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_scale_ad(2538, A::sub(A::add(s.ad_value(822), s.ad_value(687)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(822), s.ad_value(687)), A::sub(s.ad_value(822), s.ad_value(687))), ((4.0 * s.v[369]) * s.v[369])))), 0.5);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2653] != 0.0)) {
            s.store_scale_ad(2539, A::sub(s.ad_value(822), A::sqrt(A::offset(A::mul(s.ad_value(822), s.ad_value(822)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[2658] = if (s.v[674] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2658] != 0.0)) {
            s.store_scalar(1899, 0.0);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2658] != 0.0)) {
            s.store_scalar(1905, 0.0);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) {
            s.store_mul(2541, 564, 2531);
        }

        s.v[2659] = if ((s.v[523] == 0.0) && (s.v[526] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (s.v[2659] != 0.0)) {
            s.store_scalar(2542, 0.0);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) {
            s.store_sub(2543, 570, 2537);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) {
            s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));
        }

        s.v[2660] = if (s.v[512] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) && (s.v[2660] != 0.0)) {
            s.store_scalar(2545, 0.0);
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
        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) && (!(s.v[2660] != 0.0))) {
            s.store_mul_ad(2545, A::add(A::div(A::mul(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544))), A::sub_from_scalar(1.0, s.ad_value(2544))), s.ad_value(2544)), A::sub_from_scalar(1.0, A::scale(s.ad_value(512), 2.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) {
            s.store_add(2546, 2544, 2545);
        }

        s.v[2661] = if (s.v[512] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) && (s.v[2661] != 0.0)) {
            s.store_sqrt_ad(2540, A::mul(s.ad_value(2543), s.ad_value(597)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) && (!(s.v[2661] != 0.0))) {
            s.store_ad(2540, &A::pow(A::mul(s.ad_value(2543), s.ad_value(597)), s.ad_value(512)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) {
            s.store_mul(2547, 591, 2540);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) {
            s.store_mul_ad_rhs(2548, 561, A::mul(A::offset(s.ad_value(2534), (-1.0)), s.ad_value(2547)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2659] != 0.0))) {
            s.store_mul_ad_rhs(2542, 523, A::mul(s.ad_value(2548), s.ad_value(2546)));
        }

        s.v[2662] = if (s.v[526] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (s.v[2662] != 0.0)) {
            s.store_scalar(2549, 0.0);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_mul_ad_rhs(2550, 606, A::div(A::mul(s.ad_value(2547), s.ad_value(576)), s.ad_value(2543)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_div_ad_lhs(2551, A::scale(s.ad_value(603), 0.666666666666667), 2550);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_square(2552, 2551);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_sqrt_ad(2553, A::div(A::square(s.ad_value(2552)), A::offset(A::square(s.ad_value(2552)), 1.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_sqrt(2554, 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_mul(2555, 2553, 2554);
        }

        s.v[2663] = if (((-s.v[512]) * s.v[579]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (s.v[2663] != 0.0)) {
            s.store_div_from_scalar_ad(2556, 1.0, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2663] != 0.0))) {
            s.store_pow_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), A::mul(A::neg(s.ad_value(512)), s.ad_value(579)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_div_ad(2557, A::mul(s.ad_value(2546), s.ad_value(2556)), A::add(s.ad_value(2546), s.ad_value(2556)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_sqrt_ad(2558, A::scale(A::div(s.ad_value(2550), s.ad_value(2554)), 0.375));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_sub_ad_lhs(2559, A::scale(A::mul(s.ad_value(2551), s.ad_value(2554)), 2.0), 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_add_ad(2560, A::sub(A::mul(A::mul(s.ad_value(603), s.ad_value(2551)), s.ad_value(2554)), A::mul(s.ad_value(603), s.ad_value(2553))), A::scale(A::mul(s.ad_value(2550), s.ad_value(2555)), 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_mul_ad_lhs(2561, A::offset(s.ad_value(2559), (-1.0)), 2558);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_square(2522, 2561);
        }

        s.v[2664] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (s.v[2664] != 0.0)) {
            s.store_div_from_scalar_ad(2523, 1.0, A::offset(A::scale(s.ad_value(2561), s.v[373]), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2664] != 0.0))) {
            s.store_div_from_scalar_ad(2523, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2561), s.v[373])));
        }

        s.v[2665] = if (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (s.v[2665] != 0.0)) {
            s.store_exp_ad(2540, A::sub(s.ad_value(2560), s.ad_value(2522)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2665] != 0.0))) {
            let assign59880_ad_e77088: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2540, &assign59880_ad_e77088);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_mul_ad_lhs(2524, A::add(A::add(A::scale(s.ad_value(2523), 0.29214664), A::scale(A::square(s.ad_value(2523)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(2523)), s.ad_value(2523)), s.v[375])), 2540);
        }

        s.v[2666] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (s.v[2666] != 0.0)) {
            s.copy_ad(2562, 2524);
        }

        s.v[2667] = if (s.v[2560] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (s.v[2667] != 0.0)) {
            s.store_exp(2540, 2560);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) && (!(s.v[2667] != 0.0))) {
            s.store_div_from_scalar_ad(2540, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) && (!(s.v[2666] != 0.0))) {
            s.store_sub_ad_lhs(2562, A::scale(s.ad_value(2540), 2.0), 2524);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_scale_ad(2563, A::div(A::mul(s.ad_value(603), s.ad_value(2562)), s.ad_value(2558)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2662] != 0.0))) {
            s.store_mul_ad_rhs(2549, 526, A::mul(A::mul(s.ad_value(2548), s.ad_value(2563)), s.ad_value(2557)));
        }

        s.v[2668] = if (s.v[532] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (s.v[2668] != 0.0)) {
            s.store_scalar(2564, 0.0);
        }

        s.v[2669] = if (s.v[512] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) && (s.v[2669] != 0.0)) {
            s.store_sqrt_ad(2540, A::mul(A::sub(s.ad_value(509), s.ad_value(2538)), s.ad_value(597)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) && (!(s.v[2669] != 0.0))) {
            s.store_ad(2540, &A::pow(A::mul(A::sub(s.ad_value(509), s.ad_value(2538)), s.ad_value(597)), s.ad_value(512)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) {
            s.store_mul_ad_rhs(2565, 579, A::div(A::mul(A::sub(s.ad_value(509), s.ad_value(2538)), s.ad_value(594)), s.ad_value(2540)));
        }

        s.v[2670] = if (((((-s.v[609]) / s.v[2565])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) && (s.v[2670] != 0.0)) {
            s.store_exp_ad(2540, A::div(A::neg(s.ad_value(609)), s.ad_value(2565)));
        }

        s.v[2671] = if (((-s.v[609]) / s.v[2565]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) && (!(s.v[2670] != 0.0))) && (s.v[2671] != 0.0)) {
            let assign60070_ad_e77428: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2565))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2565))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2565))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2540, 1e-100, assign60070_ad_e77428);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) && (!(s.v[2670] != 0.0))) && (!(s.v[2671] != 0.0))) {
            let assign60080_ad_e77479: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2565)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2540, &assign60080_ad_e77479);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2668] != 0.0))) {
            s.store_mul_ad_rhs(2564, 532, A::mul(A::mul(A::mul(s.ad_value(822), s.ad_value(2565)), s.ad_value(2565)), s.ad_value(2540)));
        }

        s.v[2672] = if (s.v[541] > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (s.v[2672] != 0.0)) {
            s.store_scalar(2566, 1.0);
        }

        s.v[2673] = if (s.v[2539] > ((-s.v[445]) * s.v[541])) { 1.0 } else { 0.0 };

        s.v[2674] = if (s.v[544] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2672] != 0.0))) && (s.v[2673] != 0.0)) && (s.v[2674] != 0.0)) {
            s.store_mul_ad(2540, A::mul(A::mul(A::mul(s.ad_value(2539), s.ad_value(615)), A::mul(s.ad_value(2539), s.ad_value(615))), A::mul(s.ad_value(2539), s.ad_value(615))), A::mul(s.ad_value(2539), s.ad_value(615)));
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2672] != 0.0))) && (s.v[2673] != 0.0)) && (!(s.v[2674] != 0.0))) {
            s.store_ad(2540, &A::pow(A::abs(A::mul(s.ad_value(2539), s.ad_value(615))), s.ad_value(544)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2672] != 0.0))) && (s.v[2673] != 0.0)) {
            s.store_div_from_scalar_ad(2566, 1.0, A::sub_from_scalar(1.0, s.ad_value(2540)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2672] != 0.0))) && (!(s.v[2673] != 0.0))) {
            s.store_add_ad_rhs(2566, 612, A::mul(A::add(s.ad_value(2539), A::scale(s.ad_value(541), s.v[445])), s.ad_value(618)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) {
            s.store_mul_ad_lhs(1899, A::scale(A::add(A::add(A::add(s.ad_value(2541), s.ad_value(2542)), s.ad_value(2549)), s.ad_value(2564)), p.p29), 2566);
        }

        s.v[2675] = if (s.v[576] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (s.v[2675] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(573))));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) && (!(s.v[2675] != 0.0))) {
            s.store_ad(2540, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(573))), s.ad_value(576)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2658] != 0.0))) {
            s.store_scale_ad(1905, A::add(A::mul(s.ad_value(585), A::sub_from_scalar(1.0, s.ad_value(2540))), A::mul(s.ad_value(588), A::sub(s.ad_value(822), s.ad_value(2532)))), p.p30);
        }

        s.v[2676] = if (s.v[675] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2676] != 0.0)) {
            s.store_scalar(1900, 0.0);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2676] != 0.0)) {
            s.store_scalar(1906, 0.0);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) {
            s.store_mul(2541, 565, 2531);
        }

        s.v[2677] = if ((s.v[524] == 0.0) && (s.v[527] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (s.v[2677] != 0.0)) {
            s.store_scalar(2542, 0.0);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) {
            s.store_sub(2543, 571, 2537);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) {
            s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));
        }

        s.v[2678] = if (s.v[513] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) && (s.v[2678] != 0.0)) {
            s.store_scalar(2545, 0.0);
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) && (!(s.v[2678] != 0.0))) {
            s.store_mul_ad(2545, A::add(A::div(A::mul(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544))), A::sub_from_scalar(1.0, s.ad_value(2544))), s.ad_value(2544)), A::sub_from_scalar(1.0, A::scale(s.ad_value(513), 2.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) {
            s.store_add(2546, 2544, 2545);
        }

        s.v[2679] = if (s.v[513] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) && (s.v[2679] != 0.0)) {
            s.store_sqrt_ad(2540, A::mul(s.ad_value(2543), s.ad_value(598)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) && (!(s.v[2679] != 0.0))) {
            s.store_ad(2540, &A::pow(A::mul(s.ad_value(2543), s.ad_value(598)), s.ad_value(513)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) {
            s.store_mul(2547, 592, 2540);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) {
            s.store_mul_ad_rhs(2548, 562, A::mul(A::offset(s.ad_value(2534), (-1.0)), s.ad_value(2547)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2677] != 0.0))) {
            s.store_mul_ad_rhs(2542, 524, A::mul(s.ad_value(2548), s.ad_value(2546)));
        }

        s.v[2680] = if (s.v[527] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (s.v[2680] != 0.0)) {
            s.store_scalar(2549, 0.0);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_mul_ad_rhs(2550, 607, A::div(A::mul(s.ad_value(2547), s.ad_value(577)), s.ad_value(2543)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_div_ad_lhs(2551, A::scale(s.ad_value(604), 0.666666666666667), 2550);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_square(2552, 2551);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_sqrt_ad(2553, A::div(A::square(s.ad_value(2552)), A::offset(A::square(s.ad_value(2552)), 1.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_sqrt(2554, 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_mul(2555, 2553, 2554);
        }

        s.v[2681] = if (((-s.v[513]) * s.v[580]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2681] != 0.0)) {
            s.store_div_from_scalar_ad(2556, 1.0, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2681] != 0.0))) {
            s.store_pow_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), A::mul(A::neg(s.ad_value(513)), s.ad_value(580)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_div_ad(2557, A::mul(s.ad_value(2546), s.ad_value(2556)), A::add(s.ad_value(2546), s.ad_value(2556)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_sqrt_ad(2558, A::scale(A::div(s.ad_value(2550), s.ad_value(2554)), 0.375));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_sub_ad_lhs(2559, A::scale(A::mul(s.ad_value(2551), s.ad_value(2554)), 2.0), 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_add_ad(2560, A::sub(A::mul(A::mul(s.ad_value(604), s.ad_value(2551)), s.ad_value(2554)), A::mul(s.ad_value(604), s.ad_value(2553))), A::scale(A::mul(s.ad_value(2550), s.ad_value(2555)), 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_mul_ad_lhs(2561, A::offset(s.ad_value(2559), (-1.0)), 2558);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_square(2522, 2561);
        }

        s.v[2682] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2682] != 0.0)) {
            s.store_div_from_scalar_ad(2523, 1.0, A::offset(A::scale(s.ad_value(2561), s.v[373]), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2682] != 0.0))) {
            s.store_div_from_scalar_ad(2523, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2561), s.v[373])));
        }

        s.v[2683] = if (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2683] != 0.0)) {
            s.store_exp_ad(2540, A::sub(s.ad_value(2560), s.ad_value(2522)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2683] != 0.0))) {
            let assign60630_ad_e78354: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2540, &assign60630_ad_e78354);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_mul_ad_lhs(2524, A::add(A::add(A::scale(s.ad_value(2523), 0.29214664), A::scale(A::square(s.ad_value(2523)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(2523)), s.ad_value(2523)), s.v[375])), 2540);
        }

        s.v[2684] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (s.v[2684] != 0.0)) {
            s.copy_ad(2562, 2524);
        }

        s.v[2685] = if (s.v[2560] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_exp(2540, 2560);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) && (!(s.v[2685] != 0.0))) {
            s.store_div_from_scalar_ad(2540, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) && (!(s.v[2684] != 0.0))) {
            s.store_sub_ad_lhs(2562, A::scale(s.ad_value(2540), 2.0), 2524);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_scale_ad(2563, A::div(A::mul(s.ad_value(604), s.ad_value(2562)), s.ad_value(2558)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2680] != 0.0))) {
            s.store_mul_ad_rhs(2549, 527, A::mul(A::mul(s.ad_value(2548), s.ad_value(2563)), s.ad_value(2557)));
        }

        s.v[2686] = if (s.v[533] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (s.v[2686] != 0.0)) {
            s.store_scalar(2564, 0.0);
        }

        s.v[2687] = if (s.v[513] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) && (s.v[2687] != 0.0)) {
            s.store_sqrt_ad(2540, A::mul(A::sub(s.ad_value(510), s.ad_value(2538)), s.ad_value(598)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) && (!(s.v[2687] != 0.0))) {
            s.store_ad(2540, &A::pow(A::mul(A::sub(s.ad_value(510), s.ad_value(2538)), s.ad_value(598)), s.ad_value(513)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) {
            s.store_mul_ad_rhs(2565, 580, A::div(A::mul(A::sub(s.ad_value(510), s.ad_value(2538)), s.ad_value(595)), s.ad_value(2540)));
        }

        s.v[2688] = if (((((-s.v[610]) / s.v[2565])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) && (s.v[2688] != 0.0)) {
            s.store_exp_ad(2540, A::div(A::neg(s.ad_value(610)), s.ad_value(2565)));
        }

        s.v[2689] = if (((-s.v[610]) / s.v[2565]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) && (!(s.v[2688] != 0.0))) && (s.v[2689] != 0.0)) {
            let assign60820_ad_e78694: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2565))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2565))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2565))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2540, 1e-100, assign60820_ad_e78694);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) && (!(s.v[2688] != 0.0))) && (!(s.v[2689] != 0.0))) {
            let assign60830_ad_e78745: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2565)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2540, &assign60830_ad_e78745);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2686] != 0.0))) {
            s.store_mul_ad_rhs(2564, 533, A::mul(A::mul(A::mul(s.ad_value(822), s.ad_value(2565)), s.ad_value(2565)), s.ad_value(2540)));
        }

        s.v[2690] = if (s.v[542] > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (s.v[2690] != 0.0)) {
            s.store_scalar(2566, 1.0);
        }

        s.v[2691] = if (s.v[2539] > ((-s.v[445]) * s.v[542])) { 1.0 } else { 0.0 };

        s.v[2692] = if (s.v[545] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2691] != 0.0)) && (s.v[2692] != 0.0)) {
            s.store_mul_ad(2540, A::mul(A::mul(A::mul(s.ad_value(2539), s.ad_value(616)), A::mul(s.ad_value(2539), s.ad_value(616))), A::mul(s.ad_value(2539), s.ad_value(616))), A::mul(s.ad_value(2539), s.ad_value(616)));
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2691] != 0.0)) && (!(s.v[2692] != 0.0))) {
            s.store_ad(2540, &A::pow(A::abs(A::mul(s.ad_value(2539), s.ad_value(616))), s.ad_value(545)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2691] != 0.0)) {
            s.store_div_from_scalar_ad(2566, 1.0, A::sub_from_scalar(1.0, s.ad_value(2540)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) {
            s.store_add_ad_rhs(2566, 613, A::mul(A::add(s.ad_value(2539), A::scale(s.ad_value(542), s.v[445])), s.ad_value(619)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) {
            s.store_mul_ad_lhs(1900, A::scale(A::add(A::add(A::add(s.ad_value(2541), s.ad_value(2542)), s.ad_value(2549)), s.ad_value(2564)), p.p29), 2566);
        }

        s.v[2693] = if (s.v[577] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (s.v[2693] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(574))));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) && (!(s.v[2693] != 0.0))) {
            s.store_ad(2540, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(574))), s.ad_value(577)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2676] != 0.0))) {
            s.store_scale_ad(1906, A::add(A::mul(s.ad_value(586), A::sub_from_scalar(1.0, s.ad_value(2540))), A::mul(s.ad_value(589), A::sub(s.ad_value(822), s.ad_value(2532)))), p.p30);
        }

        s.v[2694] = if (s.v[676] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2694] != 0.0)) {
            s.store_scalar(1901, 0.0);
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (s.v[2694] != 0.0)) {
            s.store_scalar(1907, 0.0);
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
        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_mul(2541, 566, 2531);
        }

        s.v[2695] = if ((s.v[525] == 0.0) && (s.v[528] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2695] != 0.0)) {
            s.store_scalar(2542, 0.0);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) {
            s.store_sub(2543, 572, 2537);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) {
            s.store_sub_from_scalar_ad(2544, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2535), s.ad_value(2543)))));
        }

        s.v[2696] = if (s.v[514] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) && (s.v[2696] != 0.0)) {
            s.store_scalar(2545, 0.0);
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) && (!(s.v[2696] != 0.0))) {
            s.store_mul_ad(2545, A::add(A::div(A::mul(A::square(s.ad_value(2544)), A::ln(s.ad_value(2544))), A::sub_from_scalar(1.0, s.ad_value(2544))), s.ad_value(2544)), A::sub_from_scalar(1.0, A::scale(s.ad_value(514), 2.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) {
            s.store_add(2546, 2544, 2545);
        }

        s.v[2697] = if (s.v[514] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_sqrt_ad(2540, A::mul(s.ad_value(2543), s.ad_value(599)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) && (!(s.v[2697] != 0.0))) {
            s.store_ad(2540, &A::pow(A::mul(s.ad_value(2543), s.ad_value(599)), s.ad_value(514)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) {
            s.store_mul(2547, 593, 2540);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) {
            s.store_mul_ad_rhs(2548, 563, A::mul(A::offset(s.ad_value(2534), (-1.0)), s.ad_value(2547)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) {
            s.store_mul_ad_rhs(2542, 525, A::mul(s.ad_value(2548), s.ad_value(2546)));
        }

        s.v[2698] = if (s.v[528] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2698] != 0.0)) {
            s.store_scalar(2549, 0.0);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_mul_ad_rhs(2550, 608, A::div(A::mul(s.ad_value(2547), s.ad_value(578)), s.ad_value(2543)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_div_ad_lhs(2551, A::scale(s.ad_value(605), 0.666666666666667), 2550);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_square(2552, 2551);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_sqrt_ad(2553, A::div(A::square(s.ad_value(2552)), A::offset(A::square(s.ad_value(2552)), 1.0)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_sqrt(2554, 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_mul(2555, 2553, 2554);
        }

        s.v[2699] = if (((-s.v[514]) * s.v[581]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (s.v[2699] != 0.0)) {
            s.store_div_from_scalar_ad(2556, 1.0, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (!(s.v[2699] != 0.0))) {
            s.store_pow_ad(2556, A::offset(A::mul(s.ad_value(2550), s.ad_value(2555)), 1.0), A::mul(A::neg(s.ad_value(514)), s.ad_value(581)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_div_ad(2557, A::mul(s.ad_value(2546), s.ad_value(2556)), A::add(s.ad_value(2546), s.ad_value(2556)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_sqrt_ad(2558, A::scale(A::div(s.ad_value(2550), s.ad_value(2554)), 0.375));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_sub_ad_lhs(2559, A::scale(A::mul(s.ad_value(2551), s.ad_value(2554)), 2.0), 2553);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_add_ad(2560, A::sub(A::mul(A::mul(s.ad_value(605), s.ad_value(2551)), s.ad_value(2554)), A::mul(s.ad_value(605), s.ad_value(2553))), A::scale(A::mul(s.ad_value(2550), s.ad_value(2555)), 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_mul_ad_lhs(2561, A::offset(s.ad_value(2559), (-1.0)), 2558);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_square(2522, 2561);
        }

        s.v[2700] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (s.v[2700] != 0.0)) {
            s.store_div_from_scalar_ad(2523, 1.0, A::offset(A::scale(s.ad_value(2561), s.v[373]), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (!(s.v[2700] != 0.0))) {
            s.store_div_from_scalar_ad(2523, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2561), s.v[373])));
        }

        s.v[2701] = if (((-s.v[2522]) + s.v[2560]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (s.v[2701] != 0.0)) {
            s.store_exp_ad(2540, A::sub(s.ad_value(2560), s.ad_value(2522)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (!(s.v[2701] != 0.0))) {
            let assign61380_ad_e79620: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2560), s.ad_value(2522))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2540, &assign61380_ad_e79620);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_mul_ad_lhs(2524, A::add(A::add(A::scale(s.ad_value(2523), 0.29214664), A::scale(A::square(s.ad_value(2523)), s.v[374])), A::scale(A::mul(A::square(s.ad_value(2523)), s.ad_value(2523)), s.v[375])), 2540);
        }

        s.v[2702] = if (s.v[2561] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (s.v[2702] != 0.0)) {
            s.copy_ad(2562, 2524);
        }

        s.v[2703] = if (s.v[2560] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (!(s.v[2702] != 0.0))) && (s.v[2703] != 0.0)) {
            s.store_exp(2540, 2560);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (!(s.v[2702] != 0.0))) && (!(s.v[2703] != 0.0))) {
            s.store_div_from_scalar_ad(2540, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2560)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (!(s.v[2702] != 0.0))) {
            s.store_sub_ad_lhs(2562, A::scale(s.ad_value(2540), 2.0), 2524);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_scale_ad(2563, A::div(A::mul(s.ad_value(605), s.ad_value(2562)), s.ad_value(2558)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_mul_ad_rhs(2549, 528, A::mul(A::mul(s.ad_value(2548), s.ad_value(2563)), s.ad_value(2557)));
        }

        s.v[2704] = if (s.v[534] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2704] != 0.0)) {
            s.store_scalar(2564, 0.0);
        }

        s.v[2705] = if (s.v[514] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) && (s.v[2705] != 0.0)) {
            s.store_sqrt_ad(2540, A::mul(A::sub(s.ad_value(511), s.ad_value(2538)), s.ad_value(599)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) && (!(s.v[2705] != 0.0))) {
            s.store_ad(2540, &A::pow(A::mul(A::sub(s.ad_value(511), s.ad_value(2538)), s.ad_value(599)), s.ad_value(514)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) {
            s.store_mul_ad_rhs(2565, 581, A::div(A::mul(A::sub(s.ad_value(511), s.ad_value(2538)), s.ad_value(596)), s.ad_value(2540)));
        }

        s.v[2706] = if (((((-s.v[611]) / s.v[2565])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) && (s.v[2706] != 0.0)) {
            s.store_exp_ad(2540, A::div(A::neg(s.ad_value(611)), s.ad_value(2565)));
        }

        s.v[2707] = if (((-s.v[611]) / s.v[2565]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) && (!(s.v[2706] != 0.0))) && (s.v[2707] != 0.0)) {
            let assign61570_ad_e79960: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(611)), s.ad_value(2565))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(611)), s.ad_value(2565))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(611)), s.ad_value(2565))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2540, 1e-100, assign61570_ad_e79960);
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) && (!(s.v[2706] != 0.0))) && (!(s.v[2707] != 0.0))) {
            let assign61580_ad_e80011: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(611)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(611)), s.ad_value(2565)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(611)), s.ad_value(2565)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2540, &assign61580_ad_e80011);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2704] != 0.0))) {
            s.store_mul_ad_rhs(2564, 534, A::mul(A::mul(A::mul(s.ad_value(822), s.ad_value(2565)), s.ad_value(2565)), s.ad_value(2540)));
        }

        s.v[2708] = if (s.v[642] > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2708] != 0.0)) {
            s.store_scalar(2566, 1.0);
        }

        s.v[2709] = if (s.v[2539] > ((-s.v[445]) * s.v[642])) { 1.0 } else { 0.0 };

        s.v[2710] = if (s.v[546] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2708] != 0.0))) && (s.v[2709] != 0.0)) && (s.v[2710] != 0.0)) {
            s.store_mul_ad(2540, A::mul(A::mul(A::mul(s.ad_value(2539), s.ad_value(617)), A::mul(s.ad_value(2539), s.ad_value(617))), A::mul(s.ad_value(2539), s.ad_value(617))), A::mul(s.ad_value(2539), s.ad_value(617)));
        }

        if ((((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2708] != 0.0))) && (s.v[2709] != 0.0)) && (!(s.v[2710] != 0.0))) {
            s.store_ad(2540, &A::pow(A::abs(A::mul(s.ad_value(2539), s.ad_value(617))), s.ad_value(546)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2708] != 0.0))) && (s.v[2709] != 0.0)) {
            s.store_div_from_scalar_ad(2566, 1.0, A::sub_from_scalar(1.0, s.ad_value(2540)));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) {
            s.store_add_ad_rhs(2566, 614, A::mul(A::add(s.ad_value(2539), A::scale(s.ad_value(642), s.v[445])), s.ad_value(620)));
        }

        if (((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_mul_ad_lhs(1901, A::scale(A::add(A::add(A::add(s.ad_value(2541), s.ad_value(2542)), s.ad_value(2549)), s.ad_value(2564)), p.p29), 2566);
        }

        s.v[2711] = if (s.v[636] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            let assign61700_ad_e80236: A = {
                if (s.v[822] < s.v[551]) {
                    {
                        if (((s.v[822] - s.v[551]) / s.v[552]) < (-37.0)) {
                            s.ad_value(551)
                        } else {
                            A::add(s.ad_value(551), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(822), s.ad_value(551)), s.ad_value(552))), 1.0)), s.ad_value(552)))
                        }
                    }
                } else {
                    {
                        if (((s.v[822] - s.v[551]) / s.v[552]) > 37.0) {
                            s.ad_value(822)
                        } else {
                            A::add(s.ad_value(822), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(551), s.ad_value(822)), s.ad_value(552))), 1.0)), s.ad_value(552)))
                        }
                    }
                }
            };
            s.store_ad(2567, &assign61700_ad_e80236);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(685), 4.0), 685);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_div(2526, 685, 686);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_add_ad_rhs(2527, 2567, A::mul(s.ad_value(685), s.ad_value(2526)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_add(2528, 686, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_sub(2529, 686, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_scale_ad(2568, A::div(A::mul(s.ad_value(2567), s.ad_value(686)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2712] = if (s.v[578] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) && (s.v[2712] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(575))));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) && (!(s.v[2712] != 0.0))) {
            s.store_ad(2540, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(575))), s.ad_value(578)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_scale_ad(1907, A::add(A::mul(s.ad_value(587), A::sub_from_scalar(1.0, s.ad_value(2540))), A::mul(s.ad_value(590), A::sub(s.ad_value(2567), s.ad_value(2568)))), p.p30);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_sub_ad_lhs(2567, A::add(s.ad_value(822), s.ad_value(551)), 2567);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_mul_ad_lhs(2525, A::scale(s.ad_value(685), 4.0), 685);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_div(2526, 685, 686);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_add_ad_rhs(2527, 2567, A::mul(s.ad_value(685), s.ad_value(2526)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_add(2528, 686, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_sub(2529, 686, 2527);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_sqrt_ad(2530, A::add(A::square(s.ad_value(2529)), s.ad_value(2525)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_scale_ad(2568, A::div(A::mul(s.ad_value(2567), s.ad_value(686)), A::add(s.ad_value(2528), s.ad_value(2530))), 2.0);
        }

        s.v[2713] = if (s.v[631] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) && (s.v[2713] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(630))));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) && (!(s.v[2713] != 0.0))) {
            s.store_ad(2540, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2568), s.ad_value(630))), s.ad_value(631)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_scale_ad(473, A::add(A::mul(s.ad_value(634), A::sub_from_scalar(1.0, s.ad_value(2540))), A::mul(s.ad_value(635), A::sub(s.ad_value(2567), s.ad_value(2568)))), p.p30);
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_add(1907, 1907, 473);
        }

        s.v[2714] = if (s.v[578] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2711] != 0.0))) && (s.v[2714] != 0.0)) {
            s.store_sqrt_ad(2540, A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(575))));
        }

        if (((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2711] != 0.0))) && (!(s.v[2714] != 0.0))) {
            s.store_ad(2540, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2532), s.ad_value(575))), s.ad_value(578)));
        }

        if ((((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2711] != 0.0))) {
            s.store_scale_ad(1907, A::add(A::mul(s.ad_value(587), A::sub_from_scalar(1.0, s.ad_value(2540))), A::mul(s.ad_value(590), A::sub(s.ad_value(822), s.ad_value(2532)))), p.p30);
        }

        if ((s.v[2569] != 0.0) && (!(s.v[2570] != 0.0))) {
            s.store_add_ad(838, A::add(A::mul(s.ad_value(674), s.ad_value(1899)), A::mul(s.ad_value(675), s.ad_value(1900))), A::mul(s.ad_value(676), s.ad_value(1901)));
        }

        s.store_mul(860, 1919, 800);

        s.store_mul(861, 1919, 801);

        s.store_mul(862, 1919, 802);

        s.store_mul(863, 1919, 803);

        s.store_mul(864, 1919, 804);

        s.store_mul(865, 1919, 805);

        s.store_mul(866, 1919, 806);

        s.v[2715] = if (s.v[820] > 0.0) { 1.0 } else { 0.0 };

        s.v[2716] = if (s.v[298] > 0.0) { 1.0 } else { 0.0 };

        s.v[2717] = if (s.v[299] > 0.0) { 1.0 } else { 0.0 };

        s.v[2718] = if (s.v[300] > 0.0) { 1.0 } else { 0.0 };

        s.v[2719] = if (s.v[301] > 0.0) { 1.0 } else { 0.0 };

        s.v[2720] = if (s.v[302] > 0.0) { 1.0 } else { 0.0 };

        s.v[2721] = if (s.v[303] > 0.0) { 1.0 } else { 0.0 };

        s.v[2722] = if (s.v[304] > 0.0) { 1.0 } else { 0.0 };

        s.v[1915] = 0.0;

        s.v[2723] = 0.0;

        s.v[2724] = 0.0;

        s.v[2725] = if (s.v[299] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2725] != 0.0) {
            s.store_ad(2723, &A::mul(A::mul(s.ad_value(801), A::voltage(ctx, &nodes, Some(2), Some(7))), A::voltage(ctx, &nodes, Some(2), Some(7))));
        }

        s.v[2726] = if (s.v[300] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2726] != 0.0) {
            s.store_ad(2724, &A::mul(A::mul(s.ad_value(802), A::voltage(ctx, &nodes, Some(0), Some(8))), A::voltage(ctx, &nodes, Some(0), Some(8))));
        }

        s.v[2727] = if (s.v[172] > 0.001) { 1.0 } else { 0.0 };

        if (s.v[2727] != 0.0) {
            s.store_add_ad_lhs(1915, A::add(A::add(A::mul(A::add(s.ad_value(827), s.ad_value(835)), s.ad_value(815)), A::mul(s.ad_value(836), A::add(s.ad_value(815), s.ad_value(816)))), s.ad_value(2723)), 2724);
        }

        s.store_neg_ad(839, A::add(A::add(s.ad_value(840), s.ad_value(841)), s.ad_value(842)));

        s.store_add(843, 843, 1894);

        s.store_add(844, 844, 1895);

        s.store_add_ad(846, A::add(A::mul(s.ad_value(647), s.ad_value(1902)), A::mul(s.ad_value(648), s.ad_value(1903))), A::mul(s.ad_value(649), s.ad_value(1904)));

        s.store_add_ad(847, A::add(A::mul(s.ad_value(674), s.ad_value(1905)), A::mul(s.ad_value(675), s.ad_value(1906))), A::mul(s.ad_value(676), s.ad_value(1907)));

        s.v[2729] = if (s.v[820] < 0.0) { 1.0 } else { 0.0 };

        if (s.v[2729] != 0.0) {
            s.copy_ad(2728, 842);
        }

        if (s.v[2729] != 0.0) {
            s.copy_ad(842, 839);
        }

        if (s.v[2729] != 0.0) {
            s.copy_ad(839, 2728);
        }

        s.v[853] = 0.0;

        s.v[2746] = 0.0;

        s.v[2741] = 0.0;

        s.v[848] = 1e-40;

        s.v[850] = 0.0;

        s.v[852] = 0.0;

        s.store_mul(849, 1888, 1879);

        s.v[851] = 0.0;

        s.v[2748] = 0.0;

        s.v[858] = 0.0;

        s.v[2761] = 0.0;

        s.v[859] = 0.0;

        s.v[2762] = if ((s.v[1813] > 0.0) && (s.v[1917] > 0.0)) { 1.0 } else { 0.0 };

        s.v[2763] = if (p.p34 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2762] != 0.0) && (s.v[2763] != 0.0)) {
            s.store_mul_ad_lhs(2730, A::mul(s.ad_value(760), s.ad_value(1848)), 1916);
        }

        if ((s.v[2762] != 0.0) && (s.v[2763] != 0.0)) {
            s.store_mul(2731, 760, 1850);
        }

        if ((s.v[2762] != 0.0) && (s.v[2763] != 0.0)) {
            s.store_mul_ad_lhs(2732, A::mul(s.ad_value(760), s.ad_value(1848)), 1844);
        }

        if ((s.v[2762] != 0.0) && (s.v[2763] != 0.0)) {
            s.store_mul_ad(853, A::add(A::sub(s.ad_value(276), A::mul(s.ad_value(277), s.ad_value(2730))), A::mul(s.ad_value(278), A::square(s.ad_value(2730)))), A::ln(A::div(A::add(s.ad_value(2731), A::scale(s.ad_value(2732), 0.5)), A::sub(s.ad_value(2731), A::scale(s.ad_value(2732), 0.5)))));
        }

        if ((s.v[2762] != 0.0) && (s.v[2763] != 0.0)) {
            s.store_add_ad_rhs(853, 853, A::mul(A::add(s.ad_value(277), A::mul(s.ad_value(278), A::sub(s.ad_value(2731), A::scale(s.ad_value(2730), 2.0)))), s.ad_value(2732)));
        }

        if ((s.v[2762] != 0.0) && (s.v[2763] != 0.0)) {
            s.store_div_ad_lhs(853, A::mul(A::mul(A::mul(s.ad_value(1923), s.ad_value(827)), s.ad_value(1860)), s.ad_value(853)), 2730);
        }

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
        if ((s.v[2762] != 0.0) && (s.v[2763] != 0.0)) {
            s.store_ad(853, &{
                if (s.v[853] > 0.0) {
                    s.ad_value(853)
                } else {
                    A::constant(0.0)
                }
            });
        }

        s.v[2764] = if (p.p32 > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) {
            s.store_div(2733, 1850, 1848);
        }

        if ((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) {
            s.store_div(2734, 1849, 1850);
        }

        if ((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) {
            s.store_scaled_div(2735, 1844, 2733, (0.5 * 0.16666666666666666));
        }

        if ((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) {
            s.store_square(2736, 2735);
        }

        if ((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) {
            s.store_offset_ad(2737, A::div(s.ad_value(2733), s.ad_value(1861)), (-1.0));
        }

        if ((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) {
            s.store_ad(2738, &{
                if ((1.0 - (12.0 * (s.v[2737] * s.v[2736]))) > 1e-20) {
                    A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2737), s.ad_value(2736)), 12.0))
                } else {
                    A::constant(1e-20)
                }
            });
        }

        if ((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) {
            s.store_div_from_scalar_ad(2739, 1.0, A::square(s.ad_value(2738)));
        }

        if ((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) {
            s.store_mul_ad_lhs(2740, A::mul(s.ad_value(1917), s.ad_value(1850)), 1860);
        }

        if ((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) {
            s.store_sub_ad(2741, A::add(s.ad_value(2734), A::scale(s.ad_value(2736), 12.0)), A::scale(A::mul(A::mul(A::offset(s.ad_value(2734), 1.0), s.ad_value(2736)), s.ad_value(2737)), 24.0));
        }

        if ((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) {
            s.store_ad(2741, &{
                if (s.v[2741] > 1e-40) {
                    s.ad_value(2741)
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if ((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) {
            s.store_mul_ad_lhs(2741, A::mul(s.ad_value(2740), s.ad_value(2739)), 2741);
        }

        s.v[2765] = if (s.v[275] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) && (s.v[2765] != 0.0)) {
            s.store_div(2742, 1854, 1853);
        }

        if (((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) && (s.v[2765] != 0.0)) {
            s.store_mul_ad_lhs(2743, A::mul(A::square(s.ad_value(2742)), s.ad_value(1844)), 1844);
        }

        s.v[2766] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) && (s.v[2765] != 0.0)) && (s.v[2766] != 0.0)) {
            s.store_div_ad_rhs(2743, 2743, A::offset(A::mul(s.ad_value(2742), s.ad_value(1844)), 1.0));
        }

        if (((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) && (s.v[2765] != 0.0)) {
            s.store_scale_ad(2744, A::mul(s.ad_value(1853), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2743), 2.0), 1.0)), 1.0)), 0.5);
        }

        if (((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) && (s.v[2765] != 0.0)) {
            s.store_div_ad_rhs(2745, 1853, A::mul(s.ad_value(2744), s.ad_value(2738)));
        }

        if (((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) && (s.v[2765] != 0.0)) {
            s.store_mul_ad_lhs(2746, A::mul(A::mul(A::mul(s.ad_value(799), s.ad_value(827)), s.ad_value(1841)), s.ad_value(2745)), 2745);
        }

        if (((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) && (s.v[2765] != 0.0)) {
            s.store_add_ad_rhs(2741, 2741, A::div(s.ad_value(2746), s.ad_value(1919)));
        }

        if ((s.v[2762] != 0.0) && (s.v[2764] != 0.0)) {
            s.store_sqrt_ad(851, A::mul(s.ad_value(1920), s.ad_value(2741)));
        }

        s.v[2767] = if ((((p.p50 == 1.0) && (s.v[1920] > 0.0)) && (p.p32 > 0.0)) && (p.p33 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) {
            s.store_sub_ad(848, A::sub(A::scale(s.ad_value(2734), 0.08333333333333333), A::mul(s.ad_value(2736), A::sub(A::offset(s.ad_value(2734), 0.2), A::scale(s.ad_value(2736), 12.0)))), A::scale(A::mul(A::mul(s.ad_value(2736), A::sub(A::offset(s.ad_value(2734), 1.0), A::scale(s.ad_value(2736), 12.0))), s.ad_value(2737)), 1.6));
        }

        if ((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) {
            s.store_ad(848, &{
                if (s.v[848] > 1e-40) {
                    s.ad_value(848)
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if ((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) {
            s.store_mul_ad_lhs(848, A::div(s.ad_value(2739), s.ad_value(2740)), 848);
        }

        if ((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) {
            s.store_mul_ad(2747, A::mul(s.ad_value(2739), s.ad_value(2735)), A::sub(A::sub_from_scalar(1.0, A::scale(s.ad_value(2736), 12.0)), A::mul(A::sub(A::add(s.ad_value(2734), A::scale(s.ad_value(2736), 19.2)), A::scale(A::mul(s.ad_value(2734), s.ad_value(2736)), 12.0)), s.ad_value(2737))));
        }

        if ((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) {
            s.store_div_ad(849, A::mul(A::mul(A::square(s.ad_value(1892)), s.ad_value(1888)), s.ad_value(1879)), A::square(s.ad_value(1890)));
        }

        s.v[2768] = if (s.v[275] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) && (s.v[2768] != 0.0)) {
            s.store_add_ad_rhs(848, 848, A::div(A::mul(s.ad_value(2746), A::offset(A::scale(s.ad_value(2736), 12.0), 1.0)), A::mul(A::mul(A::scale(s.ad_value(2740), 12.0), s.ad_value(2740)), s.ad_value(1919))));
        }

        if (((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) && (s.v[2768] != 0.0)) {
            s.store_sub_ad_rhs(2747, 2747, A::div(A::mul(A::mul(s.ad_value(2746), s.ad_value(2735)), A::offset(s.ad_value(2737), 1.0)), A::mul(s.ad_value(2740), s.ad_value(1919))));
        }

        if ((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) {
            s.store_sqrt_ad(2748, A::div(s.ad_value(1920), s.ad_value(848)));
        }

        s.v[2769] = if (s.v[851] <= 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) && (s.v[2769] != 0.0)) {
            s.store_scalar(852, 0.0);
        }

        if (((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) && (!(s.v[2769] != 0.0))) {
            s.store_div_ad_lhs(852, A::mul(s.ad_value(2747), s.ad_value(2748)), 851);
        }

        if ((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) {
            s.store_ad(852, &{
                if (s.v[852] > 0.0) {
                    {
                        if (s.v[852] < 1.0) {
                            s.ad_value(852)
                        } else {
                            A::constant(1.0)
                        }
                    }
                } else {
                    A::constant(0.0)
                }
            });
        }

        if ((s.v[2762] != 0.0) && (s.v[2767] != 0.0)) {
            s.store_div_ad_lhs(850, A::mul(s.ad_value(852), s.ad_value(851)), 2748);
        }

        s.store_scale_ad(1908, A::abs(s.ad_value(830)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(1909, A::abs(s.ad_value(831)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(1910, A::abs(s.ad_value(828)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(1911, A::abs(s.ad_value(829)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(1912, A::mul(A::offset(s.ad_value(1869), 1.0), A::abs(s.ad_value(836))), (2.0 * 1.6021918e-19));

        s.store_scale_ad(1913, A::abs(s.ad_value(837)), (2.0 * 1.6021918e-19));

        s.store_scale_ad(1914, A::abs(s.ad_value(838)), (2.0 * 1.6021918e-19));

        s.v[2770] = if (s.v[820] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2770] != 0.0) {
            s.store_add(854, 1908, 1910);
        }

        if (s.v[2770] != 0.0) {
            s.store_add(855, 1909, 1911);
        }

        if (s.v[2770] != 0.0) {
            s.copy_ad(856, 1913);
        }

        if (s.v[2770] != 0.0) {
            s.store_add(857, 1914, 1912);
        }

        if (!(s.v[2770] != 0.0)) {
            s.store_add(854, 1909, 1910);
        }

        if (!(s.v[2770] != 0.0)) {
            s.store_add(855, 1908, 1911);
        }

        if (!(s.v[2770] != 0.0)) {
            s.store_add(856, 1913, 1912);
        }

        if (!(s.v[2770] != 0.0)) {
            s.copy_ad(857, 1914);
        }

        s.v[2771] = if (((p.p46 != 0.0) && (s.v[285] > 0.0)) && (s.v[1864] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2771] != 0.0) {
            s.store_div_ad_lhs(1930, A::scale(s.ad_value(1867), 4.0), 1925);
        }

        if (s.v[2771] != 0.0) {
            s.store_div_ad(2749, A::sqrt(A::offset(s.ad_value(1930), 1.0)), A::offset(A::sqrt(A::offset(s.ad_value(1930), 1.1)), (-1.0)));
        }

        if (s.v[2771] != 0.0) {
            s.store_mul(1930, 760, 1916);
        }

        if (s.v[2771] != 0.0) {
            s.store_mul(2750, 1930, 2749);
        }

        if (s.v[2771] != 0.0) {
            s.store_mul_ad_rhs(2751, 1930, A::add(s.ad_value(1866), s.ad_value(2749)));
        }

        if (s.v[2771] != 0.0) {
            s.store_mul_ad_lhs(2752, A::mul(A::mul(A::neg(s.ad_value(1930)), s.ad_value(2749)), s.ad_value(1868)), 1865);
        }

        if (s.v[2771] != 0.0) {
            s.store_mul_ad(858, A::sub(s.ad_value(294), A::mul(A::sub(s.ad_value(295), A::mul(s.ad_value(296), s.ad_value(2750))), s.ad_value(2750))), A::ln(A::div(A::add(s.ad_value(2751), A::scale(s.ad_value(2752), 0.5)), A::sub(s.ad_value(2751), A::scale(s.ad_value(2752), 0.5)))));
        }

        if (s.v[2771] != 0.0) {
            s.store_add_ad_rhs(858, 858, A::mul(A::add(s.ad_value(295), A::mul(s.ad_value(296), A::sub(s.ad_value(2751), A::scale(s.ad_value(2750), 2.0)))), s.ad_value(2752)));
        }

        if (s.v[2771] != 0.0) {
            s.store_div_ad_lhs(858, A::mul(A::mul(A::mul(s.ad_value(1927), s.ad_value(835)), s.ad_value(1860)), s.ad_value(858)), 2750);
        }

        if (s.v[2771] != 0.0) {
            s.store_ad(858, &{
                if (s.v[858] > 0.0) {
                    s.ad_value(858)
                } else {
                    A::constant(0.0)
                }
            });
        }

        if (s.v[2771] != 0.0) {
            s.store_div_ad_lhs(2753, A::mul(s.ad_value(1916), A::add(s.ad_value(1866), s.ad_value(2749))), 2749);
        }

        if (s.v[2771] != 0.0) {
            s.store_div_ad(2754, A::mul(A::div(s.ad_value(1808), s.ad_value(1916)), s.ad_value(1866)), A::add(s.ad_value(1866), s.ad_value(2749)));
        }

        if (s.v[2771] != 0.0) {
            s.store_div_ad_lhs(2755, A::mul(A::mul(A::scale(s.ad_value(1916), ((-0.5) * 0.16666666666666666)), s.ad_value(1868)), s.ad_value(1865)), 2753);
        }

        if (s.v[2771] != 0.0) {
            s.store_square(2756, 2755);
        }

        if (s.v[2771] != 0.0) {
            s.store_scalar(2757, 0.0);
        }

        if (s.v[2771] != 0.0) {
            s.store_mul(1930, 1848, 1861);
        }

        s.v[2772] = if (s.v[1930] > 1e-10) { 1.0 } else { 0.0 };

        if ((s.v[2771] != 0.0) && (s.v[2772] != 0.0)) {
            s.store_offset_ad(2757, A::div(A::mul(s.ad_value(2749), s.ad_value(2753)), s.ad_value(1930)), (-1.0));
        }

        if (s.v[2771] != 0.0) {
            s.store_ad(2758, &{
                if ((1.0 - (12.0 * (s.v[2757] * s.v[2756]))) > 1e-20) {
                    A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2757), s.ad_value(2756)), 12.0))
                } else {
                    A::constant(1e-20)
                }
            });
        }

        if (s.v[2771] != 0.0) {
            s.store_div_from_scalar_ad(2759, 1.0, A::square(s.ad_value(2758)));
        }

        if (s.v[2771] != 0.0) {
            s.store_mul_ad_lhs(2760, A::mul(A::mul(s.ad_value(1918), s.ad_value(1916)), A::add(s.ad_value(1866), s.ad_value(2749))), 1860);
        }

        if (s.v[2771] != 0.0) {
            s.store_sub_ad(2761, A::add(s.ad_value(2754), A::scale(s.ad_value(2756), 12.0)), A::scale(A::mul(A::mul(A::offset(s.ad_value(2754), 1.0), s.ad_value(2756)), s.ad_value(2757)), 24.0));
        }

        if (s.v[2771] != 0.0) {
            s.store_ad(2761, &{
                if (s.v[2761] > 1e-40) {
                    s.ad_value(2761)
                } else {
                    A::constant(1e-40)
                }
            });
        }

        if (s.v[2771] != 0.0) {
            s.store_mul_ad_lhs(2761, A::mul(s.ad_value(2760), s.ad_value(2759)), 2761);
        }

        if (s.v[2771] != 0.0) {
            s.store_sqrt_ad(859, A::mul(s.ad_value(1928), s.ad_value(2761)));
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
        s.v[981] = if (p.p37 >= 0.0) { 1.0 } else { 0.0 };

        if (s.v[981] != 0.0) {
            s.store_scalar(0, 1.0);
        }

        if (!(s.v[981] != 0.0)) {
            s.store_scalar(0, (-1.0));
        }

        s.v[756] = (8.8541878176e-12 * 11.8);

        s.v[351] = (273.15 + p.p38);

        s.v[475] = 0.0;

        s.v[982] = if (p.p944 > 0.5) { 1.0 } else { 0.0 };

        if (s.v[982] != 0.0) {
            s.store_scalar(475, 1.0);
        }

        if (!(s.v[982] != 0.0)) {
            s.store_scalar(475, 0.0);
        }

        s.v[365] = (273.15 + p.p840);

        s.v[368] = (1.3806505e-23 / 1.6021918e-19);

        s.v[369] = (s.v[368] * s.v[365]);

        s.v[370] = (1.0 / s.v[369]);

        s.v[376] = ((-((0.000702 * s.v[365]) * s.v[365])) / (1108.0 + s.v[365]));

        s.v[379] = (p.p851 + s.v[376]);

        s.v[380] = (p.p852 + s.v[376]);

        s.v[381] = (p.p853 + s.v[376]);

        s.v[409] = (1.0 - p.p848);

        s.v[410] = (1.0 - p.p849);

        s.v[411] = (1.0 - p.p850);

        s.v[412] = (1.0 / s.v[409]);

        s.v[413] = (1.0 / s.v[410]);

        s.v[414] = (1.0 / s.v[411]);

        s.v[424] = (s.v[756] / p.p842);

        s.v[425] = ((p.p860 * s.v[756]) / p.p843);

        s.v[426] = ((p.p861 * s.v[756]) / p.p844);

        s.v[427] = (1.0 / s.v[424]);

        s.v[428] = (1.0 / s.v[425]);

        s.v[429] = (1.0 / s.v[426]);

        s.v[430] = (1.0 / p.p845);

        s.v[431] = (1.0 / p.p846);

        s.v[432] = (1.0 / p.p847);

        s.v[445] = (1.0 - (1.0 / p.p841));

        s.v[449] = (1.0 / p.p877);

        s.v[450] = (1.0 / p.p878);

        s.v[451] = (1.0 / p.p879);

        s.v[983] = if ((((p.p883 != 1.0) || (p.p884 != 1.0)) || (p.p885 != 1.0)) || (p.p886 != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[983] != 0.0) {
            s.store_scalar(474, 1.0);
        }

        if (!(s.v[983] != 0.0)) {
            s.store_scalar(474, 0.0);
        }

        s.v[984] = if (s.v[474] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[984] != 0.0) {
            s.store_scalar(458, (if ((p.p844 * p.p883) > 1e-18) { (p.p844 * p.p883) } else { 1e-18 }));
        }

        if (s.v[984] != 0.0) {
            s.store_scalar(459, (if ((p.p847 * p.p884) > 0.05) { (p.p847 * p.p884) } else { 0.05 }));
        }

        if (s.v[984] != 0.0) {
            s.store_scalar(460, (if ((if ((p.p850 * p.p885) > 0.05) { (p.p850 * p.p885) } else { 0.05 }) < 0.95) { (if ((p.p850 * p.p885) > 0.05) { (p.p850 * p.p885) } else { 0.05 }) } else { 0.95 }));
        }

        if (s.v[984] != 0.0) {
            s.store_scalar(461, (p.p853 * p.p886));
        }

        if (s.v[984] != 0.0) {
            s.store_offset(463, 461, s.v[376]);
        }

        if (s.v[984] != 0.0) {
            s.store_sub_from_scalar(468, 1.0, 460);
        }

        if (s.v[984] != 0.0) {
            s.store_div_from_scalar(469, 1.0, 468);
        }

        s.v[985] = if (p.p44 == 0.0) { 1.0 } else { 0.0 };

        if (s.v[985] != 0.0) {
            s.store_scalar(506, p.p842);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(507, p.p843);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(508, p.p844);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(509, p.p845);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(510, p.p846);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(511, p.p847);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(512, p.p848);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(513, p.p849);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(514, p.p850);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(515, p.p851);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(516, p.p852);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(517, p.p853);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(518, p.p854);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(519, p.p855);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(520, p.p856);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(523, p.p857);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(524, p.p858);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(525, p.p859);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(521, p.p860);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(522, p.p861);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(526, p.p862);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(527, p.p863);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(528, p.p864);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(529, p.p865);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(530, p.p866);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(531, p.p867);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(532, p.p868);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(533, p.p869);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(534, p.p870);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(535, p.p871);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(536, p.p872);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(537, p.p873);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(538, p.p874);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(539, p.p875);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(540, p.p876);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(541, p.p877);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(542, p.p878);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(543, p.p879);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(544, p.p880);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(545, p.p881);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(546, p.p882);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(554, p.p946);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(637, p.p889);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(638, p.p890);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(639, p.p891);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(640, p.p892);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(547, p.p883);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(548, p.p884);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(549, p.p885);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(550, p.p886);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(551, p.p887);
        }

        if (s.v[985] != 0.0) {
            s.store_scalar(552, p.p888);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(506, p.p893);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(507, p.p894);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(508, p.p895);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(509, p.p896);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(510, p.p897);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(511, p.p898);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(512, p.p899);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(513, p.p900);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(514, p.p901);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(515, p.p902);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(516, p.p903);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(517, p.p904);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(518, p.p905);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(519, p.p906);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(520, p.p907);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(523, p.p908);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(524, p.p909);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(525, p.p910);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(521, p.p911);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(522, p.p912);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(526, p.p913);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(527, p.p914);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(528, p.p915);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(529, p.p916);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(530, p.p917);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(531, p.p918);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(532, p.p919);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(533, p.p920);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(534, p.p921);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(535, p.p922);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(536, p.p923);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(537, p.p924);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(538, p.p925);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(539, p.p926);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(540, p.p927);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(541, p.p928);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(542, p.p929);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(543, p.p930);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(544, p.p931);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(545, p.p932);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(546, p.p933);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(554, p.p948);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(637, p.p940);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(638, p.p941);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(639, p.p942);
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
        if (!(s.v[985] != 0.0)) {
            s.store_scalar(640, p.p943);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(547, p.p934);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(548, p.p935);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(549, p.p936);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(550, p.p937);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(551, p.p938);
        }

        if (!(s.v[985] != 0.0)) {
            s.store_scalar(552, p.p939);
        }

        s.store_offset(555, 515, s.v[376]);

        s.store_offset(556, 516, s.v[376]);

        s.store_offset(557, 517, s.v[376]);

        s.store_sub_from_scalar(576, 1.0, 512);

        s.store_sub_from_scalar(577, 1.0, 513);

        s.store_sub_from_scalar(578, 1.0, 514);

        s.store_div_from_scalar(579, 1.0, 576);

        s.store_div_from_scalar(580, 1.0, 577);

        s.store_div_from_scalar(581, 1.0, 578);

        s.store_div_from_scalar(591, s.v[756], 506);

        s.store_div_ad_lhs(592, A::scale(s.ad_value(521), s.v[756]), 507);

        s.store_div_ad_lhs(593, A::scale(s.ad_value(522), s.v[756]), 508);

        s.store_div_from_scalar(594, 1.0, 591);

        s.store_div_from_scalar(595, 1.0, 592);

        s.store_div_from_scalar(596, 1.0, 593);

        s.store_div_from_scalar(597, 1.0, 509);

        s.store_div_from_scalar(598, 1.0, 510);

        s.store_div_from_scalar(599, 1.0, 511);

        s.store_div_from_scalar(615, 1.0, 541);

        s.store_div_from_scalar(616, 1.0, 542);

        s.store_div_from_scalar(617, 1.0, 543);

        s.v[986] = if ((((s.v[547] != 1.0) || (s.v[548] != 1.0)) || (s.v[549] != 1.0)) || (s.v[550] != 1.0)) { 1.0 } else { 0.0 };

        if (s.v[986] != 0.0) {
            s.store_scalar(636, 1.0);
        }

        if (!(s.v[986] != 0.0)) {
            s.store_scalar(636, 0.0);
        }

        s.v[987] = if (s.v[636] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[987] != 0.0) {
            s.store_ad(621, &{
                if ((s.v[508] * s.v[547]) > 1e-18) {
                    A::mul(s.ad_value(508), s.ad_value(547))
                } else {
                    A::constant(1e-18)
                }
            });
        }

        if (s.v[987] != 0.0) {
            s.store_ad(622, &{
                if ((s.v[511] * s.v[548]) > 0.05) {
                    A::mul(s.ad_value(511), s.ad_value(548))
                } else {
                    A::constant(0.05)
                }
            });
        }

        if (s.v[987] != 0.0) {
            s.store_ad(623, &{
                if ((if ((s.v[514] * s.v[549]) > 0.05) { (s.v[514] * s.v[549]) } else { 0.05 }) < 0.95) {
                    {
                        if ((s.v[514] * s.v[549]) > 0.05) {
                            A::mul(s.ad_value(514), s.ad_value(549))
                        } else {
                            A::constant(0.05)
                        }
                    }
                } else {
                    A::constant(0.95)
                }
            });
        }

        if (s.v[987] != 0.0) {
            s.store_mul(624, 517, 550);
        }

        if (s.v[987] != 0.0) {
            s.store_offset(626, 624, s.v[376]);
        }

        if (s.v[987] != 0.0) {
            s.store_sub_from_scalar(631, 1.0, 623);
        }

        if (s.v[987] != 0.0) {
            s.store_div_from_scalar(632, 1.0, 631);
        }

        s.v[352] = ((ctx.temperature() + p.p55) + p.p35);

        s.v[353] = (s.v[352] / s.v[351]);

        s.v[354] = (s.v[352] - s.v[351]);

        s.v[355] = ((s.v[352] * 1.3806505e-23) / 1.6021918e-19);

        s.v[356] = (1.0 / s.v[355]);

        s.v[366] = (((ctx.temperature() + p.p55) + p.p35)).max((273.15 + (-250.0)));

        s.v[367] = (s.v[366] / s.v[365]);

        s.v[371] = (s.v[368] * s.v[366]);

        s.v[372] = (1.0 / s.v[371]);

        s.v[377] = ((-((0.000702 * s.v[366]) * s.v[366])) / (1108.0 + s.v[366]));

        s.v[382] = (p.p851 + s.v[377]);

        s.v[383] = (p.p852 + s.v[377]);

        s.v[384] = (p.p853 + s.v[377]);

        s.v[385] = (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[379] * s.v[370]) - (s.v[382] * s.v[372])))) as f64).exp());

        s.v[386] = (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[380] * s.v[370]) - (s.v[383] * s.v[372])))) as f64).exp());

        s.v[387] = (((s.v[367]) as f64).powf(1.5) * (((0.5 * ((s.v[381] * s.v[370]) - (s.v[384] * s.v[372])))) as f64).exp());

        s.v[388] = ((p.p854 * s.v[385]) * s.v[385]);

        s.v[389] = ((p.p855 * s.v[386]) * s.v[386]);

        s.v[390] = ((p.p856 * s.v[387]) * s.v[387]);

        s.v[391] = ((p.p845 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[385]) as f64).ln()));

        s.v[392] = ((p.p846 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[386]) as f64).ln()));

        s.v[393] = ((p.p847 * s.v[367]) - ((2.0 * s.v[371]) * ((s.v[387]) as f64).ln()));

        s.v[394] = (s.v[391] + (s.v[371] * (((1.0 + ((((0.05 - s.v[391]) * s.v[372])) as f64).exp())) as f64).ln()));

        s.v[395] = (s.v[392] + (s.v[371] * (((1.0 + ((((0.05 - s.v[392]) * s.v[372])) as f64).exp())) as f64).ln()));

        s.v[396] = (s.v[393] + (s.v[371] * (((1.0 + ((((0.05 - s.v[393]) * s.v[372])) as f64).exp())) as f64).ln()));

        s.v[406] = (1.0 / s.v[394]);

        s.v[407] = (1.0 / s.v[395]);

        s.v[408] = (1.0 / s.v[396]);

        s.v[415] = (p.p842 * (((p.p845 * s.v[406])) as f64).powf(p.p848));

        s.v[416] = (p.p843 * (((p.p846 * s.v[407])) as f64).powf(p.p849));

        s.v[417] = (p.p844 * (((p.p847 * s.v[408])) as f64).powf(p.p850));

        s.v[418] = ((s.v[415] * s.v[394]) * s.v[412]);

        s.v[419] = ((s.v[416] * s.v[395]) * s.v[413]);

        s.v[420] = ((s.v[417] * s.v[396]) * s.v[414]);

        s.v[421] = (2.0 * s.v[415]);

        s.v[422] = (2.0 * s.v[416]);

        s.v[423] = (2.0 * s.v[417]);

        s.v[433] = ((0.5 * s.v[382])).max(s.v[371]);

        s.v[434] = ((0.5 * s.v[383])).max(s.v[371]);

        s.v[435] = ((0.5 * s.v[384])).max(s.v[371]);

        s.v[436] = (s.v[433] * s.v[372]);

        s.v[437] = (s.v[434] * s.v[372]);

        s.v[438] = (s.v[435] * s.v[372]);

        s.v[439] = (((((((32.0 * p.p865) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[433] * s.v[433]) * s.v[433]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[440] = (((((((32.0 * p.p866) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[434] * s.v[434]) * s.v[434]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[441] = (((((((32.0 * p.p867) * 9.1093826e-31) * 1.6021918e-19) * ((s.v[435] * s.v[435]) * s.v[435]))) as f64).sqrt() / (3.0 * 1.05457168e-34));

        s.v[442] = (p.p871 * (1.0 + (p.p874 * (s.v[366] - s.v[365]))));

        s.v[443] = (p.p872 * (1.0 + (p.p875 * (s.v[366] - s.v[365]))));

        s.v[444] = (p.p873 * (1.0 + (p.p876 * (s.v[366] - s.v[365]))));

        if !(s.v[442] > 0.0) {
            s.store_scalar(442, 0.0);
        }

        if !(s.v[443] > 0.0) {
            s.store_scalar(443, 0.0);
        }

        if !(s.v[444] > 0.0) {
            s.store_scalar(444, 0.0);
        }

        s.v[1007] = if (s.v[474] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1007] != 0.0) {
            s.store_offset(462, 461, s.v[377]);
        }

        if (s.v[1007] != 0.0) {
            s.store_scale_ad(464, A::exp(A::scale(A::sub(A::scale(s.ad_value(463), s.v[370]), A::scale(s.ad_value(462), s.v[372])), 0.5)), ((s.v[367]) as f64).powf(1.5));
        }

        if (s.v[1007] != 0.0) {
            s.store_sub_ad(465, A::scale(s.ad_value(459), s.v[367]), A::scale(A::ln(s.ad_value(464)), (2.0 * s.v[371])));
        }

        if (s.v[1007] != 0.0) {
            s.store_add_ad_rhs(466, 465, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(465)), s.v[372])), 1.0)), s.v[371]));
        }

        if (s.v[1007] != 0.0) {
            s.store_div_from_scalar(467, 1.0, 466);
        }

        if (s.v[1007] != 0.0) {
            s.store_mul_ad_rhs(470, 458, A::pow(A::mul(s.ad_value(459), s.ad_value(467)), s.ad_value(460)));
        }

        if (s.v[1007] != 0.0) {
            s.store_mul_ad_lhs(471, A::mul(s.ad_value(470), s.ad_value(466)), 469);
        }

        if (s.v[1007] != 0.0) {
            s.store_scale(472, 470, 2.0);
        }

        s.store_offset(558, 515, s.v[377]);

        s.store_offset(559, 516, s.v[377]);

        s.store_offset(560, 517, s.v[377]);

        s.store_scale_ad(561, A::exp(A::scale(A::sub(A::scale(s.ad_value(555), s.v[370]), A::scale(s.ad_value(558), s.v[372])), 0.5)), ((s.v[367]) as f64).powf(1.5));

        s.store_scale_ad(562, A::exp(A::scale(A::sub(A::scale(s.ad_value(556), s.v[370]), A::scale(s.ad_value(559), s.v[372])), 0.5)), ((s.v[367]) as f64).powf(1.5));

        s.store_scale_ad(563, A::exp(A::scale(A::sub(A::scale(s.ad_value(557), s.v[370]), A::scale(s.ad_value(560), s.v[372])), 0.5)), ((s.v[367]) as f64).powf(1.5));

        s.store_mul_ad_lhs(564, A::mul(s.ad_value(518), s.ad_value(561)), 561);

        s.store_mul_ad_lhs(565, A::mul(s.ad_value(519), s.ad_value(562)), 562);

        s.store_mul_ad_lhs(566, A::mul(s.ad_value(520), s.ad_value(563)), 563);

        s.store_sub_ad(567, A::scale(s.ad_value(509), s.v[367]), A::scale(A::ln(s.ad_value(561)), (2.0 * s.v[371])));

        s.store_sub_ad(568, A::scale(s.ad_value(510), s.v[367]), A::scale(A::ln(s.ad_value(562)), (2.0 * s.v[371])));

        s.store_sub_ad(569, A::scale(s.ad_value(511), s.v[367]), A::scale(A::ln(s.ad_value(563)), (2.0 * s.v[371])));

        s.store_add_ad_rhs(570, 567, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(567)), s.v[372])), 1.0)), s.v[371]));

        s.store_add_ad_rhs(571, 568, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(568)), s.v[372])), 1.0)), s.v[371]));

        s.store_add_ad_rhs(572, 569, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(569)), s.v[372])), 1.0)), s.v[371]));

        s.store_div_from_scalar(573, 1.0, 570);

        s.store_div_from_scalar(574, 1.0, 571);

        s.store_div_from_scalar(575, 1.0, 572);

        s.store_mul_ad_rhs(582, 506, A::pow(A::mul(s.ad_value(509), s.ad_value(573)), s.ad_value(512)));

        s.store_mul_ad_rhs(583, 507, A::pow(A::mul(s.ad_value(510), s.ad_value(574)), s.ad_value(513)));

        s.store_mul_ad_rhs(584, 508, A::pow(A::mul(s.ad_value(511), s.ad_value(575)), s.ad_value(514)));

        s.store_mul_ad_lhs(585, A::mul(s.ad_value(582), s.ad_value(570)), 579);

        s.store_mul_ad_lhs(586, A::mul(s.ad_value(583), s.ad_value(571)), 580);

        s.store_mul_ad_lhs(587, A::mul(s.ad_value(584), s.ad_value(572)), 581);

        s.store_scale(588, 582, 2.0);

        s.store_scale(589, 583, 2.0);

        s.store_scale(590, 584, 2.0);

        s.store_max_with_scalar_ad(600, A::scale(s.ad_value(558), 0.5), s.v[371]);

        s.store_max_with_scalar_ad(601, A::scale(s.ad_value(559), 0.5), s.v[371]);

        s.store_max_with_scalar_ad(602, A::scale(s.ad_value(560), 0.5), s.v[371]);

        s.store_scale(603, 600, s.v[372]);

        s.store_scale(604, 601, s.v[372]);

        s.store_scale(605, 602, s.v[372]);

        s.store_scale_ad(606, A::sqrt(A::mul(A::scale(s.ad_value(529), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(600)), s.ad_value(600)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scale_ad(607, A::sqrt(A::mul(A::scale(s.ad_value(530), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(601)), s.ad_value(601)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_scale_ad(608, A::sqrt(A::mul(A::scale(s.ad_value(531), (32.0 * (9.1093826e-31 * 1.6021918e-19))), A::mul(A::square(s.ad_value(602)), s.ad_value(602)))), 1.0 / ((3.0 * 1.05457168e-34)));

        s.store_mul_ad_rhs(609, 535, A::offset(A::scale(s.ad_value(538), (s.v[366] - s.v[365])), 1.0));

        s.store_mul_ad_rhs(610, 536, A::offset(A::scale(s.ad_value(539), (s.v[366] - s.v[365])), 1.0));

        s.store_mul_ad_rhs(611, 537, A::offset(A::scale(s.ad_value(540), (s.v[366] - s.v[365])), 1.0));

        if !(s.v[609] > 0.0) {
            s.store_scalar(609, 0.0);
        }

        if !(s.v[610] > 0.0) {
            s.store_scalar(610, 0.0);
        }

        if !(s.v[611] > 0.0) {
            s.store_scalar(611, 0.0);
        }

        s.v[1008] = if (s.v[636] == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1008] != 0.0) {
            s.store_offset(625, 624, s.v[377]);
        }

        if (s.v[1008] != 0.0) {
            s.store_scale_ad(627, A::exp(A::scale(A::sub(A::scale(s.ad_value(626), s.v[370]), A::scale(s.ad_value(625), s.v[372])), 0.5)), ((s.v[367]) as f64).powf(1.5));
        }

        if (s.v[1008] != 0.0) {
            s.store_sub_ad(628, A::scale(s.ad_value(622), s.v[367]), A::scale(A::ln(s.ad_value(627)), (2.0 * s.v[371])));
        }

        if (s.v[1008] != 0.0) {
            s.store_add_ad_rhs(629, 628, A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(0.05, s.ad_value(628)), s.v[372])), 1.0)), s.v[371]));
        }

        if (s.v[1008] != 0.0) {
            s.store_div_from_scalar(630, 1.0, 629);
        }

        if (s.v[1008] != 0.0) {
            s.store_mul_ad_rhs(633, 621, A::pow(A::mul(s.ad_value(622), s.ad_value(630)), s.ad_value(623)));
        }

        if (s.v[1008] != 0.0) {
            s.store_mul_ad_lhs(634, A::mul(s.ad_value(633), s.ad_value(629)), 632);
        }

        if (s.v[1008] != 0.0) {
            s.store_scale(635, 633, 2.0);
        }

        s.v[1] = 1.0;

        s.v[2] = 1.0;

        s.v[312] = 0.0;

        s.v[313] = 0.0;

        s.v[3] = p.p0;

        s.v[4] = p.p1;

        s.v[5] = p.p2;

        s.v[6] = p.p3;

        s.v[7] = p.p4;

        s.v[8] = p.p8;

        s.v[647] = p.p19;

        s.v[648] = p.p20;

        s.v[649] = p.p21;

        s.v[674] = p.p22;

        s.v[675] = p.p23;

        s.v[676] = p.p24;

        s.v[650] = p.p25;

        s.v[651] = p.p26;

        s.v[677] = p.p27;

        s.v[678] = p.p28;

        s.v[10] = p.p14;

        s.v[1009] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1009] != 0.0) {
            s.store_scalar(1, (if (p.p9 > 1.0) { p.p9 } else { 1.0 }));
        }

        if (s.v[1009] != 0.0) {
            s.store_floor_ad(1, A::offset(s.ad_value(1), 0.5));
        }

        if (s.v[1009] != 0.0) {
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

        s.v[308] = (1e-6 / s.v[3]);

        s.store_div_from_scalar(309, 1e-6, 4);

        s.store_scale_ad(310, A::offset(A::scale(s.ad_value(309), p.p191), 1.0), (p.p189 * (1.0 + (p.p190 * s.v[308]))));

        s.store_scale_ad(311, A::offset(A::scale(s.ad_value(309), p.p195), 1.0), (p.p193 * (1.0 + (p.p194 * s.v[308]))));

        if (((s.v[3] + s.v[310]) - (2.0 * p.p192)) > 1e-9) {
            s.store_offset(312, 310, ((s.v[3]) + ((-(2.0 * p.p192)))));
        } else {
            s.store_scalar(312, 1e-9);
        }

        if (((s.v[4] + s.v[311]) - (2.0 * p.p196)) > 1e-9) {
            s.store_offset_ad(313, A::add(s.ad_value(4), s.ad_value(311)), (-(2.0 * p.p196)));
        } else {
            s.store_scalar(313, 1e-9);
        }

        s.store_div_from_scalar(314, 1e-6, 312);

        s.store_square(315, 314);

        s.store_div_from_scalar(316, 1e-6, 313);

        s.store_div_from_scalar(317, 1.0, 316);

        s.store_mul(318, 314, 316);

        s.store_div_from_scalar(319, 1.0, 318);

        if ((((s.v[3] + s.v[310]) - (2.0 * p.p192)) + p.p197) > 1e-9) {
            s.store_offset_ad(320, A::offset(A::offset(s.ad_value(310), s.v[3]), (-(2.0 * p.p192))), p.p197);
        } else {
            s.store_scalar(320, 1e-9);
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
        if ((((s.v[4] + s.v[311]) - (2.0 * p.p196)) + p.p198) > 1e-9) {
            s.store_offset_ad(321, A::offset(A::add(s.ad_value(4), s.ad_value(311)), (-(2.0 * p.p196))), p.p198);
        } else {
            s.store_scalar(321, 1e-9);
        }

        s.store_scale(322, 321, 1000000.0);

        if (((s.v[3] + s.v[310]) + p.p197) > 1e-9) {
            s.store_offset(323, 310, ((s.v[3]) + (p.p197)));
        } else {
            s.store_scalar(323, 1e-9);
        }

        if (((s.v[4] + s.v[311]) + p.p198) > 1e-9) {
            s.store_offset_ad(324, A::add(s.ad_value(4), s.ad_value(311)), p.p198);
        } else {
            s.store_scalar(324, 1e-9);
        }

        s.store_scale(325, 323, 1000000.0);

        s.store_scale(326, 324, 1000000.0);

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

        s.v[1010] = if (if self.param_given[121] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1010] != 0.0) {
            s.store_scalar(105, p.p121);
        }

        s.v[106] = p.p120;

        s.v[1011] = if (if self.param_given[122] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1011] != 0.0) {
            s.store_scalar(106, p.p122);
        }

        s.copy_ad(107, 105);

        s.v[1012] = if (if self.param_given[123] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1012] != 0.0) {
            s.store_scalar(107, p.p123);
        }

        s.copy_ad(108, 106);

        s.v[1013] = if (if self.param_given[124] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1013] != 0.0) {
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

        s.v[1014] = if (if self.param_given[137] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1014] != 0.0) {
            s.store_scalar(121, p.p137);
        }

        s.v[122] = p.p103;

        s.v[1015] = if (if self.param_given[138] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (s.v[1015] != 0.0) {
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

        s.v[173] = p.p187;

        s.v[1016] = if (p.p39 > 0.0) { 1.0 } else { 0.0 };

        if (s.v[1016] != 0.0) {
            s.store_add_ad(40, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p201), p.p200), p.p199), A::scale(s.ad_value(316), p.p202)), A::scale(s.ad_value(318), p.p203));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(41, A::add(A::offset(A::scale(s.ad_value(314), p.p205), p.p204), A::scale(s.ad_value(316), p.p206)), A::scale(s.ad_value(318), p.p207));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(42, p.p208);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(43, p.p209);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(44, p.p210);
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(331, &A::scale({
                if ((1.0 + ((p.p212 * s.v[316]) * (((1.0 + (s.v[313] / p.p213))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(316), p.p212), A::ln(A::offset(A::scale(s.ad_value(313), 1.0 / (p.p213)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p211));
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(332, &A::scale({
                if ((1.0 + ((p.p215 * s.v[316]) * (((1.0 + (s.v[313] / p.p216))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(316), p.p215), A::ln(A::offset(A::scale(s.ad_value(313), 1.0 / (p.p216)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p214));
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(333, &A::scale({
                if ((1.0 + ((p.p218 * s.v[316]) * (((1.0 + (s.v[313] / p.p216))) as f64).ln())) > 0.001) {
                    A::offset(A::mul(A::scale(s.ad_value(316), p.p218), A::ln(A::offset(A::scale(s.ad_value(313), 1.0 / (p.p216)), 1.0))), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p217));
        }

        s.v[1017] = if (s.v[312] > (2.0 * s.v[333])) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1017] != 0.0)) {
            s.store_scalar(334, 75000000000.0);
        }

        if ((s.v[1016] != 0.0) && (s.v[1017] != 0.0)) {
            s.store_sub_ad(335, A::sqrt(A::add(s.ad_value(331), A::scale(s.ad_value(332), 0.5))), A::sqrt(s.ad_value(331)));
        }

        if ((s.v[1016] != 0.0) && (s.v[1017] != 0.0)) {
            s.store_add_ad(336, A::sqrt(s.ad_value(331)), A::mul(s.ad_value(334), A::ln(A::offset(A::mul(A::div(A::scale(s.ad_value(333), 2.0), s.ad_value(312)), A::offset(A::exp(A::div(s.ad_value(335), s.ad_value(334))), (-1.0))), 1.0))));
        }

        if ((s.v[1016] != 0.0) && (s.v[1017] != 0.0)) {
            s.store_square(336, 336);
        }

        s.v[1018] = if (s.v[312] >= s.v[333]) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (!(s.v[1017] != 0.0))) && (s.v[1018] != 0.0)) {
            s.store_add_ad_rhs(336, 331, A::div(A::mul(s.ad_value(332), s.ad_value(333)), s.ad_value(312)));
        }

        if (((s.v[1016] != 0.0) && (!(s.v[1017] != 0.0))) && (!(s.v[1018] != 0.0))) {
            s.store_add_ad_rhs(336, 331, A::mul(s.ad_value(332), A::sub_from_scalar(2.0, A::div(s.ad_value(312), s.ad_value(333)))));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad_rhs(45, 336, A::sub(A::sub_from_scalar(1.0, A::scale(s.ad_value(314), p.p219)), A::scale(s.ad_value(315), p.p220)));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(46, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p223), p.p222), p.p221), A::scale(s.ad_value(316), p.p224)), A::scale(s.ad_value(318), p.p225));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(47, p.p226);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(48, p.p227);
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(49, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p230), p.p229), p.p228), A::scale(s.ad_value(316), p.p231)), A::scale(s.ad_value(318), p.p232));
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(50, &A::scale({
                if (1e-6 > (1.0 + (p.p234 * s.v[314]))) {
                    A::constant(1e-6)
                } else {
                    A::offset(A::scale(s.ad_value(314), p.p234), 1.0)
                }
            }, p.p233));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(55, p.p235);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(56, p.p236);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(57, p.p239);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(58, p.p240);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(51, A::mul(A::offset(A::scale(A::powf(s.ad_value(314), p.p243), p.p242), p.p241), A::offset(A::scale(s.ad_value(316), p.p244), 1.0)), A::offset(A::scale(s.ad_value(318), p.p245), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(52, p.p247);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(53, p.p246);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(54, p.p248);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(62, A::scale(A::powf(s.ad_value(314), p.p250), p.p249), A::offset(A::scale(s.ad_value(316), p.p251), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(63, p.p253);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(64, p.p252);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(59, A::scale(A::powf(s.ad_value(314), p.p255), p.p254), A::offset(A::scale(s.ad_value(316), p.p256), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(60, p.p258);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(61, p.p257);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale_ad(337, A::offset(A::scale(s.ad_value(316), p.p261), 1.0), p.p260);
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(338, &A::scale({
                if ((1.0 + (p.p263 * s.v[316])) > 0.001) {
                    A::offset(A::scale(s.ad_value(316), p.p263), 1.0)
                } else {
                    A::constant(0.001)
                }
            }, p.p262));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(339, A::offset(A::mul(A::div(A::mul(s.ad_value(337), s.ad_value(338)), s.ad_value(312)), A::sub_from_scalar(1.0, A::exp(A::div(A::neg(s.ad_value(312)), s.ad_value(338))))), 1.0), A::mul(A::div_from_scalar((p.p264 * p.p265), s.ad_value(312)), A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(s.ad_value(312)), 1.0 / (p.p265))))));
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(339, &{
                if (s.v[339] > 1e-15) {
                    s.ad_value(339)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(340, A::offset(A::scale(s.ad_value(316), p.p266), 1.0), A::mul(A::scale(s.ad_value(316), p.p267), A::ln(A::offset(A::scale(s.ad_value(313), 1.0 / (p.p268)), 1.0))));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad_lhs(65, A::div(A::scale(s.ad_value(313), p.p259), A::mul(s.ad_value(339), s.ad_value(312))), 340);
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(66, A::add(A::offset(A::scale(s.ad_value(314), p.p270), p.p269), A::scale(s.ad_value(316), p.p271)), A::scale(s.ad_value(318), p.p272));
        }

        if (s.v[1016] != 0.0) {
            s.store_scale_ad(67, A::offset(A::scale(s.ad_value(316), p.p274), 1.0), p.p273);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(68, p.p275);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(69, p.p276);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(70, p.p277);
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
        if (s.v[1016] != 0.0) {
            s.store_mul_ad(71, A::mul(A::offset(A::scale(A::powf(s.ad_value(314), p.p280), p.p279), p.p278), A::offset(A::scale(s.ad_value(316), p.p281), 1.0)), A::offset(A::scale(s.ad_value(318), p.p282), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(72, p.p283);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(73, p.p284);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(74, p.p285);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(75, A::mul(A::scale(A::offset(A::scale(s.ad_value(314), p.p287), 1.0), p.p286), A::offset(A::scale(s.ad_value(316), p.p288), 1.0)), A::offset(A::scale(s.ad_value(318), p.p289), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(76, p.p290);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(77, p.p291);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(78, A::scale(s.ad_value(316), p.p292), A::offset(A::scale(s.ad_value(316), p.p293), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(79, p.p294);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(80, p.p295);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(81, p.p296);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(82, A::mul(A::offset(A::mul(A::div(A::scale(s.ad_value(340), p.p298), s.ad_value(339)), A::powf(s.ad_value(314), p.p299)), p.p297), A::offset(A::scale(s.ad_value(316), p.p300), 1.0)), A::offset(A::scale(s.ad_value(318), p.p301), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(83, A::add(A::offset(A::scale(s.ad_value(314), p.p303), p.p302), A::scale(s.ad_value(316), p.p304)), A::scale(s.ad_value(318), p.p305));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(84, p.p306);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(85, p.p307);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(86, p.p308);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar_ad(87, p.p309, A::offset(A::scale(s.ad_value(314), p.p310), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(88, A::scale(A::powf(s.ad_value(314), p.p312), p.p311), A::offset(A::scale(s.ad_value(316), p.p313), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_powf(341, 314, p.p315);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_ad(89, A::mul(A::scale(s.ad_value(341), p.p314), A::offset(A::scale(s.ad_value(316), p.p317), 1.0)), A::offset(A::mul(A::scale(s.ad_value(314), p.p316), s.ad_value(341)), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_powf(341, 314, p.p319);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_ad(90, A::mul(A::scale(s.ad_value(341), p.p318), A::offset(A::scale(s.ad_value(316), p.p321), 1.0)), A::offset(A::mul(A::scale(s.ad_value(314), p.p320), s.ad_value(341)), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(91, p.p322);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(92, A::scale(A::offset(A::scale(s.ad_value(314), p.p324), 1.0), p.p323), A::offset(A::scale(s.ad_value(316), p.p325), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(93, p.p326);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(94, p.p327);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(95, A::scale(A::offset(A::scale(s.ad_value(314), p.p329), 1.0), p.p328), A::offset(A::scale(s.ad_value(316), p.p330), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(96, A::scale(A::offset(A::scale(s.ad_value(314), p.p332), 1.0), p.p331), A::offset(A::scale(s.ad_value(316), p.p333), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(97, p.p334);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(98, p.p335);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar(99, p.p336, 318);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar_ad(100, (p.p337 * p.p237), A::scale(s.ad_value(316), 1e-6));
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar_ad(101, (p.p338 * p.p238), A::scale(s.ad_value(316), 1e-6));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(102, p.p339);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(103, p.p340);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(104, p.p341);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(105, p.p340);
        }

        s.v[1019] = if (if self.param_given[342] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1019] != 0.0)) {
            s.store_scalar(105, p.p342);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(106, p.p341);
        }

        s.v[1020] = if (if self.param_given[343] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1020] != 0.0)) {
            s.store_scalar(106, p.p343);
        }

        if (s.v[1016] != 0.0) {
            s.copy_ad(107, 105);
        }

        s.v[1021] = if (if self.param_given[344] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1021] != 0.0)) {
            s.store_scalar(107, p.p344);
        }

        if (s.v[1016] != 0.0) {
            s.copy_ad(108, 106);
        }

        s.v[1022] = if (if self.param_given[345] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1022] != 0.0)) {
            s.store_scalar(108, p.p345);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(109, p.p346);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar_ad(110, (p.p347 * p.p237), A::scale(s.ad_value(316), 1e-6));
        }

        if (s.v[1016] != 0.0) {
            s.store_div_from_scalar_ad(111, (p.p348 * p.p238), A::scale(s.ad_value(316), 1e-6));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(112, p.p349);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(113, p.p350);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(114, p.p351);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(115, p.p352);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(116, p.p353);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(117, p.p354);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale_ad(118, A::mul(A::scale(s.ad_value(321), (8.8541878176e-12 * p.p210)), s.ad_value(320)), 1.0 / (p.p209));
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(125, 321, ((8.8541878176e-12 * p.p210) * (p.p237 * 1.0 / (p.p235))));
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(126, 321, ((8.8541878176e-12 * p.p210) * (p.p238 * 1.0 / (p.p236))));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(119, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p357), p.p356), p.p355), A::scale(s.ad_value(316), p.p358)), A::scale(s.ad_value(318), p.p359));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(120, A::add(A::offset(A::scale(s.ad_value(314), p.p361), p.p360), A::scale(s.ad_value(316), p.p362)), A::scale(s.ad_value(318), p.p363));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(32, p.p297);
        }

        s.v[1023] = if (if self.param_given[364] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1023] != 0.0)) {
            s.store_scalar(32, p.p364);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(33, p.p298);
        }

        s.v[1024] = if (if self.param_given[365] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1024] != 0.0)) {
            s.store_scalar(33, p.p365);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(34, p.p299);
        }

        s.v[1025] = if (if self.param_given[366] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1025] != 0.0)) {
            s.store_scalar(34, p.p366);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(35, p.p300);
        }

        s.v[1026] = if (if self.param_given[367] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1026] != 0.0)) {
            s.store_scalar(35, p.p367);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(36, p.p301);
        }

        s.v[1027] = if (if self.param_given[368] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1027] != 0.0)) {
            s.store_scalar(36, p.p368);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(121, A::mul(A::add(s.ad_value(32), A::mul(A::div(A::mul(s.ad_value(33), s.ad_value(340)), s.ad_value(339)), A::pow(s.ad_value(314), s.ad_value(34)))), A::offset(A::mul(s.ad_value(35), s.ad_value(316)), 1.0)), A::offset(A::mul(s.ad_value(36), s.ad_value(318)), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(37, p.p309);
        }

        s.v[1028] = if (if self.param_given[369] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1028] != 0.0)) {
            s.store_scalar(37, p.p369);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(38, p.p310);
        }

        s.v[1029] = if (if self.param_given[370] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1029] != 0.0)) {
            s.store_scalar(38, p.p370);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_ad_rhs(122, 37, A::offset(A::mul(s.ad_value(38), s.ad_value(314)), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(123, A::scale(A::powf(s.ad_value(314), p.p372), p.p371), A::offset(A::scale(s.ad_value(316), p.p373), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_powf(341, 314, p.p375);
        }

        if (s.v[1016] != 0.0) {
            s.store_div_ad(124, A::mul(A::scale(s.ad_value(341), p.p374), A::offset(A::scale(s.ad_value(316), p.p377), 1.0)), A::offset(A::mul(A::scale(s.ad_value(314), p.p376), s.ad_value(341)), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(127, p.p378);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(128, p.p379);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(129, p.p380);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(130, 325, p.p381);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(131, 322, p.p382);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(132, 322, p.p383);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(133, p.p384);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(134, p.p385);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(135, p.p386);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(136, p.p387);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(137, 326, p.p388);
        }

        if (s.v[1016] != 0.0) {
            s.store_scale(138, 326, p.p389);
        }

        if (s.v[1016] != 0.0) {
            s.store_sub_from_scalar_ad(998, 1.0, A::div_from_scalar((2.0 * p.p396), s.ad_value(312)));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(139, p.p390);
        }

        if (s.v[1016] != 0.0) {
            s.store_offset_scaled(344, 313, p.p399, (2.0 * p.p398));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(145, p.p400);
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(146, A::add(A::offset(A::scale(s.ad_value(314), p.p402), p.p401), A::scale(s.ad_value(316), p.p403)), A::scale(s.ad_value(318), p.p404));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(147, A::add(A::offset(A::scale(A::powf(s.ad_value(314), p.p407), p.p406), p.p405), A::scale(s.ad_value(316), p.p408)), A::scale(s.ad_value(318), p.p409));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(148, A::mul(A::scale(A::offset(A::scale(A::powf(s.ad_value(314), p.p412), p.p411), 1.0), p.p410), A::offset(A::scale(s.ad_value(316), p.p413), 1.0)), A::offset(A::scale(s.ad_value(318), p.p414), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_offset_ad(149, A::scale(A::powf(s.ad_value(314), p.p417), p.p416), p.p415);
        }

        if (s.v[1016] != 0.0) {
            s.store_offset_ad(347, A::mul(A::div_from_scalar((p.p418 * p.p419), s.ad_value(312)), A::sub_from_scalar(1.0, A::exp(A::scale(A::neg(s.ad_value(312)), 1.0 / (p.p419))))), 1.0);
        }

        if (s.v[1016] != 0.0) {
            s.store_ad(347, &{
                if (s.v[347] > 1e-15) {
                    s.ad_value(347)
                } else {
                    A::constant(1e-15)
                }
            });
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(150, A::div(A::scale(s.ad_value(344), p.p259), A::mul(s.ad_value(347), s.ad_value(312))), A::offset(A::scale(s.ad_value(316), p.p420), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(151, A::add(A::offset(A::scale(s.ad_value(314), p.p422), p.p421), A::scale(s.ad_value(316), p.p423)), A::scale(s.ad_value(318), p.p424));
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(152, A::scale(A::powf(s.ad_value(314), p.p426), p.p425), A::offset(A::scale(s.ad_value(316), p.p427), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(153, p.p428);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(154, p.p429);
        }

        if (s.v[1016] != 0.0) {
            s.store_mul_ad(155, A::scale(A::powf(s.ad_value(314), p.p431), p.p430), A::offset(A::scale(s.ad_value(316), p.p432), 1.0));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(156, p.p434);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(157, p.p433);
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(348, A::add(A::offset(A::scale(s.ad_value(314), p.p832), p.p831), A::scale(s.ad_value(316), p.p833)), A::scale(s.ad_value(318), p.p834));
        }

        if (s.v[1016] != 0.0) {
            s.store_add_ad(349, A::add(A::offset(A::scale(s.ad_value(314), p.p836), p.p835), A::scale(s.ad_value(316), p.p837)), A::scale(s.ad_value(318), p.p838));
        }

        if (s.v[1016] != 0.0) {
            s.store_offset_ad(173, A::div(A::scale(A::offset(A::offset(A::div_from_scalar(p.p458, s.ad_value(314)), 1.0), p.p457), p.p456), s.ad_value(316)), p.p455);
        }

        s.v[1031] = if ((((if self.param_given[460] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[461] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[462] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[463] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1031] != 0.0)) {
            s.store_add_ad(40, A::add(A::offset(A::scale(s.ad_value(314), p.p461), p.p460), A::scale(s.ad_value(316), p.p462)), A::scale(s.ad_value(318), p.p463));
        }

        s.v[1032] = if ((((if self.param_given[464] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[465] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[466] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[467] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1032] != 0.0)) {
            s.store_add_ad(41, A::add(A::offset(A::scale(s.ad_value(314), p.p465), p.p464), A::scale(s.ad_value(316), p.p466)), A::scale(s.ad_value(318), p.p467));
        }

        s.v[1033] = if ((((if self.param_given[468] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[469] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[470] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[471] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1033] != 0.0)) {
            s.store_add_ad(45, A::add(A::offset(A::scale(s.ad_value(314), p.p469), p.p468), A::scale(s.ad_value(316), p.p470)), A::scale(s.ad_value(318), p.p471));
        }

        s.v[1034] = if ((((if self.param_given[472] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[473] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[474] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[475] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1034] != 0.0)) {
            s.store_add_ad(46, A::add(A::offset(A::scale(s.ad_value(314), p.p473), p.p472), A::scale(s.ad_value(316), p.p474)), A::scale(s.ad_value(318), p.p475));
        }

        s.v[1035] = if ((((if self.param_given[476] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[477] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[478] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[479] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1035] != 0.0)) {
            s.store_add_ad(47, A::add(A::offset(A::scale(s.ad_value(314), p.p477), p.p476), A::scale(s.ad_value(316), p.p478)), A::scale(s.ad_value(318), p.p479));
        }

        s.v[1036] = if ((((if self.param_given[480] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[481] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[482] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[483] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1036] != 0.0)) {
            s.store_add_ad(49, A::add(A::offset(A::scale(s.ad_value(314), p.p481), p.p480), A::scale(s.ad_value(316), p.p482)), A::scale(s.ad_value(318), p.p483));
        }

        s.v[1037] = if ((((if self.param_given[484] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[485] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[486] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[487] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1037] != 0.0)) {
            s.store_add_ad(50, A::add(A::offset(A::scale(s.ad_value(314), p.p485), p.p484), A::scale(s.ad_value(316), p.p486)), A::scale(s.ad_value(318), p.p487));
        }

        s.v[1038] = if ((((if self.param_given[488] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[489] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[490] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[491] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

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
        if ((s.v[1016] != 0.0) && (s.v[1038] != 0.0)) {
            s.store_add_ad(57, A::add(A::offset(A::scale(s.ad_value(314), p.p489), p.p488), A::scale(s.ad_value(316), p.p490)), A::scale(s.ad_value(318), p.p491));
        }

        s.v[1039] = if ((((if self.param_given[492] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[493] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[494] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[495] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1039] != 0.0)) {
            s.store_add_ad(58, A::add(A::offset(A::scale(s.ad_value(314), p.p493), p.p492), A::scale(s.ad_value(316), p.p494)), A::scale(s.ad_value(318), p.p495));
        }

        s.v[1040] = if ((((if self.param_given[496] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[497] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[498] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[499] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1040] != 0.0)) {
            s.store_add_ad(51, A::add(A::offset(A::scale(s.ad_value(314), p.p497), p.p496), A::scale(s.ad_value(316), p.p498)), A::scale(s.ad_value(318), p.p499));
        }

        s.v[1041] = if ((((if self.param_given[504] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[505] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[506] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[507] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1041] != 0.0)) {
            s.store_add_ad(52, A::add(A::offset(A::scale(s.ad_value(314), p.p505), p.p504), A::scale(s.ad_value(316), p.p506)), A::scale(s.ad_value(318), p.p507));
        }

        s.v[1042] = if ((((if self.param_given[500] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[501] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[502] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[503] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1042] != 0.0)) {
            s.store_add_ad(53, A::add(A::offset(A::scale(s.ad_value(314), p.p501), p.p500), A::scale(s.ad_value(316), p.p502)), A::scale(s.ad_value(318), p.p503));
        }

        s.v[1043] = if ((((if self.param_given[508] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[509] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[510] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[511] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1043] != 0.0)) {
            s.store_add_ad(54, A::add(A::offset(A::scale(s.ad_value(314), p.p509), p.p508), A::scale(s.ad_value(316), p.p510)), A::scale(s.ad_value(318), p.p511));
        }

        s.v[1044] = if ((((if self.param_given[512] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[513] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[514] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[515] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1044] != 0.0)) {
            s.store_mul_ad_rhs(62, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p513), p.p512), A::scale(s.ad_value(316), p.p514)), A::scale(s.ad_value(318), p.p515)));
        }

        s.v[1045] = if ((((if self.param_given[520] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[521] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[522] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[523] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1045] != 0.0)) {
            s.store_add_ad(63, A::add(A::offset(A::scale(s.ad_value(314), p.p521), p.p520), A::scale(s.ad_value(316), p.p522)), A::scale(s.ad_value(318), p.p523));
        }

        s.v[1046] = if ((((if self.param_given[516] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[517] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[518] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[519] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1046] != 0.0)) {
            s.store_add_ad(64, A::add(A::offset(A::scale(s.ad_value(314), p.p517), p.p516), A::scale(s.ad_value(316), p.p518)), A::scale(s.ad_value(318), p.p519));
        }

        s.v[1047] = if ((((if self.param_given[524] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[525] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[526] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[527] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1047] != 0.0)) {
            s.store_mul_ad_rhs(59, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p525), p.p524), A::scale(s.ad_value(316), p.p526)), A::scale(s.ad_value(318), p.p527)));
        }

        s.v[1048] = if ((((if self.param_given[532] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[533] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[534] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[535] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1048] != 0.0)) {
            s.store_add_ad(60, A::add(A::offset(A::scale(s.ad_value(314), p.p533), p.p532), A::scale(s.ad_value(316), p.p534)), A::scale(s.ad_value(318), p.p535));
        }

        s.v[1049] = if ((((if self.param_given[528] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[529] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[530] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[531] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1049] != 0.0)) {
            s.store_add_ad(61, A::add(A::offset(A::scale(s.ad_value(314), p.p529), p.p528), A::scale(s.ad_value(316), p.p530)), A::scale(s.ad_value(318), p.p531));
        }

        s.v[1050] = if ((((if self.param_given[536] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[537] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[538] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[539] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1050] != 0.0)) {
            s.store_mul_ad(65, A::div(s.ad_value(313), s.ad_value(312)), A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p537), p.p536), A::scale(s.ad_value(316), p.p538)), A::scale(s.ad_value(318), p.p539)));
        }

        s.v[1051] = if ((((if self.param_given[540] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[541] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[542] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[543] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1051] != 0.0)) {
            s.store_add_ad(66, A::add(A::offset(A::scale(s.ad_value(314), p.p541), p.p540), A::scale(s.ad_value(316), p.p542)), A::scale(s.ad_value(318), p.p543));
        }

        s.v[1052] = if ((((if self.param_given[544] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[545] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[546] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[547] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1052] != 0.0)) {
            s.store_add_ad(67, A::add(A::offset(A::scale(s.ad_value(314), p.p545), p.p544), A::scale(s.ad_value(316), p.p546)), A::scale(s.ad_value(318), p.p547));
        }

        s.v[1053] = if ((((if self.param_given[548] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[549] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[550] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[551] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1053] != 0.0)) {
            s.store_add_ad(69, A::add(A::offset(A::scale(s.ad_value(314), p.p549), p.p548), A::scale(s.ad_value(316), p.p550)), A::scale(s.ad_value(318), p.p551));
        }

        s.v[1054] = if ((((if self.param_given[552] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[553] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[554] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[555] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1054] != 0.0)) {
            s.store_add_ad(71, A::add(A::offset(A::scale(s.ad_value(314), p.p553), p.p552), A::scale(s.ad_value(316), p.p554)), A::scale(s.ad_value(318), p.p555));
        }

        s.v[1055] = if ((((if self.param_given[556] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[557] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[558] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[559] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1055] != 0.0)) {
            s.store_add_ad(73, A::add(A::offset(A::scale(s.ad_value(314), p.p557), p.p556), A::scale(s.ad_value(316), p.p558)), A::scale(s.ad_value(318), p.p559));
        }

        s.v[1056] = if ((((if self.param_given[560] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[561] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[562] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[563] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1056] != 0.0)) {
            s.store_add_ad(75, A::add(A::offset(A::scale(s.ad_value(314), p.p561), p.p560), A::scale(s.ad_value(316), p.p562)), A::scale(s.ad_value(318), p.p563));
        }

        s.v[1057] = if ((((if self.param_given[564] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[565] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[566] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[567] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1057] != 0.0)) {
            s.store_mul_ad_rhs(78, 316, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p565), p.p564), A::scale(s.ad_value(316), p.p566)), A::scale(s.ad_value(318), p.p567)));
        }

        s.v[1058] = if ((((if self.param_given[568] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[569] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[570] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[571] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1058] != 0.0)) {
            s.store_add_ad(79, A::add(A::offset(A::scale(s.ad_value(314), p.p569), p.p568), A::scale(s.ad_value(316), p.p570)), A::scale(s.ad_value(318), p.p571));
        }

        s.v[1059] = if ((((if self.param_given[572] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[573] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[574] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[575] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1059] != 0.0)) {
            s.store_add_ad(80, A::add(A::offset(A::scale(s.ad_value(314), p.p573), p.p572), A::scale(s.ad_value(316), p.p574)), A::scale(s.ad_value(318), p.p575));
        }

        s.v[1060] = if ((((if self.param_given[576] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[577] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[578] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[579] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1060] != 0.0)) {
            s.store_add_ad(81, A::add(A::offset(A::scale(s.ad_value(314), p.p577), p.p576), A::scale(s.ad_value(316), p.p578)), A::scale(s.ad_value(318), p.p579));
        }

        s.v[1061] = if ((((if self.param_given[580] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[581] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[582] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[583] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1061] != 0.0)) {
            s.store_mul_ad_rhs(82, 314, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p581), p.p580), A::scale(s.ad_value(316), p.p582)), A::scale(s.ad_value(318), p.p583)));
        }

        s.v[1062] = if ((((if self.param_given[584] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[585] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[586] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[587] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1062] != 0.0)) {
            s.store_add_ad(83, A::add(A::offset(A::scale(s.ad_value(314), p.p585), p.p584), A::scale(s.ad_value(316), p.p586)), A::scale(s.ad_value(318), p.p587));
        }

        s.v[1063] = if ((((if self.param_given[588] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[589] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[590] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[591] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1063] != 0.0)) {
            s.store_add_ad(84, A::add(A::offset(A::scale(s.ad_value(314), p.p589), p.p588), A::scale(s.ad_value(316), p.p590)), A::scale(s.ad_value(318), p.p591));
        }

        s.v[1064] = if ((((if self.param_given[592] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[593] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[594] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[595] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1064] != 0.0)) {
            s.store_add_ad(85, A::add(A::offset(A::scale(s.ad_value(314), p.p593), p.p592), A::scale(s.ad_value(316), p.p594)), A::scale(s.ad_value(318), p.p595));
        }

        s.v[1065] = if ((((if self.param_given[596] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[597] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[598] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[599] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1065] != 0.0)) {
            s.store_add_ad(87, A::add(A::offset(A::scale(s.ad_value(314), p.p597), p.p596), A::scale(s.ad_value(316), p.p598)), A::scale(s.ad_value(318), p.p599));
        }

        s.v[1066] = if ((((if self.param_given[600] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[601] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[602] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[603] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1066] != 0.0)) {
            s.store_mul_ad_rhs(88, 314, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p601), p.p600), A::scale(s.ad_value(316), p.p602)), A::scale(s.ad_value(318), p.p603)));
        }

        s.v[1067] = if ((((if self.param_given[604] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[605] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[606] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[607] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1067] != 0.0)) {
            s.store_add_ad(89, A::add(A::offset(A::scale(s.ad_value(314), p.p605), p.p604), A::scale(s.ad_value(316), p.p606)), A::scale(s.ad_value(318), p.p607));
        }

        s.v[1068] = if ((((if self.param_given[608] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[609] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[610] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[611] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1068] != 0.0)) {
            s.store_add_ad(90, A::add(A::offset(A::scale(s.ad_value(314), p.p609), p.p608), A::scale(s.ad_value(316), p.p610)), A::scale(s.ad_value(318), p.p611));
        }

        s.v[1069] = if ((((if self.param_given[612] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[613] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[614] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[615] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1069] != 0.0)) {
            s.store_add_ad(92, A::add(A::offset(A::scale(s.ad_value(314), p.p613), p.p612), A::scale(s.ad_value(316), p.p614)), A::scale(s.ad_value(318), p.p615));
        }

        s.v[1070] = if ((((if self.param_given[616] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[617] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[618] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[619] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1070] != 0.0)) {
            s.store_add_ad(94, A::add(A::offset(A::scale(s.ad_value(314), p.p617), p.p616), A::scale(s.ad_value(316), p.p618)), A::scale(s.ad_value(318), p.p619));
        }

        s.v[1071] = if ((((if self.param_given[620] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[621] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[622] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[623] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1071] != 0.0)) {
            s.store_add_ad(95, A::add(A::offset(A::scale(s.ad_value(314), p.p621), p.p620), A::scale(s.ad_value(316), p.p622)), A::scale(s.ad_value(318), p.p623));
        }

        s.v[1072] = if ((((if self.param_given[624] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[625] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[626] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[627] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1072] != 0.0)) {
            s.store_add_ad(96, A::add(A::offset(A::scale(s.ad_value(314), p.p625), p.p624), A::scale(s.ad_value(316), p.p626)), A::scale(s.ad_value(318), p.p627));
        }

        s.v[1073] = if ((((if self.param_given[628] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[629] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[630] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[631] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1073] != 0.0)) {
            s.store_mul_ad_rhs(99, 319, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p629), p.p628), A::scale(s.ad_value(316), p.p630)), A::scale(s.ad_value(318), p.p631)));
        }

        s.v[1074] = if ((((if self.param_given[632] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[633] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[634] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[635] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1074] != 0.0)) {
            s.store_mul_ad_rhs(100, 317, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p633), p.p632), A::scale(s.ad_value(316), p.p634)), A::scale(s.ad_value(318), p.p635)));
        }

        s.v[1075] = if ((((if self.param_given[636] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[637] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[638] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[639] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1075] != 0.0)) {
            s.store_mul_ad_rhs(101, 317, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p637), p.p636), A::scale(s.ad_value(316), p.p638)), A::scale(s.ad_value(318), p.p639)));
        }

        s.v[1076] = if ((((if self.param_given[640] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[641] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[642] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[643] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1076] != 0.0)) {
            s.store_add_ad(102, A::add(A::offset(A::scale(s.ad_value(314), p.p641), p.p640), A::scale(s.ad_value(316), p.p642)), A::scale(s.ad_value(318), p.p643));
        }

        s.v[1077] = if ((((if self.param_given[644] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[645] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[646] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[647] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1077] != 0.0)) {
            s.store_mul_ad_rhs(110, 317, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p645), p.p644), A::scale(s.ad_value(316), p.p646)), A::scale(s.ad_value(318), p.p647)));
        }

        s.v[1078] = if ((((if self.param_given[648] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[649] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[650] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[651] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1078] != 0.0)) {
            s.store_mul_ad_rhs(111, 317, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p649), p.p648), A::scale(s.ad_value(316), p.p650)), A::scale(s.ad_value(318), p.p651)));
        }

        s.v[1079] = if ((((if self.param_given[652] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[653] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[654] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[655] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1079] != 0.0)) {
            s.store_add_ad(114, A::add(A::offset(A::scale(s.ad_value(314), p.p653), p.p652), A::scale(s.ad_value(316), p.p654)), A::scale(s.ad_value(318), p.p655));
        }

        s.v[1080] = if ((((if self.param_given[656] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[657] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[658] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[659] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1080] != 0.0)) {
            s.store_add_ad(115, A::add(A::offset(A::scale(s.ad_value(314), p.p657), p.p656), A::scale(s.ad_value(316), p.p658)), A::scale(s.ad_value(318), p.p659));
        }

        s.v[1081] = if ((((if self.param_given[660] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[661] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[662] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[663] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1081] != 0.0)) {
            s.store_mul_ad(118, A::scale(A::mul(s.ad_value(322), s.ad_value(320)), 1000000.0), A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p661), p.p660), A::scale(s.ad_value(316), p.p662)), A::scale(s.ad_value(318), p.p663)));
        }

        s.v[1082] = if ((((if self.param_given[664] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[665] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[666] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[667] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1082] != 0.0)) {
            s.store_add_ad(119, A::add(A::offset(A::scale(s.ad_value(314), p.p665), p.p664), A::scale(s.ad_value(316), p.p666)), A::scale(s.ad_value(318), p.p667));
        }

        s.v[1083] = if ((((if self.param_given[668] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[669] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[670] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[671] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1083] != 0.0)) {
            s.store_add_ad(120, A::add(A::offset(A::scale(s.ad_value(314), p.p669), p.p668), A::scale(s.ad_value(316), p.p670)), A::scale(s.ad_value(318), p.p671));
        }

        s.v[1084] = if ((((((((if self.param_given[672] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[673] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[674] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[675] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[580] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[581] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[582] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[583] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_scalar(28, p.p580);
        }

        s.v[1085] = if (if self.param_given[672] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) && (s.v[1085] != 0.0)) {
            s.store_scalar(28, p.p672);
        }

        if ((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_scalar(29, p.p581);
        }

        s.v[1086] = if (if self.param_given[673] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) && (s.v[1086] != 0.0)) {
            s.store_scalar(29, p.p673);
        }

        if ((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_scalar(30, p.p582);
        }

        s.v[1087] = if (if self.param_given[674] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) && (s.v[1087] != 0.0)) {
            s.store_scalar(30, p.p674);
        }

        if ((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_scalar(31, p.p583);
        }

        s.v[1088] = if (if self.param_given[675] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) && (s.v[1088] != 0.0)) {
            s.store_scalar(31, p.p675);
        }

        if ((s.v[1016] != 0.0) && (s.v[1084] != 0.0)) {
            s.store_mul_ad_rhs(121, 314, A::add(A::add(A::add(s.ad_value(28), A::mul(s.ad_value(29), s.ad_value(314))), A::mul(s.ad_value(30), s.ad_value(316))), A::mul(s.ad_value(31), s.ad_value(318))));
        }

        s.v[1089] = if ((((((((if self.param_given[676] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[677] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[678] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[679] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[596] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[597] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[598] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[599] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) {
            s.store_scalar(28, p.p596);
        }

        s.v[1090] = if (if self.param_given[676] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) && (s.v[1090] != 0.0)) {
            s.store_scalar(28, p.p676);
        }

        if ((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) {
            s.store_scalar(29, p.p597);
        }

        s.v[1091] = if (if self.param_given[677] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) && (s.v[1091] != 0.0)) {
            s.store_scalar(29, p.p677);
        }

        if ((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) {
            s.store_scalar(30, p.p598);
        }

        s.v[1092] = if (if self.param_given[678] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) && (s.v[1092] != 0.0)) {
            s.store_scalar(30, p.p678);
        }

        if ((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) {
            s.store_scalar(31, p.p599);
        }

        s.v[1093] = if (if self.param_given[679] { 1.0 } else { 0.0 } == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) && (s.v[1093] != 0.0)) {
            s.store_scalar(31, p.p679);
        }

        if ((s.v[1016] != 0.0) && (s.v[1089] != 0.0)) {
            s.store_add_ad(122, A::add(A::add(s.ad_value(28), A::mul(s.ad_value(29), s.ad_value(314))), A::mul(s.ad_value(30), s.ad_value(316))), A::mul(s.ad_value(31), s.ad_value(318)));
        }

        s.v[1094] = if ((((if self.param_given[680] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[681] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[682] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[683] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1094] != 0.0)) {
            s.store_mul_ad_rhs(123, 314, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p681), p.p680), A::scale(s.ad_value(316), p.p682)), A::scale(s.ad_value(318), p.p683)));
        }

        s.v[1095] = if ((((if self.param_given[684] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[685] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[686] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[687] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1095] != 0.0)) {
            s.store_mul_ad_rhs(124, 314, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p685), p.p684), A::scale(s.ad_value(316), p.p686)), A::scale(s.ad_value(318), p.p687)));
        }

        s.v[1096] = if ((((if self.param_given[688] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[689] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[690] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[691] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1096] != 0.0)) {
            s.store_mul_ad_rhs(125, 322, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p689), p.p688), A::scale(s.ad_value(316), p.p690)), A::scale(s.ad_value(318), p.p691)));
        }

        s.v[1097] = if ((((if self.param_given[692] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[693] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[694] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[695] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1097] != 0.0)) {
            s.store_mul_ad_rhs(126, 322, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p693), p.p692), A::scale(s.ad_value(316), p.p694)), A::scale(s.ad_value(318), p.p695)));
        }

        s.v[1098] = if ((((if self.param_given[696] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[697] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[698] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[699] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1098] != 0.0)) {
            s.store_mul_ad_rhs(130, 325, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p697), p.p696), A::scale(s.ad_value(316), p.p698)), A::scale(s.ad_value(318), p.p699)));
        }

        s.v[1099] = if ((((if self.param_given[700] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[701] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[702] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[703] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1099] != 0.0)) {
            s.store_mul_ad_rhs(131, 322, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p701), p.p700), A::scale(s.ad_value(316), p.p702)), A::scale(s.ad_value(318), p.p703)));
        }

        s.v[1100] = if ((((if self.param_given[704] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[705] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[706] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[707] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1100] != 0.0)) {
            s.store_mul_ad_rhs(132, 322, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p705), p.p704), A::scale(s.ad_value(316), p.p706)), A::scale(s.ad_value(318), p.p707)));
        }

        s.v[1101] = if ((((if self.param_given[708] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[709] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[710] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[711] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1101] != 0.0)) {
            s.store_mul_ad_rhs(137, 326, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p709), p.p708), A::scale(s.ad_value(316), p.p710)), A::scale(s.ad_value(318), p.p711)));
        }

        s.v[1102] = if ((((if self.param_given[712] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[713] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[714] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[715] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1102] != 0.0)) {
            s.store_mul_ad_rhs(138, 326, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p713), p.p712), A::scale(s.ad_value(316), p.p714)), A::scale(s.ad_value(318), p.p715)));
        }

        s.v[1107] = if ((((if self.param_given[732] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[733] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[734] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[735] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1107] != 0.0)) {
            s.store_add_ad(145, A::add(A::offset(A::scale(s.ad_value(314), p.p733), p.p732), A::scale(s.ad_value(316), p.p734)), A::scale(s.ad_value(318), p.p735));
        }

        s.v[1108] = if ((((if self.param_given[736] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[737] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[738] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[739] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1108] != 0.0)) {
            s.store_add_ad(146, A::add(A::offset(A::scale(s.ad_value(314), p.p737), p.p736), A::scale(s.ad_value(316), p.p738)), A::scale(s.ad_value(318), p.p739));
        }

        s.v[1109] = if ((((if self.param_given[740] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[741] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[742] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[743] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1109] != 0.0)) {
            s.store_add_ad(147, A::add(A::offset(A::scale(s.ad_value(314), p.p741), p.p740), A::scale(s.ad_value(316), p.p742)), A::scale(s.ad_value(318), p.p743));
        }

        s.v[1110] = if ((((if self.param_given[744] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[745] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[746] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[747] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1110] != 0.0)) {
            s.store_add_ad(148, A::add(A::offset(A::scale(s.ad_value(314), p.p745), p.p744), A::scale(s.ad_value(316), p.p746)), A::scale(s.ad_value(318), p.p747));
        }

        s.v[1111] = if ((((if self.param_given[748] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[749] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[750] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[751] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1111] != 0.0)) {
            s.store_add_ad(149, A::add(A::offset(A::scale(s.ad_value(314), p.p749), p.p748), A::scale(s.ad_value(316), p.p750)), A::scale(s.ad_value(318), p.p751));
        }

        s.v[1112] = if ((((if self.param_given[752] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[753] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[754] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[755] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1112] != 0.0)) {
            s.store_mul_ad(150, A::div(s.ad_value(344), s.ad_value(312)), A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p753), p.p752), A::scale(s.ad_value(316), p.p754)), A::scale(s.ad_value(318), p.p755)));
        }

        s.v[1113] = if ((((if self.param_given[756] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[757] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[758] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[759] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1113] != 0.0)) {
            s.store_add_ad(151, A::add(A::offset(A::scale(s.ad_value(314), p.p757), p.p756), A::scale(s.ad_value(316), p.p758)), A::scale(s.ad_value(318), p.p759));
        }

        s.v[1114] = if ((((if self.param_given[760] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[761] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[762] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[763] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1114] != 0.0)) {
            s.store_mul_ad_rhs(152, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p761), p.p760), A::scale(s.ad_value(316), p.p762)), A::scale(s.ad_value(318), p.p763)));
        }

        s.v[1115] = if ((((if self.param_given[764] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[765] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[766] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[767] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1115] != 0.0)) {
            s.store_add_ad(153, A::add(A::offset(A::scale(s.ad_value(314), p.p765), p.p764), A::scale(s.ad_value(316), p.p766)), A::scale(s.ad_value(318), p.p767));
        }

        s.v[1116] = if ((((if self.param_given[768] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[769] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[770] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[771] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1116] != 0.0)) {
            s.store_add_ad(154, A::add(A::offset(A::scale(s.ad_value(314), p.p769), p.p768), A::scale(s.ad_value(316), p.p770)), A::scale(s.ad_value(318), p.p771));
        }

        s.v[1117] = if ((((if self.param_given[772] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[773] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[774] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[775] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1117] != 0.0)) {
            s.store_mul_ad_rhs(155, 315, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p773), p.p772), A::scale(s.ad_value(316), p.p774)), A::scale(s.ad_value(318), p.p775)));
        }

        s.v[1118] = if ((((if self.param_given[780] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[781] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[782] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[783] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1118] != 0.0)) {
            s.store_add_ad(156, A::add(A::offset(A::scale(s.ad_value(314), p.p781), p.p780), A::scale(s.ad_value(316), p.p782)), A::scale(s.ad_value(318), p.p783));
        }

        s.v[1119] = if ((((if self.param_given[776] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[777] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[778] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[779] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1119] != 0.0)) {
            s.store_add_ad(157, A::add(A::offset(A::scale(s.ad_value(314), p.p777), p.p776), A::scale(s.ad_value(316), p.p778)), A::scale(s.ad_value(318), p.p779));
        }

        s.v[1124] = if ((((if self.param_given[800] { 1.0 } else { 0.0 } == 1.0) || (if self.param_given[801] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[802] { 1.0 } else { 0.0 } == 1.0)) || (if self.param_given[803] { 1.0 } else { 0.0 } == 1.0)) { 1.0 } else { 0.0 };

        if ((s.v[1016] != 0.0) && (s.v[1124] != 0.0)) {
            s.store_mul_ad_rhs(173, 319, A::add(A::add(A::offset(A::scale(s.ad_value(314), p.p801), p.p800), A::scale(s.ad_value(316), p.p802)), A::scale(s.ad_value(318), p.p803)));
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(1005, 0.0);
        }

        if (s.v[1016] != 0.0) {
            s.store_scalar(1006, 0.0);
        }

    }
}
