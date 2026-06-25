#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{A, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
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
        s.v[2353] = if (s.v[847] > (0.5 * s.v[234])) { 1.0 } else { 0.0 };

        if (((s.v[2349] != 0.0) && (s.v[2350] != 0.0)) && (s.v[2353] != 0.0)) {
            s.store_offset_ad(2027, A::div(A::scale(s.ad_value(847), 2.0), s.ad_value(234)), (-1.0));
        }

        if (((s.v[2349] != 0.0) && (s.v[2350] != 0.0)) && (s.v[2353] != 0.0)) {
            s.store_mul_ad(847, A::scale(s.ad_value(234), 0.5), A::offset(A::div(s.ad_value(2027), A::sqrt(A::offset(A::square(s.ad_value(2027)), 1.0))), 1.0));
        }

        s.v[2547] = if (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0)) { 1.0 } else { 0.0 };

        s.v[2548] = if ((p.p45 > 0.0) || (p.p47 > 0.0)) { 1.0 } else { 0.0 };

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.copy_ad(2388, 728);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.copy_ad(2389, 738);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.copy_ad(2390, 729);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.copy_ad(2391, 1820);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.copy_ad(2392, 1821);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2396, 0.0);
        }

        s.v[2549] = if (p.p47 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2549] != 0.0)) {
            s.store_add_ad_lhs(2391, A::scale(A::sub(A::add(s.ad_value(828), s.ad_value(827)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(828), s.ad_value(827)), A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(749)))), 0.5), 747);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2549] != 0.0)) {
            s.store_add_ad_lhs(1886, A::sub(s.ad_value(827), A::scale(A::sub(s.ad_value(2391), A::sqrt(A::add(A::mul(s.ad_value(2391), s.ad_value(2391)), s.ad_value(748)))), 0.5)), 750);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2549] != 0.0)) {
            s.copy_ad(2392, 1886);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2549] != 0.0)) {
            s.copy_ad(2388, 745);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2549] != 0.0)) {
            s.copy_ad(2389, 748);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2549] != 0.0)) {
            s.copy_ad(2390, 746);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_sub_ad_lhs(2395, A::sub(s.ad_value(829), s.ad_value(2396)), 700);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_add_ad_rhs(2397, 2392, A::scale(A::sub(s.ad_value(826), s.ad_value(830)), 0.5));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2409, 1.0);
        }

        s.v[2550] = if (s.v[190] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale(2400, 2388, s.v[361]);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale(2401, 2397, s.v[361]);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale(2402, 2395, s.v[361]);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_offset_ad(2028, A::div(A::scale(s.ad_value(2390), 0.5), A::sqrt(s.ad_value(2400))), 1.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_add_ad_rhs(2029, 2400, A::mul(s.ad_value(2390), A::sqrt(s.ad_value(2400))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_sub_ad(2403, A::add(A::div(A::sub(s.ad_value(2402), s.ad_value(2029)), s.ad_value(2028)), A::scale(s.ad_value(2400), 0.5)), A::mul(A::offset(s.ad_value(191), 1.0), s.ad_value(2401)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_offset_scaled(2404, 2400, 0.5, 2.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_add(2405, 2400, 2401);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_sub_ad(2028, A::sub(A::sub(s.ad_value(2402), s.ad_value(2405)), A::mul(s.ad_value(2390), A::sqrt(s.ad_value(2405)))), A::scale(A::ln(A::add(A::div(s.ad_value(2400), s.ad_value(2390)), A::sqrt(s.ad_value(2400)))), 2.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_add_ad_lhs(2406, A::scale(s.ad_value(2028), 2.0), 2404);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale_ad(2028, A::add(A::add(s.ad_value(2403), s.ad_value(2406)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2403), s.ad_value(2406)), A::sub(s.ad_value(2403), s.ad_value(2406))), 20.0))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_sub_ad_lhs(2029, A::scale(A::sub(s.ad_value(2402), s.ad_value(2401)), 2.0), 2404);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale_ad(2407, A::sub(A::add(s.ad_value(2028), s.ad_value(2029)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2028), s.ad_value(2029)), A::sub(s.ad_value(2028), s.ad_value(2029))), 20.0))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale_ad(2028, A::sub(A::add(s.ad_value(2407), s.ad_value(2404)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2407), s.ad_value(2404)), A::sub(s.ad_value(2407), s.ad_value(2404))), 5.0))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_scale_ad(2408, A::add(A::sub(s.ad_value(2028), s.ad_value(2404)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2028), A::neg(s.ad_value(2404))), A::sub(s.ad_value(2028), A::neg(s.ad_value(2404)))), 20.0))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) {
            s.store_mul_ad_rhs(2029, 702, A::offset(A::div(s.ad_value(2408), s.ad_value(2404)), 1.0));
        }

        s.v[2551] = if (s.v[2029] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) && (s.v[2551] != 0.0)) {
            s.store_exp(2409, 2029);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2550] != 0.0)) && (!(s.v[2551] != 0.0))) {
            s.store_div_from_scalar_ad(2409, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2029)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2029)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2029)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_offset_ad(2410, A::mul(s.ad_value(701), s.ad_value(2409)), 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scale(2411, 2410, s.v[715]);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul_ad(2412, A::mul(s.ad_value(199), A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0)), A::offset(A::mul(s.ad_value(200), s.ad_value(2397)), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul_ad_rhs(2413, 2411, A::offset(s.ad_value(2412), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_div_from_scalar(2414, 1.0, 2413);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul_ad_rhs(2398, 2390, A::sqrt(A::scale(s.ad_value(2414), s.v[715])));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_square(2399, 2398);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_div_from_scalar(2415, 1.0, 2399);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul(2416, 2392, 2414);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul(2417, 2395, 2414);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_div_ad(2418, A::scale(s.ad_value(830), 2.0), A::offset(A::sqrt(A::offset(A::mul(s.ad_value(197), s.ad_value(830)), 1.0)), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul_ad(2419, A::mul(s.ad_value(196), s.ad_value(2418)), A::offset(A::mul(s.ad_value(198), s.ad_value(2397)), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul(2420, 2388, 2414);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_sqrt_ad(2028, A::add(A::square(s.ad_value(2391)), s.ad_value(2389)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_sqrt_ad(2029, A::add(A::mul(A::sub(s.ad_value(2391), s.ad_value(2419)), A::sub(s.ad_value(2391), s.ad_value(2419))), s.ad_value(2389)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul_ad(2421, A::scale(s.ad_value(2414), 0.5), A::sub(A::add(s.ad_value(2419), s.ad_value(2028)), s.ad_value(2029)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_add(2422, 2420, 2416);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_sub(2423, 2422, 2421);
        }

        s.v[2552] = if (p.p45 > 0.0) { 1.0 } else { 0.0 };

        s.v[2553] = if (((s.v[2423]) as f64).abs() < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2552] != 0.0)) && (s.v[2553] != 0.0)) {
            s.store_offset_ad(2424, A::mul(s.ad_value(2398), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2423), 0.5), A::sub_from_scalar(1.0, A::scale(s.ad_value(2423), 0.3125))))), 1.0);
        }

        s.v[2554] = if (s.v[2423] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2552] != 0.0)) && (!(s.v[2553] != 0.0))) && (s.v[2554] != 0.0)) {
            s.store_exp_ad(2438, A::neg(s.ad_value(2423)));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2552] != 0.0)) && (!(s.v[2553] != 0.0))) && (!(s.v[2554] != 0.0))) {
            s.store_div_from_scalar_ad(2438, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2423), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2423), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2423), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2552] != 0.0)) && (!(s.v[2553] != 0.0))) {
            s.store_scalar(2027, (if (s.v[2423] > 0.0) { 1.0 } else { (-1.0) }));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2552] != 0.0)) && (!(s.v[2553] != 0.0))) {
            s.store_offset_ad(2424, A::div(A::mul(A::mul(s.ad_value(2027), s.ad_value(2398)), A::sub_from_scalar(1.0, A::mul(s.ad_value(2438), A::sub_from_scalar(1.0, s.ad_value(2423))))), A::scale(A::sqrt(A::mul(s.ad_value(2423), A::sub_from_scalar(1.0, s.ad_value(2438)))), 2.0)), 1.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2552] != 0.0))) {
            s.store_offset_ad(2424, A::div(A::scale(s.ad_value(2398), 0.5), A::sqrt(s.ad_value(2423))), 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_sub_ad(2425, A::add(s.ad_value(2423), A::mul(s.ad_value(2398), A::sqrt(s.ad_value(2423)))), A::mul(s.ad_value(2424), A::ln(A::offset(s.ad_value(2424), (-1.0)))));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_div_ad_lhs(2426, A::sub(s.ad_value(2417), s.ad_value(2425)), 2424);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul_ad(2432, A::scale(s.ad_value(2399), 0.5), A::offset(A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2399)), 1.0)), (-1.0)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2431, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2433, 1.0);
        }

        s.v[2555] = if (s.v[2426] > (-30.0)) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_offset_ad(2427, A::mul(s.ad_value(2424), s.ad_value(2426)), (-1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_scale_ad(2027, A::add(s.ad_value(2427), A::sqrt(A::offset(A::square(s.ad_value(2427)), 10.0))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_sub_ad_rhs(2428, 2426, A::ln(s.ad_value(2027)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_scale_ad(2429, A::add(s.ad_value(2428), A::sqrt(A::offset(A::square(s.ad_value(2428)), 2.0))), 0.5);
        }

        s.v[2556] = if ((s.v[2426] - s.v[2429]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) && (s.v[2556] != 0.0)) {
            s.store_exp_ad(2027, A::sub(s.ad_value(2426), s.ad_value(2429)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) && (!(s.v[2556] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_div(2430, 2027, 2424);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_sub_ad_lhs(2027, A::scale(A::offset(s.ad_value(2429), 1.0), 2.0), 2430);
        }

        s.v[2557] = if (s.v[2430] > 1e-6) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) && (s.v[2557] != 0.0)) {
            s.store_mul_ad_rhs(2431, 2424, A::offset(A::sub(s.ad_value(2429), A::div(A::offset(A::sqrt(A::offset(A::mul(s.ad_value(2430), s.ad_value(2027)), 1.0)), (-1.0)), s.ad_value(2430))), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) && (!(s.v[2557] != 0.0))) {
            s.store_mul_ad(2431, A::mul(A::scale(s.ad_value(2424), 0.5), s.ad_value(2430)), A::offset(A::mul(A::scale(s.ad_value(2027), 0.25), s.ad_value(2027)), 1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_scale_ad(2027, A::add(A::offset(A::sub(s.ad_value(2417), s.ad_value(2431)), 2.0), A::sqrt(A::offset(A::mul(A::offset(A::sub(s.ad_value(2417), s.ad_value(2431)), (-2.0)), A::offset(A::sub(s.ad_value(2417), s.ad_value(2431)), (-2.0))), 1.0))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_mul_ad(2432, A::scale(s.ad_value(2399), 0.5), A::offset(A::sqrt(A::offset(A::mul(A::div_from_scalar(4.0, s.ad_value(2399)), s.ad_value(2027)), 1.0)), (-1.0)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_div_ad_rhs(2433, 2432, A::add(s.ad_value(2432), s.ad_value(2431)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2555] != 0.0)) {
            s.store_sub_ad_rhs(2423, 2422, A::mul(s.ad_value(2433), s.ad_value(2421)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_offset_scaled(2434, 2398, 0.7071067811865475, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scale(2435, 2434, 1e-5);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_div_from_scalar(2436, 1.0, 2434);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2543, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2437, 0.0);
        }

        s.v[2558] = if (s.v[2423] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2558] != 0.0)) {
            s.store_exp_ad(2438, A::neg(s.ad_value(2423)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2558] != 0.0))) {
            s.store_div_from_scalar_ad(2438, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2423), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2423), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2423), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2559] = if (((s.v[2417]) as f64).abs() <= s.v[2435]) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2559] != 0.0)) {
            s.store_scale_ad(2523, A::square(s.ad_value(2436)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2559] != 0.0)) {
            s.store_mul_ad(2437, A::mul(s.ad_value(2417), s.ad_value(2436)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2417), A::sub_from_scalar(1.0, s.ad_value(2438))), s.ad_value(2398)), s.ad_value(2523)), 1.0));
        }

        s.v[2560] = if (s.v[2417] < (-s.v[2435])) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_neg(2525, 2417);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_scaled_mul(2526, 2525, 2436, 1.25);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_scale_ad(2527, A::sub(A::offset(s.ad_value(2526), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2526), (-6.0)), A::offset(s.ad_value(2526), (-6.0))), 64.0))), 0.5);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub(2522, 2525, 2527);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_add_ad(2528, A::square(s.ad_value(2522)), A::mul(s.ad_value(2399), A::offset(s.ad_value(2527), 1.0)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub_ad_lhs(2529, A::scale(s.ad_value(2522), 2.0), 2399);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub_ad_lhs(2530, A::ln(A::mul(s.ad_value(2528), s.ad_value(2415))), 2527);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_add(824, 2528, 2529);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2530), A::sub(A::scale(A::square(s.ad_value(2529)), 0.5), s.ad_value(2528))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_add_ad_rhs(2531, 2527, A::div(A::mul(A::mul(s.ad_value(2528), s.ad_value(824)), s.ad_value(2530)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530)), s.ad_value(2530)), s.ad_value(2529)), A::sub(A::scale(A::square(s.ad_value(2529)), 0.3333333333333333), s.ad_value(2528))))));
        }

        s.v[2561] = if (s.v[2531] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) && (s.v[2561] != 0.0)) {
            s.store_exp(2532, 2531);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) && (!(s.v[2561] != 0.0))) {
            s.store_scale_ad(2532, A::offset(A::mul(A::offset(s.ad_value(2531), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2531), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2531), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_div_from_scalar(2533, 1.0, 2532);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_div_from_scalar_ad(2522, 1.0, A::offset(A::square(s.ad_value(2531)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_mul_ad_lhs(2534, A::square(s.ad_value(2531)), 2522);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2531), s.ad_value(2522)), s.ad_value(2522)), 4.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_mul_ad_lhs(2536, A::mul(A::sub(A::scale(s.ad_value(2522), 8.0), A::scale(s.ad_value(2534), 12.0)), s.ad_value(2522)), 2522);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub(2522, 2525, 2531);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_mul(2523, 2438, 2533);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_add_ad(2537, A::scale(s.ad_value(2522), 2.0), A::mul(s.ad_value(2399), A::add(A::sub(A::offset(s.ad_value(2532), (-1.0)), s.ad_value(2523)), A::mul(s.ad_value(2438), A::sub_from_scalar(1.0, s.ad_value(2535))))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub_ad(2538, A::square(s.ad_value(2522)), A::mul(s.ad_value(2399), A::add(A::add(A::offset(A::sub(s.ad_value(2532), s.ad_value(2531)), (-1.0)), s.ad_value(2523)), A::mul(s.ad_value(2438), A::sub(A::offset(s.ad_value(2531), (-1.0)), s.ad_value(2534))))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub_from_scalar_ad(2522, 2.0, A::mul(s.ad_value(2399), A::sub(A::add(s.ad_value(2532), s.ad_value(2523)), A::mul(s.ad_value(2438), s.ad_value(2536)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub_ad(2522, A::square(s.ad_value(2537)), A::scale(A::mul(s.ad_value(2538), s.ad_value(2522)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (s.v[2560] != 0.0)) {
            s.store_sub_ad(2437, A::neg(s.ad_value(2531)), A::scale(A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_div_from_scalar_ad(2539, 1.0, A::offset(A::scale(s.ad_value(2398), 0.7324648775608221), 1.25));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_mul_ad_lhs(2540, A::offset(A::mul(A::scale(s.ad_value(2434), 1.25), s.ad_value(2539)), (-1.0)), 2539);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_mul_ad(2541, A::mul(s.ad_value(2417), s.ad_value(2436)), A::offset(A::mul(s.ad_value(2540), s.ad_value(2417)), 1.0));
        }

        s.v[2562] = if ((-s.v[2541]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (s.v[2562] != 0.0)) {
            s.store_exp_ad(2522, A::neg(s.ad_value(2541)));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (!(s.v[2562] != 0.0))) {
            s.store_div_from_scalar_ad(2522, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2541))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2541))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2541))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_from_scalar(2542, 1.0, 2522);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_ad(2543, A::add(s.ad_value(2417), A::scale(s.ad_value(2399), 0.5)), A::mul(s.ad_value(2398), A::sqrt(A::sub(A::add(s.ad_value(2417), A::scale(s.ad_value(2399), 0.25)), s.ad_value(2542)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_offset(2544, 2423, 3.0);
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
        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_ad(2527, A::scale(A::sub(A::add(s.ad_value(2543), s.ad_value(2544)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2543), s.ad_value(2544)), A::sub(s.ad_value(2543), s.ad_value(2544))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2544), A::sqrt(A::offset(A::square(s.ad_value(2544)), 5.0))), 0.5));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub(2522, 2417, 2527);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_exp_ad(2523, A::neg(s.ad_value(2527)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_div_from_scalar_ad(2524, 1.0, A::offset(A::square(s.ad_value(2527)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_mul_ad_lhs(2534, A::square(s.ad_value(2527)), 2524);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2527), s.ad_value(2524)), s.ad_value(2524)), 4.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_mul_ad_lhs(2536, A::mul(A::sub(A::scale(s.ad_value(2524), 8.0), A::scale(s.ad_value(2534), 12.0)), s.ad_value(2524)), 2524);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            let assign49800_ad_e64198: A = {
                if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2438] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2522)), A::mul(s.ad_value(2399), A::sub(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), A::mul(s.ad_value(2438), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534))))))
                }
            };
            s.store_ad(2528, &assign49800_ad_e64198);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_from_scalar_ad(2545, 1.0, A::scale(A::mul(s.ad_value(2399), A::sub(s.ad_value(2523), A::mul(s.ad_value(2438), s.ad_value(2536)))), 0.5));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_add_ad(2529, A::scale(s.ad_value(2522), 2.0), A::mul(s.ad_value(2399), A::sub(A::sub_from_scalar(1.0, s.ad_value(2523)), A::mul(s.ad_value(2438), A::offset(s.ad_value(2535), 1.0)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_add_ad(2530, A::sub(s.ad_value(2423), s.ad_value(2527)), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_add(824, 2528, 2529);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2530), A::sub(A::scale(A::square(s.ad_value(2529)), 0.5), A::mul(s.ad_value(2528), s.ad_value(2545)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            let assign49860_ad_e64345: A = A::add(s.ad_value(2527), A::div(A::mul(A::mul(s.ad_value(2528), s.ad_value(824)), s.ad_value(2530)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530)), s.ad_value(2530)), s.ad_value(2529)), A::sub(A::scale(A::square(s.ad_value(2529)), 0.3333333333333333), A::mul(s.ad_value(2528), s.ad_value(2545)))))));
            s.store_ad(2546, &assign49860_ad_e64345);
        }

        s.v[2563] = if (s.v[2546] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (s.v[2563] != 0.0)) {
            s.store_exp(2532, 2546);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (s.v[2563] != 0.0)) {
            s.store_div_from_scalar(2533, 1.0, 2532);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (s.v[2563] != 0.0)) {
            s.store_mul(2532, 2438, 2532);
        }

        s.v[2564] = if (s.v[2546] > (s.v[2423] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (!(s.v[2563] != 0.0))) && (s.v[2564] != 0.0)) {
            s.store_exp_ad(2532, A::sub(s.ad_value(2546), s.ad_value(2423)));
        }

        if ((((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (!(s.v[2563] != 0.0))) && (s.v[2564] != 0.0)) {
            s.store_div(2533, 2438, 2532);
        }

        if ((((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (!(s.v[2563] != 0.0))) && (!(s.v[2564] != 0.0))) {
            s.store_div_from_scalar_ad(2532, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2423), s.ad_value(2546)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2423), s.ad_value(2546)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2423), s.ad_value(2546)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) && (!(s.v[2563] != 0.0))) && (!(s.v[2564] != 0.0))) {
            s.store_div_from_scalar_ad(2533, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2546), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2546), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2546), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_div_from_scalar_ad(2522, 1.0, A::offset(A::square(s.ad_value(2546)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_mul_ad_lhs(2534, A::square(s.ad_value(2546)), 2522);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2546), s.ad_value(2522)), s.ad_value(2522)), 4.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_mul_ad_lhs(2536, A::mul(A::sub(A::scale(s.ad_value(2522), 8.0), A::scale(s.ad_value(2534), 12.0)), s.ad_value(2522)), 2522);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub(2522, 2417, 2546);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_add_ad(2537, A::scale(s.ad_value(2522), 2.0), A::mul(s.ad_value(2399), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2533)), s.ad_value(2532)), A::mul(s.ad_value(2438), A::offset(s.ad_value(2535), 1.0)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_ad(2538, A::square(s.ad_value(2522)), A::mul(s.ad_value(2399), A::sub(A::add(A::offset(A::add(s.ad_value(2533), s.ad_value(2546)), (-1.0)), s.ad_value(2532)), A::mul(s.ad_value(2438), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534))))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_from_scalar_ad(2522, 2.0, A::mul(s.ad_value(2399), A::sub(A::add(s.ad_value(2533), s.ad_value(2532)), A::mul(s.ad_value(2438), s.ad_value(2536)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_sub_ad(2522, A::square(s.ad_value(2537)), A::scale(A::mul(s.ad_value(2538), s.ad_value(2522)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (!(s.v[2559] != 0.0))) && (!(s.v[2560] != 0.0))) {
            s.store_add_ad_rhs(2437, 2546, A::scale(A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2440, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2441, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2442, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2443, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2444, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2445, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2446, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2447, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2448, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_sub(2449, 2417, 2437);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2450, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_mul(2451, 2413, 2449);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2452, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2453, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2457, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2458, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) {
            s.store_scalar(2460, 1.0);
        }

        s.v[2565] = if (s.v[2417] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1.0, A::offset(A::square(s.ad_value(2437)), 2.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_mul_ad_lhs(2439, A::square(s.ad_value(2437)), 2027);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_scale_ad(2440, A::mul(A::mul(s.ad_value(2437), s.ad_value(2027)), s.ad_value(2027)), 4.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_mul_ad_lhs(2441, A::mul(A::sub(A::scale(s.ad_value(2027), 8.0), A::scale(s.ad_value(2439), 12.0)), s.ad_value(2027)), 2027);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_scalar(2442, 0.0);
        }

        s.v[2566] = if (s.v[2437] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2566] != 0.0)) {
            s.store_exp(2442, 2437);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2566] != 0.0)) {
            s.store_div_from_scalar(2443, 1.0, 2442);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2566] != 0.0)) {
            s.store_mul(2442, 2438, 2442);
        }

        s.v[2567] = if (s.v[2437] > (s.v[2423] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2566] != 0.0))) && (s.v[2567] != 0.0)) {
            s.store_exp_ad(2442, A::sub(s.ad_value(2437), s.ad_value(2423)));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2566] != 0.0))) && (s.v[2567] != 0.0)) {
            s.store_div(2443, 2438, 2442);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2566] != 0.0))) && (!(s.v[2567] != 0.0))) {
            s.store_div_from_scalar_ad(2442, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2423), s.ad_value(2437)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2423), s.ad_value(2437)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2423), s.ad_value(2437)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2566] != 0.0))) && (!(s.v[2567] != 0.0))) {
            s.store_div_from_scalar_ad(2443, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2437), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2437), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2437), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_sub_ad_rhs(2444, 2442, A::mul(s.ad_value(2438), A::add(A::offset(s.ad_value(2437), 1.0), s.ad_value(2439))));
        }

        s.v[2568] = if (s.v[2437] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2568] != 0.0)) {
            s.store_scale_ad(2445, A::mul(A::square(s.ad_value(2437)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2437), A::sub_from_scalar(1.0, A::scale(s.ad_value(2437), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2568] != 0.0)) {
            s.store_scale_ad(2444, A::mul(A::mul(A::mul(A::mul(s.ad_value(2438), s.ad_value(2437)), s.ad_value(2437)), s.ad_value(2437)), A::offset(A::scale(s.ad_value(2437), 1.75), 1.0)), 0.16666666666666666);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2568] != 0.0)) {
            s.store_sqrt_ad(2027, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2437), A::sub_from_scalar(1.0, A::scale(s.ad_value(2437), 0.25))), 0.3333333333333333)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2568] != 0.0)) {
            s.store_scaled_mul(2446, 2437, 2027, 0.7071067811865475);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2568] != 0.0)) {
            s.store_offset_ad(2447, A::scale(A::div(A::mul(s.ad_value(2398), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2437), 0.5)), A::scale(A::square(s.ad_value(2437)), 0.16666666666666666))), s.ad_value(2027)), 0.7071067811865475), 1.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2568] != 0.0))) {
            s.store_add_ad_lhs(2445, A::offset(s.ad_value(2437), (-1.0)), 2443);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2568] != 0.0))) {
            s.store_sqrt(2446, 2445);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (!(s.v[2568] != 0.0))) {
            s.store_offset_ad(2447, A::scale(A::div(A::mul(s.ad_value(2398), A::sub_from_scalar(1.0, s.ad_value(2443))), s.ad_value(2446)), 0.5), 1.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) {
            s.store_div_ad(2448, A::offset(A::mul(A::scale(s.ad_value(708), 0.2), s.ad_value(2397)), 1.0), A::offset(A::mul(s.ad_value(708), s.ad_value(2397)), 1.0));
        }

        s.v[2569] = if (s.v[2444] > 1e-100) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_mul_ad_rhs(2449, 2398, A::sqrt(A::add(s.ad_value(2445), s.ad_value(2444))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_div_ad(2450, A::mul(A::mul(s.ad_value(2399), s.ad_value(2444)), s.ad_value(2413)), A::add(s.ad_value(2449), A::mul(s.ad_value(2398), s.ad_value(2446))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_mul_ad_lhs(2451, A::mul(s.ad_value(2446), s.ad_value(2398)), 2413);
        }

        s.v[2570] = if (s.v[217] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (s.v[2570] != 0.0)) {
            s.store_div_from_scalar_ad(2452, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(217), s.ad_value(2397))));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (!(s.v[2570] != 0.0))) {
            s.store_offset_ad(2452, A::mul(s.ad_value(217), s.ad_value(2397)), 1.0);
        }

        s.v[2571] = if (s.v[218] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (s.v[2571] != 0.0)) {
            s.store_sub_from_scalar_ad(2453, 1.0, A::mul(s.ad_value(218), s.ad_value(2450)));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (!(s.v[2571] != 0.0))) {
            s.store_div_from_scalar_ad(2453, 1.0, A::offset(A::mul(s.ad_value(218), s.ad_value(2450)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_mul_ad_lhs(2454, A::mul(A::mul(s.ad_value(757), s.ad_value(2452)), s.ad_value(2453)), 2450);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_mul_ad_rhs(2455, 774, A::add(s.ad_value(2451), A::mul(s.ad_value(775), s.ad_value(2450))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_ln_ad(2028, A::div(s.ad_value(2445), A::offset(A::add(s.ad_value(2445), s.ad_value(2444)), 1e-14)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_add_ad(2456, A::pow(A::mul(s.ad_value(2455), s.ad_value(704)), s.ad_value(705)), A::mul(s.ad_value(706), A::exp(A::mul(A::scale(s.ad_value(707), 0.5), s.ad_value(2028)))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_mul_ad_lhs(2457, A::add(A::offset(s.ad_value(2456), 1.0), s.ad_value(2454)), 2448);
        }

        s.v[2572] = if (s.v[221] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (s.v[2572] != 0.0)) {
            s.store_div_from_scalar_ad(2458, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(221), s.ad_value(2397))));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (!(s.v[2572] != 0.0))) {
            s.store_offset_ad(2458, A::mul(s.ad_value(221), s.ad_value(2397)), 1.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_mul(2029, 2450, 2458);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) {
            s.store_div_ad_rhs(2459, 2029, A::add(s.ad_value(223), s.ad_value(2029)));
        }

        s.v[2573] = if (s.v[222] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (s.v[2573] != 0.0)) {
            s.store_div_from_scalar_ad(2460, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(222), s.ad_value(2459))));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2548] != 0.0)) && (s.v[2565] != 0.0)) && (s.v[2569] != 0.0)) && (!(s.v[2573] != 0.0))) {
            s.store_offset_ad(2460, A::mul(s.ad_value(222), s.ad_value(2459)), 1.0);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2395, 1822);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2397, 1823);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2413, 1824);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2414, 1825);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2398, 1826);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2399, 1827);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2415, 1828);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2417, 1829);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2422, 1830);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2423, 1831);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2434, 1832);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2435, 1833);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2436, 1834);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2543, 1835);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2438, 1836);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2437, 1837);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2440, 1838);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2441, 1839);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2442, 1840);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2443, 1841);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2445, 1842);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2444, 1843);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2446, 1844);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2447, 1845);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2448, 1846);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2449, 1847);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2450, 1848);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2451, 1849);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2452, 1850);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2453, 1851);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2457, 1852);
        }

        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2458, 1853);
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
        if ((s.v[2547] != 0.0) && (!(s.v[2548] != 0.0))) {
            s.copy_ad(2460, 1854);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2393, 720);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2394, 777);
        }

        s.v[2574] = if (p.p48 != 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2547] != 0.0) && (s.v[2574] != 0.0)) {
            s.copy_ad(2393, 721);
        }

        if ((s.v[2547] != 0.0) && (s.v[2574] != 0.0)) {
            s.copy_ad(2394, 778);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2462, 0.0);
        }

        if (s.v[2547] != 0.0) {
            s.store_scale(2461, 2413, 4.60517018598809);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2478, 2461);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2479, 826);
        }

        if (s.v[2547] != 0.0) {
            s.store_mul(2480, 826, 2414);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2484, 2437);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2485, 0.0);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2488, 0.0);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2490, 2443);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2491, 2445);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2493, 2444);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2494, 2451);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2495, 2437);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2496, 2443);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2498, 2444);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2499, 2445);
        }

        if (s.v[2547] != 0.0) {
            s.store_sub(2500, 2417, 2437);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2501, 1.0);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2503, 1.0);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2502, 0.0);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2512, 2450);
        }

        if (s.v[2547] != 0.0) {
            s.store_mul(2516, 2500, 2413);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2513, 0.0);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2514, 2451);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2519, 0.0);
        }

        if (s.v[2547] != 0.0) {
            s.store_scalar(2518, 1.0);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2521, 2393);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(2520, 2516);
        }

        s.v[2575] = if (s.v[2417] > 0.0) { 1.0 } else { 0.0 };

        s.v[2576] = if (s.v[2444] > 1e-100) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_mul(2521, 2393, 2460);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_div(2462, 2521, 2457);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_add_ad_rhs(2463, 2449, A::scale(s.ad_value(2399), 0.5));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_div_ad_lhs(2027, A::div(A::mul(s.ad_value(2399), s.ad_value(2442)), s.ad_value(2463)), 2463);
        }

        s.v[2577] = if (s.v[2027] > 0.0001) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2577] != 0.0)) {
            s.store_sub_from_scalar(2028, 1.0, 2027);
        }

        s.v[2578] = if (s.v[2028] < 1e-10) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2577] != 0.0)) && (s.v[2578] != 0.0)) {
            s.store_scalar(2029, 1.0);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2577] != 0.0)) && (!(s.v[2578] != 0.0))) {
            s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (!(s.v[2577] != 0.0))) {
            s.store_scale(2029, 2027, 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_mul(2464, 2029, 2463);
        }

        s.v[2579] = if ((s.v[706] > 0.0) && (s.v[707] > 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_mul_ad_lhs(2465, A::scale(s.ad_value(2413), 0.475), 2464);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_sub_ad_rhs(2027, 2450, A::mul(s.ad_value(2447), s.ad_value(2465)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_scale_ad(2466, A::add(s.ad_value(2027), A::sqrt(A::offset(A::square(s.ad_value(2027)), 1e-12))), 0.5);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_add_ad(2467, A::sub(A::mul(s.ad_value(2413), s.ad_value(2449)), s.ad_value(2450)), A::mul(A::offset(s.ad_value(2447), (-1.0)), s.ad_value(2465)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_offset_ad(2468, A::div(A::mul(A::scale(s.ad_value(2399), 0.5), s.ad_value(2413)), s.ad_value(2467)), 1.0);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_add_ad_rhs(2027, 2467, A::mul(s.ad_value(775), s.ad_value(2466)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_ad(2469, &A::pow(A::mul(A::mul(s.ad_value(774), s.ad_value(2027)), s.ad_value(704)), s.ad_value(705)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_mul_ad_lhs(2028, A::div(A::mul(s.ad_value(705), A::offset(A::mul(s.ad_value(2468), A::sub_from_scalar(1.0, s.ad_value(775))), (-1.0))), s.ad_value(2027)), 2469);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_div(2027, 2466, 2467);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_mul_ad_rhs(2470, 706, A::pow(A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_mul_ad_lhs(2029, A::div(A::mul(s.ad_value(707), A::add(A::offset(s.ad_value(2468), (-1.0)), A::div_from_scalar(1.0, A::offset(s.ad_value(2027), 1.0)))), s.ad_value(2467)), 2470);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_mul_ad_lhs(2471, A::mul(A::mul(s.ad_value(757), s.ad_value(2452)), s.ad_value(2453)), 2466);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_offset_ad(2027, A::div(A::sub(s.ad_value(2028), A::mul(A::mul(A::mul(s.ad_value(757), s.ad_value(2452)), s.ad_value(2453)), s.ad_value(2468))), s.ad_value(2029)), 1.0);
        }

        s.v[2580] = if (s.v[2027] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) && (s.v[2580] != 0.0)) {
            s.store_scale_ad(2028, A::ln(A::offset(A::exp(A::scale(s.ad_value(2027), 2.0)), 1.0)), 0.5);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) && (!(s.v[2580] != 0.0))) {
            s.copy_ad(2028, 2027);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_div_ad(2472, A::mul(A::mul(A::neg(s.ad_value(2465)), s.ad_value(2029)), s.ad_value(2028)), A::add(A::add(A::offset(s.ad_value(2469), 1.0), s.ad_value(2470)), s.ad_value(2471)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2579] != 0.0)) {
            s.store_mul_ad_rhs(2473, 2464, A::offset(A::div(s.ad_value(2472), A::offset(A::sqrt(A::offset(A::square(s.ad_value(2472)), 1.0)), 1.0)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (!(s.v[2579] != 0.0))) {
            s.copy_ad(2473, 2464);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_scale_ad(2474, A::mul(A::mul(s.ad_value(2413), s.ad_value(2462)), s.ad_value(2473)), 0.7071067811865475);
        }

        s.v[2581] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) && (s.v[2581] != 0.0)) {
            s.store_div_ad_rhs(2474, 2474, A::sqrt(A::offset(s.ad_value(2474), 1.0)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_div_from_scalar_ad(2475, 2.0, A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2474), 4.0), 1.0)), 1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_mul(2027, 2475, 2474);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_mul_ad(2476, A::mul(s.ad_value(2473), s.ad_value(2475)), A::offset(A::div(A::mul(A::scale(s.ad_value(2027), 0.86), A::sub_from_scalar(1.0, A::mul(s.ad_value(2027), s.ad_value(2475)))), A::offset(A::mul(A::mul(A::scale(s.ad_value(2027), 4.0), s.ad_value(2027)), s.ad_value(2475)), 1.0)), 1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_scale(2477, 2476, 0.99);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_div_ad_lhs(2027, A::mul(A::mul(s.ad_value(2477), A::sub(s.ad_value(2477), A::scale(s.ad_value(2463), 2.0))), s.ad_value(2415)), 2444);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2576] != 0.0)) {
            s.store_mul_ad_rhs(2478, 2413, A::sub(s.ad_value(2477), A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2576] != 0.0))) {
            s.copy_ad(2478, 2461);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_offset(2027, 2394, 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_div_ad_lhs(2028, A::mul(A::sqrt(s.ad_value(2027)), s.ad_value(826)), 2478);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add_ad_lhs(2029, A::square(s.ad_value(2028)), 2027);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_scale(2027, 2028, 2.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_div_ad(2479, A::mul(s.ad_value(2478), s.ad_value(2027)), A::add(A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027)))));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2480, 2479, 2414);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add(2481, 2423, 2480);
        }

        s.v[2582] = if (s.v[2480] < 460.51701859880916) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2582] != 0.0)) {
            s.store_exp_ad(2482, A::neg(s.ad_value(2480)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2582] != 0.0))) {
            s.store_div_from_scalar_ad(2482, 1e-200, A::offset(A::mul(A::offset(s.ad_value(2480), (-460.51701859880916)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2480), (-460.51701859880916)), A::offset(A::scale(A::offset(s.ad_value(2480), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2483, 2438, 2482);
        }

        s.v[2583] = if (((s.v[2417]) as f64).abs() <= s.v[2435]) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2583] != 0.0)) {
            s.store_scale_ad(2523, A::square(s.ad_value(2436)), (0.16666666666666666 * 0.7071067811865475));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2583] != 0.0)) {
            s.store_mul_ad(2484, A::mul(s.ad_value(2417), s.ad_value(2436)), A::offset(A::mul(A::mul(A::mul(s.ad_value(2417), A::sub_from_scalar(1.0, s.ad_value(2483))), s.ad_value(2398)), s.ad_value(2523)), 1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_offset(2544, 2481, 3.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub_ad(2527, A::scale(A::sub(A::add(s.ad_value(2543), s.ad_value(2544)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(2543), s.ad_value(2544)), A::sub(s.ad_value(2543), s.ad_value(2544))), 5.0))), 0.5), A::scale(A::sub(s.ad_value(2544), A::sqrt(A::offset(A::square(s.ad_value(2544)), 5.0))), 0.5));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub(2522, 2417, 2527);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_exp_ad(2523, A::neg(s.ad_value(2527)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_div_from_scalar_ad(2524, 1.0, A::offset(A::square(s.ad_value(2527)), 2.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_mul_ad_lhs(2534, A::square(s.ad_value(2527)), 2524);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2527), s.ad_value(2524)), s.ad_value(2524)), 4.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_mul_ad_lhs(2536, A::mul(A::sub(A::scale(s.ad_value(2524), 8.0), A::scale(s.ad_value(2534), 12.0)), s.ad_value(2524)), 2524);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            let assign52130_ad_e66997: A = {
                if (1e-40 > ((s.v[2522] * s.v[2522]) - (s.v[2399] * (((s.v[2523] + s.v[2527]) - 1.0) - (s.v[2483] * ((s.v[2527] + 1.0) + s.v[2534])))))) {
                    A::constant(1e-40)
                } else {
                    A::sub(A::square(s.ad_value(2522)), A::mul(s.ad_value(2399), A::sub(A::offset(A::add(s.ad_value(2523), s.ad_value(2527)), (-1.0)), A::mul(s.ad_value(2483), A::add(A::offset(s.ad_value(2527), 1.0), s.ad_value(2534))))))
                }
            };
            s.store_ad(2528, &assign52130_ad_e66997);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub_from_scalar_ad(2545, 1.0, A::scale(A::mul(s.ad_value(2399), A::sub(s.ad_value(2523), A::mul(s.ad_value(2483), s.ad_value(2536)))), 0.5));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_add_ad(2529, A::scale(s.ad_value(2522), 2.0), A::mul(s.ad_value(2399), A::sub(A::sub_from_scalar(1.0, s.ad_value(2523)), A::mul(s.ad_value(2483), A::offset(s.ad_value(2535), 1.0)))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_add_ad(2530, A::sub(s.ad_value(2481), s.ad_value(2527)), A::ln(A::div(s.ad_value(2528), s.ad_value(2399))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_add(824, 2528, 2529);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2530), A::sub(A::scale(A::square(s.ad_value(2529)), 0.5), A::mul(s.ad_value(2528), s.ad_value(2545)))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            let assign52190_ad_e67126: A = A::add(s.ad_value(2527), A::div(A::mul(A::mul(s.ad_value(2528), s.ad_value(824)), s.ad_value(2530)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2530)), s.ad_value(2530)), s.ad_value(2529)), A::sub(A::scale(A::square(s.ad_value(2529)), 0.3333333333333333), A::mul(s.ad_value(2528), s.ad_value(2545)))))));
            s.store_ad(2546, &assign52190_ad_e67126);
        }

        s.v[2584] = if (s.v[2546] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (s.v[2584] != 0.0)) {
            s.store_exp(2532, 2546);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (s.v[2584] != 0.0)) {
            s.store_div_from_scalar(2533, 1.0, 2532);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (s.v[2584] != 0.0)) {
            s.store_mul(2532, 2483, 2532);
        }

        s.v[2585] = if (s.v[2546] > (s.v[2481] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (!(s.v[2584] != 0.0))) && (s.v[2585] != 0.0)) {
            s.store_exp_ad(2532, A::sub(s.ad_value(2546), s.ad_value(2481)));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (!(s.v[2584] != 0.0))) && (s.v[2585] != 0.0)) {
            s.store_div(2533, 2483, 2532);
        }

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2585] != 0.0))) {
            s.store_div_from_scalar_ad(2532, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2481), s.ad_value(2546)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) && (!(s.v[2584] != 0.0))) && (!(s.v[2585] != 0.0))) {
            s.store_div_from_scalar_ad(2533, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2546), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2546), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2546), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_div_from_scalar_ad(2522, 1.0, A::offset(A::square(s.ad_value(2546)), 2.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_mul_ad_lhs(2534, A::square(s.ad_value(2546)), 2522);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_scale_ad(2535, A::mul(A::mul(s.ad_value(2546), s.ad_value(2522)), s.ad_value(2522)), 4.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_mul_ad_lhs(2536, A::mul(A::sub(A::scale(s.ad_value(2522), 8.0), A::scale(s.ad_value(2534), 12.0)), s.ad_value(2522)), 2522);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub(2522, 2417, 2546);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_add_ad(2537, A::scale(s.ad_value(2522), 2.0), A::mul(s.ad_value(2399), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2533)), s.ad_value(2532)), A::mul(s.ad_value(2483), A::offset(s.ad_value(2535), 1.0)))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub_ad(2538, A::square(s.ad_value(2522)), A::mul(s.ad_value(2399), A::sub(A::add(A::offset(A::add(s.ad_value(2533), s.ad_value(2546)), (-1.0)), s.ad_value(2532)), A::mul(s.ad_value(2483), A::add(A::offset(s.ad_value(2546), 1.0), s.ad_value(2534))))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub_from_scalar_ad(2522, 2.0, A::mul(s.ad_value(2399), A::sub(A::add(s.ad_value(2533), s.ad_value(2532)), A::mul(s.ad_value(2483), s.ad_value(2536)))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_sub_ad(2522, A::square(s.ad_value(2537)), A::scale(A::mul(s.ad_value(2538), s.ad_value(2522)), 2.0));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2583] != 0.0))) {
            s.store_add_ad_rhs(2484, 2546, A::scale(A::div(s.ad_value(2538), A::add(s.ad_value(2537), A::sqrt(s.ad_value(2522)))), 2.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_sub(2485, 2484, 2437);
        }

        s.v[2586] = if (s.v[2485] < 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_add_ad(2486, A::scale(A::sub(s.ad_value(2417), s.ad_value(2437)), 2.0), A::mul(s.ad_value(2399), A::sub(A::add(A::sub_from_scalar(1.0, s.ad_value(2443)), A::mul(s.ad_value(2442), s.ad_value(2482))), A::mul(s.ad_value(2483), A::offset(s.ad_value(2440), 1.0)))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_mul_ad_lhs(2487, A::mul(s.ad_value(2399), A::sub_from_scalar(1.0, s.ad_value(2482))), 2444);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_sub_from_scalar_ad(2027, 2.0, A::mul(s.ad_value(2399), A::sub(A::add(s.ad_value(2443), A::mul(s.ad_value(2442), s.ad_value(2482))), A::mul(s.ad_value(2483), s.ad_value(2441)))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_sub_ad(2027, A::square(s.ad_value(2486)), A::scale(A::mul(s.ad_value(2027), s.ad_value(2487)), 2.0));
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
        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_scale_ad(2485, A::div(s.ad_value(2487), A::add(s.ad_value(2486), A::sqrt(s.ad_value(2027)))), 2.0);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2586] != 0.0)) {
            s.store_add(2484, 2437, 2485);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2488, 2485, 2413);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_div_ad(2489, A::square(s.ad_value(2484)), A::offset(A::square(s.ad_value(2484)), 2.0));
        }

        s.v[2587] = if (s.v[2484] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) {
            s.store_exp_ad(2490, A::neg(s.ad_value(2484)));
        }

        s.v[2588] = if (s.v[2484] < 1e-5) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (s.v[2588] != 0.0)) {
            s.store_scale_ad(2491, A::mul(A::square(s.ad_value(2484)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2484), A::sub_from_scalar(1.0, A::scale(s.ad_value(2484), 0.25))), 0.3333333333333333))), 0.5);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (s.v[2588] != 0.0)) {
            s.store_sqrt_ad(2027, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2484), A::sub_from_scalar(1.0, A::scale(s.ad_value(2484), 0.25))), 0.3333333333333333)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (s.v[2588] != 0.0)) {
            s.store_scaled_mul(2492, 2484, 2027, 0.7071067811865475);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (s.v[2588] != 0.0)) {
            s.store_mul_ad(2493, A::mul(A::mul(A::mul(A::scale(s.ad_value(2483), 0.16666666666666666), s.ad_value(2484)), s.ad_value(2484)), s.ad_value(2484)), A::offset(A::scale(s.ad_value(2484), 1.75), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (!(s.v[2588] != 0.0))) {
            s.store_add_ad_lhs(2491, A::offset(s.ad_value(2484), (-1.0)), 2490);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (!(s.v[2588] != 0.0))) {
            s.store_sqrt(2492, 2491);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2587] != 0.0)) && (!(s.v[2588] != 0.0))) {
            s.store_mul_ad_rhs(2493, 2483, A::sub(A::offset(A::sub(A::div_from_scalar(1.0, s.ad_value(2490)), s.ad_value(2484)), (-1.0)), s.ad_value(2489)));
        }

        s.v[2589] = if (s.v[2484] > (s.v[2481] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_exp_ad(2027, A::sub(s.ad_value(2484), s.ad_value(2481)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_div(2490, 2483, 2027);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) && (s.v[2589] != 0.0)) {
            s.store_sub_ad_rhs(2493, 2027, A::mul(s.ad_value(2483), A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) && (!(s.v[2589] != 0.0))) {
            s.store_div_from_scalar_ad(2490, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2484), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2484), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2484), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) && (!(s.v[2589] != 0.0))) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::offset(A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::sub(s.ad_value(2481), s.ad_value(2484)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) && (!(s.v[2589] != 0.0))) {
            s.store_sub_ad_rhs(2493, 2027, A::mul(s.ad_value(2483), A::add(A::offset(s.ad_value(2484), 1.0), s.ad_value(2489))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) {
            s.store_add_ad_lhs(2491, A::offset(s.ad_value(2484), (-1.0)), 2490);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2587] != 0.0))) {
            s.store_sqrt(2492, 2491);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul_ad_lhs(2494, A::mul(s.ad_value(2492), s.ad_value(2398)), 2413);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_scaled_add(2495, 2437, 2484, 0.5);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_scalar(2496, 0.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2027, 2490, 2443);
        }

        s.v[2590] = if (s.v[2027] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2590] != 0.0)) {
            s.store_sqrt(2496, 2027);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_scaled_add(2497, 2444, 2493, 0.5);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add_ad_rhs(2498, 2497, A::scale(A::mul(A::square(s.ad_value(2485)), A::sub(s.ad_value(2496), A::scale(s.ad_value(2415), 2.0))), 0.125));
        }

        s.v[2591] = if (s.v[2495] < 1e-5) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2591] != 0.0)) {
            s.store_scale_ad(2499, A::mul(A::square(s.ad_value(2495)), A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2495), A::sub_from_scalar(1.0, A::scale(s.ad_value(2495), 0.25))), 0.3333333333333333))), 0.5);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2591] != 0.0)) {
            s.store_mul_ad_rhs(2500, 2398, A::sqrt(A::add(s.ad_value(2498), s.ad_value(2499))));
        }

        s.v[2592] = if (s.v[730] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2591] != 0.0)) && (s.v[2592] != 0.0)) {
            s.store_div_from_scalar_ad(2501, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2591] != 0.0)) {
            s.store_sqrt_ad(2027, A::sub_from_scalar(1.0, A::scale(A::mul(s.ad_value(2495), A::sub_from_scalar(1.0, A::scale(s.ad_value(2495), 0.25))), 0.3333333333333333)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2591] != 0.0)) {
            s.store_scaled_mul(2502, 2495, 2027, 0.7071067811865475);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2591] != 0.0)) {
            s.store_add_ad_rhs(2503, 2501, A::scale(A::div(A::mul(s.ad_value(2398), A::add(A::sub_from_scalar(1.0, A::scale(s.ad_value(2495), 0.5)), A::scale(A::square(s.ad_value(2495)), 0.16666666666666666))), s.ad_value(2027)), 0.7071067811865475));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) {
            s.store_add_ad_lhs(2499, A::offset(s.ad_value(2495), (-1.0)), 2496);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) {
            s.store_mul_ad_rhs(2500, 2398, A::sqrt(A::add(s.ad_value(2498), s.ad_value(2499))));
        }

        s.v[2593] = if (s.v[730] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_add_ad(2504, A::sub_from_scalar(1.0, s.ad_value(2496)), A::scale(A::mul(s.ad_value(2500), s.ad_value(2415)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_div_from_scalar_ad(2501, 1.0, A::sqrt(A::offset(A::mul(s.ad_value(730), s.ad_value(2500)), 1.0)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_div_ad_rhs(2027, 2501, A::offset(s.ad_value(2501), 1.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_mul_ad_rhs(2505, 730, A::mul(A::mul(A::square(s.ad_value(2027)), s.ad_value(2399)), s.ad_value(2498)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_add_ad(2506, A::scale(A::sub(s.ad_value(2500), s.ad_value(2505)), 2.0), A::mul(s.ad_value(2399), A::add(A::sub_from_scalar(1.0, s.ad_value(2496)), s.ad_value(2498))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_mul_ad_rhs(2507, 2505, A::sub(s.ad_value(2505), A::scale(s.ad_value(2500), 2.0)));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_sub_from_scalar_ad(2508, 1.0, A::scale(A::mul(s.ad_value(2399), A::add(s.ad_value(2496), s.ad_value(2498))), 0.5));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_div_ad(2509, A::mul(s.ad_value(2507), s.ad_value(2506)), A::sub(A::square(s.ad_value(2506)), A::mul(s.ad_value(2508), s.ad_value(2507))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_add(2495, 2495, 2509);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_exp(2510, 2509);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_div(2496, 2496, 2510);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_mul(2498, 2498, 2510);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_add_ad_lhs(2499, A::offset(s.ad_value(2495), (-1.0)), 2496);
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_mul_ad_rhs(2500, 2398, A::sqrt(A::add(s.ad_value(2498), s.ad_value(2499))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_add_ad(2511, A::sub_from_scalar(1.0, s.ad_value(2496)), A::scale(A::mul(A::mul(s.ad_value(2500), s.ad_value(2501)), s.ad_value(2415)), 2.0));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_div_ad(2485, A::mul(A::mul(s.ad_value(2485), s.ad_value(2510)), A::add(s.ad_value(2504), s.ad_value(2497))), A::add(s.ad_value(2511), A::mul(s.ad_value(2510), s.ad_value(2497))));
        }

        if ((((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) && (s.v[2593] != 0.0)) {
            s.store_mul(2488, 2485, 2413);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) {
            s.store_sqrt(2502, 2499);
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2591] != 0.0))) {
            s.store_add_ad_rhs(2503, 2501, A::scale(A::div(A::mul(s.ad_value(2398), A::sub_from_scalar(1.0, s.ad_value(2496))), s.ad_value(2502)), 0.5));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul_ad_rhs(2512, 2413, A::div(A::mul(s.ad_value(2399), s.ad_value(2498)), A::add(s.ad_value(2500), A::mul(s.ad_value(2398), s.ad_value(2502)))));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add_ad_rhs(2513, 2512, A::mul(s.ad_value(2413), s.ad_value(2503)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul_ad_lhs(2514, A::mul(s.ad_value(2502), s.ad_value(2398)), 2413);
        }

        s.v[2594] = if (s.v[218] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2594] != 0.0)) {
            s.store_sub_from_scalar_ad(2453, 1.0, A::mul(s.ad_value(218), s.ad_value(2512)));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2594] != 0.0))) {
            s.store_div_from_scalar_ad(2453, 1.0, A::offset(A::mul(s.ad_value(218), s.ad_value(2512)), 1.0));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul_ad_lhs(2454, A::mul(A::mul(s.ad_value(757), s.ad_value(2452)), s.ad_value(2453)), 2512);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add_ad_rhs(2515, 2514, A::mul(s.ad_value(775), s.ad_value(2512)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add_ad_rhs(2516, 2514, A::mul(s.ad_value(776), s.ad_value(2512)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2517, 774, 2515);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_ln_ad(2028, A::div(s.ad_value(2499), A::offset(A::add(s.ad_value(2499), s.ad_value(2498)), 1e-14)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_add_ad(2456, A::pow(A::mul(s.ad_value(2517), s.ad_value(704)), s.ad_value(705)), A::mul(s.ad_value(706), A::exp(A::mul(A::scale(s.ad_value(707), 0.5), s.ad_value(2028)))));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul_ad_lhs(2518, A::add(A::offset(s.ad_value(2456), 1.0), s.ad_value(2454)), 2448);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_ln_ad(2519, A::div(A::offset(A::mul(A::sub(s.ad_value(826), s.ad_value(2488)), s.ad_value(779)), 1.0), A::offset(A::mul(A::sub(s.ad_value(2479), s.ad_value(2488)), s.ad_value(779)), 1.0)));
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2029, 2512, 2458);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_div_ad_rhs(2459, 2029, A::add(s.ad_value(223), s.ad_value(2029)));
        }

        s.v[2595] = if (s.v[222] < 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (s.v[2595] != 0.0)) {
            s.store_div_from_scalar_ad(2460, 1.0, A::sub_from_scalar(1.0, A::mul(s.ad_value(222), s.ad_value(2459))));
        }

        if (((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) && (!(s.v[2595] != 0.0))) {
            s.store_offset_ad(2460, A::mul(s.ad_value(222), s.ad_value(2459)), 1.0);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2521, 2393, 2460);
        }

        if ((s.v[2547] != 0.0) && (s.v[2575] != 0.0)) {
            s.store_mul(2520, 2500, 2413);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1887, 2395);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1888, 2413);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1889, 2398);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1890, 2417);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1891, 2422);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1892, 2451);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1893, 2488);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1894, 2494);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1895, 2501);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1896, 2503);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1897, 2512);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1898, 2513);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1899, 2516);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1900, 2518);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1901, 2519);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1902, 2521);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1903, 2520);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1932, 2414);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1933, 2435);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1934, 2495);
        }

        if (s.v[2547] != 0.0) {
            s.copy_ad(1935, 2500);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(745, 728);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1887, 1822);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1888, 1824);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1889, 1826);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1890, 1829);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1891, 1830);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1892, 1849);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1893, 1860);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1894, 1861);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1895, 1863);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1896, 1864);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1897, 1865);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1898, 1866);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1899, 1868);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1900, 1869);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1901, 1871);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1902, 1870);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1903, 1872);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1932, 1825);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1933, 1833);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1934, 1862);
        }

        if (!(s.v[2547] != 0.0)) {
            s.copy_ad(1935, 1931);
        }

        s.copy_ad(1904, 255);

        s.v[2596] = if (s.v[773] > 0.0) { 1.0 } else { 0.0 };

        if (s.v[2596] != 0.0) {
            s.store_div_ad_rhs(1904, 255, A::offset(A::mul(s.ad_value(773), A::powf(A::offset(A::square(s.ad_value(1899)), s.v[733]), ((-1.0) * 0.16666666666666666))), 1.0));
        }

        s.v[1905] = 1.0;

        s.v[1906] = 1.0;

        s.v[1907] = 0.0;

        s.v[1908] = 1.0;

        s.v[1909] = 1.0;

        s.copy_ad(2359, 1903);

        s.v[2362] = 0.0;

        s.v[2361] = 0.0;

        s.copy_ad(2363, 2359);

        s.v[2597] = if (s.v[1890] > 0.0) { 1.0 } else { 0.0 };

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
        if (s.v[2597] != 0.0) {
            s.store_mul_ad_lhs(2354, A::div(A::mul(A::add(s.ad_value(260), A::div(s.ad_value(261), s.ad_value(1898))), s.ad_value(1897)), s.ad_value(1898)), 1901);
        }

        s.v[2598] = if (s.v[2354] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2597] != 0.0) && (s.v[2598] != 0.0)) {
            s.store_div_from_scalar_ad(1905, 1.0, A::add(A::offset(s.ad_value(2354), 1.0), A::square(s.ad_value(2354))));
        }

        if ((s.v[2597] != 0.0) && (!(s.v[2598] != 0.0))) {
            s.store_sub_from_scalar(1905, 1.0, 2354);
        }

        if (s.v[2597] != 0.0) {
            s.store_mul(1906, 1900, 1905);
        }

        if (s.v[2597] != 0.0) {
            s.store_div(1907, 1902, 1906);
        }

        if (s.v[2597] != 0.0) {
            s.store_mul_ad_lhs(2355, A::mul(A::square(s.ad_value(1907)), s.ad_value(1893)), 1893);
        }

        s.v[2599] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2597] != 0.0) && (s.v[2599] != 0.0)) {
            s.store_div_ad_rhs(2355, 2355, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if (s.v[2597] != 0.0) {
            s.store_scale_ad(1908, A::mul(s.ad_value(1906), A::offset(A::sqrt(A::offset(A::scale(s.ad_value(2355), 2.0), 1.0)), 1.0)), 0.5);
        }

        if (s.v[2597] != 0.0) {
            s.store_div(2027, 1906, 1908);
        }

        if (s.v[2597] != 0.0) {
            s.store_mul_ad_rhs(2356, 1896, A::offset(A::scale(A::mul(A::mul(s.ad_value(2355), s.ad_value(2027)), s.ad_value(2027)), 0.5), 1.0));
        }

        if (s.v[2597] != 0.0) {
            s.store_div_ad_lhs(1909, A::mul(s.ad_value(2027), s.ad_value(1898)), 2356);
        }

        if (s.v[2597] != 0.0) {
            s.store_scaled_div(2357, 1893, 1909, 0.5);
        }

        if (s.v[2597] != 0.0) {
            s.store_square(2358, 2357);
        }

        if (s.v[2597] != 0.0) {
            s.store_add_ad_rhs(2359, 1903, A::scale(A::mul(A::mul(s.ad_value(1895), s.ad_value(1893)), A::add(A::offset(A::scale(A::mul(s.ad_value(2357), s.ad_value(1905)), 0.3333333333333333), (-1.0)), s.ad_value(1905))), 0.5));
        }

        if (s.v[2597] != 0.0) {
            s.store_scaled_mul(2027, 1896, 1893, 0.16666666666666666);
        }

        s.v[2600] = if (p.p49 == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2597] != 0.0) && (s.v[2600] != 0.0)) {
            s.store_scalar(2360, 0.0);
        }

        if ((s.v[2597] != 0.0) && (s.v[2600] != 0.0)) {
            s.store_mul_ad(2361, A::mul(A::scale(s.ad_value(1905), 0.5), s.ad_value(1905)), A::sub(s.ad_value(1897), A::mul(A::scale(s.ad_value(2027), 3.0), A::sub_from_scalar(2.0, s.ad_value(2357)))));
        }

        if ((s.v[2597] != 0.0) && (!(s.v[2600] != 0.0))) {
            s.store_mul_ad(2360, A::sub_from_scalar(1.0, s.ad_value(1905)), A::sub(s.ad_value(1897), A::scale(A::mul(s.ad_value(1896), s.ad_value(1893)), 0.5)));
        }

        if ((s.v[2597] != 0.0) && (!(s.v[2600] != 0.0))) {
            s.store_scale_ad(2361, A::add(A::mul(A::square(s.ad_value(1905)), A::sub(s.ad_value(1897), A::mul(s.ad_value(2027), A::sub(A::sub_from_scalar(1.0, s.ad_value(2357)), A::scale(s.ad_value(2358), 0.2))))), A::mul(s.ad_value(2360), A::offset(s.ad_value(1905), 1.0))), 0.5);
        }

        if (s.v[2597] != 0.0) {
            s.store_add_ad_lhs(2362, A::mul(s.ad_value(1905), A::add(s.ad_value(1897), A::mul(s.ad_value(2027), s.ad_value(2357)))), 2360);
        }

        if (s.v[2597] != 0.0) {
            s.store_sub(2363, 2359, 2362);
        }

        s.store_mul(851, 2359, 1904);

        s.store_mul_ad_lhs(853, A::neg(s.ad_value(2361)), 1904);

        s.store_mul_ad_lhs(852, A::neg(s.ad_value(2363)), 1904);

        s.v[2379] = 0.0;

        s.v[2380] = 0.0;

        s.v[2378] = 0.0;

        s.v[2601] = if ((s.v[268] > 0.0) || (s.v[269] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2601] != 0.0) {
            s.store_scalar(2368, 1.0);
        }

        if (s.v[2601] != 0.0) {
            s.copy_ad(2367, 1887);
        }

        s.v[2602] = if (s.v[272] > 1e-10) { 1.0 } else { 0.0 };

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_add_ad_lhs(2364, A::sub(s.ad_value(1887), s.ad_value(270)), 808);
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_scale_ad(2027, A::add(A::add(s.ad_value(2364), s.ad_value(808)), A::sqrt(A::add(A::mul(A::sub(s.ad_value(2364), s.ad_value(808)), A::sub(s.ad_value(2364), s.ad_value(808))), s.ad_value(809)))), 0.5);
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_mul_ad_rhs(2028, 2027, A::sub(A::sub(A::scale(s.ad_value(2027), 2.0), s.ad_value(808)), s.ad_value(2364)));
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_div(2029, 808, 2027);
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_mul(2365, 2364, 2029);
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_sqrt_ad(2366, A::sub_from_scalar(1.0, A::mul(s.ad_value(2365), s.ad_value(272))));
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_sub_ad_lhs(2367, A::add(A::div(A::sub_from_scalar(1.0, s.ad_value(2366)), s.ad_value(272)), s.ad_value(2364)), 2365);
        }

        if ((s.v[2601] != 0.0) && (s.v[2602] != 0.0)) {
            s.store_offset_ad(2368, A::div(A::mul(A::mul(A::offset(A::div_from_scalar(0.5, s.ad_value(2366)), (-1.0)), A::add(s.ad_value(2028), A::mul(s.ad_value(2364), A::sub(s.ad_value(808), s.ad_value(2027))))), s.ad_value(2029)), s.ad_value(2028)), 1.0);
        }

        if (s.v[2601] != 0.0) {
            s.store_scalar(2370, 1.0);
        }

        if (s.v[2601] != 0.0) {
            s.store_scalar(2371, 0.0);
        }

        s.v[2603] = if (s.v[271] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) {
            s.store_add_ad(2027, A::scale(s.ad_value(745), 0.5), A::mul(s.ad_value(1888), A::offset(A::scale(s.ad_value(1889), 0.7071067811865475), 1.0)));
        }

        if ((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) {
            s.store_div(2369, 1887, 2027);
        }

        s.v[2604] = if (((s.v[2369]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) && (s.v[2604] != 0.0)) {
            s.store_div_from_scalar_ad(2370, 1.0, A::offset(A::exp(A::neg(s.ad_value(2369))), 1.0));
        }

        s.v[2605] = if (s.v[2369] < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) && (!(s.v[2604] != 0.0))) && (s.v[2605] != 0.0)) {
            s.store_div_from_scalar_ad(2370, 1e-100, A::offset(A::mul(A::offset(s.ad_value(2369), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2369), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2369), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2606] = if (s.v[2369] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) && (s.v[2606] != 0.0)) {
            s.store_ln_ad(2028, A::offset(A::exp(s.ad_value(2369)), 1.0));
        }

        if (((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) && (!(s.v[2606] != 0.0))) {
            s.copy_ad(2028, 2369);
        }

        if ((s.v[2601] != 0.0) && (s.v[2603] != 0.0)) {
            s.store_mul(2371, 2027, 2028);
        }

        if (s.v[2601] != 0.0) {
            s.store_add_ad_lhs(2372, A::mul(s.ad_value(271), A::sub(s.ad_value(2370), s.ad_value(2368))), 2368);
        }

        if (s.v[2601] != 0.0) {
            s.store_add_ad_lhs(2373, A::mul(s.ad_value(271), A::sub(s.ad_value(2371), s.ad_value(2367))), 2367);
        }

        if (s.v[2601] != 0.0) {
            s.store_sub_ad(2374, A::sub(A::sub(s.ad_value(1887), A::mul(s.ad_value(1888), s.ad_value(1891))), s.ad_value(1903)), A::scale(s.ad_value(1893), 0.5));
        }

        if (s.v[2601] != 0.0) {
            s.store_sub_ad_lhs(2375, A::sub(s.ad_value(1887), s.ad_value(2374)), 1892);
        }

        if (s.v[2601] != 0.0) {
            s.store_sub_ad_lhs(2376, A::add(s.ad_value(1893), s.ad_value(2374)), 826);
        }

        if (s.v[2601] != 0.0) {
            s.store_sub_ad_lhs(2377, A::sub(s.ad_value(1887), s.ad_value(2376)), 1894);
        }

        s.v[2607] = if (s.v[831] > 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2601] != 0.0) && (s.v[2607] != 0.0)) {
            s.store_mul_ad_rhs(2378, 2372, A::add(A::mul(s.ad_value(269), s.ad_value(2376)), A::mul(s.ad_value(268), s.ad_value(2374))));
        }

        if ((s.v[2601] != 0.0) && (s.v[2607] != 0.0)) {
            s.store_mul_ad_rhs(2379, 268, A::sub(s.ad_value(2375), s.ad_value(2373)));
        }

        if ((s.v[2601] != 0.0) && (s.v[2607] != 0.0)) {
            s.store_mul_ad_rhs(2380, 269, A::sub(s.ad_value(2377), s.ad_value(2373)));
        }

        if ((s.v[2601] != 0.0) && (!(s.v[2607] != 0.0))) {
            s.store_mul_ad_rhs(2378, 2372, A::add(A::mul(s.ad_value(268), s.ad_value(2376)), A::mul(s.ad_value(269), s.ad_value(2374))));
        }

        if ((s.v[2601] != 0.0) && (!(s.v[2607] != 0.0))) {
            s.store_mul_ad_rhs(2379, 269, A::sub(s.ad_value(2375), s.ad_value(2373)));
        }

        if ((s.v[2601] != 0.0) && (!(s.v[2607] != 0.0))) {
            s.store_mul_ad_rhs(2380, 268, A::sub(s.ad_value(2377), s.ad_value(2373)));
        }

        if (s.v[2601] != 0.0) {
            s.store_add(851, 851, 2378);
        }

        if (s.v[2601] != 0.0) {
            s.store_add(853, 853, 2380);
        }

        if (s.v[2601] != 0.0) {
            s.store_sub_ad_lhs(852, A::sub(A::sub(s.ad_value(852), s.ad_value(2378)), s.ad_value(2380)), 2379);
        }

        s.store_mul(1910, 262, 1878);

        s.store_mul(1911, 263, 1879);

        s.v[2383] = 0.0;

        s.v[2381] = 0.0;

        s.v[2608] = if ((s.v[262] > 0.0) && (s.v[264] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2608] != 0.0) {
            s.store_mul_ad_rhs(2027, 266, A::add(A::scale(s.ad_value(1819), 0.5), s.ad_value(787)));
        }

        s.v[2609] = if (s.v[2027] < 230.25850929940458) { 1.0 } else { 0.0 };

        s.v[2610] = if (s.v[2027] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2608] != 0.0) && (s.v[2609] != 0.0)) && (s.v[2610] != 0.0)) {
            s.store_exp(2381, 2027);
        }

        if (((s.v[2608] != 0.0) && (s.v[2609] != 0.0)) && (!(s.v[2610] != 0.0))) {
            s.store_div_from_scalar_ad(2381, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2611] = if (s.v[2381] > 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2608] != 0.0) && (s.v[2609] != 0.0)) && (s.v[2611] != 0.0)) {
            s.store_ln_ad(2382, A::offset(s.ad_value(2381), 1.0));
        }

        if (((s.v[2608] != 0.0) && (s.v[2609] != 0.0)) && (s.v[2611] != 0.0)) {
            s.store_mul_ad_rhs(2028, 2382, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0))));
        }

        if (((s.v[2608] != 0.0) && (s.v[2609] != 0.0)) && (!(s.v[2611] != 0.0))) {
            s.copy_ad(2382, 2381);
        }

        if (((s.v[2608] != 0.0) && (s.v[2609] != 0.0)) && (!(s.v[2611] != 0.0))) {
            s.store_div_ad(2028, A::scale(s.ad_value(2382), 2.0), A::offset(s.ad_value(2382), 2.0));
        }

        if ((s.v[2608] != 0.0) && (!(s.v[2609] != 0.0))) {
            s.copy_ad(2382, 2027);
        }

        if ((s.v[2608] != 0.0) && (!(s.v[2609] != 0.0))) {
            s.store_mul_ad_rhs(2028, 2382, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2382), 1.0)), A::offset(s.ad_value(2382), 2.0))));
        }

        if (s.v[2608] != 0.0) {
            s.store_mul_ad_lhs(2383, A::scale(A::mul(A::div(A::scale(s.ad_value(264), (-2.0)), s.ad_value(266)), s.ad_value(262)), s.v[354]), 2028);
        }

        s.v[2386] = 0.0;

        s.v[2384] = 0.0;

        s.v[2612] = if ((s.v[263] > 0.0) && (s.v[265] > 0.0)) { 1.0 } else { 0.0 };

        if (s.v[2612] != 0.0) {
            s.store_mul_ad_rhs(2027, 266, A::add(A::scale(s.ad_value(1819), 0.5), s.ad_value(788)));
        }

        s.v[2613] = if (s.v[2027] < 230.25850929940458) { 1.0 } else { 0.0 };

        s.v[2614] = if (s.v[2027] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((s.v[2612] != 0.0) && (s.v[2613] != 0.0)) && (s.v[2614] != 0.0)) {
            s.store_exp(2384, 2027);
        }

        if (((s.v[2612] != 0.0) && (s.v[2613] != 0.0)) && (!(s.v[2614] != 0.0))) {
            s.store_div_from_scalar_ad(2384, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2027)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        s.v[2615] = if (s.v[2384] > 1e-10) { 1.0 } else { 0.0 };

        if (((s.v[2612] != 0.0) && (s.v[2613] != 0.0)) && (s.v[2615] != 0.0)) {
            s.store_ln_ad(2385, A::offset(s.ad_value(2384), 1.0));
        }

        if (((s.v[2612] != 0.0) && (s.v[2613] != 0.0)) && (s.v[2615] != 0.0)) {
            s.store_mul_ad_rhs(2028, 2385, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2385), 1.0)), A::offset(s.ad_value(2385), 2.0))));
        }

        if (((s.v[2612] != 0.0) && (s.v[2613] != 0.0)) && (!(s.v[2615] != 0.0))) {
            s.copy_ad(2385, 2384);
        }

        if (((s.v[2612] != 0.0) && (s.v[2613] != 0.0)) && (!(s.v[2615] != 0.0))) {
            s.store_div_ad(2028, A::scale(s.ad_value(2385), 2.0), A::offset(s.ad_value(2385), 2.0));
        }

        if ((s.v[2612] != 0.0) && (!(s.v[2613] != 0.0))) {
            s.copy_ad(2385, 2027);
        }

        if ((s.v[2612] != 0.0) && (!(s.v[2613] != 0.0))) {
            s.store_mul_ad_rhs(2028, 2385, A::sub_from_scalar(1.0, A::div(A::ln(A::offset(s.ad_value(2385), 1.0)), A::offset(s.ad_value(2385), 2.0))));
        }

        if (s.v[2612] != 0.0) {
            s.store_mul_ad_lhs(2386, A::scale(A::mul(A::div(A::scale(s.ad_value(265), (-2.0)), s.ad_value(266)), s.ad_value(263)), s.v[354]), 2028);
        }

        s.store_add(2387, 2383, 2386);

        s.store_add_ad_lhs(856, A::mul(s.ad_value(267), s.ad_value(829)), 2387);

        s.store_mul(854, 274, 834);

        s.store_mul(855, 275, 837);

        s.v[1938] = 0.0;

        s.v[1939] = 0.0;

        s.v[1940] = 0.0;

        s.v[1941] = 0.0;

        s.v[2616] = if (s.v[1] != 0.0) { 1.0 } else { 0.0 };

        s.v[2617] = if (s.v[1890] <= 0.0) { 1.0 } else { 0.0 };

        if ((s.v[2616] != 0.0) && (s.v[2617] != 0.0)) {
            s.store_scalar(1936, 0.5);
        }

        if ((s.v[2616] != 0.0) && (s.v[2617] != 0.0)) {
            s.store_scalar(1937, 1.0);
        }

        if ((s.v[2616] != 0.0) && (s.v[2617] != 0.0)) {
            s.copy_ad(1938, 1889);
        }

        if ((s.v[2616] != 0.0) && (!(s.v[2617] != 0.0))) {
            s.store_scale_ad(1936, A::offset(A::scale(A::div(s.ad_value(1893), s.ad_value(1909)), 0.25), 1.0), 0.5);
        }

        if ((s.v[2616] != 0.0) && (!(s.v[2617] != 0.0))) {
            s.store_div_ad_rhs(1937, 1935, A::sub(s.ad_value(1890), s.ad_value(1934)));
        }

        if ((s.v[2616] != 0.0) && (!(s.v[2617] != 0.0))) {
            s.store_div(1938, 1889, 1937);
        }

        if (s.v[2616] != 0.0) {
            s.store_square(1939, 1938);
        }

        if (s.v[2616] != 0.0) {
            s.store_offset_scaled(1940, 1938, 0.7071067811865475, 1.0);
        }

        if (s.v[2616] != 0.0) {
            s.store_scale(1941, 1940, 1e-5);
        }

        s.v[2618] = 0.0;

        s.v[2619] = 0.0;

        s.v[2620] = 0.0;

        s.v[2621] = 0.0;

        s.v[2622] = 0.0;

        s.v[2623] = 0.0;

        s.v[2624] = 0.0;

        s.v[2625] = 0.0;

        s.v[2626] = 0.0;

        s.v[2627] = 0.0;

        s.v[2628] = 0.0;

        s.v[2629] = 0.0;

        s.v[2630] = 0.0;

        s.v[2631] = 0.0;

        s.v[2632] = 0.0;

        s.v[2633] = 0.0;

        s.v[2634] = 0.0;

        s.v[2635] = 0.0;

        s.v[2636] = 0.0;

        s.v[2637] = 0.0;

        s.v[2638] = 0.0;

        s.v[2639] = 0.0;

        s.v[2640] = 0.0;

        s.v[2641] = 0.0;

        s.v[2642] = 0.0;

        s.v[2643] = 0.0;

        s.v[2644] = 0.0;

        s.v[2645] = 0.0;

        s.v[2646] = 0.0;

        s.v[2647] = 0.0;

        s.v[2648] = 0.0;

        s.v[2649] = 0.0;

        s.v[2650] = 0.0;

        s.v[2651] = 0.0;

        s.v[2652] = 0.0;

        s.v[2653] = 0.0;

        s.v[2654] = 0.0;

        s.v[2655] = 0.0;

        s.v[2656] = 0.0;

        s.v[2657] = 0.0;

        s.v[2658] = 0.0;

        s.v[2659] = 0.0;

        s.v[2660] = 0.0;

        s.v[2661] = 0.0;

        s.v[2662] = 0.0;

        s.v[2663] = 0.0;

        s.v[2664] = 0.0;

        s.v[848] = 0.0;

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
        s.v[1912] = 0.0;

        s.v[1913] = 0.0;

        s.v[1914] = 0.0;

        s.v[849] = 0.0;

        s.v[1915] = 0.0;

        s.v[1916] = 0.0;

        s.v[1917] = 0.0;

        s.v[857] = 0.0;

        s.v[1918] = 0.0;

        s.v[1919] = 0.0;

        s.v[1920] = 0.0;

        s.v[858] = 0.0;

        s.v[1921] = 0.0;

        s.v[1922] = 0.0;

        s.v[1923] = 0.0;

        s.v[2665] = if (p.p43 > 0.0) { 1.0 } else { 0.0 };

        s.v[2666] = if (s.v[474] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scale(496, 832, (s.v[371] * s.v[668]));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            let assign55740_ad_e70149: A = {
                if (s.v[496] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0))
                } else {
                    {
                        if (s.v[496] > s.v[660]) {
                            A::mul(s.ad_value(661), A::offset(A::sub(s.ad_value(496), s.ad_value(660)), 1.0))
                        } else {
                            A::exp(s.ad_value(496))
                        }
                    }
                }
            };
            s.store_ad(497, &assign55740_ad_e70149);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_mul_ad_rhs(502, 667, A::offset(s.ad_value(497), (-1.0)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_mul_ad_lhs(496, A::scale(s.ad_value(832), s.v[371]), 670);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            let assign55770_ad_e70200: A = {
                if (s.v[496] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0))
                } else {
                    {
                        if (s.v[496] > s.v[662]) {
                            A::mul(s.ad_value(663), A::offset(A::sub(s.ad_value(496), s.ad_value(662)), 1.0))
                        } else {
                            A::exp(s.ad_value(496))
                        }
                    }
                }
            };
            s.store_ad(497, &assign55770_ad_e70200);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_mul_ad_rhs(503, 669, A::offset(s.ad_value(497), (-1.0)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scalar(504, 0.0);
        }

        s.v[2667] = if (s.v[666] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2667] != 0.0)) {
            s.store_mul_ad_rhs(504, 832, A::add(s.ad_value(671), A::mul(s.ad_value(832), s.ad_value(672))));
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (!(s.v[2667] != 0.0))) {
            s.store_mul_ad_lhs(496, A::scale(A::neg(s.ad_value(832)), s.v[371]), 672);
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (!(s.v[2667] != 0.0))) {
            let assign55830_ad_e70281: A = {
                if (s.v[496] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0))
                } else {
                    {
                        if (s.v[496] > s.v[664]) {
                            A::mul(s.ad_value(665), A::offset(A::sub(s.ad_value(496), s.ad_value(664)), 1.0))
                        } else {
                            A::exp(s.ad_value(496))
                        }
                    }
                }
            };
            s.store_ad(497, &assign55830_ad_e70281);
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (!(s.v[2667] != 0.0))) {
            s.store_mul_ad(504, A::neg(s.ad_value(671)), A::offset(s.ad_value(497), (-1.0)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_add_ad_lhs(848, A::add(s.ad_value(502), s.ad_value(503)), 504);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scale(496, 833, (s.v[371] * s.v[695]));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            let assign55870_ad_e70346: A = {
                if (s.v[496] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0))
                } else {
                    {
                        if (s.v[496] > s.v[687]) {
                            A::mul(s.ad_value(688), A::offset(A::sub(s.ad_value(496), s.ad_value(687)), 1.0))
                        } else {
                            A::exp(s.ad_value(496))
                        }
                    }
                }
            };
            s.store_ad(497, &assign55870_ad_e70346);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_mul_ad_rhs(502, 694, A::offset(s.ad_value(497), (-1.0)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_mul_ad_lhs(496, A::scale(s.ad_value(833), s.v[371]), 697);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            let assign55900_ad_e70397: A = {
                if (s.v[496] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0))
                } else {
                    {
                        if (s.v[496] > s.v[689]) {
                            A::mul(s.ad_value(690), A::offset(A::sub(s.ad_value(496), s.ad_value(689)), 1.0))
                        } else {
                            A::exp(s.ad_value(496))
                        }
                    }
                }
            };
            s.store_ad(497, &assign55900_ad_e70397);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_mul_ad_rhs(503, 696, A::offset(s.ad_value(497), (-1.0)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scalar(504, 0.0);
        }

        s.v[2668] = if (s.v[693] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2668] != 0.0)) {
            s.store_mul_ad_rhs(504, 833, A::add(s.ad_value(698), A::mul(s.ad_value(833), s.ad_value(699))));
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (!(s.v[2668] != 0.0))) {
            s.store_mul_ad_lhs(496, A::scale(A::neg(s.ad_value(833)), s.v[371]), 699);
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (!(s.v[2668] != 0.0))) {
            let assign55960_ad_e70478: A = {
                if (s.v[496] < (-230.25850929940458)) {
                    A::div_from_scalar(1e-100, A::offset(A::sub_from_scalar((-230.25850929940458), s.ad_value(496)), 1.0))
                } else {
                    {
                        if (s.v[496] > s.v[691]) {
                            A::mul(s.ad_value(692), A::offset(A::sub(s.ad_value(496), s.ad_value(691)), 1.0))
                        } else {
                            A::exp(s.ad_value(496))
                        }
                    }
                }
            };
            s.store_ad(497, &assign55960_ad_e70478);
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (!(s.v[2668] != 0.0))) {
            s.store_mul_ad(504, A::neg(s.ad_value(698)), A::offset(s.ad_value(497), (-1.0)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_add_ad_lhs(849, A::add(s.ad_value(502), s.ad_value(503)), 504);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scalar(2669, 0.0);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scalar(2670, 0.0);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(657), 4.0), 657);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_div(2622, 657, 658);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_add_ad_rhs(2623, 832, A::mul(s.ad_value(657), s.ad_value(2622)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_add(2624, 658, 2623);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_sub(2625, 658, 2623);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scale_ad(2670, A::div(A::mul(s.ad_value(832), s.ad_value(658)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2671] = if (s.v[651] > 0.5) { 1.0 } else { 0.0 };

        s.v[2672] = if (s.v[408] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2671] != 0.0)) && (s.v[2672] != 0.0)) {
            s.store_sqrt_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[405])));
        }

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2671] != 0.0)) && (!(s.v[2672] != 0.0))) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[405])), s.v[408]);
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2671] != 0.0)) {
            s.store_add_ad(1918, A::scale(A::sub_from_scalar(1.0, s.ad_value(2669)), s.v[417]), A::scale(A::sub(s.ad_value(832), s.ad_value(2670)), s.v[420]));
        }

        s.v[2673] = if (s.v[652] > 0.5) { 1.0 } else { 0.0 };

        s.v[2674] = if (s.v[409] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2673] != 0.0)) && (s.v[2674] != 0.0)) {
            s.store_sqrt_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[406])));
        }

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2673] != 0.0)) && (!(s.v[2674] != 0.0))) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[406])), s.v[409]);
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2673] != 0.0)) {
            s.store_add_ad(1919, A::scale(A::sub_from_scalar(1.0, s.ad_value(2669)), s.v[418]), A::scale(A::sub(s.ad_value(832), s.ad_value(2670)), s.v[421]));
        }

        s.v[2675] = if (s.v[653] > 0.5) { 1.0 } else { 0.0 };

        s.v[2676] = if (s.v[410] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2675] != 0.0)) && (s.v[2676] != 0.0)) {
            s.store_sqrt_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[407])));
        }

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2675] != 0.0)) && (!(s.v[2676] != 0.0))) {
            s.store_powf_ad(2669, A::sub_from_scalar(1.0, A::scale(s.ad_value(2670), s.v[407])), s.v[410]);
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2675] != 0.0)) {
            s.store_add_ad(1920, A::scale(A::sub_from_scalar(1.0, s.ad_value(2669)), s.v[419]), A::scale(A::sub(s.ad_value(832), s.ad_value(2670)), s.v[422]));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scalar(2669, 0.0);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scalar(2670, 0.0);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(684), 4.0), 684);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_div(2622, 684, 685);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_add_ad_rhs(2623, 833, A::mul(s.ad_value(684), s.ad_value(2622)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_add(2624, 685, 2623);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_sub(2625, 685, 2623);
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if ((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) {
            s.store_scale_ad(2670, A::div(A::mul(s.ad_value(833), s.ad_value(685)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2677] = if (s.v[678] > 0.5) { 1.0 } else { 0.0 };

        s.v[2678] = if (s.v[575] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2677] != 0.0)) && (s.v[2678] != 0.0)) {
            s.store_sqrt_ad(2669, A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(572))));
        }

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2677] != 0.0)) && (!(s.v[2678] != 0.0))) {
            s.store_ad(2669, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(572))), s.ad_value(575)));
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2677] != 0.0)) {
            s.store_add_ad(1921, A::mul(s.ad_value(584), A::sub_from_scalar(1.0, s.ad_value(2669))), A::mul(s.ad_value(587), A::sub(s.ad_value(833), s.ad_value(2670))));
        }

        s.v[2679] = if (s.v[679] > 0.5) { 1.0 } else { 0.0 };

        s.v[2680] = if (s.v[576] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2679] != 0.0)) && (s.v[2680] != 0.0)) {
            s.store_sqrt_ad(2669, A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(573))));
        }

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2679] != 0.0)) && (!(s.v[2680] != 0.0))) {
            s.store_ad(2669, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(573))), s.ad_value(576)));
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2679] != 0.0)) {
            s.store_add_ad(1922, A::mul(s.ad_value(585), A::sub_from_scalar(1.0, s.ad_value(2669))), A::mul(s.ad_value(588), A::sub(s.ad_value(833), s.ad_value(2670))));
        }

        s.v[2681] = if (s.v[680] > 0.5) { 1.0 } else { 0.0 };

        s.v[2682] = if (s.v[577] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2681] != 0.0)) && (s.v[2682] != 0.0)) {
            s.store_sqrt_ad(2669, A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(574))));
        }

        if ((((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2681] != 0.0)) && (!(s.v[2682] != 0.0))) {
            s.store_ad(2669, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2670), s.ad_value(574))), s.ad_value(577)));
        }

        if (((s.v[2665] != 0.0) && (s.v[2666] != 0.0)) && (s.v[2681] != 0.0)) {
            s.store_add_ad(1923, A::mul(s.ad_value(586), A::sub_from_scalar(1.0, s.ad_value(2669))), A::mul(s.ad_value(589), A::sub(s.ad_value(833), s.ad_value(2670))));
        }

        s.v[2683] = if (p.p872 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2683] != 0.0)) {
            s.store_scale_ad(642, A::offset(A::powf(A::scale(A::add(A::add(s.ad_value(825), s.ad_value(827)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001)))), 0.5), p.p873), (-(((0.5 * 0.001)) as f64).powf(p.p873))), p.p872);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2683] != 0.0)) {
            s.store_offset(640, 642, p.p862);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2683] != 0.0)) {
            s.store_div_from_scalar(450, 1.0, 640);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2683] != 0.0)) {
            s.store_div_from_scalar_ad(453, s.v[453], A::offset(A::scale(s.ad_value(642), 1.0 / (p.p862)), 1.0));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2683] != 0.0))) {
            s.store_scalar(640, p.p862);
        }

        s.v[2684] = if (p.p874 > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2684] != 0.0)) {
            s.store_scale_ad(644, A::offset(A::powf(A::scale(A::add(A::add(s.ad_value(825), s.ad_value(827)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001)))), 0.5), p.p875), (-(((0.5 * 0.001)) as f64).powf(p.p875))), p.p874);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2684] != 0.0)) {
            s.store_mul_ad_rhs(443, 443, A::offset(s.ad_value(644), 1.0));
        }

        if ((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) {
            s.store_scalar(2634, 0.0);
        }

        if ((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) {
            s.store_scalar(2631, 0.0);
        }

        s.v[2685] = if !(((s.v[646] == 0.0) && (s.v[647] == 0.0)) && (s.v[648] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(657), 4.0), 657);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_div(2622, 657, 658);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_add_ad_rhs(2623, 832, A::mul(s.ad_value(657), s.ad_value(2622)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_add(2624, 658, 2623);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_sub(2625, 658, 2623);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_scale_ad(2628, A::div(A::mul(s.ad_value(832), s.ad_value(658)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2686] = if (s.v[832] < s.v[654]) { 1.0 } else { 0.0 };

        s.v[2687] = if (((((-0.5) * (s.v[832] * s.v[371]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (s.v[2686] != 0.0)) && (s.v[2687] != 0.0)) {
            s.store_exp_ad(2629, A::scale(s.ad_value(832), (s.v[371] * (-0.5))));
        }

        s.v[2688] = if (((-0.5) * (s.v[832] * s.v[371])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (s.v[2686] != 0.0)) && (!(s.v[2687] != 0.0))) && (s.v[2688] != 0.0)) {
            let assign56700_ad_e71361: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(832), (s.v[371] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(832), (s.v[371] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(832), (s.v[371] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2629, &assign56700_ad_e71361);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (s.v[2686] != 0.0)) && (!(s.v[2687] != 0.0))) && (!(s.v[2688] != 0.0))) {
            s.store_scale_ad(2629, A::offset(A::mul(A::offset(A::scale(s.ad_value(832), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(832), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(832), (s.v[371] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (s.v[2686] != 0.0)) {
            s.store_div_from_scalar(2630, 1.0, 2629);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (s.v[2686] != 0.0)) {
            s.store_square(2627, 2630);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (!(s.v[2686] != 0.0))) {
            s.store_mul_ad_lhs(2627, A::offset(A::scale(A::sub(s.ad_value(832), s.ad_value(654)), s.v[371]), 1.0), 655);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (!(s.v[2686] != 0.0))) {
            s.store_sqrt(2630, 2627);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (!(s.v[2686] != 0.0))) {
            s.store_div_from_scalar(2629, 1.0, 2630);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_offset(2627, 2627, (-1.0));
        }

        s.v[2689] = if (s.v[832] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (s.v[2689] != 0.0)) {
            s.store_scale_ad(2631, A::ln(A::add(A::offset(s.ad_value(2629), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2629), 1.0), A::offset(s.ad_value(2629), 3.0))))), (s.v[370] * 2.0));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) && (!(s.v[2689] != 0.0))) {
            s.store_sub_ad_lhs(2631, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2630), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2630), 1.0), A::offset(A::scale(s.ad_value(2630), 3.0), 1.0))))), (s.v[370] * 2.0)), 832);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_sub(2632, 656, 2631);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_scale_ad(2633, A::sub(A::add(s.ad_value(832), s.ad_value(2632)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(832), s.ad_value(2632)), A::sub(s.ad_value(832), s.ad_value(2632))), ((4.0 * s.v[370]) * s.v[370])))), 0.5);
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
        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_scale_ad(2634, A::sub(A::add(s.ad_value(832), s.ad_value(659)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(832), s.ad_value(659)), A::sub(s.ad_value(832), s.ad_value(659))), ((4.0 * s.v[368]) * s.v[368])))), 0.5);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2685] != 0.0)) {
            s.store_scale_ad(2635, A::sub(s.ad_value(832), A::sqrt(A::offset(A::mul(s.ad_value(832), s.ad_value(832)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[2690] = if (s.v[646] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2690] != 0.0)) {
            s.store_scalar(1912, 0.0);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2690] != 0.0)) {
            s.store_scalar(1918, 0.0);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) {
            s.store_scale(2637, 2627, s.v[387]);
        }

        s.v[2691] = if ((p.p840 == 0.0) && (p.p845 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2691] != 0.0)) {
            s.store_scalar(2638, 0.0);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) {
            s.store_sub_from_scalar(2639, s.v[393], 2633);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) {
            s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));
        }

        s.v[2692] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) && (s.v[2692] != 0.0)) {
            s.store_scalar(2641, 0.0);
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) && (!(s.v[2692] != 0.0))) {
            s.store_scale_ad(2641, A::add(A::div(A::mul(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640))), A::sub_from_scalar(1.0, s.ad_value(2640))), s.ad_value(2640)), (1.0 - (2.0 * p.p831)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) {
            s.store_add(2642, 2640, 2641);
        }

        s.v[2693] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) && (s.v[2693] != 0.0)) {
            s.store_sqrt_ad(2636, A::scale(s.ad_value(2639), s.v[429]));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) && (!(s.v[2693] != 0.0))) {
            s.store_powf_ad(2636, A::scale(s.ad_value(2639), s.v[429]), p.p831);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) {
            s.store_scale(2643, 2636, s.v[423]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) {
            s.store_scale_ad(2644, A::mul(A::offset(s.ad_value(2630), (-1.0)), s.ad_value(2643)), s.v[384]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2691] != 0.0))) {
            s.store_scaled_mul(2638, 2644, 2642, p.p840);
        }

        s.v[2694] = if (p.p845 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2694] != 0.0)) {
            s.store_scalar(2645, 0.0);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_scale_ad(2646, A::div(A::scale(s.ad_value(2643), s.v[408]), s.ad_value(2639)), s.v[438]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[435]), 2646);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_square(2648, 2647);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_sqrt_ad(2649, A::div(A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_sqrt(2650, 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_mul(2651, 2649, 2650);
        }

        s.v[2695] = if (((-p.p831) * s.v[411]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2695] != 0.0)) {
            s.store_div_from_scalar_ad(2652, 1.0, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2695] != 0.0))) {
            s.store_powf_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), ((-p.p831) * s.v[411]));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_div_ad(2653, A::mul(s.ad_value(2642), s.ad_value(2652)), A::add(s.ad_value(2642), s.ad_value(2652)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_sqrt_ad(2654, A::scale(A::div(s.ad_value(2646), s.ad_value(2650)), 0.375));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_add_ad(2656, A::sub(A::mul(A::scale(s.ad_value(2647), s.v[435]), s.ad_value(2650)), A::scale(s.ad_value(2649), s.v[435])), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_mul_ad_lhs(2657, A::offset(s.ad_value(2655), (-1.0)), 2654);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_square(2618, 2657);
        }

        s.v[2696] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2696] != 0.0)) {
            s.store_div_from_scalar_ad(2619, 1.0, A::offset(A::scale(s.ad_value(2657), s.v[372]), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2696] != 0.0))) {
            s.store_div_from_scalar_ad(2619, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2657), s.v[372])));
        }

        s.v[2697] = if (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2697] != 0.0)) {
            s.store_exp_ad(2636, A::sub(s.ad_value(2656), s.ad_value(2618)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2697] != 0.0))) {
            let assign57250_ad_e72311: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2636, &assign57250_ad_e72311);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_mul_ad_lhs(2620, A::add(A::add(A::scale(s.ad_value(2619), 0.29214664), A::scale(A::square(s.ad_value(2619)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(2619)), s.ad_value(2619)), s.v[374])), 2636);
        }

        s.v[2698] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (s.v[2698] != 0.0)) {
            s.copy_ad(2658, 2620);
        }

        s.v[2699] = if (s.v[2656] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (s.v[2699] != 0.0)) {
            s.store_exp(2636, 2656);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) && (!(s.v[2699] != 0.0))) {
            s.store_div_from_scalar_ad(2636, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) && (!(s.v[2698] != 0.0))) {
            s.store_sub_ad_lhs(2658, A::scale(s.ad_value(2636), 2.0), 2620);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_scale_ad(2659, A::div(A::scale(s.ad_value(2658), s.v[435]), s.ad_value(2654)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2694] != 0.0))) {
            s.store_scale_ad(2645, A::mul(A::mul(s.ad_value(2644), s.ad_value(2659)), s.ad_value(2653)), p.p845);
        }

        s.v[2700] = if (p.p851 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2700] != 0.0)) {
            s.store_scalar(2660, 0.0);
        }

        s.v[2701] = if (p.p831 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) && (s.v[2701] != 0.0)) {
            s.store_sqrt_ad(2636, A::scale(A::sub_from_scalar(p.p828, s.ad_value(2634)), s.v[429]));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) && (!(s.v[2701] != 0.0))) {
            s.store_powf_ad(2636, A::scale(A::sub_from_scalar(p.p828, s.ad_value(2634)), s.v[429]), p.p831);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) {
            s.store_scale_ad(2661, A::div(A::scale(A::sub_from_scalar(p.p828, s.ad_value(2634)), s.v[426]), s.ad_value(2636)), s.v[411]);
        }

        s.v[2702] = if (((((-s.v[441]) / s.v[2661])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) && (s.v[2702] != 0.0)) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(441)), s.ad_value(2661)));
        }

        s.v[2703] = if (((-s.v[441]) / s.v[2661]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) && (!(s.v[2702] != 0.0))) && (s.v[2703] != 0.0)) {
            let assign57440_ad_e72651: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(441)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign57440_ad_e72651);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) && (!(s.v[2702] != 0.0))) && (!(s.v[2703] != 0.0))) {
            let assign57450_ad_e72702: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(441)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2636, &assign57450_ad_e72702);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2700] != 0.0))) {
            s.store_scale_ad(2660, A::mul(A::mul(A::mul(s.ad_value(832), s.ad_value(2661)), s.ad_value(2661)), s.ad_value(2636)), p.p851);
        }

        s.v[2704] = if (p.p860 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2704] != 0.0)) {
            s.store_scalar(2662, 1.0);
        }

        s.v[2705] = if (s.v[2635] > ((-s.v[444]) * p.p860)) { 1.0 } else { 0.0 };

        s.v[2706] = if (p.p863 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2704] != 0.0))) && (s.v[2705] != 0.0)) && (s.v[2706] != 0.0)) {
            s.store_mul_ad(2636, A::mul(A::mul(A::scale(s.ad_value(2635), s.v[448]), A::scale(s.ad_value(2635), s.v[448])), A::scale(s.ad_value(2635), s.v[448])), A::scale(s.ad_value(2635), s.v[448]));
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2704] != 0.0))) && (s.v[2705] != 0.0)) && (!(s.v[2706] != 0.0))) {
            s.store_powf_ad(2636, A::abs(A::scale(s.ad_value(2635), s.v[448])), p.p863);
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2704] != 0.0))) && (s.v[2705] != 0.0)) {
            s.store_div_from_scalar_ad(2662, 1.0, A::sub_from_scalar(1.0, s.ad_value(2636)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2704] != 0.0))) && (!(s.v[2705] != 0.0))) {
            s.store_offset_ad(2662, A::scale(A::offset(s.ad_value(2635), (s.v[444] * p.p860)), s.v[451]), s.v[445]);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) {
            s.store_mul_ad_lhs(1912, A::scale(A::add(A::add(A::add(s.ad_value(2637), s.ad_value(2638)), s.ad_value(2645)), s.ad_value(2660)), p.p29), 2662);
        }

        s.v[2707] = if (s.v[408] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (s.v[2707] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[405])));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) && (!(s.v[2707] != 0.0))) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[405])), s.v[408]);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2690] != 0.0))) {
            s.store_scale_ad(1918, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2636)), s.v[417]), A::scale(A::sub(s.ad_value(832), s.ad_value(2628)), s.v[420])), p.p30);
        }

        s.v[2708] = if (s.v[647] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2708] != 0.0)) {
            s.store_scalar(1913, 0.0);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2708] != 0.0)) {
            s.store_scalar(1919, 0.0);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) {
            s.store_scale(2637, 2627, s.v[388]);
        }

        s.v[2709] = if ((p.p841 == 0.0) && (p.p846 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (s.v[2709] != 0.0)) {
            s.store_scalar(2638, 0.0);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) {
            s.store_sub_from_scalar(2639, s.v[394], 2633);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) {
            s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));
        }

        s.v[2710] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) && (s.v[2710] != 0.0)) {
            s.store_scalar(2641, 0.0);
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) && (!(s.v[2710] != 0.0))) {
            s.store_scale_ad(2641, A::add(A::div(A::mul(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640))), A::sub_from_scalar(1.0, s.ad_value(2640))), s.ad_value(2640)), (1.0 - (2.0 * p.p832)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) {
            s.store_add(2642, 2640, 2641);
        }

        s.v[2711] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) && (s.v[2711] != 0.0)) {
            s.store_sqrt_ad(2636, A::scale(s.ad_value(2639), s.v[430]));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) && (!(s.v[2711] != 0.0))) {
            s.store_powf_ad(2636, A::scale(s.ad_value(2639), s.v[430]), p.p832);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) {
            s.store_scale(2643, 2636, s.v[424]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) {
            s.store_scale_ad(2644, A::mul(A::offset(s.ad_value(2630), (-1.0)), s.ad_value(2643)), s.v[385]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2709] != 0.0))) {
            s.store_scaled_mul(2638, 2644, 2642, p.p841);
        }

        s.v[2712] = if (p.p846 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (s.v[2712] != 0.0)) {
            s.store_scalar(2645, 0.0);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_scale_ad(2646, A::div(A::scale(s.ad_value(2643), s.v[409]), s.ad_value(2639)), s.v[439]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[436]), 2646);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_square(2648, 2647);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_sqrt_ad(2649, A::div(A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_sqrt(2650, 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_mul(2651, 2649, 2650);
        }

        s.v[2713] = if (((-p.p832) * s.v[412]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (s.v[2713] != 0.0)) {
            s.store_div_from_scalar_ad(2652, 1.0, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (!(s.v[2713] != 0.0))) {
            s.store_powf_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), ((-p.p832) * s.v[412]));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_div_ad(2653, A::mul(s.ad_value(2642), s.ad_value(2652)), A::add(s.ad_value(2642), s.ad_value(2652)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_sqrt_ad(2654, A::scale(A::div(s.ad_value(2646), s.ad_value(2650)), 0.375));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_add_ad(2656, A::sub(A::mul(A::scale(s.ad_value(2647), s.v[436]), s.ad_value(2650)), A::scale(s.ad_value(2649), s.v[436])), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_mul_ad_lhs(2657, A::offset(s.ad_value(2655), (-1.0)), 2654);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_square(2618, 2657);
        }

        s.v[2714] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (s.v[2714] != 0.0)) {
            s.store_div_from_scalar_ad(2619, 1.0, A::offset(A::scale(s.ad_value(2657), s.v[372]), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (!(s.v[2714] != 0.0))) {
            s.store_div_from_scalar_ad(2619, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2657), s.v[372])));
        }

        s.v[2715] = if (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (s.v[2715] != 0.0)) {
            s.store_exp_ad(2636, A::sub(s.ad_value(2656), s.ad_value(2618)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (!(s.v[2715] != 0.0))) {
            let assign58000_ad_e73577: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2636, &assign58000_ad_e73577);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_mul_ad_lhs(2620, A::add(A::add(A::scale(s.ad_value(2619), 0.29214664), A::scale(A::square(s.ad_value(2619)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(2619)), s.ad_value(2619)), s.v[374])), 2636);
        }

        s.v[2716] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (s.v[2716] != 0.0)) {
            s.copy_ad(2658, 2620);
        }

        s.v[2717] = if (s.v[2656] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (!(s.v[2716] != 0.0))) && (s.v[2717] != 0.0)) {
            s.store_exp(2636, 2656);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (!(s.v[2716] != 0.0))) && (!(s.v[2717] != 0.0))) {
            s.store_div_from_scalar_ad(2636, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) && (!(s.v[2716] != 0.0))) {
            s.store_sub_ad_lhs(2658, A::scale(s.ad_value(2636), 2.0), 2620);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_scale_ad(2659, A::div(A::scale(s.ad_value(2658), s.v[436]), s.ad_value(2654)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2712] != 0.0))) {
            s.store_scale_ad(2645, A::mul(A::mul(s.ad_value(2644), s.ad_value(2659)), s.ad_value(2653)), p.p846);
        }

        s.v[2718] = if (p.p852 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (s.v[2718] != 0.0)) {
            s.store_scalar(2660, 0.0);
        }

        s.v[2719] = if (p.p832 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) && (s.v[2719] != 0.0)) {
            s.store_sqrt_ad(2636, A::scale(A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[430]));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) && (!(s.v[2719] != 0.0))) {
            s.store_powf_ad(2636, A::scale(A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[430]), p.p832);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) {
            s.store_scale_ad(2661, A::div(A::scale(A::sub_from_scalar(p.p829, s.ad_value(2634)), s.v[427]), s.ad_value(2636)), s.v[412]);
        }

        s.v[2720] = if (((((-s.v[442]) / s.v[2661])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) && (s.v[2720] != 0.0)) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(442)), s.ad_value(2661)));
        }

        s.v[2721] = if (((-s.v[442]) / s.v[2661]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) && (!(s.v[2720] != 0.0))) && (s.v[2721] != 0.0)) {
            let assign58190_ad_e73917: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(442)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign58190_ad_e73917);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) && (!(s.v[2720] != 0.0))) && (!(s.v[2721] != 0.0))) {
            let assign58200_ad_e73968: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(442)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2636, &assign58200_ad_e73968);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2718] != 0.0))) {
            s.store_scale_ad(2660, A::mul(A::mul(A::mul(s.ad_value(832), s.ad_value(2661)), s.ad_value(2661)), s.ad_value(2636)), p.p852);
        }

        s.v[2722] = if (p.p861 > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (s.v[2722] != 0.0)) {
            s.store_scalar(2662, 1.0);
        }

        s.v[2723] = if (s.v[2635] > ((-s.v[444]) * p.p861)) { 1.0 } else { 0.0 };

        s.v[2724] = if (p.p864 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2722] != 0.0))) && (s.v[2723] != 0.0)) && (s.v[2724] != 0.0)) {
            s.store_mul_ad(2636, A::mul(A::mul(A::scale(s.ad_value(2635), s.v[449]), A::scale(s.ad_value(2635), s.v[449])), A::scale(s.ad_value(2635), s.v[449])), A::scale(s.ad_value(2635), s.v[449]));
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
        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2722] != 0.0))) && (s.v[2723] != 0.0)) && (!(s.v[2724] != 0.0))) {
            s.store_powf_ad(2636, A::abs(A::scale(s.ad_value(2635), s.v[449])), p.p864);
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2722] != 0.0))) && (s.v[2723] != 0.0)) {
            s.store_div_from_scalar_ad(2662, 1.0, A::sub_from_scalar(1.0, s.ad_value(2636)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2722] != 0.0))) && (!(s.v[2723] != 0.0))) {
            s.store_offset_ad(2662, A::scale(A::offset(s.ad_value(2635), (s.v[444] * p.p861)), s.v[452]), s.v[446]);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) {
            s.store_mul_ad_lhs(1913, A::scale(A::add(A::add(A::add(s.ad_value(2637), s.ad_value(2638)), s.ad_value(2645)), s.ad_value(2660)), p.p29), 2662);
        }

        s.v[2725] = if (s.v[409] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (s.v[2725] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[406])));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) && (!(s.v[2725] != 0.0))) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[406])), s.v[409]);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2708] != 0.0))) {
            s.store_scale_ad(1919, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2636)), s.v[418]), A::scale(A::sub(s.ad_value(832), s.ad_value(2628)), s.v[421])), p.p30);
        }

        s.v[2726] = if (s.v[648] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2726] != 0.0)) {
            s.store_scalar(1914, 0.0);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2726] != 0.0)) {
            s.store_scalar(1920, 0.0);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) {
            s.store_scale(2637, 2627, s.v[389]);
        }

        s.v[2727] = if ((p.p842 == 0.0) && (p.p847 == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2727] != 0.0)) {
            s.store_scalar(2638, 0.0);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) {
            s.store_sub_from_scalar(2639, s.v[395], 2633);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) {
            s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));
        }

        s.v[2728] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) && (s.v[2728] != 0.0)) {
            s.store_scalar(2641, 0.0);
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) && (!(s.v[2728] != 0.0))) {
            s.store_scale_ad(2641, A::add(A::div(A::mul(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640))), A::sub_from_scalar(1.0, s.ad_value(2640))), s.ad_value(2640)), (1.0 - (2.0 * p.p833)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) {
            s.store_add(2642, 2640, 2641);
        }

        s.v[2729] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) && (s.v[2729] != 0.0)) {
            s.store_sqrt_ad(2636, A::scale(s.ad_value(2639), s.v[431]));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) && (!(s.v[2729] != 0.0))) {
            s.store_powf_ad(2636, A::scale(s.ad_value(2639), s.v[431]), p.p833);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) {
            s.store_scale(2643, 2636, s.v[425]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) {
            s.store_scale_ad(2644, A::mul(A::offset(s.ad_value(2630), (-1.0)), s.ad_value(2643)), s.v[386]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2727] != 0.0))) {
            s.store_scaled_mul(2638, 2644, 2642, p.p842);
        }

        s.v[2730] = if (p.p847 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2730] != 0.0)) {
            s.store_scalar(2645, 0.0);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_scale_ad(2646, A::div(A::scale(s.ad_value(2643), s.v[410]), s.ad_value(2639)), s.v[440]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_div_from_scalar(2647, (0.666666666666667 * s.v[437]), 2646);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_square(2648, 2647);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_sqrt_ad(2649, A::div(A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_sqrt(2650, 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_mul(2651, 2649, 2650);
        }

        s.v[2731] = if (((-p.p833) * s.v[413]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (s.v[2731] != 0.0)) {
            s.store_div_from_scalar_ad(2652, 1.0, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (!(s.v[2731] != 0.0))) {
            s.store_powf_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), ((-p.p833) * s.v[413]));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_div_ad(2653, A::mul(s.ad_value(2642), s.ad_value(2652)), A::add(s.ad_value(2642), s.ad_value(2652)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_sqrt_ad(2654, A::scale(A::div(s.ad_value(2646), s.ad_value(2650)), 0.375));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_add_ad(2656, A::sub(A::mul(A::scale(s.ad_value(2647), s.v[437]), s.ad_value(2650)), A::scale(s.ad_value(2649), s.v[437])), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_mul_ad_lhs(2657, A::offset(s.ad_value(2655), (-1.0)), 2654);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_square(2618, 2657);
        }

        s.v[2732] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (s.v[2732] != 0.0)) {
            s.store_div_from_scalar_ad(2619, 1.0, A::offset(A::scale(s.ad_value(2657), s.v[372]), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (!(s.v[2732] != 0.0))) {
            s.store_div_from_scalar_ad(2619, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2657), s.v[372])));
        }

        s.v[2733] = if (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (s.v[2733] != 0.0)) {
            s.store_exp_ad(2636, A::sub(s.ad_value(2656), s.ad_value(2618)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (!(s.v[2733] != 0.0))) {
            let assign58750_ad_e74843: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2636, &assign58750_ad_e74843);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_mul_ad_lhs(2620, A::add(A::add(A::scale(s.ad_value(2619), 0.29214664), A::scale(A::square(s.ad_value(2619)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(2619)), s.ad_value(2619)), s.v[374])), 2636);
        }

        s.v[2734] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (s.v[2734] != 0.0)) {
            s.copy_ad(2658, 2620);
        }

        s.v[2735] = if (s.v[2656] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (!(s.v[2734] != 0.0))) && (s.v[2735] != 0.0)) {
            s.store_exp(2636, 2656);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (!(s.v[2734] != 0.0))) && (!(s.v[2735] != 0.0))) {
            s.store_div_from_scalar_ad(2636, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) && (!(s.v[2734] != 0.0))) {
            s.store_sub_ad_lhs(2658, A::scale(s.ad_value(2636), 2.0), 2620);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_scale_ad(2659, A::div(A::scale(s.ad_value(2658), s.v[437]), s.ad_value(2654)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2730] != 0.0))) {
            s.store_scale_ad(2645, A::mul(A::mul(s.ad_value(2644), s.ad_value(2659)), s.ad_value(2653)), p.p847);
        }

        s.v[2736] = if (p.p853 == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2736] != 0.0)) {
            s.store_scalar(2660, 0.0);
        }

        s.v[2737] = if (p.p833 == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) && (s.v[2737] != 0.0)) {
            s.store_sqrt_ad(2636, A::scale(A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[431]));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) && (!(s.v[2737] != 0.0))) {
            s.store_powf_ad(2636, A::scale(A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[431]), p.p833);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) {
            s.store_scale_ad(2661, A::div(A::scale(A::sub_from_scalar(p.p830, s.ad_value(2634)), s.v[428]), s.ad_value(2636)), s.v[413]);
        }

        s.v[2738] = if (((((-s.v[443]) / s.v[2661])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) && (s.v[2738] != 0.0)) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(443)), s.ad_value(2661)));
        }

        s.v[2739] = if (((-s.v[443]) / s.v[2661]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) && (!(s.v[2738] != 0.0))) && (s.v[2739] != 0.0)) {
            let assign58940_ad_e75183: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(443)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign58940_ad_e75183);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) && (!(s.v[2738] != 0.0))) && (!(s.v[2739] != 0.0))) {
            let assign58950_ad_e75234: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(443)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2636, &assign58950_ad_e75234);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2736] != 0.0))) {
            s.store_scale_ad(2660, A::mul(A::mul(A::mul(s.ad_value(832), s.ad_value(2661)), s.ad_value(2661)), s.ad_value(2636)), p.p853);
        }

        s.v[2740] = if (s.v[640] > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2740] != 0.0)) {
            s.store_scalar(2662, 1.0);
        }

        s.v[2741] = if (s.v[2635] > ((-s.v[444]) * s.v[640])) { 1.0 } else { 0.0 };

        s.v[2742] = if (p.p865 == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2740] != 0.0))) && (s.v[2741] != 0.0)) && (s.v[2742] != 0.0)) {
            s.store_mul_ad(2636, A::mul(A::mul(A::mul(s.ad_value(2635), s.ad_value(450)), A::mul(s.ad_value(2635), s.ad_value(450))), A::mul(s.ad_value(2635), s.ad_value(450))), A::mul(s.ad_value(2635), s.ad_value(450)));
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2740] != 0.0))) && (s.v[2741] != 0.0)) && (!(s.v[2742] != 0.0))) {
            s.store_powf_ad(2636, A::abs(A::mul(s.ad_value(2635), s.ad_value(450))), p.p865);
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2740] != 0.0))) && (s.v[2741] != 0.0)) {
            s.store_div_from_scalar_ad(2662, 1.0, A::sub_from_scalar(1.0, s.ad_value(2636)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2740] != 0.0))) && (!(s.v[2741] != 0.0))) {
            s.store_offset_ad(2662, A::mul(A::add(s.ad_value(2635), A::scale(s.ad_value(640), s.v[444])), s.ad_value(453)), s.v[447]);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) {
            s.store_mul_ad_lhs(1914, A::scale(A::add(A::add(A::add(s.ad_value(2637), s.ad_value(2638)), s.ad_value(2645)), s.ad_value(2660)), p.p29), 2662);
        }

        s.v[2743] = if (s.v[473] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            let assign59070_ad_e75459: A = {
                if (s.v[832] < p.p870) {
                    {
                        if (((s.v[832] - p.p870) / p.p871) < (-37.0)) {
                            A::constant(p.p870)
                        } else {
                            A::offset(A::scale(A::ln(A::offset(A::exp(A::scale(A::offset(s.ad_value(832), (-p.p870)), 1.0 / (p.p871))), 1.0)), p.p871), p.p870)
                        }
                    }
                } else {
                    {
                        if (((s.v[832] - p.p870) / p.p871) > 37.0) {
                            s.ad_value(832)
                        } else {
                            A::add(s.ad_value(832), A::scale(A::ln(A::offset(A::exp(A::scale(A::sub_from_scalar(p.p870, s.ad_value(832)), 1.0 / (p.p871))), 1.0)), p.p871))
                        }
                    }
                }
            };
            s.store_ad(2663, &assign59070_ad_e75459);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(657), 4.0), 657);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_div(2622, 657, 658);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_add_ad_rhs(2623, 2663, A::mul(s.ad_value(657), s.ad_value(2622)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_add(2624, 658, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_sub(2625, 658, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_scale_ad(2664, A::div(A::mul(s.ad_value(2663), s.ad_value(658)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2744] = if (s.v[410] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) && (s.v[2744] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2664), s.v[407])));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) && (!(s.v[2744] != 0.0))) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2664), s.v[407])), s.v[410]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_scale_ad(1920, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2636)), s.v[419]), A::scale(A::sub(s.ad_value(2663), s.ad_value(2664)), s.v[422])), p.p30);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_sub_ad_lhs(2663, A::offset(s.ad_value(832), p.p870), 2663);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(657), 4.0), 657);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_div(2622, 657, 658);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_add_ad_rhs(2623, 2663, A::mul(s.ad_value(657), s.ad_value(2622)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_add(2624, 658, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_sub(2625, 658, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_scale_ad(2664, A::div(A::mul(s.ad_value(2663), s.ad_value(658)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2745] = if (s.v[467] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) && (s.v[2745] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(466))));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) && (!(s.v[2745] != 0.0))) {
            s.store_ad(2636, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(466))), s.ad_value(467)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_scale_ad(472, A::add(A::mul(s.ad_value(470), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(471), A::sub(s.ad_value(2663), s.ad_value(2664)))), p.p30);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (s.v[2743] != 0.0)) {
            s.store_add(1920, 1920, 472);
        }

        s.v[2746] = if (s.v[410] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2743] != 0.0))) && (s.v[2746] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[407])));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2743] != 0.0))) && (!(s.v[2746] != 0.0))) {
            s.store_powf_ad(2636, A::sub_from_scalar(1.0, A::scale(s.ad_value(2628), s.v[407])), s.v[410]);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2726] != 0.0))) && (!(s.v[2743] != 0.0))) {
            s.store_scale_ad(1920, A::add(A::scale(A::sub_from_scalar(1.0, s.ad_value(2636)), s.v[419]), A::scale(A::sub(s.ad_value(832), s.ad_value(2628)), s.v[422])), p.p30);
        }

        if ((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) {
            s.store_add_ad(848, A::add(A::mul(s.ad_value(646), s.ad_value(1912)), A::mul(s.ad_value(647), s.ad_value(1913))), A::mul(s.ad_value(648), s.ad_value(1914)));
        }

        s.v[2747] = if (s.v[636] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2747] != 0.0)) {
            s.store_mul_ad_rhs(643, 636, A::sub(A::pow(A::scale(A::add(A::add(s.ad_value(825), s.ad_value(827)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001)))), 0.5), s.ad_value(637)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(637))));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2747] != 0.0)) {
            s.store_add(641, 542, 643);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2747] != 0.0)) {
            s.store_div_from_scalar(616, 1.0, 641);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2747] != 0.0)) {
            s.store_div_ad_rhs(619, 619, A::offset(A::div(s.ad_value(643), s.ad_value(542)), 1.0));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2747] != 0.0))) {
            s.copy_ad(641, 542);
        }

        s.v[2748] = if (s.v[638] > 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2748] != 0.0)) {
            s.store_mul_ad_rhs(645, 638, A::sub(A::pow(A::scale(A::add(A::add(s.ad_value(825), s.ad_value(827)), A::sqrt(A::offset(A::mul(A::add(s.ad_value(825), s.ad_value(827)), A::add(s.ad_value(825), s.ad_value(827))), (0.001 * 0.001)))), 0.5), s.ad_value(639)), A::pow_from_scalar((0.5 * 0.001), s.ad_value(639))));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2748] != 0.0)) {
            s.store_mul_ad_rhs(610, 610, A::offset(s.ad_value(645), 1.0));
        }

        if ((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) {
            s.store_scalar(2634, 0.0);
        }

        if ((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) {
            s.store_scalar(2631, 0.0);
        }

        s.v[2749] = if !(((s.v[673] == 0.0) && (s.v[674] == 0.0)) && (s.v[675] == 0.0)) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(684), 4.0), 684);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_div(2622, 684, 685);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_add_ad_rhs(2623, 833, A::mul(s.ad_value(684), s.ad_value(2622)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_add(2624, 685, 2623);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_sub(2625, 685, 2623);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_scale_ad(2628, A::div(A::mul(s.ad_value(833), s.ad_value(685)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2750] = if (s.v[833] < s.v[681]) { 1.0 } else { 0.0 };

        s.v[2751] = if (((((-0.5) * (s.v[833] * s.v[371]))) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (s.v[2750] != 0.0)) && (s.v[2751] != 0.0)) {
            s.store_exp_ad(2629, A::scale(s.ad_value(833), (s.v[371] * (-0.5))));
        }

        s.v[2752] = if (((-0.5) * (s.v[833] * s.v[371])) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (s.v[2750] != 0.0)) && (!(s.v[2751] != 0.0))) && (s.v[2752] != 0.0)) {
            let assign59600_ad_e76293: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5)))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5)))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::scale(s.ad_value(833), (s.v[371] * (-0.5)))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2629, &assign59600_ad_e76293);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (s.v[2750] != 0.0)) && (!(s.v[2751] != 0.0))) && (!(s.v[2752] != 0.0))) {
            s.store_scale_ad(2629, A::offset(A::mul(A::offset(A::scale(s.ad_value(833), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::scale(s.ad_value(833), (s.v[371] * (-0.5))), (-230.25850929940458)), A::offset(A::scale(A::offset(A::scale(s.ad_value(833), (s.v[371] * (-0.5))), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (s.v[2750] != 0.0)) {
            s.store_div_from_scalar(2630, 1.0, 2629);
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
        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (s.v[2750] != 0.0)) {
            s.store_square(2627, 2630);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (!(s.v[2750] != 0.0))) {
            s.store_mul_ad_lhs(2627, A::offset(A::scale(A::sub(s.ad_value(833), s.ad_value(681)), s.v[371]), 1.0), 682);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (!(s.v[2750] != 0.0))) {
            s.store_sqrt(2630, 2627);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (!(s.v[2750] != 0.0))) {
            s.store_div_from_scalar(2629, 1.0, 2630);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_offset(2627, 2627, (-1.0));
        }

        s.v[2753] = if (s.v[833] > 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (s.v[2753] != 0.0)) {
            s.store_scale_ad(2631, A::ln(A::add(A::offset(s.ad_value(2629), 2.0), A::sqrt(A::mul(A::offset(s.ad_value(2629), 1.0), A::offset(s.ad_value(2629), 3.0))))), (s.v[370] * 2.0));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) && (!(s.v[2753] != 0.0))) {
            s.store_sub_ad_lhs(2631, A::scale(A::ln(A::add(A::offset(A::scale(s.ad_value(2630), 2.0), 1.0), A::sqrt(A::mul(A::offset(s.ad_value(2630), 1.0), A::offset(A::scale(s.ad_value(2630), 3.0), 1.0))))), (s.v[370] * 2.0)), 833);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_sub(2632, 683, 2631);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_scale_ad(2633, A::sub(A::add(s.ad_value(833), s.ad_value(2632)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(833), s.ad_value(2632)), A::sub(s.ad_value(833), s.ad_value(2632))), ((4.0 * s.v[370]) * s.v[370])))), 0.5);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_scale_ad(2634, A::sub(A::add(s.ad_value(833), s.ad_value(686)), A::sqrt(A::offset(A::mul(A::sub(s.ad_value(833), s.ad_value(686)), A::sub(s.ad_value(833), s.ad_value(686))), ((4.0 * s.v[368]) * s.v[368])))), 0.5);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2749] != 0.0)) {
            s.store_scale_ad(2635, A::sub(s.ad_value(833), A::sqrt(A::offset(A::mul(s.ad_value(833), s.ad_value(833)), ((4.0 * 1e-6) * 1e-6)))), 0.5);
        }

        s.v[2754] = if (s.v[673] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2754] != 0.0)) {
            s.store_scalar(1915, 0.0);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2754] != 0.0)) {
            s.store_scalar(1921, 0.0);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) {
            s.store_mul(2637, 563, 2627);
        }

        s.v[2755] = if ((s.v[522] == 0.0) && (s.v[525] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (s.v[2755] != 0.0)) {
            s.store_scalar(2638, 0.0);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) {
            s.store_sub(2639, 569, 2633);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) {
            s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));
        }

        s.v[2756] = if (s.v[511] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) && (s.v[2756] != 0.0)) {
            s.store_scalar(2641, 0.0);
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) && (!(s.v[2756] != 0.0))) {
            s.store_mul_ad(2641, A::add(A::div(A::mul(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640))), A::sub_from_scalar(1.0, s.ad_value(2640))), s.ad_value(2640)), A::sub_from_scalar(1.0, A::scale(s.ad_value(511), 2.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) {
            s.store_add(2642, 2640, 2641);
        }

        s.v[2757] = if (s.v[511] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) && (s.v[2757] != 0.0)) {
            s.store_sqrt_ad(2636, A::mul(s.ad_value(2639), s.ad_value(596)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) && (!(s.v[2757] != 0.0))) {
            s.store_ad(2636, &A::pow(A::mul(s.ad_value(2639), s.ad_value(596)), s.ad_value(511)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) {
            s.store_mul(2643, 590, 2636);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) {
            s.store_mul_ad_rhs(2644, 560, A::mul(A::offset(s.ad_value(2630), (-1.0)), s.ad_value(2643)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2755] != 0.0))) {
            s.store_mul_ad_rhs(2638, 522, A::mul(s.ad_value(2644), s.ad_value(2642)));
        }

        s.v[2758] = if (s.v[525] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (s.v[2758] != 0.0)) {
            s.store_scalar(2645, 0.0);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_mul_ad_rhs(2646, 605, A::div(A::mul(s.ad_value(2643), s.ad_value(575)), s.ad_value(2639)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_div_ad_lhs(2647, A::scale(s.ad_value(602), 0.666666666666667), 2646);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_square(2648, 2647);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_sqrt_ad(2649, A::div(A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_sqrt(2650, 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_mul(2651, 2649, 2650);
        }

        s.v[2759] = if (((-s.v[511]) * s.v[578]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (s.v[2759] != 0.0)) {
            s.store_div_from_scalar_ad(2652, 1.0, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (!(s.v[2759] != 0.0))) {
            s.store_pow_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), A::mul(A::neg(s.ad_value(511)), s.ad_value(578)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_div_ad(2653, A::mul(s.ad_value(2642), s.ad_value(2652)), A::add(s.ad_value(2642), s.ad_value(2652)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_sqrt_ad(2654, A::scale(A::div(s.ad_value(2646), s.ad_value(2650)), 0.375));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_add_ad(2656, A::sub(A::mul(A::mul(s.ad_value(602), s.ad_value(2647)), s.ad_value(2650)), A::mul(s.ad_value(602), s.ad_value(2649))), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_mul_ad_lhs(2657, A::offset(s.ad_value(2655), (-1.0)), 2654);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_square(2618, 2657);
        }

        s.v[2760] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (s.v[2760] != 0.0)) {
            s.store_div_from_scalar_ad(2619, 1.0, A::offset(A::scale(s.ad_value(2657), s.v[372]), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (!(s.v[2760] != 0.0))) {
            s.store_div_from_scalar_ad(2619, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2657), s.v[372])));
        }

        s.v[2761] = if (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (s.v[2761] != 0.0)) {
            s.store_exp_ad(2636, A::sub(s.ad_value(2656), s.ad_value(2618)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (!(s.v[2761] != 0.0))) {
            let assign60150_ad_e77243: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2636, &assign60150_ad_e77243);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_mul_ad_lhs(2620, A::add(A::add(A::scale(s.ad_value(2619), 0.29214664), A::scale(A::square(s.ad_value(2619)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(2619)), s.ad_value(2619)), s.v[374])), 2636);
        }

        s.v[2762] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (s.v[2762] != 0.0)) {
            s.copy_ad(2658, 2620);
        }

        s.v[2763] = if (s.v[2656] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (!(s.v[2762] != 0.0))) && (s.v[2763] != 0.0)) {
            s.store_exp(2636, 2656);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (!(s.v[2762] != 0.0))) && (!(s.v[2763] != 0.0))) {
            s.store_div_from_scalar_ad(2636, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) && (!(s.v[2762] != 0.0))) {
            s.store_sub_ad_lhs(2658, A::scale(s.ad_value(2636), 2.0), 2620);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_scale_ad(2659, A::div(A::mul(s.ad_value(602), s.ad_value(2658)), s.ad_value(2654)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2758] != 0.0))) {
            s.store_mul_ad_rhs(2645, 525, A::mul(A::mul(s.ad_value(2644), s.ad_value(2659)), s.ad_value(2653)));
        }

        s.v[2764] = if (s.v[531] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (s.v[2764] != 0.0)) {
            s.store_scalar(2660, 0.0);
        }

        s.v[2765] = if (s.v[511] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) && (s.v[2765] != 0.0)) {
            s.store_sqrt_ad(2636, A::mul(A::sub(s.ad_value(508), s.ad_value(2634)), s.ad_value(596)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) && (!(s.v[2765] != 0.0))) {
            s.store_ad(2636, &A::pow(A::mul(A::sub(s.ad_value(508), s.ad_value(2634)), s.ad_value(596)), s.ad_value(511)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) {
            s.store_mul_ad_rhs(2661, 578, A::div(A::mul(A::sub(s.ad_value(508), s.ad_value(2634)), s.ad_value(593)), s.ad_value(2636)));
        }

        s.v[2766] = if (((((-s.v[608]) / s.v[2661])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) && (s.v[2766] != 0.0)) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(608)), s.ad_value(2661)));
        }

        s.v[2767] = if (((-s.v[608]) / s.v[2661]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) && (!(s.v[2766] != 0.0))) && (s.v[2767] != 0.0)) {
            let assign60340_ad_e77583: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(608)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(608)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(608)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign60340_ad_e77583);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) && (!(s.v[2766] != 0.0))) && (!(s.v[2767] != 0.0))) {
            let assign60350_ad_e77634: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(608)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(608)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(608)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2636, &assign60350_ad_e77634);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2764] != 0.0))) {
            s.store_mul_ad_rhs(2660, 531, A::mul(A::mul(A::mul(s.ad_value(833), s.ad_value(2661)), s.ad_value(2661)), s.ad_value(2636)));
        }

        s.v[2768] = if (s.v[540] > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (s.v[2768] != 0.0)) {
            s.store_scalar(2662, 1.0);
        }

        s.v[2769] = if (s.v[2635] > ((-s.v[444]) * s.v[540])) { 1.0 } else { 0.0 };

        s.v[2770] = if (s.v[543] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2768] != 0.0))) && (s.v[2769] != 0.0)) && (s.v[2770] != 0.0)) {
            s.store_mul_ad(2636, A::mul(A::mul(A::mul(s.ad_value(2635), s.ad_value(614)), A::mul(s.ad_value(2635), s.ad_value(614))), A::mul(s.ad_value(2635), s.ad_value(614))), A::mul(s.ad_value(2635), s.ad_value(614)));
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2768] != 0.0))) && (s.v[2769] != 0.0)) && (!(s.v[2770] != 0.0))) {
            s.store_ad(2636, &A::pow(A::abs(A::mul(s.ad_value(2635), s.ad_value(614))), s.ad_value(543)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2768] != 0.0))) && (s.v[2769] != 0.0)) {
            s.store_div_from_scalar_ad(2662, 1.0, A::sub_from_scalar(1.0, s.ad_value(2636)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2768] != 0.0))) && (!(s.v[2769] != 0.0))) {
            s.store_add_ad_rhs(2662, 611, A::mul(A::add(s.ad_value(2635), A::scale(s.ad_value(540), s.v[444])), s.ad_value(617)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) {
            s.store_mul_ad_lhs(1915, A::scale(A::add(A::add(A::add(s.ad_value(2637), s.ad_value(2638)), s.ad_value(2645)), s.ad_value(2660)), p.p29), 2662);
        }

        s.v[2771] = if (s.v[575] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (s.v[2771] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(572))));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) && (!(s.v[2771] != 0.0))) {
            s.store_ad(2636, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(572))), s.ad_value(575)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2754] != 0.0))) {
            s.store_scale_ad(1921, A::add(A::mul(s.ad_value(584), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(587), A::sub(s.ad_value(833), s.ad_value(2628)))), p.p30);
        }

        s.v[2772] = if (s.v[674] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2772] != 0.0)) {
            s.store_scalar(1916, 0.0);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2772] != 0.0)) {
            s.store_scalar(1922, 0.0);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) {
            s.store_mul(2637, 564, 2627);
        }

        s.v[2773] = if ((s.v[523] == 0.0) && (s.v[526] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (s.v[2773] != 0.0)) {
            s.store_scalar(2638, 0.0);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) {
            s.store_sub(2639, 570, 2633);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) {
            s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));
        }

        s.v[2774] = if (s.v[512] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) && (s.v[2774] != 0.0)) {
            s.store_scalar(2641, 0.0);
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) && (!(s.v[2774] != 0.0))) {
            s.store_mul_ad(2641, A::add(A::div(A::mul(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640))), A::sub_from_scalar(1.0, s.ad_value(2640))), s.ad_value(2640)), A::sub_from_scalar(1.0, A::scale(s.ad_value(512), 2.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) {
            s.store_add(2642, 2640, 2641);
        }

        s.v[2775] = if (s.v[512] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) && (s.v[2775] != 0.0)) {
            s.store_sqrt_ad(2636, A::mul(s.ad_value(2639), s.ad_value(597)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) && (!(s.v[2775] != 0.0))) {
            s.store_ad(2636, &A::pow(A::mul(s.ad_value(2639), s.ad_value(597)), s.ad_value(512)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) {
            s.store_mul(2643, 591, 2636);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) {
            s.store_mul_ad_rhs(2644, 561, A::mul(A::offset(s.ad_value(2630), (-1.0)), s.ad_value(2643)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2773] != 0.0))) {
            s.store_mul_ad_rhs(2638, 523, A::mul(s.ad_value(2644), s.ad_value(2642)));
        }

        s.v[2776] = if (s.v[526] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (s.v[2776] != 0.0)) {
            s.store_scalar(2645, 0.0);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_mul_ad_rhs(2646, 606, A::div(A::mul(s.ad_value(2643), s.ad_value(576)), s.ad_value(2639)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_div_ad_lhs(2647, A::scale(s.ad_value(603), 0.666666666666667), 2646);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_square(2648, 2647);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_sqrt_ad(2649, A::div(A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_sqrt(2650, 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_mul(2651, 2649, 2650);
        }

        s.v[2777] = if (((-s.v[512]) * s.v[579]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (s.v[2777] != 0.0)) {
            s.store_div_from_scalar_ad(2652, 1.0, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (!(s.v[2777] != 0.0))) {
            s.store_pow_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), A::mul(A::neg(s.ad_value(512)), s.ad_value(579)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_div_ad(2653, A::mul(s.ad_value(2642), s.ad_value(2652)), A::add(s.ad_value(2642), s.ad_value(2652)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_sqrt_ad(2654, A::scale(A::div(s.ad_value(2646), s.ad_value(2650)), 0.375));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_add_ad(2656, A::sub(A::mul(A::mul(s.ad_value(603), s.ad_value(2647)), s.ad_value(2650)), A::mul(s.ad_value(603), s.ad_value(2649))), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_mul_ad_lhs(2657, A::offset(s.ad_value(2655), (-1.0)), 2654);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_square(2618, 2657);
        }

        s.v[2778] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (s.v[2778] != 0.0)) {
            s.store_div_from_scalar_ad(2619, 1.0, A::offset(A::scale(s.ad_value(2657), s.v[372]), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (!(s.v[2778] != 0.0))) {
            s.store_div_from_scalar_ad(2619, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2657), s.v[372])));
        }

        s.v[2779] = if (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (s.v[2779] != 0.0)) {
            s.store_exp_ad(2636, A::sub(s.ad_value(2656), s.ad_value(2618)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (!(s.v[2779] != 0.0))) {
            let assign60900_ad_e78509: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2636, &assign60900_ad_e78509);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_mul_ad_lhs(2620, A::add(A::add(A::scale(s.ad_value(2619), 0.29214664), A::scale(A::square(s.ad_value(2619)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(2619)), s.ad_value(2619)), s.v[374])), 2636);
        }

        s.v[2780] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (s.v[2780] != 0.0)) {
            s.copy_ad(2658, 2620);
        }

        s.v[2781] = if (s.v[2656] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (!(s.v[2780] != 0.0))) && (s.v[2781] != 0.0)) {
            s.store_exp(2636, 2656);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (!(s.v[2780] != 0.0))) && (!(s.v[2781] != 0.0))) {
            s.store_div_from_scalar_ad(2636, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) && (!(s.v[2780] != 0.0))) {
            s.store_sub_ad_lhs(2658, A::scale(s.ad_value(2636), 2.0), 2620);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_scale_ad(2659, A::div(A::mul(s.ad_value(603), s.ad_value(2658)), s.ad_value(2654)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2776] != 0.0))) {
            s.store_mul_ad_rhs(2645, 526, A::mul(A::mul(s.ad_value(2644), s.ad_value(2659)), s.ad_value(2653)));
        }

        s.v[2782] = if (s.v[532] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (s.v[2782] != 0.0)) {
            s.store_scalar(2660, 0.0);
        }

        s.v[2783] = if (s.v[512] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) && (s.v[2783] != 0.0)) {
            s.store_sqrt_ad(2636, A::mul(A::sub(s.ad_value(509), s.ad_value(2634)), s.ad_value(597)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) && (!(s.v[2783] != 0.0))) {
            s.store_ad(2636, &A::pow(A::mul(A::sub(s.ad_value(509), s.ad_value(2634)), s.ad_value(597)), s.ad_value(512)));
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
        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) {
            s.store_mul_ad_rhs(2661, 579, A::div(A::mul(A::sub(s.ad_value(509), s.ad_value(2634)), s.ad_value(594)), s.ad_value(2636)));
        }

        s.v[2784] = if (((((-s.v[609]) / s.v[2661])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) && (s.v[2784] != 0.0)) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(609)), s.ad_value(2661)));
        }

        s.v[2785] = if (((-s.v[609]) / s.v[2661]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) && (!(s.v[2784] != 0.0))) && (s.v[2785] != 0.0)) {
            let assign61090_ad_e78849: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(609)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign61090_ad_e78849);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) && (!(s.v[2784] != 0.0))) && (!(s.v[2785] != 0.0))) {
            let assign61100_ad_e78900: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(609)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2636, &assign61100_ad_e78900);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2782] != 0.0))) {
            s.store_mul_ad_rhs(2660, 532, A::mul(A::mul(A::mul(s.ad_value(833), s.ad_value(2661)), s.ad_value(2661)), s.ad_value(2636)));
        }

        s.v[2786] = if (s.v[541] > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (s.v[2786] != 0.0)) {
            s.store_scalar(2662, 1.0);
        }

        s.v[2787] = if (s.v[2635] > ((-s.v[444]) * s.v[541])) { 1.0 } else { 0.0 };

        s.v[2788] = if (s.v[544] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2786] != 0.0))) && (s.v[2787] != 0.0)) && (s.v[2788] != 0.0)) {
            s.store_mul_ad(2636, A::mul(A::mul(A::mul(s.ad_value(2635), s.ad_value(615)), A::mul(s.ad_value(2635), s.ad_value(615))), A::mul(s.ad_value(2635), s.ad_value(615))), A::mul(s.ad_value(2635), s.ad_value(615)));
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2786] != 0.0))) && (s.v[2787] != 0.0)) && (!(s.v[2788] != 0.0))) {
            s.store_ad(2636, &A::pow(A::abs(A::mul(s.ad_value(2635), s.ad_value(615))), s.ad_value(544)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2786] != 0.0))) && (s.v[2787] != 0.0)) {
            s.store_div_from_scalar_ad(2662, 1.0, A::sub_from_scalar(1.0, s.ad_value(2636)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2786] != 0.0))) && (!(s.v[2787] != 0.0))) {
            s.store_add_ad_rhs(2662, 612, A::mul(A::add(s.ad_value(2635), A::scale(s.ad_value(541), s.v[444])), s.ad_value(618)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) {
            s.store_mul_ad_lhs(1916, A::scale(A::add(A::add(A::add(s.ad_value(2637), s.ad_value(2638)), s.ad_value(2645)), s.ad_value(2660)), p.p29), 2662);
        }

        s.v[2789] = if (s.v[576] == 0.5) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (s.v[2789] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(573))));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) && (!(s.v[2789] != 0.0))) {
            s.store_ad(2636, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(573))), s.ad_value(576)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2772] != 0.0))) {
            s.store_scale_ad(1922, A::add(A::mul(s.ad_value(585), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(588), A::sub(s.ad_value(833), s.ad_value(2628)))), p.p30);
        }

        s.v[2790] = if (s.v[675] == 0.0) { 1.0 } else { 0.0 };

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2790] != 0.0)) {
            s.store_scalar(1917, 0.0);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (s.v[2790] != 0.0)) {
            s.store_scalar(1923, 0.0);
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) {
            s.store_mul(2637, 565, 2627);
        }

        s.v[2791] = if ((s.v[524] == 0.0) && (s.v[527] == 0.0)) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2791] != 0.0)) {
            s.store_scalar(2638, 0.0);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) {
            s.store_sub(2639, 571, 2633);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) {
            s.store_sub_from_scalar_ad(2640, 1.0, A::sqrt(A::sub_from_scalar(1.0, A::div(s.ad_value(2631), s.ad_value(2639)))));
        }

        s.v[2792] = if (s.v[513] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) && (s.v[2792] != 0.0)) {
            s.store_scalar(2641, 0.0);
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) && (!(s.v[2792] != 0.0))) {
            s.store_mul_ad(2641, A::add(A::div(A::mul(A::square(s.ad_value(2640)), A::ln(s.ad_value(2640))), A::sub_from_scalar(1.0, s.ad_value(2640))), s.ad_value(2640)), A::sub_from_scalar(1.0, A::scale(s.ad_value(513), 2.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) {
            s.store_add(2642, 2640, 2641);
        }

        s.v[2793] = if (s.v[513] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) && (s.v[2793] != 0.0)) {
            s.store_sqrt_ad(2636, A::mul(s.ad_value(2639), s.ad_value(598)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) && (!(s.v[2793] != 0.0))) {
            s.store_ad(2636, &A::pow(A::mul(s.ad_value(2639), s.ad_value(598)), s.ad_value(513)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) {
            s.store_mul(2643, 592, 2636);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) {
            s.store_mul_ad_rhs(2644, 562, A::mul(A::offset(s.ad_value(2630), (-1.0)), s.ad_value(2643)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2791] != 0.0))) {
            s.store_mul_ad_rhs(2638, 524, A::mul(s.ad_value(2644), s.ad_value(2642)));
        }

        s.v[2794] = if (s.v[527] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2794] != 0.0)) {
            s.store_scalar(2645, 0.0);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_mul_ad_rhs(2646, 607, A::div(A::mul(s.ad_value(2643), s.ad_value(577)), s.ad_value(2639)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_div_ad_lhs(2647, A::scale(s.ad_value(604), 0.666666666666667), 2646);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_square(2648, 2647);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_sqrt_ad(2649, A::div(A::square(s.ad_value(2648)), A::offset(A::square(s.ad_value(2648)), 1.0)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_sqrt(2650, 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_mul(2651, 2649, 2650);
        }

        s.v[2795] = if (((-s.v[513]) * s.v[580]) == (-1.0)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (s.v[2795] != 0.0)) {
            s.store_div_from_scalar_ad(2652, 1.0, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (!(s.v[2795] != 0.0))) {
            s.store_pow_ad(2652, A::offset(A::mul(s.ad_value(2646), s.ad_value(2651)), 1.0), A::mul(A::neg(s.ad_value(513)), s.ad_value(580)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_div_ad(2653, A::mul(s.ad_value(2642), s.ad_value(2652)), A::add(s.ad_value(2642), s.ad_value(2652)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_sqrt_ad(2654, A::scale(A::div(s.ad_value(2646), s.ad_value(2650)), 0.375));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_sub_ad_lhs(2655, A::scale(A::mul(s.ad_value(2647), s.ad_value(2650)), 2.0), 2649);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_add_ad(2656, A::sub(A::mul(A::mul(s.ad_value(604), s.ad_value(2647)), s.ad_value(2650)), A::mul(s.ad_value(604), s.ad_value(2649))), A::scale(A::mul(s.ad_value(2646), s.ad_value(2651)), 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_mul_ad_lhs(2657, A::offset(s.ad_value(2655), (-1.0)), 2654);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_square(2618, 2657);
        }

        s.v[2796] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (s.v[2796] != 0.0)) {
            s.store_div_from_scalar_ad(2619, 1.0, A::offset(A::scale(s.ad_value(2657), s.v[372]), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (!(s.v[2796] != 0.0))) {
            s.store_div_from_scalar_ad(2619, 1.0, A::sub_from_scalar(1.0, A::scale(s.ad_value(2657), s.v[372])));
        }

        s.v[2797] = if (((-s.v[2618]) + s.v[2656]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (s.v[2797] != 0.0)) {
            s.store_exp_ad(2636, A::sub(s.ad_value(2656), s.ad_value(2618)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (!(s.v[2797] != 0.0))) {
            let assign61650_ad_e79775: A = A::div_from_scalar(1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::sub(s.ad_value(2656), s.ad_value(2618))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            s.store_ad(2636, &assign61650_ad_e79775);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_mul_ad_lhs(2620, A::add(A::add(A::scale(s.ad_value(2619), 0.29214664), A::scale(A::square(s.ad_value(2619)), s.v[373])), A::scale(A::mul(A::square(s.ad_value(2619)), s.ad_value(2619)), s.v[374])), 2636);
        }

        s.v[2798] = if (s.v[2657] > 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (s.v[2798] != 0.0)) {
            s.copy_ad(2658, 2620);
        }

        s.v[2799] = if (s.v[2656] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (!(s.v[2798] != 0.0))) && (s.v[2799] != 0.0)) {
            s.store_exp(2636, 2656);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (!(s.v[2798] != 0.0))) && (!(s.v[2799] != 0.0))) {
            s.store_div_from_scalar_ad(2636, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2656)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) && (!(s.v[2798] != 0.0))) {
            s.store_sub_ad_lhs(2658, A::scale(s.ad_value(2636), 2.0), 2620);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_scale_ad(2659, A::div(A::mul(s.ad_value(604), s.ad_value(2658)), s.ad_value(2654)), (1.772453850905516 * 0.5));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2794] != 0.0))) {
            s.store_mul_ad_rhs(2645, 527, A::mul(A::mul(s.ad_value(2644), s.ad_value(2659)), s.ad_value(2653)));
        }

        s.v[2800] = if (s.v[533] == 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2800] != 0.0)) {
            s.store_scalar(2660, 0.0);
        }

        s.v[2801] = if (s.v[513] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) && (s.v[2801] != 0.0)) {
            s.store_sqrt_ad(2636, A::mul(A::sub(s.ad_value(510), s.ad_value(2634)), s.ad_value(598)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) && (!(s.v[2801] != 0.0))) {
            s.store_ad(2636, &A::pow(A::mul(A::sub(s.ad_value(510), s.ad_value(2634)), s.ad_value(598)), s.ad_value(513)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) {
            s.store_mul_ad_rhs(2661, 580, A::div(A::mul(A::sub(s.ad_value(510), s.ad_value(2634)), s.ad_value(595)), s.ad_value(2636)));
        }

        s.v[2802] = if (((((-s.v[610]) / s.v[2661])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) && (s.v[2802] != 0.0)) {
            s.store_exp_ad(2636, A::div(A::neg(s.ad_value(610)), s.ad_value(2661)));
        }

        s.v[2803] = if (((-s.v[610]) / s.v[2661]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) && (!(s.v[2802] != 0.0))) && (s.v[2803] != 0.0)) {
            let assign61840_ad_e80115: A = A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2661))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2661))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::div(A::neg(s.ad_value(610)), s.ad_value(2661))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0);
            s.store_div_from_scalar_ad(2636, 1e-100, assign61840_ad_e80115);
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) && (!(s.v[2802] != 0.0))) && (!(s.v[2803] != 0.0))) {
            let assign61850_ad_e80166: A = A::scale(A::offset(A::mul(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2661)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::div(A::neg(s.ad_value(610)), s.ad_value(2661)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
            s.store_ad(2636, &assign61850_ad_e80166);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2800] != 0.0))) {
            s.store_mul_ad_rhs(2660, 533, A::mul(A::mul(A::mul(s.ad_value(833), s.ad_value(2661)), s.ad_value(2661)), s.ad_value(2636)));
        }

        s.v[2804] = if (s.v[641] > 1000.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2804] != 0.0)) {
            s.store_scalar(2662, 1.0);
        }

        s.v[2805] = if (s.v[2635] > ((-s.v[444]) * s.v[641])) { 1.0 } else { 0.0 };

        s.v[2806] = if (s.v[545] == 4.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2804] != 0.0))) && (s.v[2805] != 0.0)) && (s.v[2806] != 0.0)) {
            s.store_mul_ad(2636, A::mul(A::mul(A::mul(s.ad_value(2635), s.ad_value(616)), A::mul(s.ad_value(2635), s.ad_value(616))), A::mul(s.ad_value(2635), s.ad_value(616))), A::mul(s.ad_value(2635), s.ad_value(616)));
        }

        if ((((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2804] != 0.0))) && (s.v[2805] != 0.0)) && (!(s.v[2806] != 0.0))) {
            s.store_ad(2636, &A::pow(A::abs(A::mul(s.ad_value(2635), s.ad_value(616))), s.ad_value(545)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2804] != 0.0))) && (s.v[2805] != 0.0)) {
            s.store_div_from_scalar_ad(2662, 1.0, A::sub_from_scalar(1.0, s.ad_value(2636)));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2804] != 0.0))) && (!(s.v[2805] != 0.0))) {
            s.store_add_ad_rhs(2662, 613, A::mul(A::add(s.ad_value(2635), A::scale(s.ad_value(641), s.v[444])), s.ad_value(619)));
        }

        if (((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) {
            s.store_mul_ad_lhs(1917, A::scale(A::add(A::add(A::add(s.ad_value(2637), s.ad_value(2638)), s.ad_value(2645)), s.ad_value(2660)), p.p29), 2662);
        }

        s.v[2807] = if (s.v[635] == 1.0) { 1.0 } else { 0.0 };

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            let assign61970_ad_e80391: A = {
                if (s.v[833] < s.v[550]) {
                    {
                        if (((s.v[833] - s.v[550]) / s.v[551]) < (-37.0)) {
                            s.ad_value(550)
                        } else {
                            A::add(s.ad_value(550), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(833), s.ad_value(550)), s.ad_value(551))), 1.0)), s.ad_value(551)))
                        }
                    }
                } else {
                    {
                        if (((s.v[833] - s.v[550]) / s.v[551]) > 37.0) {
                            s.ad_value(833)
                        } else {
                            A::add(s.ad_value(833), A::mul(A::ln(A::offset(A::exp(A::div(A::sub(s.ad_value(550), s.ad_value(833)), s.ad_value(551))), 1.0)), s.ad_value(551)))
                        }
                    }
                }
            };
            s.store_ad(2663, &assign61970_ad_e80391);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(684), 4.0), 684);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_div(2622, 684, 685);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_add_ad_rhs(2623, 2663, A::mul(s.ad_value(684), s.ad_value(2622)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_add(2624, 685, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_sub(2625, 685, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_scale_ad(2664, A::div(A::mul(s.ad_value(2663), s.ad_value(685)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2808] = if (s.v[577] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) && (s.v[2808] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(574))));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) && (!(s.v[2808] != 0.0))) {
            s.store_ad(2636, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(574))), s.ad_value(577)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_scale_ad(1923, A::add(A::mul(s.ad_value(586), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(589), A::sub(s.ad_value(2663), s.ad_value(2664)))), p.p30);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_sub_ad_lhs(2663, A::add(s.ad_value(833), s.ad_value(550)), 2663);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_mul_ad_lhs(2621, A::scale(s.ad_value(684), 4.0), 684);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_div(2622, 684, 685);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_add_ad_rhs(2623, 2663, A::mul(s.ad_value(684), s.ad_value(2622)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_add(2624, 685, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_sub(2625, 685, 2623);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_sqrt_ad(2626, A::add(A::square(s.ad_value(2625)), s.ad_value(2621)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_scale_ad(2664, A::div(A::mul(s.ad_value(2663), s.ad_value(685)), A::add(s.ad_value(2624), s.ad_value(2626))), 2.0);
        }

        s.v[2809] = if (s.v[630] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) && (s.v[2809] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(629))));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) && (!(s.v[2809] != 0.0))) {
            s.store_ad(2636, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2664), s.ad_value(629))), s.ad_value(630)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_scale_ad(472, A::add(A::mul(s.ad_value(633), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(634), A::sub(s.ad_value(2663), s.ad_value(2664)))), p.p30);
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (s.v[2807] != 0.0)) {
            s.store_add(1923, 1923, 472);
        }

        s.v[2810] = if (s.v[577] == 0.5) { 1.0 } else { 0.0 };

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2807] != 0.0))) && (s.v[2810] != 0.0)) {
            s.store_sqrt_ad(2636, A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(574))));
        }

        if (((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2807] != 0.0))) && (!(s.v[2810] != 0.0))) {
            s.store_ad(2636, &A::pow(A::sub_from_scalar(1.0, A::mul(s.ad_value(2628), s.ad_value(574))), s.ad_value(577)));
        }

        if ((((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) && (!(s.v[2790] != 0.0))) && (!(s.v[2807] != 0.0))) {
            s.store_scale_ad(1923, A::add(A::mul(s.ad_value(586), A::sub_from_scalar(1.0, s.ad_value(2636))), A::mul(s.ad_value(589), A::sub(s.ad_value(833), s.ad_value(2628)))), p.p30);
        }

        if ((s.v[2665] != 0.0) && (!(s.v[2666] != 0.0))) {
            s.store_add_ad(849, A::add(A::mul(s.ad_value(673), s.ad_value(1915)), A::mul(s.ad_value(674), s.ad_value(1916))), A::mul(s.ad_value(675), s.ad_value(1917)));
        }

        s.v[1942] = 0.0;

        s.v[1943] = 0.0;

        s.v[1944] = 0.0;

        s.v[1945] = 0.0;

        s.v[1946] = 0.0;

        s.v[1947] = 0.0;

        s.v[1948] = 0.0;

        s.v[1949] = 0.0;

        s.v[1950] = 0.0;

        s.v[1951] = 0.0;

        s.v[1952] = 0.0;

        s.v[1953] = 0.0;

        s.v[1954] = 0.0;

        s.v[1955] = 0.0;

        s.v[1956] = 0.0;

        s.v[1957] = 0.0;

        s.v[1958] = 0.0;

        s.v[1959] = 0.0;

        s.v[2811] = if (s.v[1] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[2811] != 0.0) {
            s.store_scalar(1988, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1992, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1986, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1987, 0.0);
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
        if (s.v[2811] != 0.0) {
            s.store_scalar(1993, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1969, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1970, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1971, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1972, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1973, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1974, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1975, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1976, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1977, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1960, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1961, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1962, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1963, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1964, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1965, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1966, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1967, 0.0);
        }

        if (s.v[2811] != 0.0) {
            s.store_scalar(1968, 0.0);
        }

        s.v[2812] = if (s.v[1890] > 0.0) { 1.0 } else { 0.0 };

        s.v[2813] = if (s.v[1] == 1.0) { 1.0 } else { 0.0 };

        if (((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.5, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2814] = if (((s.v[1960]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) && (s.v[2814] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.v[2815] = if ((((-s.v[1960])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) && (!(s.v[2814] != 0.0))) && (s.v[2815] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1960)));
        }

        s.v[2816] = if ((-s.v[1960]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) && (!(s.v[2814] != 0.0))) && (!(s.v[2815] != 0.0))) && (s.v[2816] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) && (!(s.v[2814] != 0.0))) && (!(s.v[2815] != 0.0))) && (!(s.v[2816] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) && (!(s.v[2814] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0))));
        }

        s.v[2817] = if (s.v[1960] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) && (!(s.v[2814] != 0.0))) && (s.v[2817] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (s.v[2813] != 0.0)) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
        }

        s.v[2818] = if (s.v[1] == 2.0) { 1.0 } else { 0.0 };

        if ((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.3333333333333333, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2819] = if (((s.v[1960]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (s.v[2819] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.v[2820] = if ((((-s.v[1960])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2819] != 0.0))) && (s.v[2820] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1960)));
        }

        s.v[2821] = if ((-s.v[1960]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2819] != 0.0))) && (!(s.v[2820] != 0.0))) && (s.v[2821] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2819] != 0.0))) && (!(s.v[2820] != 0.0))) && (!(s.v[2821] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2819] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0))));
        }

        s.v[2822] = if (s.v[1960] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2819] != 0.0))) && (s.v[2822] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
        }

        if ((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) {
            s.store_add_ad_rhs(1961, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.6666666666666666, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2823] = if (((s.v[1961]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (s.v[2823] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1961), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1961), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1961), 0.16666666666666666)))));
        }

        s.v[2824] = if ((((-s.v[1961])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2823] != 0.0))) && (s.v[2824] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1961)));
        }

        s.v[2825] = if ((-s.v[1961]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2823] != 0.0))) && (!(s.v[2824] != 0.0))) && (s.v[2825] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2823] != 0.0))) && (!(s.v[2824] != 0.0))) && (!(s.v[2825] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2823] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0))));
        }

        s.v[2826] = if (s.v[1961] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (!(s.v[2823] != 0.0))) && (s.v[2826] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) {
            s.store_sub_ad_lhs(1943, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1961))), 1996);
        }

        s.v[2827] = if (s.v[831] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (s.v[2827] != 0.0)) {
            s.copy_ad(2027, 1942);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (s.v[2827] != 0.0)) {
            s.copy_ad(1942, 1943);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (s.v[2818] != 0.0)) && (s.v[2827] != 0.0)) {
            s.copy_ad(1943, 2027);
        }

        s.v[2828] = if (s.v[1] == 3.0) { 1.0 } else { 0.0 };

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.25, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2829] = if (((s.v[1960]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (s.v[2829] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.v[2830] = if ((((-s.v[1960])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2829] != 0.0))) && (s.v[2830] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1960)));
        }

        s.v[2831] = if ((-s.v[1960]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2829] != 0.0))) && (!(s.v[2830] != 0.0))) && (s.v[2831] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2829] != 0.0))) && (!(s.v[2830] != 0.0))) && (!(s.v[2831] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2829] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0))));
        }

        s.v[2832] = if (s.v[1960] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2829] != 0.0))) && (s.v[2832] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) {
            s.store_add_ad_rhs(1961, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.5, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2833] = if (((s.v[1961]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (s.v[2833] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1961), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1961), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1961), 0.16666666666666666)))));
        }

        s.v[2834] = if ((((-s.v[1961])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2833] != 0.0))) && (s.v[2834] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1961)));
        }

        s.v[2835] = if ((-s.v[1961]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2833] != 0.0))) && (!(s.v[2834] != 0.0))) && (s.v[2835] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2833] != 0.0))) && (!(s.v[2834] != 0.0))) && (!(s.v[2835] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2833] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0))));
        }

        s.v[2836] = if (s.v[1961] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2833] != 0.0))) && (s.v[2836] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) {
            s.store_sub_ad_lhs(1943, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1961))), 1996);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) {
            s.store_add_ad_rhs(1962, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.75, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2837] = if (((s.v[1962]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (s.v[2837] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1962), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1962), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1962), 0.16666666666666666)))));
        }

        s.v[2838] = if ((((-s.v[1962])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2837] != 0.0))) && (s.v[2838] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1962)));
        }

        s.v[2839] = if ((-s.v[1962]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2837] != 0.0))) && (!(s.v[2838] != 0.0))) && (s.v[2839] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2837] != 0.0))) && (!(s.v[2838] != 0.0))) && (!(s.v[2839] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2837] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0))));
        }

        s.v[2840] = if (s.v[1962] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (!(s.v[2837] != 0.0))) && (s.v[2840] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) {
            s.store_sub_ad_lhs(1944, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1962))), 1996);
        }

        s.v[2841] = if (s.v[831] < 0.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (s.v[2841] != 0.0)) {
            s.copy_ad(2027, 1942);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (s.v[2841] != 0.0)) {
            s.copy_ad(1942, 1944);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (s.v[2828] != 0.0)) && (s.v[2841] != 0.0)) {
            s.copy_ad(1944, 2027);
        }

        s.v[2842] = if (s.v[1] == 5.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.16666666666666666, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2843] = if (((s.v[1960]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2843] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.v[2844] = if ((((-s.v[1960])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2843] != 0.0))) && (s.v[2844] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1960)));
        }

        s.v[2845] = if ((-s.v[1960]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2843] != 0.0))) && (!(s.v[2844] != 0.0))) && (s.v[2845] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2843] != 0.0))) && (!(s.v[2844] != 0.0))) && (!(s.v[2845] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2843] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0))));
        }

        s.v[2846] = if (s.v[1960] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2843] != 0.0))) && (s.v[2846] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_add_ad_rhs(1961, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.3333333333333333, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2847] = if (((s.v[1961]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2847] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1961), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1961), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1961), 0.16666666666666666)))));
        }

        s.v[2848] = if ((((-s.v[1961])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2847] != 0.0))) && (s.v[2848] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1961)));
        }

        s.v[2849] = if ((-s.v[1961]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2847] != 0.0))) && (!(s.v[2848] != 0.0))) && (s.v[2849] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2847] != 0.0))) && (!(s.v[2848] != 0.0))) && (!(s.v[2849] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2847] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0))));
        }

        s.v[2850] = if (s.v[1961] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2847] != 0.0))) && (s.v[2850] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_sub_ad_lhs(1943, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1961))), 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_add_ad_rhs(1962, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.5, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2851] = if (((s.v[1962]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2851] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1962), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1962), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1962), 0.16666666666666666)))));
        }

        s.v[2852] = if ((((-s.v[1962])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2851] != 0.0))) && (s.v[2852] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1962)));
        }

        s.v[2853] = if ((-s.v[1962]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2851] != 0.0))) && (!(s.v[2852] != 0.0))) && (s.v[2853] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2851] != 0.0))) && (!(s.v[2852] != 0.0))) && (!(s.v[2853] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2851] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0))));
        }

        s.v[2854] = if (s.v[1962] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2851] != 0.0))) && (s.v[2854] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_sub_ad_lhs(1944, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1962))), 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_add_ad_rhs(1963, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.6666666666666666, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2855] = if (((s.v[1963]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2855] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1963), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1963), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1963), 0.16666666666666666)))));
        }

        s.v[2856] = if ((((-s.v[1963])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2855] != 0.0))) && (s.v[2856] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1963)));
        }

        s.v[2857] = if ((-s.v[1963]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2855] != 0.0))) && (!(s.v[2856] != 0.0))) && (s.v[2857] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2855] != 0.0))) && (!(s.v[2856] != 0.0))) && (!(s.v[2857] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2855] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0))));
        }

        s.v[2858] = if (s.v[1963] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2855] != 0.0))) && (s.v[2858] != 0.0)) {
            s.store_neg(1996, 1996);
        }

    }

    pub(super) fn stamp_transient_block_43(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_sub_ad_lhs(1945, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1963))), 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_add_ad_rhs(1964, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.8333333333333333, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2859] = if (((s.v[1964]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2859] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1964), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1964), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1964), 0.16666666666666666)))));
        }

        s.v[2860] = if ((((-s.v[1964])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2859] != 0.0))) && (s.v[2860] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1964)));
        }

        s.v[2861] = if ((-s.v[1964]) < 0.0) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2859] != 0.0))) && (!(s.v[2860] != 0.0))) && (s.v[2861] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2859] != 0.0))) && (!(s.v[2860] != 0.0))) && (!(s.v[2861] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2859] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0))));
        }

        s.v[2862] = if (s.v[1964] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (!(s.v[2859] != 0.0))) && (s.v[2862] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) {
            s.store_sub_ad_lhs(1946, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1964))), 1996);
        }

        s.v[2863] = if (s.v[831] < 0.0) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2863] != 0.0)) {
            s.copy_ad(2027, 1942);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2863] != 0.0)) {
            s.copy_ad(1942, 1946);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2863] != 0.0)) {
            s.copy_ad(1946, 2027);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2863] != 0.0)) {
            s.copy_ad(2027, 1943);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2863] != 0.0)) {
            s.copy_ad(1943, 1945);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (s.v[2842] != 0.0)) && (s.v[2863] != 0.0)) {
            s.copy_ad(1945, 2027);
        }

        s.v[2864] = if (s.v[1] == 9.0) { 1.0 } else { 0.0 };

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_add_ad_rhs(1960, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.1, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2865] = if (((s.v[1960]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2865] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1960), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1960), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1960), 0.16666666666666666)))));
        }

        s.v[2866] = if ((((-s.v[1960])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2865] != 0.0))) && (s.v[2866] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1960)));
        }

        s.v[2867] = if ((-s.v[1960]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2865] != 0.0))) && (!(s.v[2866] != 0.0))) && (s.v[2867] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1960))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2865] != 0.0))) && (!(s.v[2866] != 0.0))) && (!(s.v[2867] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1960)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2865] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1960)), (-1.0))));
        }

        s.v[2868] = if (s.v[1960] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2865] != 0.0))) && (s.v[2868] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_sub_ad_lhs(1942, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1960))), 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_add_ad_rhs(1961, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.2, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2869] = if (((s.v[1961]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2869] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1961), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1961), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1961), 0.16666666666666666)))));
        }

        s.v[2870] = if ((((-s.v[1961])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2869] != 0.0))) && (s.v[2870] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1961)));
        }

        s.v[2871] = if ((-s.v[1961]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2869] != 0.0))) && (!(s.v[2870] != 0.0))) && (s.v[2871] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1961))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2869] != 0.0))) && (!(s.v[2870] != 0.0))) && (!(s.v[2871] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1961)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2869] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1961)), (-1.0))));
        }

        s.v[2872] = if (s.v[1961] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2869] != 0.0))) && (s.v[2872] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_sub_ad_lhs(1943, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1961))), 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_add_ad_rhs(1962, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.3, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2873] = if (((s.v[1962]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2873] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1962), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1962), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1962), 0.16666666666666666)))));
        }

        s.v[2874] = if ((((-s.v[1962])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2873] != 0.0))) && (s.v[2874] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1962)));
        }

        s.v[2875] = if ((-s.v[1962]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2873] != 0.0))) && (!(s.v[2874] != 0.0))) && (s.v[2875] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1962))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2873] != 0.0))) && (!(s.v[2874] != 0.0))) && (!(s.v[2875] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1962)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2873] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1962)), (-1.0))));
        }

        s.v[2876] = if (s.v[1962] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2873] != 0.0))) && (s.v[2876] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_sub_ad_lhs(1944, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1962))), 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_add_ad_rhs(1963, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.4, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2877] = if (((s.v[1963]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2877] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1963), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1963), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1963), 0.16666666666666666)))));
        }

        s.v[2878] = if ((((-s.v[1963])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2877] != 0.0))) && (s.v[2878] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1963)));
        }

        s.v[2879] = if ((-s.v[1963]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2877] != 0.0))) && (!(s.v[2878] != 0.0))) && (s.v[2879] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1963))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2877] != 0.0))) && (!(s.v[2878] != 0.0))) && (!(s.v[2879] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1963)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2877] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1963)), (-1.0))));
        }

        s.v[2880] = if (s.v[1963] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2877] != 0.0))) && (s.v[2880] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_sub_ad_lhs(1945, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1963))), 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_add_ad_rhs(1964, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.5, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2881] = if (((s.v[1964]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2881] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1964), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1964), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1964), 0.16666666666666666)))));
        }

        s.v[2882] = if ((((-s.v[1964])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2881] != 0.0))) && (s.v[2882] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1964)));
        }

        s.v[2883] = if ((-s.v[1964]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2881] != 0.0))) && (!(s.v[2882] != 0.0))) && (s.v[2883] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1964))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2881] != 0.0))) && (!(s.v[2882] != 0.0))) && (!(s.v[2883] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1964)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2881] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1964)), (-1.0))));
        }

        s.v[2884] = if (s.v[1964] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2881] != 0.0))) && (s.v[2884] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_sub_ad_lhs(1946, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1964))), 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_add_ad_rhs(1965, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.6, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2885] = if (((s.v[1965]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2885] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1965), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1965), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1965), 0.16666666666666666)))));
        }

        s.v[2886] = if ((((-s.v[1965])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2885] != 0.0))) && (s.v[2886] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1965)));
        }

        s.v[2887] = if ((-s.v[1965]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2885] != 0.0))) && (!(s.v[2886] != 0.0))) && (s.v[2887] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1965))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1965))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1965))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2885] != 0.0))) && (!(s.v[2886] != 0.0))) && (!(s.v[2887] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1965)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1965)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1965)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2885] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1965)), (-1.0))));
        }

        s.v[2888] = if (s.v[1965] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2885] != 0.0))) && (s.v[2888] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_sub_ad_lhs(1947, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1965))), 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_add_ad_rhs(1966, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.7, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2889] = if (((s.v[1966]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2889] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1966), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1966), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1966), 0.16666666666666666)))));
        }

        s.v[2890] = if ((((-s.v[1966])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2889] != 0.0))) && (s.v[2890] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1966)));
        }

        s.v[2891] = if ((-s.v[1966]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2889] != 0.0))) && (!(s.v[2890] != 0.0))) && (s.v[2891] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1966))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1966))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1966))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2889] != 0.0))) && (!(s.v[2890] != 0.0))) && (!(s.v[2891] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1966)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1966)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1966)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2889] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1966)), (-1.0))));
        }

        s.v[2892] = if (s.v[1966] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2889] != 0.0))) && (s.v[2892] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_sub_ad_lhs(1948, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1966))), 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_add_ad_rhs(1967, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.8, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2893] = if (((s.v[1967]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2893] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1967), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1967), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1967), 0.16666666666666666)))));
        }

        s.v[2894] = if ((((-s.v[1967])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2893] != 0.0))) && (s.v[2894] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1967)));
        }

        s.v[2895] = if ((-s.v[1967]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2893] != 0.0))) && (!(s.v[2894] != 0.0))) && (s.v[2895] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1967))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1967))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1967))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2893] != 0.0))) && (!(s.v[2894] != 0.0))) && (!(s.v[2895] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1967)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1967)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1967)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2893] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1967)), (-1.0))));
        }

        s.v[2896] = if (s.v[1967] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2893] != 0.0))) && (s.v[2896] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_sub_ad_lhs(1949, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1967))), 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_add_ad_rhs(1968, 1934, A::mul(A::mul(s.ad_value(1909), A::sub_from_scalar(1.0, A::sqrt(A::sub_from_scalar(1.0, A::mul(A::div(A::scale(s.ad_value(1893), 2.0), s.ad_value(1909)), A::sub_from_scalar(0.9, s.ad_value(1936))))))), s.ad_value(1932)));
        }

        s.v[2897] = if (((s.v[1968]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2897] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(1968), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1968), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1968), 0.16666666666666666)))));
        }

        s.v[2898] = if ((((-s.v[1968])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2897] != 0.0))) && (s.v[2898] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1968)));
        }

        s.v[2899] = if ((-s.v[1968]) < 0.0) { 1.0 } else { 0.0 };

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2897] != 0.0))) && (!(s.v[2898] != 0.0))) && (s.v[2899] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1968))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1968))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1968))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2897] != 0.0))) && (!(s.v[2898] != 0.0))) && (!(s.v[2899] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1968)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1968)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1968)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2897] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1968)), (-1.0))));
        }

        s.v[2900] = if (s.v[1968] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (!(s.v[2897] != 0.0))) && (s.v[2900] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if (((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) {
            s.store_sub_ad_lhs(1950, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1968))), 1996);
        }

        s.v[2901] = if (s.v[831] < 0.0) { 1.0 } else { 0.0 };

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2901] != 0.0)) {
            s.copy_ad(2027, 1942);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2901] != 0.0)) {
            s.copy_ad(1942, 1950);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2901] != 0.0)) {
            s.copy_ad(1950, 2027);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2901] != 0.0)) {
            s.copy_ad(2027, 1943);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2901] != 0.0)) {
            s.copy_ad(1943, 1949);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2901] != 0.0)) {
            s.copy_ad(1949, 2027);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2901] != 0.0)) {
            s.copy_ad(2027, 1944);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2901] != 0.0)) {
            s.copy_ad(1944, 1948);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2901] != 0.0)) {
            s.copy_ad(1948, 2027);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2901] != 0.0)) {
            s.copy_ad(2027, 1945);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2901] != 0.0)) {
            s.copy_ad(1945, 1947);
        }

        if ((((((((s.v[2811] != 0.0) && (s.v[2812] != 0.0)) && (!(s.v[2813] != 0.0))) && (!(s.v[2818] != 0.0))) && (!(s.v[2828] != 0.0))) && (!(s.v[2842] != 0.0))) && (s.v[2864] != 0.0)) && (s.v[2901] != 0.0)) {
            s.copy_ad(1947, 2027);
        }

        s.v[1983] = 0.0;

        s.v[1984] = 0.0;

        s.v[1978] = 0.0;

        s.v[1979] = 0.0;

        s.v[2902] = if (s.v[1] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[2902] != 0.0) {
            s.store_sub_ad_rhs(1983, 1934, A::mul(A::mul(A::scale(s.ad_value(831), 0.5), s.ad_value(1893)), s.ad_value(1932)));
        }

        if (s.v[2902] != 0.0) {
            s.store_add_ad_rhs(1984, 1934, A::mul(A::mul(A::scale(s.ad_value(831), 0.5), s.ad_value(1893)), s.ad_value(1932)));
        }

        if (s.v[2902] != 0.0) {
            s.store_scalar(1978, 0.0);
        }

        if (s.v[2902] != 0.0) {
            s.store_scalar(1979, 0.0);
        }

        s.v[2903] = if (s.v[1983] > 0.0) { 1.0 } else { 0.0 };

        s.v[2904] = if (((s.v[1983]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_44(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((s.v[2902] != 0.0) && (s.v[2903] != 0.0)) && (s.v[2904] != 0.0)) {
            s.store_mul_ad(1997, A::mul(A::scale(s.ad_value(1983), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1983), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1983), 0.16666666666666666)))));
        }

        s.v[2905] = if ((((-s.v[1983])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2902] != 0.0) && (s.v[2903] != 0.0)) && (!(s.v[2904] != 0.0))) && (s.v[2905] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1983)));
        }

        s.v[2906] = if ((-s.v[1983]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2902] != 0.0) && (s.v[2903] != 0.0)) && (!(s.v[2904] != 0.0))) && (!(s.v[2905] != 0.0))) && (s.v[2906] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1983))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1983))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1983))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2902] != 0.0) && (s.v[2903] != 0.0)) && (!(s.v[2904] != 0.0))) && (!(s.v[2905] != 0.0))) && (!(s.v[2906] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1983)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1983)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1983)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2902] != 0.0) && (s.v[2903] != 0.0)) && (!(s.v[2904] != 0.0))) {
            s.store_mul_ad_rhs(1997, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1983)), (-1.0))));
        }

        s.v[2907] = if (s.v[1983] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((s.v[2902] != 0.0) && (s.v[2903] != 0.0)) && (!(s.v[2904] != 0.0))) && (s.v[2907] != 0.0)) {
            s.store_neg(1997, 1997);
        }

        if ((s.v[2902] != 0.0) && (s.v[2903] != 0.0)) {
            s.store_sub_ad_lhs(1978, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1983))), 1997);
        }

        s.v[2908] = if (s.v[1984] > 0.0) { 1.0 } else { 0.0 };

        s.v[2909] = if (((s.v[1984]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if (((s.v[2902] != 0.0) && (s.v[2908] != 0.0)) && (s.v[2909] != 0.0)) {
            s.store_mul_ad(1997, A::mul(A::scale(s.ad_value(1984), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(1984), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(1984), 0.16666666666666666)))));
        }

        s.v[2910] = if ((((-s.v[1984])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2902] != 0.0) && (s.v[2908] != 0.0)) && (!(s.v[2909] != 0.0))) && (s.v[2910] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(1984)));
        }

        s.v[2911] = if ((-s.v[1984]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2902] != 0.0) && (s.v[2908] != 0.0)) && (!(s.v[2909] != 0.0))) && (!(s.v[2910] != 0.0))) && (s.v[2911] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1984))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1984))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(1984))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2902] != 0.0) && (s.v[2908] != 0.0)) && (!(s.v[2909] != 0.0))) && (!(s.v[2910] != 0.0))) && (!(s.v[2911] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(1984)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(1984)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(1984)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2902] != 0.0) && (s.v[2908] != 0.0)) && (!(s.v[2909] != 0.0))) {
            s.store_mul_ad_rhs(1997, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(1984)), (-1.0))));
        }

        s.v[2912] = if (s.v[1984] > s.v[1933]) { 1.0 } else { 0.0 };

        if ((((s.v[2902] != 0.0) && (s.v[2908] != 0.0)) && (!(s.v[2909] != 0.0))) && (s.v[2912] != 0.0)) {
            s.store_neg(1997, 1997);
        }

        if ((s.v[2902] != 0.0) && (s.v[2908] != 0.0)) {
            s.store_sub_ad_lhs(1979, A::mul(A::neg(s.ad_value(1937)), A::sub(s.ad_value(1890), s.ad_value(1984))), 1997);
        }

        s.store_scale(871, 811, s.v[718]);

        s.store_scale(872, 812, s.v[718]);

        s.store_scale(873, 813, s.v[718]);

        s.store_scale(874, 814, s.v[718]);

        s.store_scale(875, 815, s.v[718]);

        s.store_scale(876, 816, s.v[718]);

        s.store_scale(877, 817, s.v[718]);

        s.v[2913] = if (s.v[831] > 0.0) { 1.0 } else { 0.0 };

        s.v[2914] = if (s.v[300] > 0.0) { 1.0 } else { 0.0 };

        s.v[2915] = if (s.v[301] > 0.0) { 1.0 } else { 0.0 };

        s.v[2916] = if (s.v[302] > 0.0) { 1.0 } else { 0.0 };

        s.v[2917] = if (s.v[303] > 0.0) { 1.0 } else { 0.0 };

        s.v[2918] = if (s.v[304] > 0.0) { 1.0 } else { 0.0 };

        s.v[2919] = if (s.v[305] > 0.0) { 1.0 } else { 0.0 };

        s.v[2920] = if (s.v[306] > 0.0) { 1.0 } else { 0.0 };

        s.store_ad(1969, &A::scale(A::voltage(ctx, &nodes, Some(12), None), s.v[3]));

        s.store_ad(1970, &A::scale(A::voltage(ctx, &nodes, Some(13), None), s.v[3]));

        s.store_ad(1971, &A::scale(A::voltage(ctx, &nodes, Some(14), None), s.v[3]));

        s.store_ad(1972, &A::scale(A::voltage(ctx, &nodes, Some(15), None), s.v[3]));

        s.store_ad(1973, &A::scale(A::voltage(ctx, &nodes, Some(16), None), s.v[3]));

        s.store_ad(1974, &A::scale(A::voltage(ctx, &nodes, Some(17), None), s.v[3]));

        s.store_ad(1975, &A::scale(A::voltage(ctx, &nodes, Some(18), None), s.v[3]));

        s.store_ad(1976, &A::scale(A::voltage(ctx, &nodes, Some(19), None), s.v[3]));

        s.store_ad(1977, &A::scale(A::voltage(ctx, &nodes, Some(20), None), s.v[3]));

        s.v[1995] = 0.0;

        s.v[2921] = if (s.v[1] != 0.0) { 1.0 } else { 0.0 };

        if (s.v[2921] != 0.0) {
            s.store_div_ad(1995, A::mul(A::mul(s.ad_value(307), s.ad_value(1888)), s.ad_value(716)), A::mul(s.ad_value(1904), s.ad_value(1906)));
        }

        if (s.v[2921] != 0.0) {
            s.store_mul_ad_lhs(2018, A::mul(A::square(s.ad_value(1907)), s.ad_value(1888)), 1888);
        }

        s.v[2922] = if (s.v[1] == 1.0) { 1.0 } else { 0.0 };

        if ((s.v[2921] != 0.0) && (s.v[2922] != 0.0)) {
            s.store_sub(1992, 1979, 1978);
        }

        if ((s.v[2921] != 0.0) && (s.v[2922] != 0.0)) {
            s.store_sub_ad(1993, A::scale(A::add(s.ad_value(1978), s.ad_value(1979)), 6.0), A::scale(s.ad_value(1969), 12.0));
        }

        s.v[2923] = if (s.v[1] == 2.0) { 1.0 } else { 0.0 };

        if (((s.v[2921] != 0.0) && (!(s.v[2922] != 0.0))) && (s.v[2923] != 0.0)) {
            s.store_scale_ad(1992, A::sub(A::add(A::sub(A::scale(s.ad_value(1978), (-7.0)), A::scale(s.ad_value(1969), 3.0)), A::scale(s.ad_value(1970), 12.0)), A::scale(s.ad_value(1979), 2.0)), 0.2);
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2922] != 0.0))) && (s.v[2923] != 0.0)) {
            s.store_scale_ad(1993, A::add(A::sub(A::add(A::scale(s.ad_value(1978), (-4.0)), A::scale(s.ad_value(1969), 9.0)), A::scale(s.ad_value(1970), 6.0)), s.ad_value(1979)), ((-18.0) / 5.0));
        }

        s.v[2924] = if (s.v[1] == 3.0) { 1.0 } else { 0.0 };

        if ((((s.v[2921] != 0.0) && (!(s.v[2922] != 0.0))) && (!(s.v[2923] != 0.0))) && (s.v[2924] != 0.0)) {
            s.store_scale_ad(1992, A::add(A::sub(A::add(A::sub(A::scale(s.ad_value(1978), (-13.0)), A::scale(s.ad_value(1969), 6.0)), A::scale(s.ad_value(1970), 24.0)), A::scale(s.ad_value(1971), 6.0)), s.ad_value(1979)), 0.14285714285714285);
        }

        if ((((s.v[2921] != 0.0) && (!(s.v[2922] != 0.0))) && (!(s.v[2923] != 0.0))) && (s.v[2924] != 0.0)) {
            s.store_scale_ad(1993, A::add(A::sub(A::add(A::sub(A::scale(s.ad_value(1978), 180.0), A::scale(s.ad_value(1969), 408.0)), A::scale(s.ad_value(1970), 288.0)), A::scale(s.ad_value(1971), 72.0)), A::scale(s.ad_value(1979), 12.0)), 0.14285714285714285);
        }

        s.v[2925] = if (s.v[1] == 5.0) { 1.0 } else { 0.0 };

        if (((((s.v[2921] != 0.0) && (!(s.v[2922] != 0.0))) && (!(s.v[2923] != 0.0))) && (!(s.v[2924] != 0.0))) && (s.v[2925] != 0.0)) {
            s.store_scale_ad(1992, A::add(A::add(A::sub(A::sub(A::add(A::sub(A::scale(s.ad_value(1978), (-181.0)), A::scale(s.ad_value(1969), 84.0)), A::scale(s.ad_value(1972), 24.0)), A::scale(s.ad_value(1973), 6.0)), A::scale(s.ad_value(1971), 90.0)), s.ad_value(1979)), A::scale(s.ad_value(1970), 336.0)), 0.015384615384615385);
        }

        if (((((s.v[2921] != 0.0) && (!(s.v[2922] != 0.0))) && (!(s.v[2923] != 0.0))) && (!(s.v[2924] != 0.0))) && (s.v[2925] != 0.0)) {
            s.store_scale_ad(1993, A::add(A::sub(A::add(A::add(A::sub(A::sub(A::scale(s.ad_value(1972), 432.0), A::scale(s.ad_value(1973), 108.0)), A::scale(s.ad_value(1971), 1620.0)), A::scale(s.ad_value(1979), 18.0)), A::scale(s.ad_value(1978), 3762.0)), A::scale(s.ad_value(1969), 8532.0)), A::scale(s.ad_value(1970), 6048.0)), 0.015384615384615385);
        }

        s.v[2926] = if (s.v[1] == 9.0) { 1.0 } else { 0.0 };

        if ((((((s.v[2921] != 0.0) && (!(s.v[2922] != 0.0))) && (!(s.v[2923] != 0.0))) && (!(s.v[2924] != 0.0))) && (!(s.v[2925] != 0.0))) && (s.v[2926] != 0.0)) {
            let assign66170_ad_e88035: A = A::sub(A::add(A::sub(A::sub(A::add(A::sub(A::add(A::add(A::scale(s.ad_value(1974), 1680.0), A::scale(s.ad_value(1972), 23400.0)), A::scale(s.ad_value(1979), 5.0)), A::scale(s.ad_value(1971), 87330.0)), A::scale(s.ad_value(1976), 120.0)), A::scale(s.ad_value(1975), 450.0)), A::scale(s.ad_value(1969), 81480.0)), A::scale(s.ad_value(1970), 325920.0)), A::scale(s.ad_value(1978), 175565.0));
            s.store_sub_ad(1992, A::scale(A::sub(assign66170_ad_e88035, A::scale(s.ad_value(1977), 30.0)), 2.6434745829918846e-5), A::scale(s.ad_value(1973), (30.0 / 181.0)));
        }

        if ((((((s.v[2921] != 0.0) && (!(s.v[2922] != 0.0))) && (!(s.v[2923] != 0.0))) && (!(s.v[2924] != 0.0))) && (!(s.v[2925] != 0.0))) && (s.v[2926] != 0.0)) {
            let assign66180_ad_e88100: A = A::sub(A::add(A::add(A::add(A::add(A::sub(A::sub(A::add(A::scale(s.ad_value(1975), (-13500.0)), A::scale(s.ad_value(1972), 702000.0)), A::scale(s.ad_value(1971), 2619900.0)), A::scale(s.ad_value(1969), 13793100.0)), A::scale(s.ad_value(1970), 9777600.0)), A::scale(s.ad_value(1978), 6081750.0)), A::scale(s.ad_value(1979), 150.0)), A::scale(s.ad_value(1976), 3600.0)), A::scale(s.ad_value(1977), 900.0));
            s.store_sub_ad(1993, A::scale(A::add(assign66180_ad_e88100, A::scale(s.ad_value(1974), 50400.0)), 2.6434745829918846e-5), A::scale(s.ad_value(1973), (900.0 / 181.0)));
        }

        if ((((((s.v[2921] != 0.0) && (!(s.v[2922] != 0.0))) && (!(s.v[2923] != 0.0))) && (!(s.v[2924] != 0.0))) && (!(s.v[2925] != 0.0))) && (!(s.v[2926] != 0.0))) {
            s.store_scalar(1992, 0.0);
        }

        if ((((((s.v[2921] != 0.0) && (!(s.v[2922] != 0.0))) && (!(s.v[2923] != 0.0))) && (!(s.v[2924] != 0.0))) && (!(s.v[2925] != 0.0))) && (!(s.v[2926] != 0.0))) {
            s.store_scalar(1993, 0.0);
        }

        if (s.v[2921] != 0.0) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1969), s.ad_value(1937)), 1890);
        }

        s.v[2927] = if (((s.v[2027]) as f64).abs() <= s.v[1941]) { 1.0 } else { 0.0 };

        if ((s.v[2921] != 0.0) && (s.v[2927] != 0.0)) {
            s.store_div(2016, 2027, 1940);
        }

        s.v[2928] = if (s.v[2027] < (-s.v[1941])) { 1.0 } else { 0.0 };

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_neg(1999, 2027);
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_div_ad_lhs(2000, A::scale(s.ad_value(1999), 1.25), 1940);
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_scale_ad(2001, A::sub(A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_add(824, 2002, 2003);
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.v[2929] = if (((s.v[2015]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) && (s.v[2929] != 0.0)) {
            s.store_exp(2005, 2015);
        }

        s.v[2930] = if (s.v[2015] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) && (!(s.v[2929] != 0.0))) && (s.v[2930] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) && (!(s.v[2929] != 0.0))) && (!(s.v[2930] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (s.v[2928] != 0.0)) {
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) {
            s.store_div_from_scalar_ad(1998, 1.0, A::offset(A::scale(s.ad_value(1938), 0.732464877560822), 1.25));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) {
            s.store_mul_ad_lhs(2010, A::offset(A::mul(A::scale(s.ad_value(1940), 1.25), s.ad_value(1998)), (-1.0)), 1998);
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) {
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.v[2931] = if ((((-s.v[2011])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) && (s.v[2931] != 0.0)) {
            s.store_exp_ad(2009, A::neg(s.ad_value(2011)));
        }

        s.v[2932] = if ((-s.v[2011]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) && (!(s.v[2931] != 0.0))) && (s.v[2932] != 0.0)) {
            s.store_div_from_scalar_ad(2009, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) && (!(s.v[2931] != 0.0))) && (!(s.v[2932] != 0.0))) {
            s.store_scale_ad(2009, A::offset(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) {
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.v[2933] = if ((((-s.v[2013])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) && (s.v[2933] != 0.0)) {
            s.store_exp_ad(2005, A::neg(s.ad_value(2013)));
        }

        s.v[2934] = if ((-s.v[2013]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) && (!(s.v[2933] != 0.0))) && (s.v[2934] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) && (!(s.v[2933] != 0.0))) && (!(s.v[2934] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) {
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) {
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

        if (((s.v[2921] != 0.0) && (!(s.v[2927] != 0.0))) && (!(s.v[2928] != 0.0))) {
            s.store_add(2016, 2013, 2014);
        }

        s.v[2935] = if (((s.v[2016]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((s.v[2921] != 0.0) && (s.v[2935] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
        }

        if ((s.v[2921] != 0.0) && (s.v[2935] != 0.0)) {
            s.store_mul_ad(1991, A::scale(s.ad_value(1889), (-0.70710678)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
        }

        if ((s.v[2921] != 0.0) && (s.v[2935] != 0.0)) {
            s.store_mul_ad(1990, A::scale(s.ad_value(1889), (-0.235702)), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.v[2936] = if ((((-s.v[2016])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2921] != 0.0) && (!(s.v[2935] != 0.0))) && (s.v[2936] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(2016)));
        }

        s.v[2937] = if ((-s.v[2016]) < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2921] != 0.0) && (!(s.v[2935] != 0.0))) && (!(s.v[2936] != 0.0))) && (s.v[2937] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2921] != 0.0) && (!(s.v[2935] != 0.0))) && (!(s.v[2936] != 0.0))) && (!(s.v[2937] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2921] != 0.0) && (!(s.v[2935] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0))));
        }

        s.v[2938] = if (s.v[2016] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((s.v[2921] != 0.0) && (!(s.v[2935] != 0.0))) && (s.v[2938] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((s.v[2921] != 0.0) && (!(s.v[2935] != 0.0))) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
        }

        if ((s.v[2921] != 0.0) && (!(s.v[2935] != 0.0))) {
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if (s.v[2921] != 0.0) {
            s.store_sub(1988, 1937, 1991);
        }

        if (s.v[2921] != 0.0) {
            s.store_div_from_scalar(1989, 1.0, 1988);
        }

        if (s.v[2921] != 0.0) {
            s.store_offset_ad(1987, A::mul(s.ad_value(1969), s.ad_value(1989)), (-1.0));
        }

        if (s.v[2921] != 0.0) {
            s.store_mul_ad_lhs(1986, A::sub_from_scalar(1.0, A::mul(A::mul(A::mul(s.ad_value(1969), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989))), 1989);
        }

        if (s.v[2921] != 0.0) {
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
        }

        if (s.v[2921] != 0.0) {
            s.store_mul_ad_lhs(1994, A::mul(A::square(s.ad_value(1992)), s.ad_value(1989)), 1989);
        }

        if (s.v[2921] != 0.0) {
            s.store_mul(1985, 2018, 1994);
        }

        s.v[2939] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2921] != 0.0) && (s.v[2939] != 0.0)) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if (s.v[2921] != 0.0) {
            s.store_sqrt_ad(2027, A::offset(A::scale(s.ad_value(1985), 2.0), 1.0));
        }

        if (s.v[2921] != 0.0) {
            s.store_div_from_scalar_ad(2019, 2.0, A::offset(s.ad_value(2027), 1.0));
        }

        if (s.v[2921] != 0.0) {
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
        }

        if (s.v[2921] != 0.0) {
            s.store_mul_ad_rhs(1951, 2019, A::sub(s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027))));
        }

        if (!(s.v[2921] != 0.0)) {
            s.store_scalar(2018, 0.0);
        }

        s.v[2940] = if (s.v[1] >= 2.0) { 1.0 } else { 0.0 };

        s.v[2941] = if (s.v[1] == 2.0) { 1.0 } else { 0.0 };

        if ((s.v[2940] != 0.0) && (s.v[2941] != 0.0)) {
            s.store_scale_ad(1992, A::add(A::add(A::sub(A::scale(s.ad_value(1978), 2.0), A::scale(s.ad_value(1969), 12.0)), A::scale(s.ad_value(1970), 3.0)), A::scale(s.ad_value(1979), 7.0)), 0.2);
        }

        if ((s.v[2940] != 0.0) && (s.v[2941] != 0.0)) {
            s.store_scale_ad(1993, A::add(A::sub(A::add(A::scale(s.ad_value(1979), (-4.0)), A::scale(s.ad_value(1970), 9.0)), A::scale(s.ad_value(1969), 6.0)), s.ad_value(1978)), ((-18.0) / 5.0));
        }

        s.v[2942] = if (s.v[1] == 3.0) { 1.0 } else { 0.0 };

        if (((s.v[2940] != 0.0) && (!(s.v[2941] != 0.0))) && (s.v[2942] != 0.0)) {
            s.store_sub_ad(1992, A::add(A::sub(A::scale(s.ad_value(1978), 0.5), A::scale(s.ad_value(1969), 3.0)), A::scale(s.ad_value(1971), 3.0)), A::scale(s.ad_value(1979), 0.5));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2941] != 0.0))) && (s.v[2942] != 0.0)) {
            s.store_scale_ad(1993, A::sub(A::add(A::sub(A::add(A::scale(s.ad_value(1978), (-48.0)), A::scale(s.ad_value(1969), 288.0)), A::scale(s.ad_value(1970), 480.0)), A::scale(s.ad_value(1971), 288.0)), A::scale(s.ad_value(1979), 48.0)), 0.14285714285714285);
        }

        s.v[2943] = if (s.v[1] == 5.0) { 1.0 } else { 0.0 };

        if ((((s.v[2940] != 0.0) && (!(s.v[2941] != 0.0))) && (!(s.v[2942] != 0.0))) && (s.v[2943] != 0.0)) {
            s.store_add_ad(1992, A::scale(A::add(A::sub(A::sub(A::scale(s.ad_value(1969), (-291.0)), A::scale(s.ad_value(1970), 6.0)), A::scale(s.ad_value(1972), 84.0)), A::scale(s.ad_value(1973), 21.0)), 0.015384615384615385), A::scale(A::add(A::sub(A::scale(s.ad_value(1971), 630.0), A::scale(s.ad_value(1979), 7.0)), A::scale(s.ad_value(1978), 97.0)), 0.007692307692307693));
        }

        if ((((s.v[2940] != 0.0) && (!(s.v[2941] != 0.0))) && (!(s.v[2942] != 0.0))) && (s.v[2943] != 0.0)) {
            s.store_scale_ad(1993, A::sub(A::add(A::sub(A::sub(A::add(A::add(A::scale(s.ad_value(1972), (-1728.0)), A::scale(s.ad_value(1973), 432.0)), A::scale(s.ad_value(1971), 6480.0)), A::scale(s.ad_value(1979), 72.0)), A::scale(s.ad_value(1978), 1008.0)), A::scale(s.ad_value(1969), 6048.0)), A::scale(s.ad_value(1970), 10152.0)), 0.015384615384615385);
        }

        s.v[2944] = if (s.v[1] == 9.0) { 1.0 } else { 0.0 };

        if (((((s.v[2940] != 0.0) && (!(s.v[2941] != 0.0))) && (!(s.v[2942] != 0.0))) && (!(s.v[2943] != 0.0))) && (s.v[2944] != 0.0)) {
            let assign67050_ad_e89539: A = A::scale(A::sub(A::add(A::sub(A::add(A::sub(A::add(A::sub(A::scale(s.ad_value(1974), (-5880.0)), A::scale(s.ad_value(1972), 81900.0)), A::scale(s.ad_value(1971), 305655.0)), A::scale(s.ad_value(1976), 420.0)), A::scale(s.ad_value(1977), 105.0)), A::scale(s.ad_value(1969), 282255.0)), A::scale(s.ad_value(1975), 1575.0)), A::scale(s.ad_value(1970), 5850.0)), 2.6434745829918846e-5);
            s.store_add_ad(1992, A::add(assign67050_ad_e89539, A::scale(s.ad_value(1973), (105.0 / 181.0))), A::scale(A::sub(A::scale(s.ad_value(1978), 94085.0), A::scale(s.ad_value(1979), 35.0)), 1.3217372914959423e-5));
        }

        if (((((s.v[2940] != 0.0) && (!(s.v[2941] != 0.0))) && (!(s.v[2942] != 0.0))) && (!(s.v[2943] != 0.0))) && (s.v[2944] != 0.0)) {
            let assign67060_ad_e89604: A = A::add(A::sub(A::sub(A::sub(A::sub(A::add(A::sub(A::add(A::scale(s.ad_value(1969), 9777600.0), A::scale(s.ad_value(1975), 54000.0)), A::scale(s.ad_value(1972), 2808000.0)), A::scale(s.ad_value(1971), 10479600.0)), A::scale(s.ad_value(1970), 16413000.0)), A::scale(s.ad_value(1978), 1629600.0)), A::scale(s.ad_value(1979), 600.0)), A::scale(s.ad_value(1976), 14400.0)), A::scale(s.ad_value(1977), 3600.0));
            s.store_add_ad(1993, A::scale(A::sub(assign67060_ad_e89604, A::scale(s.ad_value(1974), 201600.0)), 2.6434745829918846e-5), A::scale(s.ad_value(1973), (3600.0 * 0.0055248618784530384)));
        }

        if (((((s.v[2940] != 0.0) && (!(s.v[2941] != 0.0))) && (!(s.v[2942] != 0.0))) && (!(s.v[2943] != 0.0))) && (!(s.v[2944] != 0.0))) {
            s.store_scalar(1992, 0.0);
        }

        if (((((s.v[2940] != 0.0) && (!(s.v[2941] != 0.0))) && (!(s.v[2942] != 0.0))) && (!(s.v[2943] != 0.0))) && (!(s.v[2944] != 0.0))) {
            s.store_scalar(1993, 0.0);
        }

    }

    pub(super) fn stamp_transient_block_45(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (s.v[2940] != 0.0) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1970), s.ad_value(1937)), 1890);
        }

        s.v[2945] = if (((s.v[2027]) as f64).abs() <= s.v[1941]) { 1.0 } else { 0.0 };

        if ((s.v[2940] != 0.0) && (s.v[2945] != 0.0)) {
            s.store_div(2016, 2027, 1940);
        }

        s.v[2946] = if (s.v[2027] < (-s.v[1941])) { 1.0 } else { 0.0 };

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_neg(1999, 2027);
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_div_ad_lhs(2000, A::scale(s.ad_value(1999), 1.25), 1940);
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_scale_ad(2001, A::sub(A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_add(824, 2002, 2003);
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.v[2947] = if (((s.v[2015]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) && (s.v[2947] != 0.0)) {
            s.store_exp(2005, 2015);
        }

        s.v[2948] = if (s.v[2015] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) && (!(s.v[2947] != 0.0))) && (s.v[2948] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) && (!(s.v[2947] != 0.0))) && (!(s.v[2948] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (s.v[2946] != 0.0)) {
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) {
            s.store_div_from_scalar_ad(1998, 1.0, A::offset(A::scale(s.ad_value(1938), 0.732464877560822), 1.25));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) {
            s.store_mul_ad_lhs(2010, A::offset(A::mul(A::scale(s.ad_value(1940), 1.25), s.ad_value(1998)), (-1.0)), 1998);
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) {
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.v[2949] = if ((((-s.v[2011])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) && (s.v[2949] != 0.0)) {
            s.store_exp_ad(2009, A::neg(s.ad_value(2011)));
        }

        s.v[2950] = if ((-s.v[2011]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) && (!(s.v[2949] != 0.0))) && (s.v[2950] != 0.0)) {
            s.store_div_from_scalar_ad(2009, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) && (!(s.v[2949] != 0.0))) && (!(s.v[2950] != 0.0))) {
            s.store_scale_ad(2009, A::offset(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) {
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.v[2951] = if ((((-s.v[2013])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) && (s.v[2951] != 0.0)) {
            s.store_exp_ad(2005, A::neg(s.ad_value(2013)));
        }

        s.v[2952] = if ((-s.v[2013]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) && (!(s.v[2951] != 0.0))) && (s.v[2952] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) && (!(s.v[2951] != 0.0))) && (!(s.v[2952] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) {
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) {
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

        if (((s.v[2940] != 0.0) && (!(s.v[2945] != 0.0))) && (!(s.v[2946] != 0.0))) {
            s.store_add(2016, 2013, 2014);
        }

        s.v[2953] = if (((s.v[2016]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((s.v[2940] != 0.0) && (s.v[2953] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
        }

        if ((s.v[2940] != 0.0) && (s.v[2953] != 0.0)) {
            s.store_mul_ad(1991, A::scale(s.ad_value(1889), (-0.70710678)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
        }

        if ((s.v[2940] != 0.0) && (s.v[2953] != 0.0)) {
            s.store_mul_ad(1990, A::scale(s.ad_value(1889), (-0.235702)), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.v[2954] = if ((((-s.v[2016])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2940] != 0.0) && (!(s.v[2953] != 0.0))) && (s.v[2954] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(2016)));
        }

        s.v[2955] = if ((-s.v[2016]) < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2940] != 0.0) && (!(s.v[2953] != 0.0))) && (!(s.v[2954] != 0.0))) && (s.v[2955] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2940] != 0.0) && (!(s.v[2953] != 0.0))) && (!(s.v[2954] != 0.0))) && (!(s.v[2955] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2940] != 0.0) && (!(s.v[2953] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0))));
        }

        s.v[2956] = if (s.v[2016] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((s.v[2940] != 0.0) && (!(s.v[2953] != 0.0))) && (s.v[2956] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((s.v[2940] != 0.0) && (!(s.v[2953] != 0.0))) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
        }

        if ((s.v[2940] != 0.0) && (!(s.v[2953] != 0.0))) {
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if (s.v[2940] != 0.0) {
            s.store_sub(1988, 1937, 1991);
        }

        if (s.v[2940] != 0.0) {
            s.store_div_from_scalar(1989, 1.0, 1988);
        }

        if (s.v[2940] != 0.0) {
            s.store_offset_ad(1987, A::mul(s.ad_value(1970), s.ad_value(1989)), (-1.0));
        }

        if (s.v[2940] != 0.0) {
            s.store_mul_ad_lhs(1986, A::sub_from_scalar(1.0, A::mul(A::mul(A::mul(s.ad_value(1970), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989))), 1989);
        }

        if (s.v[2940] != 0.0) {
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
        }

        if (s.v[2940] != 0.0) {
            s.store_mul_ad_lhs(1994, A::mul(A::square(s.ad_value(1992)), s.ad_value(1989)), 1989);
        }

        if (s.v[2940] != 0.0) {
            s.store_mul(1985, 2018, 1994);
        }

        s.v[2957] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2940] != 0.0) && (s.v[2957] != 0.0)) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if (s.v[2940] != 0.0) {
            s.store_sqrt_ad(2027, A::offset(A::scale(s.ad_value(1985), 2.0), 1.0));
        }

        if (s.v[2940] != 0.0) {
            s.store_div_from_scalar_ad(2019, 2.0, A::offset(s.ad_value(2027), 1.0));
        }

        if (s.v[2940] != 0.0) {
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
        }

        if (s.v[2940] != 0.0) {
            s.store_mul_ad_rhs(1952, 2019, A::sub(s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027))));
        }

        s.v[2958] = if (s.v[1] >= 3.0) { 1.0 } else { 0.0 };

        s.v[2959] = if (s.v[1] == 3.0) { 1.0 } else { 0.0 };

        if ((s.v[2958] != 0.0) && (s.v[2959] != 0.0)) {
            s.store_scale_ad(1992, A::sub(A::add(A::sub(A::add(A::scale(s.ad_value(1979), 13.0), A::scale(s.ad_value(1971), 6.0)), A::scale(s.ad_value(1970), 24.0)), A::scale(s.ad_value(1969), 6.0)), s.ad_value(1978)), 0.14285714285714285);
        }

        if ((s.v[2958] != 0.0) && (s.v[2959] != 0.0)) {
            s.store_scale_ad(1993, A::add(A::sub(A::add(A::sub(A::scale(s.ad_value(1979), 180.0), A::scale(s.ad_value(1971), 408.0)), A::scale(s.ad_value(1970), 288.0)), A::scale(s.ad_value(1969), 72.0)), A::scale(s.ad_value(1978), 12.0)), 0.14285714285714285);
        }

        s.v[2960] = if (s.v[1] == 5.0) { 1.0 } else { 0.0 };

        if (((s.v[2958] != 0.0) && (!(s.v[2959] != 0.0))) && (s.v[2960] != 0.0)) {
            s.store_scale_ad(1992, A::sub(A::add(A::sub(A::add(A::sub(s.ad_value(1979), A::scale(s.ad_value(1973), 6.0)), A::scale(s.ad_value(1972), 24.0)), A::scale(s.ad_value(1970), 24.0)), A::scale(s.ad_value(1969), 6.0)), s.ad_value(1978)), 0.2);
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2959] != 0.0))) && (s.v[2960] != 0.0)) {
            s.store_scale_ad(1993, A::add(A::sub(A::sub(A::scale(A::add(s.ad_value(1972), s.ad_value(1970)), 1296.0), A::scale(A::add(s.ad_value(1973), s.ad_value(1969)), 324.0)), A::scale(s.ad_value(1971), 2052.0)), A::scale(A::add(s.ad_value(1979), s.ad_value(1978)), 54.0)), 0.07692307692307693);
        }

        s.v[2961] = if (s.v[1] == 9.0) { 1.0 } else { 0.0 };

        if ((((s.v[2958] != 0.0) && (!(s.v[2959] != 0.0))) && (!(s.v[2960] != 0.0))) && (s.v[2961] != 0.0)) {
            let assign67890_ad_e90954: A = A::sub(A::add(A::sub(A::sub(A::add(A::sub(A::add(A::add(A::scale(s.ad_value(1974), 21840.0), A::scale(s.ad_value(1972), 304200.0)), A::scale(s.ad_value(1979), 65.0)), A::scale(s.ad_value(1971), 420.0)), A::scale(s.ad_value(1976), 1560.0)), A::scale(s.ad_value(1978), 12605.0)), A::scale(s.ad_value(1977), 390.0)), A::scale(s.ad_value(1969), 75630.0)), A::scale(s.ad_value(1975), 5850.0));
            s.store_sub_ad(1992, A::scale(A::sub(assign67890_ad_e90954, A::scale(s.ad_value(1970), 302520.0)), 2.6434745829918846e-5), A::scale(s.ad_value(1973), (390.0 / 181.0)));
        }

        if ((((s.v[2958] != 0.0) && (!(s.v[2959] != 0.0))) && (!(s.v[2960] != 0.0))) && (s.v[2961] != 0.0)) {
            let assign67900_ad_e91013: A = A::sub(A::add(A::add(A::add(A::add(A::sub(A::add(A::sub(A::scale(s.ad_value(1969), (-2619900.0)), A::scale(s.ad_value(1975), 202500.0)), A::scale(s.ad_value(1972), 10530000.0)), A::scale(s.ad_value(1971), 16601100.0)), A::scale(s.ad_value(1970), 10479600.0)), A::scale(s.ad_value(1978), 436650.0)), A::scale(s.ad_value(1979), 2250.0)), A::scale(s.ad_value(1976), 54000.0)), A::scale(s.ad_value(1977), 13500.0));
            s.store_sub_ad(1993, A::scale(A::add(assign67900_ad_e91013, A::scale(s.ad_value(1974), 756000.0)), 2.6434745829918846e-5), A::scale(s.ad_value(1973), (13500.0 * 0.0055248618784530384)));
        }

        if ((((s.v[2958] != 0.0) && (!(s.v[2959] != 0.0))) && (!(s.v[2960] != 0.0))) && (!(s.v[2961] != 0.0))) {
            s.store_scalar(1992, 0.0);
        }

        if ((((s.v[2958] != 0.0) && (!(s.v[2959] != 0.0))) && (!(s.v[2960] != 0.0))) && (!(s.v[2961] != 0.0))) {
            s.store_scalar(1993, 0.0);
        }

        if (s.v[2958] != 0.0) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1971), s.ad_value(1937)), 1890);
        }

        s.v[2962] = if (((s.v[2027]) as f64).abs() <= s.v[1941]) { 1.0 } else { 0.0 };

        if ((s.v[2958] != 0.0) && (s.v[2962] != 0.0)) {
            s.store_div(2016, 2027, 1940);
        }

        s.v[2963] = if (s.v[2027] < (-s.v[1941])) { 1.0 } else { 0.0 };

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_neg(1999, 2027);
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_div_ad_lhs(2000, A::scale(s.ad_value(1999), 1.25), 1940);
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_scale_ad(2001, A::sub(A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_add(824, 2002, 2003);
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.v[2964] = if (((s.v[2015]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) && (s.v[2964] != 0.0)) {
            s.store_exp(2005, 2015);
        }

        s.v[2965] = if (s.v[2015] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) && (!(s.v[2964] != 0.0))) && (s.v[2965] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) && (!(s.v[2964] != 0.0))) && (!(s.v[2965] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (s.v[2963] != 0.0)) {
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) {
            s.store_div_from_scalar_ad(1998, 1.0, A::offset(A::scale(s.ad_value(1938), 0.732464877560822), 1.25));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) {
            s.store_mul_ad_lhs(2010, A::offset(A::mul(A::scale(s.ad_value(1940), 1.25), s.ad_value(1998)), (-1.0)), 1998);
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) {
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.v[2966] = if ((((-s.v[2011])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) && (s.v[2966] != 0.0)) {
            s.store_exp_ad(2009, A::neg(s.ad_value(2011)));
        }

        s.v[2967] = if ((-s.v[2011]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) && (!(s.v[2966] != 0.0))) && (s.v[2967] != 0.0)) {
            s.store_div_from_scalar_ad(2009, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) && (!(s.v[2966] != 0.0))) && (!(s.v[2967] != 0.0))) {
            s.store_scale_ad(2009, A::offset(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) {
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.v[2968] = if ((((-s.v[2013])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) && (s.v[2968] != 0.0)) {
            s.store_exp_ad(2005, A::neg(s.ad_value(2013)));
        }

        s.v[2969] = if ((-s.v[2013]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) && (!(s.v[2968] != 0.0))) && (s.v[2969] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) && (!(s.v[2968] != 0.0))) && (!(s.v[2969] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) {
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) {
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

        if (((s.v[2958] != 0.0) && (!(s.v[2962] != 0.0))) && (!(s.v[2963] != 0.0))) {
            s.store_add(2016, 2013, 2014);
        }

        s.v[2970] = if (((s.v[2016]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((s.v[2958] != 0.0) && (s.v[2970] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
        }

        if ((s.v[2958] != 0.0) && (s.v[2970] != 0.0)) {
            s.store_mul_ad(1991, A::scale(s.ad_value(1889), (-0.70710678)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
        }

        if ((s.v[2958] != 0.0) && (s.v[2970] != 0.0)) {
            s.store_mul_ad(1990, A::scale(s.ad_value(1889), (-0.235702)), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.v[2971] = if ((((-s.v[2016])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2958] != 0.0) && (!(s.v[2970] != 0.0))) && (s.v[2971] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(2016)));
        }

        s.v[2972] = if ((-s.v[2016]) < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2958] != 0.0) && (!(s.v[2970] != 0.0))) && (!(s.v[2971] != 0.0))) && (s.v[2972] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2958] != 0.0) && (!(s.v[2970] != 0.0))) && (!(s.v[2971] != 0.0))) && (!(s.v[2972] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2958] != 0.0) && (!(s.v[2970] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0))));
        }

        s.v[2973] = if (s.v[2016] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((s.v[2958] != 0.0) && (!(s.v[2970] != 0.0))) && (s.v[2973] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((s.v[2958] != 0.0) && (!(s.v[2970] != 0.0))) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
        }

    }

    pub(super) fn stamp_transient_block_46(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if ((s.v[2958] != 0.0) && (!(s.v[2970] != 0.0))) {
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if (s.v[2958] != 0.0) {
            s.store_sub(1988, 1937, 1991);
        }

        if (s.v[2958] != 0.0) {
            s.store_div_from_scalar(1989, 1.0, 1988);
        }

        if (s.v[2958] != 0.0) {
            s.store_offset_ad(1987, A::mul(s.ad_value(1971), s.ad_value(1989)), (-1.0));
        }

        if (s.v[2958] != 0.0) {
            s.store_mul_ad_lhs(1986, A::sub_from_scalar(1.0, A::mul(A::mul(A::mul(s.ad_value(1971), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989))), 1989);
        }

        if (s.v[2958] != 0.0) {
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
        }

        if (s.v[2958] != 0.0) {
            s.store_mul_ad_lhs(1994, A::mul(A::square(s.ad_value(1992)), s.ad_value(1989)), 1989);
        }

        if (s.v[2958] != 0.0) {
            s.store_mul(1985, 2018, 1994);
        }

        s.v[2974] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2958] != 0.0) && (s.v[2974] != 0.0)) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if (s.v[2958] != 0.0) {
            s.store_sqrt_ad(2027, A::offset(A::scale(s.ad_value(1985), 2.0), 1.0));
        }

        if (s.v[2958] != 0.0) {
            s.store_div_from_scalar_ad(2019, 2.0, A::offset(s.ad_value(2027), 1.0));
        }

        if (s.v[2958] != 0.0) {
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
        }

        if (s.v[2958] != 0.0) {
            s.store_mul_ad_rhs(1953, 2019, A::sub(s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027))));
        }

        s.v[2975] = if (s.v[1] >= 4.0) { 1.0 } else { 0.0 };

        s.v[2976] = if (s.v[1] == 5.0) { 1.0 } else { 0.0 };

        if ((s.v[2975] != 0.0) && (s.v[2976] != 0.0)) {
            s.store_scale_ad(1992, A::add(A::sub(A::add(A::sub(A::add(A::add(A::scale(s.ad_value(1971), (-630.0)), A::scale(s.ad_value(1972), 12.0)), A::scale(s.ad_value(1973), 582.0)), A::scale(s.ad_value(1979), 97.0)), A::scale(s.ad_value(1978), 7.0)), A::scale(s.ad_value(1969), 42.0)), A::scale(s.ad_value(1970), 168.0)), 0.007692307692307693);
        }

        if ((s.v[2975] != 0.0) && (s.v[2976] != 0.0)) {
            s.store_scale_ad(1993, A::sub(A::add(A::sub(A::sub(A::add(A::add(A::scale(s.ad_value(1972), (-10152.0)), A::scale(s.ad_value(1973), 6048.0)), A::scale(s.ad_value(1971), 6480.0)), A::scale(s.ad_value(1979), 1008.0)), A::scale(s.ad_value(1978), 72.0)), A::scale(s.ad_value(1969), 432.0)), A::scale(s.ad_value(1970), 1728.0)), 0.015384615384615385);
        }

        s.v[2977] = if (s.v[1] == 9.0) { 1.0 } else { 0.0 };

        if (((s.v[2975] != 0.0) && (!(s.v[2976] != 0.0))) && (s.v[2977] != 0.0)) {
            let assign68700_ad_e92310: A = A::scale(A::add(A::add(A::sub(A::add(A::sub(A::sub(A::sub(A::scale(s.ad_value(1974), (-81480.0)), A::scale(s.ad_value(1972), 30.0)), A::scale(s.ad_value(1971), 303975.0)), A::scale(s.ad_value(1976), 5820.0)), A::scale(s.ad_value(1977), 1455.0)), A::scale(s.ad_value(1969), 20265.0)), A::scale(s.ad_value(1975), 21825.0)), A::scale(s.ad_value(1970), 81060.0)), 2.6434745829918846e-5);
            s.store_add_ad(1992, A::add(A::sub(assign68700_ad_e92310, A::scale(s.ad_value(1979), (485.0 / 75658.0))), A::scale(s.ad_value(1973), (1455.0 * 0.0055248618784530384))), A::scale(s.ad_value(1978), (6755.0 * 1.3217372914959423e-5)));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2976] != 0.0))) && (s.v[2977] != 0.0)) {
            let assign68710_ad_e92371: A = A::add(A::sub(A::sub(A::sub(A::sub(A::add(A::sub(A::add(A::scale(s.ad_value(1969), 702000.0), A::scale(s.ad_value(1975), 756000.0)), A::scale(s.ad_value(1972), 16614600.0)), A::scale(s.ad_value(1971), 10530000.0)), A::scale(s.ad_value(1970), 2808000.0)), A::scale(s.ad_value(1978), 117000.0)), A::scale(s.ad_value(1979), 8400.0)), A::scale(s.ad_value(1976), 201600.0)), A::scale(s.ad_value(1977), 50400.0));
            s.store_add_ad(1993, A::scale(A::sub(assign68710_ad_e92371, A::scale(s.ad_value(1974), 2822400.0)), 2.6434745829918846e-5), A::scale(s.ad_value(1973), (50400.0 * 0.0055248618784530384)));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2976] != 0.0))) && (!(s.v[2977] != 0.0))) {
            s.store_scalar(1992, 0.0);
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2976] != 0.0))) && (!(s.v[2977] != 0.0))) {
            s.store_scalar(1993, 0.0);
        }

        if (s.v[2975] != 0.0) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1972), s.ad_value(1937)), 1890);
        }

        s.v[2978] = if (((s.v[2027]) as f64).abs() <= s.v[1941]) { 1.0 } else { 0.0 };

        if ((s.v[2975] != 0.0) && (s.v[2978] != 0.0)) {
            s.store_div(2016, 2027, 1940);
        }

        s.v[2979] = if (s.v[2027] < (-s.v[1941])) { 1.0 } else { 0.0 };

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_neg(1999, 2027);
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_div_ad_lhs(2000, A::scale(s.ad_value(1999), 1.25), 1940);
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_scale_ad(2001, A::sub(A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_add(824, 2002, 2003);
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.v[2980] = if (((s.v[2015]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) && (s.v[2980] != 0.0)) {
            s.store_exp(2005, 2015);
        }

        s.v[2981] = if (s.v[2015] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) && (!(s.v[2980] != 0.0))) && (s.v[2981] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) && (!(s.v[2980] != 0.0))) && (!(s.v[2981] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (s.v[2979] != 0.0)) {
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) {
            s.store_div_from_scalar_ad(1998, 1.0, A::offset(A::scale(s.ad_value(1938), 0.732464877560822), 1.25));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) {
            s.store_mul_ad_lhs(2010, A::offset(A::mul(A::scale(s.ad_value(1940), 1.25), s.ad_value(1998)), (-1.0)), 1998);
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) {
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.v[2982] = if ((((-s.v[2011])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) && (s.v[2982] != 0.0)) {
            s.store_exp_ad(2009, A::neg(s.ad_value(2011)));
        }

        s.v[2983] = if ((-s.v[2011]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) && (!(s.v[2982] != 0.0))) && (s.v[2983] != 0.0)) {
            s.store_div_from_scalar_ad(2009, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) && (!(s.v[2982] != 0.0))) && (!(s.v[2983] != 0.0))) {
            s.store_scale_ad(2009, A::offset(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) {
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.v[2984] = if ((((-s.v[2013])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) && (s.v[2984] != 0.0)) {
            s.store_exp_ad(2005, A::neg(s.ad_value(2013)));
        }

        s.v[2985] = if ((-s.v[2013]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) && (!(s.v[2984] != 0.0))) && (s.v[2985] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) && (!(s.v[2984] != 0.0))) && (!(s.v[2985] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) {
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) {
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

        if (((s.v[2975] != 0.0) && (!(s.v[2978] != 0.0))) && (!(s.v[2979] != 0.0))) {
            s.store_add(2016, 2013, 2014);
        }

        s.v[2986] = if (((s.v[2016]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((s.v[2975] != 0.0) && (s.v[2986] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
        }

        if ((s.v[2975] != 0.0) && (s.v[2986] != 0.0)) {
            s.store_mul_ad(1991, A::scale(s.ad_value(1889), (-0.70710678)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
        }

        if ((s.v[2975] != 0.0) && (s.v[2986] != 0.0)) {
            s.store_mul_ad(1990, A::scale(s.ad_value(1889), (-0.235702)), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.v[2987] = if ((((-s.v[2016])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2975] != 0.0) && (!(s.v[2986] != 0.0))) && (s.v[2987] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(2016)));
        }

        s.v[2988] = if ((-s.v[2016]) < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2975] != 0.0) && (!(s.v[2986] != 0.0))) && (!(s.v[2987] != 0.0))) && (s.v[2988] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2975] != 0.0) && (!(s.v[2986] != 0.0))) && (!(s.v[2987] != 0.0))) && (!(s.v[2988] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2975] != 0.0) && (!(s.v[2986] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0))));
        }

        s.v[2989] = if (s.v[2016] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((s.v[2975] != 0.0) && (!(s.v[2986] != 0.0))) && (s.v[2989] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((s.v[2975] != 0.0) && (!(s.v[2986] != 0.0))) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
        }

        if ((s.v[2975] != 0.0) && (!(s.v[2986] != 0.0))) {
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if (s.v[2975] != 0.0) {
            s.store_sub(1988, 1937, 1991);
        }

        if (s.v[2975] != 0.0) {
            s.store_div_from_scalar(1989, 1.0, 1988);
        }

        if (s.v[2975] != 0.0) {
            s.store_offset_ad(1987, A::mul(s.ad_value(1972), s.ad_value(1989)), (-1.0));
        }

        if (s.v[2975] != 0.0) {
            s.store_mul_ad_lhs(1986, A::sub_from_scalar(1.0, A::mul(A::mul(A::mul(s.ad_value(1972), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989))), 1989);
        }

        if (s.v[2975] != 0.0) {
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
        }

        if (s.v[2975] != 0.0) {
            s.store_mul_ad_lhs(1994, A::mul(A::square(s.ad_value(1992)), s.ad_value(1989)), 1989);
        }

        if (s.v[2975] != 0.0) {
            s.store_mul(1985, 2018, 1994);
        }

        s.v[2990] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2975] != 0.0) && (s.v[2990] != 0.0)) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if (s.v[2975] != 0.0) {
            s.store_sqrt_ad(2027, A::offset(A::scale(s.ad_value(1985), 2.0), 1.0));
        }

        if (s.v[2975] != 0.0) {
            s.store_div_from_scalar_ad(2019, 2.0, A::offset(s.ad_value(2027), 1.0));
        }

        if (s.v[2975] != 0.0) {
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
        }

        if (s.v[2975] != 0.0) {
            s.store_mul_ad_rhs(1954, 2019, A::sub(s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027))));
        }

        s.v[2991] = if (s.v[1] >= 5.0) { 1.0 } else { 0.0 };

        s.v[2992] = if (s.v[1] == 5.0) { 1.0 } else { 0.0 };

        if ((s.v[2991] != 0.0) && (s.v[2992] != 0.0)) {
            s.store_scale_ad(1992, A::sub(A::add(A::sub(A::add(A::add(A::add(A::scale(s.ad_value(1972), (-336.0)), A::scale(s.ad_value(1973), 84.0)), A::scale(s.ad_value(1971), 90.0)), A::scale(s.ad_value(1979), 181.0)), s.ad_value(1978)), A::scale(s.ad_value(1969), 6.0)), A::scale(s.ad_value(1970), 24.0)), 0.015384615384615385);
        }

        if ((s.v[2991] != 0.0) && (s.v[2992] != 0.0)) {
            s.store_scale_ad(1993, A::sub(A::sub(A::sub(A::add(A::add(A::add(A::scale(s.ad_value(1978), 18.0), A::scale(s.ad_value(1979), 3762.0)), A::scale(s.ad_value(1972), 6048.0)), A::scale(s.ad_value(1970), 432.0)), A::scale(s.ad_value(1971), 1620.0)), A::scale(s.ad_value(1969), 108.0)), A::scale(s.ad_value(1973), 8532.0)), 0.015384615384615385);
        }

        s.v[2993] = if (s.v[1] == 9.0) { 1.0 } else { 0.0 };

        if (((s.v[2991] != 0.0) && (!(s.v[2992] != 0.0))) && (s.v[2993] != 0.0)) {
            let assign69510_ad_e93656: A = A::scale(A::sub(A::add(A::add(A::add(A::scale(A::sub(s.ad_value(1974), s.ad_value(1972)), 1680.0), A::scale(A::sub(s.ad_value(1979), s.ad_value(1978)), 5.0)), A::scale(A::sub(s.ad_value(1971), s.ad_value(1975)), 450.0)), A::scale(A::sub(s.ad_value(1976), s.ad_value(1970)), 120.0)), A::scale(A::sub(s.ad_value(1977), s.ad_value(1969)), 30.0)), 0.004784688995215311);
            s.store_ad(1992, &assign69510_ad_e93656);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2992] != 0.0))) && (s.v[2993] != 0.0)) {
            let assign69520_ad_e93698: A = A::add(A::add(A::add(A::sub(A::sub(A::scale(A::add(s.ad_value(1969), s.ad_value(1977)), (-900.0)), A::scale(A::add(s.ad_value(1975), s.ad_value(1971)), 13500.0)), A::scale(s.ad_value(1973), 79500.0)), A::scale(A::add(s.ad_value(1972), s.ad_value(1974)), 50400.0)), A::scale(A::add(s.ad_value(1970), s.ad_value(1976)), 3600.0)), A::scale(A::add(s.ad_value(1978), s.ad_value(1979)), 150.0));
            s.store_scale_ad(1993, assign69520_ad_e93698, 0.0055248618784530384);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2992] != 0.0))) && (!(s.v[2993] != 0.0))) {
            s.store_scalar(1992, 0.0);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2992] != 0.0))) && (!(s.v[2993] != 0.0))) {
            s.store_scalar(1993, 0.0);
        }

        if (s.v[2991] != 0.0) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1973), s.ad_value(1937)), 1890);
        }

        s.v[2994] = if (((s.v[2027]) as f64).abs() <= s.v[1941]) { 1.0 } else { 0.0 };

        if ((s.v[2991] != 0.0) && (s.v[2994] != 0.0)) {
            s.store_div(2016, 2027, 1940);
        }

        s.v[2995] = if (s.v[2027] < (-s.v[1941])) { 1.0 } else { 0.0 };

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_neg(1999, 2027);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_div_ad_lhs(2000, A::scale(s.ad_value(1999), 1.25), 1940);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_scale_ad(2001, A::sub(A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_add(824, 2002, 2003);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.v[2996] = if (((s.v[2015]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) && (s.v[2996] != 0.0)) {
            s.store_exp(2005, 2015);
        }

        s.v[2997] = if (s.v[2015] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) && (!(s.v[2996] != 0.0))) && (s.v[2997] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) && (!(s.v[2996] != 0.0))) && (!(s.v[2997] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (s.v[2995] != 0.0)) {
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) {
            s.store_div_from_scalar_ad(1998, 1.0, A::offset(A::scale(s.ad_value(1938), 0.732464877560822), 1.25));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) {
            s.store_mul_ad_lhs(2010, A::offset(A::mul(A::scale(s.ad_value(1940), 1.25), s.ad_value(1998)), (-1.0)), 1998);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) {
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.v[2998] = if ((((-s.v[2011])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) && (s.v[2998] != 0.0)) {
            s.store_exp_ad(2009, A::neg(s.ad_value(2011)));
        }

        s.v[2999] = if ((-s.v[2011]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) && (!(s.v[2998] != 0.0))) && (s.v[2999] != 0.0)) {
            s.store_div_from_scalar_ad(2009, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) && (!(s.v[2998] != 0.0))) && (!(s.v[2999] != 0.0))) {
            s.store_scale_ad(2009, A::offset(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) {
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.v[3000] = if ((((-s.v[2013])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) && (s.v[3000] != 0.0)) {
            s.store_exp_ad(2005, A::neg(s.ad_value(2013)));
        }

        s.v[3001] = if ((-s.v[2013]) < 0.0) { 1.0 } else { 0.0 };

    }

    pub(super) fn stamp_transient_block_47(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        s: &mut Scratch,
    ) {
        let _ = stamper;
        let p = self.params;
        let nodes = self.nodes;
        let branches = self.branches;
        if (((((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) && (!(s.v[3000] != 0.0))) && (s.v[3001] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) && (!(s.v[3000] != 0.0))) && (!(s.v[3001] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) {
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) {
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

        if (((s.v[2991] != 0.0) && (!(s.v[2994] != 0.0))) && (!(s.v[2995] != 0.0))) {
            s.store_add(2016, 2013, 2014);
        }

        s.v[3002] = if (((s.v[2016]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((s.v[2991] != 0.0) && (s.v[3002] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
        }

        if ((s.v[2991] != 0.0) && (s.v[3002] != 0.0)) {
            s.store_mul_ad(1991, A::scale(s.ad_value(1889), (-0.70710678)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
        }

        if ((s.v[2991] != 0.0) && (s.v[3002] != 0.0)) {
            s.store_mul_ad(1990, A::scale(s.ad_value(1889), (-0.235702)), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.v[3003] = if ((((-s.v[2016])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[2991] != 0.0) && (!(s.v[3002] != 0.0))) && (s.v[3003] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(2016)));
        }

        s.v[3004] = if ((-s.v[2016]) < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[2991] != 0.0) && (!(s.v[3002] != 0.0))) && (!(s.v[3003] != 0.0))) && (s.v[3004] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[2991] != 0.0) && (!(s.v[3002] != 0.0))) && (!(s.v[3003] != 0.0))) && (!(s.v[3004] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[2991] != 0.0) && (!(s.v[3002] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0))));
        }

        s.v[3005] = if (s.v[2016] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((s.v[2991] != 0.0) && (!(s.v[3002] != 0.0))) && (s.v[3005] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((s.v[2991] != 0.0) && (!(s.v[3002] != 0.0))) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
        }

        if ((s.v[2991] != 0.0) && (!(s.v[3002] != 0.0))) {
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if (s.v[2991] != 0.0) {
            s.store_sub(1988, 1937, 1991);
        }

        if (s.v[2991] != 0.0) {
            s.store_div_from_scalar(1989, 1.0, 1988);
        }

        if (s.v[2991] != 0.0) {
            s.store_offset_ad(1987, A::mul(s.ad_value(1973), s.ad_value(1989)), (-1.0));
        }

        if (s.v[2991] != 0.0) {
            s.store_mul_ad_lhs(1986, A::sub_from_scalar(1.0, A::mul(A::mul(A::mul(s.ad_value(1973), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989))), 1989);
        }

        if (s.v[2991] != 0.0) {
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
        }

        if (s.v[2991] != 0.0) {
            s.store_mul_ad_lhs(1994, A::mul(A::square(s.ad_value(1992)), s.ad_value(1989)), 1989);
        }

        if (s.v[2991] != 0.0) {
            s.store_mul(1985, 2018, 1994);
        }

        s.v[3006] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[2991] != 0.0) && (s.v[3006] != 0.0)) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if (s.v[2991] != 0.0) {
            s.store_sqrt_ad(2027, A::offset(A::scale(s.ad_value(1985), 2.0), 1.0));
        }

        if (s.v[2991] != 0.0) {
            s.store_div_from_scalar_ad(2019, 2.0, A::offset(s.ad_value(2027), 1.0));
        }

        if (s.v[2991] != 0.0) {
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
        }

        if (s.v[2991] != 0.0) {
            s.store_mul_ad_rhs(1955, 2019, A::sub(s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027))));
        }

        s.v[3007] = if (s.v[1] >= 6.0) { 1.0 } else { 0.0 };

        s.v[3008] = if (s.v[1] == 9.0) { 1.0 } else { 0.0 };

        if ((s.v[3007] != 0.0) && (s.v[3008] != 0.0)) {
            let assign70290_ad_e94902: A = A::scale(A::add(A::add(A::sub(A::add(A::sub(A::sub(A::add(A::scale(s.ad_value(1974), 30.0), A::scale(s.ad_value(1972), 81480.0)), A::scale(s.ad_value(1971), 21825.0)), A::scale(s.ad_value(1976), 81060.0)), A::scale(s.ad_value(1977), 20265.0)), A::scale(s.ad_value(1969), 1455.0)), A::scale(s.ad_value(1975), 303975.0)), A::scale(s.ad_value(1970), 5820.0)), 2.6434745829918846e-5);
            s.store_sub_ad(1992, A::sub(assign70290_ad_e94902, A::scale(A::sub(A::scale(s.ad_value(1979), 6755.0), A::scale(s.ad_value(1978), 485.0)), 1.3217372914959423e-5)), A::scale(s.ad_value(1973), (1455.0 / 181.0)));
        }

        if ((s.v[3007] != 0.0) && (s.v[3008] != 0.0)) {
            let assign70300_ad_e94958: A = A::add(A::sub(A::sub(A::sub(A::sub(A::add(A::sub(A::add(A::scale(s.ad_value(1969), 50400.0), A::scale(s.ad_value(1975), 10530000.0)), A::scale(s.ad_value(1972), 2822400.0)), A::scale(s.ad_value(1971), 756000.0)), A::scale(s.ad_value(1970), 201600.0)), A::scale(s.ad_value(1978), 8400.0)), A::scale(s.ad_value(1979), 117000.0)), A::scale(s.ad_value(1976), 2808000.0)), A::scale(s.ad_value(1977), 702000.0));
            s.store_add_ad(1993, A::scale(A::sub(assign70300_ad_e94958, A::scale(s.ad_value(1974), 16614600.0)), 2.6434745829918846e-5), A::scale(s.ad_value(1973), (50400.0 * 0.0055248618784530384)));
        }

        if ((s.v[3007] != 0.0) && (!(s.v[3008] != 0.0))) {
            s.store_scalar(1992, 0.0);
        }

        if ((s.v[3007] != 0.0) && (!(s.v[3008] != 0.0))) {
            s.store_scalar(1993, 0.0);
        }

        if (s.v[3007] != 0.0) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1974), s.ad_value(1937)), 1890);
        }

        s.v[3009] = if (((s.v[2027]) as f64).abs() <= s.v[1941]) { 1.0 } else { 0.0 };

        if ((s.v[3007] != 0.0) && (s.v[3009] != 0.0)) {
            s.store_div(2016, 2027, 1940);
        }

        s.v[3010] = if (s.v[2027] < (-s.v[1941])) { 1.0 } else { 0.0 };

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_neg(1999, 2027);
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_div_ad_lhs(2000, A::scale(s.ad_value(1999), 1.25), 1940);
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_scale_ad(2001, A::sub(A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_add(824, 2002, 2003);
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.v[3011] = if (((s.v[2015]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) && (s.v[3011] != 0.0)) {
            s.store_exp(2005, 2015);
        }

        s.v[3012] = if (s.v[2015] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) && (!(s.v[3011] != 0.0))) && (s.v[3012] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) && (!(s.v[3011] != 0.0))) && (!(s.v[3012] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_div_ad(2012, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (s.v[3010] != 0.0)) {
            s.store_neg_ad(2016, A::add(s.ad_value(2015), s.ad_value(2012)));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) {
            s.store_div_from_scalar_ad(1998, 1.0, A::offset(A::scale(s.ad_value(1938), 0.732464877560822), 1.25));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) {
            s.store_mul_ad_lhs(2010, A::offset(A::mul(A::scale(s.ad_value(1940), 1.25), s.ad_value(1998)), (-1.0)), 1998);
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) {
            s.store_mul_ad(2011, A::div(s.ad_value(2027), s.ad_value(1940)), A::offset(A::mul(s.ad_value(2010), s.ad_value(2027)), 1.0));
        }

        s.v[3013] = if ((((-s.v[2011])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) && (s.v[3013] != 0.0)) {
            s.store_exp_ad(2009, A::neg(s.ad_value(2011)));
        }

        s.v[3014] = if ((-s.v[2011]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) && (!(s.v[3013] != 0.0))) && (s.v[3014] != 0.0)) {
            s.store_div_from_scalar_ad(2009, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2011))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) && (!(s.v[3013] != 0.0))) && (!(s.v[3014] != 0.0))) {
            s.store_scale_ad(2009, A::offset(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2011)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) {
            s.store_sub_from_scalar(2012, 1.0, 2009);
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) {
            s.store_sub_ad(2013, A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.5)), A::mul(s.ad_value(1938), A::sqrt(A::sub(A::add(s.ad_value(2027), A::scale(s.ad_value(1939), 0.25)), s.ad_value(2012)))));
        }

        s.v[3015] = if ((((-s.v[2013])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) && (s.v[3015] != 0.0)) {
            s.store_exp_ad(2005, A::neg(s.ad_value(2013)));
        }

        s.v[3016] = if ((-s.v[2013]) < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) && (!(s.v[3015] != 0.0))) && (s.v[3016] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2013))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) && (!(s.v[3015] != 0.0))) && (!(s.v[3016] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2013)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::mul(A::scale(s.ad_value(1939), 0.5), s.ad_value(2005)));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(2027), s.ad_value(2013)), 2.0), A::mul(s.ad_value(1939), A::sub_from_scalar(1.0, s.ad_value(2005))));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) {
            s.store_sub_ad(2008, A::mul(A::sub(s.ad_value(2027), s.ad_value(2013)), A::sub(s.ad_value(2027), s.ad_value(2013))), A::mul(s.ad_value(1939), A::add(A::offset(s.ad_value(2013), (-1.0)), s.ad_value(2005))));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) {
            s.store_sub_ad(2009, A::square(s.ad_value(2007)), A::mul(A::scale(s.ad_value(2006), 4.0), s.ad_value(2008)));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) {
            s.store_div_ad(2014, A::scale(s.ad_value(2008), 2.0), A::add(s.ad_value(2007), A::sqrt(s.ad_value(2009))));
        }

        if (((s.v[3007] != 0.0) && (!(s.v[3009] != 0.0))) && (!(s.v[3010] != 0.0))) {
            s.store_add(2016, 2013, 2014);
        }

        s.v[3017] = if (((s.v[2016]) as f64).abs() <= s.v[1933]) { 1.0 } else { 0.0 };

        if ((s.v[3007] != 0.0) && (s.v[3017] != 0.0)) {
            s.store_mul_ad(1996, A::mul(A::scale(s.ad_value(2016), (-0.70710678)), s.ad_value(1889)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.16666666666666666), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.16666666666666666)))));
        }

        if ((s.v[3007] != 0.0) && (s.v[3017] != 0.0)) {
            s.store_mul_ad(1991, A::scale(s.ad_value(1889), (-0.70710678)), A::sub_from_scalar(1.0, A::mul(A::scale(s.ad_value(2016), 0.3333333333333333), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.25)))));
        }

        if ((s.v[3007] != 0.0) && (s.v[3017] != 0.0)) {
            s.store_mul_ad(1990, A::scale(s.ad_value(1889), (-0.235702)), A::sub_from_scalar(1.0, A::scale(s.ad_value(2016), 0.5)));
        }

        s.v[3018] = if ((((-s.v[2016])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((s.v[3007] != 0.0) && (!(s.v[3017] != 0.0))) && (s.v[3018] != 0.0)) {
            s.store_exp_ad(2027, A::neg(s.ad_value(2016)));
        }

        s.v[3019] = if ((-s.v[2016]) < 0.0) { 1.0 } else { 0.0 };

        if ((((s.v[3007] != 0.0) && (!(s.v[3017] != 0.0))) && (!(s.v[3018] != 0.0))) && (s.v[3019] != 0.0)) {
            s.store_div_from_scalar_ad(2027, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), A::neg(s.ad_value(2016))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if ((((s.v[3007] != 0.0) && (!(s.v[3017] != 0.0))) && (!(s.v[3018] != 0.0))) && (!(s.v[3019] != 0.0))) {
            s.store_scale_ad(2027, A::offset(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), A::offset(A::scale(A::offset(A::neg(s.ad_value(2016)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if ((s.v[3007] != 0.0) && (!(s.v[3017] != 0.0))) {
            s.store_mul_ad_rhs(1996, 1889, A::sqrt(A::offset(A::add(s.ad_value(2027), s.ad_value(2016)), (-1.0))));
        }

        s.v[3020] = if (s.v[2016] > s.v[1933]) { 1.0 } else { 0.0 };

        if (((s.v[3007] != 0.0) && (!(s.v[3017] != 0.0))) && (s.v[3020] != 0.0)) {
            s.store_neg(1996, 1996);
        }

        if ((s.v[3007] != 0.0) && (!(s.v[3017] != 0.0))) {
            s.store_div_ad_lhs(1991, A::mul(A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889)), A::sub_from_scalar(1.0, s.ad_value(2027))), 1996);
        }

        if ((s.v[3007] != 0.0) && (!(s.v[3017] != 0.0))) {
            s.store_add_ad_lhs(1990, A::div(A::sub(A::square(s.ad_value(1991)), A::mul(A::scale(s.ad_value(1889), 0.5), s.ad_value(1889))), s.ad_value(1996)), 1991);
        }

        if (s.v[3007] != 0.0) {
            s.store_sub(1988, 1937, 1991);
        }

        if (s.v[3007] != 0.0) {
            s.store_div_from_scalar(1989, 1.0, 1988);
        }

        if (s.v[3007] != 0.0) {
            s.store_offset_ad(1987, A::mul(s.ad_value(1974), s.ad_value(1989)), (-1.0));
        }

        if (s.v[3007] != 0.0) {
            s.store_mul_ad_lhs(1986, A::sub_from_scalar(1.0, A::mul(A::mul(A::mul(s.ad_value(1974), s.ad_value(1990)), s.ad_value(1989)), s.ad_value(1989))), 1989);
        }

        if (s.v[3007] != 0.0) {
            s.store_add_ad(2017, A::mul(A::mul(s.ad_value(1986), s.ad_value(1992)), s.ad_value(1992)), A::mul(s.ad_value(1987), s.ad_value(1993)));
        }

        if (s.v[3007] != 0.0) {
            s.store_mul_ad_lhs(1994, A::mul(A::square(s.ad_value(1992)), s.ad_value(1989)), 1989);
        }

        if (s.v[3007] != 0.0) {
            s.store_mul(1985, 2018, 1994);
        }

        s.v[3021] = if (s.v[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((s.v[3007] != 0.0) && (s.v[3021] != 0.0)) {
            s.store_div_ad_rhs(1985, 1985, A::offset(A::mul(s.ad_value(1907), s.ad_value(1893)), 1.0));
        }

        if (s.v[3007] != 0.0) {
            s.store_sqrt_ad(2027, A::offset(A::scale(s.ad_value(1985), 2.0), 1.0));
        }

        if (s.v[3007] != 0.0) {
            s.store_div_from_scalar_ad(2019, 2.0, A::offset(s.ad_value(2027), 1.0));
        }

        if (s.v[3007] != 0.0) {
            s.store_sub_ad_rhs(2028, 1993, A::mul(s.ad_value(1994), s.ad_value(1990)));
        }

        if (s.v[3007] != 0.0) {
            s.store_mul_ad_rhs(1956, 2019, A::sub(s.ad_value(2017), A::div(A::mul(A::mul(A::mul(s.ad_value(1985), s.ad_value(1987)), s.ad_value(2028)), s.ad_value(2019)), s.ad_value(2027))));
        }

        s.v[3022] = if (s.v[1] >= 7.0) { 1.0 } else { 0.0 };

        s.v[3023] = if (s.v[1] == 9.0) { 1.0 } else { 0.0 };

        if ((s.v[3022] != 0.0) && (s.v[3023] != 0.0)) {
            let assign71070_ad_e96169: A = A::add(A::add(A::sub(A::sub(A::add(A::add(A::add(A::sub(A::scale(s.ad_value(1974), (-304200.0)), A::scale(s.ad_value(1972), 21840.0)), A::scale(s.ad_value(1979), 12605.0)), A::scale(s.ad_value(1971), 5850.0)), A::scale(s.ad_value(1976), 302520.0)), A::scale(s.ad_value(1978), 65.0)), A::scale(s.ad_value(1977), 75630.0)), A::scale(s.ad_value(1969), 390.0)), A::scale(s.ad_value(1975), 420.0));
            s.store_add_ad(1992, A::scale(A::sub(assign71070_ad_e96169, A::scale(s.ad_value(1970), 1560.0)), 2.6434745829918846e-5), A::scale(s.ad_value(1973), (390.0 / 181.0)));
        }

        if ((s.v[3022] != 0.0) && (s.v[3023] != 0.0)) {
            let assign71080_ad_e96222: A = A::sub(A::add(A::add(A::add(A::add(A::sub(A::add(A::sub(A::scale(s.ad_value(1969), (-13500.0)), A::scale(s.ad_value(1975), 16601100.0)), A::scale(s.ad_value(1972), 756000.0)), A::scale(s.ad_value(1971), 202500.0)), A::scale(s.ad_value(1970), 54000.0)), A::scale(s.ad_value(1978), 2250.0)), A::scale(s.ad_value(1979), 436650.0)), A::scale(s.ad_value(1976), 10479600.0)), A::scale(s.ad_value(1977), 2619900.0));
            s.store_sub_ad(1993, A::scale(A::add(assign71080_ad_e96222, A::scale(s.ad_value(1974), 10530000.0)), 2.6434745829918846e-5), A::scale(s.ad_value(1973), (13500.0 * 0.0055248618784530384)));
        }

        if ((s.v[3022] != 0.0) && (!(s.v[3023] != 0.0))) {
            s.store_scalar(1992, 0.0);
        }

        if ((s.v[3022] != 0.0) && (!(s.v[3023] != 0.0))) {
            s.store_scalar(1993, 0.0);
        }

        if (s.v[3022] != 0.0) {
            s.store_add_ad_lhs(2027, A::div(s.ad_value(1975), s.ad_value(1937)), 1890);
        }

        s.v[3024] = if (((s.v[2027]) as f64).abs() <= s.v[1941]) { 1.0 } else { 0.0 };

        if ((s.v[3022] != 0.0) && (s.v[3024] != 0.0)) {
            s.store_div(2016, 2027, 1940);
        }

        s.v[3025] = if (s.v[2027] < (-s.v[1941])) { 1.0 } else { 0.0 };

        if (((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) {
            s.store_neg(1999, 2027);
        }

        if (((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) {
            s.store_div_ad_lhs(2000, A::scale(s.ad_value(1999), 1.25), 1940);
        }

        if (((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) {
            s.store_scale_ad(2001, A::sub(A::offset(s.ad_value(2000), 10.0), A::sqrt(A::offset(A::mul(A::offset(s.ad_value(2000), (-6.0)), A::offset(s.ad_value(2000), (-6.0))), 64.0))), 0.5);
        }

        if (((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) {
            s.store_add_ad(2002, A::mul(A::sub(s.ad_value(1999), s.ad_value(2001)), A::sub(s.ad_value(1999), s.ad_value(2001))), A::mul(s.ad_value(1939), A::offset(s.ad_value(2001), 1.0)));
        }

        if (((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) {
            s.store_sub_ad_lhs(2003, A::scale(A::sub(s.ad_value(1999), s.ad_value(2001)), 2.0), 1939);
        }

        if (((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) {
            s.store_sub_ad_lhs(2004, A::ln(A::div(s.ad_value(2002), s.ad_value(1939))), 2001);
        }

        if (((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) {
            s.store_add(824, 2002, 2003);
        }

        if (((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) {
            s.store_add_ad(823, A::square(s.ad_value(824)), A::mul(s.ad_value(2004), A::sub(A::scale(A::square(s.ad_value(2003)), 0.5), s.ad_value(2002))));
        }

        if (((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) {
            s.store_add_ad_rhs(2015, 2001, A::div(A::mul(A::mul(s.ad_value(2002), s.ad_value(824)), s.ad_value(2004)), A::add(s.ad_value(823), A::mul(A::mul(A::mul(A::mul(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2004)), s.ad_value(2004)), s.ad_value(2003)), A::sub(A::scale(A::square(s.ad_value(2003)), 0.3333333333333333), s.ad_value(2002))))));
        }

        s.v[3026] = if (((s.v[2015]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) && (s.v[3026] != 0.0)) {
            s.store_exp(2005, 2015);
        }

        s.v[3027] = if (s.v[2015] < 0.0) { 1.0 } else { 0.0 };

        if (((((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) && (!(s.v[3026] != 0.0))) && (s.v[3027] != 0.0)) {
            s.store_div_from_scalar_ad(2005, 1e-100, A::offset(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::mul(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), A::offset(A::scale(A::sub_from_scalar((-230.25850929940458), s.ad_value(2015)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        if (((((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) && (!(s.v[3026] != 0.0))) && (!(s.v[3027] != 0.0))) {
            s.store_scale_ad(2005, A::offset(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::mul(A::offset(s.ad_value(2015), (-230.25850929940458)), A::offset(A::scale(A::offset(s.ad_value(2015), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100);
        }

        if (((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) {
            s.store_sub_from_scalar_ad(2006, 1.0, A::scale(A::mul(s.ad_value(1939), s.ad_value(2005)), 0.5));
        }

        if (((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) {
            s.store_add_ad(2007, A::scale(A::sub(s.ad_value(1999), s.ad_value(2015)), 2.0), A::mul(s.ad_value(1939), A::offset(s.ad_value(2005), (-1.0))));
        }

        if (((s.v[3022] != 0.0) && (!(s.v[3024] != 0.0))) && (s.v[3025] != 0.0)) {
            s.store_add_ad(2008, A::mul(A::sub(s.ad_value(1999), s.ad_value(2015)), A::sub(s.ad_value(1999), s.ad_value(2015))), A::mul(s.ad_value(1939), A::sub(A::offset(s.ad_value(2015), 1.0), s.ad_value(2005))));
        }

    }
}
