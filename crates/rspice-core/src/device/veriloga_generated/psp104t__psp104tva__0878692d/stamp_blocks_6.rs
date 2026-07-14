#![allow(dead_code, non_snake_case, unused_assignments, unused_imports, unused_parens, unused_variables)]
use super::{A, ddt_jacobian, eval_ddt, eval_idt, GeneratedDerivative, GeneratedEvalContext, GeneratedReactiveStamper, GeneratedStamper, idt_jacobian, Scratch, LIMEXP_MAX, THERMAL_VOLTAGE_PER_K};
use super::super::state::{Instance, Parameters};
impl Instance {
    #[inline(never)]
    pub(super) fn stamp_transient_block_96(
        s: &mut Scratch,
    ) {
        if ((s.b[2194] && s.b[2206]) && (!s.b[2207])) {s.store_mul_add_scaled_inputs3_offset_rhs_mixed_aii(2115, 2105, A::div_from_scalar(1.0, s.ad_value(2112)), 1.0, 2106, (-1.0), 2111, -1.0, (-1.0));}
        s.b[2208] = (s.v[2106] > (s.v[2103] - 230.25850929940458));s.store_scalar(2208, if s.b[2208] { 1.0 } else { 0.0 });
        if ((s.b[2194] && (!s.b[2206])) && s.b[2208]) {s.store_exp_sub(1929, 2106, 2103);s.store_div(2112, 2105, 1929);s.store_add_scaled_product_mixed_iia(2115, 1929, 1.0, 2105, A::add(A::offset(s.ad_value(2106), 1.0), s.ad_value(2111)), (-1.0));}
        if ((s.b[2194] && (!s.b[2206])) && (!s.b[2208])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2112, 1e-100, 2106, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(1929, 1e-100, A::sub(s.ad_value(2103), s.ad_value(2106)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(2115, 1929, 1.0, 2105, A::add(A::offset(s.ad_value(2106), 1.0), s.ad_value(2111)), (-1.0));}
        if (s.b[2194] && (!s.b[2206])) {s.store_add_offset_lhs(2113, 2106, (-1.0), 2112);s.store_sqrt(2114, 2113);}
        if s.b[2194] {s.store_mul3_lhs(2116, 2114, 2020, 2035);s.store_scaled_add(2117, 2059, 2106, 0.5);s.store_scalar(2118, 0.0);s.store_mul(1929, 2112, 2065);}
        s.b[2209] = (s.v[1929] > 0.0);s.store_scalar(2209, if s.b[2209] { 1.0 } else { 0.0 });
        if (s.b[2194] && s.b[2209]) {s.store_sqrt(2118, 1929);}
        if s.b[2194] {s.store_scaled_add(2119, 2066, 2115, 0.5);s.store_add_scaled_product_mixed_iaa(2120, 2119, 1.0, A::square(s.ad_value(2107)), A::sub_scaled_inputs(s.ad_value(2118), 1.0, s.ad_value(2037), 2.0), 0.125);}
        s.b[2210] = (s.v[2117] < 1e-5);s.store_scalar(2210, if s.b[2210] { 1.0 } else { 0.0 });
        if (s.b[2194] && s.b[2210]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2121, 2117, 1.0, 2117, 1.0, 2117, 0.25, 0.3333333333333333, 0.5);s.store_mul_sqrt_mixed_ia(2122, 2020, A::add(s.ad_value(2120), s.ad_value(2121)));}
        s.b[2211] = (s.v[719] > 0.0);s.store_scalar(2211, if s.b[2211] { 1.0 } else { 0.0 });
        if ((s.b[2194] && s.b[2210]) && s.b[2211]) {s.store_div_from_scalar_sqrt_ad(2123, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2122)), 1.0));}
        if (s.b[2194] && s.b[2210]) {s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2117), 1.0, A::scale(s.ad_value(2117), 0.25), 0.3333333333333333));s.store_scaled_mul(2124, 2117, 1929, 0.7071067811865475);s.store_add_mixed_ia(2125, 2123, A::div_scaled_product(s.ad_value(2020), A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2117), 0.5)), 1.0, A::square(s.ad_value(2117)), 0.16666666666666666), 0.7071067811865475, s.ad_value(1929), 1.0));}
        if (s.b[2194] && (!s.b[2210])) {s.store_add_offset_lhs(2121, 2117, (-1.0), 2118);s.store_mul_sqrt_mixed_ia(2122, 2020, A::add(s.ad_value(2120), s.ad_value(2121)));}
        s.b[2212] = (s.v[719] > 0.0);s.store_scalar(2212, if s.b[2212] { 1.0 } else { 0.0 });
        if ((s.b[2194] && (!s.b[2210])) && s.b[2212]) {s.store_add_scaled_sub_value_product_indices(2126, 1.0, 2118, 1.0, 2122, 2037, 2.0);s.store_div_from_scalar_sqrt_ad(2123, 1.0, A::offset(A::mul(s.ad_value(719), s.ad_value(2122)), 1.0));s.store_div_scaled_value_offset_denominator(1929, s.ad_value(2123), 1.0, s.ad_value(2123), 1.0, 1.0);s.store_mul_product3_mixed_iaii(2127, 719, A::square(s.ad_value(1929)), 2021, 2120, 1.0);s.store_add_scaled_inputs_product_mixed_iiia(2128, 2122, 2.0, 2127, (-2.0), 2021, A::add(A::sub_from_scalar(1.0, s.ad_value(2118)), s.ad_value(2120)), 1.0);s.store_mul_sub_scaled_inputs_rhs_indices(2129, 2127, 2127, 1.0, 2122, 2.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_97(
        s: &mut Scratch,
    ) {
        if ((s.b[2194] && (!s.b[2210])) && s.b[2212]) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2130, 1.0, 2021, A::add(s.ad_value(2118), s.ad_value(2120)), 0.5);s.store_div_scaled_product_mixed_iia(2131, 2129, 2128, 1.0, A::add_scaled_square_product(s.ad_value(2128), 1.0, s.ad_value(2130), s.ad_value(2129), (-1.0)), 1.0);s.store_add(2117, 2117, 2131);s.store_exp(2132, 2131);s.store_div(2118, 2118, 2132);s.store_mul(2120, 2120, 2132);s.store_add_offset_lhs(2121, 2117, (-1.0), 2118);s.store_mul_sqrt_mixed_ia(2122, 2020, A::add(s.ad_value(2120), s.ad_value(2121)));s.store_add_ad(2133, A::sub_from_scalar(1.0, s.ad_value(2118)), A::mul3_scaled_output(s.ad_value(2122), s.ad_value(2123), s.ad_value(2037), 2.0));s.store_div_scaled_product3_mixed_iiaa(2107, 2107, 2132, A::add(s.ad_value(2126), s.ad_value(2119)), 1.0, A::add_scaled_product(s.ad_value(2133), 1.0, s.ad_value(2132), s.ad_value(2119), 1.0), 1.0);s.store_mul(2110, 2107, 2035);}
        if (s.b[2194] && (!s.b[2210])) {s.store_sqrt(2124, 2121);s.store_add_scaled_inputs_mixed_ia(2125, 2123, 1.0, A::div(A::mul_sub_from_scalar_rhs(s.ad_value(2020), 1.0, s.ad_value(2118)), s.ad_value(2124)), 0.5);}
        if s.b[2194] {s.store_mul_div_scaled_product_mixed_iiia(2134, 2035, 2021, 2120, 1.0, A::add_scaled_product(s.ad_value(2122), 1.0, s.ad_value(2020), s.ad_value(2124), 1.0), 1.0);s.store_add_scaled_product_indices(2135, 2134, 1.0, 2035, 2125, 1.0);s.store_mul3_lhs(2136, 2124, 2020, 2035);}
        s.b[2213] = (s.v[216] < 0.0);s.store_scalar(2213, if s.b[2213] { 1.0 } else { 0.0 });
        if (s.b[2194] && s.b[2213]) {s.store_sub_from_scalar_scaled_mul(2075, 1.0, 216, 2134, 1.0);}
        if (s.b[2194] && (!s.b[2213])) {s.store_div_from_scalar_offset_product(2075, 1.0, 216, 2134, 1.0);}
        if s.b[2194] {s.store_mul_product3_indices(2076, 2134, 746, 2074, 2075, 1.0);s.store_add_scaled_product_indices(2137, 2136, 1.0, 764, 2134, 1.0);s.store_add_scaled_product_indices(2138, 2136, 1.0, 765, 2134, 1.0);s.store_mul(2139, 763, 2137);s.store_ln_ad(1930, A::div_scaled_value_offset_denominator(s.ad_value(2121), 1.0, A::add(s.ad_value(2121), s.ad_value(2120)), 1e-14, 1.0));s.store_add_scaled_product_mixed_aia(2078, A::pow(A::mul(s.ad_value(2139), s.ad_value(705)), s.ad_value(706)), 1.0, 707, A::exp(A::mul_scaled_lhs(s.ad_value(708), 0.5, s.ad_value(1930))), 1.0);s.store_mul_add_mixed_iai(2140, 2070, A::offset(s.ad_value(2078), 1.0), 2076);s.store_ln_ad(2141, A::div_scaled_offset_numerator(A::mul(A::sub(s.ad_value(815), s.ad_value(2110)), s.ad_value(768)), 1.0, 1.0, A::offset(A::mul(A::sub(s.ad_value(2101), s.ad_value(2110)), s.ad_value(768)), 1.0), 1.0));s.store_mul(1931, 2134, 2080);s.store_div_add_scaled_inputs_rhs_indices(2081, 1931, 221, 1.0, 1931, 1.0);}
        s.b[2214] = (s.v[220] < 0.0);s.store_scalar(2214, if s.b[2214] { 1.0 } else { 0.0 });
        if (s.b[2194] && s.b[2214]) {s.store_div_from_scalar_sub_from_scalar_ad(2082, 1.0, 1.0, A::mul(s.ad_value(220), s.ad_value(2081)));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_98(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2194] && (!s.b[2214])) {s.store_offset_mul(2082, 220, 2081, 1.0);}
        if s.b[2194] {s.store_mul(2143, 2015, 2082);s.store_mul(2142, 2122, 2035);}
        s.copy_ad(1839, 2083);s.copy_ad(1841, 2101);s.copy_ad(1842, 2102);s.copy_ad(1843, 2107);s.copy_ad(1844, 2110);s.copy_ad(1846, 2117);s.copy_ad(1845, 2116);s.copy_ad(1847, 2123);s.copy_ad(1848, 2125);s.copy_ad(1849, 2134);s.copy_ad(1850, 2135);s.copy_ad(1851, 2136);s.copy_ad(1852, 2138);s.copy_ad(1853, 2140);s.copy_ad(1855, 2141);s.copy_ad(1854, 2143);s.copy_ad(1856, 2142);s.store_scalar(1857, 1.0);s.store_scalar(1858, 1.0);s.store_scalar(1860, 1.0);s.store_scalar(1861, 1.0);s.store_scalar(827, 0.0);s.b[2215] = (s.v[1813] > 0.0);s.store_scalar(2215, if s.b[2215] { 1.0 } else { 0.0 });
        if s.b[2215] {s.store_ln_ad(1939, A::offset(A::mul(s.ad_value(819), s.ad_value(768)), 1.0));s.store_div_scaled_product_indices(1929, 1808, 1848, 1.0, 1850, 1.0);s.store_add_scaled_product_mixed_aai(1938, A::mul3(A::mul3(s.ad_value(225), s.ad_value(1851), s.ad_value(1929)), s.ad_value(1929), s.ad_value(1939)), 1.0, A::div_scaled_product(A::add(s.ad_value(223), A::div(s.ad_value(224), s.ad_value(1850))), s.ad_value(1849), 1.0, s.ad_value(1850), 1.0), 1855, 1.0);s.store_div_from_scalar_add_ad(1857, 1.0, A::offset(s.ad_value(1938), 1.0), A::square(s.ad_value(1938)));s.store_mul(1858, 1853, 1857);s.store_div(1859, 1854, 1858);s.store_mul_ad_product_lhs_mixed_ai(1940, A::square(s.ad_value(1859)), 1844, 1844);}
        s.b[2216] = (s.v[0] == (-1.0));s.store_scalar(2216, if s.b[2216] { 1.0 } else { 0.0 });
        if (s.b[2215] && s.b[2216]) {s.store_div_scaled_value_offset_denominator(1940, s.ad_value(1940), 1.0, A::mul(s.ad_value(1859), s.ad_value(1844)), 1.0, 1.0);}
        if s.b[2215] {s.store_mul_scale_offset_mixed_ia(1941, 1858, A::sqrt(A::scale_offset(s.ad_value(1940), 2.0, 1.0)), 0.5, (1.0) * (0.5));s.store_div_from_scalar(1860, 1.0, 1941);s.store_mul(1929, 1858, 1860);s.store_mul_scale_offset_mixed_ia(1942, 1848, A::mul3_scaled_output(s.ad_value(1940), s.ad_value(1929), s.ad_value(1929), 0.5), 1.0, 1.0);s.store_div_scaled_product_indices(1861, 1929, 1850, 1.0, 1942, 1.0);s.store_mul_product3_indices(827, 1860, 1917, 1850, 1844, 1.0);}
        s.store_scalar(1944, 0.0);s.store_scalar(1945, 0.0);s.store_scalar(1862, 0.0);s.store_scalar(1863, 0.0);s.b[2217] = (((((p.p40 != 0.0) && ((s.v[235] > 0.0) || (s.v[236] > 0.0))) || ((p.p42 != 0.0) && ((s.v[245] > 0.0) || (s.v[246] > 0.0)))) || (s.v[260] > 0.0)) || (s.v[261] > 0.0));s.store_scalar(2217, if s.b[2217] { 1.0 } else { 0.0 });
        if s.b[2217] {s.store_scaled_add_mixed_ia(1943, 1801, A::sqrt(A::add(A::square(s.ad_value(1801)), s.ad_value(778))), 0.5);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_99(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if s.b[2217] {s.store_add_mixed_ai(1944, A::add_scaled_inputs_product(s.ad_value(1943), -1.0, s.ad_value(773), (-0.5), s.ad_value(771), A::sqrt(A::add_scaled_inputs3(s.ad_value(1943), 1.0, s.ad_value(773), 0.25, s.ad_value(779), 1.0)), 1.0), 780);s.store_scaled_add_mixed_ia(1943, 1802, A::sqrt(A::add(A::square(s.ad_value(1802)), s.ad_value(781))), 0.5);s.store_add_mixed_ai(1945, A::add_scaled_inputs_product(s.ad_value(1943), -1.0, s.ad_value(774), (-0.5), s.ad_value(772), A::sqrt(A::add_scaled_inputs3(s.ad_value(1943), 1.0, s.ad_value(774), 0.25, s.ad_value(782), 1.0)), 1.0), 783);s.store_scaled_add(1862, 1801, 1944, (-s.v[355]));s.store_scaled_add(1863, 1802, 1945, (-s.v[355]));}
        s.store_scalar(828, 0.0);s.store_scalar(829, 0.0);s.store_scalar(1972, 0.0);s.store_scalar(832, 0.0);s.store_scalar(830, 0.0);s.store_scalar(831, 0.0);s.b[2218] = (p.p40 != 0.0);s.store_scalar(2218, if s.b[2218] { 1.0 } else { 0.0 });s.b[2219] = (s.v[235] > 0.0);s.store_scalar(2219, if s.b[2219] { 1.0 } else { 0.0 });
        if (s.b[2218] && s.b[2219]) {s.store_mul_sqrt_mixed_ia(1946, 784, A::offset(A::square(s.ad_value(1862)), 1e-6));}
        s.b[2220] = (s.v[241] < 0.0);s.store_scalar(2220, if s.b[2220] { 1.0 } else { 0.0 });
        if ((s.b[2218] && s.b[2219]) && s.b[2220]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1946, 1946, 0.5, 790, 0.5, 1946, 790, 1e-6, (-0.5));}
        if (s.b[2218] && s.b[2219]) {s.store_mul_scale_offset_mixed_ia(1929, 787, A::mul(s.ad_value(1946), A::add_scaled_product(s.ad_value(240), 1.0, s.ad_value(241), s.ad_value(1946), 1.0)), 1.0, (-1.5));}
        s.b[2221] = (s.v[1929] > 0.0);s.store_scalar(2221, if s.b[2221] { 1.0 } else { 0.0 });
        if ((s.b[2218] && s.b[2219]) && s.b[2221]) {s.store_offset_mul_offset_rhs_mixed_ia(1947, 1929, A::mul_scaled_output(s.ad_value(1929), A::scale_offset(s.ad_value(1929), 0.3333333333333333, 1.0), 0.5), 1.0, 1.0);}
        s.b[2222] = (s.v[1929] > (-230.25850929940458));s.store_scalar(2222, if s.b[2222] { 1.0 } else { 0.0 });
        if (((s.b[2218] && s.b[2219]) && (!s.b[2221])) && s.b[2222]) {s.store_exp(1947, 1929);}
        if (((s.b[2218] && s.b[2219]) && (!s.b[2221])) && (!s.b[2222])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1947, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2218] && s.b[2219]) {s.store_offset(1948, 1944, 3.0);s.store_primal_sub_from_scalar(1949, (-3.0), 233);s.store_scale(1950, 823, 30.0);s.store_scalar(807, (4.0 - 0.9));s.store_add(808, 1948, 1950);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1929, 2.0, 807, A::sub(s.ad_value(808), A::sqrt(A::sub(A::square(s.ad_value(808)), A::mul3(s.ad_value(807), s.ad_value(1948), s.ad_value(1950))))));s.store_scalar(807, (4.0 - 0.3));s.store_add(808, 1949, 1929);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1951, 2.0, 807, A::add(s.ad_value(808), A::sqrt(A::sub(A::square(s.ad_value(808)), A::mul3(s.ad_value(807), s.ad_value(1949), s.ad_value(1929))))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_100(
        s: &mut Scratch,
    ) {
        if (s.b[2218] && s.b[2219]) {s.store_mul3_lhs(828, 235, 1947, 1951);}
        s.b[2223] = (s.v[236] > 0.0);s.store_scalar(2223, if s.b[2223] { 1.0 } else { 0.0 });
        if (s.b[2218] && s.b[2223]) {s.store_mul_sqrt_mixed_ia(1946, 784, A::offset(A::square(s.ad_value(1863)), 1e-6));}
        s.b[2224] = (s.v[243] < 0.0);s.store_scalar(2224, if s.b[2224] { 1.0 } else { 0.0 });
        if ((s.b[2218] && s.b[2223]) && s.b[2224]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1946, 1946, 0.5, 791, 0.5, 1946, 791, 1e-6, (-0.5));}
        if (s.b[2218] && s.b[2223]) {s.store_mul_scale_offset_mixed_ia(1929, 788, A::mul(s.ad_value(1946), A::add_scaled_product(s.ad_value(242), 1.0, s.ad_value(243), s.ad_value(1946), 1.0)), 1.0, (-1.5));}
        s.b[2225] = (s.v[1929] > 0.0);s.store_scalar(2225, if s.b[2225] { 1.0 } else { 0.0 });
        if ((s.b[2218] && s.b[2223]) && s.b[2225]) {s.store_offset_mul_offset_rhs_mixed_ia(1947, 1929, A::mul_scaled_output(s.ad_value(1929), A::scale_offset(s.ad_value(1929), 0.3333333333333333, 1.0), 0.5), 1.0, 1.0);}
        s.b[2226] = (s.v[1929] > (-230.25850929940458));s.store_scalar(2226, if s.b[2226] { 1.0 } else { 0.0 });
        if (((s.b[2218] && s.b[2223]) && (!s.b[2225])) && s.b[2226]) {s.store_exp(1947, 1929);}
        if (((s.b[2218] && s.b[2223]) && (!s.b[2225])) && (!s.b[2226])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1947, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2218] && s.b[2223]) {s.store_offset(1948, 1945, 3.0);s.store_primal_sub_from_scalar(1949, (-3.0), 233);s.store_scale(1950, 826, 30.0);s.store_scalar(807, (4.0 - 0.9));s.store_add(808, 1948, 1950);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1929, 2.0, 807, A::sub(s.ad_value(808), A::sqrt(A::sub(A::square(s.ad_value(808)), A::mul3(s.ad_value(807), s.ad_value(1948), s.ad_value(1950))))));s.store_scalar(807, (4.0 - 0.3));s.store_add(808, 1949, 1929);s.store_mul_div_from_scalar_lhs_ad_mixed_ia(1951, 2.0, 807, A::add(s.ad_value(808), A::sqrt(A::sub(A::square(s.ad_value(808)), A::mul3(s.ad_value(807), s.ad_value(1949), s.ad_value(1929))))));s.store_mul3_lhs(829, 236, 1947, 1951);}
        s.b[2227] = (s.v[234] > 0.0);s.store_scalar(2227, if s.b[2227] { 1.0 } else { 0.0 });s.b[2228] = (s.v[1813] <= 0.0);s.store_scalar(2228, if s.b[2228] { 1.0 } else { 0.0 });
        if ((s.b[2218] && s.b[2227]) && s.b[2228]) {s.store_offset(1929, 766, 1.0);s.store_div_scaled_product_mixed_aii(1930, A::sqrt(s.ad_value(1929)), 815, 1.0, 1839, 1.0);s.store_add_mixed_ai(1931, A::square(s.ad_value(1930)), 1929);s.store_scale(1929, 1930, 2.0);s.store_div_scaled_product3_mixed_iiia(1842, 1839, 1809, 1929, 1.0, A::add(A::sqrt(A::sub(s.ad_value(1931), s.ad_value(1929))), A::sqrt(A::add(s.ad_value(1931), s.ad_value(1929)))), 1.0);}
        s.b[2229] = ((s.v[1843] - s.v[1842]) > (-230.25850929940458));s.store_scalar(2229, if s.b[2229] { 1.0 } else { 0.0 });
        if ((s.b[2218] && s.b[2227]) && s.b[2229]) {s.store_exp_sub(1929, 1843, 1842);}
        if ((s.b[2218] && s.b[2227]) && (!s.b[2229])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1929, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1843), s.ad_value(1842)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_101(
        s: &mut Scratch,
    ) {
        if (s.b[2218] && s.b[2227]) {s.store_add_scaled_product_mixed_iia(1952, 1932, 1.0, 1808, A::sub_scaled_inputs(s.ad_value(1843), 0.5, A::ln_scaled_input(A::offset(s.ad_value(1929), 1.0), 0.5), 1.0), 1.0);s.store_mul(1953, 233, 1808);s.store_add(1954, 1856, 1953);s.store_scaled_sub_mixed_ia(1955, 1954, A::sqrt_square_offset(A::neg(s.ad_value(1954)), 0.01), 0.5);s.store_mul_sqrt_mixed_ia(1946, 784, A::offset(A::square(s.ad_value(1856)), 1e-6));}
        s.b[2230] = (s.v[239] < 0.0);s.store_scalar(2230, if s.b[2230] { 1.0 } else { 0.0 });
        if ((s.b[2218] && s.b[2227]) && s.b[2230]) {s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1946, 1946, 0.5, 789, 0.5, 1946, 789, 1e-6, (-0.5));}
        if (s.b[2218] && s.b[2227]) {s.store_add_scaled_product_mixed_iai(1956, 1846, 1.0, A::add_scaled_inputs3(s.ad_value(1955), 1.0, s.ad_value(731), (-1.0), s.ad_value(1952), -1.0), 1809, 1.0);}
        s.b[2231] = (((s.v[1956]) as f64).abs() < 230.25850929940458);s.store_scalar(2231, if s.b[2231] { 1.0 } else { 0.0 });
        if ((s.b[2218] && s.b[2227]) && s.b[2231]) {s.store_exp(1957, 1956);}
        s.b[2232] = (s.v[1956] < 0.0);s.store_scalar(2232, if s.b[2232] { 1.0 } else { 0.0 });
        if (((s.b[2218] && s.b[2227]) && (!s.b[2231])) && s.b[2232]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1957, 1e-100, (-230.25850929940458), 1956, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2218] && s.b[2227]) && (!s.b[2231])) && (!s.b[2232])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(1957, 1956, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2218] && s.b[2227]) {s.store_mul_scale_offset_mixed_ia(1956, 1809, A::add_scaled_inputs3(s.ad_value(814), 1.0, s.ad_value(1932), 1.0, s.ad_value(1952), -1.0), -1.0, 0.0);}
        s.b[2233] = (((s.v[1956]) as f64).abs() < 230.25850929940458);s.store_scalar(2233, if s.b[2233] { 1.0 } else { 0.0 });
        if ((s.b[2218] && s.b[2227]) && s.b[2233]) {s.store_exp(1929, 1956);}
        s.b[2234] = (s.v[1956] < 0.0);s.store_scalar(2234, if s.b[2234] { 1.0 } else { 0.0 });
        if (((s.b[2218] && s.b[2227]) && (!s.b[2233])) && s.b[2234]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1929, 1e-100, (-230.25850929940458), 1956, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2218] && s.b[2227]) && (!s.b[2233])) && (!s.b[2234])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(1929, 1956, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2218] && s.b[2227]) {s.store_mul(1958, 1957, 1929);s.store_mul_scale_offset_mixed_ia(1929, 786, A::mul(s.ad_value(1946), A::add_scaled_product(s.ad_value(238), 1.0, s.ad_value(239), s.ad_value(1946), 1.0)), 1.0, (-1.5));}
        s.b[2235] = (s.v[1929] > 0.0);s.store_scalar(2235, if s.b[2235] { 1.0 } else { 0.0 });
        if ((s.b[2218] && s.b[2227]) && s.b[2235]) {s.store_offset_mul_offset_rhs_mixed_ia(1947, 1929, A::mul_scaled_output(s.ad_value(1929), A::scale_offset(s.ad_value(1929), 0.3333333333333333, 1.0), 0.5), 1.0, 1.0);}
        s.b[2236] = (s.v[1929] > (-230.25850929940458));s.store_scalar(2236, if s.b[2236] { 1.0 } else { 0.0 });
        if (((s.b[2218] && s.b[2227]) && (!s.b[2235])) && s.b[2236]) {s.store_exp(1947, 1929);}
        if (((s.b[2218] && s.b[2227]) && (!s.b[2235])) && (!s.b[2236])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1947, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2218] && s.b[2227]) {s.store_mul_ad_product_rhs_mixed_ia(1959, 234, 1947, A::ln(A::div_scaled_offset_numerator(s.ad_value(1957), 1.0, 1.0, A::offset(s.ad_value(1958), 1.0), 1.0)));}
        s.b[2237] = ((s.v[1813] <= 0.0) || ((s.v[238] == 0.0) && (s.v[239] == 0.0)));s.store_scalar(2237, if s.b[2237] { 1.0 } else { 0.0 });
        if ((s.b[2218] && s.b[2227]) && s.b[2237]) {s.store_scalar(1966, 1.0);s.store_scalar(1967, 0.5);}
        if ((s.b[2218] && s.b[2227]) && (!s.b[2237])) {s.store_add_scaled_product_indices(1929, 238, 1.0, 239, 1946, 2.0);s.store_div_scaled_value_by_product_indices(1960, 244, 1.0, 1929, 786, 1.0);s.store_scaled_div(1961, 1844, 1960, 0.5);s.store_div(1962, 1960, 1861);s.store_mul_sub_from_scalar_scaled_rhs_scaled_output(1963, 1962, 1.0, 1962, 1.0, 0.5);s.store_sub_from_scalar_scaled_input(1964, 0.5, 1963, 3.0);}
        s.b[2238] = (s.v[1961] < 0.001);s.store_scalar(2238, if s.b[2238] { 1.0 } else { 0.0 });
        if (((s.b[2218] && s.b[2227]) && (!s.b[2237])) && s.b[2238]) {s.store_square(1965, 1961);s.store_offset_mul_ad(1966, s.ad_value(1965), A::add_scaled_product(A::scale_offset(s.ad_value(1962), 0.3333333333333333, 0.16666666666666666), 1.0, s.ad_value(1965), A::scale_offset(s.ad_value(1962), 0.2, 0.05), 0.16666666666666666), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_102(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2218] && s.b[2227]) && (!s.b[2237])) && s.b[2238]) {s.store_add_scaled_offset_product_rhs_mixed_iia(1967, 1966, 0.5, 1961, A::mul(s.ad_value(1965), A::add_scaled_offset_product_rhs(A::scaled_offset(s.ad_value(1963), 0.25, 0.4), 1.0, s.ad_value(1965), s.ad_value(1963), 0.125, 0.0285714285714)), 1.0, (-0.16666666666666666));}
        if (((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) {s.store_div_from_scalar(1968, 1.0, 1961);}
        s.b[2239] = (((s.v[1961]) as f64).abs() < 230.25850929940458);s.store_scalar(2239, if s.b[2239] { 1.0 } else { 0.0 });
        if ((((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) && s.b[2239]) {s.store_exp(1969, 1961);}
        s.b[2240] = (s.v[1961] < 0.0);s.store_scalar(2240, if s.b[2240] { 1.0 } else { 0.0 });
        if (((((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) && (!s.b[2239])) && s.b[2240]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1969, 1e-100, (-230.25850929940458), 1961, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) && (!s.b[2239])) && (!s.b[2240])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(1969, 1961, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (((s.b[2218] && s.b[2227]) && (!s.b[2237])) && (!s.b[2238])) {s.store_div_from_scalar(1970, 1.0, 1969);s.store_sub(1929, 1969, 1970);s.store_add(1931, 1969, 1970);s.store_add_scaled_products_mixed_aiii(1966, A::mul_sub_from_scalar_lhs(1.0, s.ad_value(1962), s.ad_value(1929)), 1968, 0.5, 1962, 1931, 0.5);s.store_scaled_sub_ad(1967, A::add_scaled_product(s.ad_value(1966), 1.0, s.ad_value(1929), A::sub(s.ad_value(1963), A::mul3(s.ad_value(1964), s.ad_value(1968), s.ad_value(1968))), (-1.0)), A::mul3(s.ad_value(1964), s.ad_value(1931), s.ad_value(1968)), 0.5);}
        if (s.b[2218] && s.b[2227]) {s.store_scaled_offset_ad(1971, A::div(s.ad_value(1813), A::sqrt_square_offset(s.ad_value(1813), 1e-6)), 1.0, 0.5);s.store_mul3_lhs(1972, 1959, 1966, 1971);s.store_mul3_lhs(831, 1959, 1967, 1971);s.store_sub(830, 1972, 831);s.store_mul_ad_product_rhs_mixed_ia(832, 1959, 1966, A::sub_from_scalar(1.0, s.ad_value(1971)));}
        s.store_scalar(834, 0.0);s.store_scalar(833, 0.0);s.b[2241] = (p.p42 != 0.0);s.store_scalar(2241, if s.b[2241] { 1.0 } else { 0.0 });s.b[2242] = ((s.v[246] > 0.0) && (s.v[1863] < 0.0));s.store_scalar(2242, if s.b[2242] { 1.0 } else { 0.0 });
        if (s.b[2241] && s.b[2242]) {s.store_sqrt_offset_ad(1973, A::add_scaled_square_product(s.ad_value(1863), 1.0, A::square(s.ad_value(252)), A::square(s.ad_value(825)), 1.0), 1e-6);s.store_div_scaled_inputs_indices(1929, 796, -1.0, 1973, 1.0);}
        s.b[2243] = (s.v[1929] > (-230.25850929940458));s.store_scalar(2243, if s.b[2243] { 1.0 } else { 0.0 });
        if ((s.b[2241] && s.b[2242]) && s.b[2243]) {s.store_exp(1931, 1929);}
        if ((s.b[2241] && s.b[2242]) && (!s.b[2243])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1931, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2241] && s.b[2242]) {s.store_mul_ad_affine_product_lhs(834, s.ad_value(794), A::mul3(s.ad_value(825), s.ad_value(1863), s.ad_value(1973)), -1.0, 0.0, 1931);}
        s.b[2244] = ((s.v[245] > 0.0) && (s.v[1862] < 0.0));s.store_scalar(2244, if s.b[2244] { 1.0 } else { 0.0 });
        if (s.b[2241] && s.b[2244]) {s.store_sqrt_offset_ad(1974, A::add_scaled_square_product(s.ad_value(1862), 1.0, A::square(s.ad_value(251)), A::square(s.ad_value(824)), 1.0), 1e-6);s.store_div_scaled_inputs_indices(1929, 795, -1.0, 1974, 1.0);}
        s.b[2245] = (s.v[1929] > (-230.25850929940458));s.store_scalar(2245, if s.b[2245] { 1.0 } else { 0.0 });
        if ((s.b[2241] && s.b[2244]) && s.b[2245]) {s.store_exp(1931, 1929);}
        if ((s.b[2241] && s.b[2244]) && (!s.b[2245])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(1931, 1e-100, (-230.25850929940458), 1929, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2241] && s.b[2244]) {s.store_mul_ad_affine_product_lhs(833, s.ad_value(793), A::mul3(s.ad_value(824), s.ad_value(1862), s.ad_value(1974)), -1.0, 0.0, 1931);}
        s.copy_ad(1978, 1916);s.store_scalar(1864, 0.0);
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_103(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        s.store_scalar(1865, 0.0);s.store_scalar(1866, 0.0);s.store_scalar(1867, 1e-40);s.store_scalar(1868, 1.0);s.store_scalar(835, 0.0);s.b[2246] = ((p.p46 != 0.0) && (s.v[285] > 0.0));s.store_scalar(2246, if s.b[2246] { 1.0 } else { 0.0 });
        if s.b[2246] {s.store_add_scaled_inputs4_mixed_iiai(1929, 817, 0.5, 816, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(817), s.ad_value(816))), s.ad_value(753))), (-0.5), 751, 1.0);s.store_add_scaled_inputs4_mixed_iiai(1975, 816, 1.0, 1929, (-0.5), A::sqrt(A::add(A::square(s.ad_value(1929)), s.ad_value(752))), (-(-0.5)), 755, 1.0);s.store_add_scaled_inputs3_indices(1976, 1975, 1.0, 815, 0.5, 819, (-0.5));s.store_mul_ad_product_rhs(1977, 287, A::offset(A::mul(s.ad_value(289), s.ad_value(819)), 1.0), A::offset(A::mul(s.ad_value(288), s.ad_value(1976)), 1.0));s.store_mul_scale_offset_indices(1978, 1924, 1977, 1.0, 1.0);s.store_div_from_scalar(1979, 1.0, 1978);s.store_div_scaled_value_offset_denominator(1980, s.ad_value(819), 2.0, A::sqrt_product_offset(s.ad_value(291), s.ad_value(819), 1.0), 1.0, 1.0);s.store_mul_ad_product_rhs_mixed_ia(1981, 290, 1980, A::offset(A::mul(s.ad_value(292), s.ad_value(1976)), 1.0));s.store_mul_add_scaled_inputs3_offset_rhs_indices(1864, 1979, 818, 1.0, 1981, 1.0, 714, -1.0, 0.0);s.store_mul(1982, 1979, 749);s.store_scaled_ln_ad(1983, A::add(A::div(s.ad_value(1982), s.ad_value(750)), A::sqrt(s.ad_value(1982))), 2.0);s.store_mul(1984, 1979, 1975);s.store_add(1989, 1982, 1984);s.store_add_scaled_product_mixed_iia(1990, 1989, 1.0, 750, A::sqrt(s.ad_value(1989)), 1.0);s.store_add(1991, 1990, 1983);s.store_offset_div_scaled_inputs_sqrt_rhs(1992, 750, 1.0, 1989, 2.0, 1.0);s.store_div_from_scalar(1993, 1.0, 1992);s.store_sub(1994, 1864, 1991);}
        s.b[2247] = (s.v[1994] > (-12.0));s.store_scalar(2247, if s.b[2247] { 1.0 } else { 0.0 });
        if (s.b[2246] && s.b[2247]) {s.store_offset_add(1995, 1994, 1926, (-1.0));s.store_scaled_add_mixed_ia(1996, 1995, A::sqrt_square_offset(s.ad_value(1995), 10.0), 0.5);s.store_add_mixed_ai(1997, A::add_scaled_product(s.ad_value(1994), 1.0, s.ad_value(1992), A::ln(s.ad_value(1996)), (-1.0)), 1926);s.store_scaled_add_mixed_ia(1998, 1997, A::sqrt_square_offset(s.ad_value(1997), 2.0), 0.5);}
        s.b[2248] = ((s.v[1994] - s.v[1998]) < 230.25850929940458);s.store_scalar(2248, if s.b[2248] { 1.0 } else { 0.0 });
        if ((s.b[2246] && s.b[2247]) && s.b[2248]) {s.store_exp_sub(1999, 1994, 1998);}
        if ((s.b[2246] && s.b[2247]) && (!s.b[2248])) {s.store_scaled_softlimit_poly_offset_lhs_ad(1999, A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2246] && s.b[2247]) {s.store_mul(2000, 1925, 1999);s.store_pow_indices(2001, 2000, 1993);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_104(
        s: &mut Scratch,
    ) {
        if (s.b[2246] && s.b[2247]) {s.store_add_scaled_square_product_mixed_iai(2002, 1992, 1.0, A::add_scaled_inputs3(s.ad_value(1998), 2.0, s.ad_value(1992), 2.0, s.ad_value(2001), -1.0), 2001, 1.0);s.store_mul_scale_offset_mixed_ia(2003, 1992, A::div_scaled_inputs2(A::sqrt(s.ad_value(2002)), 1.0, s.ad_value(1992), (-1.0), s.ad_value(2001), 1.0), 1.0, (-1.0));s.store_sub(1985, 1998, 2003);}
        s.b[2249] = ((s.v[1993] * (s.v[1994] + s.v[1926])) > (-230.25850929940458));s.store_scalar(2249, if s.b[2249] { 1.0 } else { 0.0 });
        if ((s.b[2246] && (!s.b[2247])) && s.b[2249]) {s.store_exp_ad(1985, A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))));}
        if ((s.b[2246] && (!s.b[2247])) && (!s.b[2249])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1985, 1e-100, (-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if s.b[2246] {s.store_mul_add_rhs(1986, 1979, 1841, 1975);}
        s.b[2250] = ((s.v[1985] < 0.001) && (s.v[1841] < 1e-6));s.store_scalar(2250, if s.b[2250] { 1.0 } else { 0.0 });s.b[2251] = (((-s.v[1986]) + s.v[1984]) > (-230.25850929940458));s.store_scalar(2251, if s.b[2251] { 1.0 } else { 0.0 });
        if ((s.b[2246] && s.b[2250]) && s.b[2251]) {s.store_exp_sub(1929, 1984, 1986);}
        if ((s.b[2246] && s.b[2250]) && (!s.b[2251])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1929, 1e-100, (-230.25850929940458), A::sub(s.ad_value(1984), s.ad_value(1986)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2246] && s.b[2250]) {s.store_mul_scale_offset_indices(1865, 1985, 1929, 1.0, (-1.0));s.store_add(1987, 1865, 1985);}
        if (s.b[2246] && (!s.b[2250])) {s.store_add(1989, 1982, 1986);s.store_add_scaled_product_mixed_iia(1990, 1989, 1.0, 750, A::sqrt(s.ad_value(1989)), 1.0);s.store_add(1991, 1990, 1983);s.store_offset_div_scaled_inputs_sqrt_rhs(1992, 750, 1.0, 1989, 2.0, 1.0);s.store_div_from_scalar(1993, 1.0, 1992);s.store_sub(1994, 1864, 1991);}
        s.b[2252] = (s.v[1994] > (-12.0));s.store_scalar(2252, if s.b[2252] { 1.0 } else { 0.0 });
        if ((s.b[2246] && (!s.b[2250])) && s.b[2252]) {s.store_offset_add(1995, 1994, 1926, (-1.0));s.store_scaled_add_mixed_ia(1996, 1995, A::sqrt_square_offset(s.ad_value(1995), 10.0), 0.5);s.store_add_mixed_ai(1997, A::add_scaled_product(s.ad_value(1994), 1.0, s.ad_value(1992), A::ln(s.ad_value(1996)), (-1.0)), 1926);s.store_scaled_add_mixed_ia(1998, 1997, A::sqrt_square_offset(s.ad_value(1997), 2.0), 0.5);}
        s.b[2253] = ((s.v[1994] - s.v[1998]) < 230.25850929940458);s.store_scalar(2253, if s.b[2253] { 1.0 } else { 0.0 });
        if (((s.b[2246] && (!s.b[2250])) && s.b[2252]) && s.b[2253]) {s.store_exp_sub(1999, 1994, 1998);}
        if (((s.b[2246] && (!s.b[2250])) && s.b[2252]) && (!s.b[2253])) {s.store_scaled_softlimit_poly_offset_lhs_ad(1999, A::sub(s.ad_value(1994), s.ad_value(1998)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2246] && (!s.b[2250])) && s.b[2252]) {s.store_mul(2000, 1925, 1999);s.store_pow_indices(2001, 2000, 1993);s.store_add_scaled_square_product_mixed_iai(2002, 1992, 1.0, A::add_scaled_inputs3(s.ad_value(1998), 2.0, s.ad_value(1992), 2.0, s.ad_value(2001), -1.0), 2001, 1.0);s.store_mul_scale_offset_mixed_ia(2003, 1992, A::div_scaled_inputs2(A::sqrt(s.ad_value(2002)), 1.0, s.ad_value(1992), (-1.0), s.ad_value(2001), 1.0), 1.0, (-1.0));s.store_sub(1987, 1998, 2003);}
        s.b[2254] = ((s.v[1993] * (s.v[1994] + s.v[1926])) > (-230.25850929940458));s.store_scalar(2254, if s.b[2254] { 1.0 } else { 0.0 });
        if (((s.b[2246] && (!s.b[2250])) && (!s.b[2252])) && s.b[2254]) {s.store_exp_ad(1987, A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))));}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_105(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (((s.b[2246] && (!s.b[2250])) && (!s.b[2252])) && (!s.b[2254])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1987, 1e-100, (-230.25850929940458), A::mul(s.ad_value(1993), A::add(s.ad_value(1994), s.ad_value(1926))), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2246] && (!s.b[2250])) {s.store_sub(1865, 1987, 1985);}
        if s.b[2246] {s.store_scaled_add(1866, 1987, 1985, 0.5);}
        if s.b[2246] {
            if ((s.v[1864] - s.v[1866]) > 1e-40) {
                s.store_sub(1867, 1864, 1866);
            } else {
                s.store_scalar(1867, 1e-40);
            }
        }
        if s.b[2246] {s.store_sub_from_scalar_ad(1868, 1.0, A::div_scaled_inputs(s.ad_value(750), 0.5, A::sqrt(A::add_scaled_inputs(s.ad_value(1867), 1.0, s.ad_value(1925), 0.25)), 1.0));s.store_div_scaled_product3_mixed_aaii(835, A::mul3_scaled_output(s.ad_value(1918), s.ad_value(1978), s.ad_value(1978), -1.0), A::offset(A::mul(s.ad_value(1868), s.ad_value(1866)), 1.0), 1865, 1.0, 1853, 1.0);}
        s.store_scalar(1869, 0.0);s.store_scalar(836, 0.0);s.b[2255] = ((s.v[1813] > 0.0) && (p.p41 != 0.0));s.store_scalar(2255, if s.b[2255] { 1.0 } else { 0.0 });
        if s.b[2255] {s.store_add_scaled_product_indices(1988, 815, 1.0, 230, 1844, (-1.0));}
        s.b[2256] = (s.v[1988] > 0.0);s.store_scalar(2256, if s.b[2256] { 1.0 } else { 0.0 });
        if (s.b[2255] && s.b[2256]) {s.store_mul_div_scaled_offset_numerator_rhs(1931, 713, A::mul(s.ad_value(231), A::sub(A::sqrt(A::add(s.ad_value(717), s.ad_value(1932))), s.ad_value(725))), 1.0, 1.0, A::offset(s.ad_value(1988), 1e-30), 1.0);}
        s.b[2257] = ((((-s.v[1931])) as f64).abs() < 230.25850929940458);s.store_scalar(2257, if s.b[2257] { 1.0 } else { 0.0 });
        if ((s.b[2255] && s.b[2256]) && s.b[2257]) {s.store_exp_neg_input(1929, 1931);}
        s.b[2258] = ((-s.v[1931]) < 0.0);s.store_scalar(2258, if s.b[2258] { 1.0 } else { 0.0 });
        if (((s.b[2255] && s.b[2256]) && (!s.b[2257])) && s.b[2258]) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(1929, 1e-100, (-230.25850929940458), A::neg(s.ad_value(1931)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2255] && s.b[2256]) && (!s.b[2257])) && (!s.b[2258])) {s.store_scaled_softlimit_poly_offset_lhs_ad(1929, A::neg(s.ad_value(1931)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if (s.b[2255] && s.b[2256]) {s.store_mul3_lhs(1869, 227, 1988, 1929);s.store_mul_add_rhs(836, 1869, 827, 835);}
        s.b[2259] = (s.v[836] > (0.5 * s.v[232]));s.store_scalar(2259, if s.b[2259] { 1.0 } else { 0.0 });
        if ((s.b[2255] && s.b[2256]) && s.b[2259]) {s.store_offset_div_scaled_inputs_indices(1929, 836, 2.0, 232, 1.0, (-1.0));s.store_mul_scaled_offset_ad_rhs(836, 232, 0.5, A::div(s.ad_value(1929), A::sqrt_square_offset(s.ad_value(1929), 1.0)), 1.0);}
        s.b[2453] = (((p.p45 == 1.0) || (p.p47 > 0.0)) || (p.p48 > 0.0));s.store_scalar(2453, if s.b[2453] { 1.0 } else { 0.0 });s.b[2454] = ((p.p45 > 0.0) || (p.p47 > 0.0));s.store_scalar(2454, if s.b[2454] { 1.0 } else { 0.0 });
        if (s.b[2453] && s.b[2454]) {s.copy_ad(2294, 717);s.copy_ad(2295, 727);s.copy_ad(2296, 718);s.copy_ad(2297, 1804);s.copy_ad(2298, 1805);s.store_scalar(2302, 0.0);}
        s.b[2455] = (p.p47 > 0.0);s.store_scalar(2455, if s.b[2455] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2454]) && s.b[2455]) {s.store_add_scaled_inputs4_mixed_iiai(2297, 817, 0.5, 816, 0.5, A::sqrt(A::add(A::square(A::sub(s.ad_value(817), s.ad_value(816))), s.ad_value(738))), (-0.5), 736, 1.0);s.store_add_scaled_inputs4_mixed_iiai(1870, 816, 1.0, 2297, (-0.5), A::sqrt(A::add(A::square(s.ad_value(2297)), s.ad_value(737))), (-(-0.5)), 739, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_106(
        s: &mut Scratch,
    ) {
        if ((s.b[2453] && s.b[2454]) && s.b[2455]) {s.copy_ad(2298, 1870);s.copy_ad(2294, 734);s.copy_ad(2295, 737);s.copy_ad(2296, 735);}
        if (s.b[2453] && s.b[2454]) {s.store_add_scaled_inputs3_indices(2301, 818, 1.0, 2302, (-1.0), 701, -1.0);s.store_add_scaled_inputs3_indices(2303, 2298, 1.0, 815, 0.5, 819, (-0.5));s.store_scalar(2315, 1.0);}
        s.b[2456] = (s.v[188] > 0.0);s.store_scalar(2456, if s.b[2456] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2454]) && s.b[2456]) {s.store_mul(2306, 2294, 362);s.store_mul(2307, 2303, 362);s.store_mul(2308, 2301, 362);s.store_offset_div_scaled_inputs_sqrt_rhs(1930, 2296, 0.5, 2306, 1.0, 1.0);s.store_add_scaled_product_mixed_iia(1931, 2306, 1.0, 2296, A::sqrt(s.ad_value(2306)), 1.0);s.store_add_scaled_inputs_product_mixed_aiai(2309, A::div_scaled_inputs2(s.ad_value(2308), 1.0, s.ad_value(1931), (-1.0), s.ad_value(1930), 1.0), 1.0, 2306, 0.5, A::offset(s.ad_value(189), 1.0), 2307, (-1.0));s.store_offset_scaled(2310, 2306, 0.5, 2.0);s.store_add(2311, 2306, 2307);s.store_sub_scaled_inputs_ad(1930, A::add_scaled_inputs_product(s.ad_value(2308), 1.0, s.ad_value(2311), (-1.0), s.ad_value(2296), A::sqrt(s.ad_value(2311)), (-1.0)), 1.0, A::ln(A::add(A::div(s.ad_value(2306), s.ad_value(2296)), A::sqrt(s.ad_value(2306)))), 2.0);s.store_add_scaled_inputs(2312, 1930, 2.0, 2310, 1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1930, 2309, 0.5, 2312, 0.5, 2309, 2312, 20.0, 0.5);s.store_add_scaled_inputs3_indices(1931, 2308, 2.0, 2307, (-2.0), 2310, -1.0);s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(2313, 1930, 0.5, 1931, 0.5, 1930, 1931, 20.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_sub_square_offset(1930, 2313, 0.5, 2310, 0.5, 2313, 2310, 5.0, (-0.5));s.store_add_scaled_inputs3_sqrt_third_mixed_iia(2314, 1930, 0.5, 2310, ((-1.0) * 0.5), A::offset(A::square(A::sub_scaled_inputs(s.ad_value(1930), 1.0, s.ad_value(2310), -1.0)), 20.0), 0.5);s.store_mul_scale_offset_mixed_ia(1931, 703, A::div(s.ad_value(2314), s.ad_value(2310)), 1.0, 1.0);}
        s.b[2457] = (s.v[1931] > (-230.25850929940458));s.store_scalar(2457, if s.b[2457] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2454]) && s.b[2456]) && s.b[2457]) {s.store_exp(2315, 1931);}
        if (((s.b[2453] && s.b[2454]) && s.b[2456]) && (!s.b[2457])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_self_offset_rhs(2315, 1e-100, (-230.25850929940458), 1931, 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (s.b[2453] && s.b[2454]) {s.store_offset_mul(2316, 702, 2315, 1.0);s.store_mul(2317, 1916, 2316);s.store_mul_ad_product_rhs(2318, 197, A::offset(A::mul(s.ad_value(199), s.ad_value(819)), 1.0), A::offset(A::mul(s.ad_value(198), s.ad_value(2303)), 1.0));s.store_mul_scale_offset_indices(2319, 2317, 2318, 1.0, 1.0);s.store_div_from_scalar(2320, 1.0, 2319);s.store_mul_sqrt_mixed_ia(2304, 2296, A::mul(s.ad_value(1916), s.ad_value(2320)));s.store_square(2305, 2304);s.store_div_from_scalar(2321, 1.0, 2305);s.store_mul(2322, 2298, 2320);s.store_mul(2323, 2301, 2320);s.store_div_scaled_value_offset_denominator(2324, s.ad_value(819), 2.0, A::sqrt_product_offset(s.ad_value(195), s.ad_value(819), 1.0), 1.0, 1.0);s.store_mul_ad_product_rhs_mixed_ia(2325, 194, 2324, A::offset(A::mul(s.ad_value(196), s.ad_value(2303)), 1.0));s.store_mul(2326, 2294, 2320);s.store_sqrt_square_add(1930, 2297, 2295);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_107(
        s: &mut Scratch,
        p: &Parameters,
    ) {
        if (s.b[2453] && s.b[2454]) {s.store_sqrt_add_ad(1931, A::square(A::sub(s.ad_value(2297), s.ad_value(2325))), s.ad_value(2295));s.store_mul_add_scaled_inputs3_offset_rhs_indices(2327, 2320, 2325, 0.5, 1930, 0.5, 1931, ((-1.0) * (0.5)), 0.0);s.store_add(2328, 2326, 2322);s.store_sub(2329, 2328, 2327);}
        s.b[2458] = (p.p45 > 0.0);s.store_scalar(2458, if s.b[2458] { 1.0 } else { 0.0 });s.b[2459] = (((s.v[2329]) as f64).abs() < 1e-5);s.store_scalar(2459, if s.b[2459] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2454]) && s.b[2458]) && s.b[2459]) {s.store_offset_ad(2330, A::mul_sub_from_scalar_rhs(s.ad_value(2304), 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2329), 1.0, A::scale(s.ad_value(2329), 0.3125), 0.5)), 1.0);}
        s.b[2460] = (s.v[2329] < 460.51701859880916);s.store_scalar(2460, if s.b[2460] { 1.0 } else { 0.0 });
        if ((((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) && s.b[2460]) {s.store_exp_neg_input(2344, 2329);}
        if ((((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) && (!s.b[2460])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2344, 1e-200, 2329, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) {s.store_scalar(1929, (if (s.v[2329] > 0.0) { 1.0 } else { (-1.0) }));}
        if (((s.b[2453] && s.b[2454]) && s.b[2458]) && (!s.b[2459])) {s.store_offset_ad(2330, A::div_scaled_product3(s.ad_value(1929), s.ad_value(2304), A::sub_from_scalar(1.0, A::mul_sub_from_scalar_rhs(s.ad_value(2344), 1.0, s.ad_value(2329))), 1.0, A::sqrt(A::mul_sub_from_scalar_rhs(s.ad_value(2329), 1.0, s.ad_value(2344))), 2.0), 1.0);}
        if ((s.b[2453] && s.b[2454]) && (!s.b[2458])) {s.store_offset_div_scaled_inputs_sqrt_rhs(2330, 2304, 0.5, 2329, 1.0, 1.0);}
        if (s.b[2453] && s.b[2454]) {s.store_add_scaled_value_products_mixed_iiaia(2331, 2329, 1.0, 2304, A::sqrt(s.ad_value(2329)), 1.0, 2330, A::ln(A::offset(s.ad_value(2330), (-1.0))), (-1.0));s.store_div_scaled_inputs2_indices(2332, 2323, 1.0, 2331, (-1.0), 2330, 1.0);s.store_mul_scaled_offset_ad_rhs(2338, 2305, 0.5, A::sqrt(A::offset(A::div_from_scalar(8.0, s.ad_value(2305)), 1.0)), (-1.0));s.store_scalar(2337, 0.0);s.store_scalar(2339, 1.0);}
        s.b[2461] = (s.v[2332] > (-30.0));s.store_scalar(2461, if s.b[2461] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2454]) && s.b[2461]) {s.store_offset_mul(2333, 2330, 2332, (-1.0));s.store_scaled_add_mixed_ia(1929, 2333, A::sqrt_square_offset(s.ad_value(2333), 10.0), 0.5);s.store_sub_mixed_ia(2334, 2332, A::ln(s.ad_value(1929)));s.store_scaled_add_mixed_ia(2335, 2334, A::sqrt_square_offset(s.ad_value(2334), 2.0), 0.5);}
        s.b[2462] = ((s.v[2332] - s.v[2335]) < 230.25850929940458);s.store_scalar(2462, if s.b[2462] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && s.b[2462]) {s.store_exp_sub(1929, 2332, 2335);}
        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && (!s.b[2462])) {s.store_scaled_softlimit_poly_offset_lhs_ad(1929, A::sub(s.ad_value(2332), s.ad_value(2335)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
        if ((s.b[2453] && s.b[2454]) && s.b[2461]) {s.store_div(2336, 1929, 2330);s.store_sub_mixed_ai(1929, A::scaled_offset(s.ad_value(2335), 1.0, 2.0), 2336);}
        s.b[2463] = (s.v[2336] > 1e-6);s.store_scalar(2463, if s.b[2463] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && s.b[2463]) {s.store_mul_scale_offset_mixed_ia(2337, 2330, A::sub(s.ad_value(2335), A::div_scaled_offset_numerator(A::sqrt_product_offset(s.ad_value(2336), s.ad_value(1929), 1.0), 1.0, (-1.0), s.ad_value(2336), 1.0)), 1.0, 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_108(
        s: &mut Scratch,
    ) {
        if (((s.b[2453] && s.b[2454]) && s.b[2461]) && (!s.b[2463])) {s.store_mul_ad_affine_product_rhs(2337, 2330, s.ad_value(2336), A::offset(A::mul_scaled_lhs(s.ad_value(1929), 0.25, s.ad_value(1929)), 1.0), 0.5, 0.0);}
        if ((s.b[2453] && s.b[2454]) && s.b[2461]) {s.store_add_scaled_inputs3_offset_mixed_iia(1929, 2323, 0.5, 2337, ((-1.0) * 0.5), A::sqrt_square_offset(A::offset(A::sub(s.ad_value(2323), s.ad_value(2337)), (-2.0)), 1.0), 0.5, (2.0 * 0.5));s.store_mul_scaled_offset_ad_rhs(2338, 2305, 0.5, A::sqrt_product_offset(A::div_from_scalar(4.0, s.ad_value(2305)), s.ad_value(1929), 1.0), (-1.0));s.store_div_add_scaled_inputs_rhs_indices(2339, 2338, 2338, 1.0, 2337, 1.0);s.store_add_scaled_product_indices(2329, 2328, 1.0, 2339, 2327, (-1.0));}
        if (s.b[2453] && s.b[2454]) {s.store_offset_scaled(2340, 2304, 0.7071067811865475, 1.0);}
        let (t1,) = {
    if (s.b[2453] && s.b[2454]) {
        let t0: f64 = (1e-5 * s.v[2340]);
        (t0,)
    } else {
        (s.v[2341],)
    }
};
        s.store_scalar(2341, t1);
        if (s.b[2453] && s.b[2454]) {s.store_div_from_scalar(2342, 1.0, 2340);s.store_scalar(2449, 0.0);s.store_scalar(2343, 0.0);}
        s.b[2464] = (s.v[2329] < 460.51701859880916);s.store_scalar(2464, if s.b[2464] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2454]) && s.b[2464]) {s.store_exp_neg_input(2344, 2329);}
        if ((s.b[2453] && s.b[2454]) && (!s.b[2464])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2344, 1e-200, 2329, (-460.51701859880916), 0.3333333333333333, (((((-460.51701859880916)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        s.b[2465] = (((s.v[2323]) as f64).abs() <= s.v[2341]);s.store_scalar(2465, if s.b[2465] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2454]) && s.b[2465]) {s.store_scaled_square(2429, 2342, (0.16666666666666666 * 0.7071067811865475));s.store_mul_ad_product_rhs_mixed_ia(2343, 2323, 2342, A::offset(A::mul3(A::mul_sub_from_scalar_rhs(s.ad_value(2323), 1.0, s.ad_value(2344)), s.ad_value(2304), s.ad_value(2429)), 1.0));}
        s.b[2466] = (s.v[2323] < (-s.v[2341]));s.store_scalar(2466, if s.b[2466] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) {s.store_neg(2431, 2323);s.store_scaled_mul(2432, 2431, 2342, 1.25);s.store_scaled_sub_offset_sqrt_square_offset(2433, 2432, 10.0, (-6.0), 64.0, 0.5);s.store_sub(2428, 2431, 2433);s.store_add_scaled_square_product_mixed_iia(2434, 2428, 1.0, 2305, A::offset(s.ad_value(2433), 1.0), 1.0);s.store_sub_scaled_inputs(2435, 2428, 2.0, 2305, 1.0);s.store_sub_ln_mul_lhs(2436, 2434, 2321, 2433);s.store_add(813, 2434, 2435);s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2436, A::sub_scaled_inputs(A::square(s.ad_value(2435)), 0.5, s.ad_value(2434), 1.0), 1.0);s.store_add_mixed_ia(2437, 2433, A::div_scaled_product3(s.ad_value(2434), s.ad_value(813), s.ad_value(2436), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436), s.ad_value(2436)), s.ad_value(2435), A::sub_scaled_inputs(A::square(s.ad_value(2435)), 0.3333333333333333, s.ad_value(2434), 1.0))), 1.0));}
        s.b[2467] = (s.v[2437] < 230.25850929940458);s.store_scalar(2467, if s.b[2467] { 1.0 } else { 0.0 });
        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) && s.b[2467]) {s.store_exp(2438, 2437);}
        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) && (!s.b[2467])) {s.store_scaled_softlimit_poly_offset_lhs_ad_rhs(2438, 2437, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0, 1e100);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_109(
        s: &mut Scratch,
    ) {
        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && s.b[2466]) {s.store_div_from_scalar(2439, 1.0, 2438);s.store_div_from_scalar_offset_square(2428, 1.0, 2437, 2.0);s.store_mul_square_lhs(2440, 2437, 2428);s.store_mul3_affine_lhs(2441, 2437, 2428, 4.0, 0.0, 2428);s.store_mul_ad_product_lhs_mixed_ai(2442, A::sub_scaled_inputs(s.ad_value(2428), 8.0, s.ad_value(2440), 12.0), 2428, 2428);s.store_sub(2428, 2431, 2437);s.store_mul(2429, 2344, 2439);s.store_add_scaled_product_mixed_iia(2443, 2428, 2.0, 2305, A::add_scaled_inputs3_offset(s.ad_value(2438), 1.0, s.ad_value(2429), (-1.0), A::mul_sub_from_scalar_rhs(s.ad_value(2344), 1.0, s.ad_value(2441)), 1.0, (-1.0)), 1.0);s.store_add_scaled_square_product_mixed_iia(2444, 2428, 1.0, 2305, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2438), 1.0, s.ad_value(2437), (-1.0), s.ad_value(2429), 1.0, (-1.0)), 1.0, s.ad_value(2344), A::sub(A::offset(s.ad_value(2437), (-1.0)), s.ad_value(2440)), 1.0), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2428, 2.0, 2305, A::add_scaled_inputs_product(s.ad_value(2438), 1.0, s.ad_value(2429), 1.0, s.ad_value(2344), s.ad_value(2442), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2428, 2443, 1.0, 2444, 2428, (-2.0));s.store_sub_scaled_inputs_mixed_ia(2343, 2437, -1.0, A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0);}
        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {s.store_div_from_scalar_offset_scaled_input(2445, 1.0, 2304, 0.7324648775608221, 1.25);s.store_mul_scale_offset_mixed_ia(2446, 2445, A::mul_scaled_lhs(s.ad_value(2340), 1.25, s.ad_value(2445)), 1.0, (-1.0));s.store_mul_ad_product_rhs_mixed_ia(2447, 2323, 2342, A::offset(A::mul(s.ad_value(2446), s.ad_value(2323)), 1.0));}
        s.b[2468] = ((-s.v[2447]) > (-230.25850929940458));s.store_scalar(2468, if s.b[2468] { 1.0 } else { 0.0 });
        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && s.b[2468]) {s.store_exp_neg_input(2428, 2447);}
        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2468])) {s.store_div_from_scalar_offset_mul_sub_from_scalar_lhs_ad_self_offset_rhs(2428, 1e-100, (-230.25850929940458), A::neg(s.ad_value(2447)), 0.3333333333333333, 1.0, 0.5, 1.0, 1.0);}
        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {s.store_sub_from_scalar(2448, 1.0, 2428);s.store_add_scaled_inputs_product_mixed_iiia(2449, 2323, 1.0, 2305, 0.5, 2304, A::sqrt(A::add_scaled_inputs3(s.ad_value(2323), 1.0, s.ad_value(2305), 0.25, s.ad_value(2448), -1.0)), (-1.0));s.store_offset(2450, 2329, 3.0);s.store_sub_ad(2433, A::add_scaled_inputs3(s.ad_value(2449), 0.5, s.ad_value(2450), 0.5, A::sqrt_square_offset(A::sub(s.ad_value(2449), s.ad_value(2450)), 5.0), (-0.5)), A::sub_scaled_inputs(s.ad_value(2450), 0.5, A::sqrt_square_offset(s.ad_value(2450), 5.0), 0.5));s.store_sub(2428, 2323, 2433);s.store_exp_neg_input(2429, 2433);s.store_div_from_scalar_offset_square(2430, 1.0, 2433, 2.0);s.store_mul_square_lhs(2440, 2433, 2430);s.store_mul3_affine_lhs(2441, 2433, 2430, 4.0, 0.0, 2430);s.store_mul_ad_product_lhs_mixed_ai(2442, A::sub_scaled_inputs(s.ad_value(2430), 8.0, s.ad_value(2440), 12.0), 2430, 2430);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_110(
        s: &mut Scratch,
    ) {
        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {
            if (1e-40 > ((s.v[2428] * s.v[2428]) - (s.v[2305] * (((s.v[2429] + s.v[2433]) - 1.0) - (s.v[2344] * ((s.v[2433] + 1.0) + s.v[2440])))))) {
                s.store_scalar(2434, 1e-40);
            } else {
                s.store_add_scaled_square_product_mixed_iia(2434, 2428, 1.0, 2305, A::add_scaled_product(A::offset(A::add(s.ad_value(2429), s.ad_value(2433)), (-1.0)), 1.0, s.ad_value(2344), A::add(A::offset(s.ad_value(2433), 1.0), s.ad_value(2440)), (-1.0)), (-1.0));
            }
        }
        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {s.store_sub_from_scalar_scaled_mul_mixed_ia(2451, 1.0, 2305, A::add_scaled_product(s.ad_value(2429), 1.0, s.ad_value(2344), s.ad_value(2442), (-1.0)), 0.5);s.store_add_scaled_product_mixed_iia(2435, 2428, 2.0, 2305, A::add_scaled_sub_value_product(1.0, s.ad_value(2429), 1.0, s.ad_value(2344), A::offset(s.ad_value(2441), 1.0), (-1.0)), 1.0);s.store_add_scaled_inputs3_mixed_iia(2436, 2329, 1.0, 2433, (-1.0), A::ln(A::div(s.ad_value(2434), s.ad_value(2305))), 1.0);s.store_add(813, 2434, 2435);s.store_add_scaled_square_product_mixed_iia(812, 813, 1.0, 2436, A::add_scaled_square_product(s.ad_value(2435), 0.5, s.ad_value(2434), s.ad_value(2451), (-1.0)), 1.0);s.store_add_mixed_ia(2452, 2433, A::div_scaled_product3(s.ad_value(2434), s.ad_value(813), s.ad_value(2436), 1.0, A::add(s.ad_value(812), A::mul3(A::mul3(A::div(s.ad_value(813), s.ad_value(812)), s.ad_value(2436), s.ad_value(2436)), s.ad_value(2435), A::add_scaled_square_product(s.ad_value(2435), 0.3333333333333333, s.ad_value(2434), s.ad_value(2451), (-1.0)))), 1.0));}
        s.b[2469] = (s.v[2452] < 230.25850929940458);s.store_scalar(2469, if s.b[2469] { 1.0 } else { 0.0 });
        if ((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && s.b[2469]) {s.store_exp(2438, 2452);s.store_div_from_scalar(2439, 1.0, 2438);s.store_mul(2438, 2344, 2438);}
        s.b[2470] = (s.v[2452] > (s.v[2329] - 230.25850929940458));s.store_scalar(2470, if s.b[2470] { 1.0 } else { 0.0 });
        if (((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2469])) && s.b[2470]) {s.store_exp_sub(2438, 2452, 2329);s.store_div(2439, 2344, 2438);}
        if (((((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) && (!s.b[2469])) && (!s.b[2470])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2438, 1e-100, A::sub(s.ad_value(2329), s.ad_value(2452)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2439, 1e-100, 2452, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {s.store_div_from_scalar_offset_square(2428, 1.0, 2452, 2.0);s.store_mul_square_lhs(2440, 2452, 2428);s.store_mul3_affine_lhs(2441, 2452, 2428, 4.0, 0.0, 2428);s.store_mul_ad_product_lhs_mixed_ai(2442, A::sub_scaled_inputs(s.ad_value(2428), 8.0, s.ad_value(2440), 12.0), 2428, 2428);s.store_sub(2428, 2323, 2452);s.store_add_scaled_product_mixed_iia(2443, 2428, 2.0, 2305, A::add_scaled_inputs_product(A::sub_from_scalar(1.0, s.ad_value(2439)), 1.0, s.ad_value(2438), 1.0, s.ad_value(2344), A::offset(s.ad_value(2441), 1.0), (-1.0)), 1.0);}
    }
    #[inline(never)]
    pub(super) fn stamp_transient_block_111(
        s: &mut Scratch,
    ) {
        if (((s.b[2453] && s.b[2454]) && (!s.b[2465])) && (!s.b[2466])) {s.store_add_scaled_square_product_mixed_iia(2444, 2428, 1.0, 2305, A::add_scaled_product(A::add_scaled_inputs3_offset(s.ad_value(2439), 1.0, s.ad_value(2452), 1.0, s.ad_value(2438), 1.0, (-1.0)), 1.0, s.ad_value(2344), A::add(A::offset(s.ad_value(2452), 1.0), s.ad_value(2440)), (-1.0)), (-1.0));s.store_sub_from_scalar_scaled_mul_mixed_ia(2428, 2.0, 2305, A::add_scaled_inputs_product(s.ad_value(2439), 1.0, s.ad_value(2438), 1.0, s.ad_value(2344), s.ad_value(2442), (-1.0)), 1.0);s.store_add_scaled_square_product_indices(2428, 2443, 1.0, 2444, 2428, (-2.0));s.store_add_scaled_inputs_mixed_ia(2343, 2452, 1.0, A::div(s.ad_value(2444), A::add(s.ad_value(2443), A::sqrt(s.ad_value(2428)))), 2.0);}
        if (s.b[2453] && s.b[2454]) {s.store_scalar(2346, 0.0);s.store_scalar(2347, 0.0);s.store_scalar(2348, 0.0);s.store_scalar(2349, 0.0);s.store_scalar(2350, 0.0);s.store_scalar(2351, 0.0);s.store_scalar(2352, 0.0);s.store_scalar(2353, 1.0);s.store_scalar(2354, 1.0);s.store_sub(2355, 2323, 2343);s.store_scalar(2356, 0.0);s.store_mul(2357, 2319, 2355);s.store_scalar(2358, 1.0);s.store_scalar(2359, 1.0);s.store_scalar(2363, 1.0);s.store_scalar(2364, 1.0);s.store_scalar(2366, 1.0);}
        s.b[2471] = (s.v[2323] > 0.0);s.store_scalar(2471, if s.b[2471] { 1.0 } else { 0.0 });
        if ((s.b[2453] && s.b[2454]) && s.b[2471]) {s.store_div_from_scalar_offset_square(1929, 1.0, 2343, 2.0);s.store_mul_square_lhs(2345, 2343, 1929);s.store_mul3_affine_lhs(2346, 2343, 1929, 4.0, 0.0, 1929);s.store_mul_ad_product_lhs_mixed_ai(2347, A::sub_scaled_inputs(s.ad_value(1929), 8.0, s.ad_value(2345), 12.0), 1929, 1929);s.store_scalar(2348, 0.0);}
        s.b[2472] = (s.v[2343] < 230.25850929940458);s.store_scalar(2472, if s.b[2472] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2472]) {s.store_exp(2348, 2343);s.store_div_from_scalar(2349, 1.0, 2348);s.store_mul(2348, 2344, 2348);}
        s.b[2473] = (s.v[2343] > (s.v[2329] - 230.25850929940458));s.store_scalar(2473, if s.b[2473] { 1.0 } else { 0.0 });
        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2472])) && s.b[2473]) {s.store_exp_sub(2348, 2343, 2329);s.store_div(2349, 2344, 2348);}
        if ((((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2472])) && (!s.b[2473])) {s.store_div_from_scalar_softlimit_poly_offset_lhs_ad(2348, 1e-100, A::sub(s.ad_value(2329), s.ad_value(2343)), (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);s.store_div_from_scalar_softlimit_poly_offset_lhs_mixed_ia(2349, 1e-100, 2343, (-230.25850929940458), 0.3333333333333333, (((((-230.25850929940458)) * (0.3333333333333333))) + (1.0)), 0.5, 1.0, 1.0);}
        if ((s.b[2453] && s.b[2454]) && s.b[2471]) {s.store_add_scaled_product_mixed_iia(2350, 2348, 1.0, 2344, A::add(A::offset(s.ad_value(2343), 1.0), s.ad_value(2345)), (-1.0));}
        s.b[2474] = (s.v[2343] < 1e-5);s.store_scalar(2474, if s.b[2474] { 1.0 } else { 0.0 });
        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && s.b[2474]) {s.store_square_mul_sub_from_scalar_scaled_sub_rhs_scaled_output(2351, 2343, 1.0, 2343, 1.0, 2343, 0.25, 0.3333333333333333, 0.5);s.store_mul3_ad_middle_scaled_output(2350, A::mul3(s.ad_value(2344), s.ad_value(2343), s.ad_value(2343)), 2343, A::scale_offset(s.ad_value(2343), 1.75, 1.0), 0.16666666666666666);s.store_sqrt_sub_from_scalar_ad(1929, 1.0, A::mul_sub_from_scalar_rhs_scaled_output(s.ad_value(2343), 1.0, A::scale(s.ad_value(2343), 0.25), 0.3333333333333333));s.store_scaled_mul(2352, 2343, 1929, 0.7071067811865475);s.store_offset_div_scaled_product_mixed_iai(2353, 2304, A::add_scaled_inputs(A::sub_from_scalar(1.0, A::scale(s.ad_value(2343), 0.5)), 1.0, A::square(s.ad_value(2343)), 0.16666666666666666), 0.7071067811865475, 1929, 1.0, 1.0);}
        if (((s.b[2453] && s.b[2454]) && s.b[2471]) && (!s.b[2474])) {s.store_add_offset_lhs(2351, 2343, (-1.0), 2349);s.store_sqrt(2352, 2351);}
    }
}
