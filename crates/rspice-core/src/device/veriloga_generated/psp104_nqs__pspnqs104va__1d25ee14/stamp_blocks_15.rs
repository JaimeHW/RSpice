#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, ReactiveScratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_reactive_block_30(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2279] && s.b[2283]) {s.store_mul3_lhs(2167, 2162, 2114, 2129);}
        s.b[2284] = (s.v[217] < 0.0);s.store_scalar(2284, if s.b[2284] { 1.0 } else { 0.0 });
        if ((s.b[2279] && s.b[2283]) && s.b[2284]) {s.store_div_from_scalar_sub_from_scalar_ad(2168, 1.0, 1.0, A::mul(s.ad_value(217), s.ad_value(2113)));}
        if ((s.b[2279] && s.b[2283]) && (!s.b[2284])) {s.store_offset_mul(2168, 217, 2113, 1.0);}
        s.b[2285] = (s.v[218] < 0.0);s.store_scalar(2285, if s.b[2285] { 1.0 } else { 0.0 });
        if ((s.b[2279] && s.b[2283]) && s.b[2285]) {s.store_sub_from_scalar_scaled_mul(2169, 1.0, 218, 2166, 1.0);}
        if ((s.b[2279] && s.b[2283]) && (!s.b[2285])) {s.store_div_from_scalar_offset_product(2169, 1.0, 218, 2166, 1.0);}
        if (s.b[2279] && s.b[2283]) {s.store_mul_product3_indices(2170, 2166, 757, 2168, 2169, 1.0);s.store_mul_add_scaled_product_rhs_indices(2171, 774, 2167, 1.0, 775, 2166, 1.0);s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2161), 1.0, A::add(s.ad_value(2161), s.ad_value(2160)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2172, A::pow(A::mul(s.ad_value(2171), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);s.store_mul_add_mixed_iai(2173, 2164, A::offset(s.ad_value(2172), 1.0), 2170);}
        s.b[2286] = (s.v[221] < 0.0);s.store_scalar(2286, if s.b[2286] { 1.0 } else { 0.0 });
        if ((s.b[2279] && s.b[2283]) && s.b[2286]) {s.store_div_from_scalar_sub_from_scalar_ad(2174, 1.0, 1.0, A::mul(s.ad_value(221), s.ad_value(2113)));}
        if ((s.b[2279] && s.b[2283]) && (!s.b[2286])) {s.store_offset_mul(2174, 221, 2113, 1.0);}
        if (s.b[2279] && s.b[2283]) {s.store_mul(2029, 2166, 2174);s.store_div_add_scaled_inputs_rhs_indices(2175, 2029, 223, 1.0, 2029, 1.0);}
        s.b[2287] = (s.v[222] < 0.0);s.store_scalar(2287, if s.b[2287] { 1.0 } else { 0.0 });
        if ((s.b[2279] && s.b[2283]) && s.b[2287]) {s.store_div_from_scalar_sub_from_scalar_ad(2176, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2175)));}
        if ((s.b[2279] && s.b[2283]) && (!s.b[2287])) {s.store_offset_mul(2176, 222, 2175, 1.0);}
        s.copy_ad(1822, 2111);s.copy_ad(1823, 2113);s.copy_ad(1824, 2129);s.copy_ad(1825, 2130);s.copy_ad(1826, 2114);s.copy_ad(1827, 2115);s.copy_ad(1828, 2131);s.copy_ad(1829, 2133);s.copy_ad(1830, 2138);s.copy_ad(1831, 2139);s.copy_ad(1832, 2150);s.copy_ad(1833, 2151);s.copy_ad(1834, 2152);s.copy_ad(1835, 2259);s.copy_ad(1836, 2154);s.copy_ad(1837, 2153);s.copy_ad(1838, 2156);s.copy_ad(1839, 2157);s.copy_ad(1840, 2158);s.copy_ad(1841, 2159);s.copy_ad(1842, 2161);s.copy_ad(1843, 2160);s.copy_ad(1844, 2162);s.copy_ad(1845, 2163);s.copy_ad(1846, 2164);s.copy_ad(1847, 2165);s.copy_ad(1848, 2166);s.copy_ad(1849, 2167);s.copy_ad(1850, 2168);s.copy_ad(1851, 2169);s.copy_ad(1852, 2173);s.copy_ad(1853, 2174);s.copy_ad(1854, 2176);s.store_scalar(2178, 0.0);s.store_scale(2177, 2129, 4.60517018598809);s.copy_ad(2194, 2177);s.copy_ad(2195, 826);s.store_mul(2196, 826, 2130);s.copy_ad(2200, 2153);s.store_scalar(2201, 0.0);s.store_scalar(2204, 0.0);s.copy_ad(2206, 2159);s.copy_ad(2207, 2161);s.copy_ad(2209, 2160);s.copy_ad(2210, 2167);s.copy_ad(2211, 2153);s.copy_ad(2212, 2159);
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_31(
        s: &mut ReactiveScratch,
    ) {
        s.copy_ad(2214, 2160);s.copy_ad(2215, 2161);s.store_sub(2216, 2133, 2153);s.store_scalar(2217, 1.0);s.store_scalar(2219, 1.0);s.store_scalar(2218, 0.0);s.copy_ad(2228, 2166);s.store_mul(2232, 2216, 2129);s.store_scalar(2229, 0.0);s.copy_ad(2230, 2167);s.store_scalar(2235, 0.0);s.store_scalar(2234, 1.0);s.copy_ad(2237, 2109);s.copy_ad(2236, 2232);s.b[2288] = (s.v[2133] > 0.0);s.store_scalar(2288, if s.b[2288] { 1.0 } else { 0.0 });s.b[2289] = (s.v[2160] > 1e-100);s.store_scalar(2289, if s.b[2289] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2289]) {s.store_mul(2237, 2109, 2176);s.store_div(2178, 2237, 2173);s.store_add_scaled_inputs(2179, 2165, 1.0, 2115, 0.5);s.store_div_scaled_product_by_product_indices(2027, 2115, 2158, 1.0, 2179, 2179, 1.0);}
        s.b[2290] = (s.v[2027] > 0.0001);s.store_scalar(2290, if s.b[2290] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2289]) && s.b[2290]) {s.store_sub_from_scalar(2028, 1.0, 2027);}
        s.b[2291] = (s.v[2028] < 1e-10);s.store_scalar(2291, if s.b[2291] { 1.0 } else { 0.0 });
        if (((s.b[2288] && s.b[2289]) && s.b[2290]) && s.b[2291]) {s.store_scalar(2029, 1.0);}
        if (((s.b[2288] && s.b[2289]) && s.b[2290]) && (!s.b[2291])) {s.store_sub_from_scalar_ad(2029, 1.0, A::sqrt(s.ad_value(2028)));}
        if ((s.b[2288] && s.b[2289]) && (!s.b[2290])) {s.store_scale(2029, 2027, 0.5);}
        if (s.b[2288] && s.b[2289]) {s.store_mul(2180, 2029, 2179);}
        s.b[2292] = ((s.v[706] > 0.0) && (s.v[707] > 0.0));s.store_scalar(2292, if s.b[2292] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {s.store_scaled_mul(2181, 2129, 2180, 0.475);s.store_add_scaled_product_indices(2027, 2166, 1.0, 2163, 2181, (-1.0));s.store_scaled_add_mixed_ia(2182, 2027, A::sqrt_square_offset(s.ad_value(2027), 1e-12), 0.5);s.store_add_scaled_value_products_mixed_iiiai(2183, 2166, (-1.0), 2129, 2165, 1.0, A::offset(s.ad_value(2163), (-1.0)), 2181, 1.0);s.store_offset_div_scaled_product_indices(2184, 2115, 2129, 0.5, 2183, 1.0, 1.0);s.store_add_scaled_product_indices(2027, 2183, 1.0, 775, 2182, 1.0);s.store_pow_ad(2185, A::mul3(s.ad_value(774), s.ad_value(2027), s.ad_value(704)), s.ad_value(705));s.store_mul_mixed_ai(2028, A::div_scaled_product_offset_rhs(s.ad_value(705), A::mul_sub_from_scalar_rhs(s.ad_value(2184), 1.0, s.ad_value(775)), (-1.0), 1.0, s.ad_value(2027), 1.0), 2185);s.store_div(2027, 2182, 2183);s.store_mul_pow_mixed_iaa(2186, 706, A::offset(s.ad_value(2027), 1.0), A::neg(s.ad_value(707)));s.store_mul_div_scaled_product_mixed_iiai(2029, 2186, 707, A::add(A::offset(s.ad_value(2184), (-1.0)), A::div_scalar_offset_denominator(1.0, s.ad_value(2027), 1.0, 1.0)), 1.0, 2183, 1.0);s.store_mul_product3_indices(2187, 2182, 757, 2168, 2169, 1.0);s.store_offset_ad(2027, A::div_scaled_add_product(s.ad_value(2028), 1.0, A::mul3(s.ad_value(757), s.ad_value(2168), s.ad_value(2169)), s.ad_value(2184), (-1.0), s.ad_value(2029), 1.0), 1.0);}
        s.b[2293] = (s.v[2027] < 230.25850929940458);s.store_scalar(2293, if s.b[2293] { 1.0 } else { 0.0 });
        if (((s.b[2288] && s.b[2289]) && s.b[2292]) && s.b[2293]) {s.store_scaled_ln_one_plus_exp_scaled_input(2028, 2027, 2.0, 0.5);}
        if (((s.b[2288] && s.b[2289]) && s.b[2292]) && (!s.b[2293])) {s.copy_ad(2028, 2027);}
        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {s.store_div_scaled_product3_mixed_iiia(2188, 2181, 2029, 2028, -1.0, A::add_scaled_inputs3_offset(s.ad_value(2185), 1.0, s.ad_value(2186), 1.0, s.ad_value(2187), 1.0, 1.0), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_32(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2288] && s.b[2289]) && s.b[2292]) {s.store_mul_scale_offset_mixed_ia(2189, 2180, A::div_scaled_value_offset_denominator(s.ad_value(2188), 1.0, A::sqrt_square_offset(s.ad_value(2188), 1.0), 1.0, 1.0), 1.0, 1.0);}
        if ((s.b[2288] && s.b[2289]) && (!s.b[2292])) {s.copy_ad(2189, 2180);}
        if (s.b[2288] && s.b[2289]) {s.store_mul3_affine_lhs(2190, 2129, 2178, 0.7071067811865475, 0.0, 2189);}
        s.b[2294] = (s.v[0] == (-1.0));s.store_scalar(2294, if s.b[2294] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2289]) && s.b[2294]) {s.store_div_mixed_ia(2190, 2190, A::sqrt(A::offset(s.ad_value(2190), 1.0)));}
        if (s.b[2288] && s.b[2289]) {s.store_div_from_scalar_offset_ad(2191, 2.0, A::sqrt(A::scale_offset(s.ad_value(2190), 4.0, 1.0)), 1.0);s.store_mul(2027, 2191, 2190);s.store_mul_ad_product_rhs_mixed_ia(2192, 2189, 2191, A::offset(A::div(A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2027), 1.0, A::mul(s.ad_value(2027), s.ad_value(2191)), 0.86), A::offset(A::mul3_scaled_output(s.ad_value(2027), s.ad_value(2027), s.ad_value(2191), 4.0), 1.0)), 1.0));s.store_scale(2193, 2192, 0.99);s.store_div_scaled_product3_mixed_iaii(2027, 2193, A::sub_scaled_inputs(s.ad_value(2193), 1.0, s.ad_value(2179), 2.0), 2131, 1.0, 2160, 1.0);}
        if (s.b[2288] && s.b[2289]) {
            s.store_mul_sub_mixed_iia(2194, 2129, 2193, A::ln(A::offset({
                if (s.v[2027] > (-0.99)) {
                    s.ad_value(2027)
                } else {
                    A::neg(A::constant(0.99))
                }
            }, 1.0)));
        }
        if (s.b[2288] && (!s.b[2289])) {s.copy_ad(2194, 2177);}
        if s.b[2288] {s.store_offset(2027, 2110, 1.0);s.store_div_scaled_product_mixed_aii(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 2194, 1.0);s.store_add_mixed_ai(2029, A::square(s.ad_value(2028)), 2027);s.store_scale(2027, 2028, 2.0);s.store_div_scaled_product_add_scaled_denominator(2195, 2194, 2027, 1.0, A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), 1.0, A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027))), 1.0, 1.0);s.store_mul(2196, 2195, 2130);s.store_add(2197, 2139, 2196);}
        s.b[2295] = (s.v[2196] < 460.51701859880916);s.store_scalar(2295, if s.b[2295] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2295]) {s.store_exp_neg_input(2198, 2196);}
        if (s.b[2288] && (!s.b[2295])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2198, 1e-200, 2196, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if s.b[2288] {s.store_mul(2199, 2154, 2198);}
        s.b[2296] = (((s.v[2133]) as f64).abs() <= s.v[2151]);s.store_scalar(2296, if s.b[2296] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2296]) {s.store_scaled_square(2239, 2152, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2200, 2133, 2152, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2133), 1.0, s.ad_value(2199)), s.ad_value(2114), s.ad_value(2239)), 1.0));}
        if (s.b[2288] && (!s.b[2296])) {s.store_offset(2260, 2197, 3.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_33(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2288] && (!s.b[2296])) {s.store_sub_ad(2243, A::add_scaled_inputs3(s.ad_value(2259), 0.5, s.ad_value(2260), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2259), s.ad_value(2260)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2260), 0.5, A::sqrt_square_offset(s.ad_value(2260), 5.0), 0.5));s.store_sub(2238, 2133, 2243);s.store_exp_neg_input(2239, 2243);s.store_div_from_scalar_offset_square(2240, 1.0, 2243, 2.0);s.store_mul_square_lhs(2250, 2243, 2240);s.store_mul3_affine_lhs(2251, 2243, 2240, 4.0, 0.0, 2240);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2240), 8.0, s.ad_value(2250), 12.0), 2240, 2240);}
        if (s.b[2288] && (!s.b[2296])) {
            if (1e-40 > ((s.v[2238] * s.v[2238]) - (s.v[2115] * (((s.v[2239] + s.v[2243]) - 1.0) - (s.v[2199] * ((s.v[2243] + 1.0) + s.v[2250])))))) {
                s.store_scalar(2244, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2244, 2238, 1.0, 2115, A::add_scaled_product(A::offset(A::add(s.ad_value(2239), s.ad_value(2243)), (-1.0)), 1.0, s.ad_value(2199), A::add(A::offset(s.ad_value(2243), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));
            }
        }
        if (s.b[2288] && (!s.b[2296])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2261, 1.0, 2115, A::add_scaled_product(s.ad_value(2239), 1.0, s.ad_value(2199), s.ad_value(2252), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2245, 2238, 2.0, 2115, A::add_scaled_sub_value_product(1.0, s.ad_value(2239), 1.0, s.ad_value(2199), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2246, 2197, 1.0, 2243, (-1.0), A::ln(A::div(s.ad_value(2244), s.ad_value(2115))), 1.0);s.store_add(824, 2244, 2245);s.store_add_scaled_square_product_mixed_iia(823, 824, 1.0, 2246, A::add_scaled_square_product(s.ad_value(2245), 0.5, s.ad_value(2244), s.ad_value(2261), (-1.0)), 1.0);s.store_add_mixed_ia(2262, 2243, A::div_scaled_product3(s.ad_value(2244), s.ad_value(824), s.ad_value(2246), 1.0, A::add(s.ad_value(823), A::mul3(A::mul3(A::div(s.ad_value(824), s.ad_value(823)), s.ad_value(2246), s.ad_value(2246)), s.ad_value(2245), A::add_scaled_square_product(s.ad_value(2245), 0.3333333333333333, s.ad_value(2244), s.ad_value(2261), (-1.0)))), 1.0));}
        s.b[2297] = (s.v[2262] < 230.25850929940458);s.store_scalar(2297, if s.b[2297] { 1.0 } else { 0.0 });
        if ((s.b[2288] && (!s.b[2296])) && s.b[2297]) {s.store_exp(2248, 2262);s.store_div_from_scalar(2249, 1.0, 2248);s.store_mul(2248, 2199, 2248);}
        s.b[2298] = (s.v[2262] > (s.v[2197] - 230.25850929940458));s.store_scalar(2298, if s.b[2298] { 1.0 } else { 0.0 });
        if (((s.b[2288] && (!s.b[2296])) && (!s.b[2297])) && s.b[2298]) {s.store_exp_sub(2248, 2262, 2197);s.store_div(2249, 2199, 2248);}
        if (((s.b[2288] && (!s.b[2296])) && (!s.b[2297])) && (!s.b[2298])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2248, 1e-100, A::sub(s.ad_value(2197), s.ad_value(2262)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2249, 1e-100, 2262, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (s.b[2288] && (!s.b[2296])) {s.store_div_from_scalar_offset_square(2238, 1.0, 2262, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_34(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2288] && (!s.b[2296])) {s.store_mul_square_lhs(2250, 2262, 2238);s.store_mul3_affine_lhs(2251, 2262, 2238, 4.0, 0.0, 2238);s.store_mul_ad_product_lhs_mixed_ai(2252, A::sub_scaled_inputs(s.ad_value(2238), 8.0, s.ad_value(2250), 12.0), 2238, 2238);s.store_sub(2238, 2133, 2262);s.store_add_scaled_product_mixed_iia(2253, 2238, 2.0, 2115, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2249)), 1.0, s.ad_value(2248), 1.0, s.ad_value(2199), A::offset(s.ad_value(2251), 1.0), (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2254, 2238, 1.0, 2115, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2249), 1.0, s.ad_value(2262), 1.0, s.ad_value(2248), 1.0, (-1.0)), 1.0, s.ad_value(2199), A::add(A::offset(s.ad_value(2262), 1.0), s.ad_value(2250)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2238, 2.0, 2115, A::add_scaled_inputs_product(s.ad_value(2249), 1.0, s.ad_value(2248), 1.0, s.ad_value(2199), s.ad_value(2252), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2238, 2253, 1.0, 2254, 2238, (-2.0));s.store_add_scaled_inputs_mixed_ia(2200, 2262, 1.0, A::div(s.ad_value(2254), A::add(s.ad_value(2253), A::sqrt(s.ad_value(2238)))), 2.0);}
        if s.b[2288] {s.store_sub(2201, 2200, 2153);}
        s.b[2299] = (s.v[2201] < 1e-10);s.store_scalar(2299, if s.b[2299] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2299]) {s.store_add_scaled_inputs_product_mixed_iiia(2202, 2133, 2.0, 2153, (-2.0), 2115, A::add_scaled_offset_product_rhs(A::add_scaled_sub_value_product(1.0, s.ad_value(2159), 1.0, s.ad_value(2158), s.ad_value(2198), 1.0), 1.0, s.ad_value(2199), s.ad_value(2156), 1.0, (-1.0)), 1.0);s.store_mul_mixed_ai(2203, A::mul_sub_from_scalar_rhs(s.ad_value(2115), 1.0, s.ad_value(2198)), 2160);s.store_sub_from_scalar_scaled_mul_mixed_ia(2027, 2.0, 2115, A::add_scaled_value_products(s.ad_value(2159), 1.0, s.ad_value(2158), s.ad_value(2198), 1.0, s.ad_value(2199), s.ad_value(2157), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2027, 2202, 1.0, 2027, 2203, (-2.0));s.store_scaled_div_mixed_ia(2201, 2203, A::add(s.ad_value(2202), A::sqrt(s.ad_value(2027))), 2.0);s.store_add(2200, 2153, 2201);}
        if s.b[2288] {s.store_mul(2204, 2201, 2129);s.store_div_scaled_product_offset_denominator_mixed_iia(2205, 2200, 2200, 1.0, A::square(s.ad_value(2200)), 2.0, 1.0);}
        s.b[2300] = (s.v[2200] < 230.25850929940458);s.store_scalar(2300, if s.b[2300] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2300]) {s.store_exp_neg_input(2206, 2200);}
        s.b[2301] = (s.v[2200] < 1e-5);s.store_scalar(2301, if s.b[2301] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2300]) && s.b[2301]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2207, 2200, 1.0, 2200, 1.0, 2200, 0.25, 0.3333333333333333, 0.5);s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2200), 1.0, A::scale(s.ad_value(2200), 0.25), 0.3333333333333333));s.store_scaled_mul(2208, 2200, 2027, 0.7071067811865475);s.store_mul3_ad_middle(2209, A::mul3_scaled_output(s.ad_value(2199), s.ad_value(2200), s.ad_value(2200), 0.16666666666666666), 2200, A::scale_offset(s.ad_value(2200), 1.75, 1.0));}
        if ((s.b[2288] && s.b[2300]) && (!s.b[2301])) {s.store_add_offset_lhs(2207, 2200, (-1.0), 2206);s.store_sqrt(2208, 2207);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_35(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2288] && s.b[2300]) && (!s.b[2301])) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(2209, 2199, A::div_from_scalar(1.0, s.ad_value(2206)), 1.0, 2200, (-1.0), 2205, -1.0, (-1.0));}
        s.b[2302] = (s.v[2200] > (s.v[2197] - 230.25850929940458));s.store_scalar(2302, if s.b[2302] { 1.0 } else { 0.0 });
        if ((s.b[2288] && (!s.b[2300])) && s.b[2302]) {s.store_exp_sub(2027, 2200, 2197);s.store_div(2206, 2199, 2027);s.store_add_scaled_product_mixed_iia(2209, 2027, 1.0, 2199, A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205)), (-1.0));}
        if ((s.b[2288] && (!s.b[2300])) && (!s.b[2302])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2206, 1e-100, 2200, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2027, 1e-100, A::sub(s.ad_value(2197), s.ad_value(2200)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(2209, 2027, 1.0, 2199, A::add(A::offset(s.ad_value(2200), 1.0), s.ad_value(2205)), (-1.0));}
        if (s.b[2288] && (!s.b[2300])) {s.store_add_offset_lhs(2207, 2200, (-1.0), 2206);s.store_sqrt(2208, 2207);}
        if s.b[2288] {s.store_mul3_lhs(2210, 2208, 2114, 2129);s.store_scaled_add(2211, 2153, 2200, 0.5);s.store_scalar(2212, 0.0);s.store_mul(2027, 2206, 2159);}
        s.b[2303] = (s.v[2027] > 0.0);s.store_scalar(2303, if s.b[2303] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2303]) {s.store_sqrt(2212, 2027);}
        if s.b[2288] {s.store_scaled_add(2213, 2160, 2209, 0.5);s.store_add_scaled_product_mixed_iaa(2214, 2213, 1.0, A::square(s.ad_value(2201)), A::sub_scaled_inputs(s.ad_value(2212), 1.0, s.ad_value(2131), 2.0), 0.125);}
        s.b[2304] = (s.v[2211] < 1e-5);s.store_scalar(2304, if s.b[2304] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2304]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2215, 2211, 1.0, 2211, 1.0, 2211, 0.25, 0.3333333333333333, 0.5);s.store_mul_sqrt_mixed_ia(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));}
        s.b[2305] = (s.v[730] > 0.0);s.store_scalar(2305, if s.b[2305] { 1.0 } else { 0.0 });
        if ((s.b[2288] && s.b[2304]) && s.b[2305]) {s.store_div_from_scalar_sqrt_ad(2217, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2216)), 1.0));}
        if (s.b[2288] && s.b[2304]) {s.store_sqrt_sub_from_scalar_ad(2027, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2211), 1.0, A::scale(s.ad_value(2211), 0.25), 0.3333333333333333));s.store_scaled_mul(2218, 2211, 2027, 0.7071067811865475);s.store_add_mixed_ia(2219, 2217, A::div_scaled_product(s.ad_value(2114), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2211), 0.5)), 1.0, A::square(s.ad_value(2211)), 0.16666666666666666), 0.7071067811865475, s.ad_value(2027), 1.0));}
        if (s.b[2288] && (!s.b[2304])) {s.store_add_offset_lhs(2215, 2211, (-1.0), 2212);s.store_mul_sqrt_mixed_ia(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));}
        s.b[2306] = (s.v[730] > 0.0);s.store_scalar(2306, if s.b[2306] { 1.0 } else { 0.0 });
        if ((s.b[2288] && (!s.b[2304])) && s.b[2306]) {s.store_add_scaled_sub_value_product_indices(2220, 1.0, 2212, 1.0, 2216, 2131, 2.0);s.store_div_from_scalar_sqrt_ad(2217, 1.0, A::offset(A::mul(s.ad_value(730), s.ad_value(2216)), 1.0));s.store_div_scaled_value_offset_denominator(2027, s.ad_value(2217), 1.0, s.ad_value(2217), 1.0, 1.0);s.store_mul_product3_mixed_iaii(2221, 730, A::square(s.ad_value(2027)), 2115, 2214, 1.0);s.store_add_scaled_inputs_product_mixed_iiia(2222, 2216, 2.0, 2221, (-2.0), 2115, A::add(A::sub_from_scalar(1.0, s.ad_value(2212)), s.ad_value(2214)), 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(2223, 2221, 2221, 1.0, 2216, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_36(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2288] && (!s.b[2304])) && s.b[2306]) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2224, 1.0, 2115, A::add(s.ad_value(2212), s.ad_value(2214)), 0.5);s.store_div_scaled_product_mixed_iia(2225, 2223, 2222, 1.0, A::add_scaled_square_product(s.ad_value(2222), 1.0, s.ad_value(2224), s.ad_value(2223), (-1.0)), 1.0);s.store_add(2211, 2211, 2225);s.store_exp(2226, 2225);s.store_div(2212, 2212, 2226);s.store_mul(2214, 2214, 2226);s.store_add_offset_lhs(2215, 2211, (-1.0), 2212);s.store_mul_sqrt_mixed_ia(2216, 2114, A::add(s.ad_value(2214), s.ad_value(2215)));s.store_add_ad(2227, A::sub_from_scalar(1.0, s.ad_value(2212)), A::mul3_scaled_output(s.ad_value(2216), s.ad_value(2217), s.ad_value(2131), 2.0));s.store_div_scaled_product3_mixed_iiaa(2201, 2201, 2226, A::add(s.ad_value(2220), s.ad_value(2213)), 1.0, A::add_scaled_product(s.ad_value(2227), 1.0, s.ad_value(2226), s.ad_value(2213), 1.0), 1.0);s.store_mul(2204, 2201, 2129);}
        if (s.b[2288] && (!s.b[2304])) {s.store_sqrt(2218, 2215);s.store_add_scaled_inputs_mixed_ia(2219, 2217, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2114), 1.0, s.ad_value(2212)), s.ad_value(2218)), 0.5);}
        if s.b[2288] {s.store_mul_div_scaled_product_mixed_iiia(2228, 2129, 2115, 2214, 1.0, A::add_scaled_product(s.ad_value(2216), 1.0, s.ad_value(2114), s.ad_value(2218), 1.0), 1.0);s.store_add_scaled_product_indices(2229, 2228, 1.0, 2129, 2219, 1.0);s.store_mul3_lhs(2230, 2218, 2114, 2129);}
        s.b[2307] = (s.v[218] < 0.0);s.store_scalar(2307, if s.b[2307] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2307]) {s.store_sub_from_scalar_scaled_mul(2169, 1.0, 218, 2228, 1.0);}
        if (s.b[2288] && (!s.b[2307])) {s.store_div_from_scalar_offset_product(2169, 1.0, 218, 2228, 1.0);}
        if s.b[2288] {s.store_mul_product3_indices(2170, 2228, 757, 2168, 2169, 1.0);s.store_add_scaled_product_indices(2231, 2230, 1.0, 775, 2228, 1.0);s.store_add_scaled_product_indices(2232, 2230, 1.0, 776, 2228, 1.0);s.store_mul(2233, 774, 2231);s.store_ln_ad(2028, A::div_scaled_value_offset_denominator(s.ad_value(2215), 1.0, A::add(s.ad_value(2215), s.ad_value(2214)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2172, A::pow(A::mul(s.ad_value(2233), s.ad_value(704)), s.ad_value(705)), 1.0, 706, A::exp(A::mul_scaled_lhs(s.ad_value(707), 0.5, s.ad_value(2028))), 1.0);s.store_mul_add_mixed_iai(2234, 2164, A::offset(s.ad_value(2172), 1.0), 2170);s.store_ln_ad(2235, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(826), s.ad_value(2204)), s.ad_value(779)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2195), s.ad_value(2204)), s.ad_value(779)), 1.0), 1.0));s.store_mul(2029, 2228, 2174);s.store_div_add_scaled_inputs_rhs_indices(2175, 2029, 223, 1.0, 2029, 1.0);}
        s.b[2308] = (s.v[222] < 0.0);s.store_scalar(2308, if s.b[2308] { 1.0 } else { 0.0 });
        if (s.b[2288] && s.b[2308]) {s.store_div_from_scalar_sub_from_scalar_ad(2176, 1.0, 1.0, A::mul(s.ad_value(222), s.ad_value(2175)));}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_37(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (s.b[2288] && (!s.b[2308])) {s.store_offset_mul(2176, 222, 2175, 1.0);}
        if s.b[2288] {s.store_mul(2237, 2109, 2176);s.store_mul(2236, 2216, 2129);}
        s.copy_ad(1855, 2177);s.copy_ad(1857, 2195);s.copy_ad(1858, 2196);s.copy_ad(1859, 2201);s.copy_ad(1860, 2204);s.copy_ad(1862, 2211);s.copy_ad(1861, 2210);s.copy_ad(1863, 2217);s.copy_ad(1864, 2219);s.copy_ad(1865, 2228);s.copy_ad(1866, 2229);s.copy_ad(1867, 2230);s.copy_ad(1868, 2232);s.copy_ad(1869, 2234);s.copy_ad(1871, 2235);s.copy_ad(1870, 2237);s.copy_ad(1872, 2236);s.copy_ad(1931, 2216);s.store_scalar(1873, 1.0);s.store_scalar(1874, 1.0);s.store_scalar(1876, 1.0);s.store_scalar(1877, 1.0);s.store_scalar(838, 0.0);s.b[2309] = (s.v[1829] > 0.0);s.store_scalar(2309, if s.b[2309] { 1.0 } else { 0.0 });
        if s.b[2309] {s.store_ln_ad(2037, A::offset(A::mul(s.ad_value(830), s.ad_value(779)), 1.0));s.store_div_scaled_product_indices(2027, 1824, 1864, 1.0, 1866, 1.0);s.store_add_scaled_product_mixed_aai(2036, A::mul3(A::mul3(s.ad_value(227), s.ad_value(1867), s.ad_value(2027)), s.ad_value(2027), s.ad_value(2037)), 1.0, A::div_scaled_product(A::add(s.ad_value(225), A::div(s.ad_value(226), s.ad_value(1866))), s.ad_value(1865), 1.0, s.ad_value(1866), 1.0), 1871, 1.0);s.store_div_from_scalar_add_ad(1873, 1.0, A::offset(s.ad_value(2036), 1.0), A::square(s.ad_value(2036)));s.store_mul(1874, 1869, 1873);s.store_div(1875, 1870, 1874);s.store_mul_ad_product_lhs_mixed_ai(2038, A::square(s.ad_value(1875)), 1860, 1860);}
        s.b[2310] = (s.v[0] == (-1.0));s.store_scalar(2310, if s.b[2310] { 1.0 } else { 0.0 });
        if (s.b[2309] && s.b[2310]) {s.store_div_scaled_value_offset_denominator(2038, s.ad_value(2038), 1.0, A::mul(s.ad_value(1875), s.ad_value(1860)), 1.0, 1.0);}
        if s.b[2309] {s.store_mul_scale_offset_mixed_ia(2039, 1874, A::sqrt(A::scale_offset(s.ad_value(2038), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div_from_scalar(1876, 1.0, 2039);s.store_mul(2027, 1874, 1876);s.store_mul_scale_offset_mixed_ia(2040, 1864, A::mul3_scaled_output(s.ad_value(2038), s.ad_value(2027), s.ad_value(2027), 0.5), 1.0, 1.0);s.store_div_scaled_product_indices(1877, 2027, 1866, 1.0, 2040, 1.0);s.store_mul_product3_indices(838, 1876, 716, 1866, 1860, 1.0);}
        s.store_scalar(2042, 0.0);s.store_scalar(2043, 0.0);s.store_scalar(1878, 0.0);s.store_scalar(1879, 0.0);s.b[2311] = (((((p.p40 != 0.0) && ((s.v[237] > 0.0) || (s.v[238] > 0.0))) || ((p.p42 != 0.0) && ((s.v[247] > 0.0) || (s.v[248] > 0.0)))) || (s.v[262] > 0.0)) || (s.v[263] > 0.0));s.store_scalar(2311, if s.b[2311] { 1.0 } else { 0.0 });
        if s.b[2311] {s.store_scaled_add_mixed_ia(2041, 1817, A::sqrt(A::add(A::square(s.ad_value(1817)), s.ad_value(789))), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_38(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if s.b[2311] {s.store_add_mixed_ai(2042, A::add_scaled_inputs_product(s.ad_value(2041), -1.0, s.ad_value(784), (-0.5), s.ad_value(782), A::sqrt(A::add_scaled_inputs3(s.ad_value(2041), 1.0, s.ad_value(784), 0.25, s.ad_value(790), 1.0)), 1.0), 791);s.store_scaled_add_mixed_ia(2041, 1818, A::sqrt(A::add(A::square(s.ad_value(1818)), s.ad_value(792))), 0.5);s.store_add_mixed_ai(2043, A::add_scaled_inputs_product(s.ad_value(2041), -1.0, s.ad_value(785), (-0.5), s.ad_value(783), A::sqrt(A::add_scaled_inputs3(s.ad_value(2041), 1.0, s.ad_value(785), 0.25, s.ad_value(793), 1.0)), 1.0), 794);s.store_scaled_add(1878, 1817, 2042, (-s.v[354]));s.store_scaled_add(1879, 1818, 2043, (-s.v[354]));}
        s.b[2312] = (p.p40 != 0.0);s.store_scalar(2312, if s.b[2312] { 1.0 } else { 0.0 });s.b[2313] = (s.v[237] > 0.0);s.store_scalar(2313, if s.b[2313] { 1.0 } else { 0.0 });
        if (s.b[2312] && s.b[2313]) {s.store_mul_sqrt_mixed_ia(2044, 795, A::offset(A::square(s.ad_value(1878)), 1e-6));}
        s.b[2314] = (s.v[243] < 0.0);s.store_scalar(2314, if s.b[2314] { 1.0 } else { 0.0 });
        if ((s.b[2312] && s.b[2313]) && s.b[2314]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2044, 2044, 0.5, 801, 0.5, 2044, 801, 1e-6, (-0.5));}
        if (s.b[2312] && s.b[2313]) {s.store_mul_scale_offset_mixed_ia(2027, 798, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(2044), 1.0)), 1.0, (-1.5));s.store_offset(2046, 2042, 3.0);s.store_primal_sub_from_scalar(2047, (-3.0), 235);s.store_scale(2048, 834, 30.0);s.store_scalar(818, (4.0 - 0.9));s.store_add(819, 2046, 2048);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(2027, 2.0, 818, A::sub(s.ad_value(819), A::sqrt(A::sub(A::square(s.ad_value(819)), A::mul3(s.ad_value(818), s.ad_value(2046), s.ad_value(2048))))));s.store_scalar(818, (4.0 - 0.3));s.store_add(819, 2047, 2027);}
        s.b[2317] = (s.v[238] > 0.0);s.store_scalar(2317, if s.b[2317] { 1.0 } else { 0.0 });
        if (s.b[2312] && s.b[2317]) {s.store_mul_sqrt_mixed_ia(2044, 795, A::offset(A::square(s.ad_value(1879)), 1e-6));}
        s.b[2318] = (s.v[245] < 0.0);s.store_scalar(2318, if s.b[2318] { 1.0 } else { 0.0 });
        if ((s.b[2312] && s.b[2317]) && s.b[2318]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2044, 2044, 0.5, 802, 0.5, 2044, 802, 1e-6, (-0.5));}
        if (s.b[2312] && s.b[2317]) {s.store_mul_scale_offset_mixed_ia(2027, 799, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(244), 1.0, s.ad_value(245), s.ad_value(2044), 1.0)), 1.0, (-1.5));s.store_offset(2046, 2043, 3.0);s.store_primal_sub_from_scalar(2047, (-3.0), 235);s.store_scale(2048, 837, 30.0);s.store_scalar(818, (4.0 - 0.9));s.store_add(819, 2046, 2048);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_39(
        s: &mut ReactiveScratch,
    ) {
        if (s.b[2312] && s.b[2317]) {s.store_mul_div_from_scalar_lhs_ad_mixed_ia(2027, 2.0, 818, A::sub(s.ad_value(819), A::sqrt(A::sub(A::square(s.ad_value(819)), A::mul3(s.ad_value(818), s.ad_value(2046), s.ad_value(2048))))));s.store_scalar(818, (4.0 - 0.3));s.store_add(819, 2047, 2027);}
        s.b[2321] = (s.v[236] > 0.0);s.store_scalar(2321, if s.b[2321] { 1.0 } else { 0.0 });s.b[2322] = (s.v[1829] <= 0.0);s.store_scalar(2322, if s.b[2322] { 1.0 } else { 0.0 });
        if ((s.b[2312] && s.b[2321]) && s.b[2322]) {s.store_offset(2027, 777, 1.0);s.store_div_scaled_product_mixed_aii(2028, A::sqrt(s.ad_value(2027)), 826, 1.0, 1855, 1.0);s.store_add_mixed_ai(2029, A::square(s.ad_value(2028)), 2027);s.store_scale(2027, 2028, 2.0);s.store_div_scaled_product3_mixed_iiia(1858, 1855, 1825, 2027, 1.0, A::add(A::sqrt(A::sub(s.ad_value(2029), s.ad_value(2027))), A::sqrt(A::add(s.ad_value(2029), s.ad_value(2027)))), 1.0);}
        s.b[2323] = ((s.v[1859] - s.v[1858]) > (-230.25850929940458));s.store_scalar(2323, if s.b[2323] { 1.0 } else { 0.0 });
        if ((s.b[2312] && s.b[2321]) && s.b[2323]) {s.store_exp_sub(2027, 1859, 1858);}
        if ((s.b[2312] && s.b[2321]) && (!s.b[2323])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1859), s.ad_value(1858)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2312] && s.b[2321]) {s.store_add_scaled_product_mixed_iia(2050, 2030, 1.0, 1824, A::sub_scaled_inputs(s.ad_value(1859), 0.5, A::ln_scaled_input(A::offset(s.ad_value(2027), 1.0), 0.5), 1.0), 1.0);s.store_mul(2051, 235, 1824);s.store_add(2052, 1872, 2051);s.store_scaled_sub_mixed_ia(2053, 2052, A::sqrt_square_offset(A::neg(s.ad_value(2052)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(2044, 795, A::offset(A::square(s.ad_value(1872)), 1e-6));}
        s.b[2324] = (s.v[241] < 0.0);s.store_scalar(2324, if s.b[2324] { 1.0 } else { 0.0 });
        if ((s.b[2312] && s.b[2321]) && s.b[2324]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2044, 2044, 0.5, 800, 0.5, 2044, 800, 1e-6, (-0.5));}
        if (s.b[2312] && s.b[2321]) {s.store_add_scaled_product_mixed_iai(2054, 1862, 1.0, A::add_scaled_inputs3(s.ad_value(2053), 1.0, s.ad_value(742), (-1.0), s.ad_value(2050), -1.0), 1825, 1.0);s.store_mul_scale_offset_mixed_ia(2054, 1825, A::add_scaled_inputs3(s.ad_value(825), 1.0, s.ad_value(2030), 1.0, s.ad_value(2050), -1.0), -1.0, 0.0);}
        s.b[2327] = (((s.v[2054]) as f64).abs() < 230.25850929940458);s.store_scalar(2327, if s.b[2327] { 1.0 } else { 0.0 });
        if ((s.b[2312] && s.b[2321]) && s.b[2327]) {s.store_exp(2027, 2054);}
        s.b[2328] = (s.v[2054] < 0.0);s.store_scalar(2328, if s.b[2328] { 1.0 } else { 0.0 });
        if (((s.b[2312] && s.b[2321]) && (!s.b[2327])) && s.b[2328]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2027, 1e-100, (-230.25850929940458), 2054, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2312] && s.b[2321]) && (!s.b[2327])) && (!s.b[2328])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2027, 2054, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2312] && s.b[2321]) {s.store_mul_scale_offset_mixed_ia(2027, 797, A::mul(s.ad_value(2044), A::add_scaled_product(s.ad_value(240), 1.0, s.ad_value(241), s.ad_value(2044), 1.0)), 1.0, (-1.5));}
        s.b[2331] = ((s.v[1829] <= 0.0) || ((s.v[240] == 0.0) && (s.v[241] == 0.0)));s.store_scalar(2331, if s.b[2331] { 1.0 } else { 0.0 });
        if ((s.b[2312] && s.b[2321]) && (!s.b[2331])) {s.store_add_scaled_product_indices(2027, 240, 1.0, 241, 2044, 2.0);s.store_div_scaled_value_by_product_indices(2058, 246, 1.0, 2027, 797, 1.0);s.store_scaled_div(2059, 1860, 2058, 0.5);}
        s.b[2332] = (s.v[2059] < 0.001);s.store_scalar(2332, if s.b[2332] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_40(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        s.b[2333] = (((s.v[2059]) as f64).abs() < 230.25850929940458);s.store_scalar(2333, if s.b[2333] { 1.0 } else { 0.0 });
        if ((((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) && s.b[2333]) {s.store_exp(2067, 2059);}
        s.b[2334] = (s.v[2059] < 0.0);s.store_scalar(2334, if s.b[2334] { 1.0 } else { 0.0 });
        if (((((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) && (!s.b[2333])) && s.b[2334]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2067, 1e-100, (-230.25850929940458), 2059, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) && (!s.b[2333])) && (!s.b[2334])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2067, 2059, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2312] && s.b[2321]) && (!s.b[2331])) && (!s.b[2332])) {s.store_div_from_scalar(2068, 1.0, 2067);s.store_sub(2027, 2067, 2068);s.store_add(2029, 2067, 2068);}
        s.b[2335] = (p.p42 != 0.0);s.store_scalar(2335, if s.b[2335] { 1.0 } else { 0.0 });s.b[2336] = ((s.v[248] > 0.0) && (s.v[1879] < 0.0));s.store_scalar(2336, if s.b[2336] { 1.0 } else { 0.0 });
        if (s.b[2335] && s.b[2336]) {s.store_sqrt_offset_ad(2071, A::add_scaled_square_product(s.ad_value(1879), 1.0, A::square(s.ad_value(254)), A::square(s.ad_value(836)), 1.0), 1e-6);s.store_div_scaled_inputs_indices(2027, 807, -1.0, 2071, 1.0);}
        s.b[2337] = (s.v[2027] > (-230.25850929940458));s.store_scalar(2337, if s.b[2337] { 1.0 } else { 0.0 });
        if ((s.b[2335] && s.b[2336]) && s.b[2337]) {s.store_exp(2029, 2027);}
        if ((s.b[2335] && s.b[2336]) && (!s.b[2337])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2029, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.b[2338] = ((s.v[247] > 0.0) && (s.v[1878] < 0.0));s.store_scalar(2338, if s.b[2338] { 1.0 } else { 0.0 });
        if (s.b[2335] && s.b[2338]) {s.store_sqrt_offset_ad(2072, A::add_scaled_square_product(s.ad_value(1878), 1.0, A::square(s.ad_value(253)), A::square(s.ad_value(835)), 1.0), 1e-6);s.store_div_scaled_inputs_indices(2027, 806, -1.0, 2072, 1.0);}
        s.b[2339] = (s.v[2027] > (-230.25850929940458));s.store_scalar(2339, if s.b[2339] { 1.0 } else { 0.0 });
        if ((s.b[2335] && s.b[2338]) && s.b[2339]) {s.store_exp(2029, 2027);}
        if ((s.b[2335] && s.b[2338]) && (!s.b[2339])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2029, 1e-100, (-230.25850929940458), 2027, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        s.store_scalar(2076, s.v[715]);s.store_scalar(1880, 0.0);s.store_scalar(1881, 0.0);s.store_scalar(1882, 0.0);s.store_scalar(1883, 1e-40);s.store_scalar(1884, 1.0);s.store_scalar(846, 0.0);s.b[2340] = ((p.p46 != 0.0) && (s.v[287] > 0.0));s.store_scalar(2340, if s.b[2340] { 1.0 } else { 0.0 });
        if s.b[2340] {s.store_add_scaled_inputs4_mixed_iiai(2027, 828, 0.5, 827, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(764))), (-0.5), 762, 1.0);s.store_add_scaled_inputs4_mixed_iiai(2073, 827, 1.0, 2027, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2027)), s.ad_value(763))), (-(-0.5)), 766, 1.0);s.store_add_scaled_inputs3_indices(2074, 2073, 1.0, 826, 0.5, 830, (-0.5));s.store_mul_ad_product_rhs(2075, 289, A::offset(A::mul(s.ad_value(291), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(290), s.ad_value(2074)), 1.0));s.store_mul_scale_offset_indices(2076, 723, 2075, 1.0, 1.0);s.store_div_from_scalar(2077, 1.0, 2076);s.store_div_scaled_value_offset_denominator(2078, s.ad_value(830), 2.0, A::sqrt_product_offset(s.ad_value(293), s.ad_value(830), 1.0), 1.0, 1.0);s.store_mul_ad_product_rhs_mixed_ia(2079, 292, 2078, A::offset(A::mul(s.ad_value(294), s.ad_value(2074)), 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(1880, 2077, 829, 1.0, 2079, 1.0, 713, -1.0, 0.0);s.store_mul(2080, 2077, 760);s.store_scaled_ln_ad(2081, A::add(A::div(s.ad_value(2080), s.ad_value(761)), A::sqrt(s.ad_value(2080))), 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_41(
        s: &mut ReactiveScratch,
    ) {
        if s.b[2340] {s.store_mul(2082, 2077, 2073);s.store_add(2087, 2080, 2082);s.store_add_scaled_product_mixed_iia(2088, 2087, 1.0, 761, A::sqrt(s.ad_value(2087)), 1.0);s.store_add(2089, 2088, 2081);s.store_offset_div_scaled_inputs_sqrt_rhs(2090, 761, 1.0, 2087, 2.0, 1.0);s.store_div_from_scalar(2091, 1.0, 2090);s.store_sub(2092, 1880, 2089);}
        s.b[2341] = (s.v[2092] > (-12.0));s.store_scalar(2341, if s.b[2341] { 1.0 } else { 0.0 });
        if (s.b[2340] && s.b[2341]) {s.store_offset_add(2093, 2092, 725, (-1.0));s.store_scaled_add_mixed_ia(2094, 2093, A::sqrt_square_offset(s.ad_value(2093), 10.0), 0.5);s.store_add_mixed_ai(2095, A::add_scaled_product(s.ad_value(2092), 1.0, s.ad_value(2090), A::ln(s.ad_value(2094)), (-1.0)), 725);s.store_scaled_add_mixed_ia(2096, 2095, A::sqrt_square_offset(s.ad_value(2095), 2.0), 0.5);}
        s.b[2342] = ((s.v[2092] - s.v[2096]) < 230.25850929940458);s.store_scalar(2342, if s.b[2342] { 1.0 } else { 0.0 });
        if ((s.b[2340] && s.b[2341]) && s.b[2342]) {s.store_exp_sub(2097, 2092, 2096);}
        if ((s.b[2340] && s.b[2341]) && (!s.b[2342])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2097, A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2340] && s.b[2341]) {s.store_mul(2098, 724, 2097);s.store_pow_indices(2099, 2098, 2091);s.store_add_scaled_square_product_mixed_iai(2100, 2090, 1.0, A::add_scaled_inputs3(s.ad_value(2096), 2.0, s.ad_value(2090), 2.0, s.ad_value(2099), -1.0), 2099, 1.0);s.store_mul_scale_offset_mixed_ia(2101, 2090, A::div_scaled_inputs2(A::sqrt(s.ad_value(2100)), 1.0, s.ad_value(2090), (-1.0), s.ad_value(2099), 1.0), 1.0, (-1.0));s.store_sub(2083, 2096, 2101);}
        s.b[2343] = ((s.v[2091] * (s.v[2092] + s.v[725])) > (-230.25850929940458));s.store_scalar(2343, if s.b[2343] { 1.0 } else { 0.0 });
        if ((s.b[2340] && (!s.b[2341])) && s.b[2343]) {s.store_exp_ad(2083, A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))));}
        if ((s.b[2340] && (!s.b[2341])) && (!s.b[2343])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2083, 1e-100, (-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if s.b[2340] {s.store_mul_add_rhs(2084, 2077, 1857, 2073);}
        s.b[2344] = ((s.v[2083] < 0.001) && (s.v[1857] < 1e-6));s.store_scalar(2344, if s.b[2344] { 1.0 } else { 0.0 });s.b[2345] = (((-s.v[2084]) + s.v[2082]) > (-230.25850929940458));s.store_scalar(2345, if s.b[2345] { 1.0 } else { 0.0 });
        if ((s.b[2340] && s.b[2344]) && s.b[2345]) {s.store_exp_sub(2027, 2082, 2084);}
        if ((s.b[2340] && s.b[2344]) && (!s.b[2345])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::sub(s.ad_value(2082), s.ad_value(2084)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2340] && s.b[2344]) {s.store_mul_scale_offset_indices(1881, 2083, 2027, 1.0, (-1.0));s.store_add(2085, 1881, 2083);}
        if (s.b[2340] && (!s.b[2344])) {s.store_add(2087, 2080, 2084);s.store_add_scaled_product_mixed_iia(2088, 2087, 1.0, 761, A::sqrt(s.ad_value(2087)), 1.0);s.store_add(2089, 2088, 2081);s.store_offset_div_scaled_inputs_sqrt_rhs(2090, 761, 1.0, 2087, 2.0, 1.0);s.store_div_from_scalar(2091, 1.0, 2090);s.store_sub(2092, 1880, 2089);}
        s.b[2346] = (s.v[2092] > (-12.0));s.store_scalar(2346, if s.b[2346] { 1.0 } else { 0.0 });
        if ((s.b[2340] && (!s.b[2344])) && s.b[2346]) {s.store_offset_add(2093, 2092, 725, (-1.0));s.store_scaled_add_mixed_ia(2094, 2093, A::sqrt_square_offset(s.ad_value(2093), 10.0), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_42(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[2340] && (!s.b[2344])) && s.b[2346]) {s.store_add_mixed_ai(2095, A::add_scaled_product(s.ad_value(2092), 1.0, s.ad_value(2090), A::ln(s.ad_value(2094)), (-1.0)), 725);s.store_scaled_add_mixed_ia(2096, 2095, A::sqrt_square_offset(s.ad_value(2095), 2.0), 0.5);}
        s.b[2347] = ((s.v[2092] - s.v[2096]) < 230.25850929940458);s.store_scalar(2347, if s.b[2347] { 1.0 } else { 0.0 });
        if (((s.b[2340] && (!s.b[2344])) && s.b[2346]) && s.b[2347]) {s.store_exp_sub(2097, 2092, 2096);}
        if (((s.b[2340] && (!s.b[2344])) && s.b[2346]) && (!s.b[2347])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2097, A::sub(s.ad_value(2092), s.ad_value(2096)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2340] && (!s.b[2344])) && s.b[2346]) {s.store_mul(2098, 724, 2097);s.store_pow_indices(2099, 2098, 2091);s.store_add_scaled_square_product_mixed_iai(2100, 2090, 1.0, A::add_scaled_inputs3(s.ad_value(2096), 2.0, s.ad_value(2090), 2.0, s.ad_value(2099), -1.0), 2099, 1.0);s.store_mul_scale_offset_mixed_ia(2101, 2090, A::div_scaled_inputs2(A::sqrt(s.ad_value(2100)), 1.0, s.ad_value(2090), (-1.0), s.ad_value(2099), 1.0), 1.0, (-1.0));s.store_sub(2085, 2096, 2101);}
        s.b[2348] = ((s.v[2091] * (s.v[2092] + s.v[725])) > (-230.25850929940458));s.store_scalar(2348, if s.b[2348] { 1.0 } else { 0.0 });
        if (((s.b[2340] && (!s.b[2344])) && (!s.b[2346])) && s.b[2348]) {s.store_exp_ad(2085, A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))));}
        if (((s.b[2340] && (!s.b[2344])) && (!s.b[2346])) && (!s.b[2348])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2085, 1e-100, (-230.25850929940458), A::mul(s.ad_value(2091), A::add(s.ad_value(2092), s.ad_value(725))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2340] && (!s.b[2344])) {s.store_sub(1881, 2085, 2083);}
        if s.b[2340] {s.store_scaled_add(1882, 2085, 2083, 0.5);}
        if s.b[2340] {
            if ((s.v[1880] - s.v[1882]) > 1e-40) {
                s.store_sub(1883, 1880, 1882);
            } else {
                s.store_scalar(1883, 1e-40);
            }
        }
        if s.b[2340] {s.store_sub_from_scalar_ad(1884, 1.0, A::div_scaled_inputs(s.ad_value(761), 0.5, A::sqrt(A::add_scaled_inputs(s.ad_value(1883), 1.0, s.ad_value(724), 0.25)), 1.0));s.store_div_scaled_product3_mixed_aaii(846, A::mul3_scaled_output(s.ad_value(717), s.ad_value(2076), s.ad_value(2076), -1.0), A::offset(A::mul(s.ad_value(1884), s.ad_value(1882)), 1.0), 1881, 1.0, 1869, 1.0);}
        s.store_scalar(1885, 0.0);s.store_scalar(847, 0.0);s.b[2349] = ((s.v[1829] > 0.0) && (p.p41 != 0.0));s.store_scalar(2349, if s.b[2349] { 1.0 } else { 0.0 });
        if s.b[2349] {s.store_add_scaled_product_indices(2086, 826, 1.0, 232, 1860, (-1.0));}
        s.b[2350] = (s.v[2086] > 0.0);s.store_scalar(2350, if s.b[2350] { 1.0 } else { 0.0 });
        if (s.b[2349] && s.b[2350]) {s.store_mul_div_scaled_offset_numerator_rhs(2029, 712, A::mul(s.ad_value(233), A::sub(A::sqrt(A::add(s.ad_value(728), s.ad_value(2030))), s.ad_value(736))), 1.0, 1.0, A::offset(s.ad_value(2086), 1e-30), 1.0);}
        s.b[2351] = ((((-s.v[2029])) as f64).abs() < 230.25850929940458);s.store_scalar(2351, if s.b[2351] { 1.0 } else { 0.0 });
        if ((s.b[2349] && s.b[2350]) && s.b[2351]) {s.store_exp_neg_input(2027, 2029);}
        s.b[2352] = ((-s.v[2029]) < 0.0);s.store_scalar(2352, if s.b[2352] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_43(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if (((s.b[2349] && s.b[2350]) && (!s.b[2351])) && s.b[2352]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2027, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2029)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2349] && s.b[2350]) && (!s.b[2351])) && (!s.b[2352])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::neg(s.ad_value(2029)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2349] && s.b[2350]) {s.store_mul3_lhs(1885, 229, 2086, 2027);s.store_mul_add_rhs(847, 1885, 838, 846);}
        s.b[2353] = (s.v[847] > (0.5 * s.v[234]));s.store_scalar(2353, if s.b[2353] { 1.0 } else { 0.0 });
        if ((s.b[2349] && s.b[2350]) && s.b[2353]) {s.store_offset_div_scaled_inputs_indices(2027, 847, 2.0, 234, 1.0, (-1.0));s.store_mul_scaled_offset_ad_rhs(847, 234, 0.5, A::div(s.ad_value(2027), A::sqrt_square_offset(s.ad_value(2027), 1.0)), 1.0);}
        s.b[2547] = (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0));s.store_scalar(2547, if s.b[2547] { 1.0 } else { 0.0 });s.b[2548] = ((p.p45 > 0.0) || (p.p47 > 0.0));s.store_scalar(2548, if s.b[2548] { 1.0 } else { 0.0 });
        if (s.b[2547] && s.b[2548]) {s.copy_ad(2388, 728);s.copy_ad(2389, 738);s.copy_ad(2390, 729);s.copy_ad(2391, 1820);s.copy_ad(2392, 1821);s.store_scalar(2396, 0.0);}
        s.b[2549] = (p.p47 > 0.0);s.store_scalar(2549, if s.b[2549] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2548]) && s.b[2549]) {s.store_add_scaled_inputs4_mixed_iiai(2391, 828, 0.5, 827, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(828), s.ad_value(827))), s.ad_value(749))), (-0.5), 747, 1.0);s.store_add_scaled_inputs4_mixed_iiai(1886, 827, 1.0, 2391, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2391)), s.ad_value(748))), (-(-0.5)), 750, 1.0);s.copy_ad(2392, 1886);s.copy_ad(2388, 745);s.copy_ad(2389, 748);s.copy_ad(2390, 746);}
        if (s.b[2547] && s.b[2548]) {s.store_add_scaled_inputs3_indices(2395, 829, 1.0, 2396, (-1.0), 700, -1.0);s.store_add_scaled_inputs3_indices(2397, 2392, 1.0, 826, 0.5, 830, (-0.5));s.store_scalar(2409, 1.0);}
        s.b[2550] = (s.v[190] > 0.0);s.store_scalar(2550, if s.b[2550] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2548]) && s.b[2550]) {s.store_primal_scale(2400, 2388, s.v[361]);s.store_scale(2401, 2397, s.v[361]);s.store_scale(2402, 2395, s.v[361]);s.store_offset_div_scaled_inputs_sqrt_rhs(2028, 2390, 0.5, 2400, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(2029, 2400, 1.0, 2390, A::sqrt(s.ad_value(2400)), 1.0);s.store_add_scaled_inputs_product_mixed_aiai(2403, A::div_scaled_inputs2(s.ad_value(2402), 1.0, s.ad_value(2029), (-1.0), s.ad_value(2028), 1.0), 1.0, 2400, 0.5, A::offset(s.ad_value(191), 1.0), 2401, (-1.0));s.store_primal_offset_scaled(2404, 2400, 0.5, 2.0);s.store_add(2405, 2400, 2401);s.store_sub_scaled_inputs_ad(2028, A::add_scaled_inputs_product(s.ad_value(2402), 1.0, s.ad_value(2405), (-1.0), s.ad_value(2390), A::sqrt(s.ad_value(2405)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2400), s.ad_value(2390)), A::sqrt(s.ad_value(2400)))), 2.0);s.store_add_scaled_inputs(2406, 2028, 2.0, 2404, 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2403, 0.5, 2406, 0.5, 2403, 2406, 20.0, 0.5);s.store_add_scaled_inputs3_indices(2029, 2402, 2.0, 2401, (-2.0), 2404, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2407, 2028, 0.5, 2029, 0.5, 2028, 2029, 20.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2028, 2407, 0.5, 2404, 0.5, 2407, 2404, 5.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2408, 2028, 0.5, 2404, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(2028), 1.0, s.ad_value(2404), -1.0)), 20.0), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_44(
        s: &mut ReactiveScratch,
        p: &Parameters,
    ) {
        if ((s.b[2547] && s.b[2548]) && s.b[2550]) {s.store_mul_scale_offset_mixed_ia(2029, 702, A::div(s.ad_value(2408), s.ad_value(2404)), 1.0, 1.0);}
        s.b[2551] = (s.v[2029] > (-230.25850929940458));s.store_scalar(2551, if s.b[2551] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2548]) && s.b[2550]) && s.b[2551]) {s.store_exp(2409, 2029);}
        if (((s.b[2547] && s.b[2548]) && s.b[2550]) && (!s.b[2551])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2409, 1e-100, (-230.25850929940458), 2029, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2547] && s.b[2548]) {s.store_offset_mul(2410, 701, 2409, 1.0);s.store_scale(2411, 2410, s.v[715]);s.store_mul_ad_product_rhs(2412, 199, A::offset(A::mul(s.ad_value(201), s.ad_value(830)), 1.0), A::offset(A::mul(s.ad_value(200), s.ad_value(2397)), 1.0));s.store_mul_scale_offset_indices(2413, 2411, 2412, 1.0, 1.0);s.store_div_from_scalar(2414, 1.0, 2413);s.store_mul_mixed_ia(2398, 2390, A::sqrt_scaled_input(s.ad_value(2414), s.v[715]));s.store_square(2399, 2398);s.store_div_from_scalar(2415, 1.0, 2399);s.store_mul(2416, 2392, 2414);s.store_mul(2417, 2395, 2414);s.store_div_scaled_value_offset_denominator(2418, s.ad_value(830), 2.0, A::sqrt_product_offset(s.ad_value(197), s.ad_value(830), 1.0), 1.0, 1.0);s.store_mul_ad_product_rhs_mixed_ia(2419, 196, 2418, A::offset(A::mul(s.ad_value(198), s.ad_value(2397)), 1.0));s.store_mul(2420, 2388, 2414);s.store_sqrt_square_add(2028, 2391, 2389);s.store_sqrt_add_ad(2029, A::square(A::sub(s.ad_value(2391), s.ad_value(2419))), s.ad_value(2389));s.store_mul_add_scaled_inputs3_offset_rhs_indices(2421, 2414, 2419, 0.5, 2028, 0.5, 2029, ((-1.0) * (0.5)), 0.0);s.store_add(2422, 2420, 2416);s.store_sub(2423, 2422, 2421);}
        s.b[2552] = (p.p45 > 0.0);s.store_scalar(2552, if s.b[2552] { 1.0 } else { 0.0 });s.b[2553] = (((s.v[2423]) as f64).abs() < 1e-5);s.store_scalar(2553, if s.b[2553] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && s.b[2553]) {s.store_offset_ad(2424, A::mul_sub_from_scalar_rhs(s.ad_value(2398), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2423), 1.0, A::scale(s.ad_value(2423), 0.3125), 0.5)), 1.0);}
        s.b[2554] = (s.v[2423] < 460.51701859880916);s.store_scalar(2554, if s.b[2554] { 1.0 } else { 0.0 });
        if ((((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) && s.b[2554]) {s.store_exp_neg_input(2438, 2423);}
        if ((((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) && (!s.b[2554])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2438, 1e-200, 2423, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) {s.store_scalar(2027, (if (s.v[2423] > 0.0) { 1.0 } else { (-1.0) }));}
        if (((s.b[2547] && s.b[2548]) && s.b[2552]) && (!s.b[2553])) {s.store_offset_ad(2424, A::div_scaled_product3(s.ad_value(2027), s.ad_value(2398), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2438), 1.0, s.ad_value(2423))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2423), 1.0, s.ad_value(2438))), 2.0), 1.0);}
        if ((s.b[2547] && s.b[2548]) && (!s.b[2552])) {s.store_offset_div_scaled_inputs_sqrt_rhs(2424, 2398, 0.5, 2423, 1.0, 1.0);}
        if (s.b[2547] && s.b[2548]) {s.store_add_scaled_value_products_mixed_iiaia(2425, 2423, 1.0, 2398, A::sqrt(s.ad_value(2423)), 1.0, 2424, A::ln(A::offset(s.ad_value(2424), (-1.0))), (-1.0));s.store_div_scaled_inputs2_indices(2426, 2417, 1.0, 2425, (-1.0), 2424, 1.0);s.store_mul_scaled_offset_ad_rhs(2432, 2399, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2399)), 1.0)), (-1.0));s.store_scalar(2431, 0.0);s.store_scalar(2433, 1.0);}
        s.b[2555] = (s.v[2426] > (-30.0));s.store_scalar(2555, if s.b[2555] { 1.0 } else { 0.0 });
    }
    #[inline(never)]
    pub(super) fn stamp_reactive_block_45(
        s: &mut ReactiveScratch,
    ) {
        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {s.store_offset_mul(2427, 2424, 2426, (-1.0));s.store_scaled_add_mixed_ia(2027, 2427, A::sqrt_square_offset(s.ad_value(2427), 10.0), 0.5);s.store_sub_mixed_ia(2428, 2426, A::ln(s.ad_value(2027)));s.store_scaled_add_mixed_ia(2429, 2428, A::sqrt_square_offset(s.ad_value(2428), 2.0), 0.5);}
        s.b[2556] = ((s.v[2426] - s.v[2429]) < 230.25850929940458);s.store_scalar(2556, if s.b[2556] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && s.b[2556]) {s.store_exp_sub(2027, 2426, 2429);}
        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && (!s.b[2556])) {s.store_scaled_softlimit_poly_offset_lhs_ad(2027, A::sub(s.ad_value(2426), s.ad_value(2429)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {s.store_div(2430, 2027, 2424);s.store_sub_mixed_ai(2027, A::scaled_offset(s.ad_value(2429), 1.0, 2.0), 2430);}
        s.b[2557] = (s.v[2430] > 1e-6);s.store_scalar(2557, if s.b[2557] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && s.b[2557]) {s.store_mul_scale_offset_mixed_ia(2431, 2424, A::sub(s.ad_value(2429), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2430), s.ad_value(2027), 1.0), 1.0, (-1.0), s.ad_value(2430), 1.0)), 1.0, 1.0);}
        if (((s.b[2547] && s.b[2548]) && s.b[2555]) && (!s.b[2557])) {s.store_mul_ad_affine_product_rhs(2431, 2424, s.ad_value(2430), A::offset(A::mul_scaled_lhs(s.ad_value(2027), 0.25, s.ad_value(2027)), 1.0), 0.5, 0.0);}
        if ((s.b[2547] && s.b[2548]) && s.b[2555]) {s.store_add_scaled_inputs3_offset_mixed_iia(2027, 2417, 0.5, 2431, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2417), s.ad_value(2431)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));s.store_mul_scaled_offset_ad_rhs(2432, 2399, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2399)), s.ad_value(2027), 1.0), (-1.0));s.store_div_add_scaled_inputs_rhs_indices(2433, 2432, 2432, 1.0, 2431, 1.0);s.store_add_scaled_product_indices(2423, 2422, 1.0, 2433, 2421, (-1.0));}
        if (s.b[2547] && s.b[2548]) {s.store_offset_scaled(2434, 2398, 0.7071067811865475, 1.0);s.store_scale(2435, 2434, 1e-5);s.store_div_from_scalar(2436, 1.0, 2434);s.store_scalar(2543, 0.0);s.store_scalar(2437, 0.0);}
        s.b[2558] = (s.v[2423] < 460.51701859880916);s.store_scalar(2558, if s.b[2558] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2548]) && s.b[2558]) {s.store_exp_neg_input(2438, 2423);}
        if ((s.b[2547] && s.b[2548]) && (!s.b[2558])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2438, 1e-200, 2423, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[2559] = (((s.v[2417]) as f64).abs() <= s.v[2435]);s.store_scalar(2559, if s.b[2559] { 1.0 } else { 0.0 });
        if ((s.b[2547] && s.b[2548]) && s.b[2559]) {s.store_scaled_square(2523, 2436, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2437, 2417, 2436, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2417), 1.0, s.ad_value(2438)), s.ad_value(2398), s.ad_value(2523)), 1.0));}
        s.b[2560] = (s.v[2417] < (-s.v[2435]));s.store_scalar(2560, if s.b[2560] { 1.0 } else { 0.0 });
        if (((s.b[2547] && s.b[2548]) && (!s.b[2559])) && s.b[2560]) {s.store_neg(2525, 2417);s.store_scaled_mul(2526, 2525, 2436, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(2527, 2526, 10.0, (-6.0), 64.0, 0.5);s.store_sub(2522, 2525, 2527);s.store_add_scaled_square_product_mixed_iia(2528, 2522, 1.0, 2399, A::offset(s.ad_value(2527), 1.0), 1.0);}
    }
}
