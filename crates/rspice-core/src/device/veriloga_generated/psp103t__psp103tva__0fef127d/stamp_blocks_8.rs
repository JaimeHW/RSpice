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
        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2137, &AdValue::add(AdValue::div(scratch.ad_value(2127), scratch.ad_value(2167)), scratch.ad_value(2008)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2138, &AdValue::scale(AdValue::mul(scratch.ad_value(2136), scratch.ad_value(2137)), 0.7071067811865475));
        }

        scratch.values[2243] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) && (scratch.values[2243] != 0.0)) {
            scratch.store_ad(2138, &AdValue::div(scratch.ad_value(2138), AdValue::sqrt(AdValue::offset(scratch.ad_value(2138), 1.0))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2139, &AdValue::div_from_scalar(2.0, AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(scratch.ad_value(2138), 4.0), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2087, &AdValue::mul(scratch.ad_value(2139), scratch.ad_value(2138)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2140, &AdValue::mul(AdValue::mul(scratch.ad_value(2137), scratch.ad_value(2139)), AdValue::offset(AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2087), AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(2087), scratch.ad_value(2139)))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2087)), scratch.ad_value(2139)), 4.0), 1.0)), 0.86), 1.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2141, &AdValue::add(scratch.ad_value(2126), AdValue::scale(scratch.ad_value(2109), 0.5)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2142, &AdValue::scale(AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2109), scratch.ad_value(2123)), scratch.ad_value(2008)), AdValue::add(scratch.ad_value(2141), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(2141)), AdValue::scale(AdValue::mul(scratch.ad_value(2109), scratch.ad_value(2123)), 0.98))))), 0.98));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2143, &AdValue::add(scratch.ad_value(2140), scratch.ad_value(2142)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2144, &AdValue::scale(AdValue::mul(scratch.ad_value(2140), scratch.ad_value(2142)), 2.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(2145, &AdValue::div(scratch.ad_value(2144), AdValue::add(scratch.ad_value(2143), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(2143)), AdValue::scale(scratch.ad_value(2144), 1.98))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2238] != 0.0)) {
            scratch.store_ad(866, &AdValue::sub(scratch.ad_value(2145), AdValue::mul(scratch.ad_value(2008), AdValue::ln(AdValue::offset(AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2145), AdValue::sub(scratch.ad_value(2145), AdValue::mul(AdValue::scale(scratch.ad_value(2141), 2.0), scratch.ad_value(2008)))), scratch.ad_value(2110)), AdValue::mul(AdValue::square(scratch.ad_value(2008)), scratch.ad_value(2123))), 1.0)))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2238] != 0.0))) {
            scratch.values[866] = scratch.values[865];
            scratch.node_derivatives[866] = scratch.node_derivatives[865];
            scratch.branch_derivatives[866] = scratch.branch_derivatives[865];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2086, &AdValue::pow(AdValue::div(scratch.ad_value(850), scratch.ad_value(866)), scratch.ad_value(243)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2146, &AdValue::mul(scratch.ad_value(850), AdValue::pow(AdValue::offset(scratch.ad_value(2086), 1.0), AdValue::neg(scratch.ad_value(816)))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2147, &AdValue::mul(scratch.ad_value(2146), scratch.ad_value(2009)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2148, &AdValue::add(scratch.ad_value(2114), scratch.ad_value(2147)));
        }

        scratch.values[2244] = if (scratch.values[2147] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2244] != 0.0)) {
            scratch.store_ad(2149, &AdValue::exp(AdValue::neg(scratch.ad_value(2147))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2244] != 0.0))) {
            scratch.store_ad(2149, &AdValue::div_from_scalar(1e-200, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2147), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2147), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2147), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2150, &AdValue::mul(scratch.ad_value(2115), scratch.ad_value(2149)));
        }

        scratch.values[2245] = if (((scratch.values[2106]) as f64).abs() <= scratch.values[2116]) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2245] != 0.0)) {
            scratch.store_ad(2192, &AdValue::scale(AdValue::square(scratch.ad_value(2112)), (0.16666666666666666 * 0.7071067811865475)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2245] != 0.0)) {
            scratch.store_ad(2151, &AdValue::mul(AdValue::mul(scratch.ad_value(2106), scratch.ad_value(2112)), AdValue::offset(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2106), AdValue::sub_from_scalar(1.0, scratch.ad_value(2150))), scratch.ad_value(2108)), scratch.ad_value(2192)), 1.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2213, &AdValue::offset(scratch.ad_value(2148), 3.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2196, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2212), scratch.ad_value(2213)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2212), scratch.ad_value(2213)), AdValue::sub(scratch.ad_value(2212), scratch.ad_value(2213))), 5.0))), 0.5), AdValue::scale(AdValue::sub(scratch.ad_value(2213), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2213)), 5.0))), 0.5)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub(scratch.ad_value(2106), scratch.ad_value(2196)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2192, &AdValue::exp(AdValue::neg(scratch.ad_value(2196))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2193, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2196)), 2.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2203, &AdValue::mul(AdValue::square(scratch.ad_value(2196)), scratch.ad_value(2193)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2204, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2196), scratch.ad_value(2193)), scratch.ad_value(2193)), 4.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2205, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2193), 8.0), AdValue::scale(scratch.ad_value(2203), 12.0)), scratch.ad_value(2193)), scratch.ad_value(2193)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2197, &AdValue::max_from_scalar(1e-40, AdValue::sub(AdValue::square(scratch.ad_value(2191)), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::offset(AdValue::add(scratch.ad_value(2192), scratch.ad_value(2196)), (-1.0)), AdValue::mul(scratch.ad_value(2150), AdValue::add(AdValue::offset(scratch.ad_value(2196), 1.0), scratch.ad_value(2203))))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2214, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2109), AdValue::sub(scratch.ad_value(2192), AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2205)))), 0.5)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2198, &AdValue::add(AdValue::scale(scratch.ad_value(2191), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(2192)), AdValue::mul(scratch.ad_value(2150), AdValue::offset(scratch.ad_value(2204), 1.0))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2199, &AdValue::add(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2196)), AdValue::ln(AdValue::div(scratch.ad_value(2197), scratch.ad_value(2109)))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(962, &AdValue::add(scratch.ad_value(2197), scratch.ad_value(2198)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(961, &AdValue::add(AdValue::square(scratch.ad_value(962)), AdValue::mul(scratch.ad_value(2199), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2198)), 0.5), AdValue::mul(scratch.ad_value(2197), scratch.ad_value(2214))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            let assign43230_ad_e55896: AdValue = AdValue::add(scratch.ad_value(2196), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2197), scratch.ad_value(962)), scratch.ad_value(2199)), AdValue::add(scratch.ad_value(961), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(962), scratch.ad_value(961)), scratch.ad_value(2199)), scratch.ad_value(2199)), scratch.ad_value(2198)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2198)), 0.3333333333333333), AdValue::mul(scratch.ad_value(2197), scratch.ad_value(2214)))))));
            scratch.store_ad(2215, &assign43230_ad_e55896);
        }

        scratch.values[2246] = if (scratch.values[2215] < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (scratch.values[2246] != 0.0)) {
            scratch.store_ad(2201, &AdValue::exp(scratch.ad_value(2215)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (scratch.values[2246] != 0.0)) {
            scratch.store_ad(2202, &AdValue::div_from_scalar(1.0, scratch.ad_value(2201)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (scratch.values[2246] != 0.0)) {
            scratch.store_ad(2201, &AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2201)));
        }

        scratch.values[2247] = if (scratch.values[2215] > (scratch.values[2148] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (!(scratch.values[2246] != 0.0))) && (scratch.values[2247] != 0.0)) {
            scratch.store_ad(2201, &AdValue::exp(AdValue::sub(scratch.ad_value(2215), scratch.ad_value(2148))));
        }

        if ((((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (!(scratch.values[2246] != 0.0))) && (scratch.values[2247] != 0.0)) {
            scratch.store_ad(2202, &AdValue::div(scratch.ad_value(2150), scratch.ad_value(2201)));
        }

        if ((((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (!(scratch.values[2246] != 0.0))) && (!(scratch.values[2247] != 0.0))) {
            scratch.store_ad(2201, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2215)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2215)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) && (!(scratch.values[2246] != 0.0))) && (!(scratch.values[2247] != 0.0))) {
            scratch.store_ad(2202, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2215), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2215), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2215), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2191, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2215)), 2.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2203, &AdValue::mul(AdValue::square(scratch.ad_value(2215)), scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2204, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2215), scratch.ad_value(2191)), scratch.ad_value(2191)), 4.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2205, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2191), 8.0), AdValue::scale(scratch.ad_value(2203), 12.0)), scratch.ad_value(2191)), scratch.ad_value(2191)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub(scratch.ad_value(2106), scratch.ad_value(2215)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2206, &AdValue::add(AdValue::scale(scratch.ad_value(2191), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2202)), scratch.ad_value(2201)), AdValue::mul(scratch.ad_value(2150), AdValue::offset(scratch.ad_value(2204), 1.0))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2207, &AdValue::sub(AdValue::square(scratch.ad_value(2191)), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(AdValue::offset(AdValue::add(scratch.ad_value(2202), scratch.ad_value(2215)), (-1.0)), scratch.ad_value(2201)), AdValue::mul(scratch.ad_value(2150), AdValue::add(AdValue::offset(scratch.ad_value(2215), 1.0), scratch.ad_value(2203)))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(scratch.ad_value(2202), scratch.ad_value(2201)), AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2205))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2191, &AdValue::sub(AdValue::square(scratch.ad_value(2206)), AdValue::scale(AdValue::mul(scratch.ad_value(2207), scratch.ad_value(2191)), 2.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2245] != 0.0))) {
            scratch.store_ad(2151, &AdValue::add(scratch.ad_value(2215), AdValue::scale(AdValue::div(scratch.ad_value(2207), AdValue::add(scratch.ad_value(2206), AdValue::sqrt(scratch.ad_value(2191)))), 2.0)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2152, &AdValue::sub(scratch.ad_value(2151), scratch.ad_value(2117)));
        }

        scratch.values[2248] = if (scratch.values[2152] < 1e-10) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2248] != 0.0)) {
            scratch.store_ad(2153, &AdValue::add(AdValue::scale(AdValue::sub(scratch.ad_value(2106), scratch.ad_value(2117)), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2122)), AdValue::mul(scratch.ad_value(2118), scratch.ad_value(2149))), AdValue::mul(scratch.ad_value(2150), AdValue::offset(scratch.ad_value(2120), 1.0))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2248] != 0.0)) {
            scratch.store_ad(2154, &AdValue::mul(AdValue::mul(scratch.ad_value(2109), AdValue::sub_from_scalar(1.0, scratch.ad_value(2149))), scratch.ad_value(2123)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2248] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2109), AdValue::sub(AdValue::add(scratch.ad_value(2122), AdValue::mul(scratch.ad_value(2118), scratch.ad_value(2149))), AdValue::mul(scratch.ad_value(2150), scratch.ad_value(2121))))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2248] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sub(AdValue::square(scratch.ad_value(2153)), AdValue::scale(AdValue::mul(scratch.ad_value(2086), scratch.ad_value(2154)), 2.0)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2248] != 0.0)) {
            scratch.store_ad(2152, &AdValue::scale(AdValue::div(scratch.ad_value(2154), AdValue::add(scratch.ad_value(2153), AdValue::sqrt(scratch.ad_value(2086)))), 2.0));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2248] != 0.0)) {
            scratch.store_ad(2151, &AdValue::add(scratch.ad_value(2117), scratch.ad_value(2152)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2155, &AdValue::mul(scratch.ad_value(2152), scratch.ad_value(2008)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2156, &AdValue::div(AdValue::square(scratch.ad_value(2151)), AdValue::offset(AdValue::square(scratch.ad_value(2151)), 2.0)));
        }

        scratch.values[2249] = if (scratch.values[2151] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2249] != 0.0)) {
            scratch.store_ad(2157, &AdValue::exp(AdValue::neg(scratch.ad_value(2151))));
        }

        scratch.values[2250] = if (scratch.values[2151] < 1e-5) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2249] != 0.0)) && (scratch.values[2250] != 0.0)) {
            scratch.store_ad(2158, &AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::scale(scratch.ad_value(2150), 0.16666666666666666), scratch.ad_value(2151)), scratch.ad_value(2151)), scratch.ad_value(2151)), AdValue::offset(AdValue::scale(scratch.ad_value(2151), 1.75), 1.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2249] != 0.0)) && (!(scratch.values[2250] != 0.0))) {
            scratch.store_ad(2158, &AdValue::mul(scratch.ad_value(2150), AdValue::sub(AdValue::offset(AdValue::sub(AdValue::div_from_scalar(1.0, scratch.ad_value(2157)), scratch.ad_value(2151)), (-1.0)), scratch.ad_value(2156))));
        }

        scratch.values[2251] = if (scratch.values[2151] > (scratch.values[2148] - 230.25850929940458)) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2249] != 0.0))) && (scratch.values[2251] != 0.0)) {
            scratch.store_ad(2086, &AdValue::exp(AdValue::sub(scratch.ad_value(2151), scratch.ad_value(2148))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2249] != 0.0))) && (scratch.values[2251] != 0.0)) {
            scratch.store_ad(2157, &AdValue::div(scratch.ad_value(2150), scratch.ad_value(2086)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2249] != 0.0))) && (scratch.values[2251] != 0.0)) {
            scratch.store_ad(2158, &AdValue::sub(scratch.ad_value(2086), AdValue::mul(scratch.ad_value(2150), AdValue::add(AdValue::offset(scratch.ad_value(2151), 1.0), scratch.ad_value(2156)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2249] != 0.0))) && (!(scratch.values[2251] != 0.0))) {
            scratch.store_ad(2157, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2151), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2151), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2151), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2249] != 0.0))) && (!(scratch.values[2251] != 0.0))) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2151)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2151)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(2148), scratch.ad_value(2151)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2249] != 0.0))) && (!(scratch.values[2251] != 0.0))) {
            scratch.store_ad(2158, &AdValue::sub(scratch.ad_value(2086), AdValue::mul(scratch.ad_value(2150), AdValue::add(AdValue::offset(scratch.ad_value(2151), 1.0), scratch.ad_value(2156)))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2159, &AdValue::scale(AdValue::add(scratch.ad_value(2117), scratch.ad_value(2151)), 0.5));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.values[2160] = 0.0;
            scratch.node_derivatives[2160] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2160] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2086, &AdValue::mul(scratch.ad_value(2157), scratch.ad_value(2122)));
        }

        scratch.values[2252] = if (scratch.values[2086] > 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2252] != 0.0)) {
            scratch.store_ad(2160, &AdValue::sqrt(scratch.ad_value(2086)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2161, &AdValue::scale(AdValue::add(scratch.ad_value(2123), scratch.ad_value(2158)), 0.5));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2162, &AdValue::add(scratch.ad_value(2161), AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2152)), AdValue::sub(scratch.ad_value(2160), AdValue::scale(scratch.ad_value(2110), 2.0))), 0.125)));
        }

        scratch.values[2253] = if (scratch.values[2159] < 1e-5) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2163, &AdValue::scale(AdValue::mul(AdValue::square(scratch.ad_value(2159)), AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2159), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2159), 0.25))), 0.3333333333333333))), 0.5));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2164, &AdValue::mul(scratch.ad_value(2108), AdValue::sqrt(AdValue::add(scratch.ad_value(2162), scratch.ad_value(2163)))));
        }

        scratch.values[2254] = if (scratch.values[769] > 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (scratch.values[2253] != 0.0)) && (scratch.values[2254] != 0.0)) {
            scratch.store_ad(2165, &AdValue::div_from_scalar(1.0, AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(769), scratch.ad_value(2164)), 1.0))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sqrt(AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2159), AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2159), 0.25))), 0.3333333333333333))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2166, &AdValue::scale(AdValue::mul(scratch.ad_value(2159), scratch.ad_value(2086)), 0.7071067811865475));
        }

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2253] != 0.0)) {
            scratch.store_ad(2167, &AdValue::add(scratch.ad_value(2165), AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2108), AdValue::add(AdValue::sub_from_scalar(1.0, AdValue::scale(scratch.ad_value(2159), 0.5)), AdValue::scale(AdValue::square(scratch.ad_value(2159)), 0.16666666666666666))), scratch.ad_value(2086)), 0.7071067811865475)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) {
            scratch.store_ad(2163, &AdValue::add(AdValue::offset(scratch.ad_value(2159), (-1.0)), scratch.ad_value(2160)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) {
            scratch.store_ad(2164, &AdValue::mul(scratch.ad_value(2108), AdValue::sqrt(AdValue::add(scratch.ad_value(2162), scratch.ad_value(2163)))));
        }

        scratch.values[2255] = if (scratch.values[769] > 0.0) { 1.0 } else { 0.0 };

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2168, &AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2160)), AdValue::scale(AdValue::mul(scratch.ad_value(2164), scratch.ad_value(2110)), 2.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2165, &AdValue::div_from_scalar(1.0, AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(769), scratch.ad_value(2164)), 1.0))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div(scratch.ad_value(2165), AdValue::offset(scratch.ad_value(2165), 1.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2169, &AdValue::mul(scratch.ad_value(769), AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2086)), scratch.ad_value(2109)), scratch.ad_value(2162))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2170, &AdValue::add(AdValue::scale(AdValue::sub(scratch.ad_value(2164), scratch.ad_value(2169)), 2.0), AdValue::mul(scratch.ad_value(2109), AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2160)), scratch.ad_value(2162)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2171, &AdValue::mul(scratch.ad_value(2169), AdValue::sub(scratch.ad_value(2169), AdValue::scale(scratch.ad_value(2164), 2.0))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2172, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2109), AdValue::add(scratch.ad_value(2160), scratch.ad_value(2162))), 0.5)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2173, &AdValue::div(AdValue::mul(scratch.ad_value(2171), scratch.ad_value(2170)), AdValue::sub(AdValue::square(scratch.ad_value(2170)), AdValue::mul(scratch.ad_value(2172), scratch.ad_value(2171)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2159, &AdValue::add(scratch.ad_value(2159), scratch.ad_value(2173)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2174, &AdValue::exp(scratch.ad_value(2173)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2160, &AdValue::div(scratch.ad_value(2160), scratch.ad_value(2174)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2162, &AdValue::mul(scratch.ad_value(2162), scratch.ad_value(2174)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2163, &AdValue::add(AdValue::offset(scratch.ad_value(2159), (-1.0)), scratch.ad_value(2160)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2164, &AdValue::mul(scratch.ad_value(2108), AdValue::sqrt(AdValue::add(scratch.ad_value(2162), scratch.ad_value(2163)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2175, &AdValue::add(AdValue::sub_from_scalar(1.0, scratch.ad_value(2160)), AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2164), scratch.ad_value(2165)), scratch.ad_value(2110)), 2.0)));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2152, &AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2152), scratch.ad_value(2174)), AdValue::add(scratch.ad_value(2168), scratch.ad_value(2161))), AdValue::add(scratch.ad_value(2175), AdValue::mul(scratch.ad_value(2174), scratch.ad_value(2161)))));
        }

        if (((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) && (scratch.values[2255] != 0.0)) {
            scratch.store_ad(2155, &AdValue::mul(scratch.ad_value(2152), scratch.ad_value(2008)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) {
            scratch.store_ad(2166, &AdValue::sqrt(scratch.ad_value(2163)));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2253] != 0.0))) {
            scratch.store_ad(2167, &AdValue::add(scratch.ad_value(2165), AdValue::scale(AdValue::div(AdValue::mul(scratch.ad_value(2108), AdValue::sub_from_scalar(1.0, scratch.ad_value(2160))), scratch.ad_value(2166)), 0.5)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2176, &AdValue::mul(scratch.ad_value(2008), AdValue::div(AdValue::mul(scratch.ad_value(2109), scratch.ad_value(2162)), AdValue::add(scratch.ad_value(2164), AdValue::mul(scratch.ad_value(2108), scratch.ad_value(2166))))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2177, &AdValue::add(scratch.ad_value(2176), AdValue::mul(scratch.ad_value(2008), scratch.ad_value(2167))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2178, &AdValue::mul(AdValue::mul(scratch.ad_value(2166), scratch.ad_value(2108)), scratch.ad_value(2008)));
        }

        scratch.values[2256] = if (scratch.values[238] < 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2256] != 0.0)) {
            scratch.store_ad(2086, &AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(238), scratch.ad_value(2176))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2256] != 0.0))) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::mul(scratch.ad_value(238), scratch.ad_value(2176)), 1.0)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2130, &AdValue::mul(scratch.ad_value(796), AdValue::mul(AdValue::mul(scratch.ad_value(2129), scratch.ad_value(2086)), scratch.ad_value(2176))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2179, &AdValue::add(scratch.ad_value(2178), AdValue::mul(scratch.ad_value(814), scratch.ad_value(2176))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2180, &AdValue::add(scratch.ad_value(2178), AdValue::mul(scratch.ad_value(815), scratch.ad_value(2176))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2131, &AdValue::mul(scratch.ad_value(813), scratch.ad_value(2179)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2087, &AdValue::ln(AdValue::div(scratch.ad_value(2163), AdValue::offset(AdValue::add(scratch.ad_value(2163), scratch.ad_value(2162)), 1e-14))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2132, &AdValue::add(AdValue::pow(AdValue::mul(scratch.ad_value(2131), scratch.ad_value(755)), scratch.ad_value(756)), AdValue::mul(scratch.ad_value(757), AdValue::exp(AdValue::mul(AdValue::scale(scratch.ad_value(758), 0.5), scratch.ad_value(2087))))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2133, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2132), 1.0), scratch.ad_value(2130)), scratch.ad_value(2125)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2181, &AdValue::ln(AdValue::div(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(850), scratch.ad_value(2155)), scratch.ad_value(817)), 1.0), AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2146), scratch.ad_value(2155)), scratch.ad_value(817)), 1.0))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2182, &AdValue::mul(scratch.ad_value(244), scratch.ad_value(2181)));
        }

    }

    pub(super) fn stamp_transient_block_33(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2183, &AdValue::div_from_scalar(1.0, AdValue::add(AdValue::offset(scratch.ad_value(2182), 1.0), AdValue::square(scratch.ad_value(2182)))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2088, &AdValue::mul(scratch.ad_value(2176), scratch.ad_value(2134)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2135, &AdValue::scale(AdValue::div(scratch.ad_value(2088), AdValue::offset(scratch.ad_value(2088), 100.0)), 100.0));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2184, &AdValue::mul(scratch.ad_value(2133), scratch.ad_value(2183)));
        }

        scratch.values[2257] = if (scratch.values[242] < 0.0) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2257] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1.0, AdValue::sub_from_scalar(1.0, AdValue::mul(scratch.ad_value(242), scratch.ad_value(2135)))));
        }

        if ((!(scratch.values[2234] != 0.0)) && (!(scratch.values[2257] != 0.0))) {
            scratch.store_ad(2086, &AdValue::offset(AdValue::mul(scratch.ad_value(242), scratch.ad_value(2135)), 1.0));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2136, &AdValue::mul(scratch.ad_value(2079), AdValue::div(scratch.ad_value(2086), scratch.ad_value(2184))));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2185, &AdValue::mul(AdValue::mul(AdValue::square(scratch.ad_value(2136)), scratch.ad_value(2155)), scratch.ad_value(2155)));
        }

        scratch.values[2258] = if (scratch.values[0] == (-1.0)) { 1.0 } else { 0.0 };

        if ((!(scratch.values[2234] != 0.0)) && (scratch.values[2258] != 0.0)) {
            scratch.store_ad(2185, &AdValue::div(scratch.ad_value(2185), AdValue::offset(AdValue::mul(scratch.ad_value(2136), scratch.ad_value(2155)), 1.0)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2186, &AdValue::scale(AdValue::mul(scratch.ad_value(2184), AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::scale(scratch.ad_value(2185), 2.0), 1.0)), 1.0)), 0.5));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2187, &AdValue::div_from_scalar(1.0, scratch.ad_value(2186)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2188, &AdValue::mul(scratch.ad_value(2164), scratch.ad_value(2008)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2086, &AdValue::mul(scratch.ad_value(2184), scratch.ad_value(2187)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2189, &AdValue::mul(scratch.ad_value(2167), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2185), scratch.ad_value(2086)), scratch.ad_value(2086)), 0.5), 1.0)));
        }

        if (!(scratch.values[2234] != 0.0)) {
            scratch.store_ad(2190, &AdValue::div(AdValue::mul(scratch.ad_value(2086), scratch.ad_value(2177)), scratch.ad_value(2189)));
        }

        scratch.values[2259] = if (scratch.values[2106] > 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2259] != 0.0) {
            scratch.store_ad(2217, &AdValue::div_from_scalar(1.0, scratch.ad_value(2177)));
        }

        if (scratch.values[2259] != 0.0) {
            scratch.store_ad(2218, &AdValue::mul(scratch.ad_value(2176), scratch.ad_value(2217)));
        }

        if (scratch.values[2259] != 0.0) {
            scratch.store_ad(2219, &AdValue::mul(scratch.ad_value(2008), AdValue::mul(scratch.ad_value(2167), scratch.ad_value(2217))));
        }

        if (scratch.values[2259] != 0.0) {
            scratch.store_ad(2220, &AdValue::ln(AdValue::offset(AdValue::mul(scratch.ad_value(860), scratch.ad_value(817)), 1.0)));
        }

        if (scratch.values[2259] != 0.0) {
            scratch.store_ad(2221, &AdValue::add(AdValue::add(scratch.ad_value(2182), AdValue::mul(scratch.ad_value(245), AdValue::mul(AdValue::mul(scratch.ad_value(2217), scratch.ad_value(2218)), scratch.ad_value(2181)))), AdValue::mul(scratch.ad_value(246), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2178), scratch.ad_value(2219)), scratch.ad_value(2219)), scratch.ad_value(2220)))));
        }

        if (scratch.values[2259] != 0.0) {
            scratch.store_ad(2216, &AdValue::mul(AdValue::add(AdValue::offset(scratch.ad_value(2221), 1.0), AdValue::square(scratch.ad_value(2221))), scratch.ad_value(2183)));
        }

        scratch.values[2010] = scratch.values[2106];
        scratch.node_derivatives[2010] = scratch.node_derivatives[2106];
        scratch.branch_derivatives[2010] = scratch.branch_derivatives[2106];

        scratch.values[875] = scratch.values[2180];
        scratch.node_derivatives[875] = scratch.node_derivatives[2180];
        scratch.branch_derivatives[875] = scratch.branch_derivatives[2180];

        scratch.values[876] = scratch.values[2188];
        scratch.node_derivatives[876] = scratch.node_derivatives[2188];
        scratch.branch_derivatives[876] = scratch.branch_derivatives[2188];

        scratch.values[2011] = scratch.values[2167];
        scratch.node_derivatives[2011] = scratch.node_derivatives[2167];
        scratch.branch_derivatives[2011] = scratch.branch_derivatives[2167];

        scratch.values[2012] = scratch.values[2155];
        scratch.node_derivatives[2012] = scratch.node_derivatives[2155];
        scratch.branch_derivatives[2012] = scratch.branch_derivatives[2155];

        scratch.values[2013] = scratch.values[2176];
        scratch.node_derivatives[2013] = scratch.node_derivatives[2176];
        scratch.branch_derivatives[2013] = scratch.branch_derivatives[2176];

        scratch.values[2014] = scratch.values[2177];
        scratch.node_derivatives[2014] = scratch.node_derivatives[2177];
        scratch.branch_derivatives[2014] = scratch.branch_derivatives[2177];

        scratch.values[877] = scratch.values[2183];
        scratch.node_derivatives[877] = scratch.node_derivatives[2183];
        scratch.branch_derivatives[877] = scratch.branch_derivatives[2183];

        scratch.values[2016] = scratch.values[2216];
        scratch.node_derivatives[2016] = scratch.node_derivatives[2216];
        scratch.branch_derivatives[2016] = scratch.branch_derivatives[2216];

        scratch.values[2015] = scratch.values[2190];
        scratch.node_derivatives[2015] = scratch.node_derivatives[2190];
        scratch.branch_derivatives[2015] = scratch.branch_derivatives[2190];

        scratch.values[878] = scratch.values[2165];
        scratch.node_derivatives[878] = scratch.node_derivatives[2165];
        scratch.branch_derivatives[878] = scratch.branch_derivatives[2165];

        scratch.values[879] = scratch.values[2186];
        scratch.node_derivatives[879] = scratch.node_derivatives[2186];
        scratch.branch_derivatives[879] = scratch.branch_derivatives[2186];

        scratch.values[2017] = scratch.values[2187];
        scratch.node_derivatives[2017] = scratch.node_derivatives[2187];
        scratch.branch_derivatives[2017] = scratch.branch_derivatives[2187];

        scratch.values[880] = scratch.values[2184];
        scratch.node_derivatives[880] = scratch.node_derivatives[2184];
        scratch.branch_derivatives[880] = scratch.branch_derivatives[2184];

        scratch.values[881] = scratch.values[2152];
        scratch.node_derivatives[881] = scratch.node_derivatives[2152];
        scratch.branch_derivatives[881] = scratch.branch_derivatives[2152];

        scratch.values[882] = scratch.values[2159];
        scratch.node_derivatives[882] = scratch.node_derivatives[2159];
        scratch.branch_derivatives[882] = scratch.branch_derivatives[2159];

        scratch.values[885] = scratch.values[2147];
        scratch.node_derivatives[885] = scratch.node_derivatives[2147];
        scratch.branch_derivatives[885] = scratch.branch_derivatives[2147];

        scratch.values[2048] = scratch.values[2133];
        scratch.node_derivatives[2048] = scratch.node_derivatives[2133];
        scratch.branch_derivatives[2048] = scratch.branch_derivatives[2133];

        scratch.values[2049] = scratch.values[2134];
        scratch.node_derivatives[2049] = scratch.node_derivatives[2134];
        scratch.branch_derivatives[2049] = scratch.branch_derivatives[2134];

        scratch.values[2050] = scratch.values[2146];
        scratch.node_derivatives[2050] = scratch.node_derivatives[2146];
        scratch.branch_derivatives[2050] = scratch.branch_derivatives[2146];

        scratch.values[2260] = if (scratch.values[2010] <= 0.0) { 1.0 } else { 0.0 };

        if (scratch.values[2260] != 0.0) {
            scratch.values[2018] = 0.0;
            scratch.node_derivatives[2018] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2018] = [0.0; Instance::BRANCH_COUNT];
        }

        if (!(scratch.values[2260] != 0.0)) {
            scratch.store_ad(2018, &AdValue::mul(scratch.ad_value(2075), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2016), scratch.ad_value(2014)), scratch.ad_value(2012)), scratch.ad_value(2017))));
        }

        scratch.values[2261] = if (((((scratch.values[2] != 0.0) && ((scratch.values[255] > 0.0) || (scratch.values[256] > 0.0))) || ((scratch.values[4] != 0.0) && ((scratch.values[261] > 0.0) || (scratch.values[262] > 0.0)))) || (scratch.values[270] > 0.0)) || (scratch.values[271] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2261] != 0.0) {
            scratch.store_ad(886, &AdValue::scale(AdValue::add(scratch.ad_value(867), AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(867)), scratch.ad_value(824)))), 0.5));
        }

        if (scratch.values[2261] != 0.0) {
            scratch.store_ad(887, &AdValue::add(AdValue::add(AdValue::sub(AdValue::neg(scratch.ad_value(886)), AdValue::scale(scratch.ad_value(822), 0.5)), AdValue::mul(scratch.ad_value(820), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(886), AdValue::scale(scratch.ad_value(822), 0.25)), scratch.ad_value(825))))), scratch.ad_value(826)));
        }

        if (scratch.values[2261] != 0.0) {
            scratch.store_ad(886, &AdValue::scale(AdValue::add(scratch.ad_value(868), AdValue::sqrt(AdValue::add(AdValue::square(scratch.ad_value(868)), scratch.ad_value(827)))), 0.5));
        }

        if (scratch.values[2261] != 0.0) {
            scratch.store_ad(888, &AdValue::add(AdValue::add(AdValue::sub(AdValue::neg(scratch.ad_value(886)), AdValue::scale(scratch.ad_value(823), 0.5)), AdValue::mul(scratch.ad_value(821), AdValue::sqrt(AdValue::add(AdValue::add(scratch.ad_value(886), AdValue::scale(scratch.ad_value(823), 0.25)), scratch.ad_value(828))))), scratch.ad_value(829)));
        }

        if (scratch.values[2261] != 0.0) {
            scratch.store_ad(889, &AdValue::scale(AdValue::add(scratch.ad_value(867), scratch.ad_value(887)), (-scratch.values[363])));
        }

        if (scratch.values[2261] != 0.0) {
            scratch.store_ad(890, &AdValue::scale(AdValue::add(scratch.ad_value(868), scratch.ad_value(888)), (-scratch.values[363])));
        }

        scratch.values[2022] = 0.0;

        scratch.values[2021] = 0.0;

        scratch.values[917] = 0.0;

        scratch.values[918] = 0.0;

        scratch.values[2024] = 0.0;

        scratch.values[2023] = 0.0;

        scratch.values[2262] = if (scratch.values[2] != 0.0) { 1.0 } else { 0.0 };

        scratch.values[2263] = if (scratch.values[255] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) {
            scratch.store_ad(891, &AdValue::mul(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(889)), 1e-6)), scratch.ad_value(830)));
        }

        scratch.values[2264] = if (scratch.values[259] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) && (scratch.values[2264] != 0.0)) {
            scratch.store_ad(891, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(891), scratch.ad_value(835)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(891), scratch.ad_value(835)), AdValue::sub(scratch.ad_value(891), scratch.ad_value(835))), 1e-6))), 0.5));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) {
            scratch.store_ad(2086, &AdValue::mul(scratch.ad_value(833), AdValue::offset(AdValue::mul(scratch.ad_value(891), AdValue::add(scratch.ad_value(258), AdValue::mul(scratch.ad_value(259), scratch.ad_value(891)))), (-1.5))));
        }

        scratch.values[2265] = if (scratch.values[2086] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) && (scratch.values[2265] != 0.0)) {
            scratch.store_ad(892, &AdValue::offset(AdValue::mul(scratch.ad_value(2086), AdValue::offset(AdValue::scale(AdValue::mul(scratch.ad_value(2086), AdValue::offset(AdValue::scale(scratch.ad_value(2086), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        scratch.values[2266] = if (scratch.values[2086] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) && (!(scratch.values[2265] != 0.0))) && (scratch.values[2266] != 0.0)) {
            scratch.store_ad(892, &AdValue::exp(scratch.ad_value(2086)));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) && (!(scratch.values[2265] != 0.0))) && (!(scratch.values[2266] != 0.0))) {
            scratch.store_ad(892, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) {
            scratch.store_ad(893, &AdValue::offset(scratch.ad_value(887), 3.0));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) {
            scratch.store_ad(894, &AdValue::sub_from_scalar((-3.0), scratch.ad_value(253)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) {
            scratch.store_ad(895, &AdValue::scale(scratch.ad_value(856), 30.0));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) {
            scratch.values[956] = (4.0 - 0.9);
            scratch.node_derivatives[956] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[956] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) {
            scratch.store_ad(957, &AdValue::add(scratch.ad_value(893), scratch.ad_value(895)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) {
            scratch.store_ad(2086, &AdValue::mul(AdValue::div_from_scalar(2.0, scratch.ad_value(956)), AdValue::sub(scratch.ad_value(957), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(957)), AdValue::mul(AdValue::mul(scratch.ad_value(956), scratch.ad_value(893)), scratch.ad_value(895)))))));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) {
            scratch.values[956] = (4.0 - 0.3);
            scratch.node_derivatives[956] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[956] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) {
            scratch.store_ad(957, &AdValue::add(scratch.ad_value(894), scratch.ad_value(2086)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) {
            scratch.store_ad(896, &AdValue::mul(AdValue::div_from_scalar(2.0, scratch.ad_value(956)), AdValue::add(scratch.ad_value(957), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(957)), AdValue::mul(AdValue::mul(scratch.ad_value(956), scratch.ad_value(894)), scratch.ad_value(2086)))))));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2263] != 0.0)) {
            scratch.store_ad(2022, &AdValue::mul(scratch.ad_value(255), AdValue::mul(scratch.ad_value(892), scratch.ad_value(896))));
        }

        scratch.values[2267] = if (scratch.values[256] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) {
            scratch.store_ad(891, &AdValue::mul(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(890)), 1e-6)), scratch.ad_value(830)));
        }

        scratch.values[2268] = if (scratch.values[259] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) && (scratch.values[2268] != 0.0)) {
            scratch.store_ad(891, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(891), scratch.ad_value(835)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(891), scratch.ad_value(835)), AdValue::sub(scratch.ad_value(891), scratch.ad_value(835))), 1e-6))), 0.5));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) {
            scratch.store_ad(2086, &AdValue::mul(scratch.ad_value(834), AdValue::offset(AdValue::mul(scratch.ad_value(891), AdValue::add(scratch.ad_value(258), AdValue::mul(scratch.ad_value(259), scratch.ad_value(891)))), (-1.5))));
        }

        scratch.values[2269] = if (scratch.values[2086] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) && (scratch.values[2269] != 0.0)) {
            scratch.store_ad(892, &AdValue::offset(AdValue::mul(scratch.ad_value(2086), AdValue::offset(AdValue::scale(AdValue::mul(scratch.ad_value(2086), AdValue::offset(AdValue::scale(scratch.ad_value(2086), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        scratch.values[2270] = if (scratch.values[2086] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) && (!(scratch.values[2269] != 0.0))) && (scratch.values[2270] != 0.0)) {
            scratch.store_ad(892, &AdValue::exp(scratch.ad_value(2086)));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) && (!(scratch.values[2269] != 0.0))) && (!(scratch.values[2270] != 0.0))) {
            scratch.store_ad(892, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) {
            scratch.store_ad(893, &AdValue::offset(scratch.ad_value(888), 3.0));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) {
            scratch.store_ad(894, &AdValue::sub_from_scalar((-3.0), scratch.ad_value(253)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) {
            scratch.store_ad(895, &AdValue::scale(scratch.ad_value(859), 30.0));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) {
            scratch.values[956] = (4.0 - 0.9);
            scratch.node_derivatives[956] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[956] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) {
            scratch.store_ad(957, &AdValue::add(scratch.ad_value(893), scratch.ad_value(895)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) {
            scratch.store_ad(2086, &AdValue::mul(AdValue::div_from_scalar(2.0, scratch.ad_value(956)), AdValue::sub(scratch.ad_value(957), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(957)), AdValue::mul(AdValue::mul(scratch.ad_value(956), scratch.ad_value(893)), scratch.ad_value(895)))))));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) {
            scratch.values[956] = (4.0 - 0.3);
            scratch.node_derivatives[956] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[956] = [0.0; Instance::BRANCH_COUNT];
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) {
            scratch.store_ad(957, &AdValue::add(scratch.ad_value(894), scratch.ad_value(2086)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) {
            scratch.store_ad(896, &AdValue::mul(AdValue::div_from_scalar(2.0, scratch.ad_value(956)), AdValue::add(scratch.ad_value(957), AdValue::sqrt(AdValue::sub(AdValue::square(scratch.ad_value(957)), AdValue::mul(AdValue::mul(scratch.ad_value(956), scratch.ad_value(894)), scratch.ad_value(2086)))))));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2267] != 0.0)) {
            scratch.store_ad(2021, &AdValue::mul(scratch.ad_value(256), AdValue::mul(scratch.ad_value(892), scratch.ad_value(896))));
        }

        scratch.values[2271] = if (scratch.values[254] > 0.0) { 1.0 } else { 0.0 };

        scratch.values[2272] = if (scratch.values[2010] <= 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (scratch.values[2272] != 0.0)) {
            scratch.store_ad(2086, &AdValue::pow(AdValue::div(scratch.ad_value(850), scratch.ad_value(865)), scratch.ad_value(243)));
        }

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (scratch.values[2272] != 0.0)) {
            scratch.store_ad(885, &AdValue::mul(AdValue::mul(scratch.ad_value(850), AdValue::pow(AdValue::offset(scratch.ad_value(2086), 1.0), AdValue::neg(scratch.ad_value(816)))), scratch.ad_value(2009)));
        }

        scratch.values[2273] = if ((scratch.values[881] - scratch.values[885]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (scratch.values[2273] != 0.0)) {
            scratch.store_ad(2086, &AdValue::exp(AdValue::sub(scratch.ad_value(881), scratch.ad_value(885))));
        }

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2273] != 0.0))) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(881), scratch.ad_value(885))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(881), scratch.ad_value(885))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(881), scratch.ad_value(885))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(897, &AdValue::add(scratch.ad_value(869), AdValue::mul(scratch.ad_value(2008), AdValue::sub(AdValue::scale(scratch.ad_value(881), 0.5), AdValue::ln(AdValue::scale(AdValue::offset(scratch.ad_value(2086), 1.0), 0.5))))));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(898, &AdValue::mul(scratch.ad_value(253), scratch.ad_value(2008)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(899, &AdValue::add(scratch.ad_value(876), scratch.ad_value(898)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(900, &AdValue::scale(AdValue::sub(scratch.ad_value(899), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::neg(scratch.ad_value(899)), AdValue::neg(scratch.ad_value(899))), 0.01))), 0.5));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(891, &AdValue::mul(AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(876)), 1e-6)), scratch.ad_value(830)));
        }

        scratch.values[2274] = if (scratch.values[259] < 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (scratch.values[2274] != 0.0)) {
            scratch.store_ad(891, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(891), scratch.ad_value(835)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(891), scratch.ad_value(835)), AdValue::sub(scratch.ad_value(891), scratch.ad_value(835))), 1e-6))), 0.5));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(901, &AdValue::add(scratch.ad_value(882), AdValue::mul(AdValue::sub(AdValue::sub(scratch.ad_value(900), scratch.ad_value(781)), scratch.ad_value(897)), scratch.ad_value(2009))));
        }

        scratch.values[2275] = if (((scratch.values[901]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (scratch.values[2275] != 0.0)) {
            scratch.store_ad(902, &AdValue::exp(scratch.ad_value(901)));
        }

        scratch.values[2276] = if (scratch.values[901] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2275] != 0.0))) && (scratch.values[2276] != 0.0)) {
            scratch.store_ad(902, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(901)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(901)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(901)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2275] != 0.0))) && (!(scratch.values[2276] != 0.0))) {
            scratch.store_ad(902, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(901), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(901), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(901), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(901, &AdValue::mul(AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(849), scratch.ad_value(869)), scratch.ad_value(897))), scratch.ad_value(2009)));
        }

        scratch.values[2277] = if (((scratch.values[901]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (scratch.values[2277] != 0.0)) {
            scratch.store_ad(2086, &AdValue::exp(scratch.ad_value(901)));
        }

        scratch.values[2278] = if (scratch.values[901] < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2277] != 0.0))) && (scratch.values[2278] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(901)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(901)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(901)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2277] != 0.0))) && (!(scratch.values[2278] != 0.0))) {
            scratch.store_ad(2086, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(901), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(901), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(901), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(903, &AdValue::mul(scratch.ad_value(902), scratch.ad_value(2086)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(2086, &AdValue::mul(scratch.ad_value(832), AdValue::offset(AdValue::mul(scratch.ad_value(891), AdValue::add(scratch.ad_value(258), AdValue::mul(scratch.ad_value(259), scratch.ad_value(891)))), (-1.5))));
        }

        scratch.values[2279] = if (scratch.values[2086] > 0.0) { 1.0 } else { 0.0 };

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (scratch.values[2279] != 0.0)) {
            scratch.store_ad(892, &AdValue::offset(AdValue::mul(scratch.ad_value(2086), AdValue::offset(AdValue::scale(AdValue::mul(scratch.ad_value(2086), AdValue::offset(AdValue::scale(scratch.ad_value(2086), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
        }

        scratch.values[2280] = if (scratch.values[2086] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2279] != 0.0))) && (scratch.values[2280] != 0.0)) {
            scratch.store_ad(892, &AdValue::exp(scratch.ad_value(2086)));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2279] != 0.0))) && (!(scratch.values[2280] != 0.0))) {
            scratch.store_ad(892, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(904, &AdValue::mul(scratch.ad_value(254), AdValue::mul(scratch.ad_value(892), AdValue::ln(AdValue::div(AdValue::offset(scratch.ad_value(902), 1.0), AdValue::offset(scratch.ad_value(903), 1.0))))));
        }

        scratch.values[2281] = if ((scratch.values[2010] <= 0.0) || ((scratch.values[258] == 0.0) && (scratch.values[259] == 0.0))) { 1.0 } else { 0.0 };

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (scratch.values[2281] != 0.0)) {
            scratch.values[905] = 1.0;
            scratch.node_derivatives[905] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[905] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (scratch.values[2281] != 0.0)) {
            scratch.values[906] = 0.5;
            scratch.node_derivatives[906] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[906] = [0.0; Instance::BRANCH_COUNT];
        }

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(2086, &AdValue::add(scratch.ad_value(258), AdValue::mul(AdValue::scale(scratch.ad_value(259), 2.0), scratch.ad_value(891))));
        }

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(907, &AdValue::div(scratch.ad_value(260), AdValue::mul(scratch.ad_value(2086), scratch.ad_value(832))));
        }

    }

    pub(super) fn stamp_transient_block_34(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(908, &AdValue::scale(AdValue::div(scratch.ad_value(2012), scratch.ad_value(907)), 0.5));
        }

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(909, &AdValue::div(scratch.ad_value(907), scratch.ad_value(2015)));
        }

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(910, &AdValue::scale(AdValue::mul(scratch.ad_value(909), AdValue::sub_from_scalar(1.0, scratch.ad_value(909))), 0.5));
        }

        if (((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) {
            scratch.store_ad(911, &AdValue::sub_from_scalar(0.5, AdValue::scale(scratch.ad_value(910), 3.0)));
        }

        scratch.values[2282] = if (scratch.values[908] < 0.001) { 1.0 } else { 0.0 };

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2282] != 0.0)) {
            scratch.store_ad(912, &AdValue::square(scratch.ad_value(908)));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2282] != 0.0)) {
            scratch.store_ad(905, &AdValue::offset(AdValue::mul(scratch.ad_value(912), AdValue::add(AdValue::offset(AdValue::scale(scratch.ad_value(909), 0.3333333333333333), 0.16666666666666666), AdValue::scale(AdValue::mul(scratch.ad_value(912), AdValue::offset(AdValue::scale(scratch.ad_value(909), 0.2), 0.05)), 0.16666666666666666))), 1.0));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) && (scratch.values[2282] != 0.0)) {
            scratch.store_ad(906, &AdValue::sub(AdValue::scale(scratch.ad_value(905), 0.5), AdValue::scale(AdValue::mul(scratch.ad_value(908), AdValue::offset(AdValue::mul(scratch.ad_value(912), AdValue::add(AdValue::scale(AdValue::offset(scratch.ad_value(910), 0.25), 0.4), AdValue::scale(AdValue::mul(scratch.ad_value(912), AdValue::offset(scratch.ad_value(910), 0.125)), 0.0285714285714))), 1.0)), 0.16666666666666666)));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2282] != 0.0))) {
            scratch.store_ad(913, &AdValue::div_from_scalar(1.0, scratch.ad_value(908)));
        }

        scratch.values[2283] = if (((scratch.values[908]) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2282] != 0.0))) && (scratch.values[2283] != 0.0)) {
            scratch.store_ad(914, &AdValue::exp(scratch.ad_value(908)));
        }

        scratch.values[2284] = if (scratch.values[908] < 0.0) { 1.0 } else { 0.0 };

        if ((((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2282] != 0.0))) && (!(scratch.values[2283] != 0.0))) && (scratch.values[2284] != 0.0)) {
            scratch.store_ad(914, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(908)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(908)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(908)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2282] != 0.0))) && (!(scratch.values[2283] != 0.0))) && (!(scratch.values[2284] != 0.0))) {
            scratch.store_ad(914, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(908), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(908), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(908), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2282] != 0.0))) {
            scratch.store_ad(915, &AdValue::div_from_scalar(1.0, scratch.ad_value(914)));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2282] != 0.0))) {
            scratch.store_ad(2086, &AdValue::sub(scratch.ad_value(914), scratch.ad_value(915)));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2282] != 0.0))) {
            scratch.store_ad(2088, &AdValue::add(scratch.ad_value(914), scratch.ad_value(915)));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2282] != 0.0))) {
            scratch.store_ad(905, &AdValue::scale(AdValue::add(AdValue::mul(AdValue::mul(AdValue::sub_from_scalar(1.0, scratch.ad_value(909)), scratch.ad_value(2086)), scratch.ad_value(913)), AdValue::mul(scratch.ad_value(909), scratch.ad_value(2088))), 0.5));
        }

        if ((((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) && (!(scratch.values[2281] != 0.0))) && (!(scratch.values[2282] != 0.0))) {
            scratch.store_ad(906, &AdValue::scale(AdValue::sub(AdValue::sub(scratch.ad_value(905), AdValue::mul(scratch.ad_value(2086), AdValue::sub(scratch.ad_value(910), AdValue::mul(AdValue::mul(scratch.ad_value(911), scratch.ad_value(913)), scratch.ad_value(913))))), AdValue::mul(AdValue::mul(scratch.ad_value(911), scratch.ad_value(2088)), scratch.ad_value(913))), 0.5));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(916, &AdValue::scale(AdValue::offset(AdValue::div(scratch.ad_value(2010), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2010)), 1e-6))), 1.0), 0.5));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(917, &AdValue::mul(AdValue::mul(scratch.ad_value(904), scratch.ad_value(905)), scratch.ad_value(916)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(2023, &AdValue::mul(AdValue::mul(scratch.ad_value(904), scratch.ad_value(906)), scratch.ad_value(916)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(2024, &AdValue::sub(scratch.ad_value(917), scratch.ad_value(2023)));
        }

        if ((scratch.values[2262] != 0.0) && (scratch.values[2271] != 0.0)) {
            scratch.store_ad(918, &AdValue::mul(AdValue::mul(scratch.ad_value(904), scratch.ad_value(905)), AdValue::sub_from_scalar(1.0, scratch.ad_value(916))));
        }

        scratch.values[920] = 0.0;

        scratch.values[922] = 0.0;

        scratch.values[2285] = if (scratch.values[4] != 0.0) { 1.0 } else { 0.0 };

        scratch.values[2286] = if ((scratch.values[262] > 0.0) && (scratch.values[890] < 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2285] != 0.0) && (scratch.values[2286] != 0.0)) {
            scratch.store_ad(919, &AdValue::sqrt(AdValue::offset(AdValue::add(AdValue::square(scratch.ad_value(890)), AdValue::mul(AdValue::square(scratch.ad_value(268)), AdValue::square(scratch.ad_value(858)))), 1e-6)));
        }

        if ((scratch.values[2285] != 0.0) && (scratch.values[2286] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div(AdValue::neg(scratch.ad_value(840)), scratch.ad_value(919)));
        }

        scratch.values[2287] = if (scratch.values[2086] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2285] != 0.0) && (scratch.values[2286] != 0.0)) && (scratch.values[2287] != 0.0)) {
            scratch.store_ad(2088, &AdValue::exp(scratch.ad_value(2086)));
        }

        if (((scratch.values[2285] != 0.0) && (scratch.values[2286] != 0.0)) && (!(scratch.values[2287] != 0.0))) {
            scratch.store_ad(2088, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2285] != 0.0) && (scratch.values[2286] != 0.0)) {
            scratch.store_ad(920, &AdValue::mul(AdValue::neg(scratch.ad_value(838)), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(858), scratch.ad_value(890)), scratch.ad_value(919)), scratch.ad_value(2088))));
        }

        scratch.values[2288] = if ((scratch.values[261] > 0.0) && (scratch.values[889] < 0.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2285] != 0.0) && (scratch.values[2288] != 0.0)) {
            scratch.store_ad(921, &AdValue::sqrt(AdValue::offset(AdValue::add(AdValue::square(scratch.ad_value(889)), AdValue::mul(AdValue::square(scratch.ad_value(267)), AdValue::square(scratch.ad_value(857)))), 1e-6)));
        }

        if ((scratch.values[2285] != 0.0) && (scratch.values[2288] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div(AdValue::neg(scratch.ad_value(839)), scratch.ad_value(921)));
        }

        scratch.values[2289] = if (scratch.values[2086] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2285] != 0.0) && (scratch.values[2288] != 0.0)) && (scratch.values[2289] != 0.0)) {
            scratch.store_ad(2088, &AdValue::exp(scratch.ad_value(2086)));
        }

        if (((scratch.values[2285] != 0.0) && (scratch.values[2288] != 0.0)) && (!(scratch.values[2289] != 0.0))) {
            scratch.store_ad(2088, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2086)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((scratch.values[2285] != 0.0) && (scratch.values[2288] != 0.0)) {
            scratch.store_ad(922, &AdValue::mul(AdValue::neg(scratch.ad_value(837)), AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(857), scratch.ad_value(889)), scratch.ad_value(921)), scratch.ad_value(2088))));
        }

        scratch.values[2058] = 0.0;

        scratch.values[2064] = 0.0;

        scratch.values[2066] = 0.0;

        scratch.values[2067] = 1e-40;

        scratch.values[2068] = 1.0;

        scratch.values[2069] = 0.0;

        scratch.values[2290] = if ((scratch.values[8] != 0.0) && (scratch.values[286] > 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2086, &AdValue::add(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(852), scratch.ad_value(851)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(852), scratch.ad_value(851)), AdValue::sub(scratch.ad_value(852), scratch.ad_value(851))), scratch.ad_value(803)))), 0.5), scratch.ad_value(801)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2051, &AdValue::add(AdValue::sub(scratch.ad_value(851), AdValue::scale(AdValue::sub(scratch.ad_value(2086), AdValue::sqrt(AdValue::add(AdValue::mul(scratch.ad_value(2086), scratch.ad_value(2086)), scratch.ad_value(802)))), 0.5)), scratch.ad_value(805)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2052, &AdValue::add(scratch.ad_value(2051), AdValue::scale(AdValue::sub(scratch.ad_value(850), scratch.ad_value(860)), 0.5)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2053, &AdValue::mul(AdValue::mul(scratch.ad_value(288), AdValue::offset(AdValue::mul(scratch.ad_value(290), scratch.ad_value(860)), 1.0)), AdValue::offset(AdValue::mul(scratch.ad_value(289), scratch.ad_value(2052)), 1.0)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2054, &AdValue::mul(scratch.ad_value(2081), AdValue::offset(scratch.ad_value(2053), 1.0)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2055, &AdValue::div_from_scalar(1.0, scratch.ad_value(2054)));
        }

        scratch.values[2291] = if (scratch.values[292] < 0.05) { 1.0 } else { 0.0 };

        if ((scratch.values[2290] != 0.0) && (scratch.values[2291] != 0.0)) {
            scratch.values[2056] = scratch.values[860];
            scratch.node_derivatives[2056] = scratch.node_derivatives[860];
            scratch.branch_derivatives[2056] = scratch.branch_derivatives[860];
        }

        if ((scratch.values[2290] != 0.0) && (!(scratch.values[2291] != 0.0))) {
            scratch.store_ad(2056, &AdValue::div(AdValue::scale(AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(292), scratch.ad_value(860)), 1.0)), (-1.0)), 2.0), scratch.ad_value(292)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2057, &AdValue::mul(AdValue::mul(scratch.ad_value(291), scratch.ad_value(2056)), AdValue::offset(AdValue::mul(scratch.ad_value(293), scratch.ad_value(2052)), 1.0)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2058, &AdValue::mul(scratch.ad_value(2055), AdValue::sub(AdValue::add(AdValue::add(scratch.ad_value(849), scratch.ad_value(2051)), scratch.ad_value(2057)), scratch.ad_value(764))));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2059, &AdValue::mul(scratch.ad_value(2055), scratch.ad_value(799)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2060, &AdValue::scale(AdValue::ln(AdValue::add(AdValue::div(scratch.ad_value(2059), scratch.ad_value(800)), AdValue::sqrt(scratch.ad_value(2059)))), 2.0));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2061, &AdValue::mul(scratch.ad_value(2055), scratch.ad_value(2051)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(963, &AdValue::add(scratch.ad_value(2059), scratch.ad_value(2061)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(964, &AdValue::add(scratch.ad_value(963), AdValue::mul(scratch.ad_value(800), AdValue::sqrt(scratch.ad_value(963)))));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(965, &AdValue::add(scratch.ad_value(964), scratch.ad_value(2060)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(966, &AdValue::offset(AdValue::div(scratch.ad_value(800), AdValue::scale(AdValue::sqrt(scratch.ad_value(963)), 2.0)), 1.0));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(967, &AdValue::div_from_scalar(1.0, scratch.ad_value(966)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(968, &AdValue::sub(scratch.ad_value(2058), scratch.ad_value(965)));
        }

        scratch.values[2292] = if (scratch.values[968] > (-12.0)) { 1.0 } else { 0.0 };

        if ((scratch.values[2290] != 0.0) && (scratch.values[2292] != 0.0)) {
            scratch.store_ad(969, &AdValue::offset(AdValue::add(scratch.ad_value(968), scratch.ad_value(2083)), (-1.0)));
        }

        if ((scratch.values[2290] != 0.0) && (scratch.values[2292] != 0.0)) {
            scratch.store_ad(970, &AdValue::scale(AdValue::add(scratch.ad_value(969), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(969)), 10.0))), 0.5));
        }

        if ((scratch.values[2290] != 0.0) && (scratch.values[2292] != 0.0)) {
            scratch.store_ad(971, &AdValue::add(AdValue::sub(scratch.ad_value(968), AdValue::mul(scratch.ad_value(966), AdValue::ln(scratch.ad_value(970)))), scratch.ad_value(2083)));
        }

        if ((scratch.values[2290] != 0.0) && (scratch.values[2292] != 0.0)) {
            scratch.store_ad(972, &AdValue::scale(AdValue::add(scratch.ad_value(971), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(971)), 2.0))), 0.5));
        }

        scratch.values[2293] = if ((scratch.values[968] - scratch.values[972]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2290] != 0.0) && (scratch.values[2292] != 0.0)) && (scratch.values[2293] != 0.0)) {
            scratch.store_ad(973, &AdValue::exp(AdValue::sub(scratch.ad_value(968), scratch.ad_value(972))));
        }

        if (((scratch.values[2290] != 0.0) && (scratch.values[2292] != 0.0)) && (!(scratch.values[2293] != 0.0))) {
            scratch.store_ad(973, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(968), scratch.ad_value(972)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(968), scratch.ad_value(972)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(968), scratch.ad_value(972)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((scratch.values[2290] != 0.0) && (scratch.values[2292] != 0.0)) {
            scratch.store_ad(974, &AdValue::mul(scratch.ad_value(2082), scratch.ad_value(973)));
        }

        if ((scratch.values[2290] != 0.0) && (scratch.values[2292] != 0.0)) {
            scratch.store_ad(975, &AdValue::pow(scratch.ad_value(974), scratch.ad_value(967)));
        }

        if ((scratch.values[2290] != 0.0) && (scratch.values[2292] != 0.0)) {
            scratch.store_ad(976, &AdValue::add(AdValue::square(scratch.ad_value(966)), AdValue::mul(AdValue::sub(AdValue::scale(AdValue::add(scratch.ad_value(972), scratch.ad_value(966)), 2.0), scratch.ad_value(975)), scratch.ad_value(975))));
        }

        if ((scratch.values[2290] != 0.0) && (scratch.values[2292] != 0.0)) {
            scratch.store_ad(977, &AdValue::mul(scratch.ad_value(966), AdValue::offset(AdValue::div(AdValue::sub(AdValue::sqrt(scratch.ad_value(976)), scratch.ad_value(966)), scratch.ad_value(975)), (-1.0))));
        }

        if ((scratch.values[2290] != 0.0) && (scratch.values[2292] != 0.0)) {
            scratch.store_ad(2062, &AdValue::sub(scratch.ad_value(972), scratch.ad_value(977)));
        }

        scratch.values[2294] = if ((scratch.values[967] * (scratch.values[968] + scratch.values[2083])) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2290] != 0.0) && (!(scratch.values[2292] != 0.0))) && (scratch.values[2294] != 0.0)) {
            scratch.store_ad(2062, &AdValue::exp(AdValue::mul(scratch.ad_value(967), AdValue::add(scratch.ad_value(968), scratch.ad_value(2083)))));
        }

        if (((scratch.values[2290] != 0.0) && (!(scratch.values[2292] != 0.0))) && (!(scratch.values[2294] != 0.0))) {
            let assign46420_ad_e59710: AdValue = AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(967), AdValue::add(scratch.ad_value(968), scratch.ad_value(2083)))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(967), AdValue::add(scratch.ad_value(968), scratch.ad_value(2083)))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(967), AdValue::add(scratch.ad_value(968), scratch.ad_value(2083)))), 0.3333333333333333), 1.0)), 0.5), 1.0));
            scratch.store_ad(2062, &AdValue::div_from_scalar(1e-100, AdValue::offset(assign46420_ad_e59710, 1.0)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2063, &AdValue::mul(scratch.ad_value(2055), AdValue::add(scratch.ad_value(2050), scratch.ad_value(2051))));
        }

        scratch.values[2295] = if ((scratch.values[2062] < 0.001) && (scratch.values[2050] < 1e-6)) { 1.0 } else { 0.0 };

        scratch.values[2296] = if (((-scratch.values[2063]) + scratch.values[2061]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2290] != 0.0) && (scratch.values[2295] != 0.0)) && (scratch.values[2296] != 0.0)) {
            scratch.store_ad(2086, &AdValue::exp(AdValue::sub(scratch.ad_value(2061), scratch.ad_value(2063))));
        }

        if (((scratch.values[2290] != 0.0) && (scratch.values[2295] != 0.0)) && (!(scratch.values[2296] != 0.0))) {
            let assign46470_ad_e59789: AdValue = AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(2061), scratch.ad_value(2063))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(2061), scratch.ad_value(2063))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::sub(scratch.ad_value(2061), scratch.ad_value(2063))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0));
            scratch.store_ad(2086, &assign46470_ad_e59789);
        }

        if ((scratch.values[2290] != 0.0) && (scratch.values[2295] != 0.0)) {
            scratch.store_ad(2064, &AdValue::mul(scratch.ad_value(2062), AdValue::offset(scratch.ad_value(2086), (-1.0))));
        }

        if ((scratch.values[2290] != 0.0) && (scratch.values[2295] != 0.0)) {
            scratch.store_ad(2065, &AdValue::add(scratch.ad_value(2064), scratch.ad_value(2062)));
        }

        if ((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) {
            scratch.store_ad(963, &AdValue::add(scratch.ad_value(2059), scratch.ad_value(2063)));
        }

        if ((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) {
            scratch.store_ad(964, &AdValue::add(scratch.ad_value(963), AdValue::mul(scratch.ad_value(800), AdValue::sqrt(scratch.ad_value(963)))));
        }

        if ((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) {
            scratch.store_ad(965, &AdValue::add(scratch.ad_value(964), scratch.ad_value(2060)));
        }

        if ((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) {
            scratch.store_ad(966, &AdValue::offset(AdValue::div(scratch.ad_value(800), AdValue::scale(AdValue::sqrt(scratch.ad_value(963)), 2.0)), 1.0));
        }

        if ((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) {
            scratch.store_ad(967, &AdValue::div_from_scalar(1.0, scratch.ad_value(966)));
        }

        if ((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) {
            scratch.store_ad(968, &AdValue::sub(scratch.ad_value(2058), scratch.ad_value(965)));
        }

        scratch.values[2297] = if (scratch.values[968] > (-12.0)) { 1.0 } else { 0.0 };

        if (((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (scratch.values[2297] != 0.0)) {
            scratch.store_ad(969, &AdValue::offset(AdValue::add(scratch.ad_value(968), scratch.ad_value(2083)), (-1.0)));
        }

        if (((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (scratch.values[2297] != 0.0)) {
            scratch.store_ad(970, &AdValue::scale(AdValue::add(scratch.ad_value(969), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(969)), 10.0))), 0.5));
        }

        if (((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (scratch.values[2297] != 0.0)) {
            scratch.store_ad(971, &AdValue::add(AdValue::sub(scratch.ad_value(968), AdValue::mul(scratch.ad_value(966), AdValue::ln(scratch.ad_value(970)))), scratch.ad_value(2083)));
        }

        if (((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (scratch.values[2297] != 0.0)) {
            scratch.store_ad(972, &AdValue::scale(AdValue::add(scratch.ad_value(971), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(971)), 2.0))), 0.5));
        }

        scratch.values[2298] = if ((scratch.values[968] - scratch.values[972]) < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (scratch.values[2297] != 0.0)) && (scratch.values[2298] != 0.0)) {
            scratch.store_ad(973, &AdValue::exp(AdValue::sub(scratch.ad_value(968), scratch.ad_value(972))));
        }

        if ((((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (scratch.values[2297] != 0.0)) && (!(scratch.values[2298] != 0.0))) {
            scratch.store_ad(973, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(968), scratch.ad_value(972)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::sub(scratch.ad_value(968), scratch.ad_value(972)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::sub(scratch.ad_value(968), scratch.ad_value(972)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (scratch.values[2297] != 0.0)) {
            scratch.store_ad(974, &AdValue::mul(scratch.ad_value(2082), scratch.ad_value(973)));
        }

        if (((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (scratch.values[2297] != 0.0)) {
            scratch.store_ad(975, &AdValue::pow(scratch.ad_value(974), scratch.ad_value(967)));
        }

        if (((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (scratch.values[2297] != 0.0)) {
            scratch.store_ad(976, &AdValue::add(AdValue::square(scratch.ad_value(966)), AdValue::mul(AdValue::sub(AdValue::scale(AdValue::add(scratch.ad_value(972), scratch.ad_value(966)), 2.0), scratch.ad_value(975)), scratch.ad_value(975))));
        }

        if (((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (scratch.values[2297] != 0.0)) {
            scratch.store_ad(977, &AdValue::mul(scratch.ad_value(966), AdValue::offset(AdValue::div(AdValue::sub(AdValue::sqrt(scratch.ad_value(976)), scratch.ad_value(966)), scratch.ad_value(975)), (-1.0))));
        }

        if (((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (scratch.values[2297] != 0.0)) {
            scratch.store_ad(2065, &AdValue::sub(scratch.ad_value(972), scratch.ad_value(977)));
        }

        scratch.values[2299] = if ((scratch.values[967] * (scratch.values[968] + scratch.values[2083])) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (!(scratch.values[2297] != 0.0))) && (scratch.values[2299] != 0.0)) {
            scratch.store_ad(2065, &AdValue::exp(AdValue::mul(scratch.ad_value(967), AdValue::add(scratch.ad_value(968), scratch.ad_value(2083)))));
        }

        if ((((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) && (!(scratch.values[2297] != 0.0))) && (!(scratch.values[2299] != 0.0))) {
            let assign46710_ad_e60142: AdValue = AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(967), AdValue::add(scratch.ad_value(968), scratch.ad_value(2083)))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(967), AdValue::add(scratch.ad_value(968), scratch.ad_value(2083)))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::mul(scratch.ad_value(967), AdValue::add(scratch.ad_value(968), scratch.ad_value(2083)))), 0.3333333333333333), 1.0)), 0.5), 1.0));
            scratch.store_ad(2065, &AdValue::div_from_scalar(1e-100, AdValue::offset(assign46710_ad_e60142, 1.0)));
        }

        if ((scratch.values[2290] != 0.0) && (!(scratch.values[2295] != 0.0))) {
            scratch.store_ad(2064, &AdValue::sub(scratch.ad_value(2065), scratch.ad_value(2062)));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2066, &AdValue::scale(AdValue::add(scratch.ad_value(2065), scratch.ad_value(2062)), 0.5));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2067, &AdValue::max_with_scalar(AdValue::sub(scratch.ad_value(2058), scratch.ad_value(2066)), 1e-40));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2068, &AdValue::sub_from_scalar(1.0, AdValue::div(AdValue::scale(scratch.ad_value(800), 0.5), AdValue::sqrt(AdValue::add(scratch.ad_value(2067), AdValue::scale(scratch.ad_value(2082), 0.25))))));
        }

        if (scratch.values[2290] != 0.0) {
            scratch.store_ad(2069, &AdValue::div(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::neg(scratch.ad_value(2076)), scratch.ad_value(2054)), scratch.ad_value(2054)), AdValue::offset(AdValue::mul(scratch.ad_value(2068), scratch.ad_value(2066)), 1.0)), scratch.ad_value(2064)), scratch.ad_value(2048)));
        }

        scratch.values[2300] = if ((scratch.values[2010] > 0.0) && (scratch.values[3] != 0.0)) { 1.0 } else { 0.0 };

        if (scratch.values[2300] != 0.0) {
            scratch.store_ad(923, &AdValue::sub(scratch.ad_value(850), AdValue::mul(scratch.ad_value(251), scratch.ad_value(2012))));
        }

        scratch.values[2301] = if (scratch.values[923] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2300] != 0.0) && (scratch.values[2301] != 0.0)) {
            scratch.store_ad(2088, &AdValue::mul(scratch.ad_value(763), AdValue::div(AdValue::offset(AdValue::mul(scratch.ad_value(252), AdValue::sub(AdValue::sqrt(AdValue::add(scratch.ad_value(767), scratch.ad_value(869))), scratch.ad_value(775))), 1.0), AdValue::offset(scratch.ad_value(923), 1e-30))));
        }

        scratch.values[2302] = if ((((-scratch.values[2088])) as f64).abs() < 230.25850929940458) { 1.0 } else { 0.0 };

        if (((scratch.values[2300] != 0.0) && (scratch.values[2301] != 0.0)) && (scratch.values[2302] != 0.0)) {
            scratch.store_ad(2086, &AdValue::exp(AdValue::neg(scratch.ad_value(2088))));
        }

        scratch.values[2303] = if ((-scratch.values[2088]) < 0.0) { 1.0 } else { 0.0 };

        if ((((scratch.values[2300] != 0.0) && (scratch.values[2301] != 0.0)) && (!(scratch.values[2302] != 0.0))) && (scratch.values[2303] != 0.0)) {
            scratch.store_ad(2086, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2088))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2088))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2088))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if ((((scratch.values[2300] != 0.0) && (scratch.values[2301] != 0.0)) && (!(scratch.values[2302] != 0.0))) && (!(scratch.values[2303] != 0.0))) {
            scratch.store_ad(2086, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(AdValue::neg(scratch.ad_value(2088)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(AdValue::neg(scratch.ad_value(2088)), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(AdValue::neg(scratch.ad_value(2088)), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if ((scratch.values[2300] != 0.0) && (scratch.values[2301] != 0.0)) {
            scratch.store_ad(2020, &AdValue::mul(scratch.ad_value(248), AdValue::mul(scratch.ad_value(923), scratch.ad_value(2086))));
        }

        if ((scratch.values[2300] != 0.0) && (scratch.values[2301] != 0.0)) {
            scratch.store_ad(2019, &AdValue::mul(scratch.ad_value(2020), AdValue::add(scratch.ad_value(2018), scratch.ad_value(2069))));
        }

        scratch.values[2418] = if ((scratch.values[7] == 1.0) || (scratch.values[9] != 0.0)) { 1.0 } else { 0.0 };

        scratch.values[2419] = if (scratch.values[9] != 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2418] != 0.0) && (scratch.values[2419] != 0.0)) {
            scratch.store_ad(2086, &AdValue::add(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(852), scratch.ad_value(851)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::sub(scratch.ad_value(852), scratch.ad_value(851)), AdValue::sub(scratch.ad_value(852), scratch.ad_value(851))), scratch.ad_value(788)))), 0.5), scratch.ad_value(786)));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2419] != 0.0)) {
            scratch.store_ad(924, &AdValue::add(AdValue::sub(scratch.ad_value(851), AdValue::scale(AdValue::sub(scratch.ad_value(2086), AdValue::sqrt(AdValue::add(AdValue::mul(scratch.ad_value(2086), scratch.ad_value(2086)), scratch.ad_value(787)))), 0.5)), scratch.ad_value(789)));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2419] != 0.0)) {
            scratch.values[2306] = scratch.values[924];
            scratch.node_derivatives[2306] = scratch.node_derivatives[924];
            scratch.branch_derivatives[2306] = scratch.branch_derivatives[924];
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2419] != 0.0)) {
            scratch.values[2304] = scratch.values[784];
            scratch.node_derivatives[2304] = scratch.node_derivatives[784];
            scratch.branch_derivatives[2304] = scratch.branch_derivatives[784];
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2419] != 0.0)) {
            scratch.values[2305] = scratch.values[785];
            scratch.node_derivatives[2305] = scratch.node_derivatives[785];
            scratch.branch_derivatives[2305] = scratch.branch_derivatives[785];
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2419] != 0.0))) {
            scratch.values[2306] = scratch.values[870];
            scratch.node_derivatives[2306] = scratch.node_derivatives[870];
            scratch.branch_derivatives[2306] = scratch.branch_derivatives[870];
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2419] != 0.0))) {
            scratch.values[2304] = scratch.values[767];
            scratch.node_derivatives[2304] = scratch.node_derivatives[767];
            scratch.branch_derivatives[2304] = scratch.branch_derivatives[767];
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2419] != 0.0))) {
            scratch.values[2305] = scratch.values[768];
            scratch.node_derivatives[2305] = scratch.node_derivatives[768];
            scratch.branch_derivatives[2305] = scratch.branch_derivatives[768];
        }

    }

    pub(super) fn stamp_transient_block_35(
        &mut self,
        ctx: &GeneratedEvalContext<'_>,
        stamper: &mut GeneratedStamper<'_>,
        scratch: &mut Scratch,
    ) {
        let _ = stamper;
        if (scratch.values[2418] != 0.0) {
            scratch.values[2369] = 0.0;
            scratch.node_derivatives[2369] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2369] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2385] = 1.0;
            scratch.node_derivatives[2385] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2385] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2384] = 0.0;
            scratch.node_derivatives[2384] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2384] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2380] = 0.0;
            scratch.node_derivatives[2380] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2380] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2357] = 0.0;
            scratch.node_derivatives[2357] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2357] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2378] = 0.0;
            scratch.node_derivatives[2378] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2378] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2379] = 0.0;
            scratch.node_derivatives[2379] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2379] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2392] = 1.0;
            scratch.node_derivatives[2392] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2392] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2383] = 0.0;
            scratch.node_derivatives[2383] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2383] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2367] = 1.0;
            scratch.node_derivatives[2367] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2367] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2388] = 1.0;
            scratch.node_derivatives[2388] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2388] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2389] = 1.0;
            scratch.node_derivatives[2389] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2389] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2414] = 0.0;
            scratch.node_derivatives[2414] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2414] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2319] = 0.0;
            scratch.node_derivatives[2319] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2319] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2368] = 0.0;
            scratch.node_derivatives[2368] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2368] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2336] = 0.0;
            scratch.node_derivatives[2336] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2336] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2331] = 0.0;
            scratch.node_derivatives[2331] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2331] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2335] = 1.0;
            scratch.node_derivatives[2335] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2335] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2386] = 1.0;
            scratch.node_derivatives[2386] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2386] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2349] = 0.0;
            scratch.node_derivatives[2349] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2349] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2338] = 0.0;
            scratch.node_derivatives[2338] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2338] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2366] = 0.0;
            scratch.node_derivatives[2366] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2366] = [0.0; Instance::BRANCH_COUNT];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(861, &AdValue::add(scratch.ad_value(849), scratch.ad_value(2306)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(862, &AdValue::sub(scratch.ad_value(861), scratch.ad_value(751)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2307, &AdValue::add(scratch.ad_value(2306), AdValue::scale(AdValue::sub(scratch.ad_value(850), scratch.ad_value(860)), 0.5)));
        }

        scratch.values[2420] = if (scratch.values[217] < 1e-10) { 1.0 } else { 0.0 };

        if ((scratch.values[2418] != 0.0) && (scratch.values[2420] != 0.0)) {
            scratch.values[863] = scratch.values[860];
            scratch.node_derivatives[863] = scratch.node_derivatives[860];
            scratch.branch_derivatives[863] = scratch.branch_derivatives[860];
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2420] != 0.0))) {
            scratch.store_ad(863, &AdValue::div(AdValue::scale(AdValue::offset(AdValue::sqrt(AdValue::offset(AdValue::mul(scratch.ad_value(217), scratch.ad_value(860)), 1.0)), (-1.0)), 2.0), scratch.ad_value(217)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(864, &AdValue::mul(AdValue::mul(scratch.ad_value(216), scratch.ad_value(863)), AdValue::offset(AdValue::mul(scratch.ad_value(218), scratch.ad_value(2307)), 1.0)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2000, &AdValue::mul(AdValue::mul(scratch.ad_value(219), AdValue::offset(AdValue::mul(scratch.ad_value(221), scratch.ad_value(860)), 1.0)), AdValue::offset(AdValue::mul(scratch.ad_value(220), scratch.ad_value(2307)), 1.0)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(862, &AdValue::add(scratch.ad_value(862), scratch.ad_value(864)));
        }

        scratch.values[2421] = if (scratch.values[205] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2418] != 0.0) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2309, &AdValue::mul(AdValue::scale(scratch.ad_value(205), 0.5), AdValue::add(AdValue::sub(AdValue::add(scratch.ad_value(849), scratch.ad_value(851)), scratch.ad_value(203)), AdValue::sqrt(AdValue::add(AdValue::mul(AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(849), scratch.ad_value(851)), scratch.ad_value(203))), AdValue::neg(AdValue::sub(AdValue::add(scratch.ad_value(849), scratch.ad_value(851)), scratch.ad_value(203)))), scratch.ad_value(204))))));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2421] != 0.0)) {
            scratch.store_ad(2310, &AdValue::mul(scratch.ad_value(2305), AdValue::sqrt(AdValue::offset(scratch.ad_value(2309), 1.0))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2421] != 0.0))) {
            scratch.values[2310] = scratch.values[2305];
            scratch.node_derivatives[2310] = scratch.node_derivatives[2305];
            scratch.branch_derivatives[2310] = scratch.branch_derivatives[2305];
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2311, &AdValue::square(scratch.ad_value(2310)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2312, &AdValue::div_from_scalar(1.0, scratch.ad_value(2311)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.values[2006] = 1.0;
            scratch.node_derivatives[2006] = [0.0; Instance::NODE_COUNT];
            scratch.branch_derivatives[2006] = [0.0; Instance::BRANCH_COUNT];
        }

        scratch.values[2422] = if (scratch.values[210] > 0.0) { 1.0 } else { 0.0 };

        if ((scratch.values[2418] != 0.0) && (scratch.values[2422] != 0.0)) {
            scratch.store_ad(2001, &AdValue::mul(AdValue::scale(scratch.ad_value(862), 2.0), scratch.ad_value(370)));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2422] != 0.0)) {
            scratch.store_ad(2087, &AdValue::add(scratch.ad_value(2311), scratch.ad_value(2001)));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2422] != 0.0)) {
            scratch.store_ad(2088, &AdValue::scale(AdValue::add(AdValue::add(scratch.ad_value(2087), scratch.ad_value(2001)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::add(scratch.ad_value(2087), scratch.ad_value(2001)), AdValue::add(scratch.ad_value(2087), scratch.ad_value(2001))), 5.0))), 0.5));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2422] != 0.0)) {
            scratch.store_ad(2002, &AdValue::scale(AdValue::sub(scratch.ad_value(2087), AdValue::mul(scratch.ad_value(2310), AdValue::sqrt(scratch.ad_value(2088)))), 0.5));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2422] != 0.0)) {
            scratch.store_ad(2003, &AdValue::mul(scratch.ad_value(2304), scratch.ad_value(370)));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2422] != 0.0)) {
            scratch.store_ad(2004, &AdValue::mul(scratch.ad_value(2307), scratch.ad_value(370)));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2422] != 0.0)) {
            scratch.store_ad(2087, &AdValue::offset(AdValue::add(scratch.ad_value(2003), scratch.ad_value(2004)), 2.0));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2422] != 0.0)) {
            scratch.store_ad(2005, &AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2002), scratch.ad_value(2087)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2002), scratch.ad_value(2087)), AdValue::sub(scratch.ad_value(2002), scratch.ad_value(2087))), 5.0))), 0.5));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2422] != 0.0)) {
            scratch.store_ad(2088, &AdValue::mul(scratch.ad_value(753), AdValue::sub(scratch.ad_value(2005), AdValue::mul(AdValue::offset(scratch.ad_value(211), 1.0), AdValue::add(AdValue::scale(scratch.ad_value(2003), 0.5), scratch.ad_value(2004))))));
        }

        scratch.values[2423] = if (scratch.values[2088] > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (scratch.values[2422] != 0.0)) && (scratch.values[2423] != 0.0)) {
            scratch.store_ad(2006, &AdValue::exp(scratch.ad_value(2088)));
        }

        if (((scratch.values[2418] != 0.0) && (scratch.values[2422] != 0.0)) && (!(scratch.values[2423] != 0.0))) {
            scratch.store_ad(2006, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2088)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2088)), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), scratch.ad_value(2088)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2007, &AdValue::offset(AdValue::mul(scratch.ad_value(752), scratch.ad_value(2006)), 1.0));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2008, &AdValue::mul(AdValue::mul(scratch.ad_value(2074), scratch.ad_value(2007)), AdValue::offset(scratch.ad_value(2000), 1.0)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2009, &AdValue::div_from_scalar(1.0, scratch.ad_value(2008)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2308, &AdValue::mul(scratch.ad_value(862), scratch.ad_value(2009)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2313, &AdValue::offset(AdValue::scale(scratch.ad_value(2310), 0.7071067811865475), 1.0));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2314, &AdValue::div_from_scalar(1.0, scratch.ad_value(2313)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2315, &AdValue::mul(scratch.ad_value(2306), scratch.ad_value(2009)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2316, &AdValue::add(AdValue::mul(scratch.ad_value(2304), scratch.ad_value(2009)), scratch.ad_value(2315)));
        }

        scratch.values[2424] = if (scratch.values[2316] < 460.51701859880916) { 1.0 } else { 0.0 };

        if ((scratch.values[2418] != 0.0) && (scratch.values[2424] != 0.0)) {
            scratch.store_ad(2317, &AdValue::exp(AdValue::neg(scratch.ad_value(2316))));
        }

        if ((scratch.values[2418] != 0.0) && (!(scratch.values[2424] != 0.0))) {
            scratch.store_ad(2317, &AdValue::div_from_scalar(1e-200, AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2316), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2316), (-460.51701859880916)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2316), (-460.51701859880916)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (scratch.values[2418] != 0.0) {
            scratch.store_ad(2318, &AdValue::scale(scratch.ad_value(2313), 1e-5));
        }

        scratch.values[2425] = if (((scratch.values[2308]) as f64).abs() <= scratch.values[2318]) { 1.0 } else { 0.0 };

        if ((scratch.values[2418] != 0.0) && (scratch.values[2425] != 0.0)) {
            scratch.store_ad(2394, &AdValue::scale(AdValue::square(scratch.ad_value(2314)), (0.16666666666666666 * 0.7071067811865475)));
        }

        if ((scratch.values[2418] != 0.0) && (scratch.values[2425] != 0.0)) {
            scratch.store_ad(2319, &AdValue::mul(AdValue::mul(scratch.ad_value(2308), scratch.ad_value(2314)), AdValue::offset(AdValue::mul(AdValue::mul(AdValue::mul(scratch.ad_value(2308), AdValue::sub_from_scalar(1.0, scratch.ad_value(2317))), scratch.ad_value(2310)), scratch.ad_value(2394)), 1.0)));
        }

        scratch.values[2426] = if (scratch.values[2308] < (-scratch.values[2318])) { 1.0 } else { 0.0 };

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2396, &AdValue::neg(scratch.ad_value(2308)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2397, &AdValue::scale(AdValue::mul(scratch.ad_value(2396), scratch.ad_value(2314)), 1.25));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2398, &AdValue::scale(AdValue::sub(AdValue::offset(scratch.ad_value(2397), 10.0), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2397), (-6.0)), AdValue::offset(scratch.ad_value(2397), (-6.0))), 64.0))), 0.5));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2393, &AdValue::sub(scratch.ad_value(2396), scratch.ad_value(2398)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2399, &AdValue::add(AdValue::square(scratch.ad_value(2393)), AdValue::mul(scratch.ad_value(2311), AdValue::offset(scratch.ad_value(2398), 1.0))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2400, &AdValue::sub(AdValue::scale(scratch.ad_value(2393), 2.0), scratch.ad_value(2311)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2401, &AdValue::sub(AdValue::ln(AdValue::mul(scratch.ad_value(2399), scratch.ad_value(2312))), scratch.ad_value(2398)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(962, &AdValue::add(scratch.ad_value(2399), scratch.ad_value(2400)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(961, &AdValue::add(AdValue::square(scratch.ad_value(962)), AdValue::mul(scratch.ad_value(2401), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2400)), 0.5), scratch.ad_value(2399)))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2402, &AdValue::add(scratch.ad_value(2398), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2399), scratch.ad_value(962)), scratch.ad_value(2401)), AdValue::add(scratch.ad_value(961), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(962), scratch.ad_value(961)), scratch.ad_value(2401)), scratch.ad_value(2401)), scratch.ad_value(2400)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2400)), 0.3333333333333333), scratch.ad_value(2399)))))));
        }

        scratch.values[2427] = if (scratch.values[2402] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) && (scratch.values[2427] != 0.0)) {
            scratch.store_ad(2403, &AdValue::exp(scratch.ad_value(2402)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) && (!(scratch.values[2427] != 0.0))) {
            scratch.store_ad(2403, &AdValue::scale(AdValue::offset(AdValue::mul(AdValue::offset(scratch.ad_value(2402), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::offset(scratch.ad_value(2402), (-230.25850929940458)), AdValue::offset(AdValue::scale(AdValue::offset(scratch.ad_value(2402), (-230.25850929940458)), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0), 1e100));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2404, &AdValue::div_from_scalar(1.0, scratch.ad_value(2403)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2393, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2402)), 2.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2405, &AdValue::mul(AdValue::square(scratch.ad_value(2402)), scratch.ad_value(2393)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2406, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2402), scratch.ad_value(2393)), scratch.ad_value(2393)), 4.0));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2407, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2393), 8.0), AdValue::scale(scratch.ad_value(2405), 12.0)), scratch.ad_value(2393)), scratch.ad_value(2393)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2393, &AdValue::sub(scratch.ad_value(2396), scratch.ad_value(2402)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2394, &AdValue::mul(scratch.ad_value(2317), scratch.ad_value(2404)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2408, &AdValue::add(AdValue::scale(scratch.ad_value(2393), 2.0), AdValue::mul(scratch.ad_value(2311), AdValue::add(AdValue::sub(AdValue::offset(scratch.ad_value(2403), (-1.0)), scratch.ad_value(2394)), AdValue::mul(scratch.ad_value(2317), AdValue::sub_from_scalar(1.0, scratch.ad_value(2406)))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2409, &AdValue::sub(AdValue::square(scratch.ad_value(2393)), AdValue::mul(scratch.ad_value(2311), AdValue::add(AdValue::add(AdValue::offset(AdValue::sub(scratch.ad_value(2403), scratch.ad_value(2402)), (-1.0)), scratch.ad_value(2394)), AdValue::mul(scratch.ad_value(2317), AdValue::sub(AdValue::offset(scratch.ad_value(2402), (-1.0)), scratch.ad_value(2405)))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2393, &AdValue::sub_from_scalar(2.0, AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::add(scratch.ad_value(2403), scratch.ad_value(2394)), AdValue::mul(scratch.ad_value(2317), scratch.ad_value(2407))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2393, &AdValue::sub(AdValue::square(scratch.ad_value(2408)), AdValue::scale(AdValue::mul(scratch.ad_value(2409), scratch.ad_value(2393)), 2.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (scratch.values[2426] != 0.0)) {
            scratch.store_ad(2319, &AdValue::sub(AdValue::neg(scratch.ad_value(2402)), AdValue::scale(AdValue::div(scratch.ad_value(2409), AdValue::add(scratch.ad_value(2408), AdValue::sqrt(scratch.ad_value(2393)))), 2.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2410, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::scale(scratch.ad_value(2310), 0.7324648775608221), 1.25)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2411, &AdValue::mul(AdValue::offset(AdValue::mul(AdValue::scale(scratch.ad_value(2313), 1.25), scratch.ad_value(2410)), (-1.0)), scratch.ad_value(2410)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2412, &AdValue::mul(AdValue::mul(scratch.ad_value(2308), scratch.ad_value(2314)), AdValue::offset(AdValue::mul(scratch.ad_value(2411), scratch.ad_value(2308)), 1.0)));
        }

        scratch.values[2428] = if ((-scratch.values[2412]) > (-230.25850929940458)) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) && (scratch.values[2428] != 0.0)) {
            scratch.store_ad(2393, &AdValue::exp(AdValue::neg(scratch.ad_value(2412))));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) && (!(scratch.values[2428] != 0.0))) {
            scratch.store_ad(2393, &AdValue::div_from_scalar(1e-100, AdValue::offset(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2412))), AdValue::offset(AdValue::scale(AdValue::mul(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2412))), AdValue::offset(AdValue::scale(AdValue::sub_from_scalar((-230.25850929940458), AdValue::neg(scratch.ad_value(2412))), 0.3333333333333333), 1.0)), 0.5), 1.0)), 1.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2413, &AdValue::sub_from_scalar(1.0, scratch.ad_value(2393)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2414, &AdValue::sub(AdValue::add(scratch.ad_value(2308), AdValue::scale(scratch.ad_value(2311), 0.5)), AdValue::mul(scratch.ad_value(2310), AdValue::sqrt(AdValue::sub(AdValue::add(scratch.ad_value(2308), AdValue::scale(scratch.ad_value(2311), 0.25)), scratch.ad_value(2413))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2415, &AdValue::offset(scratch.ad_value(2316), 3.0));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2398, &AdValue::sub(AdValue::scale(AdValue::sub(AdValue::add(scratch.ad_value(2414), scratch.ad_value(2415)), AdValue::sqrt(AdValue::offset(AdValue::mul(AdValue::sub(scratch.ad_value(2414), scratch.ad_value(2415)), AdValue::sub(scratch.ad_value(2414), scratch.ad_value(2415))), 5.0))), 0.5), AdValue::scale(AdValue::sub(scratch.ad_value(2415), AdValue::sqrt(AdValue::offset(AdValue::square(scratch.ad_value(2415)), 5.0))), 0.5)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2393, &AdValue::sub(scratch.ad_value(2308), scratch.ad_value(2398)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2394, &AdValue::exp(AdValue::neg(scratch.ad_value(2398))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2395, &AdValue::div_from_scalar(1.0, AdValue::offset(AdValue::square(scratch.ad_value(2398)), 2.0)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2405, &AdValue::mul(AdValue::square(scratch.ad_value(2398)), scratch.ad_value(2395)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2406, &AdValue::scale(AdValue::mul(AdValue::mul(scratch.ad_value(2398), scratch.ad_value(2395)), scratch.ad_value(2395)), 4.0));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2407, &AdValue::mul(AdValue::mul(AdValue::sub(AdValue::scale(scratch.ad_value(2395), 8.0), AdValue::scale(scratch.ad_value(2405), 12.0)), scratch.ad_value(2395)), scratch.ad_value(2395)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2399, &AdValue::max_from_scalar(1e-40, AdValue::sub(AdValue::square(scratch.ad_value(2393)), AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::offset(AdValue::add(scratch.ad_value(2394), scratch.ad_value(2398)), (-1.0)), AdValue::mul(scratch.ad_value(2317), AdValue::add(AdValue::offset(scratch.ad_value(2398), 1.0), scratch.ad_value(2405))))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2416, &AdValue::sub_from_scalar(1.0, AdValue::scale(AdValue::mul(scratch.ad_value(2311), AdValue::sub(scratch.ad_value(2394), AdValue::mul(scratch.ad_value(2317), scratch.ad_value(2407)))), 0.5)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2400, &AdValue::add(AdValue::scale(scratch.ad_value(2393), 2.0), AdValue::mul(scratch.ad_value(2311), AdValue::sub(AdValue::sub_from_scalar(1.0, scratch.ad_value(2394)), AdValue::mul(scratch.ad_value(2317), AdValue::offset(scratch.ad_value(2406), 1.0))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(2401, &AdValue::add(AdValue::sub(scratch.ad_value(2316), scratch.ad_value(2398)), AdValue::ln(AdValue::div(scratch.ad_value(2399), scratch.ad_value(2311)))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(962, &AdValue::add(scratch.ad_value(2399), scratch.ad_value(2400)));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            scratch.store_ad(961, &AdValue::add(AdValue::square(scratch.ad_value(962)), AdValue::mul(scratch.ad_value(2401), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2400)), 0.5), AdValue::mul(scratch.ad_value(2399), scratch.ad_value(2416))))));
        }

        if (((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) {
            let assign48170_ad_e61921: AdValue = AdValue::add(scratch.ad_value(2398), AdValue::div(AdValue::mul(AdValue::mul(scratch.ad_value(2399), scratch.ad_value(962)), scratch.ad_value(2401)), AdValue::add(scratch.ad_value(961), AdValue::mul(AdValue::mul(AdValue::mul(AdValue::mul(AdValue::div(scratch.ad_value(962), scratch.ad_value(961)), scratch.ad_value(2401)), scratch.ad_value(2401)), scratch.ad_value(2400)), AdValue::sub(AdValue::scale(AdValue::square(scratch.ad_value(2400)), 0.3333333333333333), AdValue::mul(scratch.ad_value(2399), scratch.ad_value(2416)))))));
            scratch.store_ad(2417, &assign48170_ad_e61921);
        }

        scratch.values[2429] = if (scratch.values[2417] < 230.25850929940458) { 1.0 } else { 0.0 };

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) && (scratch.values[2429] != 0.0)) {
            scratch.store_ad(2403, &AdValue::exp(scratch.ad_value(2417)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) && (scratch.values[2429] != 0.0)) {
            scratch.store_ad(2404, &AdValue::div_from_scalar(1.0, scratch.ad_value(2403)));
        }

        if ((((scratch.values[2418] != 0.0) && (!(scratch.values[2425] != 0.0))) && (!(scratch.values[2426] != 0.0))) && (scratch.values[2429] != 0.0)) {
            scratch.store_ad(2403, &AdValue::mul(scratch.ad_value(2317), scratch.ad_value(2403)));
        }

        scratch.values[2430] = if (scratch.values[2417] > (scratch.values[2316] - 230.25850929940458)) { 1.0 } else { 0.0 };

    }
}
