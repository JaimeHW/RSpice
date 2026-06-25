#![allow(dead_code, unused_assignments, unused_imports, unused_parens, unused_variables)]

use super::{AdValue, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, Scratch, THERMAL_VOLTAGE_PER_K};
use super::super::state::Instance;

impl Instance {

    pub(super) fn stamp_transient_block_32(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2136, &AdValue::mul(scratch.ad_value(2101), scratch.ad_value(2135)));
        }

        scratch.values[2231] = if (((scratch.values[2092]) as f64).abs() <= scratch.values[2102]) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2231] != 0.0)) {
            scratch.store_ad(2178, &AdValue::scale(AdValue::square(scratch.ad_value(2098)), (0.16666666666666666 * 0.7071067811865475)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2231] != 0.0)) {
            scratch.store_ad(2137, &AdValue::mul(AdValue::mul(scratch.ad_value(2092), scratch.ad_value(2098)), AdValue::offset(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2092), AdValue::sub_from_scalar(1.0, scratch.ad_value(2136))), scratch.ad_value(2094)), scratch.ad_value(2178)), 1.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2199, &AdValue::offset(scratch.ad_value(2134), 3.0));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2182, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2198), scratch.ad_value(2199)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2198), scratch.ad_value(2199)), AdValue::sub(scratch.ad_value(2198), scratch.ad_value(2199))), 5.0))), 0.5), AdValue::scale(AdValue::sub(scratch.ad_value(2199), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2199)), 5.0))), 0.5)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2177, &AdValue::sub(scratch.ad_value(2092), scratch.ad_value(2182)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2178, &AdValue::exp(AdValue::neg(scratch.ad_value(2182))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2179, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2182)), 2.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2189, &AdValue::mul(AdValue::square(scratch.ad_value(2182)), scratch.ad_value(2179)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2190, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2182), scratch.ad_value(2179)), scratch.ad_value(2179)), 4.0));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2191, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2179), 8.0), AdValue::scale(scratch.ad_value(2189), 12.0)), scratch.ad_value(2179)), scratch.ad_value(2179)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2183, &AdValue::max_from_scalar(1e-40, AdValue::sub(AdValue::square(scratch.ad_value(2177)), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::offset(AdValue::add(scratch.ad_value(2178), scratch.ad_value(2182)), (-1.0)), AdValue::mul(scratch.ad_value(2136), AdValue::add(AdValue::offset(scratch.ad_value(2182), 1.0), scratch.ad_value(2189))))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2200, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2095), AdValue::sub(scratch.ad_value(2178), AdValue::mul(scratch.ad_value(2136), scratch.ad_value(2191)))), 0.5)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2184, &AdValue::add(AdValue::scale(scratch.ad_value(2177), 2.0), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(2178)), AdValue::mul(scratch.ad_value(2136), AdValue::offset(scratch.ad_value(2190), 1.0))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2185, &AdValue::add(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2182)), AdValue::ln(AdValue::div(scratch.ad_value(2183), scratch.ad_value(2095)))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(966, &AdValue::add(scratch.ad_value(2183), scratch.ad_value(2184)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(965, &AdValue::add(AdValue::square(scratch.ad_value(966)), AdValue::mul(scratch.ad_value(2185), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2184)), 0.5), AdValue::mul(scratch.ad_value(2183), scratch.ad_value(2200))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            let assign43110_ad_e55789: AdValue = AdValue::add(scratch.ad_value(2182), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2183), scratch.ad_value(966)), scratch.ad_value(2185)), AdValue::add(scratch.ad_value(965), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(966), scratch.ad_value(965)), scratch.ad_value(2185)), scratch.ad_value(2185)), scratch.ad_value(2184)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2184)), 0.3333333333333333), AdValue::mul(scratch.ad_value(2183), scratch.ad_value(2200)))))));
            scratch.store_ad(2201, &assign43110_ad_e55789);
        }

        scratch.values[2232] = if (scratch.values[2201] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (scratch.values[2232] != 0.0)) {
            scratch.store_ad(2187, &AdValue::exp(scratch.ad_value(2201)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (scratch.values[2232] != 0.0)) {
            scratch.store_ad(2188, &AdValue::div_from_scalar(1.0, scratch.ad_value(2187)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (scratch.values[2232] != 0.0)) {
            scratch.store_ad(2187, &AdValue::mul(scratch.ad_value(2136), scratch.ad_value(2187)));
        }

        scratch.values[2233] = if (scratch.values[2201] > (scratch.values[2134] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (scratch.values[2233] != 0.0)) {
            scratch.store_ad(2187, &AdValue::exp(AdValue::sub(scratch.ad_value(2201), scratch.ad_value(2134))));
        }

        if ((((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (scratch.values[2233] != 0.0)) {
            scratch.store_ad(2188, &AdValue::div(scratch.ad_value(2136), scratch.ad_value(2187)));
        }

        if ((((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (!(scratch.values[2233] != 0.0))) {
            scratch.store_ad(2187, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2201)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2201)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2201)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) && (!(scratch.values[2232] != 0.0))) && (!(scratch.values[2233] != 0.0))) {
            scratch.store_ad(2188, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2201), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2201), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2201), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2177, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2201)), 2.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2189, &AdValue::mul(AdValue::square(scratch.ad_value(2201)), scratch.ad_value(2177)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2190, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2201), scratch.ad_value(2177)), scratch.ad_value(2177)), 4.0));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2191, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2177), 8.0), AdValue::scale(scratch.ad_value(2189), 12.0)), scratch.ad_value(2177)), scratch.ad_value(2177)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2177, &AdValue::sub(scratch.ad_value(2092), scratch.ad_value(2201)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2192, &AdValue::add(AdValue::scale(scratch.ad_value(2177), 2.0), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2188)), scratch.ad_value(2187)), AdValue::mul(scratch.ad_value(2136), AdValue::offset(scratch.ad_value(2190), 1.0))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2193, &AdValue::sub(AdValue::square(scratch.ad_value(2177)), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(AdValue::offset(AdValue::add(scratch.ad_value(2188), scratch.ad_value(2201)), (-1.0)), scratch.ad_value(2187)), AdValue::mul(scratch.ad_value(2136), AdValue::add(AdValue::offset(scratch.ad_value(2201), 1.0), scratch.ad_value(2189)))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2177, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(scratch.ad_value(2188), scratch.ad_value(2187)), AdValue::mul(scratch.ad_value(2136), scratch.ad_value(2191))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2177, &AdValue::sub(AdValue::square(scratch.ad_value(2192)), AdValue::scale(AdValue::mul(scratch.ad_value(2193), scratch.ad_value(2177)), 2.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2231] != 0.0))) {
            scratch.store_ad(2137, &AdValue::add(scratch.ad_value(2201), AdValue::scale(AdValue::div(scratch.ad_value(2193), AdValue::add(scratch.ad_value(2192), AdValue::sqrt(scratch.ad_value(2177)))), 2.0)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2138, &AdValue::sub(scratch.ad_value(2137), scratch.ad_value(2103)));
        }

        scratch.values[2234] = if (scratch.values[2138] < 1e-10) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2234] != 0.0)) {
            scratch.store_ad(2139, &AdValue::add(AdValue::scale(AdValue::sub(scratch.ad_value(2092), scratch.ad_value(2103)), 2.0), AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2108)), AdValue::mul(scratch.ad_value(2104), scratch.ad_value(2135))), AdValue::mul(scratch.ad_value(2136), AdValue::offset(scratch.ad_value(2106), 1.0))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2234] != 0.0)) {
            scratch.store_ad(2140, &AdValue::mul(AdValue::mul(scratch.ad_value(2095), AdValue::sub_from_scalar(1.0, scratch.ad_value(2135))), scratch.ad_value(2109)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2234] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2095), AdValue::sub(AdValue::add(scratch.ad_value(2108), AdValue::mul(scratch.ad_value(2104), scratch.ad_value(2135))), AdValue::mul(scratch.ad_value(2136), scratch.ad_value(2107))))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2234] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sub(AdValue::square(scratch.ad_value(2139)), AdValue::scale(AdValue::mul(scratch.ad_value(2076), scratch.ad_value(2140)), 2.0)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2234] != 0.0)) {
            scratch.store_ad(2138, &AdValue::scale(AdValue::div(scratch.ad_value(2140), AdValue::add(scratch.ad_value(2139), AdValue::sqrt(scratch.ad_value(2076)))), 2.0));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2234] != 0.0)) {
            scratch.store_ad(2137, &AdValue::add(scratch.ad_value(2103), scratch.ad_value(2138)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2141, &AdValue::mul(scratch.ad_value(2138), scratch.ad_value(2011)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2142, &AdValue::div(AdValue::square(scratch.ad_value(2137)), AdValue::offset(AdValue::square(scratch.ad_value(2137)), 2.0)));
        }

        scratch.values[2235] = if (scratch.values[2137] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2235] != 0.0)) {
            scratch.store_ad(2143, &AdValue::exp(AdValue::neg(scratch.ad_value(2137))));
        }

        scratch.values[2236] = if (scratch.values[2137] < 1e-5) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2235] != 0.0)) && (scratch.values[2236] != 0.0)) {
            scratch.store_ad(2144, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(2136), 0.16666666666666666), scratch.ad_value(2137)), scratch.ad_value(2137)), scratch.ad_value(2137)), AdValue::offset(AdValue::scale(scratch.ad_value(2137), 1.75), 1.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2235] != 0.0)) && (!(scratch.values[2236] != 0.0))) {
            scratch.store_ad(2144, &AdValue::mul(scratch.ad_value(2136), AdValue::sub(AdValue::offset(AdValue::sub(AdValue::div_from_scalar(1.0, scratch.ad_value(2143)), scratch.ad_value(2137)), (-1.0)), scratch.ad_value(2142))));
        }

        scratch.values[2237] = if (scratch.values[2137] > (scratch.values[2134] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2076, &AdValue::exp(AdValue::sub(scratch.ad_value(2137), scratch.ad_value(2134))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2143, &AdValue::div(scratch.ad_value(2136), scratch.ad_value(2076)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (scratch.values[2237] != 0.0)) {
            scratch.store_ad(2144, &AdValue::sub(scratch.ad_value(2076), AdValue::mul(scratch.ad_value(2136), AdValue::add(AdValue::offset(scratch.ad_value(2137), 1.0), scratch.ad_value(2142)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (!(scratch.values[2237] != 0.0))) {
            scratch.store_ad(2143, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2137), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2137), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2137), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (!(scratch.values[2237] != 0.0))) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2137)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2137)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2134), scratch.ad_value(2137)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2235] != 0.0))) && (!(scratch.values[2237] != 0.0))) {
            scratch.store_ad(2144, &AdValue::sub(scratch.ad_value(2076), AdValue::mul(scratch.ad_value(2136), AdValue::add(AdValue::offset(scratch.ad_value(2137), 1.0), scratch.ad_value(2142)))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2145, &AdValue::scale(AdValue::add(scratch.ad_value(2103), scratch.ad_value(2137)), 0.5));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.values[2146] = 0.0;
            scratch.node_derivatives[2146] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2146] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(scratch.ad_value(2143), scratch.ad_value(2108)));
        }

        scratch.values[2238] = if (scratch.values[2076] > 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2146, &AdValue::sqrt(scratch.ad_value(2076)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2147, &AdValue::scale(AdValue::add(scratch.ad_value(2109), scratch.ad_value(2144)), 0.5));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2148, &AdValue::add(scratch.ad_value(2147), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2138)), AdValue::sub(scratch.ad_value(2146), AdValue::scale(scratch.ad_value(2096), 2.0))), 0.125)));
        }

        scratch.values[2239] = if (scratch.values[2145] < 1e-5) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2239] != 0.0)) {
            scratch.store_ad(2149, &AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2145)), AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2145), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2145), 0.25))), 0.3333333333333333))), 0.5));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2239] != 0.0)) {
            scratch.store_ad(2150, &AdValue::mul(scratch.ad_value(2094), AdValue::sqrt(AdValue::add(scratch.ad_value(2148), scratch.ad_value(2149)))));
        }

        scratch.values[2240] = if (scratch.values[773] > 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (scratch.values[2239] != 0.0)) && (scratch.values[2240] != 0.0)) {
            scratch.store_ad(2151, &AdValue::div_from_scalar(1.0, AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(773), scratch.ad_value(2150)), 1.0))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2239] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2145), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2145), 0.25))), 0.3333333333333333))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2239] != 0.0)) {
            scratch.store_ad(2152, &AdValue::scale(AdValue::mul(scratch.ad_value(2145), scratch.ad_value(2076)), 0.7071067811865475));
        }

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2239] != 0.0)) {
            scratch.store_ad(2153, &AdValue::add(scratch.ad_value(2151), AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2094), AdValue::add(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2145), 0.5)), AdValue::scale(AdValue::square(scratch.ad_value(2145)), 0.16666666666666666))), scratch.ad_value(2076)), 0.7071067811865475)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) {
            scratch.store_ad(2149, &AdValue::add(AdValue::offset(scratch.ad_value(2145), (-1.0)), scratch.ad_value(2146)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) {
            scratch.store_ad(2150, &AdValue::mul(scratch.ad_value(2094), AdValue::sqrt(AdValue::add(scratch.ad_value(2148), scratch.ad_value(2149)))));
        }

        scratch.values[2241] = if (scratch.values[773] > 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2154, &AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2146)), AdValue::scale(AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2096)), 2.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2151, &AdValue::div_from_scalar(1.0, AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(773), scratch.ad_value(2150)), 1.0))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div(scratch.ad_value(2151), AdValue::offset(scratch.ad_value(2151), 1.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2155, &AdValue::mul(scratch.ad_value(773), AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2076)), scratch.ad_value(2095)), scratch.ad_value(2148))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2156, &AdValue::add(AdValue::scale(AdValue::sub(scratch.ad_value(2150), scratch.ad_value(2155)), 2.0), AdValue::mul(scratch.ad_value(2095), AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2146)), scratch.ad_value(2148)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2157, &AdValue::mul(scratch.ad_value(2155), AdValue::sub(scratch.ad_value(2155), AdValue::scale(scratch.ad_value(2150), 2.0))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2158, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2095), AdValue::add(scratch.ad_value(2146), scratch.ad_value(2148))), 0.5)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2159, &AdValue::div(AdValue::mul(scratch.ad_value(2157), scratch.ad_value(2156)), AdValue::sub(AdValue::square(scratch.ad_value(2156)), AdValue::mul(scratch.ad_value(2158), scratch.ad_value(2157)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2145, &AdValue::add(scratch.ad_value(2145), scratch.ad_value(2159)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2160, &AdValue::exp(scratch.ad_value(2159)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2146, &AdValue::div(scratch.ad_value(2146), scratch.ad_value(2160)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2148, &AdValue::mul(scratch.ad_value(2148), scratch.ad_value(2160)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2149, &AdValue::add(AdValue::offset(scratch.ad_value(2145), (-1.0)), scratch.ad_value(2146)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2150, &AdValue::mul(scratch.ad_value(2094), AdValue::sqrt(AdValue::add(scratch.ad_value(2148), scratch.ad_value(2149)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2161, &AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2146)), AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2151)), scratch.ad_value(2096)), 2.0)));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2138, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2138), scratch.ad_value(2160)), AdValue::add(scratch.ad_value(2154), scratch.ad_value(2147))), AdValue::add(scratch.ad_value(2161), AdValue::mul(scratch.ad_value(2160), scratch.ad_value(2147)))));
        }

        if (((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) && (scratch.values[2241] != 0.0)) {
            scratch.store_ad(2141, &AdValue::mul(scratch.ad_value(2138), scratch.ad_value(2011)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) {
            scratch.store_ad(2152, &AdValue::sqrt(scratch.ad_value(2149)));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2239] != 0.0))) {
            scratch.store_ad(2153, &AdValue::add(scratch.ad_value(2151), AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2094), AdValue::sub_from_scalar(1.0, scratch.ad_value(2146))), scratch.ad_value(2152)), 0.5)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2162, &AdValue::mul(scratch.ad_value(2011), AdValue::div(AdValue::mul(scratch.ad_value(2095), scratch.ad_value(2148)), AdValue::add(scratch.ad_value(2150), AdValue::mul(scratch.ad_value(2094), scratch.ad_value(2152))))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2163, &AdValue::add(scratch.ad_value(2162), AdValue::mul(scratch.ad_value(2011), scratch.ad_value(2153))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2164, &AdValue::mul(AdValue::mul(scratch.ad_value(2152), scratch.ad_value(2094)), scratch.ad_value(2011)));
        }

        scratch.values[2242] = if (scratch.values[235] < 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2242] != 0.0)) {
            scratch.store_ad(2076, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(235), scratch.ad_value(2162))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2242] != 0.0))) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(235), scratch.ad_value(2162)), 1.0)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2116, &AdValue::mul(scratch.ad_value(800), AdValue::mul(AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2076)), scratch.ad_value(2162))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2165, &AdValue::add(scratch.ad_value(2164), AdValue::mul(scratch.ad_value(818), scratch.ad_value(2162))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2166, &AdValue::add(scratch.ad_value(2164), AdValue::mul(scratch.ad_value(819), scratch.ad_value(2162))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2117, &AdValue::mul(scratch.ad_value(817), scratch.ad_value(2165)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2077, &AdValue::ln(AdValue::div(scratch.ad_value(2149), AdValue::offset(AdValue::add(scratch.ad_value(2149), scratch.ad_value(2148)), 1e-14))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2118, &AdValue::add(AdValue::pow(AdValue::mul(scratch.ad_value(2117), scratch.ad_value(748)), scratch.ad_value(749)), AdValue::mul(scratch.ad_value(750), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(751), 0.5), scratch.ad_value(2077))))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2119, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2118), 1.0), scratch.ad_value(2116)), scratch.ad_value(2111)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2167, &AdValue::ln(AdValue::div(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(854), scratch.ad_value(2141)), scratch.ad_value(821)), 1.0), AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2132), scratch.ad_value(2141)), scratch.ad_value(821)), 1.0))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2168, &AdValue::mul(scratch.ad_value(241), scratch.ad_value(2167)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2169, &AdValue::div_from_scalar(1.0, AdValue::add(AdValue::offset(scratch.ad_value(2168), 1.0), AdValue::square(scratch.ad_value(2168)))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2078, &AdValue::mul(scratch.ad_value(2162), scratch.ad_value(2120)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2121, &AdValue::scale(AdValue::div(scratch.ad_value(2078), AdValue::offset(scratch.ad_value(2078), 100.0)), 100.0));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2170, &AdValue::mul(scratch.ad_value(2119), scratch.ad_value(2169)));
        }

        scratch.values[2243] = if (scratch.values[239] < 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2243] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(239), scratch.ad_value(2121)))));
        }

        if ((!(scratch.values[2220] != 0.0)) && (!(scratch.values[2243] != 0.0))) {
            scratch.store_ad(2076, &AdValue::offset(AdValue::mul(scratch.ad_value(239), scratch.ad_value(2121)), 1.0));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2122, &AdValue::mul(scratch.ad_value(764), AdValue::div(scratch.ad_value(2076), scratch.ad_value(2170))));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2171, &AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2122)), scratch.ad_value(2141)), scratch.ad_value(2141)));
        }

        scratch.values[2244] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2220] != 0.0)) && (scratch.values[2244] != 0.0)) {
            scratch.store_ad(2171, &AdValue::div(scratch.ad_value(2171), AdValue::offset(AdValue::mul(scratch.ad_value(2122), scratch.ad_value(2141)), 1.0)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2172, &AdValue::scale(AdValue::mul(scratch.ad_value(2170), AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(scratch.ad_value(2171), 2.0), 1.0)), 1.0)), 0.5));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2173, &AdValue::div_from_scalar(1.0, scratch.ad_value(2172)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2174, &AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2011)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(scratch.ad_value(2170), scratch.ad_value(2173)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2175, &AdValue::mul(scratch.ad_value(2153), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2171), scratch.ad_value(2076)), scratch.ad_value(2076)), 0.5), 1.0)));
        }

        if (!(scratch.values[2220] != 0.0)) {
            scratch.store_ad(2176, &AdValue::div(AdValue::mul(scratch.ad_value(2076), scratch.ad_value(2163)), scratch.ad_value(2175)));
        }

        scratch.values[2245] = if (scratch.values[2092] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2245] != 0.0) {
            scratch.store_ad(2203, &AdValue::div_from_scalar(1.0, scratch.ad_value(2163)));
        }

        if (scratch.values[2245] != 0.0) {
            scratch.store_ad(2204, &AdValue::mul(scratch.ad_value(2162), scratch.ad_value(2203)));
        }

        if (scratch.values[2245] != 0.0) {
            scratch.store_ad(2205, &AdValue::mul(scratch.ad_value(2011), AdValue::mul(scratch.ad_value(2153), scratch.ad_value(2203))));
        }

        if (scratch.values[2245] != 0.0) {
            scratch.store_ad(2206, &AdValue::ln(AdValue::offset(AdValue::mul(scratch.ad_value(864), scratch.ad_value(821)), 1.0)));
        }

    }

    pub(super) fn stamp_transient_block_33(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (scratch.values[2245] != 0.0) {
            scratch.store_ad(2207, &AdValue::add(AdValue::add(scratch.ad_value(2168), AdValue::mul(scratch.ad_value(242), AdValue::mul(AdValue::mul(scratch.ad_value(2203), scratch.ad_value(2204)), scratch.ad_value(2167)))), AdValue::mul(scratch.ad_value(243), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2164), scratch.ad_value(2205)), scratch.ad_value(2205)), scratch.ad_value(2206)))));
        }

        if (scratch.values[2245] != 0.0) {
            scratch.store_ad(2202, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2207), 1.0), AdValue::square(scratch.ad_value(2207))), scratch.ad_value(2169)));
        }

        scratch.values[2013] = scratch.values[2092];
        scratch.node_derivatives[2013] = scratch.node_derivatives[2092];
        scratch.branch_derivatives[2013] = scratch.branch_derivatives[2092];

        scratch.values[879] = scratch.values[2166];
        scratch.node_derivatives[879] = scratch.node_derivatives[2166];
        scratch.branch_derivatives[879] = scratch.branch_derivatives[2166];

        scratch.values[880] = scratch.values[2174];
        scratch.node_derivatives[880] = scratch.node_derivatives[2174];
        scratch.branch_derivatives[880] = scratch.branch_derivatives[2174];

        scratch.values[2014] = scratch.values[2153];
        scratch.node_derivatives[2014] = scratch.node_derivatives[2153];
        scratch.branch_derivatives[2014] = scratch.branch_derivatives[2153];

        scratch.values[2015] = scratch.values[2141];
        scratch.node_derivatives[2015] = scratch.node_derivatives[2141];
        scratch.branch_derivatives[2015] = scratch.branch_derivatives[2141];

        scratch.values[2016] = scratch.values[2162];
        scratch.node_derivatives[2016] = scratch.node_derivatives[2162];
        scratch.branch_derivatives[2016] = scratch.branch_derivatives[2162];

        scratch.values[2017] = scratch.values[2163];
        scratch.node_derivatives[2017] = scratch.node_derivatives[2163];
        scratch.branch_derivatives[2017] = scratch.branch_derivatives[2163];

        scratch.values[881] = scratch.values[2169];
        scratch.node_derivatives[881] = scratch.node_derivatives[2169];
        scratch.branch_derivatives[881] = scratch.branch_derivatives[2169];

        scratch.values[2019] = scratch.values[2202];
        scratch.node_derivatives[2019] = scratch.node_derivatives[2202];
        scratch.branch_derivatives[2019] = scratch.branch_derivatives[2202];

        scratch.values[2018] = scratch.values[2176];
        scratch.node_derivatives[2018] = scratch.node_derivatives[2176];
        scratch.branch_derivatives[2018] = scratch.branch_derivatives[2176];

        scratch.values[882] = scratch.values[2151];
        scratch.node_derivatives[882] = scratch.node_derivatives[2151];
        scratch.branch_derivatives[882] = scratch.branch_derivatives[2151];

        scratch.values[883] = scratch.values[2172];
        scratch.node_derivatives[883] = scratch.node_derivatives[2172];
        scratch.branch_derivatives[883] = scratch.branch_derivatives[2172];

        scratch.values[2020] = scratch.values[2173];
        scratch.node_derivatives[2020] = scratch.node_derivatives[2173];
        scratch.branch_derivatives[2020] = scratch.branch_derivatives[2173];

        scratch.values[884] = scratch.values[2170];
        scratch.node_derivatives[884] = scratch.node_derivatives[2170];
        scratch.branch_derivatives[884] = scratch.branch_derivatives[2170];

        scratch.values[885] = scratch.values[2138];
        scratch.node_derivatives[885] = scratch.node_derivatives[2138];
        scratch.branch_derivatives[885] = scratch.branch_derivatives[2138];

        scratch.values[886] = scratch.values[2145];
        scratch.node_derivatives[886] = scratch.node_derivatives[2145];
        scratch.branch_derivatives[886] = scratch.branch_derivatives[2145];

        scratch.values[889] = scratch.values[2133];
        scratch.node_derivatives[889] = scratch.node_derivatives[2133];
        scratch.branch_derivatives[889] = scratch.branch_derivatives[2133];

        scratch.values[2051] = scratch.values[2119];
        scratch.node_derivatives[2051] = scratch.node_derivatives[2119];
        scratch.branch_derivatives[2051] = scratch.branch_derivatives[2119];

        scratch.values[2052] = scratch.values[2120];
        scratch.node_derivatives[2052] = scratch.node_derivatives[2120];
        scratch.branch_derivatives[2052] = scratch.branch_derivatives[2120];

        scratch.values[2053] = scratch.values[2132];
        scratch.node_derivatives[2053] = scratch.node_derivatives[2132];
        scratch.branch_derivatives[2053] = scratch.branch_derivatives[2132];

        scratch.values[2246] = if (scratch.values[2013] <= 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2246] != 0.0) {
            scratch.values[2021] = 0.0;
            scratch.node_derivatives[2021] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2021] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2246] != 0.0)) {
            scratch.store_ad(2021, &AdValue::mul(scratch.ad_value(760), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2019), scratch.ad_value(2017)), scratch.ad_value(2015)), scratch.ad_value(2020))));
        }

        scratch.values[2247] = if (((((scratch.values[2] != 0.0) && ((scratch.values[252] > 0.0) || (scratch.values[253] > 0.0))) || ((scratch.values[4] != 0.0) && ((scratch.values[258] > 0.0) || (scratch.values[259] > 0.0)))) || (scratch.values[267] > 0.0)) || (scratch.values[268] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2247] != 0.0) {
            scratch.store_ad(890, &AdValue::scale(AdValue::add(scratch.ad_value(871), AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(871)), scratch.ad_value(828)))), 0.5));
        }

        if (scratch.values[2247] != 0.0) {
            scratch.store_ad(891, &AdValue::add(AdValue::add(AdValue::sub(AdValue::neg(scratch.ad_value(890)), AdValue::scale(scratch.ad_value(826), 0.5)), AdValue::mul(scratch.ad_value(824), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(890), AdValue::scale(scratch.ad_value(826), 0.25)), scratch.ad_value(829))))), scratch.ad_value(830)));
        }

        if (scratch.values[2247] != 0.0) {
            scratch.store_ad(890, &AdValue::scale(AdValue::add(scratch.ad_value(872), AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(872)), scratch.ad_value(831)))), 0.5));
        }

        if (scratch.values[2247] != 0.0) {
            scratch.store_ad(892, &AdValue::add(AdValue::add(AdValue::sub(AdValue::neg(scratch.ad_value(890)), AdValue::scale(scratch.ad_value(827), 0.5)), AdValue::mul(scratch.ad_value(825), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(890), AdValue::scale(scratch.ad_value(827), 0.25)), scratch.ad_value(832))))), scratch.ad_value(833)));
        }

        if (scratch.values[2247] != 0.0) {
            scratch.store_ad(893, &AdValue::scale(AdValue::add(scratch.ad_value(871), scratch.ad_value(891)), (-scratch.values[356])));
        }

        if (scratch.values[2247] != 0.0) {
            scratch.store_ad(894, &AdValue::scale(AdValue::add(scratch.ad_value(872), scratch.ad_value(892)), (-scratch.values[356])));
        }

        scratch.values[2025] = 0.0;

        scratch.values[2024] = 0.0;

        scratch.values[921] = 0.0;

        scratch.values[922] = 0.0;

        scratch.values[2027] = 0.0;

        scratch.values[2026] = 0.0;

        scratch.values[2248] = if (scratch.values[2] != 0.0) { 1.0 } else { 0.0 };

        scratch.values[2249] = if (scratch.values[252] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(895, &AdValue::mul(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(893)), 1e-6)), scratch.ad_value(834)));
        }

        scratch.values[2250] = if (scratch.values[256] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) && (scratch.values[2250] != 0.0)) {
            scratch.store_ad(895, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(895), scratch.ad_value(839)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(895), scratch.ad_value(839)), AdValue::sub(scratch.ad_value(895), scratch.ad_value(839))), 1e-6))), 0.5));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(scratch.ad_value(837), AdValue::offset(AdValue::mul(scratch.ad_value(895), AdValue::add(scratch.ad_value(255), AdValue::mul(scratch.ad_value(256), scratch.ad_value(895)))), (-1.5))));
        }

        scratch.values[2251] = if (scratch.values[2076] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) && (scratch.values[2251] != 0.0)) {
            scratch.store_ad(896, &AdValue::offset(AdValue::mul(scratch.ad_value(2076), AdValue::offset(AdValue::scale(AdValue::mul(scratch.ad_value(2076), AdValue::offset(AdValue::scale(scratch.ad_value(2076), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        scratch.values[2252] = if (scratch.values[2076] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) && (!(scratch.values[2251] != 0.0))) && (scratch.values[2252] != 0.0)) {
            scratch.store_ad(896, &AdValue::exp(scratch.ad_value(2076)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) && (!(scratch.values[2251] != 0.0))) && (!(scratch.values[2252] != 0.0))) {
            scratch.store_ad(896, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(897, &AdValue::offset(scratch.ad_value(891), 3.0));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(898, &AdValue::sub_from_scalar((-3.0), scratch.ad_value(250)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(899, &AdValue::scale(scratch.ad_value(860), 30.0));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.values[960] = (4.0 - 0.9);
            scratch.node_derivatives[960] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[960] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(961, &AdValue::add(scratch.ad_value(897), scratch.ad_value(899)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(AdValue::div_from_scalar(2.0, scratch.ad_value(960)), AdValue::sub(scratch.ad_value(961), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(961)), AdValue::mul(AdValue::mul(scratch.ad_value(960), scratch.ad_value(897)), scratch.ad_value(899)))))));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.values[960] = (4.0 - 0.3);
            scratch.node_derivatives[960] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[960] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(961, &AdValue::add(scratch.ad_value(898), scratch.ad_value(2076)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(900, &AdValue::mul(AdValue::div_from_scalar(2.0, scratch.ad_value(960)), AdValue::add(scratch.ad_value(961), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(961)), AdValue::mul(AdValue::mul(scratch.ad_value(960), scratch.ad_value(898)), scratch.ad_value(2076)))))));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(2025, &AdValue::mul(scratch.ad_value(252), AdValue::mul(scratch.ad_value(896), scratch.ad_value(900))));
        }

        scratch.values[2253] = if (scratch.values[253] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(895, &AdValue::mul(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(894)), 1e-6)), scratch.ad_value(834)));
        }

        scratch.values[2254] = if (scratch.values[256] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) && (scratch.values[2254] != 0.0)) {
            scratch.store_ad(895, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(895), scratch.ad_value(839)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(895), scratch.ad_value(839)), AdValue::sub(scratch.ad_value(895), scratch.ad_value(839))), 1e-6))), 0.5));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(scratch.ad_value(838), AdValue::offset(AdValue::mul(scratch.ad_value(895), AdValue::add(scratch.ad_value(255), AdValue::mul(scratch.ad_value(256), scratch.ad_value(895)))), (-1.5))));
        }

        scratch.values[2255] = if (scratch.values[2076] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(896, &AdValue::offset(AdValue::mul(scratch.ad_value(2076), AdValue::offset(AdValue::scale(AdValue::mul(scratch.ad_value(2076), AdValue::offset(AdValue::scale(scratch.ad_value(2076), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        scratch.values[2256] = if (scratch.values[2076] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) && (!(scratch.values[2255] != 0.0))) && (scratch.values[2256] != 0.0)) {
            scratch.store_ad(896, &AdValue::exp(scratch.ad_value(2076)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) && (!(scratch.values[2255] != 0.0))) && (!(scratch.values[2256] != 0.0))) {
            scratch.store_ad(896, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(897, &AdValue::offset(scratch.ad_value(892), 3.0));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(898, &AdValue::sub_from_scalar((-3.0), scratch.ad_value(250)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(899, &AdValue::scale(scratch.ad_value(863), 30.0));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.values[960] = (4.0 - 0.9);
            scratch.node_derivatives[960] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[960] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(961, &AdValue::add(scratch.ad_value(897), scratch.ad_value(899)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(AdValue::div_from_scalar(2.0, scratch.ad_value(960)), AdValue::sub(scratch.ad_value(961), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(961)), AdValue::mul(AdValue::mul(scratch.ad_value(960), scratch.ad_value(897)), scratch.ad_value(899)))))));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.values[960] = (4.0 - 0.3);
            scratch.node_derivatives[960] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[960] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(961, &AdValue::add(scratch.ad_value(898), scratch.ad_value(2076)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(900, &AdValue::mul(AdValue::div_from_scalar(2.0, scratch.ad_value(960)), AdValue::add(scratch.ad_value(961), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(961)), AdValue::mul(AdValue::mul(scratch.ad_value(960), scratch.ad_value(898)), scratch.ad_value(2076)))))));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2024, &AdValue::mul(scratch.ad_value(253), AdValue::mul(scratch.ad_value(896), scratch.ad_value(900))));
        }

        scratch.values[2257] = if (scratch.values[251] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2258] = if (scratch.values[2013] <= 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2258] != 0.0)) {
            scratch.store_ad(2076, &AdValue::pow(AdValue::div(scratch.ad_value(854), scratch.ad_value(869)), scratch.ad_value(240)));
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2258] != 0.0)) {
            scratch.store_ad(889, &AdValue::mul(AdValue::mul(scratch.ad_value(854), AdValue::pow(AdValue::offset(scratch.ad_value(2076), 1.0), AdValue::neg(scratch.ad_value(820)))), scratch.ad_value(2012)));
        }

        scratch.values[2259] = if ((scratch.values[885] - scratch.values[889]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2259] != 0.0)) {
            scratch.store_ad(2076, &AdValue::exp(AdValue::sub(scratch.ad_value(885), scratch.ad_value(889))));
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2259] != 0.0))) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(885), scratch.ad_value(889))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(885), scratch.ad_value(889))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(885), scratch.ad_value(889))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(901, &AdValue::add(scratch.ad_value(873), AdValue::mul(scratch.ad_value(2011), AdValue::sub(AdValue::scale(scratch.ad_value(885), 0.5), AdValue::ln(AdValue::scale(AdValue::offset(scratch.ad_value(2076), 1.0), 0.5))))));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(902, &AdValue::mul(scratch.ad_value(250), scratch.ad_value(2011)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(903, &AdValue::add(scratch.ad_value(880), scratch.ad_value(902)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(904, &AdValue::scale(AdValue::sub(scratch.ad_value(903), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::neg(scratch.ad_value(903)), AdValue::neg(scratch.ad_value(903))), 0.01))), 0.5));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(895, &AdValue::mul(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(880)), 1e-6)), scratch.ad_value(834)));
        }

        scratch.values[2260] = if (scratch.values[256] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2260] != 0.0)) {
            scratch.store_ad(895, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(895), scratch.ad_value(839)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(895), scratch.ad_value(839)), AdValue::sub(scratch.ad_value(895), scratch.ad_value(839))), 1e-6))), 0.5));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(905, &AdValue::add(scratch.ad_value(886), AdValue::mul(AdValue::sub(AdValue::sub(scratch.ad_value(904), scratch.ad_value(785)), scratch.ad_value(901)), scratch.ad_value(2012))));
        }

        scratch.values[2261] = if (((scratch.values[905]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2261] != 0.0)) {
            scratch.store_ad(906, &AdValue::exp(scratch.ad_value(905)));
        }

        scratch.values[2262] = if (scratch.values[905] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2261] != 0.0))) && (scratch.values[2262] != 0.0)) {
            scratch.store_ad(906, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(905)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(905)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(905)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2261] != 0.0))) && (!(scratch.values[2262] != 0.0))) {
            scratch.store_ad(906, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(905), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(905), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(905), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(905, &AdValue::mul(AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(853), scratch.ad_value(873)), scratch.ad_value(901))), scratch.ad_value(2012)));
        }

        scratch.values[2263] = if (((scratch.values[905]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2263] != 0.0)) {
            scratch.store_ad(2076, &AdValue::exp(scratch.ad_value(905)));
        }

        scratch.values[2264] = if (scratch.values[905] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2263] != 0.0))) && (scratch.values[2264] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(905)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(905)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(905)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2263] != 0.0))) && (!(scratch.values[2264] != 0.0))) {
            scratch.store_ad(2076, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(905), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(905), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(905), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(907, &AdValue::mul(scratch.ad_value(906), scratch.ad_value(2076)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(2076, &AdValue::mul(scratch.ad_value(836), AdValue::offset(AdValue::mul(scratch.ad_value(895), AdValue::add(scratch.ad_value(255), AdValue::mul(scratch.ad_value(256), scratch.ad_value(895)))), (-1.5))));
        }

        scratch.values[2265] = if (scratch.values[2076] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2265] != 0.0)) {
            scratch.store_ad(896, &AdValue::offset(AdValue::mul(scratch.ad_value(2076), AdValue::offset(AdValue::scale(AdValue::mul(scratch.ad_value(2076), AdValue::offset(AdValue::scale(scratch.ad_value(2076), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        scratch.values[2266] = if (scratch.values[2076] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2265] != 0.0))) && (scratch.values[2266] != 0.0)) {
            scratch.store_ad(896, &AdValue::exp(scratch.ad_value(2076)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2265] != 0.0))) && (!(scratch.values[2266] != 0.0))) {
            scratch.store_ad(896, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(908, &AdValue::mul(scratch.ad_value(251), AdValue::mul(scratch.ad_value(896), AdValue::ln(AdValue::div(AdValue::offset(scratch.ad_value(906), 1.0), AdValue::offset(scratch.ad_value(907), 1.0))))));
        }

        scratch.values[2267] = if ((scratch.values[2013] <= 0.0) || ((scratch.values[255] == 0.0) && (scratch.values[256] == 0.0))) { 1.0 } else { 0.0 };

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2267] != 0.0)) {
            scratch.values[909] = 1.0;
            scratch.node_derivatives[909] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[909] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (scratch.values[2267] != 0.0)) {
            scratch.values[910] = 0.5;
            scratch.node_derivatives[910] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[910] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) {
            scratch.store_ad(2076, &AdValue::add(scratch.ad_value(255), AdValue::mul(AdValue::scale(scratch.ad_value(256), 2.0), scratch.ad_value(895))));
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) {
            scratch.store_ad(911, &AdValue::div(scratch.ad_value(257), AdValue::mul(scratch.ad_value(2076), scratch.ad_value(836))));
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) {
            scratch.store_ad(912, &AdValue::scale(AdValue::div(scratch.ad_value(2015), scratch.ad_value(911)), 0.5));
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) {
            scratch.store_ad(913, &AdValue::div(scratch.ad_value(911), scratch.ad_value(2018)));
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) {
            scratch.store_ad(914, &AdValue::scale(AdValue::mul(scratch.ad_value(913), AdValue::sub_from_scalar(1.0, scratch.ad_value(913))), 0.5));
        }

        if (((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) {
            scratch.store_ad(915, &AdValue::sub_from_scalar(0.5, AdValue::scale(scratch.ad_value(914), 3.0)));
        }

        scratch.values[2268] = if (scratch.values[912] < 0.001) { 1.0 } else { 0.0 };

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (scratch.values[2268] != 0.0)) {
            scratch.store_ad(916, &AdValue::square(scratch.ad_value(912)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (scratch.values[2268] != 0.0)) {
            scratch.store_ad(909, &AdValue::offset(AdValue::mul(scratch.ad_value(916), AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(913), 0.3333333333333333), 0.16666666666666666), AdValue::scale(AdValue::mul(scratch.ad_value(916), AdValue::offset(AdValue::scale(scratch.ad_value(913), 0.2), 0.05)), 0.16666666666666666))), 1.0));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (scratch.values[2268] != 0.0)) {
            scratch.store_ad(910, &AdValue::sub(AdValue::scale(scratch.ad_value(909), 0.5), AdValue::scale(AdValue::mul(scratch.ad_value(912), AdValue::offset(AdValue::mul(scratch.ad_value(916), AdValue::add(AdValue::scale(AdValue::offset(scratch.ad_value(914), 0.25), 0.4), AdValue::scale(AdValue::mul(scratch.ad_value(916), AdValue::offset(scratch.ad_value(914), 0.125)), 0.0285714285714))), 1.0)), 0.16666666666666666)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) {
            scratch.store_ad(917, &AdValue::div_from_scalar(1.0, scratch.ad_value(912)));
        }

        scratch.values[2269] = if (((scratch.values[912]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) && (scratch.values[2269] != 0.0)) {
            scratch.store_ad(918, &AdValue::exp(scratch.ad_value(912)));
        }

        scratch.values[2270] = if (scratch.values[912] < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) && (!(scratch.values[2269] != 0.0))) && (scratch.values[2270] != 0.0)) {
            scratch.store_ad(918, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(912)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(912)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(912)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) && (!(scratch.values[2269] != 0.0))) && (!(scratch.values[2270] != 0.0))) {
            scratch.store_ad(918, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(912), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(912), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(912), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) {
            scratch.store_ad(919, &AdValue::div_from_scalar(1.0, scratch.ad_value(918)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) {
            scratch.store_ad(2076, &AdValue::sub(scratch.ad_value(918), scratch.ad_value(919)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) {
            scratch.store_ad(2078, &AdValue::add(scratch.ad_value(918), scratch.ad_value(919)));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) {
            scratch.store_ad(909, &AdValue::scale(AdValue::add(AdValue::mul(AdValue::mul(AdValue::sub_from_scalar(1.0, scratch.ad_value(913)), scratch.ad_value(2076)), scratch.ad_value(917)), AdValue::mul(scratch.ad_value(913), scratch.ad_value(2078))), 0.5));
        }

        if ((((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) && (!(scratch.values[2267] != 0.0))) && (!(scratch.values[2268] != 0.0))) {
            scratch.store_ad(910, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(909), AdValue::mul(scratch.ad_value(2076), AdValue::sub(scratch.ad_value(914), AdValue::mul(AdValue::mul(scratch.ad_value(915), scratch.ad_value(917)), scratch.ad_value(917))))), AdValue::mul(AdValue::mul(scratch.ad_value(915), scratch.ad_value(2078)), scratch.ad_value(917))), 0.5));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(920, &AdValue::scale(AdValue::offset(AdValue::div(scratch.ad_value(2013), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2013)), 1e-6))), 1.0), 0.5));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(921, &AdValue::mul(AdValue::mul(scratch.ad_value(908), scratch.ad_value(909)), scratch.ad_value(920)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(2026, &AdValue::mul(AdValue::mul(scratch.ad_value(908), scratch.ad_value(910)), scratch.ad_value(920)));
        }

    }

    pub(super) fn stamp_transient_block_34(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(2027, &AdValue::sub(scratch.ad_value(921), scratch.ad_value(2026)));
        }

        if ((scratch.values[2248] != 0.0) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(922, &AdValue::mul(AdValue::mul(scratch.ad_value(908), scratch.ad_value(909)), AdValue::sub_from_scalar(1.0, scratch.ad_value(920))));
        }

        scratch.values[924] = 0.0;

        scratch.values[926] = 0.0;

        scratch.values[2271] = if (scratch.values[4] != 0.0) { 1.0 } else { 0.0 };

        scratch.values[2272] = if ((scratch.values[259] > 0.0) && (scratch.values[894] < 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2271] != 0.0) && (scratch.values[2272] != 0.0)) {
            scratch.store_ad(923, &AdValue::sqrt(AdValue::offset(AdValue::add(AdValue::square(scratch.ad_value(894)), AdValue::mul(AdValue::square(scratch.ad_value(265)), AdValue::square(scratch.ad_value(862)))), 1e-6)));
        }

        if ((scratch.values[2271] != 0.0) && (scratch.values[2272] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div(AdValue::neg(scratch.ad_value(844)), scratch.ad_value(923)));
        }

        scratch.values[2273] = if (scratch.values[2076] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2271] != 0.0) && (scratch.values[2272] != 0.0)) && (scratch.values[2273] != 0.0)) {
            scratch.store_ad(2078, &AdValue::exp(scratch.ad_value(2076)));
        }

        if (((scratch.values[2271] != 0.0) && (scratch.values[2272] != 0.0)) && (!(scratch.values[2273] != 0.0))) {
            scratch.store_ad(2078, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2271] != 0.0) && (scratch.values[2272] != 0.0)) {
            scratch.store_ad(924, &AdValue::mul(AdValue::neg(scratch.ad_value(842)), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(862), scratch.ad_value(894)), scratch.ad_value(923)), scratch.ad_value(2078))));
        }

        scratch.values[2274] = if ((scratch.values[258] > 0.0) && (scratch.values[893] < 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2271] != 0.0) && (scratch.values[2274] != 0.0)) {
            scratch.store_ad(925, &AdValue::sqrt(AdValue::offset(AdValue::add(AdValue::square(scratch.ad_value(893)), AdValue::mul(AdValue::square(scratch.ad_value(264)), AdValue::square(scratch.ad_value(861)))), 1e-6)));
        }

        if ((scratch.values[2271] != 0.0) && (scratch.values[2274] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div(AdValue::neg(scratch.ad_value(843)), scratch.ad_value(925)));
        }

        scratch.values[2275] = if (scratch.values[2076] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2271] != 0.0) && (scratch.values[2274] != 0.0)) && (scratch.values[2275] != 0.0)) {
            scratch.store_ad(2078, &AdValue::exp(scratch.ad_value(2076)));
        }

        if (((scratch.values[2271] != 0.0) && (scratch.values[2274] != 0.0)) && (!(scratch.values[2275] != 0.0))) {
            scratch.store_ad(2078, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2076)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2271] != 0.0) && (scratch.values[2274] != 0.0)) {
            scratch.store_ad(926, &AdValue::mul(AdValue::neg(scratch.ad_value(841)), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(861), scratch.ad_value(893)), scratch.ad_value(925)), scratch.ad_value(2078))));
        }

        scratch.values[2061] = 0.0;

        scratch.values[2067] = 0.0;

        scratch.values[2069] = 0.0;

        scratch.values[2070] = 1e-40;

        scratch.values[2071] = 1.0;

        scratch.values[2072] = 0.0;

        scratch.values[2276] = if ((scratch.values[8] != 0.0) && (scratch.values[283] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2076, &AdValue::add(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(856), scratch.ad_value(855)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(856), scratch.ad_value(855)), AdValue::sub(scratch.ad_value(856), scratch.ad_value(855))), scratch.ad_value(807)))), 0.5), scratch.ad_value(805)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2054, &AdValue::add(AdValue::sub(scratch.ad_value(855), AdValue::scale(AdValue::sub(scratch.ad_value(2076), AdValue::sqrt(AdValue::add(AdValue::mul(scratch.ad_value(2076), scratch.ad_value(2076)), scratch.ad_value(806)))), 0.5)), scratch.ad_value(809)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2055, &AdValue::add(scratch.ad_value(2054), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(864)), 0.5)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2056, &AdValue::mul(AdValue::mul(scratch.ad_value(285), AdValue::offset(AdValue::mul(scratch.ad_value(287), scratch.ad_value(864)), 1.0)), AdValue::offset(AdValue::mul(scratch.ad_value(286), scratch.ad_value(2055)), 1.0)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2057, &AdValue::mul(scratch.ad_value(766), AdValue::offset(scratch.ad_value(2056), 1.0)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2058, &AdValue::div_from_scalar(1.0, scratch.ad_value(2057)));
        }

        scratch.values[2277] = if (scratch.values[289] < 0.05) { 1.0 } else { 0.0 };

        if ((scratch.values[2276] != 0.0) && (scratch.values[2277] != 0.0)) {
            scratch.values[2059] = scratch.values[864];
            scratch.node_derivatives[2059] = scratch.node_derivatives[864];
            scratch.branch_derivatives[2059] = scratch.branch_derivatives[864];
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2277] != 0.0))) {
            scratch.store_ad(2059, &AdValue::div(AdValue::scale(AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(289), scratch.ad_value(864)), 1.0)), (-1.0)), 2.0), scratch.ad_value(289)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2060, &AdValue::mul(AdValue::mul(scratch.ad_value(288), scratch.ad_value(2059)), AdValue::offset(AdValue::mul(scratch.ad_value(290), scratch.ad_value(2055)), 1.0)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2061, &AdValue::mul(scratch.ad_value(2058), AdValue::sub(AdValue::add(AdValue::add(scratch.ad_value(853), scratch.ad_value(2054)), scratch.ad_value(2060)), scratch.ad_value(757))));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2062, &AdValue::mul(scratch.ad_value(2058), scratch.ad_value(803)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2063, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::div(scratch.ad_value(2062), scratch.ad_value(804)), AdValue::sqrt(scratch.ad_value(2062)))), 2.0));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2064, &AdValue::mul(scratch.ad_value(2058), scratch.ad_value(2054)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(967, &AdValue::add(scratch.ad_value(2062), scratch.ad_value(2064)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(968, &AdValue::add(scratch.ad_value(967), AdValue::mul(scratch.ad_value(804), AdValue::sqrt(scratch.ad_value(967)))));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(969, &AdValue::add(scratch.ad_value(968), scratch.ad_value(2063)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(970, &AdValue::offset(AdValue::div(scratch.ad_value(804), AdValue::scale(AdValue::sqrt(scratch.ad_value(967)), 2.0)), 1.0));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(971, &AdValue::div_from_scalar(1.0, scratch.ad_value(970)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(972, &AdValue::sub(scratch.ad_value(2061), scratch.ad_value(969)));
        }

        scratch.values[2278] = if (scratch.values[972] > (-12.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(973, &AdValue::offset(AdValue::add(scratch.ad_value(972), scratch.ad_value(768)), (-1.0)));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(974, &AdValue::scale(AdValue::add(scratch.ad_value(973), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(973)), 10.0))), 0.5));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(975, &AdValue::add(AdValue::sub(scratch.ad_value(972), AdValue::mul(scratch.ad_value(970), AdValue::ln(scratch.ad_value(974)))), scratch.ad_value(768)));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(976, &AdValue::scale(AdValue::add(scratch.ad_value(975), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(975)), 2.0))), 0.5));
        }

        scratch.values[2279] = if ((scratch.values[972] - scratch.values[976]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) && (scratch.values[2279] != 0.0)) {
            scratch.store_ad(977, &AdValue::exp(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976))));
        }

        if (((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) && (!(scratch.values[2279] != 0.0))) {
            scratch.store_ad(977, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(978, &AdValue::mul(scratch.ad_value(767), scratch.ad_value(977)));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(979, &AdValue::pow(scratch.ad_value(978), scratch.ad_value(971)));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(980, &AdValue::add(AdValue::square(scratch.ad_value(970)), AdValue::mul(AdValue::sub(AdValue::scale(AdValue::add(scratch.ad_value(976), scratch.ad_value(970)), 2.0), scratch.ad_value(979)), scratch.ad_value(979))));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(981, &AdValue::mul(scratch.ad_value(970), AdValue::offset(AdValue::div(AdValue::sub(AdValue::sqrt(scratch.ad_value(980)), scratch.ad_value(970)), scratch.ad_value(979)), (-1.0))));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(2065, &AdValue::sub(scratch.ad_value(976), scratch.ad_value(981)));
        }

        scratch.values[2280] = if ((scratch.values[971] * (scratch.values[972] + scratch.values[768])) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2278] != 0.0))) && (scratch.values[2280] != 0.0)) {
            scratch.store_ad(2065, &AdValue::exp(AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2278] != 0.0))) && (!(scratch.values[2280] != 0.0))) {
            let assign46300_ad_e59603: AdValue = AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))), 0.3333333333333333), 1.0)), 0.5), 1.0));
            scratch.store_ad(2065, &AdValue::div_from_scalar(1e-100, AdValue::offset(assign46300_ad_e59603, 1.0)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2066, &AdValue::mul(scratch.ad_value(2058), AdValue::add(scratch.ad_value(2053), scratch.ad_value(2054))));
        }

        scratch.values[2281] = if ((scratch.values[2065] < 0.001) && (scratch.values[2053] < 1e-6)) { 1.0 } else { 0.0 };

        scratch.values[2282] = if (((-scratch.values[2066]) + scratch.values[2064]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2276] != 0.0) && (scratch.values[2281] != 0.0)) && (scratch.values[2282] != 0.0)) {
            scratch.store_ad(2076, &AdValue::exp(AdValue::sub(scratch.ad_value(2064), scratch.ad_value(2066))));
        }

        if (((scratch.values[2276] != 0.0) && (scratch.values[2281] != 0.0)) && (!(scratch.values[2282] != 0.0))) {
            let assign46350_ad_e59682: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(2064), scratch.ad_value(2066))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(2064), scratch.ad_value(2066))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(2064), scratch.ad_value(2066))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(2076, &assign46350_ad_e59682);
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2281] != 0.0)) {
            scratch.store_ad(2067, &AdValue::mul(scratch.ad_value(2065), AdValue::offset(scratch.ad_value(2076), (-1.0))));
        }

        if ((scratch.values[2276] != 0.0) && (scratch.values[2281] != 0.0)) {
            scratch.store_ad(2068, &AdValue::add(scratch.ad_value(2067), scratch.ad_value(2065)));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(967, &AdValue::add(scratch.ad_value(2062), scratch.ad_value(2066)));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(968, &AdValue::add(scratch.ad_value(967), AdValue::mul(scratch.ad_value(804), AdValue::sqrt(scratch.ad_value(967)))));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(969, &AdValue::add(scratch.ad_value(968), scratch.ad_value(2063)));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(970, &AdValue::offset(AdValue::div(scratch.ad_value(804), AdValue::scale(AdValue::sqrt(scratch.ad_value(967)), 2.0)), 1.0));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(971, &AdValue::div_from_scalar(1.0, scratch.ad_value(970)));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(972, &AdValue::sub(scratch.ad_value(2061), scratch.ad_value(969)));
        }

        scratch.values[2283] = if (scratch.values[972] > (-12.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(973, &AdValue::offset(AdValue::add(scratch.ad_value(972), scratch.ad_value(768)), (-1.0)));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(974, &AdValue::scale(AdValue::add(scratch.ad_value(973), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(973)), 10.0))), 0.5));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(975, &AdValue::add(AdValue::sub(scratch.ad_value(972), AdValue::mul(scratch.ad_value(970), AdValue::ln(scratch.ad_value(974)))), scratch.ad_value(768)));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(976, &AdValue::scale(AdValue::add(scratch.ad_value(975), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(975)), 2.0))), 0.5));
        }

        scratch.values[2284] = if ((scratch.values[972] - scratch.values[976]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) && (scratch.values[2284] != 0.0)) {
            scratch.store_ad(977, &AdValue::exp(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976))));
        }

        if ((((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) && (!(scratch.values[2284] != 0.0))) {
            scratch.store_ad(977, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(972), scratch.ad_value(976)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(978, &AdValue::mul(scratch.ad_value(767), scratch.ad_value(977)));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(979, &AdValue::pow(scratch.ad_value(978), scratch.ad_value(971)));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(980, &AdValue::add(AdValue::square(scratch.ad_value(970)), AdValue::mul(AdValue::sub(AdValue::scale(AdValue::add(scratch.ad_value(976), scratch.ad_value(970)), 2.0), scratch.ad_value(979)), scratch.ad_value(979))));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(981, &AdValue::mul(scratch.ad_value(970), AdValue::offset(AdValue::div(AdValue::sub(AdValue::sqrt(scratch.ad_value(980)), scratch.ad_value(970)), scratch.ad_value(979)), (-1.0))));
        }

        if (((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(2068, &AdValue::sub(scratch.ad_value(976), scratch.ad_value(981)));
        }

        scratch.values[2285] = if ((scratch.values[971] * (scratch.values[972] + scratch.values[768])) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2283] != 0.0))) && (scratch.values[2285] != 0.0)) {
            scratch.store_ad(2068, &AdValue::exp(AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))));
        }

        if ((((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2283] != 0.0))) && (!(scratch.values[2285] != 0.0))) {
            let assign46590_ad_e60035: AdValue = AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(971), AdValue::add(scratch.ad_value(972), scratch.ad_value(768)))), 0.3333333333333333), 1.0)), 0.5), 1.0));
            scratch.store_ad(2068, &AdValue::div_from_scalar(1e-100, AdValue::offset(assign46590_ad_e60035, 1.0)));
        }

        if ((scratch.values[2276] != 0.0) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(2067, &AdValue::sub(scratch.ad_value(2068), scratch.ad_value(2065)));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2069, &AdValue::scale(AdValue::add(scratch.ad_value(2068), scratch.ad_value(2065)), 0.5));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2070, &AdValue::max_with_scalar(AdValue::sub(scratch.ad_value(2061), scratch.ad_value(2069)), 1e-40));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2071, &AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(804), 0.5), AdValue::sqrt(AdValue::add(scratch.ad_value(2070), AdValue::scale(scratch.ad_value(767), 0.25))))));
        }

        if (scratch.values[2276] != 0.0) {
            scratch.store_ad(2072, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::neg(scratch.ad_value(761)), scratch.ad_value(2057)), scratch.ad_value(2057)), AdValue::offset(AdValue::mul(scratch.ad_value(2071), scratch.ad_value(2069)), 1.0)), scratch.ad_value(2067)), scratch.ad_value(2051)));
        }

        scratch.values[2286] = if ((scratch.values[2013] > 0.0) && (scratch.values[3] != 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2286] != 0.0) {
            scratch.store_ad(927, &AdValue::sub(scratch.ad_value(854), AdValue::mul(scratch.ad_value(248), scratch.ad_value(2015))));
        }

        scratch.values[2287] = if (scratch.values[927] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2286] != 0.0) && (scratch.values[2287] != 0.0)) {
            scratch.store_ad(2078, &AdValue::mul(scratch.ad_value(756), AdValue::div(AdValue::offset(AdValue::mul(scratch.ad_value(249), AdValue::sub(AdValue::sqrt(AdValue::add(scratch.ad_value(771), scratch.ad_value(873))), scratch.ad_value(779))), 1.0), AdValue::offset(scratch.ad_value(927), 1e-30))));
        }

        scratch.values[2288] = if ((((-scratch.values[2078])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2286] != 0.0) && (scratch.values[2287] != 0.0)) && (scratch.values[2288] != 0.0)) {
            scratch.store_ad(2076, &AdValue::exp(AdValue::neg(scratch.ad_value(2078))));
        }

        scratch.values[2289] = if ((-scratch.values[2078]) < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2286] != 0.0) && (scratch.values[2287] != 0.0)) && (!(scratch.values[2288] != 0.0))) && (scratch.values[2289] != 0.0)) {
            scratch.store_ad(2076, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2078))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2078))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2078))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2286] != 0.0) && (scratch.values[2287] != 0.0)) && (!(scratch.values[2288] != 0.0))) && (!(scratch.values[2289] != 0.0))) {
            scratch.store_ad(2076, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::neg(scratch.ad_value(2078)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::neg(scratch.ad_value(2078)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::neg(scratch.ad_value(2078)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((scratch.values[2286] != 0.0) && (scratch.values[2287] != 0.0)) {
            scratch.store_ad(2023, &AdValue::mul(scratch.ad_value(245), AdValue::mul(scratch.ad_value(927), scratch.ad_value(2076))));
        }

        if ((scratch.values[2286] != 0.0) && (scratch.values[2287] != 0.0)) {
            scratch.store_ad(2022, &AdValue::mul(scratch.ad_value(2023), AdValue::add(scratch.ad_value(2021), scratch.ad_value(2072))));
        }

        scratch.values[2404] = if ((scratch.values[7] == 1.0) || (scratch.values[9] != 0.0)) { 1.0 } else { 0.0 };

        scratch.values[2405] = if (scratch.values[9] != 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2405] != 0.0)) {
            scratch.store_ad(2076, &AdValue::add(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(856), scratch.ad_value(855)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(856), scratch.ad_value(855)), AdValue::sub(scratch.ad_value(856), scratch.ad_value(855))), scratch.ad_value(792)))), 0.5), scratch.ad_value(790)));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2405] != 0.0)) {
            scratch.store_ad(928, &AdValue::add(AdValue::sub(scratch.ad_value(855), AdValue::scale(AdValue::sub(scratch.ad_value(2076), AdValue::sqrt(AdValue::add(AdValue::mul(scratch.ad_value(2076), scratch.ad_value(2076)), scratch.ad_value(791)))), 0.5)), scratch.ad_value(793)));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2405] != 0.0)) {
            scratch.values[2292] = scratch.values[928];
            scratch.node_derivatives[2292] = scratch.node_derivatives[928];
            scratch.branch_derivatives[2292] = scratch.branch_derivatives[928];
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2405] != 0.0)) {
            scratch.values[2290] = scratch.values[788];
            scratch.node_derivatives[2290] = scratch.node_derivatives[788];
            scratch.branch_derivatives[2290] = scratch.branch_derivatives[788];
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2405] != 0.0)) {
            scratch.values[2291] = scratch.values[789];
            scratch.node_derivatives[2291] = scratch.node_derivatives[789];
            scratch.branch_derivatives[2291] = scratch.branch_derivatives[789];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2405] != 0.0))) {
            scratch.values[2292] = scratch.values[874];
            scratch.node_derivatives[2292] = scratch.node_derivatives[874];
            scratch.branch_derivatives[2292] = scratch.branch_derivatives[874];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2405] != 0.0))) {
            scratch.values[2290] = scratch.values[771];
            scratch.node_derivatives[2290] = scratch.node_derivatives[771];
            scratch.branch_derivatives[2290] = scratch.branch_derivatives[771];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2405] != 0.0))) {
            scratch.values[2291] = scratch.values[772];
            scratch.node_derivatives[2291] = scratch.node_derivatives[772];
            scratch.branch_derivatives[2291] = scratch.branch_derivatives[772];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2355] = 0.0;
            scratch.node_derivatives[2355] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2355] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2371] = 1.0;
            scratch.node_derivatives[2371] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2371] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2370] = 0.0;
            scratch.node_derivatives[2370] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2370] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2366] = 0.0;
            scratch.node_derivatives[2366] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2366] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2343] = 0.0;
            scratch.node_derivatives[2343] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2343] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2364] = 0.0;
            scratch.node_derivatives[2364] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2364] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2365] = 0.0;
            scratch.node_derivatives[2365] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2365] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2378] = 1.0;
            scratch.node_derivatives[2378] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2378] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2369] = 0.0;
            scratch.node_derivatives[2369] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2369] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2353] = 1.0;
            scratch.node_derivatives[2353] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2353] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2374] = 1.0;
            scratch.node_derivatives[2374] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2374] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2375] = 1.0;
            scratch.node_derivatives[2375] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2375] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2400] = 0.0;
            scratch.node_derivatives[2400] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2400] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2305] = 0.0;
            scratch.node_derivatives[2305] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2305] = [0.0; Instance::BRANCH_COUNT];
        }

    }

    pub(super) fn stamp_transient_block_35(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (scratch.values[2404] != 0.0) {
            scratch.values[2354] = 0.0;
            scratch.node_derivatives[2354] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2354] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2322] = 0.0;
            scratch.node_derivatives[2322] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2322] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2317] = 0.0;
            scratch.node_derivatives[2317] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2317] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2321] = 1.0;
            scratch.node_derivatives[2321] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2321] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2372] = 1.0;
            scratch.node_derivatives[2372] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2372] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2335] = 0.0;
            scratch.node_derivatives[2335] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2335] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2324] = 0.0;
            scratch.node_derivatives[2324] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2324] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2352] = 0.0;
            scratch.node_derivatives[2352] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2352] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(865, &AdValue::add(scratch.ad_value(853), scratch.ad_value(2292)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(866, &AdValue::sub(scratch.ad_value(865), scratch.ad_value(744)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2293, &AdValue::add(scratch.ad_value(2292), AdValue::scale(AdValue::sub(scratch.ad_value(854), scratch.ad_value(864)), 0.5)));
        }

        scratch.values[2406] = if (scratch.values[214] < 1e-10) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2406] != 0.0)) {
            scratch.values[867] = scratch.values[864];
            scratch.node_derivatives[867] = scratch.node_derivatives[864];
            scratch.branch_derivatives[867] = scratch.branch_derivatives[864];
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2406] != 0.0))) {
            scratch.store_ad(867, &AdValue::div(AdValue::scale(AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(214), scratch.ad_value(864)), 1.0)), (-1.0)), 2.0), scratch.ad_value(214)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(868, &AdValue::mul(AdValue::mul(scratch.ad_value(213), scratch.ad_value(867)), AdValue::offset(AdValue::mul(scratch.ad_value(215), scratch.ad_value(2293)), 1.0)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2003, &AdValue::mul(AdValue::mul(scratch.ad_value(216), AdValue::offset(AdValue::mul(scratch.ad_value(218), scratch.ad_value(864)), 1.0)), AdValue::offset(AdValue::mul(scratch.ad_value(217), scratch.ad_value(2293)), 1.0)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(866, &AdValue::add(scratch.ad_value(866), scratch.ad_value(868)));
        }

        scratch.values[2407] = if (scratch.values[202] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2407] != 0.0)) {
            scratch.store_ad(2295, &AdValue::mul(AdValue::scale(scratch.ad_value(202), 0.5), AdValue::add(AdValue::sub(AdValue::add(scratch.ad_value(853), scratch.ad_value(855)), scratch.ad_value(200)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(853), scratch.ad_value(855)), scratch.ad_value(200))), AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(853), scratch.ad_value(855)), scratch.ad_value(200)))), scratch.ad_value(201))))));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2407] != 0.0)) {
            scratch.store_ad(2296, &AdValue::mul(scratch.ad_value(2291), AdValue::sqrt(AdValue::offset(scratch.ad_value(2295), 1.0))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2407] != 0.0))) {
            scratch.values[2296] = scratch.values[2291];
            scratch.node_derivatives[2296] = scratch.node_derivatives[2291];
            scratch.branch_derivatives[2296] = scratch.branch_derivatives[2291];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2297, &AdValue::square(scratch.ad_value(2296)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2298, &AdValue::div_from_scalar(1.0, scratch.ad_value(2297)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2009] = 1.0;
            scratch.node_derivatives[2009] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2009] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2408] = if (scratch.values[207] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2004, &AdValue::scale(scratch.ad_value(866), (2.0 * scratch.values[363])));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2077, &AdValue::add(scratch.ad_value(2297), scratch.ad_value(2004)));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2078, &AdValue::scale(AdValue::add(AdValue::add(scratch.ad_value(2077), scratch.ad_value(2004)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::add(scratch.ad_value(2077), scratch.ad_value(2004)), AdValue::add(scratch.ad_value(2077), scratch.ad_value(2004))), 5.0))), 0.5));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2005, &AdValue::scale(AdValue::sub(scratch.ad_value(2077), AdValue::mul(scratch.ad_value(2296), AdValue::sqrt(scratch.ad_value(2078)))), 0.5));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2006, &AdValue::scale(scratch.ad_value(2290), scratch.values[363]));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2007, &AdValue::scale(scratch.ad_value(2293), scratch.values[363]));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2077, &AdValue::offset(AdValue::add(scratch.ad_value(2006), scratch.ad_value(2007)), 2.0));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2008, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2005), scratch.ad_value(2077)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2005), scratch.ad_value(2077)), AdValue::sub(scratch.ad_value(2005), scratch.ad_value(2077))), 5.0))), 0.5));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) {
            scratch.store_ad(2078, &AdValue::mul(scratch.ad_value(746), AdValue::sub(scratch.ad_value(2008), AdValue::mul(AdValue::offset(scratch.ad_value(208), 1.0), AdValue::add(AdValue::scale(scratch.ad_value(2006), 0.5), scratch.ad_value(2007))))));
        }

        scratch.values[2409] = if (scratch.values[2078] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) && (scratch.values[2409] != 0.0)) {
            scratch.store_ad(2009, &AdValue::exp(scratch.ad_value(2078)));
        }

        if (((scratch.values[2404] != 0.0) && (scratch.values[2408] != 0.0)) && (!(scratch.values[2409] != 0.0))) {
            scratch.store_ad(2009, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2078)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2078)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2078)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2010, &AdValue::offset(AdValue::mul(scratch.ad_value(745), scratch.ad_value(2009)), 1.0));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2011, &AdValue::mul(AdValue::scale(scratch.ad_value(2010), scratch.values[759]), AdValue::offset(scratch.ad_value(2003), 1.0)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2012, &AdValue::div_from_scalar(1.0, scratch.ad_value(2011)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2294, &AdValue::mul(scratch.ad_value(866), scratch.ad_value(2012)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2299, &AdValue::offset(AdValue::scale(scratch.ad_value(2296), 0.7071067811865475), 1.0));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2300, &AdValue::div_from_scalar(1.0, scratch.ad_value(2299)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2301, &AdValue::mul(scratch.ad_value(2292), scratch.ad_value(2012)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2302, &AdValue::add(AdValue::mul(scratch.ad_value(2290), scratch.ad_value(2012)), scratch.ad_value(2301)));
        }

        scratch.values[2410] = if (scratch.values[2302] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2410] != 0.0)) {
            scratch.store_ad(2303, &AdValue::exp(AdValue::neg(scratch.ad_value(2302))));
        }

        if ((scratch.values[2404] != 0.0) && (!(scratch.values[2410] != 0.0))) {
            scratch.store_ad(2303, &AdValue::div_from_scalar(1e-200, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2302), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2302), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2302), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(2304, &AdValue::scale(scratch.ad_value(2299), 1e-5));
        }

        scratch.values[2411] = if (((scratch.values[2294]) as f64).abs() <= scratch.values[2304]) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2411] != 0.0)) {
            scratch.store_ad(2380, &AdValue::scale(AdValue::square(scratch.ad_value(2300)), (0.16666666666666666 * 0.7071067811865475)));
        }

        if ((scratch.values[2404] != 0.0) && (scratch.values[2411] != 0.0)) {
            scratch.store_ad(2305, &AdValue::mul(AdValue::mul(scratch.ad_value(2294), scratch.ad_value(2300)), AdValue::offset(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2294), AdValue::sub_from_scalar(1.0, scratch.ad_value(2303))), scratch.ad_value(2296)), scratch.ad_value(2380)), 1.0)));
        }

        scratch.values[2412] = if (scratch.values[2294] < (-scratch.values[2304])) { 1.0 } else { 0.0 };

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2382, &AdValue::neg(scratch.ad_value(2294)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2383, &AdValue::scale(AdValue::mul(scratch.ad_value(2382), scratch.ad_value(2300)), 1.25));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2384, &AdValue::scale(AdValue::sub(AdValue::offset(scratch.ad_value(2383), 10.0), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2383), (-6.0)), AdValue::offset(scratch.ad_value(2383), (-6.0))), 64.0))), 0.5));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2379, &AdValue::sub(scratch.ad_value(2382), scratch.ad_value(2384)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2385, &AdValue::add(AdValue::square(scratch.ad_value(2379)), AdValue::mul(scratch.ad_value(2297), AdValue::offset(scratch.ad_value(2384), 1.0))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2386, &AdValue::sub(AdValue::scale(scratch.ad_value(2379), 2.0), scratch.ad_value(2297)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2387, &AdValue::sub(AdValue::ln(AdValue::mul(scratch.ad_value(2385), scratch.ad_value(2298))), scratch.ad_value(2384)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(966, &AdValue::add(scratch.ad_value(2385), scratch.ad_value(2386)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(965, &AdValue::add(AdValue::square(scratch.ad_value(966)), AdValue::mul(scratch.ad_value(2387), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2386)), 0.5), scratch.ad_value(2385)))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2388, &AdValue::add(scratch.ad_value(2384), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2385), scratch.ad_value(966)), scratch.ad_value(2387)), AdValue::add(scratch.ad_value(965), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(966), scratch.ad_value(965)), scratch.ad_value(2387)), scratch.ad_value(2387)), scratch.ad_value(2386)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2386)), 0.3333333333333333), scratch.ad_value(2385)))))));
        }

        scratch.values[2413] = if (scratch.values[2388] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) && (scratch.values[2413] != 0.0)) {
            scratch.store_ad(2389, &AdValue::exp(scratch.ad_value(2388)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) && (!(scratch.values[2413] != 0.0))) {
            scratch.store_ad(2389, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2388), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2388), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2388), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2390, &AdValue::div_from_scalar(1.0, scratch.ad_value(2389)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2379, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2388)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2391, &AdValue::mul(AdValue::square(scratch.ad_value(2388)), scratch.ad_value(2379)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2392, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2388), scratch.ad_value(2379)), scratch.ad_value(2379)), 4.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2393, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2379), 8.0), AdValue::scale(scratch.ad_value(2391), 12.0)), scratch.ad_value(2379)), scratch.ad_value(2379)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2379, &AdValue::sub(scratch.ad_value(2382), scratch.ad_value(2388)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2380, &AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2390)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2394, &AdValue::add(AdValue::scale(scratch.ad_value(2379), 2.0), AdValue::mul(scratch.ad_value(2297), AdValue::add(AdValue::sub(AdValue::offset(scratch.ad_value(2389), (-1.0)), scratch.ad_value(2380)), AdValue::mul(scratch.ad_value(2303), AdValue::sub_from_scalar(1.0, scratch.ad_value(2392)))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2395, &AdValue::sub(AdValue::square(scratch.ad_value(2379)), AdValue::mul(scratch.ad_value(2297), AdValue::add(AdValue::add(AdValue::offset(AdValue::sub(scratch.ad_value(2389), scratch.ad_value(2388)), (-1.0)), scratch.ad_value(2380)), AdValue::mul(scratch.ad_value(2303), AdValue::sub(AdValue::offset(scratch.ad_value(2388), (-1.0)), scratch.ad_value(2391)))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2379, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(scratch.ad_value(2389), scratch.ad_value(2380)), AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2393))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2379, &AdValue::sub(AdValue::square(scratch.ad_value(2394)), AdValue::scale(AdValue::mul(scratch.ad_value(2395), scratch.ad_value(2379)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (scratch.values[2412] != 0.0)) {
            scratch.store_ad(2305, &AdValue::sub(AdValue::neg(scratch.ad_value(2388)), AdValue::scale(AdValue::div(scratch.ad_value(2395), AdValue::add(scratch.ad_value(2394), AdValue::sqrt(scratch.ad_value(2379)))), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2396, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(2296), 0.7324648775608221), 1.25)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2397, &AdValue::mul(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(2299), 1.25), scratch.ad_value(2396)), (-1.0)), scratch.ad_value(2396)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2398, &AdValue::mul(AdValue::mul(scratch.ad_value(2294), scratch.ad_value(2300)), AdValue::offset(AdValue::mul(scratch.ad_value(2397), scratch.ad_value(2294)), 1.0)));
        }

        scratch.values[2414] = if ((-scratch.values[2398]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (scratch.values[2414] != 0.0)) {
            scratch.store_ad(2379, &AdValue::exp(AdValue::neg(scratch.ad_value(2398))));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (!(scratch.values[2414] != 0.0))) {
            scratch.store_ad(2379, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2398))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2398))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2398))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2399, &AdValue::sub_from_scalar(1.0, scratch.ad_value(2379)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2400, &AdValue::sub(AdValue::add(scratch.ad_value(2294), AdValue::scale(scratch.ad_value(2297), 0.5)), AdValue::mul(scratch.ad_value(2296), AdValue::sqrt(AdValue::sub(AdValue::add(scratch.ad_value(2294), AdValue::scale(scratch.ad_value(2297), 0.25)), scratch.ad_value(2399))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2401, &AdValue::offset(scratch.ad_value(2302), 3.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2384, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2400), scratch.ad_value(2401)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2400), scratch.ad_value(2401)), AdValue::sub(scratch.ad_value(2400), scratch.ad_value(2401))), 5.0))), 0.5), AdValue::scale(AdValue::sub(scratch.ad_value(2401), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2401)), 5.0))), 0.5)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2379, &AdValue::sub(scratch.ad_value(2294), scratch.ad_value(2384)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2380, &AdValue::exp(AdValue::neg(scratch.ad_value(2384))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2381, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2384)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2391, &AdValue::mul(AdValue::square(scratch.ad_value(2384)), scratch.ad_value(2381)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2392, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2384), scratch.ad_value(2381)), scratch.ad_value(2381)), 4.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2393, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2381), 8.0), AdValue::scale(scratch.ad_value(2391), 12.0)), scratch.ad_value(2381)), scratch.ad_value(2381)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2385, &AdValue::max_from_scalar(1e-40, AdValue::sub(AdValue::square(scratch.ad_value(2379)), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::offset(AdValue::add(scratch.ad_value(2380), scratch.ad_value(2384)), (-1.0)), AdValue::mul(scratch.ad_value(2303), AdValue::add(AdValue::offset(scratch.ad_value(2384), 1.0), scratch.ad_value(2391))))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2402, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2297), AdValue::sub(scratch.ad_value(2380), AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2393)))), 0.5)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2386, &AdValue::add(AdValue::scale(scratch.ad_value(2379), 2.0), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(2380)), AdValue::mul(scratch.ad_value(2303), AdValue::offset(scratch.ad_value(2392), 1.0))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2387, &AdValue::add(AdValue::sub(scratch.ad_value(2302), scratch.ad_value(2384)), AdValue::ln(AdValue::div(scratch.ad_value(2385), scratch.ad_value(2297)))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(966, &AdValue::add(scratch.ad_value(2385), scratch.ad_value(2386)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(965, &AdValue::add(AdValue::square(scratch.ad_value(966)), AdValue::mul(scratch.ad_value(2387), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2386)), 0.5), AdValue::mul(scratch.ad_value(2385), scratch.ad_value(2402))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            let assign48050_ad_e61814: AdValue = AdValue::add(scratch.ad_value(2384), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2385), scratch.ad_value(966)), scratch.ad_value(2387)), AdValue::add(scratch.ad_value(965), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(966), scratch.ad_value(965)), scratch.ad_value(2387)), scratch.ad_value(2387)), scratch.ad_value(2386)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2386)), 0.3333333333333333), AdValue::mul(scratch.ad_value(2385), scratch.ad_value(2402)))))));
            scratch.store_ad(2403, &assign48050_ad_e61814);
        }

        scratch.values[2415] = if (scratch.values[2403] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (scratch.values[2415] != 0.0)) {
            scratch.store_ad(2389, &AdValue::exp(scratch.ad_value(2403)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (scratch.values[2415] != 0.0)) {
            scratch.store_ad(2390, &AdValue::div_from_scalar(1.0, scratch.ad_value(2389)));
        }

        if ((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (scratch.values[2415] != 0.0)) {
            scratch.store_ad(2389, &AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2389)));
        }

        scratch.values[2416] = if (scratch.values[2403] > (scratch.values[2302] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (!(scratch.values[2415] != 0.0))) && (scratch.values[2416] != 0.0)) {
            scratch.store_ad(2389, &AdValue::exp(AdValue::sub(scratch.ad_value(2403), scratch.ad_value(2302))));
        }

        if (((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (!(scratch.values[2415] != 0.0))) && (scratch.values[2416] != 0.0)) {
            scratch.store_ad(2390, &AdValue::div(scratch.ad_value(2303), scratch.ad_value(2389)));
        }

        if (((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (!(scratch.values[2415] != 0.0))) && (!(scratch.values[2416] != 0.0))) {
            scratch.store_ad(2389, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2302), scratch.ad_value(2403)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2302), scratch.ad_value(2403)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2302), scratch.ad_value(2403)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) && (!(scratch.values[2415] != 0.0))) && (!(scratch.values[2416] != 0.0))) {
            scratch.store_ad(2390, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2403), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2403), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2403), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2379, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2403)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2391, &AdValue::mul(AdValue::square(scratch.ad_value(2403)), scratch.ad_value(2379)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2392, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2403), scratch.ad_value(2379)), scratch.ad_value(2379)), 4.0));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2393, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2379), 8.0), AdValue::scale(scratch.ad_value(2391), 12.0)), scratch.ad_value(2379)), scratch.ad_value(2379)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2379, &AdValue::sub(scratch.ad_value(2294), scratch.ad_value(2403)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2394, &AdValue::add(AdValue::scale(scratch.ad_value(2379), 2.0), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2390)), scratch.ad_value(2389)), AdValue::mul(scratch.ad_value(2303), AdValue::offset(scratch.ad_value(2392), 1.0))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2395, &AdValue::sub(AdValue::square(scratch.ad_value(2379)), AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(AdValue::offset(AdValue::add(scratch.ad_value(2390), scratch.ad_value(2403)), (-1.0)), scratch.ad_value(2389)), AdValue::mul(scratch.ad_value(2303), AdValue::add(AdValue::offset(scratch.ad_value(2403), 1.0), scratch.ad_value(2391)))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2379, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2297), AdValue::sub(AdValue::add(scratch.ad_value(2390), scratch.ad_value(2389)), AdValue::mul(scratch.ad_value(2303), scratch.ad_value(2393))))));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2379, &AdValue::sub(AdValue::square(scratch.ad_value(2394)), AdValue::scale(AdValue::mul(scratch.ad_value(2395), scratch.ad_value(2379)), 2.0)));
        }

        if (((scratch.values[2404] != 0.0) && (!(scratch.values[2411] != 0.0))) && (!(scratch.values[2412] != 0.0))) {
            scratch.store_ad(2305, &AdValue::add(scratch.ad_value(2403), AdValue::scale(AdValue::div(scratch.ad_value(2395), AdValue::add(scratch.ad_value(2394), AdValue::sqrt(scratch.ad_value(2379)))), 2.0)));
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2339] = scratch.values[2305];
            scratch.node_derivatives[2339] = scratch.node_derivatives[2305];
            scratch.branch_derivatives[2339] = scratch.branch_derivatives[2305];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2347] = scratch.values[2305];
            scratch.node_derivatives[2347] = scratch.node_derivatives[2305];
            scratch.branch_derivatives[2347] = scratch.branch_derivatives[2305];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.values[2340] = 0.0;
            scratch.node_derivatives[2340] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2340] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2404] != 0.0) {
            scratch.store_ad(869, &AdValue::scale(scratch.ad_value(2011), 3.912023005));
        }

        scratch.values[2417] = if (scratch.values[2294] <= 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2404] != 0.0) && (scratch.values[2417] != 0.0)) {
            scratch.values[2315] = 0.0;
            scratch.node_derivatives[2315] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2315] = [0.0; Instance::BRANCH_COUNT];
        }

    }
}
